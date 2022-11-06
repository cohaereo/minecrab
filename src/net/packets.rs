use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::varint::{ReadProtoExt, WriteProtoExt};

use super::{codec::RawPacket, decoders::*};

// TODO: better variants?
#[derive(Debug)]
pub enum GameState {
    InvalidBed,
    EndRaining,
    BeginRaining,
    ChangeGamemode(f32),
    EnterCredits,
    DemoMessages(f32),
    ArrowHittingPlayer,
    FadeValue(f32),
    FadeTime(f32),
}

#[derive(Debug)]
pub struct EntityProperty {
    pub key: String,
    pub value: f64,
    pub modifiers: Vec<EntityModifier>,
}

#[derive(Debug)]
pub struct EntityModifier {
    pub uuid: u128,
    pub amount: f64,
    pub operation: u8,
}

#[derive(Debug)]
pub struct ChunkMetadata {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub primary_bitmap: u16,
    pub add_bitmap: u16,
}

#[derive(Debug)]
pub struct BlockChangeRecord {
    pub block_meta: u8,
    pub block_id: u16,
    pub y: u8,
    pub z: u8,
    pub x: u8,
}

#[derive(Debug)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}

impl BlockChangeRecord {
    pub fn from_u32(v: u32) -> Self {
        Self {
            block_meta: (v & 0x0f) as u8,
            block_id: ((v >> 4) & 0x0fff) as u16,
            y: ((v >> 16) & 0xff) as u8,
            z: ((v >> 24) & 0x0f) as u8,
            x: ((v >> 28) & 0x0f) as u8,
        }
    }
}

#[derive(Debug)]
pub enum Packet {
    Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: i32,
    },

    LoginStart(String),

    KeepAlive(i32),

    ChatMessage(String),

    SpawnPosition {
        x: i32,
        y: i32,
        z: i32,
    },

    HeldItemChangeServer(i8),  // From server
    HeldItemChangeClient(i16), // From client

    PlayerPositionLookServer {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },

    PlayerPositionLookClient {
        x: f64,
        feet_y: f64,
        head_y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },

    ChangeGameState(GameState),

    TimeUpdate {
        // These values are both in ticks
        world_age: i64,
        time_of_day: i64,
    },

    PlayerListItem {
        player_name: String,
        online: bool,
        ping: i16,
    },

    JoinGame {
        eid: i32,
        gamemode: u8,
        dimension: i8,
        difficulty: u8,
        max_players: u8,
        level_type: String,
    },

    // TODO: Enum
    ClientStatus(u8),

    SpawnMob {
        eid: i32,
        mobtype: u8,
        x: f64,
        y: f64,
        z: f64,
        yaw: u8,
        pitch: u8,
        head_pitch: u8,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
        // TODO: Metadata
    },

    SpawnObject {
        eid: i32,
        objtype: u8,
        x: f64,
        y: f64,
        z: f64,
        yaw: u8,
        pitch: u8,
        // TODO: Metadata
    },

    EntityProperties {
        eid: i32,
        properties: Vec<EntityProperty>,
    },
    // EntityEquipment {
    //     eid: i32,
    //     slot: i16,
    //     item: Slot,
    // },
    EntityVelocity {
        eid: i32,
        x: i16,
        y: i16,
        z: i16,
    },

    EntityHeadLook {
        eid: i32,
        head_yaw: u8,
    },

    EntityTeleport {
        eid: i32,
        x: f64,
        y: f64,
        z: f64,
        yaw: u8,
        pitch: u8,
    },

    EntityRelativeMove {
        eid: i32,
        dx: f64,
        dy: f64,
        dz: f64,
    },

    EntityLook {
        eid: i32,
        yaw: u8,
        pitch: u8,
    },

    EntityLookRelativeMove {
        eid: i32,
        dx: f64,
        dy: f64,
        dz: f64,
        yaw: u8,
        pitch: u8,
    },

    DestroyEntities(Vec<i32>),

    Disconnect(String),

    EntityStatus {
        eid: i32,
        // TODO: Enum
        status: u8,
    },

    UpdateHealth {
        health: f32,
        food: i16,
        saturation: f32,
    },

    SoundEffect {
        sound_name: String,
        pos_x: i32,
        pos_y: i32,
        pos_z: i32,
        volume: f32,
        pitch: u8,
    },

    EntityMetadata {
        eid: i32,
        // TODO: the actual metadata
    },

    MapChunkBulk {
        columns: i16,
        has_sky_light: bool,
        data: Vec<u8>,
        meta: Vec<ChunkMetadata>,
    },

    ChunkData {
        chunk_x: i32,
        chunk_z: i32,
        ground_up_continuous: bool,
        primary_bitmap: u16,
        add_bitmap: u16,
        data: Vec<u8>,
    },

    Respawn {
        dimension: i32,
        difficulty: u8,
        gamemode: u8,
        level_type: String,
    },

    BlockChange {
        x: i32,
        y: u8,
        z: i32,
        block_id: i32,
        block_meta: u8,
    },

    MultiBlockChange {
        chunk_x: i32,
        chunk_z: i32,
        records: Vec<BlockChangeRecord>,
    },

    SpawnPlayer {
        eid: i32,
        player_uuid: u128,
        player_name: String,
        properties: Vec<PlayerProperty>,
        x: f64,
        y: f64,
        z: f64,
        yaw: u8,
        pitch: u8,
        current_item: i16,
        // TODO: metadata
    },
}

