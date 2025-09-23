use std::{error, fmt, io};

use quick_xml::DeError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Decode(DeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "{}", err),
            Error::Decode(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            Error::Decode(err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<DeError> for Error {
    fn from(err: DeError) -> Self {
        Error::Decode(err)
    }
}
