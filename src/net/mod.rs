use num_derive::{FromPrimitive, ToPrimitive};

pub mod codec;
pub mod connection;
pub mod packet_helpers;
pub mod packets;
pub mod types;
pub mod versions;
pub mod wrapper;

#[derive(Debug, Copy, Clone, PartialEq, ToPrimitive, FromPrimitive)]
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

// TODO: Constants might be a bit redundant?
use crate::net::versions::*;
#[derive(Debug, PartialEq, Clone, Copy, ToPrimitive)]
#[repr(i32)]
pub enum ProtocolVersion {
    Proto1_7 = PROTO_1_7,
    Proto1_7_6 = PROTO_1_7_6,
    Proto1_8 = PROTO_1_8,
    Proto1_9 = PROTO_1_9,
    Proto1_9_2 = PROTO_1_9_2,
    Proto1_9_4 = PROTO_1_9_4,
    Proto1_10 = PROTO_1_10,
    Proto1_11 = PROTO_1_11,
    Proto1_12 = PROTO_1_12,
    Proto1_12_1 = PROTO_1_12_1,
    Proto1_12_2 = PROTO_1_12_2,
}
