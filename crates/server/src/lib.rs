mod error;
mod listener;

pub use error::{Error, Result};
pub use listener::Listener;
// pub use waynest_macros::Dispatcher;

use async_trait::async_trait;
use core::fmt;
use futures_util::{Sink, Stream};
use pin_project_lite::pin_project;
use std::{any::Any, collections::HashMap, sync::Arc};
use tokio::net::UnixStream;

use waynest::{DecodeError, Message, ObjectId, Socket};

pin_project! {
    pub struct Connection {
        #[pin]
        socket: Socket,
        store: Store,
        _next_id: usize,
        event_serial: u32,
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish()
    }
}

impl waynest::Connection for Connection {
    fn fd(&mut self) -> std::result::Result<std::os::unix::prelude::OwnedFd, DecodeError> {
        self.socket.fd()
    }
}

impl Connection {
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

    pub fn insert<D: RequestDispatcher>(&mut self, id: ObjectId, object: D) -> Arc<D> {
        let dispatcher = Arc::new(object);
        self.insert_raw(id, dispatcher.clone());
        dispatcher
    }

    pub fn insert_raw<D: RequestDispatcher>(&mut self, id: ObjectId, object: Arc<D>) {
        self.store.insert(id, object);
    }

    pub fn get<D: RequestDispatcher>(&self, id: ObjectId) -> Option<Arc<D>> {
        let dispatcher = RequestDispatcher::as_any(self.store.get(id)?);
        Arc::downcast(dispatcher).ok()
    }

    pub fn remove(&mut self, id: ObjectId) {
        self.store.remove(id)
    }

    pub async fn handle_message(&mut self, message: &mut Message) -> Result<()> {
        let object = self
            .store
            .get(message.object_id())
            .ok_or(Error::MissingObject(message.object_id()))?;

        object
            .dispatch_request(self, message.object_id(), message)
            .await
    }
}

impl Stream for Connection {
    type Item = Result<Message, DecodeError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().socket.poll_next(cx)
    }
}

impl Sink<Message> for Connection {
    type Error = DecodeError;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_ready(cx)
    }

    fn start_send(
        self: std::pin::Pin<&mut Self>,
        item: Message,
    ) -> std::result::Result<(), Self::Error> {
        self.project().socket.start_send(item)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_close(cx)
    }
}

struct Store {
    objects: HashMap<ObjectId, Arc<dyn RequestDispatcher>>,
}

impl Store {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, sender_id: ObjectId, object: Arc<dyn RequestDispatcher>) {
        self.objects.insert(sender_id, object);
    }

    fn get(&self, id: ObjectId) -> Option<Arc<dyn RequestDispatcher>> {
        self.objects.get(&id).cloned()
    }

    fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }
}

#[async_trait]
pub trait RequestDispatcher: Any + Send + Sync + 'static {
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;

    async fn dispatch_request(
        &self,
        client: &mut Connection,
        sender_id: ObjectId,
        message: &mut Message,
    ) -> Result<()>;
}
