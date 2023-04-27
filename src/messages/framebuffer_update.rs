use std::io::{Read, Write};

use crate::io::{BigEndian, DecodeFrom, EncodeTo};

use super::EncodingType;
#[derive(Debug, PartialEq, PartialOrd)]
pub struct FramebufferUpdate {
    pub rectangles: Vec<Rectangle>,
}
#[derive(PartialEq, PartialOrd)]

pub struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub encoding_type: EncodingType,
    pub pixels: Vec<u8>,
}

impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Rectangle {{ x: {}, y: {}, width: {}, height: {}, encoding_type: {:?} }}",
            self.x, self.y, self.width, self.height, self.encoding_type
        ))
    }
}

impl<R: Read> DecodeFrom<R> for Rectangle {
    type Error = crate::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0; 8];

        reader.read_exact(&mut buf)?;
        let x = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        let y = u16::from_be_bytes(buf[2..4].try_into().unwrap());
        let width = u16::from_be_bytes(buf[4..6].try_into().unwrap());
        let height = u16::from_be_bytes(buf[6..8].try_into().unwrap());

        let encoding_type = EncodingType::decode_from(reader)?;
        let size = (width as usize) * (height as usize) * 4;
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;
        let data = Self {
            x,
            y,
            width,
            height,
            encoding_type,
            pixels: buf,
        };
        println!("Received: {data:?}");
        Ok(data)
    }
}

impl<W: Write> EncodeTo<W> for Rectangle {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        println!("Sent: {self:?}");
        let len = self.pixels.len();
        self.x.encode_to(writer)?;
        self.y.encode_to(writer)?;
        self.width.encode_to(writer)?;
        self.height.encode_to(writer)?;
        self.encoding_type.encode_to(writer)?;
        writer.write_all(&self.pixels)?;
        Ok(len * 2 + 8)
    }
}

impl<R: Read> DecodeFrom<R> for FramebufferUpdate {
    type Error = crate::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        <[u8; 2]>::decode_from(reader)?;

        let rectangles = Vec::<Rectangle>::decode_from(reader)?;
        let data = Self { rectangles };
        println!("Received: {:?}", data);
        Ok(data)
    }
}

impl<W: Write> EncodeTo<W> for FramebufferUpdate {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        println!("Sent: {self:?}");
        writer.write_all(&[0, 1])?;

        Ok(self.rectangles.encode_to(writer)? + 2)
    }
}

impl<R: Read> DecodeFrom<R> for Vec<Rectangle> {
    type Error = crate::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let len = u16::decode_from(reader)? as usize;
        let mut collection = Vec::with_capacity(len);
        for _ in 0..len {
            collection.push(Rectangle::decode_from(reader)?)
        }
        println!("Received: {:?}", collection);
        Ok(collection)
    }
}

impl<W: Write> EncodeTo<W> for Vec<Rectangle> {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        let len: u16 = self.len().try_into()?;
        len.encode_to(writer)?;
        let mut len = 2;
        for elem in self {
            len += elem.encode_to(writer)?;
        }
        Ok(len)
    }
}
