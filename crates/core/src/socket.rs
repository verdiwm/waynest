use std::{
    collections::VecDeque,
    io::{self, IoSlice, IoSliceMut},
    mem::MaybeUninit,
    os::{
        fd::{FromRawFd, IntoRawFd, OwnedFd, RawFd},
        unix::net::UnixStream,
    },
    task::Poll,
};

use futures_core::{Stream, ready};
use futures_sink::Sink;
use pin_project_lite::pin_project;
use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer,
    SendAncillaryMessage, SendFlags, recvmsg, sendmsg,
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf, unix::AsyncFd};
use tokio_util::codec::{Decoder, Encoder, Framed};

use crate::{Connection, Message, ProtocolError};

pin_project! {
    pub struct Socket {
        #[pin]
        inner: Framed<StreamWrapper, MessageCodec>,
        decode_fds: VecDeque<RawFd>,
    }
}

impl Socket {
    #[inline]
    pub fn new(stream: UnixStream) -> io::Result<Self> {
        Ok(Self {
            inner: Framed::new(StreamWrapper::new(stream)?, MessageCodec::new()),
            decode_fds: VecDeque::new(),
        })
    }
}

impl Connection for Socket {
    fn fd(&mut self) -> Result<std::os::unix::prelude::OwnedFd, ProtocolError> {
        self.decode_fds
            .pop_front()
            .map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
            .ok_or(ProtocolError::MalformedPayload)
    }
}

impl Stream for Socket {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.inner.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(msg))) => {
                let stream = Framed::get_mut(&mut this.inner);

                this.decode_fds.extend(stream.decode_fds.drain(..));

                Poll::Ready(Some(Ok(msg)))
            }
            res => res,
        }
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

    fn start_send(self: std::pin::Pin<&mut Self>, mut msg: Message) -> Result<(), Self::Error> {
        let mut this = self.project();

        let stream = Framed::get_mut(&mut this.inner);
        core::mem::swap(&mut msg.fds, &mut stream.encode_fds);

        this.inner.start_send(msg)
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

pin_project! {
    struct StreamWrapper {
        stream: AsyncFd<UnixStream>,
        pub decode_fds: VecDeque<RawFd>,
        pub encode_fds: VecDeque<RawFd>,

    }
}

impl StreamWrapper {
    pub fn new(stream: UnixStream) -> io::Result<Self> {
        Ok(Self {
            stream: AsyncFd::new(stream)?,
            decode_fds: VecDeque::new(),
            encode_fds: VecDeque::new(),
        })
    }
}

impl AsyncRead for StreamWrapper {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self.stream.poll_read_ready(cx))?;

            let mut cmsg_space = vec![MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(28))];
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
                                self.decode_fds.push_back(fd.into_raw_fd());
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
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, io::Error>> {
        loop {
            let mut fds = core::mem::take(&mut self.encode_fds);
            let fds = fds.make_contiguous();

            let mut cmsg_space =
                vec![MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(fds.len()))];
            let mut ancillary_buf = SendAncillaryBuffer::new(&mut cmsg_space);

            ancillary_buf.push(SendAncillaryMessage::ScmRights(unsafe {
                core::slice::from_raw_parts(fds.as_ptr().cast(), fds.len())
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

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;
        self.stream.get_ref().shutdown(std::net::Shutdown::Write)?;

        Poll::Ready(Ok(()))
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
