pub mod pixel_format;
use std::{io::Read, io::Write};

pub use pixel_format::*;

pub mod security;
pub use security::*;

pub mod init;
pub use init::*;

pub mod framebuffer_update;
pub use framebuffer_update::*;

pub mod framebuffer_update_request;
pub use framebuffer_update_request::*;

pub mod key_event;
pub use key_event::*;

pub mod set_encodings;
pub use set_encodings::*;

use crate::io::*;

pub trait Message<R: Read, W: Write>: EncodeTo<W> + DecodeFrom<R> {
    const NUMBER: u8;
}

const RFB33: [u8; 12] = *b"RFB 003.003\n";
const RFB37: [u8; 12] = *b"RFB 003.007\n";
const RFB38: [u8; 12] = *b"RFB 003.008\n";

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub enum Version {
    Rfb33,
    Rfb37,
    #[default]
    Rfb38,
}
impl Length for Version {
    const LENGTH: usize = 12;
}
impl Decode for Version {
    type Error = crate::error::Error;
    fn decode(data: [u8; 12]) -> Result<Self, Self::Error> {
        match data {
            RFB33 => Ok(Self::Rfb33),
            RFB37 => Ok(Self::Rfb37),
            RFB38 => Ok(Self::Rfb38),
            _ => Err(Self::Error::UnsupportedVersion),
        }
    }
}

impl Encode for Version {
    type Error = crate::error::Error;
    fn encode(self) -> Result<[u8; 12], Self::Error> {
        use Version::*;
        Ok(match self {
            Rfb33 => RFB33,
            Rfb37 => RFB37,
            Rfb38 => RFB38,
        })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum MessageType {
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

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        use MessageType::*;
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
