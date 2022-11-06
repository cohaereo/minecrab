use std::io::{Cursor, Read};

use anyhow::{ensure, Result};
use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;

use crate::{
    fixed_point::FixedPoint,
    net::packets::{
        BlockChangeRecord, ChunkMetadata, EntityModifier, EntityProperty, GameState, Packet,
        PlayerProperty,
    },
    varint::ReadProtoExt,
};

pub fn decode_0x0<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::KeepAlive(r.read_i32::<BigEndian>()?))
}

pub fn decode_0x1<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::JoinGame {
        eid: r.read_i32::<BigEndian>()?,
        gamemode: r.read_u8()?,
        dimension: r.read_i8()?,
        difficulty: r.read_u8()?,
        max_players: r.read_u8()?,
        level_type: r.read_varstring()?,
    })
}

pub fn decode_0x2<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::ChatMessage(r.read_varstring()?))
}

pub fn decode_0x3<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::TimeUpdate {
        world_age: r.read_i64::<BigEndian>()?,
        time_of_day: r.read_i64::<BigEndian>()?,
    })
}

pub fn decode_0x5<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::SpawnPosition {
        x: r.read_i32::<BigEndian>()?,
        y: r.read_i32::<BigEndian>()?,
        z: r.read_i32::<BigEndian>()?,
    })
}

pub fn decode_0x6<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::UpdateHealth {
        health: r.read_f32::<BigEndian>()?,
        food: r.read_i16::<BigEndian>()?,
        saturation: r.read_f32::<BigEndian>()?,
    })
}

pub fn decode_0x7<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::Respawn {
        dimension: r.read_i32::<BigEndian>()?,
        difficulty: r.read_u8()?,
        gamemode: r.read_u8()?,
        level_type: r.read_varstring()?,
    })
}

pub fn decode_0x8<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::PlayerPositionLookServer {
        x: r.read_f64::<BigEndian>()?,
        y: r.read_f64::<BigEndian>()?,
        z: r.read_f64::<BigEndian>()?,
        yaw: r.read_f32::<BigEndian>()?,
        pitch: r.read_f32::<BigEndian>()?,
        on_ground: r.read_u8()? == 1,
    })
}

pub fn decode_0x9<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::HeldItemChangeServer(r.read_i8()?))
}

pub fn decode_0x12<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityVelocity {
        eid: r.read_i32::<BigEndian>()?,
        x: r.read_i16::<BigEndian>()?,
        y: r.read_i16::<BigEndian>()?,
        z: r.read_i16::<BigEndian>()?,
    })
}

pub fn decode_0x13<R: Read>(r: &mut R) -> Result<Packet> {
    let count = r.read_u8()? as usize;
    let mut entities = Vec::with_capacity(count);

    for _ in 0..count {
        entities.push(r.read_i32::<BigEndian>()?);
    }

    Ok(Packet::DestroyEntities(entities))
}

pub fn decode_0x15<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityRelativeMove {
        eid: r.read_i32::<BigEndian>()?,
        dx: r.read_i8()?.to_f64(),
        dy: r.read_i8()?.to_f64(),
        dz: r.read_i8()?.to_f64(),
    })
}

pub fn decode_0x16<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityLook {
        eid: r.read_i32::<BigEndian>()?,
        yaw: r.read_u8()?,
        pitch: r.read_u8()?,
    })
}

pub fn decode_0x17<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityLookRelativeMove {
        eid: r.read_i32::<BigEndian>()?,
        dx: r.read_i8()?.to_f64(),
        dy: r.read_i8()?.to_f64(),
        dz: r.read_i8()?.to_f64(),
        yaw: r.read_u8()?,
        pitch: r.read_u8()?,
    })
}

