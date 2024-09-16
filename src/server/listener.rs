use std::path::PathBuf;

use pin_project_lite::pin_project;
use rustix::fd::OwnedFd;
use tokio::net::UnixListener;

pin_project! {
    pub struct Listener {
        unix_listener: UnixListener,
        _lock: OwnedFd,
        socket_path: PathBuf,
        lock_path:PathBuf,
    }
}

impl Listener {
    pub fn new() -> Self {
        todo!()
    }
}
