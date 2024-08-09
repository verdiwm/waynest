#![feature(unix_socket_ancillary_data)]

mod error;

pub mod client;
pub mod server;
pub mod wire;

pub use error::{Error, Result};
