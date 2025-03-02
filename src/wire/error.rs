#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Sender object id cannot be null")]
    InvalidSenderId,
    #[error("Received invalid payload length: {0}")]
    InvalidLength(usize),
    #[error("Malformed payload")]
    MalformedPayload,
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}
