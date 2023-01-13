use crate::net::versions::{PROTO_1_9, PROTO_MAX};
use anyhow::Result;
use bitflags::bitflags;
use cgmath::{Point3, Vector3};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::varint::*;

use super::{
    packets::Packet,
    types::Position,
    versions::{PROTO_1_7, PROTO_1_7_6, PROTO_1_8},
    ConnectionState,
};

#[allow(non_camel_case_types)]
#[derive(strum::IntoStaticStr, Clone)]
pub enum ChunkData {
    Bulk_5(crate::net::packets::play::clientbound::MapChunkBulk_5),
    Bulk_47(crate::net::packets::play::clientbound::MapChunkBulk_47),
    Single_5(crate::net::packets::play::clientbound::MapChunk_5),
    Single_47(crate::net::packets::play::clientbound::MapChunk_47),
    Single_107(crate::net::packets::play::clientbound::MapChunk_107),
    Single_110(crate::net::packets::play::clientbound::MapChunk_110),
}

bitflags! {
    pub struct PositionFlags: i8 {
        /// If set, X is relative
        const X =       0b00000001; // 0x01
        /// If set, Y is relative
        const Y =       0b00000010; // 0x02
        /// If set, Z is relative
        const Z =       0b00000100; // 0x04
        /// If set, yaw is relative
        const Y_ROT =   0b00001000; // 0x08
        /// If set, pitch is relative
        const X_ROT =   0b00010000; // 0x10
    }
}

#[derive(strum::IntoStaticStr, Clone)]
pub enum AbstractPacket {
    // * Handshake
    SetProtocol {
        protocol_version: VarInt,
        server_host: String,
        server_port: u16,
        /// Only Login and Status are allowed for this field
        next_state: ConnectionState,
    },

    // * Login
    SetCompression {
        threshold: VarInt,
    },
    Disconnect {
        reason: String,
    },
    LoginStart {
        username: String,
    },
    LoginSuccess {
        uuid: String,
        username: String,
    },
    EncryptionBeginClientbound {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
    },
    EncryptionBeginServerbound {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },

    // * Status
    ServerInfoRequest {},
    ServerInfo {
        response: String,
    },
    Ping {
        time: i64,
    },
    PingResponse {
        time: i64,
    },

    // * Play
    KeepAlive {
        keep_alive_id: i64,
    },

    PositionLookClientBound {
        pos: Point3<f64>,
        yaw: f32,
        pitch: f32,
        flags: Option<PositionFlags>,
        teleport_id: Option<i32>,
    },

    PositionLookServerBound {
        pos: Point3<f64>,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },

    PositionServerBound {
        pos: Point3<f64>,
        on_ground: bool,
    },

    ClientCommand {
        action_id: i32,
    },

    Explosion {
        pos: Point3<f64>,
        radius: f32,
        affected_block_offsets: Vec<(i8, i8, i8)>,
        player_motion: Vector3<f64>,
    },

    Respawn {
        dimension: i32,
        difficulty: u8, // TODO: Enum?
        gamemode: u8,   // TODO: Enum?
        level_kind: String,
    },

    BlockChange {
        location: Position,
        kind: i32,
    },

    NamedSoundEffect {
        sound_name: String,
        sound_category: Option<i32>,
        pos: Point3<f64>,
        volume: f32,
        pitch: f32,
    },

    ChatServerbound(String),

    Chunks(ChunkData),
}

