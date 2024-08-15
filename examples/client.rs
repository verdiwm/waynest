use std::{borrow::Cow, os::unix::net::UnixStream, path::Path};

use anyhow::Result;
use futures_util::{SinkExt, TryStreamExt};
use waynest::wire::{Message, ObjectId, PayloadBuilder, Socket};

#[tokio::main]
async fn main() -> Result<()> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    let wayland_connection = std::env::var("WAYLAND_DISPLAY")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("wayland-0"));

    let socket_path = Path::new(&xdg_runtime_dir).join(wayland_connection.as_ref());

    let mut socket = Socket::new(UnixStream::connect(socket_path)?)?;

    let registry = unsafe { ObjectId::from_raw(2) };

    let (payload, fds) = PayloadBuilder::new().put_object(Some(registry)).build();

    socket
        .send(Message::new(ObjectId::DISPLAY, 1, payload, fds))
        .await?;

    while let Some(message) = socket.try_next().await? {
        dbg!(message);
    }

    Ok(())
}
