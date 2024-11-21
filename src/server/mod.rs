mod error;
mod listener;
pub mod protocol;

pub use error::{Error, Result};
pub use listener::Listener;
pub use waynest_macros::Dispatcher;

use async_trait::async_trait;
use core::fmt;
use downcast_rs::{impl_downcast, DowncastSync};
use futures_util::{SinkExt, TryStreamExt};
use std::{collections::HashMap, io, sync::Arc};
use tokio::net::UnixStream;

use crate::wire::{Message, ObjectId, Socket};

pub struct Client {
    socket: Socket,
    store: Store,
    _next_id: usize,
    event_serial: u32,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish()
    }
}

impl Client {
    pub fn new(stream: UnixStream) -> Result<Self> {
        Ok(Self {
            socket: Socket::new(stream.into_std()?)?,
            _next_id: 0xff000000,
            store: Store::new(),
            event_serial: 0,
        })
    }

    pub fn next_event_serial(&mut self) -> u32 {
        let prev = self.event_serial;
        self.event_serial = self.event_serial.wrapping_add(1);

        prev
    }

    pub fn insert(&mut self, object: Object) {
        self.store.insert(object)
    }

    pub async fn handle_message(&mut self, message: &mut Message) -> Result<()> {
        let object = self.store.get(&message.object_id).ok_or(Error::Internal)?;

        object.dispatch(self, message).await
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>> {
        let next = self.socket.try_next().await?;

        Ok(next)
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.socket.send(message).await
    }
}

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    dispatcher: Arc<dyn Dispatcher>,
}

impl Object {
    pub fn new<D: Dispatcher>(id: ObjectId, dispatcher: D) -> Self {
        Self {
            id,
            dispatcher: Arc::new(dispatcher),
        }
    }

    pub fn as_dispatcher<D: Dispatcher>(&self) -> Result<&D> {
        self.dispatcher.downcast_ref().ok_or(Error::Internal)
    }

    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.dispatcher.dispatch(self, client, message).await
    }
}

struct Store {
    objects: HashMap<ObjectId, Object>,
}

impl Store {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, object: Object) {
        self.objects.insert(object.id, object);
    }

    fn get(&self, id: &ObjectId) -> Option<Object> {
        self.objects.get(id).cloned()
    }
}

#[async_trait]
pub trait Dispatcher: DowncastSync {
    async fn dispatch(
        &self,
        object: &Object,
        client: &mut Client,
        message: &mut Message,
    ) -> Result<()>;
}

impl_downcast!(sync Dispatcher);
