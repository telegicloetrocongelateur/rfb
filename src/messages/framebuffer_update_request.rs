use crate::io::{Decode, Encode, Length};
#[derive(Debug, PartialEq, PartialOrd, Clone)]

pub struct FramebufferUpdateRequest {
    pub incremental: bool,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
impl Length for FramebufferUpdateRequest {
    const LENGTH: usize = 10;
}

impl Decode for FramebufferUpdateRequest {
    type Error = crate::Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
        //TODO: implement generic int from bytes (le, be...)
        Ok(Self {
            incremental: data[1] != 0,
            x: u16::from_be_bytes([data[2], data[3]]),
            y: u16::from_be_bytes([data[4], data[5]]),
            width: u16::from_be_bytes([data[6], data[7]]),
            height: u16::from_be_bytes([data[8], data[9]]),
        })
    }
}

impl Encode for FramebufferUpdateRequest {
    type Error = crate::Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
        let x = self.x.to_be_bytes();
        let y = self.y.to_be_bytes();
        let width = self.width.to_be_bytes();
        let height = self.height.to_be_bytes();

        Ok([
            3,
            self.incremental.into(),
            x[0],
            x[1],
            y[0],
            y[1],
            width[0],
            width[1],
            height[0],
            height[1],
        ])
    }
}
