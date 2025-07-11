use std::{
    os::fd::{FromRawFd, OwnedFd, RawFd},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

pub type FdWrapper = Arc<Wrapper>;

#[derive(Debug)]
pub struct Wrapper {
    fd: RawFd,
    popped: AtomicBool,
}

impl Wrapper {
    pub fn new(fd: RawFd) -> Arc<Self> {
        Arc::new(Wrapper {
            fd,
            popped: AtomicBool::new(false),
        })
    }
    pub fn as_owned_fd(&self) -> Option<OwnedFd> {
        if self.popped.load(Ordering::Relaxed) {
            None
        } else {
            self.popped.store(true, Ordering::Relaxed);
            Some(unsafe { OwnedFd::from_raw_fd(self.fd) })
        }
    }
}

impl std::hash::Hash for Wrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fd.hash(state);
    }
}
impl PartialOrd for Wrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fd.partial_cmp(&other.fd)
    }
}
impl Ord for Wrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fd.cmp(&other.fd)
    }
}
impl PartialEq for Wrapper {
    fn eq(&self, other: &Self) -> bool {
        self.fd == other.fd
    }
}
impl Eq for Wrapper {}
