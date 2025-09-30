#![allow(clippy::type_complexity)]

use std::{any::Any, collections::BTreeMap, sync::Arc};

pub use async_trait;

use waynest::{Message, ObjectId, ProtocolError};

mod listener;

pub use listener::{Listener, ListenerError};
pub use waynest_macros::RequestDispatcher;

pub trait Client {
    type Store: ObjectStore + 'static;

    fn store(&self) -> &Self::Store;
    fn store_mut(&mut self) -> &mut Self::Store;

    fn insert<
        D: RequestDispatcher<
                Error = <Self::Store as ObjectStore>::Error,
                Connection = <Self::Store as ObjectStore>::Connection,
            >,
    >(
        &mut self,
        id: ObjectId,
        object: D,
    ) -> Result<
        Arc<D>,
        StoreError<
            Arc<
                dyn RequestDispatcher<
                        Error = <Self::Store as ObjectStore>::Error,
                        Connection = <Self::Store as ObjectStore>::Connection,
                    >,
            >,
        >,
    > {
        let dispatcher = Arc::new(object);

        self.insert_raw(id, dispatcher.clone())?;

        Ok(dispatcher)
    }

    fn insert_raw<
        D: RequestDispatcher<
                Error = <Self::Store as ObjectStore>::Error,
                Connection = <Self::Store as ObjectStore>::Connection,
            >,
    >(
        &mut self,
        id: ObjectId,
        object: Arc<D>,
    ) -> Result<
        (),
        StoreError<
            Arc<
                dyn RequestDispatcher<
                        Error = <Self::Store as ObjectStore>::Error,
                        Connection = <Self::Store as ObjectStore>::Connection,
                    >,
            >,
        >,
    > {
        self.store_mut().insert(id, object)
    }

    fn get<D: RequestDispatcher>(&self, id: ObjectId) -> Option<Arc<D>> {
        let dispatcher = RequestDispatcher::as_any(self.store().get(id)?);
        Arc::downcast(dispatcher).ok()
    }

    fn get_raw(
        &self,
        id: ObjectId,
    ) -> Option<
        Arc<
            dyn RequestDispatcher<
                    Error = <Self::Store as ObjectStore>::Error,
                    Connection = <Self::Store as ObjectStore>::Connection,
                >,
        >,
    > {
        self.store().get(id)
    }

    fn remove(&mut self, id: ObjectId) {
        self.store_mut().remove(id)
    }
}

pub struct Store<C: waynest::Connection, E: From<ProtocolError>> {
    objects: BTreeMap<ObjectId, Arc<dyn RequestDispatcher<Error = E, Connection = C>>>,
}

impl<C: waynest::Connection, E: From<ProtocolError>> Default for Store<C, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: waynest::Connection, E: From<ProtocolError>> Store<C, E> {
    pub fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
        }
    }
}

impl<C: waynest::Connection, E: From<ProtocolError>> ObjectStore for Store<C, E> {
    type Error = E;
    type Connection = C;

    fn insert(
        &mut self,
        sender_id: ObjectId,
        object: Arc<dyn RequestDispatcher<Error = Self::Error, Connection = Self::Connection>>,
    ) -> Result<
        (),
        StoreError<Arc<dyn RequestDispatcher<Error = Self::Error, Connection = Self::Connection>>>,
    > {
        if self.objects.contains_key(&sender_id) {
            return Err(StoreError(object));
        }

        self.objects.insert(sender_id, object);

        Ok(())
    }

    fn get(
        &self,
        id: ObjectId,
    ) -> Option<Arc<dyn RequestDispatcher<Error = Self::Error, Connection = Self::Connection>>>
    {
        self.objects.get(&id).cloned()
    }

    fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreError<T: Clone>(pub T);

pub trait ObjectStore {
    type Connection: waynest::Connection;
    type Error: From<ProtocolError>;

    fn insert(
        &mut self,
        sender_id: ObjectId,
        object: Arc<dyn RequestDispatcher<Error = Self::Error, Connection = Self::Connection>>,
    ) -> Result<
        (),
        StoreError<Arc<dyn RequestDispatcher<Error = Self::Error, Connection = Self::Connection>>>,
    >;

    fn get(
        &self,
        id: ObjectId,
    ) -> Option<Arc<dyn RequestDispatcher<Error = Self::Error, Connection = Self::Connection>>>;

    fn remove(&mut self, id: ObjectId);
}

#[async_trait::async_trait]
pub trait RequestDispatcher: Any + Send + Sync + 'static {
    type Connection: waynest::Connection;
    type Error: From<ProtocolError>;

    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;

    async fn dispatch_request(
        &self,
        connection: &mut Self::Connection,
        sender_id: ObjectId,
        message: &mut Message,
    ) -> Result<(), Self::Error>;
}