pub fn decode_0x18<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityTeleport {
        eid: r.read_i32::<BigEndian>()?,
        x: r.read_i32::<BigEndian>()?.to_f64(),
        y: r.read_i32::<BigEndian>()?.to_f64(),
        z: r.read_i32::<BigEndian>()?.to_f64(),
        yaw: r.read_u8()?,
        pitch: r.read_u8()?,
    })
}

pub fn decode_0x19<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityHeadLook {
        eid: r.read_i32::<BigEndian>()?,
        head_yaw: r.read_u8()?,
    })
}

pub fn decode_0x1a<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::EntityStatus {
        eid: r.read_i32::<BigEndian>()?,
        status: r.read_u8()?,
    })
}

pub fn decode_0xc<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::SpawnPlayer {
        eid: r.read_varint()?,
        player_uuid: r.read_u128::<BigEndian>()?,
        player_name: r.read_varstring()?,
        properties: (0..r.read_varint()?)
            .map(|_| PlayerProperty {
                name: r.read_varstring().unwrap(),
                value: r.read_varstring().unwrap(),
                signature: r.read_varstring().unwrap(),
            })
            .collect(),
        x: r.read_i32::<BigEndian>()?.to_f64(),
        y: r.read_i32::<BigEndian>()?.to_f64(),
        z: r.read_i32::<BigEndian>()?.to_f64(),
        yaw: r.read_u8()?,
        pitch: r.read_u8()?,
        current_item: r.read_i16::<BigEndian>()?,
    })
}

pub fn decode_0xe<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::SpawnObject {
        eid: r.read_varint()?,
        objtype: r.read_u8()?,
        x: r.read_i32::<BigEndian>()?.to_f64(),
        y: r.read_i32::<BigEndian>()?.to_f64(),
        z: r.read_i32::<BigEndian>()?.to_f64(),
        yaw: r.read_u8()?,
        pitch: r.read_u8()?,
    })
}

pub fn decode_0xf<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::SpawnMob {
        eid: r.read_varint()?,
        mobtype: r.read_u8()?,
        x: r.read_i32::<BigEndian>()?.to_f64(),
        y: r.read_i32::<BigEndian>()?.to_f64(),
        z: r.read_i32::<BigEndian>()?.to_f64(),
        yaw: r.read_u8()?,
        pitch: r.read_u8()?,
        head_pitch: r.read_u8()?,
        velocity_x: r.read_i16::<BigEndian>()?,
        velocity_y: r.read_i16::<BigEndian>()?,
        velocity_z: r.read_i16::<BigEndian>()?,
    })
}

pub fn decode_0x22<R: Read>(r: &mut R) -> Result<Packet> {
    let chunk_x = r.read_i32::<BigEndian>()?;
    let chunk_z = r.read_i32::<BigEndian>()?;
    let record_count = r.read_i16::<BigEndian>()?;
    let data_size = r.read_i32::<BigEndian>()?;
    ensure!(data_size == record_count as i32 * 4);
    let mut records = Vec::with_capacity(record_count as usize);
    for _ in 0..record_count {
        records.push(BlockChangeRecord::from_u32(r.read_u32::<BigEndian>()?));
    }
    Ok(Packet::MultiBlockChange {
        chunk_x,
        chunk_z,
        records,
    })
}

pub fn decode_0x23<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::BlockChange {
        x: r.read_i32::<BigEndian>()?,
        y: r.read_u8()?,
        z: r.read_i32::<BigEndian>()?,
        block_id: r.read_varint()?,
        block_meta: r.read_u8()?,
    })
}

pub fn decode_0x29<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::SoundEffect {
        sound_name: r.read_varstring()?,
        pos_x: r.read_i32::<BigEndian>()?,
        pos_y: r.read_i32::<BigEndian>()?,
        pos_z: r.read_i32::<BigEndian>()?,
        volume: r.read_f32::<BigEndian>()?,
        pitch: r.read_u8()?,
    })
}

