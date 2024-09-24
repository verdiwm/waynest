use std::{borrow::Cow, collections::HashMap, os::unix::net::UnixStream, path::Path, sync::Arc};

use anyhow::Result;
use futures_util::TryStreamExt;
use waynest::{
    client::{protocol::core::wayland::wl_display::WlDisplay, Dispatcher},
    wire::{Message, ObjectId, Socket},
};

struct Client {
    objects: HashMap<ObjectId, Arc<dyn Dispatcher>>,
}

impl Client {
    pub fn new() -> Result<Self> {
        let mut objects: HashMap<ObjectId, Arc<dyn Dispatcher>> = HashMap::new();

        objects.insert(ObjectId::DISPLAY, Arc::new(Display::new()));

        Ok(Self { objects })
    }

    pub async fn _handle_message(&mut self, message: &Message) {
        let dispatcher = self
            .objects
            .get(&message.object_id)
            .expect("Invalid object id");

        dispatcher.dispatch(message);
    }

    pub fn display(&self) -> &Display {
        let dispatcher = self
            .objects
            .get(&ObjectId::DISPLAY)
            .expect("Invalid object id");

        dispatcher.downcast_ref().unwrap()
    }
}

#[derive(Clone)]
struct Display {
    // socket: Socket,
}

impl Display {
    pub fn new() -> Self {
        Self {}
    }
}

impl Dispatcher for Display {
    fn dispatch(&self, _message: &Message) {
        todo!()
    }
}

impl WlDisplay for Display {}

#[tokio::main]
async fn main() -> Result<()> {
    let registry = unsafe { ObjectId::from_raw(2) };

    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    let wayland_connection = std::env::var("WAYLAND_DISPLAY")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("wayland-0"));

    let socket_path = Path::new(&xdg_runtime_dir).join(wayland_connection.as_ref());

    let mut socket = Socket::new(UnixStream::connect(socket_path)?)?;

    let client = Client::new()?;

    let display = client.display();

    display
        .get_registry(&mut socket, ObjectId::DISPLAY, registry)
        .await?;

    while let Some(message) = socket.try_next().await? {
        dbg!(message);
    }

    Ok(())
}
