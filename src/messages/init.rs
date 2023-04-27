use crate::io::*;
use crate::messages::PixelFormat;
use std::io::{Read, Write};

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

        let data = Self {
            framebuffer_width,
            framebuffer_height,
            pixel_format,
            name,
        };
        println!("Received: {data:?}");
        Ok(data)
    }
}

impl<W: Write> EncodeTo<W> for ServerInit {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        self.framebuffer_width.encode_to(writer)?;
        self.framebuffer_height.encode_to(writer)?;
        self.pixel_format.encode_to(writer)?;
        let len = self.name.encode_to(writer)?;

        Ok(20 + len)
    }
}
