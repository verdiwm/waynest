use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::Stream;

use crate::server::Error;

pin_project! {
    pub struct Listener {
        unix_listener: UnixListener,
        // _lock: OwnedFd,
        socket_path: PathBuf,
        lock_path: PathBuf,
    }
}

impl Listener {
    pub fn new() -> Result<Self, Error> {
        // FIXME: add a proper error
        let runtime_dir: PathBuf = std::env::var("XDG_RUNTIME_DIR")
            .map_err(|_| Error::XdgError)?
            .into();

        #[allow(clippy::never_loop)]
        for i in 1..=32u8 {
            let path = runtime_dir.join(format!("wayland-{i}"));

            // FIXME: actually check

            return Self::new_with_path(path);
        }

        Err(Error::XdgError)
    }

    pub fn new_with_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        if !path.as_ref().exists() {
            // FIXME: add a proper error
            // return Err(Error::Internal);
        }

        // FIXME: actually implement this
        Ok(Self {
            unix_listener: UnixListener::bind(&path)?,
            // _lock: unsafe { OwnedFd::from_raw_fd(5) },
            socket_path: path.as_ref().to_path_buf(),
            lock_path: PathBuf::new(),
        })
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
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
