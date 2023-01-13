use crate::net::ProtocolVersion;

pub mod v1_10;
pub mod v1_11;
pub mod v1_12;
pub mod v1_12_1;
pub mod v1_12_2;
pub mod v1_7_6;
pub mod v1_8;
pub mod v1_9;
pub mod v1_9_2;
pub mod v1_9_4;

pub fn decode_packet(protocol: ProtocolVersion, p: &crate::net::codec::RawPacket, state: crate::net::ConnectionState, dir: crate::net::PacketDirection) -> anyhow::Result<crate::net::packets::Packet> {
    // TODO: Macro can simplify this a bit
    let func = match protocol {
        ProtocolVersion::Proto1_7_6 => v1_7_6::decode_packet,
        ProtocolVersion::Proto1_8 => v1_8::decode_packet,
        ProtocolVersion::Proto1_9 => v1_9::decode_packet,
        ProtocolVersion::Proto1_9_2 => v1_9_2::decode_packet,
        ProtocolVersion::Proto1_9_4 => v1_9_4::decode_packet,
        ProtocolVersion::Proto1_10 => v1_10::decode_packet,
        ProtocolVersion::Proto1_11 => v1_11::decode_packet,
        ProtocolVersion::Proto1_12 => v1_12::decode_packet,
        ProtocolVersion::Proto1_12_1 => v1_12_1::decode_packet,
        ProtocolVersion::Proto1_12_2 => v1_12_2::decode_packet,
        _ => anyhow::bail!("No ID mapping found for {:?} in decode_packet", protocol),
    };

    func(p, state, dir)
}

pub fn encode_packet(protocol: ProtocolVersion, p: &crate::net::packets::Packet, state: crate::net::ConnectionState, dir: crate::net::PacketDirection) -> anyhow::Result<crate::net::codec::RawPacket> {
    // TODO: Macro can simplify this a bit
    let func = match protocol {
        ProtocolVersion::Proto1_7_6 => v1_7_6::encode_packet,
        ProtocolVersion::Proto1_8 => v1_8::encode_packet,
        ProtocolVersion::Proto1_9 => v1_9::encode_packet,
        ProtocolVersion::Proto1_9_2 => v1_9_2::encode_packet,
        ProtocolVersion::Proto1_9_4 => v1_9_4::encode_packet,
        ProtocolVersion::Proto1_10 => v1_10::encode_packet,
        ProtocolVersion::Proto1_11 => v1_11::encode_packet,
        ProtocolVersion::Proto1_12 => v1_12::encode_packet,
        ProtocolVersion::Proto1_12_1 => v1_12_1::encode_packet,
        ProtocolVersion::Proto1_12_2 => v1_12_2::encode_packet,
        _ => anyhow::bail!("No ID mapping found for {:?} in encode_packet", protocol),
    };

    func(p, state, dir)
}

pub const PROTO_1_7: i32 = 4;
pub const PROTO_1_7_6: i32 = 5;
pub const PROTO_1_8: i32 = 47;
pub const PROTO_1_9: i32 = 107;
pub const PROTO_1_9_2: i32 = 109;
pub const PROTO_1_9_4: i32 = 110;
pub const PROTO_1_10: i32 = 210;
pub const PROTO_1_11: i32 = 315;
pub const PROTO_1_12: i32 = 335;
pub const PROTO_1_12_1: i32 = 338;
pub const PROTO_1_12_2: i32 = 340;
pub const PROTO_1_13: i32 = 393;
pub const PROTO_1_13_1: i32 = 401;
pub const PROTO_1_13_2: i32 = 404;
pub const PROTO_1_14: i32 = 477;
pub const PROTO_1_14_1: i32 = 480;
pub const PROTO_1_14_3: i32 = 490;
pub const PROTO_1_14_4: i32 = 498;
pub const PROTO_1_15: i32 = 573;
pub const PROTO_1_15_1: i32 = 575;
pub const PROTO_1_15_2: i32 = 578;
pub const PROTO_1_16: i32 = 735;
pub const PROTO_1_16_1: i32 = 736;
pub const PROTO_1_16_2: i32 = 751;
pub const PROTO_1_17: i32 = 755;
pub const PROTO_1_17_1: i32 = 756;
pub const PROTO_1_18: i32 = 757;
pub const PROTO_1_18_2: i32 = 758;
pub const PROTO_1_19: i32 = 759;
pub const PROTO_1_19_2: i32 = 760;
pub const PROTO_MAX: i32 = PROTO_1_19_2;

// Snapshot versions used for certain types (eg Position type's bit ordering changed in 18w43a)
pub const PROTO_18W43A: i32 = 441;
