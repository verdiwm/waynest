mod error;
pub mod protocol;

use std::{any::Any, sync::Arc};

use async_trait::async_trait;
pub use error::{Error, Result};

use crate::wire::{Message, ObjectId, Socket};

#[async_trait]
pub trait Dispatcher: Any + Send + Sync + 'static {
    // necessary for trait upcasting
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;

    async fn dispatch(
        &self,
        socket: &mut Socket,
        sender_id: ObjectId,
        message: &mut Message,
    ) -> Result<()>;
}
