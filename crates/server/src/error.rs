use std::{error, fmt, io};

use waynest::{ObjectId, ProtocolError};

#[derive(Debug)]
pub enum Error {
    XdgError,
    Protocol(ProtocolError),
    MissingObject(ObjectId),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Protocol(err) => write!(f, "Failed to decode message: {}", err),
            Error::MissingObject(id) => write!(f, "No object found with ID: {}", id),
            Error::XdgError => write!(f, "Failed to access XDG socket path"),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Protocol(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Self {
        Error::Protocol(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Protocol(err.into())
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
