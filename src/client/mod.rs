mod error;
pub mod protocol;
pub use error::{Error, Result};

use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};
use tracing::error;

use crate::wire::{Message, ObjectId, Socket};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Object not found: {0}")]
    ObjectNotFound(ObjectId),
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Connection dropped")]
    ConnectionDropped,
}

#[async_trait]
pub trait Object: DowncastSync {
    fn interface(&self) -> &'static str;
    fn version(&self) -> u32;

    fn id(&self) -> ObjectId;

    fn connection(&self) -> &ConnectionRef;

    async fn send_message(&self, msg: Message) -> Result<(), ClientError> {
        self.connection().send_message(msg).await
    }
}

impl_downcast!(sync Object);

struct ConnectionState {
    socket: Arc<Socket>,
    tx: mpsc::Sender<Message>,
    objects: HashMap<ObjectId, Arc<dyn Object>>,
    next_id: u32,
}

#[derive(Clone)]
pub struct ConnectionRef {
    state: Weak<Mutex<ConnectionState>>,
}

impl ConnectionRef {
    pub async fn send_message(&self, msg: Message) -> Result<(), ClientError> {
        let state = self.state.upgrade().ok_or(ClientError::ConnectionDropped)?;
        let state = state.lock().await;

        state
            .tx
            .send(msg)
            .await
            .map_err(|_| ClientError::ConnectionClosed)
    }

    pub async fn register_object(
        &self,
        id: ObjectId,
        object: Arc<dyn Object>,
    ) -> Result<(), ClientError> {
        let state = self.state.upgrade().ok_or(ClientError::ConnectionDropped)?;
        let mut state = state.lock().await;
        state.objects.insert(id, object);
        Ok(())
    }

    pub async fn unregister_object(&self, id: &ObjectId) -> Result<(), ClientError> {
        let state = self.state.upgrade().ok_or(ClientError::ConnectionDropped)?;
        let mut state = state.lock().await;
        state.objects.remove(id);
        Ok(())
    }

    pub async fn next_id(&self) -> Result<ObjectId, ClientError> {
        let state = self.state.upgrade().ok_or(ClientError::ConnectionDropped)?;
        let mut state = state.lock().await;
        let id = state.next_id;
        state.next_id += 1;
        Ok(unsafe { ObjectId::from_raw(id) })
    }
}

// Connection implementation
pub struct Connection {
    state: Arc<Mutex<ConnectionState>>,
}

impl Connection {
    pub async fn new(socket: Socket) -> Self {
        todo!()
    }

    // pub async fn connect() -> Result<(Self, Arc<dyn Display>), ClientError> {
    //     let socket_path =
    //         std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| String::from("wayland-0"));

    //     let stream = tokio::net::UnixStream::connect(socket_path).await?;
    //     let socket = Arc::new(Socket::new(stream.into_std()?)?);

    //     let (tx, mut rx) = mpsc::channel(32);

    //     // Spawn message sender task
    //     let socket_clone = socket.clone();
    //     tokio::spawn(async move {
    //         while let Some(msg) = rx.recv().await {
    //             trace!("Sending message: {:?}", msg);
    //             if let Err(e) = socket_clone.send(msg).await {
    //                 error!("Failed to send message: {}", e);
    //                 break;
    //             }
    //         }
    //     });

    //     let display_id = ObjectId::DISPLAY;

    //     let state = Arc::new(Mutex::new(ConnectionState {
    //         socket: socket.clone(),
    //         tx,
    //         objects: HashMap::from_iter([(
    //             display_id,
    //             ObjectInfo {
    //                 interface: "wl_display",
    //                 version: 1,
    //             },
    //         )]),
    //         next_id: 2, // Start after display
    //     }));

    //     let connection = Connection {
    //         state: state.clone(),
    //     };
    //     let display = Arc::new(WlDisplay {
    //         id: display_id,
    //         version: 1,
    //         conn: ConnectionRef {
    //             state: Arc::downgrade(&state),
    //         },
    //     });

    //     Ok((connection, display))
    // }

    // pub fn events(&self) -> impl Stream<Item = Result<Message, ClientError>> + '_ {
    //     let state = self.state.clone();
    //     async_stream::stream! {
    //         let state = state.lock().await;
    //         let mut events = state.socket.clone()
    //             .map(|result| result.map_err(|e| ClientError::Protocol(e.to_string())));

    //         while let Some(event) = events.next().await {
    //             yield event;
    //         }
    //     }
    // }
}
