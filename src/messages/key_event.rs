use crate::io::{Decode, Encode, Length};

pub struct KeyEvent {
    down: bool,
    key: u32,
}

impl Length for KeyEvent {
    const LENGTH: usize = 8;
}

impl Decode for KeyEvent {
    type Error = crate::Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
        Ok(Self {
            down: data[1] != 0,
            key: u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
        })
    }
}

impl Encode for KeyEvent {
    type Error = crate::Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
        let key = self.key.to_be_bytes();
        Ok([4, self.down as u8, 0, 0, key[0], key[1], key[2], key[3]])
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Default, PartialOrd)]
pub enum Key {
    #[default]
    VoidSymbol = 0xFFFFFF,

    Space = 0x020,

    BackSpace = 0xFF08,
    Tab = 0xFF09,
    Linefeed = 0xFF0A,
    Clear = 0xFF0B,
    Return = 0xFF0D,
    Pause = 0xFF13,
    ScrollLock = 0xFF14,
    SysReq = 0xFF15,
    Escape = 0xFF1B,

    Delete = 0xFFFF,

    Print = 0xFF61,
    DeadGrave = 0xFE50,
    DeadAcute = 0xFE51,
    DeadCircumflex = 0xFE52,
    FeadTilde = 0xFE53,
    DeadDiaeresis = 0xFE57,

    Home = 0xFF50,
    Left = 0xFF51,
    Up = 0xFF52,
    Right = 0xFF53,
    Down = 0xFF54,
    PageUp = 0xFF55,
    PageDown = 0xFF56,
    End = 0xFF57,
    Begin = 0xFF58,

    Select = 0xFF60,
    Execute = 0xFF62,
    Insert = 0xFF63,

    Cancel = 0xFF69,
    Help = 0xFF6A,
    Break = 0xFF6B,
    NumLock = 0xFF7F,

    KeypadSpace = 0xFF80,
    KeypadTab = 0xFF89,
    KeypadEnter = 0xFF8D,

    KeypadHome = 0xFF95,
    KeypadLeft = 0xFF96,
    KeypadUp = 0xFF97,
    KeypadRight = 0xFF98,
    KeypadDown = 0xFF99,
    KeypadPrior = 0xFF9A,
    KeypadNext = 0xFF9B,
    KeypadEnd = 0xFF9C,
    KeypadBegin = 0xFF9D,
    KeypadInsert = 0xFF9E,
    KeypadDelete = 0xFF9F,
    KeypadEqual = 0xFFBD,
    KeypadMultiply = 0xFFAA,
    KeypadAdd = 0xFFAB,
    KeypadSeparator = 0xFFAC,
    KeypadSubtract = 0xFFAD,
    KeypadDecimal = 0xFFAE,
    KeypadDivide = 0xFFAF,

    Keypad0 = 0xFFB0,
    Keypad1 = 0xFFB1,
    Keypad2 = 0xFFB2,
    Keypad3 = 0xFFB3,
    Keypad4 = 0xFFB4,
    Keypad5 = 0xFFB5,
    Keypad6 = 0xFFB6,
    Keypad7 = 0xFFB7,
    Keypad8 = 0xFFB8,
    Keypad9 = 0xFFB9,

    F1 = 0xFFBE,
    F2 = 0xFFBF,
    F3 = 0xFFC0,
    F4 = 0xFFC1,
    F5 = 0xFFC2,
    F6 = 0xFFC3,
    F7 = 0xFFC4,
    F8 = 0xFFC5,
    F9 = 0xFFC6,
    F10 = 0xFFC7,
    F11 = 0xFFC8,
    F12 = 0xFFC9,
    F13 = 0xFFCA,
    F14 = 0xFFCB,
    F15 = 0xFFCC,
    F16 = 0xFFCD,
    F17 = 0xFFCE,
    F18 = 0xFFCF,
    F19 = 0xFFD0,
    F20 = 0xFFD1,
    F21 = 0xFFD2,
    F22 = 0xFFD3,
    F23 = 0xFFD4,
    F24 = 0xFFD5,

    ShiftL = 0xFFE1,
    ShiftR = 0xFFE2,
    ControlL = 0xFFE3,
    ControlR = 0xFFE4,
    CapsLock = 0xFFE5,
    ShiftLock = 0xFFE6,
    MetaL = 0xFFE7,
    MetaR = 0xFFE8,
    AltL = 0xFFE9,
    AltR = 0xFFEA,
}

