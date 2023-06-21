use std::io::{Read, Write};

pub trait Length {
    const LENGTH: usize;
}
pub type Be<T> = BigEndian<T>;
pub struct BigEndian<T> {
    pub inner: T,
}
impl<T> BigEndian<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Length> Length for BigEndian<T> {
    const LENGTH: usize = T::LENGTH;
}

pub type Le<T> = LittleEndian<T>;
pub struct LittleEndian<T> {
    pub inner: T,
}
impl<T> LittleEndian<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Length> Length for LittleEndian<T> {
    const LENGTH: usize = T::LENGTH;
}

pub trait Decode: Sized + Length {
    type Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error>;
}

pub trait DecodeFrom<R>: Sized {
    type Error;

    fn decode_from(reader: &mut R) -> Result<Self, Self::Error>;
}

impl<R: Read, T: Decode> DecodeFrom<R> for T
where
    T::Error: From<std::io::Error>,
    [(); <Self as Length>::LENGTH]:,
{
    type Error = T::Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let mut buf = [0; <T as Length>::LENGTH];
        reader.read_exact(&mut buf)?;

        Self::decode(buf)
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

pub trait Encode: Length + Sized {
    type Error;
    type Input = Self;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error>;
}
pub trait EncodeTo<W>: Sized {
    type Error: From<std::io::Error>;
    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error>;
}

impl<W: Write, T: Encode> EncodeTo<W> for T
where
    T::Error: From<std::io::Error>,
    [(); <Self as Length>::LENGTH]:,
{
    type Error = T::Error;

    fn encode_to(self, writer: &mut W) -> Result<usize, Self::Error> {
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

macro_rules! impl_numbers {
    ($($type: ident),*) => {
        $(
            impl Length for $type {
                const LENGTH: usize = (Self::BITS / 8) as usize;
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
            impl_endians!(BigEndian, $type, from_be_bytes, to_be_bytes);
            impl_endians!(LittleEndian, $type, from_le_bytes, to_le_bytes);
        )*
    };
}
#[macro_export]
macro_rules! impl_endians {
    ($endian:ident, $type:ident, $from_bytes: ident, $to_bytes: ident) => {
        impl Decode for $endian<$type> {
            type Error = $crate::Error;
            fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
                Ok($endian {
                    inner: <$type>::$from_bytes(data),
                })
            }
        }
        impl Encode for $endian<$type> {
            type Error = $crate::Error;
            fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
                Ok(self.inner.$to_bytes())
            }
        }
    };
}

impl_numbers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
