mod error;
pub mod protocol;

pub use error::{Error, Result};

use crate::wire::Socket;

pub trait Dispatcher {
    fn socket(&mut self) -> &mut Socket;
}
