#![allow(incomplete_features)]
#![feature(
    generic_const_exprs,
    array_try_from_fn,
    exclusive_range_pattern,
    associated_type_defaults
)]
pub mod client;
pub mod encodings;
pub mod error;
pub mod handshake;
pub mod io;
pub mod messages;
pub mod server;
use std::{io::Read, io::Write};

pub use error::Error;
use io::{Decode, DecodeFrom, Encode, EncodeTo, Length};
use messages::PixelFormat;

const RFB33: [u8; 12] = *b"RFB 003.003\n";
const RFB37: [u8; 12] = *b"RFB 003.007\n";
const RFB38: [u8; 12] = *b"RFB 003.008\n";

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub enum Version {
    Rfb33,
    Rfb37,
    #[default]
    Rfb38,
}
impl Length for Version {
    const LENGTH: usize = 12;
}
impl Decode for Version {
    type Error = crate::error::Error;
    fn decode(data: [u8; 12]) -> Result<Self, Self::Error> {
        match data {
            RFB33 => Ok(Self::Rfb33),
            RFB37 => Ok(Self::Rfb37),
            RFB38 => Ok(Self::Rfb38),
            _ => Err(Self::Error::UnsupportedVersion),
        }
    }
}

impl Encode for Version {
    type Error = crate::error::Error;
    fn encode(self) -> Result<[u8; 12], Self::Error> {
        use Version::*;
        Ok(match self {
            Rfb33 => RFB33,
            Rfb37 => RFB37,
            Rfb38 => RFB38,
        })
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]

pub struct ServerInit {
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub pixel_format: PixelFormat,
    pub name: String,
}

impl<R: Read> DecodeFrom<R> for ServerInit {
    type Error = crate::error::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let framebuffer_width = u16::decode_from(reader)?;
        let framebuffer_height = u16::decode_from(reader)?;
        let pixel_format = PixelFormat::decode_from(reader)?;
        let size = u32::decode_from(reader)? as usize;
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;
        let name = String::from_utf8(buf)?;

        Ok(Self {
            framebuffer_width,
            framebuffer_height,
            pixel_format,
            name,
        })
    }
}

impl<W: Write> EncodeTo<W> for ServerInit {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        self.framebuffer_width.encode_to(writer)?;
        self.framebuffer_height.encode_to(writer)?;
        self.pixel_format.encode_to(writer)?;
        (self.name.len() as u32).encode_to(writer)?;
        writer.write_all(self.name.as_bytes())?;

        Ok(20 + self.name.len())
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]

pub struct ClientInit {
    pub shared: bool,
}
impl Length for ClientInit {
    const LENGTH: usize = 1;
}
impl Encode for ClientInit {
    type Error = crate::error::Error;
    fn encode(self) -> Result<[u8; 1], Self::Error> {
        Ok([match self.shared {
            false => 0,
            true => 1,
        }])
    }
}

impl Decode for ClientInit {
    type Error = crate::error::Error;
    fn decode(data: [u8; 1]) -> Result<Self, Self::Error> {
        Ok(Self {
            shared: data[0] != 0,
        })
    }
}
