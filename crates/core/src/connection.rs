use std::os::fd::OwnedFd;

use futures_core::Stream;
use futures_sink::Sink;

use crate::{Message, ProtocolError};

pub trait Connection:
    Stream<Item = Result<Message, ProtocolError>>
    + Sink<Message, Error = ProtocolError>
    + Send
    + Sync
    + Unpin
{
    type Error: From<crate::ProtocolError>;

    fn fd(&mut self) -> Result<OwnedFd, <Self as Connection>::Error>;
}
