use std::io;

use crate::{DecodeError, ObjectId};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to decode message: {0}")]
    Decode(#[from] DecodeError),
    #[error("I/O operation failed: {0}")]
    IoError(#[from] io::Error),
    #[error("Received unsupported opcode: {0}")]
    UnknownOpcode(u16),
    #[error("No object found with ID: {0}")]
    MissingObject(ObjectId),
    #[error("Failed to access XDG socket path")]
    XdgError,
    #[error("{0}")]
    Custom(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
