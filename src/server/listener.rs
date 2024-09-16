use std::{
    io,
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;
use pin_project_lite::pin_project;
use rustix::fd::OwnedFd;
use tokio::net::{UnixListener, UnixStream};

pin_project! {
    pub struct Listener {
        unix_listener: UnixListener,
        _lock: OwnedFd,
        socket_path: PathBuf,
        lock_path: PathBuf,
    }
}

impl Listener {
    pub fn new() -> Self {
        todo!()
    }
}

impl Stream for Listener {
    type Item = io::Result<UnixStream>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<io::Result<UnixStream>>> {
        match self.unix_listener.poll_accept(cx) {
            Poll::Ready(Ok((stream, _))) => Poll::Ready(Some(Ok(stream))),
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
            Poll::Pending => Poll::Pending,
        }
    }
}
