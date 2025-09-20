#[derive(Debug)]
pub enum DecodeError {
    InvalidSenderId,
    InvalidLength(usize),
    MalformedPayload,
    IoError(std::io::Error),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::InvalidSenderId => write!(f, "Sender object ID cannot be null"),
            DecodeError::InvalidLength(len) => write!(f, "Invalid payload length: {len} bytes"),
            DecodeError::MalformedPayload => write!(f, "Failed to decode malformed payload"),
            DecodeError::IoError(err) => write!(f, "I/O error during decoding: {err}"),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> Self {
        DecodeError::IoError(err)
    }
}
