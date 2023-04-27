use crate::io::{BigEndian, Decode, DecodeFrom, Encode, EncodeTo, Length};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SetEncodings {
    pub encodings: Vec<EncodingType>,
}

impl<W: Write> EncodeTo<W> for SetEncodings {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        println!("Sent: {self:?}");
        [2, 0].encode_to(writer)?;

        let len: u16 = self.encodings.len().try_into()?;
        len.encode_to(writer)?;

        self.encodings.encode_to(writer)
    }
}
impl<R: Read> DecodeFrom<R> for SetEncodings {
    type Error = crate::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        <[u8; 2]>::decode_from(reader)?;
        let data = Self {
            encodings: Vec::<EncodingType>::decode_from(reader)?,
        };
        println!("Received: {data:?}");
        Ok(data)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum EncodingType {
    Raw,
    CopyRect,
    Rre,
    CoRre = 4,
    Hextile,
}
impl Length for EncodingType {
    const LENGTH: usize = 4;
}

impl Encode for EncodingType {
    type Error = crate::Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
        Ok((self as i32).to_be_bytes())
    }
}

impl Decode for EncodingType {
    type Error = crate::Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
        let n = i32::from_be_bytes(data);
        use EncodingType::*;
        match n {
            0 => Ok(Raw),
            1 => Ok(CopyRect),
            2 => Ok(Rre),
            4 => Ok(CoRre),
            5 => Ok(Hextile),
            //TODO: add all encodings!!!
            _ => Ok(Raw),
        }
    }
}
impl<R: std::io::Read> DecodeFrom<R> for Vec<EncodingType> {
    type Error = crate::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let len = u16::decode_from(reader)? as usize;
        let mut collection = Vec::with_capacity(len);

        for _ in 0..len {
            collection.push(EncodingType::decode_from(reader)?)
        }
        Ok(collection)
    }
}

impl<W: std::io::Write> EncodeTo<W> for Vec<EncodingType> {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        let len: u16 = self.len().try_into()?;

        println!("Sent: {self:?}");
        len.encode_to(writer)?;

        for encoding_type in self {
            encoding_type.encode_to(writer)?;
        }
        Ok(2 + len as usize)
    }
}
