use std::io::{Read, Write};

use crate::{error::Error, io::*};
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub enum SecurityType {
    Invalid,
    None,
    VncAuthentication,
    RealVnc,
    Tight = 16,
    Ultra,
    Tls,
    VeNCrypt,
    GtkVncSasl,
    Md5HashAuthentication,
    ColinDeanXvp,
    SecureTunnel,
    IntegratedSsh,
    #[default]
    Unassigned,
    AppleInc = 30,
    TightUnixLoginAuthentication,
}

impl Decode for SecurityType {
    type Error = crate::error::Error;
    fn decode(data: [u8; 1]) -> Result<Self, Self::Error> {
        use SecurityType::*;
        let value = data[0];
        Ok(match value {
            0 => Invalid,
            1 => None,
            2 => VncAuthentication,
            3..15 | 128 | 130..134 | 192 => RealVnc,
            16 => Tight,
            17 => Ultra,
            18 => Tls,
            19 => VeNCrypt,
            20 => GtkVncSasl,
            21 => Md5HashAuthentication,
            22 => ColinDeanXvp,
            23 => SecureTunnel,
            24 => IntegratedSsh,
            30..36 => AppleInc,
            129 => TightUnixLoginAuthentication,
            _ => Unassigned,
        })
    }
}
impl Length for SecurityType {
    const LENGTH: usize = 1;
}

impl Encode for SecurityType {
    type Error = crate::Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
        Ok([self as u8])
    }
}

impl<R: Read> DecodeFrom<R> for Vec<SecurityType> {
    type Error = crate::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let len = u8::decode_from(reader)? as usize;
        let mut collection = Vec::with_capacity(len);

        for _ in 0..len {
            collection.push(SecurityType::decode_from(reader)?)
        }
        println!("Received: {collection:?}");
        Ok(collection)
    }
}

impl<W: Write> EncodeTo<W> for Vec<SecurityType> {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        println!("Sent: {self:?}");
        let len: u8 = self.len().try_into()?;
        len.encode_to(writer)?;

        for security_type in self {
            security_type.encode_to(writer)?;
        }
        Ok(1 + len as usize)
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SecurityResult {
    Ok,
    Failed,
    TooManyAttempts,
}
impl Length for SecurityResult {
    const LENGTH: usize = 4;
}

impl Decode for SecurityResult {
    type Error = Error;
    fn decode(data: [u8; 4]) -> Result<Self, Self::Error> {
        match data[0] {
            0 => Ok(Self::Ok),
            1 => Ok(Self::Failed),
            2 => Ok(Self::TooManyAttempts),
            _ => Err(Error::BadResponse),
        }
    }
}
impl Encode for SecurityResult {
    type Error = Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
        Ok((self as u32).to_be_bytes())
    }
}
