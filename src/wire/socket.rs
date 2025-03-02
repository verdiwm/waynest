use std::{
    io::{self, IoSlice, IoSliceMut},
    os::{
        fd::{BorrowedFd, RawFd},
        unix::net::UnixStream,
    },
    pin::Pin,
    task::Poll,
};

use futures_util::{Sink, Stream, ready};
use pin_project_lite::pin_project;
use rustix::{
    fd::IntoRawFd,
    net::{
        RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer,
        SendAncillaryMessage, SendFlags, recvmsg, sendmsg,
    },
};
use tokio::io::{AsyncRead, AsyncWrite, unix::AsyncFd};
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::trace;

use super::{DecodeError, Message};

pin_project! {
    pub struct Socket {
        #[pin]
        framed: Framed<StreamWrapper, MessageCodec>,
    }
}

impl Socket {
    pub fn new(stream: UnixStream) -> io::Result<Self> {
        Ok(Self {
            framed: Framed::new(
                StreamWrapper {
                    stream: AsyncFd::new(stream)?,
                    read_fds: Vec::new(),
                    write_fds: Vec::new(),
                },
                MessageCodec,
            ),
        })
    }
}

struct StreamWrapper {
    stream: AsyncFd<UnixStream>,
    read_fds: Vec<RawFd>,
    write_fds: Vec<RawFd>,
}

impl AsyncRead for StreamWrapper {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self.stream.poll_read_ready(cx))?;

            let mut cmsg_space = vec![0; rustix::cmsg_space!(ScmRights(28))];
            let mut ancillary_buf = RecvAncillaryBuffer::new(&mut cmsg_space);

            let unfilled = buf.initialize_unfilled();

            match guard.try_io(|stream| {
                recvmsg(
                    stream,
                    &mut [IoSliceMut::new(unfilled)],
                    &mut ancillary_buf,
                    RecvFlags::DONTWAIT | RecvFlags::CMSG_CLOEXEC,
                )
                .map_err(|errno| io::Error::from_raw_os_error(errno.raw_os_error()))
            }) {
                Ok(Ok(msg)) => {
                    for message in ancillary_buf.drain() {
                        if let RecvAncillaryMessage::ScmRights(scm_rights) = message {
                            for fd in scm_rights {
                                let fd = fd.into_raw_fd();
                                trace!("receive file descriptor: {fd}");
                                self.read_fds.push(fd);
                            }
                        }
                    }

                    buf.advance(msg.bytes);
                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(_would_block) => continue,
            }
        }
    }
}

impl AsyncWrite for StreamWrapper {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            let fds = self.write_fds.as_slice();

            let mut cmsg_space = vec![0; rustix::cmsg_space!(ScmRights(fds.len()))];
            let mut ancillary_buf = SendAncillaryBuffer::new(&mut cmsg_space);

            ancillary_buf.push(SendAncillaryMessage::ScmRights(unsafe {
                core::slice::from_raw_parts(fds.as_ptr() as *const BorrowedFd, fds.len())
            }));

            let mut guard = ready!(self.stream.poll_write_ready(cx))?;

            match guard.try_io(|stream| {
                sendmsg(
                    stream,
                    &[IoSlice::new(buf)],
                    &mut ancillary_buf,
                    SendFlags::DONTWAIT | SendFlags::NOSIGNAL,
                )
                .map_err(|errno| io::Error::from_raw_os_error(errno.raw_os_error()))
            }) {
                Ok(result) => return Poll::Ready(result),
                Err(_would_block) => continue,
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        self.stream.get_ref().shutdown(std::net::Shutdown::Write)?;

        Poll::Ready(Ok(()))
    }
}

#[derive(Debug)]
struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = Message;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Message::decode(src)
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, message: Message, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        message.encode(dst);

        Ok(())
    }
}

impl Stream for Socket {
    type Item = Result<Message, DecodeError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut result = self.as_mut().project().framed.poll_next(cx);

        if let Poll::Ready(Some(Ok(message))) = &mut result {
            message.fds = std::mem::take(&mut self.framed.get_mut().read_fds);
        }

        result
    }
}

impl Sink<Message> for Socket {
    type Error = io::Error;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().framed.poll_ready(cx)
    }

    fn start_send(
        mut self: std::pin::Pin<&mut Self>,
        mut message: Message,
    ) -> Result<(), Self::Error> {
        self.framed.get_mut().write_fds = std::mem::take(&mut message.fds);

        self.project().framed.start_send(message)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().framed.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().framed.poll_close(cx)
    }
}
