pub mod codec;
pub mod packet_helpers;
pub mod packets;
pub mod types;
pub mod versions;

#[derive(Debug)]
pub enum ClientState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

#[derive(Debug)]
pub enum PacketDirection {
    Client,
    Server,
}
