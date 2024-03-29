use crate::varint::{ReadProtoExt, WriteProtoExt};

#[allow(unused_variables)]
pub trait Serializable: Sized {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        unimplemented!()
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn read_from_versioned<R: std::io::Read>(r: &mut R, version: i32) -> anyhow::Result<Self> {
        Self::read_from(r)
    }

    fn write_to_versioned<W: std::io::Write>(&self, w: &mut W, version: i32) -> anyhow::Result<()> {
        self.write_to(w)
    }
}

#[macro_export]
macro_rules! packet_structs {
    ($($state:ident {
        $($dir:ident {
            $(
                packet $name:ident {
                    $($field:ident: $field_type:ty
                        $(> when ($cond:expr))?
                        $(> vec ($count_var:ident))?, )*
                }
            )*
        })+
    })+) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum Packet {
        $($($($name($state::$dir::$name),)*)+)+
        }

        $(
            pub mod $state {
                $(
                    pub mod $dir {
                        #![allow(unused_imports)]
                        use crate::net::packet_helpers::*;
                        use crate::net::types::*;
                        use crate::varint::{VarInt, VarLong};

                        $(

                            #[derive(Default, Debug, Clone, PartialEq)]
                            pub struct $name {
                                $(
                                    pub $field: $field_type,
                                )*
                            }

                            #[allow(unused_mut)]
                            impl Serializable for $name {
                                fn read_from_versioned<R: std::io::Read>(r: &mut R, version: i32) -> anyhow::Result<Self> {
                                    let _ = r;
                                    let mut p = $name::default();

                                    $(
                                        $(
                                            for _ in 0..p.$count_var {
                                                p.$field.push(Serializable::read_from_versioned(r, version)?);
                                            }

                                            // ! This is some dark magic used to disable the block below this one when we're reading a vec
                                            #[cfg(not)]
                                        )?
                                        if true $(&& ($cond(&p)))? {
                                            p.$field = Serializable::read_from_versioned(r, version)?;
                                        }
                                    )*

                                    Ok(p)
                                }

                                fn write_to_versioned<W: std::io::Write>(&self, w: &mut W, version: i32) -> anyhow::Result<()> {
                                    let _ = w;
                                    let mut _self = self.clone();
                                    $(
                                        $(
                                            _self.$count_var = _self.$field.len() as _;
                                            for v in &_self.$field {
                                                v.write_to_versioned(w, version)?;
                                            }

                                            #[cfg(not)]
                                        )?
                                        if true $(&& ($cond(&_self)))? {
                                            _self.$field.write_to_versioned(w, version)?;
                                        }
                                    )*

                                    Ok(())
                                }
                            }
                        )*
                    }
                )*
            }
        )*
    };
}

