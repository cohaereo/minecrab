use anyhow::ensure;
use tokio::{
    io::AsyncReadExt,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::varint::{read_varint, varint_len, WriteProtoExt};

pub struct MinecraftCodec;

#[derive(Debug)]
pub struct RawPacket {
    pub id: i32,
    pub data: Vec<u8>,
}

impl MinecraftCodec {
    pub async fn read(reader: &mut OwnedReadHalf) -> anyhow::Result<RawPacket> {
        let len = read_varint(reader).await?;
        let id = read_varint(reader).await?;

        // println!("Reading {} - {} bytes", len, varint_len(id));
        let mut data = vec![0u8; len as usize - varint_len(id)];
        reader.read_exact(&mut data).await?;

        ensure!((data.len() + varint_len(id)) == len as usize);

        Ok(RawPacket { id, data })
    }

    pub async fn write(writer: &mut OwnedWriteHalf, packet: &RawPacket) -> anyhow::Result<()> {
        debug!("Writing {:?}", packet);
        let len = varint_len(packet.id) + packet.data.len();

        let mut buf = vec![];
        buf.write_varint(len as i32)?;
        buf.write_varint(packet.id)?;
        std::io::Write::write_all(&mut buf, &packet.data);

        // writer.write_all(&buf).await?;
        writer.try_write(&buf)?;

        Ok(())
    }
}
