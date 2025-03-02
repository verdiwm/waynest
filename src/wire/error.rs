#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Malformed header")]
    MalformedHeader,
    #[error("Received invalid payload length: {0}")]
    InvalidLength(usize),
    #[error("Malformed payload")]
    MalformedPayload,
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}
