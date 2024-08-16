use std::{borrow::Cow, collections::HashMap, os::unix::net::UnixStream, path::Path};

use anyhow::Result;
use futures_util::{SinkExt, TryStreamExt};
use waynest::{
    client::{protocol::wayland::wl_display::WlDisplay, Dispatcher},
    wire::{DecodeError, Message, ObjectId, PayloadBuilder, Socket},
};

struct Client {
    socket: Socket,
    objects: HashMap<ObjectId, Arc<dyn Dispatcher>>,
}

struct Display {
    socket: Socket,
}

impl Client {
    pub fn new() -> Result<Self> {
        let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
        let wayland_connection = std::env::var("WAYLAND_DISPLAY")
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed("wayland-0"));

        let socket_path = Path::new(&xdg_runtime_dir).join(wayland_connection.as_ref());

        let socket = Socket::new(UnixStream::connect(socket_path)?)?;


        let objects = HashMap::new();

        Ok(Self { socket, objects })
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>, DecodeError> {
        self.socket.try_next().await
    }
}

impl Dispatcher for Client {
    fn socket(&mut self) -> &mut Socket {
        &mut self.socket
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let registry = unsafe { ObjectId::from_raw(2) };

    let mut client = Client::new()?;

    client.get_registry(registry).await?;

    while let Some(message) = client.next_message().await? {
        dbg!(message);
    }

    Ok(())
}
