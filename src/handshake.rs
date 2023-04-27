use std::io::{Read, Write};

use crate::io::*;
use crate::messages::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct SetPixelFormat {
    pixel_format: PixelFormat,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct SetEncodings {
    number_of_encodings: u16,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct FramebufferUpdateRequest {
    incremental: u8,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct KeyEvent {
    down_flag: bool,
    key: u32,
}

pub struct PointerEvent {
    button_mask: u8,
    x: u16,
    y: u16,
}
pub struct ClientCutText {
    text: String,
}

pub struct EnableContinuousUpdates {
    enable_flag: bool,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub struct ClientFence {
    flags: u32,
    payload: Vec<u8>,
}
