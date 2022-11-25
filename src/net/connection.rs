use tokio::{net::TcpStream, sync::mpsc, task::JoinHandle};

use super::{
    codec::{MinecraftCodec, RawPacket},
    packets::Packet,
    ConnectionState, ProtocolVersion,
};

pub struct ClientConnection {
    pub protocol: ProtocolVersion,
    pub state: ConnectionState,

    packet_tx: mpsc::Sender<RawPacket>,
    packet_rx: mpsc::Receiver<RawPacket>,
    thread_send: JoinHandle<()>,
    thread_recv: JoinHandle<()>,
}

impl ClientConnection {
    pub fn from_stream(tcp: TcpStream, protocol: ProtocolVersion) -> Self {
        let (mut read_half, mut write_half) = tcp.into_split();

        let (write_tx, mut write_rx) = tokio::sync::mpsc::channel::<RawPacket>(128);
        let thread_send = tokio::spawn(async move {
            loop {
                if let Some(rp) = write_rx.recv().await {
                    MinecraftCodec::write(&mut write_half, &rp).await.unwrap();
                } else {
                    // Channel is closed, exit thread
                    break;
                }
            }
        });

        let (read_tx, read_rx) = tokio::sync::mpsc::channel::<RawPacket>(512);
        let thread_recv = tokio::spawn(async move {
            loop {
                if let Ok(rp) = MinecraftCodec::read(&mut read_half).await {
                    read_tx.send(rp).await.ok();
                }
            }
        });

        Self {
            packet_tx: write_tx,
            packet_rx: read_rx,
            thread_send,
            thread_recv,

            protocol,
            state: ConnectionState::Handshaking,
        }
    }

    fn check_for_state_change(&mut self, p: &Packet) {
        match p {
            Packet::SetProtocol(p) => {
                self.state = match p.next_state.0 {
                    1 => ConnectionState::Status,
                    2 => ConnectionState::Login,
                    _ => panic!("Invalid next_state!"),
                };
                debug!("State switching to {:?}", self.state);
            }
            Packet::LoginSuccess(_) => {
                debug!("State switching to Play");
                self.state = ConnectionState::Play;
            }
            _ => {}
        }
    }

    pub fn read(&mut self) -> Option<Packet> {
        let data = self.packet_rx.try_recv().ok()?;
        match super::versions::v1_7_10::decode_packet(
            &data,
            self.state,
            super::PacketDirection::Client,
        ) {
            Ok(p) => {
                self.check_for_state_change(&p);
                Some(p)
            }
            Err(e) => {
                error!("Error decoding packet 0x{:x}: {}", data.id, e);
                None
            }
        }
    }

    pub fn write(&mut self, p: &Packet) -> anyhow::Result<()> {
        let rp =
            super::versions::v1_7_10::encode_packet(p, self.state, super::PacketDirection::Server)?;

        self.packet_tx.try_send(rp)?;
        self.check_for_state_change(p);

        Ok(())
    }
}
