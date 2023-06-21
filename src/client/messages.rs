use std::{default, io::Read};

use crate::{
    io::{Decode, DecodeFrom, Encode, Length},
    messages::PixelFormat,
    Error,
};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ClientMessageType {
    SetPixelFormat,
    #[default]
    Unassigned,
    SetEncodings,
    FramebufferUpdateRequest,
    KeyEvent,
    PointerEvent,
    ClientCutText,
    VmWare = 127,
    CarConnectivity,
    TightVncEnableContinuousUpdates = 150,
    ReplitAudioClientMessage = 245,
    DellEmc,
    ClientFence = 248,
    OliveCallControl,
    ColinDeanXvp,
    PierreOssmanSetDesktopSize,
    Tight,
    Gii,
    AnthonyLiguori = 255,
}

impl From<u8> for ClientMessageType {
    fn from(value: u8) -> Self {
        use ClientMessageType::*;
        match value {
            0 => SetPixelFormat,
            2 => SetEncodings,
            3 => FramebufferUpdateRequest,
            4 => KeyEvent,
            5 => PointerEvent,
            6 => ClientCutText,
            127 | 254 => VmWare,
            128 => CarConnectivity,
            150 => TightVncEnableContinuousUpdates,
            245 => ReplitAudioClientMessage,
            246 => DellEmc,
            248 => ClientFence,
            249 => OliveCallControl,
            250 => ColinDeanXvp,
            251 => PierreOssmanSetDesktopSize,
            252 => Tight,
            253 => Gii,
            255 => AnthonyLiguori,

            _ => Unassigned,
        }
    }
}
#[repr(u8)]
#[derive(Clone, PartialEq, PartialOrd, Default)]
pub enum ClientMessage {
    SetPixelFormat(SetPixelFormat),
    #[default]
    Unassigned,
    SetEncodings,
    FramebufferUpdateRequest,
    KeyEvent,
    PointerEvent,
    ClientCutText,
    VmWare = 127,
    CarConnectivity,
    TightVncEnableContinuousUpdates = 150,
    ReplitAudioClientMessage = 245,
    DellEmc,
    ClientFence = 248,
    OliveCallControl,
    ColinDeanXvp,
    PierreOssmanSetDesktopSize,
    Tight,
    Gii,
    AnthonyLiguori = 255,
}
/*
impl<R: Read> DecodeFrom<R> for ClientMessage {
    type Error = Error;
    fn decode_from(reader: &mut R) -> Result<Self, Self::Error> {
        let message_type = u8::decode_from(reader)?;
        match message_type {
            0 => Ok(ClientMessage::SetPixelFormat(SetPixelFormat::decode_from(
                reader,
            )?)),
        }
    }
}
*/

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
    fn decode(data: [u8; <Self as crate::io::Length>::LENGTH]) -> Result<Self, Self::Error> {
        //TODO: implement generic int from bytes (le, be...)

        Ok(FramebufferUpdateRequest {
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

#[derive(Clone, PartialEq, PartialOrd)]

pub struct SetPixelFormat {
    pixel_format: PixelFormat,
}
impl Length for SetPixelFormat {
    const LENGTH: usize = 19;
}

impl Decode for SetPixelFormat {
    type Error = Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
        Ok(Self {
            pixel_format: PixelFormat::decode(data[3..].try_into().unwrap())?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct SetEncodings {
    number_of_encodings: u16,
}
