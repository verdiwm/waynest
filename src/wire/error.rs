#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Sender object ID cannot be null")]
    InvalidSenderId,
    #[error("Invalid payload length: {0} bytes")]
    InvalidLength(usize),
    #[error("Failed to decode malformed payload")]
    MalformedPayload,
    #[error("I/O error during decoding: {0}")]
    IoError(#[from] std::io::Error),
}
