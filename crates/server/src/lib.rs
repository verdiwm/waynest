use std::{any::Any, collections::HashMap, io, sync::Arc};

pub use async_trait;

use core::fmt;
use futures_core::Stream;
use futures_sink::Sink;
use pin_project_lite::pin_project;
use tokio::net::UnixStream;

use waynest::{Message, ObjectId, ProtocolError, Socket};

mod listener;

pub use listener::Listener;
pub use waynest_macros::RequestDispatcher;

pin_project! {
    pub struct Connection<E: From<ProtocolError>> {
        #[pin]
        socket: Socket,
        store: Store<E>,
        _next_id: usize,
        event_serial: u32,
    }
}

impl<E: From<ProtocolError>> fmt::Debug for Connection<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish()
    }
}

impl<E: From<ProtocolError>> waynest::Connection for Connection<E> {
    type Error = E;

    fn fd(&mut self) -> std::result::Result<std::os::unix::prelude::OwnedFd, E> {
        self.socket.fd().map_err(E::from)
    }
}

impl<E: From<ProtocolError> + From<io::Error> + 'static> Connection<E> {
    pub fn new(stream: UnixStream) -> Result<Self, E> {
        Ok(Self {
            socket: Socket::new(stream.into_std()?)?,
            _next_id: 0xff000000,
            store: Store::new(),
            event_serial: 0,
        })
    }
}

impl<E: From<ProtocolError> + 'static> Connection<E> {
    pub fn next_event_serial(&mut self) -> u32 {
        let prev = self.event_serial;
        self.event_serial = self.event_serial.wrapping_add(1);

        prev
    }

    pub fn insert<D: RequestDispatcher<Error = E>>(&mut self, id: ObjectId, object: D) -> Arc<D> {
        let dispatcher = Arc::new(object);
        self.insert_raw(id, dispatcher.clone());
        dispatcher
    }

    pub fn insert_raw<D: RequestDispatcher<Error = E>>(&mut self, id: ObjectId, object: Arc<D>) {
        self.store.insert(id, object);
    }

    pub fn get<D: RequestDispatcher>(&self, id: ObjectId) -> Option<Arc<D>> {
        let dispatcher = RequestDispatcher::as_any(self.store.get(id)?);
        Arc::downcast(dispatcher).ok()
    }

    pub fn get_raw(&self, id: ObjectId) -> Option<Arc<dyn RequestDispatcher<Error = E>>> {
        self.store.get(id)
    }

    pub fn remove(&mut self, id: ObjectId) {
        self.store.remove(id)
    }

    // pub async fn handle_message(&mut self, message: &mut Message) -> Result<()> {
    //     let object = self
    //         .store
    //         .get(message.object_id())
    //         .ok_or(Error::MissingObject(message.object_id()))?;

    //     object
    //         .dispatch_request(self, message.object_id(), message)
    //         .await
    // }
}

impl<E: From<ProtocolError>> Stream for Connection<E> {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().socket.poll_next(cx)
    }
}

impl<E: From<ProtocolError>> Sink<Message> for Connection<E> {
    type Error = ProtocolError;

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

struct Store<E: From<ProtocolError>> {
    objects: HashMap<ObjectId, Arc<dyn RequestDispatcher<Error = E>>>,
}

impl<E: From<ProtocolError>> Store<E> {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, sender_id: ObjectId, object: Arc<dyn RequestDispatcher<Error = E>>) {
        self.objects.insert(sender_id, object);
    }

    fn get(&self, id: ObjectId) -> Option<Arc<dyn RequestDispatcher<Error = E>>> {
        self.objects.get(&id).cloned()
    }

    fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }
}

#[async_trait::async_trait]
pub trait RequestDispatcher: Any + Send + Sync + 'static {
    type Error: From<ProtocolError>;

    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;

    async fn dispatch_request(
        &self,
        connection: &mut Connection<Self::Error>,
        sender_id: ObjectId,
        message: &mut Message,
    ) -> Result<(), Self::Error>;
}
