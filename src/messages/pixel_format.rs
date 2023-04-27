use crate::io::*;
use std::convert::Infallible;
#[derive(Clone, PartialEq, PartialOrd, Debug)]

pub struct PixelFormat {
    pub bits_per_pixel: u8,
    pub depth: u8,
    pub big_endian_flag: bool,
    pub true_colour_flag: bool,
    pub red_max: u16,
    pub green_max: u16,
    pub blue_max: u16,
    pub red_shift: u8,
    pub green_shift: u8,
    pub blue_shift: u8,
}

impl Length for PixelFormat {
    const LENGTH: usize = 16;
}

impl Decode for PixelFormat {
    type Error = crate::Error;
    fn decode(data: [u8; 16]) -> Result<Self, Self::Error> {
        Ok(Self {
            bits_per_pixel: data[0],
            depth: data[1],
            big_endian_flag: data[2] != 0,
            true_colour_flag: data[3] != 0,
            red_max: u16::from_be_bytes([data[4], data[5]]),
            green_max: u16::from_be_bytes([data[6], data[7]]),
            blue_max: u16::from_be_bytes([data[8], data[9]]),
            red_shift: data[10],
            green_shift: data[11],
            blue_shift: data[12],
        })
    }
}

impl Encode for PixelFormat {
    type Error = crate::Error;
    fn encode(self) -> Result<[u8; 16], Self::Error> {
        let red_max = self.red_max.to_be_bytes();
        let green_max = self.green_max.to_be_bytes();
        let blue_max = self.blue_max.to_be_bytes();
        Ok([
            self.bits_per_pixel,
            self.depth,
            self.big_endian_flag.into(),
            self.true_colour_flag.into(),
            red_max[0],
            red_max[1],
            green_max[0],
            green_max[1],
            blue_max[0],
            blue_max[1],
            self.red_shift,
            self.green_shift,
            self.blue_shift,
            0,
            0,
            0,
        ])
    }
}
