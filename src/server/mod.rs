mod error;
mod listener;
pub mod protocol;

pub use error::{Error, Result};
pub use listener::Listener;
pub use waynest_macros::Dispatcher;

use async_trait::async_trait;
use core::fmt;
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

    pub fn insert<D: Dispatcher + 'static>(&mut self, sender_id: ObjectId, object: D) {
        self.store.insert(sender_id, Arc::new(object))
    }

    pub async fn handle_message(&mut self, message: &mut Message) -> Result<()> {
        let object = self.store.get(&message.object_id).ok_or(Error::Internal)?;

        object.dispatch(self, message.object_id, message).await
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>> {
        let next = self.socket.try_next().await?;

        Ok(next)
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.socket.send(message).await
    }
}

struct Store {
    objects: HashMap<ObjectId, Arc<dyn Dispatcher>>,
}

impl Store {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, sender_id: ObjectId, object: Arc<dyn Dispatcher>) {
        self.objects.insert(sender_id, object);
    }

    fn get(&self, id: &ObjectId) -> Option<Arc<dyn Dispatcher>> {
        self.objects.get(id).cloned()
    }
}

#[async_trait]
pub trait Dispatcher: Send + Sync {
    async fn dispatch(
        &self,
        client: &mut Client,
        sender_id: ObjectId,
        message: &mut Message,
    ) -> Result<()>;
}
