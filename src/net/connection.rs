use crate::net::wrapper::AbstractPacket;
use crate::net::PacketDirection;
use num_traits::ToPrimitive;
use tokio::{net::TcpStream, sync::mpsc, task::JoinHandle};

use super::{
    codec::{MinecraftCodec, RawPacket},
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

    fn check_for_state_change(&mut self, p: &AbstractPacket) {
        match p {
            AbstractPacket::SetProtocol { next_state, .. } => {
                self.state = *next_state;
                debug!("State switching to {:?}", self.state);
            }
            AbstractPacket::LoginSuccess { .. } => {
                debug!("State switching to Play");
                self.state = ConnectionState::Play;
            }

            AbstractPacket::SetCompression { .. } => {
                panic!("Compression is not supported!");
            }

            AbstractPacket::EncryptionBeginClientbound { .. } => {
                panic!("Encryption is not supported!");
            }
            _ => {}
        }
    }

    // TODO: Error handling
    pub fn read(&mut self) -> Option<AbstractPacket> {
        let data = self.packet_rx.try_recv().ok()?;
        match super::versions::decode_packet(
            self.protocol,
            &data,
            self.state,
            super::PacketDirection::Client,
        ) {
            Ok(p) => match AbstractPacket::from_packet(p) {
                Some(ap) => {
                    self.check_for_state_change(&ap);
                    Some(ap)
                }
                None => {
                    if self.state != ConnectionState::Play {
                        panic!("Non-play packet could not be translated to an AbstractPacket");
                    } else {
                        None
                    }
                }
            },
            Err(e) => {
                if self.state != ConnectionState::Play {
                    panic!(
                        "An error occurred while decoding non-play packet 0x{:x}: {}",
                        data.id, e
                    );
                } else {
                    error!("Error decoding packet 0x{:x}: {}", data.id, e);
                    None
                }
            }
        }
    }

    pub fn write(&mut self, ap: AbstractPacket) -> anyhow::Result<()> {
        // FIXME: we're cloning the packet because the state change check happens after decoding, otherwise the encoder will fail because the state changed before encoding
        match ap.clone().to_packet(self.protocol.to_i32().unwrap()) {
            Ok(p) => {
                let rp = super::versions::encode_packet(
                    self.protocol,
                    &p,
                    self.state,
                    PacketDirection::Server,
                )?;
                self.check_for_state_change(&ap);

                self.packet_tx.try_send(rp)?;
            }
            Err(e) => {
                if self.state != ConnectionState::Play {
                    panic!("An error occurred while encoding a non-play packet: {e}")
                } else {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
