use std::{any::Any, collections::HashMap, io, sync::Arc};

pub use async_trait;

use core::fmt;
use futures_core::Stream;
use futures_sink::Sink;
use pin_project_lite::pin_project;
use tokio::net::UnixStream;

use waynest::{Message, ObjectId, ProtocolError, Socket};

mod listener;

pub use listener::{Listener, ListenerError};
pub use waynest_macros::RequestDispatcher;

pin_project! {
    pub struct Client<T, E: From<ProtocolError>> {
        #[pin]
        socket: Socket,
        store: Store<T, E>,
        next_object_id: usize,
        next_event_serial: u32,
        state: T,
    }
}

impl<T: Send + Sync, E: From<ProtocolError>> fmt::Debug for Client<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish()
    }
}

impl<T: Send + Sync, E: From<ProtocolError>> waynest::Connection for Client<T, E> {
    type Error = E;

    fn fd(&mut self) -> std::result::Result<std::os::unix::prelude::OwnedFd, E> {
        self.socket.fd().map_err(E::from)
    }
}

impl<T: Send + Sync + 'static, E: From<ProtocolError> + From<io::Error> + 'static> Client<T, E> {
    pub fn new(stream: UnixStream, state: T) -> Result<Self, E> {
        Ok(Self {
            socket: Socket::new(stream.into_std()?)?,
            next_object_id: 0xff000000,
            store: Store::new(),
            next_event_serial: 0,
            state,
        })
    }

    pub fn next_event_serial(&mut self) -> u32 {
        let prev = self.next_event_serial;
        self.next_event_serial = self.next_event_serial.wrapping_add(1);

        prev
    }

    pub fn next_object_id(&mut self) -> usize {
        let prev = self.next_object_id;
        self.next_object_id = self.next_object_id.wrapping_add(1);

        prev
    }

    pub fn insert<D: RequestDispatcher<Error = E, Connection = Client<T, E>>>(
        &mut self,
        id: ObjectId,
        object: D,
    ) -> Arc<D> {
        let dispatcher = Arc::new(object);
        self.insert_raw(id, dispatcher.clone());
        dispatcher
    }

    pub fn insert_raw<D: RequestDispatcher<Error = E, Connection = Client<T, E>>>(
        &mut self,
        id: ObjectId,
        object: Arc<D>,
    ) {
        self.store.insert(id, object);
    }

    pub fn get<D: RequestDispatcher>(&self, id: ObjectId) -> Option<Arc<D>> {
        let dispatcher = RequestDispatcher::as_any(self.store.get(id)?);
        Arc::downcast(dispatcher).ok()
    }

    pub fn get_raw(
        &self,
        id: ObjectId,
    ) -> Option<Arc<dyn RequestDispatcher<Error = E, Connection = Client<T, E>>>> {
        self.store.get(id)
    }

    pub fn remove(&mut self, id: ObjectId) {
        self.store.remove(id)
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut T {
        &mut self.state
    }
}

impl<T: Send + Sync, E: From<ProtocolError>> Stream for Client<T, E> {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().socket.poll_next(cx)
    }
}

impl<T: Send + Sync, E: From<ProtocolError>> Sink<Message> for Client<T, E> {
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

struct Store<T, E: From<ProtocolError>> {
    objects: HashMap<ObjectId, Arc<dyn RequestDispatcher<Error = E, Connection = Client<T, E>>>>,
}

impl<T, E: From<ProtocolError>> Store<T, E> {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    // FIXME: handle possible error if id already exists
    fn insert(
        &mut self,
        sender_id: ObjectId,
        object: Arc<dyn RequestDispatcher<Error = E, Connection = Client<T, E>>>,
    ) {
        self.objects.insert(sender_id, object);
    }

    fn get(
        &self,
        id: ObjectId,
    ) -> Option<Arc<dyn RequestDispatcher<Error = E, Connection = Client<T, E>>>> {
        self.objects.get(&id).cloned()
    }

    fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }
}

#[async_trait::async_trait]
pub trait RequestDispatcher: Any + Send + Sync + 'static {
    type Error: From<ProtocolError>;
    type Connection: waynest::Connection;

    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;

    async fn dispatch_request(
        &self,
        connection: &mut Self::Connection,
        sender_id: ObjectId,
        message: &mut Message,
    ) -> Result<(), Self::Error>;
}