impl Length for Key {
    const LENGTH: usize = 4;
}
impl Encode for Key {
    type Error = crate::Error;
    fn encode(self) -> Result<[u8; <Self as Length>::LENGTH], Self::Error> {
        Ok((self as u32).to_be_bytes())
    }
}
impl Decode for Key {
    type Error = crate::Error;
    fn decode(data: [u8; <Self as Length>::LENGTH]) -> Result<Self, Self::Error> {
        use Key::*;
        Ok(match u32::from_be_bytes(data) {
            0x020 => Space,

            0xFF08 => BackSpace,
            0xFF09 => Tab,
            0xFF0A => Linefeed,
            0xFF0B => Clear,
            0xFF0D => Return,
            0xFF13 => Pause,
            0xFF14 => ScrollLock,
            0xFF15 => SysReq,
            0xFF1B => Escape,

            0xFFFF => Delete,

            0xFF61 => Print,
            0xFE50 => DeadGrave,
            0xFE51 => DeadAcute,
            0xFE52 => DeadCircumflex,
            0xFE53 => FeadTilde,
            0xFE57 => DeadDiaeresis,

            0xFF50 => Home,
            0xFF51 => Left,
            0xFF52 => Up,
            0xFF53 => Right,
            0xFF54 => Down,
            0xFF55 => PageUp,
            0xFF56 => PageDown,
            0xFF57 => End,
            0xFF58 => Begin,

            0xFF60 => Select,
            0xFF62 => Execute,
            0xFF63 => Insert,

            0xFF69 => Cancel,
            0xFF6A => Help,
            0xFF6B => Break,
            0xFF7F => NumLock,

            0xFF80 => KeypadSpace,
            0xFF89 => KeypadTab,
            0xFF8D => KeypadEnter,

            0xFF95 => KeypadHome,
            0xFF96 => KeypadLeft,
            0xFF97 => KeypadUp,
            0xFF98 => KeypadRight,
            0xFF99 => KeypadDown,
            0xFF9A => KeypadPrior,
            0xFF9B => KeypadNext,
            0xFF9C => KeypadEnd,
            0xFF9D => KeypadBegin,
            0xFF9E => KeypadInsert,
            0xFF9F => KeypadDelete,
            0xFFBD => KeypadEqual,
            0xFFAA => KeypadMultiply,
            0xFFAB => KeypadAdd,
            0xFFAC => KeypadSeparator,
            0xFFAD => KeypadSubtract,
            0xFFAE => KeypadDecimal,
            0xFFAF => KeypadDivide,

            0xFFB0 => Keypad0,
            0xFFB1 => Keypad1,
            0xFFB2 => Keypad2,
            0xFFB3 => Keypad3,
            0xFFB4 => Keypad4,
            0xFFB5 => Keypad5,
            0xFFB6 => Keypad6,
            0xFFB7 => Keypad7,
            0xFFB8 => Keypad8,
            0xFFB9 => Keypad9,

            0xFFBE => F1,
            0xFFBF => F2,
            0xFFC0 => F3,
            0xFFC1 => F4,
            0xFFC2 => F5,
            0xFFC3 => F6,
            0xFFC4 => F7,
            0xFFC5 => F8,
            0xFFC6 => F9,
            0xFFC7 => F10,
            0xFFC8 => F11,
            0xFFC9 => F12,
            0xFFCA => F13,
            0xFFCB => F14,
            0xFFCC => F15,
            0xFFCD => F16,
            0xFFCE => F17,
            0xFFCF => F18,
            0xFFD0 => F19,
            0xFFD1 => F20,
            0xFFD2 => F21,
            0xFFD3 => F22,
            0xFFD4 => F23,
            0xFFD5 => F24,

            0xFFE1 => ShiftL,
            0xFFE2 => ShiftR,
            0xFFE3 => ControlL,
            0xFFE4 => ControlR,
            0xFFE5 => CapsLock,
            0xFFE6 => ShiftLock,
            0xFFE7 => MetaL,
            0xFFE8 => MetaR,
            0xFFE9 => AltL,
            0xFFEA => AltR,
            _ => VoidSymbol,
        })
    }
}
