use std::{error, fmt, io};

use waynest::{DecodeError, ObjectId};

#[derive(Debug)]
pub enum Error {
    Decode(DecodeError),
    IoError(io::Error),
    UnknownOpcode(u16),
    MissingObject(ObjectId),
    XdgError,
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Decode(err) => write!(f, "Failed to decode message: {}", err),
            Error::IoError(err) => write!(f, "I/O operation failed: {}", err),
            Error::UnknownOpcode(opcode) => write!(f, "Received unsupported opcode: {}", opcode),
            Error::MissingObject(id) => write!(f, "No object found with ID: {}", id),
            Error::XdgError => write!(f, "Failed to access XDG socket path"),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Decode(err) => Some(err),
            Error::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Error::Decode(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
