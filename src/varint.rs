use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::io;
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::net::packet_helpers::Serializable;

pub trait ReadProtoExt: Read {
    fn read_varint(&mut self) -> anyhow::Result<i32> {
        let mut result = 0i32;
        for i in 0..5 {
            let byte = self.read_u8()?;
            result |= ((byte & 0b0111_1111) as i32) << (7 * i);
            if byte & 0b1000_0000 == 0 {
                break;
            }
        }

        Ok(result)
    }

    fn read_varlong(&mut self) -> anyhow::Result<i64> {
        let mut result = 0i64;
        for i in 0..9 {
            let byte = self.read_u8()?;
            result |= ((byte & 0b0111_1111) as i64) << (7 * i);
            if byte & 0b1000_0000 == 0 {
                break;
            }
        }

        Ok(result)
    }

    fn read_varstring(&mut self) -> anyhow::Result<String> {
        let length = self.read_varint()?;
        let mut buf = vec![0u8; length as usize];
        self.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}

// Automatically implement ReadProtoExt for everything implementing Read
impl<R: io::Read + ?Sized> ReadProtoExt for R {}

pub trait WriteProtoExt: Write {
    fn write_varint(&mut self, value: i32) -> anyhow::Result<()> {
        let mut val = value as u32;
        loop {
            let mut temp = (val & 0b1111_1111) as u8;
            val >>= 7;
            if val != 0 {
                temp |= 0b1000_0000;
            }

            self.write_u8(temp)?;

            if val == 0 {
                return Ok(());
            }
        }
    }

    fn write_varlong(&mut self, value: i64) -> anyhow::Result<()> {
        let mut val = value as u64;
        loop {
            let mut temp = (val & 0b1111_1111) as u8;
            val >>= 7;
            if val != 0 {
                temp |= 0b1000_0000;
            }

            self.write_u8(temp)?;

            if val == 0 {
                return Ok(());
            }
        }
    }

    fn write_varstring(&mut self, value: &str) -> anyhow::Result<()> {
        self.write_varint(value.len() as i32)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }
}

// Automatically implement WriteProtoExt for everything implementing Write
impl<W: io::Write + ?Sized> WriteProtoExt for W {}

pub fn varint_len(value: i32) -> usize {
    (((i32::BITS as usize - value.leading_zeros() as usize) + 6) / 7).max(1)
}

// * Implementations for AsyncRead/AsyncWrite
pub async fn read_varint(reader: &mut OwnedReadHalf) -> io::Result<i32> {
    let mut result = 0i32;
    for i in 0..5 {
        let byte = reader.read_u8().await?;
        result |= ((byte & 0b0111_1111) as i32) << (7 * i);
        if byte & 0b1000_0000 == 0 {
            break;
        }
    }

    Ok(result)
}

pub async fn read_varstring(reader: &mut OwnedReadHalf) -> anyhow::Result<String> {
    let length = read_varint(reader).await?;
    let mut buf = vec![0u8; length as usize];
    reader.read_exact(&mut buf).await?;
    Ok(String::from_utf8(buf)?)
}

pub async fn write_varint(writer: &mut OwnedWriteHalf, value: i32) -> io::Result<()> {
    let mut val = value as u32;
    loop {
        let mut temp = (val & 0b1111_1111) as u8;
        val >>= 7;
        if val != 0 {
            temp |= 0b1000_0000;
        }

        writer.write_u8(temp).await?;

        if val == 0 {
            return Ok(());
        }
    }
}

pub async fn write_varstring(writer: &mut OwnedWriteHalf, value: &str) -> anyhow::Result<()> {
    write_varint(writer, value.len() as i32).await?;
    writer.write_all(value.as_bytes()).await?;
    Ok(())
}

#[derive(Default, Clone, PartialEq)]
pub struct VarInt(pub i32);

impl PartialEq<i32> for VarInt {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Serializable for VarInt {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        Ok(VarInt(r.read_varint()?))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_varint(self.0)?;
        Ok(())
    }
}

impl Debug for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Into<isize> for VarInt {
    fn into(self) -> isize {
        self.0 as isize
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct VarLong(pub i64);

impl Into<i64> for VarLong {
    fn into(self) -> i64 {
        self.0
    }
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Serializable for VarLong {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        Ok(VarLong(r.read_varlong()?))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_varlong(self.0)?;
        Ok(())
    }
}

impl Debug for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Into<isize> for VarLong {
    fn into(self) -> isize {
        self.0 as isize
    }
}
