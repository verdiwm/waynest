use std::io;

use crate::wire::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal Error")]
    Internal,
    #[error("Malformed")]
    Malformed(#[from] DecodeError),
    #[error("Io Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Unknown Opcode")]
    UnknownOpcode,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
