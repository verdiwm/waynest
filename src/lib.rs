#![cfg_attr(docsrs, feature(doc_cfg))]

// #[cfg(feature = "client")]
// #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
// pub mod client;
#[cfg(feature = "server")]
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub mod server;
pub mod wire;

#[cfg(any(feature = "client", feature = "server"))]
pub use async_trait;

#[cfg(feature = "tracing")]
pub use tracing::trace;

#[cfg(not(feature = "tracing"))]
macro_rules! trace {
    ($($_:tt)+) => {};
}

#[cfg(not(feature = "tracing"))]
pub(crate) use trace;
