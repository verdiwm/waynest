#[derive(Debug)]
pub enum ProtocolError {
    InvalidSenderId,
    MissingFd,
    MalformedPayload,
    UnknownOpcode(u16),
    InvalidLength(usize),
    IoError(std::io::Error),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::InvalidSenderId => write!(f, "Sender object ID cannot be null"),
            ProtocolError::MissingFd => write!(f, "Missing fd from payload"),
            ProtocolError::MalformedPayload => write!(f, "Failed to decode malformed payload"),
            ProtocolError::UnknownOpcode(opcode) => {
                write!(f, "Received unsupported opcode: {}", opcode)
            }
            ProtocolError::InvalidLength(len) => write!(f, "Invalid payload length: {len} bytes"),
            ProtocolError::IoError(err) => write!(f, "I/O error during decoding: {err}"),
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProtocolError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> Self {
        ProtocolError::IoError(err)
    }
}
