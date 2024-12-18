use std::{borrow::Cow, collections::HashMap, os::unix::net::UnixStream, path::Path, sync::Arc};

use anyhow::Result;
use futures_util::TryStreamExt;
use waynest::{
    client::{protocol::core::wayland::wl_display::WlDisplay, Connection, ConnectionRef, Object},
    wire::{Message, ObjectId, Socket},
};

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use futures_util::{Sink, SinkExt};
use tokio::sync::{mpsc, Mutex};

#[derive(Clone)]
struct State {
    tx: mpsc::Sender<Message>,
    objects: HashMap<ObjectId, Arc<dyn Object>>,
    next_id: u32,
    _socket: Arc<Socket>,
}

#[async_trait]
impl ConnectionState for State {
    async fn send(&self, value: Message) -> Result<(), Error> {
        todo!()
    }

    pub async fn register_object(
        &self,
        id: ObjectId,
        object: Arc<dyn Object>,
    ) -> Result<(), Error> {
        let state = self.state.upgrade().ok_or(Error::Internal)?;
        let mut state = state.lock().await;
        state.objects.insert(id, object);
        Ok(())
    }

    pub async fn unregister_object(&self, id: &ObjectId) -> Result<(), Error> {
        let state = self.state.upgrade().ok_or(Error::Internal)?;
        let mut state = state.lock().await;
        state.objects.remove(id);
        Ok(())
    }
}

impl State {
    pub async fn next_id(&self) -> Result<ObjectId, Error> {
        let state = self.state.upgrade().ok_or(Error::Internal)?;
        let mut state = state.lock().await;
        let id = state.next_id;
        state.next_id += 1;
        Ok(unsafe { ObjectId::from_raw(id) })
    }
}

#[derive(Clone)]
struct Display {
    connection: ConnectionRef,
}

impl Display {
    pub fn new(connection: ConnectionRef) -> Self {
        Self { connection }
    }
}

impl Object for Display {
    fn interface(&self) -> &'static str {
        Self::INTERFACE
    }

    fn version(&self) -> u32 {
        Self::VERSION
    }

    fn id(&self) -> ObjectId {
        ObjectId::DISPLAY
    }

    fn connection(&self) -> &ConnectionRef {
        &self.connection
    }
}

impl WlDisplay for Display {
    async fn error(
        &self,
        object_id: waynest::wire::ObjectId,
        code: u32,
        message: String,
    ) -> waynest::client::Result<()> {
        todo!()
    }

    async fn delete_id(&self, id: u32) -> waynest::client::Result<()> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // let registry = unsafe { ObjectId::from_raw(2) };

    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    let wayland_connection = std::env::var("WAYLAND_DISPLAY")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("wayland-0"));

    let socket_path = Path::new(&xdg_runtime_dir).join(wayland_connection.as_ref());

    let mut socket = Socket::new(UnixStream::connect(socket_path)?)?;

    let connection = Connection::new(socket).await;

    let display = Display::new(connection.downgrade());

    // display
    //     .connection()
    //     .register_object(ObjectId::DISPLAY, Arc::new(display));

    // let client = Client::new()?;

    // let display : &Display {
    //     let dispatcher = self
    //         .objects
    //         .get(&ObjectId::DISPLAY)
    //         .expect("Invalid object id");

    //     dispatcher.downcast_ref().unwrap()
    // }

    // display
    //     .get_registry(&mut socket, ObjectId::DISPLAY, registry)
    //     .await?;

    // while let Some(message) = socket.try_next().await? {
    //     dbg!(message);
    // }

    Ok(())
}