// TODO: Packets should be able to be encoded by struct definitions. Seeing as how this is rust, well....
// TODO: XYZ/Velocity should be a single struct?

pub fn decode_packet(p: &RawPacket) -> anyhow::Result<Packet> {
    let mut reader = std::io::Cursor::new(&p.data);
    match p.id {
        0x0 => play::decode_0x0(&mut reader),
        0x1 => play::decode_0x1(&mut reader),
        0x2 => play::decode_0x2(&mut reader),
        0x3 => play::decode_0x3(&mut reader),
        0x5 => play::decode_0x5(&mut reader),
        0x6 => play::decode_0x6(&mut reader),
        0x7 => play::decode_0x7(&mut reader),
        0x8 => play::decode_0x8(&mut reader),
        0x9 => play::decode_0x9(&mut reader),
        0xc => play::decode_0xc(&mut reader),
        0xe => play::decode_0xe(&mut reader),
        0xf => play::decode_0xf(&mut reader),
        0x12 => play::decode_0x12(&mut reader),
        0x13 => play::decode_0x13(&mut reader),
        0x15 => play::decode_0x15(&mut reader),
        0x16 => play::decode_0x16(&mut reader),
        0x17 => play::decode_0x17(&mut reader),
        0x18 => play::decode_0x18(&mut reader),
        0x19 => play::decode_0x19(&mut reader),
        0x1c => {
            trace!("0x1c IS STUBBED!!!!");
            Ok(Packet::EntityMetadata {
                eid: reader.read_i32::<BigEndian>()?,
            })
        }
        0x1a => play::decode_0x1a(&mut reader),
        0x20 => play::decode_0x20(&mut reader),
        0x22 => play::decode_0x22(&mut reader),
        0x23 => play::decode_0x23(&mut reader),
        0x29 => play::decode_0x29(&mut reader),
        0x2b => play::decode_0x2b(&mut reader),
        0x38 => play::decode_0x38(&mut reader),
        0x21 => play::decode_0x21(&mut reader),
        0x26 => play::decode_0x26(&mut reader),
        // {
        //     println!("{} chunks", reader.read_i16::<BigEndian>()?);
        //     println!(
        //         "{} data len (total {})",
        //         reader.read_i32::<BigEndian>()?,
        //         p.data.len()
        //     );
        //     anyhow::bail!("Chunkbatch");
        // }
        0x40 => Ok(Packet::Disconnect(reader.read_varstring()?)),
        _ => anyhow::bail!("Unhandled packet ID 0x{:x}", p.id),
    }
}

pub fn encode_packet(p: &Packet) -> anyhow::Result<RawPacket> {
    match p {
        Packet::LoginStart(username) => {
            let mut rp = RawPacket {
                id: 0,
                data: vec![],
            };

            rp.data.write_varstring(username)?;

            Ok(rp)
        }
        Packet::Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        } => {
            let mut rp = RawPacket {
                id: 0,
                data: vec![],
            };

            rp.data.write_varint(*protocol_version)?;
            rp.data.write_varstring(server_address)?;
            rp.data.write_u16::<BigEndian>(*server_port)?;
            rp.data.write_varint(*next_state)?;

            Ok(rp)
        }
        Packet::ChatMessage(s) => {
            let mut rp = RawPacket {
                id: 1,
                data: vec![],
            };

            rp.data.write_varstring(s)?;

            Ok(rp)
        }
        Packet::PlayerPositionLookClient {
            x,
            feet_y,
            head_y,
            z,
            yaw,
            pitch,
            on_ground,
        } => {
            let mut rp = RawPacket {
                id: 6,
                data: vec![],
            };

            rp.data.write_f64::<BigEndian>(*x)?;
            rp.data.write_f64::<BigEndian>(*feet_y)?;
            rp.data.write_f64::<BigEndian>(*head_y)?;
            rp.data.write_f64::<BigEndian>(*z)?;
            rp.data.write_f32::<BigEndian>(*yaw)?;
            rp.data.write_f32::<BigEndian>(*pitch)?;
            rp.data.write_u8(if *on_ground { 1 } else { 0 })?;

            Ok(rp)
        }
        Packet::ClientStatus(s) => {
            let mut rp = RawPacket {
                id: 0x16,
                data: vec![],
            };

            rp.data.write_u8(*s)?;

            Ok(rp)
        }
        _ => anyhow::bail!("No match arm to encode packet {:?}!", p),
    }
}
