use std::os::fd::OwnedFd;

use anchovy::{AnchovyStream, IntoUnixStream, WAYLAND_FD_LIMIT};
use futures_core::Stream;
use futures_sink::Sink;
use pin_project_lite::pin_project;
use tokio_util::codec::{Decoder, Encoder, Framed};

use crate::{Message, ProtocolError};

pin_project! {
    #[repr(transparent)]
    pub struct Socket {
        #[pin]
        inner: Framed<AnchovyStream<WAYLAND_FD_LIMIT>, MessageCodec>,
    }
}

impl Socket {
    #[inline]
    pub fn new<S: IntoUnixStream>(stream: S) -> std::io::Result<Self> {
        Ok(Self {
            inner: Framed::new(AnchovyStream::new(stream)?, MessageCodec::new()),
        })
    }
}

impl Socket {
    pub fn pop_fd(&mut self) -> Result<OwnedFd, ProtocolError> {
        self.inner
            .get_mut()
            .read_queue_mut()
            .pop_front()
            .ok_or(ProtocolError::MissingFd)
    }

    pub fn push_fd(&mut self, fd: OwnedFd) {
        self.inner.get_mut().write_queue_mut().push_back(fd);
    }
}

impl Stream for Socket {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

impl Sink<Message> for Socket {
    type Error = ProtocolError;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: std::pin::Pin<&mut Self>, msg: Message) -> Result<(), Self::Error> {
        self.project().inner.start_send(msg)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

impl crate::Connection for Socket {
    type Error = ProtocolError;

    fn fd(&mut self) -> Result<OwnedFd, ProtocolError> {
        self.pop_fd()
    }

    fn push_fd(&mut self, fd: OwnedFd) {
        Socket::push_fd(self, fd);
    }
}

#[derive(Debug)]
struct MessageCodec {}

impl MessageCodec {
    const fn new() -> Self {
        Self {}
    }
}

impl Decoder for MessageCodec {
    type Item = Message;

    type Error = ProtocolError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Message::decode(src)
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = ProtocolError;

    fn encode(&mut self, msg: Message, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        msg.encode(dst);

        Ok(())
    }
}