#[macro_export]
macro_rules! packet_ids {
    (version $version:expr,
        $($state:ident $tstate:ident {
        $($dir:ident $tdir:ident {
            $(
                $id:expr => $name:ident,
            )*
        })+
    })+) => {
        use crate::net::packet_helpers::Serializable;
        use crate::net::{ConnectionState, PacketDirection};

        pub fn decode_packet(p: &crate::net::codec::RawPacket, state: crate::net::ConnectionState, dir: crate::net::PacketDirection) -> anyhow::Result<crate::net::packets::Packet> {
            let mut reader = std::io::Cursor::new(&p.data);
            let r = match state {
                    $( // State
                    ConnectionState::$tstate => {
                        match dir {
                            $( // Direction
                                PacketDirection::$tdir => {
                                    match p.id {
                                        $(
                                            $id => crate::net::packets::Packet::$name(Serializable::read_from_versioned(&mut reader, $version)?),
                                        )*
                                        _ => anyhow::bail!("No mapping found for {:?}bound packet 0x{:x} in state {:?}", $tdir, p.id, $tstate)
                                    }
                                }
                            )*
                        }
                    }
                )*
                };



            if (reader.position() as usize) < p.data.len() {
                warn!(
                    "Packet data overrun! Packet with ID 0x{:x} has {} bytes left!",
                    p.id,
                    (p.data.len() - reader.position() as usize)
                );
            }

            Ok(r)
        }

        pub fn encode_packet(p: &crate::net::packets::Packet, state: crate::net::ConnectionState, dir: crate::net::PacketDirection) -> anyhow::Result<crate::net::codec::RawPacket> {
            let mut rp = crate::net::codec::RawPacket {
                id: 0,
                data: vec![],
            };

            let mut writer = std::io::Cursor::new(&mut rp.data);
            // println!("Writing {:?} {:?} {:?}", dir, state, p);

            match state {
                    $( // State
                    ConnectionState::$tstate => {
                        match dir {
                            $( // Direction
                                PacketDirection::$tdir => {
                                    match p {
                                        $(
                                            crate::net::packets::Packet::$name(p) => {
                                                rp.id = $id;
                                                p.write_to_versioned(&mut writer, $version)?;
                                            },
                                        )*
                                        _ => anyhow::bail!("No mapping found for {:?}bound packet {:?} in state {:?}", $tdir, p, $tstate)
                                    }
                                }
                            )*
                        }
                    }
                )*
                };

            Ok(rp)
        }
    };
}

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;
macro_rules! serializable_primitive_impl {
    ($t:ident, $read:ident, $write:ident, multibyte) => {
        impl Serializable for $t {
            fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
                Ok(r.$read::<BigEndian>()?)
            }
            fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
                Ok(w.$write::<BigEndian>(*self)?)
            }
        }
    };
    ($t:ident, $read:ident, $write:ident, singlebyte) => {
        impl Serializable for $t {
            fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
                Ok(r.$read()?)
            }
            fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
                Ok(w.$write(*self)?)
            }
        }
    };
}

serializable_primitive_impl!(u8, read_u8, write_u8, singlebyte);
serializable_primitive_impl!(u16, read_u16, write_u16, multibyte);
serializable_primitive_impl!(u32, read_u32, write_u32, multibyte);
serializable_primitive_impl!(u64, read_u64, write_u64, multibyte);
serializable_primitive_impl!(u128, read_u128, write_u128, multibyte);
serializable_primitive_impl!(i8, read_i8, write_i8, singlebyte);
serializable_primitive_impl!(i16, read_i16, write_i16, multibyte);
serializable_primitive_impl!(i32, read_i32, write_i32, multibyte);
serializable_primitive_impl!(i64, read_i64, write_i64, multibyte);
serializable_primitive_impl!(i128, read_i128, write_i128, multibyte);
serializable_primitive_impl!(f32, read_f32, write_f32, multibyte);
serializable_primitive_impl!(f64, read_f64, write_f64, multibyte);

impl Serializable for bool {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        Ok(r.read_u8()? != 0)
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

impl Serializable for String {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        r.read_varstring()
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_varstring(self)
    }
}

impl Serializable for () {
    fn read_from<R: std::io::Read>(_r: &mut R) -> anyhow::Result<Self> {
        Ok(())
    }

    fn write_to<W: std::io::Write>(&self, _w: &mut W) -> anyhow::Result<()> {
        Ok(())
    }
}

impl<T: Serializable> Serializable for Option<T> {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        Ok(Some(Serializable::read_from(r)?))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        if let Some(s) = self {
            s.write_to(w)?;
        }

        Ok(())
    }
}

impl Serializable for Uuid {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 16];
        r.read(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn read_from<R: std::io::Read>(r: &mut R) -> anyhow::Result<Self> {
        let mut v = Vec::new();
        while let Ok(e) = T::read_from(r) {
            v.push(e);
        }
        Ok(v)
    }

    fn write_to<W: std::io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        for v in self {
            v.write_to(w)?;
        }
        Ok(())
    }
}
