use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

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

    fn write_varstring(&mut self, value: &str) -> anyhow::Result<()> {
        self.write_varint(value.len() as i32)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }
}

// Automatically implement WriteProtoExt for everything implementing Write
impl<W: io::Write + ?Sized> WriteProtoExt for W {}

pub fn varint_len(value: i32) -> usize {
    (i32::BITS as usize - value.leading_zeros() as usize)
        .div_ceil(7)
        .max(1)
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