pub fn decode_0x2b<R: Read>(r: &mut R) -> Result<Packet> {
    let reason = r.read_u8()?;
    let value = r.read_f32::<BigEndian>()?;

    Ok(Packet::ChangeGameState(match reason {
        0 => GameState::InvalidBed,
        1 => GameState::EndRaining,
        2 => GameState::BeginRaining,
        3 => GameState::ChangeGamemode(value),
        4 => GameState::EnterCredits,
        5 => GameState::DemoMessages(value),
        6 => GameState::ArrowHittingPlayer,
        7 => GameState::FadeValue(value),
        8 => GameState::FadeTime(value),
        _ => anyhow::bail!("Invalid GameState!"),
    }))
}

pub fn decode_0x21<R: Read>(r: &mut R) -> Result<Packet> {
    let chunk_x = r.read_i32::<BigEndian>()?;
    let chunk_z = r.read_i32::<BigEndian>()?;
    let ground_up_continuous = r.read_u8()? != 0;
    let primary_bitmap = r.read_u16::<BigEndian>()?;
    let add_bitmap = r.read_u16::<BigEndian>()?;

    let data_length = r.read_i32::<BigEndian>()?;

    let mut data_compressed = vec![0u8; data_length as usize];
    r.read_exact(&mut data_compressed)?;

    let mut c = Cursor::new(&data_compressed);
    let mut z = ZlibDecoder::new(&mut c);

    let mut data = Vec::new();
    z.read_to_end(&mut data)?;

    // println!("{}", hex::encode(&data));

    Ok(Packet::ChunkData {
        chunk_x,
        chunk_z,
        ground_up_continuous,
        primary_bitmap,
        add_bitmap,
        data,
    })
}

pub fn decode_0x26<R: Read>(r: &mut R) -> Result<Packet> {
    let column_count = r.read_i16::<BigEndian>()?;
    let data_length = r.read_i32::<BigEndian>()?;
    let has_sky_light = r.read_u8()? == 1;

    let mut data_compressed = vec![0u8; data_length as usize];
    r.read_exact(&mut data_compressed)?;

    let mut meta = Vec::with_capacity(column_count as usize);
    for _ in 0..column_count {
        meta.push(ChunkMetadata {
            chunk_x: r.read_i32::<BigEndian>()?,
            chunk_z: r.read_i32::<BigEndian>()?,
            primary_bitmap: r.read_u16::<BigEndian>()?,
            add_bitmap: r.read_u16::<BigEndian>()?,
        });
    }

    let mut c = Cursor::new(&data_compressed);
    let mut z = ZlibDecoder::new(&mut c);

    let mut data = Vec::new();
    z.read_to_end(&mut data)?;

    // println!("{}", hex::encode(&data));

    Ok(Packet::MapChunkBulk {
        columns: column_count,
        has_sky_light,
        data,
        meta,
    })
}

pub fn decode_0x38<R: Read>(r: &mut R) -> Result<Packet> {
    Ok(Packet::PlayerListItem {
        player_name: r.read_varstring()?,
        online: r.read_u8()? == 1,
        ping: r.read_i16::<BigEndian>()?,
    })
}

pub fn decode_0x20<R: Read>(r: &mut R) -> Result<Packet> {
    let eid = r.read_i32::<BigEndian>()?;
    let count = r.read_i32::<BigEndian>()?;

    let mut properties = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let key = r.read_varstring()?;
        let value = r.read_f64::<BigEndian>()?;

        let list_length = r.read_i16::<BigEndian>()?;
        let mut modifiers = Vec::with_capacity(list_length as usize);
        for _ in 0..list_length {
            let uuid = r.read_u128::<BigEndian>()?;
            let amount = r.read_f64::<BigEndian>()?;
            let operation = r.read_u8()?;

            modifiers.push(EntityModifier {
                uuid,
                amount,
                operation,
            });
        }

        properties.push(EntityProperty {
            key,
            value,
            modifiers,
        });
    }

    Ok(Packet::EntityProperties { eid, properties })
}
