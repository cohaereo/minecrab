pub mod codec;
pub mod connection;
pub mod packet_helpers;
pub mod packets;
pub mod types;
pub mod versions;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConnectionState {
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

#[derive(Debug, PartialEq)]
pub enum ProtocolVersion {
    Proto1_7_2 = 4,
    Proto1_7_6 = 5,
    Proto1_8 = 47,
    Proto1_9 = 107,
    Proto1_10 = 210,
    Proto1_11 = 315,
    Proto1_12 = 335,
}
