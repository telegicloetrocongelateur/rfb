use std::{convert::Infallible, fmt::Display, io, string::FromUtf8Error, num::TryFromIntError};
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    UnsupportedVersion,
    BadResponse,
    Utf8Error,
    Infallible,
    IncompatibleSecurity,
    HandshakeFailed,
    UnsupportedEncoding,
    LengthTooBig
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("data")
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
    fn description(&self) -> &str {
        "RFB Error"
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::Utf8Error
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::Infallible
    }
}


impl From<TryFromIntError> for Error {
    fn from(_value: TryFromIntError) -> Self {
        Self::LengthTooBig
    }
}