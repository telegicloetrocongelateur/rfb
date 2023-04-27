use std::array;
use std::convert::Infallible;
use std::io::{Read, Write};

pub trait Length {
    const LENGTH: usize;
}

pub trait Decode: Sized + Length {
    type Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error>;
}

pub trait DecodeFrom<R: Read>: Sized {
    type Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error>;
}

impl<R: Read, T: Decode + std::fmt::Debug> DecodeFrom<R> for T
where
    T::Error: From<std::io::Error>,
    [(); <Self as Length>::LENGTH]:,
{
    type Error = T::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0; <T as Length>::LENGTH];
        reader.read_exact(&mut buf)?;

        let t = Self::decode(buf)?;
        println!("Received: {:?}", t);
        Ok(t)
    }
}

impl<R: Read> DecodeFrom<R> for String {
    type Error = crate::error::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        let size = u32::from_be_bytes(buf) as usize;
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}

pub trait Encode: Length {
    type Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error>;
}
pub trait EncodeTo<W: Write>: Sized {
    type Error: From<std::io::Error>;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error>;
}

impl<W: Write, T: Encode + std::fmt::Debug> EncodeTo<W> for T
where
    T::Error: From<std::io::Error>,
    [(); <Self as Length>::LENGTH]:,
{
    type Error = T::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        println!("Sent : {:?}", self);
        let data = self.encode()?;
        let size = data.len();
        writer.write_all(&data)?;
        Ok(size)
    }
}

impl<W: Write, T: EncodeTo<W>, const N: usize> EncodeTo<W> for [T; N]
where
    T::Error: From<std::io::Error>,
{
    type Error = T::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        let mut len = 0;
        for elem in self {
            len += elem.encode_to(writer)?;
        }
        Ok(len)
    }
}
impl<T: Length, const N: usize> Length for [T; N] {
    const LENGTH: usize = T::LENGTH * N;
}

impl<R: Read, T: DecodeFrom<R>, const N: usize> DecodeFrom<R> for [T; N] {
    type Error = T::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        std::array::try_from_fn(|_| T::decode_from(reader))
    }
}
impl<W: Write> EncodeTo<W> for String {
    type Error = crate::Error;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
        let len: u32 = self.len().try_into()?;
        len.encode_to(writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(4 + self.len())
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BigEndian<T> {
    pub data: T,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]

pub struct LittleEndian<T> {
    pub data: T,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]

pub struct NativeEndian<T> {
    pub data: T,
}

trait Endian {}
impl<T> Endian for BigEndian<T> {}
impl<T> Endian for LittleEndian<T> {}
impl<T> Endian for NativeEndian<T> {}
crate::impl_decode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[macro_export]
macro_rules! impl_decode {
    ($($type: ident),*) => {
        $(
            impl Length for $type {
                const LENGTH: usize = ($type::BITS/ 8) as usize;
            }
            impl Decode for $type {
                type Error = $crate::Error;
                fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
                    Ok(
                        <$type>::from_be_bytes(data),
                    )
                }
            }
            impl Encode for $type {
                type Error = $crate::Error;
                fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
                    Ok(self.to_be_bytes())
                }
            }


            $crate::impl_endians!(BigEndian, $type, from_be_bytes, to_be_bytes);

        )*

    };
}
#[macro_export]

macro_rules! impl_endians {
    ($endian:ident, $type:ident, $from_bytes: ident, $to_bytes: ident) => {
        impl Length for $endian<$type> {
            const LENGTH: usize = <$type as Length>::LENGTH;
        }
        impl Decode for $endian<$type> {
            type Error = $crate::Error;
            fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
                Ok($endian {
                    data: <$type>::$from_bytes(data),
                })
            }
        }

        impl Encode for $endian<$type> {
            type Error = $crate::Error;
            fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
                Ok(self.data.$to_bytes())
            }
        }

        impl TryFrom<$endian<$type>> for usize {
            type Error = <$type as TryInto<usize>>::Error;
            fn try_from(value: $endian<$type>) -> Result<usize, Self::Error> {
                value.data.try_into()
            }
        }
    };
}
