use std::os::fd::OwnedFd;

use futures_core::Stream;
use futures_sink::Sink;

use crate::{DecodeError, Message};

pub trait Connection:
    Stream<Item = Result<Message, DecodeError>> + Sink<Message, Error = DecodeError>
{
    fn fd(&mut self) -> Result<OwnedFd, DecodeError>;
}
