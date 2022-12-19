use std::{fmt::Debug, io::Cursor};

use intmap::IntMap;
use minecrab_derive::Serializable;

use crate::varint::VarInt;

use super::{packet_helpers::Serializable, versions};

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct PositionIBI {
    pub x: i32,
    pub y: u8,
    pub z: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct PositionISI {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct PositionIII {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// General position type. Also used for 26-26-12 encoding
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32, // Using 32-bit Y so that we can safely convert from PositionIII
    pub z: i32,
}

impl Serializable for Position {
    fn read_from_versioned<R: std::io::Read>(r: &mut R, version: i32) -> anyhow::Result<Self> {
        let v = i64::read_from(r)?;

        if version < versions::PROTO_18W43A {
            let p = Position {
                x: (v >> 38) as i32,
                y: ((v >> 26) & 0xFFF) as i32,
                z: ((v << 38) >> 38) as i32,
            };

            Ok(p)
        } else {
            let p = Position {
                x: (v >> 38) as i32,
                y: ((v << 52) >> 52) as i32,
                z: ((v << 26) >> 38) as i32,
            };

            Ok(p)
        }
    }

    fn write_to_versioned<W: std::io::Write>(&self, w: &mut W, version: i32) -> anyhow::Result<()> {
        if version < versions::PROTO_18W43A {
            let v = ((self.x as i64 & 0x3FFFFFF) << 38)
                | ((self.y as i64 & 0xFFF) << 26)
                | (self.z as i64 & 0x3FFFFFF);

            i64::write_to(&v, w)
        } else {
            let v = (((self.x as i64) & 0x3FFFFFF) << 38)
                | (((self.z as i64) & 0x3FFFFFF) << 12)
                | ((self.y as i64) & 0xFFF);

            i64::write_to(&v, w)
        }
    }
}

impl From<PositionIBI> for Position {
    fn from(p: PositionIBI) -> Self {
        Self {
            x: p.x,
            y: p.y as i32,
            z: p.z,
        }
    }
}

impl From<PositionISI> for Position {
    fn from(p: PositionISI) -> Self {
        Self {
            x: p.x,
            y: p.y as i32,
            z: p.z,
        }
    }
}

impl From<PositionIII> for Position {
    fn from(p: PositionIII) -> Self {
        Self {
            x: p.x,
            y: p.y as i32,
            z: p.z,
        }
    }
}

// TODO: better variants?
#[allow(dead_code)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum GameState {
    #[default]
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

#[allow(dead_code)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum EntityAnimation {
    #[default]
    SwingArm = 0,
    DamageAnimation = 1,
    LeaveBed = 2,
    EatFood = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
    Unknown = 102,
    Crouch = 104,
    Uncrouch = 105,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct EntityProperty {
    pub key: String,
    pub value: f64,
    pub modifiers: PrefixedVec<EntityModifier, i16>,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct EntityModifier {
    pub uuid: u128,
    pub amount: f64,
    pub operation: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct ChunkMetadata {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub primary_bitmap: u16,
    pub add_bitmap: u16,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct ChunkMetadata_47 {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub bitmap: u16,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockChangeRecord_5 {
    pub block_meta: u8,
    pub block_id: u16,
    pub y: u8,
    pub z: u8,
    pub x: u8,
}

impl Serializable for BlockChangeRecord_5 {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let v = u32::read_from(r)?;
        Ok(Self {
            block_meta: (v & 0x0f) as u8,
            block_id: ((v >> 4) & 0x0fff) as u16,
            y: ((v >> 16) & 0xff) as u8,
            z: ((v >> 24) & 0x0f) as u8,
            x: ((v >> 28) & 0x0f) as u8,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct BlockChangeRecord_47 {
    pub pos_horizontal: u8,
    pub y: u8,
    pub block_id: VarInt,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct EntitySpawnProperty_5 {
    pub name: String,
    pub value: String,
    pub signature: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Slot {
    pub item_id: i16,
    pub item_count: Option<u8>,
    pub item_damage: Option<i16>,
    pub data: Option<nbt::Blob>,
}

impl Serializable for Slot {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let mut s = Self {
            item_id: i16::read_from(r)?,
            ..Default::default()
        };

        if s.item_id != -1 {
            s.item_count = Some(u8::read_from(r)?);
            s.item_damage = Some(i16::read_from(r)?);

            let nbt_length = i16::read_from(r)?;
            if nbt_length != -1 {
                let mut data = vec![0u8; nbt_length as usize];
                r.read_exact(&mut data)?;
                let mut c = Cursor::new(&data);
                let nbt_data = CompressedGzData::<nbt::Blob>::read_from(&mut c)?.0;
                s.data = Some(nbt_data);
            }
        }

        Ok(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixedVec<T: Serializable, C: Serializable + TryInto<isize>> {
    pub data: Vec<T>,
    _m: std::marker::PhantomData<C>,
}

impl<T: Serializable, C: Serializable + TryInto<isize>> Serializable for PrefixedVec<T, C> {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let count = C::read_from(r)?;
        let icount: isize = if let Ok(s) = count.try_into() {
            s
        } else {
            anyhow::bail!("Failed to cast count type for PrefixedVec to isize")
        };
        let mut v = PrefixedVec::default();

        for _ in 0..icount {
            v.data.push(T::read_from(r)?);
        }

        Ok(v)
    }
}

impl<T: Serializable, C: Serializable + TryInto<isize>> Default for PrefixedVec<T, C> {
    fn default() -> Self {
        Self {
            _m: std::marker::PhantomData,
            data: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataField {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Slot(Slot),
    // Position(PositionIII),
}

impl MetadataField {
    pub fn get_type(&self) -> u8 {
        match self {
            MetadataField::Byte(_) => 0,
            MetadataField::Short(_) => 1,
            MetadataField::Int(_) => 2,
            MetadataField::Float(_) => 3,
            MetadataField::String(_) => 4,
            MetadataField::Slot(_) => 5,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EntityMeta {
    pub meta: IntMap<MetadataField>,
}

impl Serializable for EntityMeta {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let mut m = EntityMeta::default();

        for _ in 0..256 {
            let tb = u8::read_from(r)?;
            if tb == 0x7f {
                break;
            }

            let index = tb & 0x1f;
            let kind = (tb >> 5) & 0x07;

            let v = match kind {
                0 => MetadataField::Byte(Serializable::read_from(r)?),
                1 => MetadataField::Short(Serializable::read_from(r)?),
                2 => MetadataField::Int(Serializable::read_from(r)?),
                3 => MetadataField::Float(Serializable::read_from(r)?),
                4 => MetadataField::String(Serializable::read_from(r)?),
                5 => MetadataField::Slot(Serializable::read_from(r)?),
                _ => anyhow::bail!("Invalid metadata type {}", kind),
            };

            m.meta.insert(index as u64, v);
        }

        Ok(m)
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        for (k, v) in self.meta.iter() {
            let kind = v.get_type();
            u8::write_to(&(*k as u8 | (kind << 5)), w)?;

            match v {
                MetadataField::Byte(v) => Serializable::write_to(v, w)?,
                MetadataField::Short(v) => Serializable::write_to(v, w)?,
                MetadataField::Int(v) => Serializable::write_to(v, w)?,
                MetadataField::Float(v) => Serializable::write_to(v, w)?,
                MetadataField::String(v) => Serializable::write_to(v, w)?,
                MetadataField::Slot(v) => Serializable::write_to(v, w)?,
            };
        }

        u8::write_to(&0x7f, w)?;

        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct ExplosionRecord_5 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Debug, Default, Clone, PartialEq, Serializable)]
pub struct StatisticsEntry {
    pub name: String,
    pub value: VarInt,
}

impl Serializable for nbt::Blob {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        Ok(nbt::Blob::from_reader(r)?)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CompressedGzData<T: Serializable>(T);

impl<T: Serializable> Serializable for CompressedGzData<T> {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let mut gzwrapper = flate2::read::GzDecoder::new(r);

        Ok(Self(T::read_from(&mut gzwrapper)?))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjectData {
    pub data: i32,
    pub velocity: Option<(i16, i16, i16)>,
}

impl Serializable for ObjectData {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let data = i32::read_from(r)?;
        let velocity = if data != 0 {
            Some((i16::read_from(r)?, i16::read_from(r)?, i16::read_from(r)?))
        } else {
            None
        };

        Ok(Self { data, velocity })
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct FixedPoint32(pub f64);

impl Serializable for FixedPoint32 {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let v = i32::read_from(r)?;

        Ok(Self(v as f64 / 32.0))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        ((self.0 * 32.0) as i32).write_to(w)
    }
}

impl Debug for FixedPoint32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct FixedPoint8(pub f64);

impl Serializable for FixedPoint8 {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let v = i8::read_from(r)?;

        Ok(Self(v as f64 / 32.0))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        ((self.0 * 32.0) as i8).write_to(w)
    }
}

impl Debug for FixedPoint8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct FixedPoint16(pub f64);

impl Serializable for FixedPoint16 {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let v = i8::read_from(r)?;

        Ok(Self(v as f64 / 32.0))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        ((self.0 * 32.0) as i16).write_to(w)
    }
}

impl Debug for FixedPoint16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
