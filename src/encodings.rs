use std::fmt::Write;

use crate::{io::EncodeTo, messages::PixelFormat};

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(i32)]
#[non_exhaustive]
pub enum EncodingType {
    Raw = 0,
    CopyRect = 1,
    Rre = 2,
    CoRre = 4,
    Hextile = 5,
    Other(i32),
}
impl From<EncodingType> for i32 {
    fn from(value: EncodingType) -> Self {
        use EncodingType::*;
        match value {
            Raw => 0,
            CopyRect => 1,
            Rre => 2,
            CoRre => 4,
            Hextile => 5,
            Other(n) => n,
        }
    }
}

impl From<i32> for EncodingType {
    fn from(n: i32) -> Self {
        use EncodingType::*;
        match n {
            0 => Raw,
            1 => CopyRect,
            2 => Rre,
            4 => CoRre,
            5 => Hextile,
            _ => Other(n),
        }
    }
}

pub trait Encoding<W: Write>: EncodeTo<W> {
    fn get_type() -> EncodingType;
}

pub struct Raw {
    pixels: Vec<u8>,
}

pub struct CopyRect {
    pub x: u16,
    pub y: u16,
}

pub struct RRE {
    subrectangles: Vec<SubRectangle>, //background_pixel_value:
}

pub struct SubRectangle {
    //pixel_value:
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub struct Hextile {}
