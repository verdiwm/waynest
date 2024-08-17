mod error;
pub mod protocol;

use downcast_rs::{impl_downcast, DowncastSync};
pub use error::{Error, Result};

use crate::wire::{Message, Socket};

pub trait Dispatcher: DowncastSync {
    fn dispatch(&self, message: &Message);
}

impl_downcast!(sync Dispatcher);