impl AbstractPacket {
    // TODO: Error handling
    pub fn from_packet(packet: Packet) -> Option<Self> {
        Some(match packet {
            // * Handshake
            Packet::SetProtocol_5(p) => Self::SetProtocol {
                protocol_version: p.protocol_version,
                server_host: p.server_host,
                server_port: p.server_port,
                next_state: ConnectionState::from_i32(p.next_state.0)?,
            },
            // * Login
            Packet::SetCompression_47(p) => Self::SetCompression {
                threshold: p.threshold,
            },
            Packet::LoginStart_5(p) => Self::LoginStart {
                username: p.username.clone(),
            },

            // * Chunk data
            Packet::MapChunkBulk_5(p) => Self::Chunks(ChunkData::Bulk_5(p)),
            Packet::MapChunkBulk_47(p) => Self::Chunks(ChunkData::Bulk_47(p)),
            Packet::MapChunk_5(p) => Self::Chunks(ChunkData::Single_5(p)),
            Packet::MapChunk_47(p) => Self::Chunks(ChunkData::Single_47(p)),
            Packet::MapChunk_107(p) => Self::Chunks(ChunkData::Single_107(p)),
            Packet::MapChunk_110(p) => Self::Chunks(ChunkData::Single_110(p)),

            Packet::Position_5(p) => Self::PositionLookClientBound {
                pos: Point3::new(p.x, p.y, p.z),
                yaw: p.yaw,
                pitch: p.pitch,
                flags: None,
                teleport_id: None,
            },
            Packet::Position_47(p) => Self::PositionLookClientBound {
                pos: Point3::new(p.x, p.y, p.z),
                yaw: p.yaw,
                pitch: p.pitch,
                flags: PositionFlags::from_bits(p.flags),
                teleport_id: None,
            },
            Packet::Position_107(p) => Self::PositionLookClientBound {
                pos: Point3::new(p.x, p.y, p.z),
                yaw: p.yaw,
                pitch: p.pitch,
                flags: PositionFlags::from_bits(p.flags),
                teleport_id: Some(p.teleport_id.0),
            },
            Packet::Success_5(p) => Self::LoginSuccess {
                username: p.username,
                uuid: p.uuid,
            },
            Packet::Explosion_5(p) => Self::Explosion {
                pos: Point3::new(p.x as f64, p.y as f64, p.z as f64),
                radius: p.radius,
                affected_block_offsets: p
                    .affected_block_offsets
                    .data
                    .iter()
                    .map(|r| (r.x, r.y, r.z))
                    .collect(),
                player_motion: Vector3::new(
                    p.player_motion_x as f64,
                    p.player_motion_y as f64,
                    p.player_motion_z as f64,
                ),
            },

            Packet::Respawn_5(p) => Self::Respawn {
                dimension: p.dimension,
                difficulty: p.difficulty,
                gamemode: p.gamemode,
                level_kind: p.level_kind,
            },

            Packet::BlockChange_5(p) => Self::BlockChange {
                location: p.location.into(),
                kind: p.kind.0,
            },

            Packet::BlockChange_47(p) => Self::BlockChange {
                location: p.location,
                kind: p.kind.0,
            },

            Packet::NamedSoundEffect_5(p) => Self::NamedSoundEffect {
                sound_name: p.sound_name,
                sound_category: None,
                pos: Point3::new(p.x as f64 / 8., p.y as f64 / 8., p.z as f64 / 8.),
                volume: p.volume,
                pitch: p.pitch as f32 / 63.,
            },

            Packet::NamedSoundEffect_107(p) => Self::NamedSoundEffect {
                sound_name: p.sound_name,
                sound_category: Some(p.sound_category.0),
                pos: Point3::new(p.x as f64 / 8., p.y as f64 / 8., p.z as f64 / 8.),
                volume: p.volume,
                pitch: p.pitch as f32 / 63.,
            },

            Packet::NamedSoundEffect_210(p) => Self::NamedSoundEffect {
                sound_name: p.sound_name,
                sound_category: Some(p.sound_category.0),
                pos: Point3::new(p.x as f64 / 8., p.y as f64 / 8., p.z as f64 / 8.),
                volume: p.volume,
                pitch: p.pitch,
            },

            Packet::EntityHeadRotation_5 { .. }
            | Packet::EntityLook_5 { .. }
            | Packet::EntityMetadata_5 { .. }
            | Packet::EntityMoveLook_5 { .. }
            | Packet::EntityVelocity_5 { .. }
            | Packet::RelEntityMove_5 { .. }
            | Packet::EntityDestroy_5 { .. }
            | Packet::EntityTeleport_5 { .. }
            | Packet::SpawnEntityLiving_5 { .. }
            | Packet::EntityHeadRotation_47 { .. }
            | Packet::EntityLook_47 { .. }
            | Packet::EntityMetadata_47 { .. }
            | Packet::EntityMoveLook_47 { .. }
            | Packet::EntityVelocity_47 { .. }
            | Packet::RelEntityMove_47 { .. }
            | Packet::EntityDestroy_47 { .. }
            | Packet::EntityTeleport_47 { .. }
            | Packet::UpdateAttributes_5 { .. }
            | Packet::UpdateAttributes_47 { .. } => {
                trace!("Packet {packet:?} suppressed");
                return None;
            }

            _ => {
                warn!("Packet {:?} was not translated", packet);
                return None;
            }
        })
    }

    // FIXME: The protocol matches may need a bit of a rework
    pub fn to_packet(self, protocol: i32) -> Result<Packet> {
        macro_rules! quick_bail {
            () => {
                anyhow::bail!(
                    "No mapping for protocol {} for packet {}",
                    protocol,
                    <&'static str>::from(self)
                )
            };
        }

        Ok(match self {
            AbstractPacket::SetProtocol {
                protocol_version,
                server_host,
                server_port,
                next_state,
            } => Packet::SetProtocol_5(super::packets::handshaking::serverbound::SetProtocol_5 {
                // FIXME: Fucking ugly
                protocol_version,
                server_host,
                server_port,
                next_state: VarInt(next_state.to_i32().unwrap()),
            }),

            AbstractPacket::LoginStart { username } => {
                Packet::LoginStart_5(super::packets::login::serverbound::LoginStart_5 { username })
            }
            AbstractPacket::PositionLookServerBound {
                pos,
                yaw,
                pitch,
                on_ground,
            } => match protocol {
                PROTO_1_7..=PROTO_1_7_6 => {
                    Packet::PositionLook_5(super::packets::play::serverbound::PositionLook_5 {
                        x: pos.x,
                        stance: pos.y,
                        y: pos.y + 1.62, // TODO: Do we have to account for crouching?
                        z: pos.z,
                        yaw,
                        pitch,
                        on_ground,
                    })
                }
                PROTO_1_8 => {
                    Packet::PositionLook_47(super::packets::play::serverbound::PositionLook_47 {
                        x: pos.x,
                        y: pos.y,
                        z: pos.z,
                        yaw,
                        pitch,
                        on_ground,
                    })
                }
                _ => quick_bail!(),
            },
            AbstractPacket::ClientCommand { action_id } => match protocol {
                PROTO_1_7..=PROTO_1_7_6 => {
                    Packet::ClientCommand_5(super::packets::play::serverbound::ClientCommand_5 {
                        payload: action_id as i8,
                    })
                }
                PROTO_1_8 => {
                    Packet::ClientCommand_47(super::packets::play::serverbound::ClientCommand_47 {
                        payload: VarInt(action_id),
                    })
                }
                PROTO_1_9..=PROTO_MAX => Packet::ClientCommand_107(
                    super::packets::play::serverbound::ClientCommand_107 {
                        action_id: VarInt(action_id),
                    },
                ),
                _ => quick_bail!(),
            },
            AbstractPacket::ChatServerbound(m) => {
                Packet::ChatServerbound_5(super::packets::play::serverbound::ChatServerbound_5 {
                    message: m,
                })
            }
            _ => quick_bail!(),
        })
    }
}
