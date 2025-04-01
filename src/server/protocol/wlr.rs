#[doc = "This protocol allows a privileged client to control data devices. In"]
#[doc = "particular, the client will be able to manage the current selection and take"]
#[doc = "the role of a clipboard manager."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod wlr_data_control_unstable_v1 {
    #[doc = "This interface is a manager that allows creating per-seat data device"]
    #[doc = "controls."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_data_control_manager_v1";
        pub const VERSION: u32 = 2u32;
        #[doc = "Trait to implement the zwlr_data_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_data_control_manager_v1#{}.create_data_source({})",
                                sender_id,
                                id
                            );
                            self.create_data_source(client, sender_id, id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_data_control_manager_v1#{}.get_data_device({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_data_device(client, sender_id, id, seat).await
                        }
                        2u16 => {
                            tracing::debug!("zwlr_data_control_manager_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Create a new data source."]
            fn create_data_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Create a data device that can be used to manage a seat's selection."]
            fn get_data_device(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "This interface allows a client to manage a seat's selection."]
    #[doc = ""]
    #[doc = "When the seat is destroyed, this object becomes inert."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_device_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "source given to set_selection or set_primary_selection was already used before"]
            UsedSource = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::UsedSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_data_control_device_v1";
        pub const VERSION: u32 = 2u32;
        #[doc = "Trait to implement the zwlr_data_control_device_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let source = message.object()?;
                            tracing::debug!(
                                "zwlr_data_control_device_v1#{}.set_selection({})",
                                sender_id,
                                source
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_selection(client, sender_id, source).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_data_control_device_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        2u16 => {
                            let source = message.object()?;
                            tracing::debug!(
                                "zwlr_data_control_device_v1#{}.set_primary_selection({})",
                                sender_id,
                                source
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_primary_selection(client, sender_id, source).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "This request asks the compositor to set the selection to the data from"]
            #[doc = "the source on behalf of the client."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "set_primary_selection requests. Attempting to use a previously used"]
            #[doc = "source is a protocol error."]
            #[doc = ""]
            #[doc = "To unset the selection, set the source to NULL."]
            fn set_selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the data device object."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request asks the compositor to set the primary selection to the"]
            #[doc = "data from the source on behalf of the client."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "set_primary_selection requests. Attempting to use a previously used"]
            #[doc = "source is a protocol error."]
            #[doc = ""]
            #[doc = "To unset the primary selection, set the source to NULL."]
            #[doc = ""]
            #[doc = "The compositor will ignore this request if it does not support primary"]
            #[doc = "selection."]
            fn set_primary_selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "The data_offer event introduces a new wlr_data_control_offer object,"]
            #[doc = "which will subsequently be used in either the"]
            #[doc = "wlr_data_control_device.selection event (for the regular clipboard"]
            #[doc = "selections) or the wlr_data_control_device.primary_selection event (for"]
            #[doc = "the primary clipboard selections). Immediately following the"]
            #[doc = "wlr_data_control_device.data_offer event, the new data_offer object"]
            #[doc = "will send out wlr_data_control_offer.offer events to describe the MIME"]
            #[doc = "types it offers."]
            fn data_offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_data_control_device_v1#{}.data_offer({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(id))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "The selection event is sent out to notify the client of a new"]
            #[doc = "wlr_data_control_offer for the selection for this device. The"]
            #[doc = "wlr_data_control_device.data_offer and the wlr_data_control_offer.offer"]
            #[doc = "events are sent out immediately before this event to introduce the data"]
            #[doc = "offer object. The selection event is sent to a client when a new"]
            #[doc = "selection is set. The wlr_data_control_offer is valid until a new"]
            #[doc = "wlr_data_control_offer or NULL is received. The client must destroy the"]
            #[doc = "previous selection wlr_data_control_offer, if any, upon receiving this"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The first selection event is sent upon binding the"]
            #[doc = "wlr_data_control_device object."]
            fn selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_data_control_device_v1#{}.selection({})",
                        sender_id,
                        id.as_ref().map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_object(id).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This data control object is no longer valid and should be destroyed by"]
            #[doc = "the client."]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_data_control_device_v1#{}.finished()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "The primary_selection event is sent out to notify the client of a new"]
            #[doc = "wlr_data_control_offer for the primary selection for this device. The"]
            #[doc = "wlr_data_control_device.data_offer and the wlr_data_control_offer.offer"]
            #[doc = "events are sent out immediately before this event to introduce the data"]
            #[doc = "offer object. The primary_selection event is sent to a client when a"]
            #[doc = "new primary selection is set. The wlr_data_control_offer is valid until"]
            #[doc = "a new wlr_data_control_offer or NULL is received. The client must"]
            #[doc = "destroy the previous primary selection wlr_data_control_offer, if any,"]
            #[doc = "upon receiving this event."]
            #[doc = ""]
            #[doc = "If the compositor supports primary selection, the first"]
            #[doc = "primary_selection event is sent upon binding the"]
            #[doc = "wlr_data_control_device object."]
            fn primary_selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_data_control_device_v1#{}.primary_selection({})",
                        sender_id,
                        id.as_ref().map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_object(id).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "The wlr_data_control_source object is the source side of a"]
    #[doc = "wlr_data_control_offer. It is created by the source client in a data"]
    #[doc = "transfer and provides a way to describe the offered data and a way to"]
    #[doc = "respond to requests to transfer the data."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_source_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "offer sent after wlr_data_control_device.set_selection"]
            InvalidOffer = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidOffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_data_control_source_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_data_control_source_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let mime_type = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_data_control_source_v1#{}.offer(\"{}\")",
                                sender_id,
                                mime_type
                            );
                            self.offer(client, sender_id, mime_type).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_data_control_source_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "This request adds a MIME type to the set of MIME types advertised to"]
            #[doc = "targets. Can be called several times to offer multiple types."]
            #[doc = ""]
            #[doc = "Calling this after wlr_data_control_device.set_selection is a protocol"]
            #[doc = "error."]
            fn offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the data source object."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Request for data from the client. Send the data as the specified MIME"]
            #[doc = "type over the passed file descriptor, then close it."]
            fn send(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_data_control_source_v1#{}.send(\"{}\", {})",
                        sender_id,
                        mime_type,
                        fd.as_raw_fd()
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(mime_type))
                        .put_fd(fd)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This data source is no longer valid. The data source has been replaced"]
            #[doc = "by another data source."]
            #[doc = ""]
            #[doc = "The client should clean up and destroy this data source."]
            fn cancelled(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_data_control_source_v1#{}.cancelled()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "A wlr_data_control_offer represents a piece of data offered for transfer"]
    #[doc = "by another client (the source client). The offer describes the different"]
    #[doc = "MIME types that the data can be converted to and provides the mechanism"]
    #[doc = "for transferring the data directly from the source client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_offer_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_data_control_offer_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_data_control_offer_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let mime_type = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let fd = message.fd()?;
                            tracing::debug!(
                                "zwlr_data_control_offer_v1#{}.receive(\"{}\", {})",
                                sender_id,
                                mime_type,
                                fd.as_raw_fd()
                            );
                            self.receive(client, sender_id, mime_type, fd).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_data_control_offer_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "To transfer the offered data, the client issues this request and"]
            #[doc = "indicates the MIME type it wants to receive. The transfer happens"]
            #[doc = "through the passed file descriptor (typically created with the pipe"]
            #[doc = "system call). The source client writes the data in the MIME type"]
            #[doc = "representation requested and then closes the file descriptor."]
            #[doc = ""]
            #[doc = "The receiving client reads from the read end of the pipe until EOF and"]
            #[doc = "then closes its end, at which point the transfer is complete."]
            #[doc = ""]
            #[doc = "This request may happen multiple times for different MIME types."]
            fn receive(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the data offer object."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Sent immediately after creating the wlr_data_control_offer object."]
            #[doc = "One event per offered MIME type."]
            fn offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_data_control_offer_v1#{}.offer(\"{}\")",
                        sender_id,
                        mime_type
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(mime_type))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[doc = "An interface to capture surfaces in an efficient way by exporting DMA-BUFs."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod wlr_export_dmabuf_unstable_v1 {
    #[doc = "This object is a manager with which to start capturing from sources."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_export_dmabuf_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_export_dmabuf_manager_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_export_dmabuf_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let frame = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let overlay_cursor = message.int()?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_export_dmabuf_manager_v1#{}.capture_output({}, {}, {})",
                                sender_id,
                                frame,
                                overlay_cursor,
                                output
                            );
                            self.capture_output(client, sender_id, frame, overlay_cursor, output)
                                .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zwlr_export_dmabuf_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Capture the next frame of an entire output."]
            fn capture_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "This object represents a single DMA-BUF frame."]
    #[doc = ""]
    #[doc = "If the capture is successful, the compositor will first send a \"frame\""]
    #[doc = "event, followed by one or several \"object\". When the frame is available"]
    #[doc = "for readout, the \"ready\" event is sent."]
    #[doc = ""]
    #[doc = "If the capture failed, the \"cancel\" event is sent. This can happen anytime"]
    #[doc = "before the \"ready\" event."]
    #[doc = ""]
    #[doc = "Once either a \"ready\" or a \"cancel\" event is received, the client should"]
    #[doc = "destroy the frame. Once an \"object\" event is received, the client is"]
    #[doc = "responsible for closing the associated file descriptor."]
    #[doc = ""]
    #[doc = "All frames are read-only and may not be written into or altered."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_export_dmabuf_frame_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Special flags that should be respected by the client."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Flags {
            #[doc = "clients should copy frame before processing"]
            Transient = 1u32,
        }
        impl TryFrom<u32> for Flags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Transient),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Indicates reason for cancelling the frame."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum CancelReason {
            #[doc = "temporary error, source will produce more frames"]
            Temporary = 0u32,
            #[doc = "fatal error, source will not produce frames"]
            Permanent = 1u32,
            #[doc = "temporary error, source will produce more frames"]
            Resizing = 2u32,
        }
        impl TryFrom<u32> for CancelReason {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Temporary),
                    1u32 => Ok(Self::Permanent),
                    2u32 => Ok(Self::Resizing),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for CancelReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_export_dmabuf_frame_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_export_dmabuf_frame_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zwlr_export_dmabuf_frame_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Unreferences the frame. This request must be called as soon as its no"]
            #[doc = "longer used."]
            #[doc = ""]
            #[doc = "It can be called at any time by the client. The client will still have"]
            #[doc = "to close any FDs it has been given."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Main event supplying the client with information about the frame. If the"]
            #[doc = "capture didn't fail, this event is always emitted first before any other"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "This event is followed by a number of \"object\" as specified by the"]
            #[doc = "\"num_objects\" argument."]
            fn frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: u32,
                height: u32,
                offset_x: u32,
                offset_y: u32,
                buffer_flags: u32,
                flags: Flags,
                format: u32,
                mod_high: u32,
                mod_low: u32,
                num_objects: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_export_dmabuf_frame_v1#{}.frame({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        width,
                        height,
                        offset_x,
                        offset_y,
                        buffer_flags,
                        flags,
                        format,
                        mod_high,
                        mod_low,
                        num_objects
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(width)
                        .put_uint(height)
                        .put_uint(offset_x)
                        .put_uint(offset_y)
                        .put_uint(buffer_flags)
                        .put_uint(flags as u32)
                        .put_uint(format)
                        .put_uint(mod_high)
                        .put_uint(mod_low)
                        .put_uint(num_objects)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Event which serves to supply the client with the file descriptors"]
            #[doc = "containing the data for each object."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must always close the file"]
            #[doc = "descriptor as soon as they're done with it and even if the frame fails."]
            fn object(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                index: u32,
                fd: rustix::fd::OwnedFd,
                size: u32,
                offset: u32,
                stride: u32,
                plane_index: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_export_dmabuf_frame_v1#{}.object({}, {}, {}, {}, {}, {})",
                        sender_id,
                        index,
                        fd.as_raw_fd(),
                        size,
                        offset,
                        stride,
                        plane_index
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(index)
                        .put_fd(fd)
                        .put_uint(size)
                        .put_uint(offset)
                        .put_uint(stride)
                        .put_uint(plane_index)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent as soon as the frame is presented, indicating it is"]
            #[doc = "available for reading. This event includes the time at which"]
            #[doc = "presentation happened at."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part"]
            #[doc = "may have an arbitrary offset at start."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy this object."]
            fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_export_dmabuf_frame_v1#{}.ready({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "If the capture failed or if the frame is no longer valid after the"]
            #[doc = "\"frame\" event has been emitted, this event will be used to inform the"]
            #[doc = "client to scrap the frame."]
            #[doc = ""]
            #[doc = "If the failure is temporary, the client may capture again the same"]
            #[doc = "source. If the failure is permanent, any further attempts to capture the"]
            #[doc = "same source will fail again."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy this object."]
            fn cancel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                reason: CancelReason,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_export_dmabuf_frame_v1#{}.cancel({})",
                        sender_id,
                        reason
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(reason as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod wlr_foreign_toplevel_management_unstable_v1 {
    #[doc = "The purpose of this protocol is to enable the creation of taskbars"]
    #[doc = "and docks by providing them with a list of opened applications and"]
    #[doc = "letting them request certain actions on them, like maximizing, etc."]
    #[doc = ""]
    #[doc = "After a client binds the zwlr_foreign_toplevel_manager_v1, each opened"]
    #[doc = "toplevel window will be sent via the toplevel event"]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_foreign_toplevel_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_foreign_toplevel_manager_v1";
        pub const VERSION: u32 = 3u32;
        #[doc = "Trait to implement the zwlr_foreign_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_manager_v1#{}.stop()",
                                sender_id,
                            );
                            self.stop(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Indicates the client no longer wishes to receive events for new toplevels."]
            #[doc = "However the compositor may emit further toplevel_created events, until"]
            #[doc = "the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            fn stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "This event is emitted whenever a new toplevel window is created. It"]
            #[doc = "is emitted for all toplevels, regardless of the app that has created"]
            #[doc = "them."]
            #[doc = ""]
            #[doc = "All initial details of the toplevel(title, app_id, states, etc.) will"]
            #[doc = "be sent immediately after this event via the corresponding events in"]
            #[doc = "zwlr_foreign_toplevel_handle_v1."]
            fn toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_manager_v1#{}.toplevel({})",
                        sender_id,
                        toplevel
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the compositor is done sending events to the"]
            #[doc = "zwlr_foreign_toplevel_manager_v1. The server will destroy the object"]
            #[doc = "immediately after sending this request, so it will become invalid and"]
            #[doc = "the client should free any resources associated with it."]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_manager_v1#{}.finished()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "A zwlr_foreign_toplevel_handle_v1 object represents an opened toplevel"]
    #[doc = "window. Each app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Each toplevel has a list of outputs it is visible on, conveyed to the"]
    #[doc = "client with the output_enter and output_leave events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_foreign_toplevel_handle_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "The different states that a toplevel can have. These have the same meaning"]
        #[doc = "as the states with the same names defined in xdg-toplevel"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            #[doc = "the toplevel is maximized"]
            Maximized = 0u32,
            #[doc = "the toplevel is minimized"]
            Minimized = 1u32,
            #[doc = "the toplevel is active"]
            Activated = 2u32,
            #[doc = "the toplevel is fullscreen"]
            Fullscreen = 3u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Maximized),
                    1u32 => Ok(Self::Minimized),
                    2u32 => Ok(Self::Activated),
                    3u32 => Ok(Self::Fullscreen),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the provided rectangle is invalid"]
            InvalidRectangle = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidRectangle),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_foreign_toplevel_handle_v1";
        pub const VERSION: u32 = 3u32;
        #[doc = "Trait to implement the zwlr_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.set_maximized()",
                                sender_id,
                            );
                            self.set_maximized(client, sender_id).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.unset_maximized()",
                                sender_id,
                            );
                            self.unset_maximized(client, sender_id).await
                        }
                        2u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.set_minimized()",
                                sender_id,
                            );
                            self.set_minimized(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.unset_minimized()",
                                sender_id,
                            );
                            self.unset_minimized(client, sender_id).await
                        }
                        4u16 => {
                            let seat = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.activate({})",
                                sender_id,
                                seat
                            );
                            self.activate(client, sender_id, seat).await
                        }
                        5u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.close()",
                                sender_id,
                            );
                            self.close(client, sender_id).await
                        }
                        6u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.set_rectangle({}, {}, {}, {}, {})",
                                sender_id,
                                surface,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_rectangle(client, sender_id, surface, x, y, width, height)
                                .await
                        }
                        7u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        8u16 => {
                            let output = message.object()?;
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.set_fullscreen({})",
                                sender_id,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_fullscreen(client, sender_id, output).await
                        }
                        9u16 => {
                            tracing::debug!(
                                "zwlr_foreign_toplevel_handle_v1#{}.unset_fullscreen()",
                                sender_id,
                            );
                            self.unset_fullscreen(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Requests that the toplevel be maximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn set_maximized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the toplevel be unmaximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn unset_maximized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the toplevel be minimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn set_minimized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the toplevel be unminimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn unset_minimized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Request that this toplevel be activated on the given seat."]
            #[doc = "There is no guarantee the toplevel will be actually activated."]
            fn activate(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Send a request to the toplevel to close itself. The compositor would"]
            #[doc = "typically use a shell-specific method to carry out this request, for"]
            #[doc = "example by sending the xdg_toplevel.close event. However, this gives"]
            #[doc = "no guarantees the toplevel will actually be destroyed. If and when"]
            #[doc = "this happens, the zwlr_foreign_toplevel_handle_v1.closed event will"]
            #[doc = "be emitted."]
            fn close(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "The rectangle of the surface specified in this request corresponds to"]
            #[doc = "the place where the app using this protocol represents the given toplevel."]
            #[doc = "It can be used by the compositor as a hint for some operations, e.g"]
            #[doc = "minimizing. The client is however not required to set this, in which"]
            #[doc = "case the compositor is free to decide some default value."]
            #[doc = ""]
            #[doc = "If the client specifies more than one rectangle, only the last one is"]
            #[doc = "considered."]
            #[doc = ""]
            #[doc = "The dimensions are given in surface-local coordinates."]
            #[doc = "Setting width=height=0 removes the already-set rectangle."]
            fn set_rectangle(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the zwlr_foreign_toplevel_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the toplevel be fullscreened on the given output. If the"]
            #[doc = "fullscreen state and/or the outputs the toplevel is visible on actually"]
            #[doc = "change, this will be indicated by the state and output_enter/leave"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "The output parameter is only a hint to the compositor. Also, if output"]
            #[doc = "is NULL, the compositor should decide which output the toplevel will be"]
            #[doc = "fullscreened on, if at all."]
            fn set_fullscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the toplevel be unfullscreened. If the fullscreen state"]
            #[doc = "actually changes, this will be indicated by the state event."]
            fn unset_fullscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            fn title(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                title: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_handle_v1#{}.title(\"{}\")",
                        sender_id,
                        title
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(title))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is emitted whenever the app-id of the toplevel changes."]
            fn app_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                app_id: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_handle_v1#{}.app_id(\"{}\")",
                        sender_id,
                        app_id
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is emitted whenever the toplevel becomes visible on"]
            #[doc = "the given output. A toplevel may be visible on multiple outputs."]
            fn output_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_handle_v1#{}.output_enter({})",
                        sender_id,
                        output
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is emitted whenever the toplevel stops being visible on"]
            #[doc = "the given output. It is guaranteed that an entered-output event"]
            #[doc = "with the same output has been emitted before this event."]
            fn output_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_handle_v1#{}.output_leave({})",
                        sender_id,
                        output
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is emitted immediately after the zlw_foreign_toplevel_handle_v1"]
            #[doc = "is created and each time the toplevel state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            fn state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_handle_v1#{}.state(array[{}])",
                        sender_id,
                        state.len()
                    );
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_array(state).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent after all changes in the toplevel state have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "This allows changes to the zwlr_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_foreign_toplevel_handle_v1#{}.done()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event means the toplevel has been destroyed. It is guaranteed there"]
            #[doc = "won't be any more events for this zwlr_foreign_toplevel_handle_v1. The"]
            #[doc = "toplevel itself becomes inert so any requests will be ignored except the"]
            #[doc = "destroy request."]
            fn closed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_foreign_toplevel_handle_v1#{}.closed()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is emitted whenever the parent of the toplevel changes."]
            #[doc = ""]
            #[doc = "No event is emitted when the parent handle is destroyed by the client."]
            fn parent(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                parent: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_foreign_toplevel_handle_v1#{}.parent({})",
                        sender_id,
                        parent
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(parent)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[doc = "This protocol allows a privileged client to set the gamma tables for"]
#[doc = "outputs."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod wlr_gamma_control_unstable_v1 {
    #[doc = "This interface is a manager that allows creating per-output gamma"]
    #[doc = "controls."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_gamma_control_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_gamma_control_manager_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_gamma_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_gamma_control_manager_v1#{}.get_gamma_control({}, {})",
                                sender_id,
                                id,
                                output
                            );
                            self.get_gamma_control(client, sender_id, id, output).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zwlr_gamma_control_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Create a gamma control that can be used to adjust gamma tables for the"]
            #[doc = "provided output."]
            fn get_gamma_control(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "This interface allows a client to adjust gamma tables for a particular"]
    #[doc = "output."]
    #[doc = ""]
    #[doc = "The client will receive the gamma size, and will then be able to set gamma"]
    #[doc = "tables. At any time the compositor can send a failed event indicating that"]
    #[doc = "this object is no longer valid."]
    #[doc = ""]
    #[doc = "There can only be at most one gamma control object per output, which"]
    #[doc = "has exclusive access to this particular output. When the gamma control"]
    #[doc = "object is destroyed, the gamma table is restored to its original value."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_gamma_control_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid gamma tables"]
            InvalidGamma = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidGamma),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_gamma_control_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_gamma_control_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let fd = message.fd()?;
                            tracing::debug!(
                                "zwlr_gamma_control_v1#{}.set_gamma({})",
                                sender_id,
                                fd.as_raw_fd()
                            );
                            self.set_gamma(client, sender_id, fd).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_gamma_control_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Set the gamma table. The file descriptor can be memory-mapped to provide"]
            #[doc = "the raw gamma table, which contains successive gamma ramps for the red,"]
            #[doc = "green and blue channels. Each gamma ramp is an array of 16-byte unsigned"]
            #[doc = "integers which has the same length as the gamma size."]
            #[doc = ""]
            #[doc = "The file descriptor data must have the same length as three times the"]
            #[doc = "gamma size."]
            fn set_gamma(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the gamma control object. If the object is still valid, this"]
            #[doc = "restores the original gamma tables."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Advertise the size of each gamma ramp."]
            #[doc = ""]
            #[doc = "This event is sent immediately when the gamma control object is created."]
            fn gamma_size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                size: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_gamma_control_v1#{}.gamma_size({})",
                        sender_id,
                        size
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(size).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the gamma control is no longer valid. This"]
            #[doc = "can happen for a number of reasons, including:"]
            #[doc = "- The output doesn't support gamma tables"]
            #[doc = "- Setting the gamma tables failed"]
            #[doc = "- Another client already has exclusive gamma control for this output"]
            #[doc = "- The compositor has transferred gamma control to another client"]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_gamma_control_v1#{}.failed()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod wlr_input_inhibit_unstable_v1 {
    #[doc = "Clients can use this interface to prevent input events from being sent to"]
    #[doc = "any surfaces but its own, which is useful for example in lock screen"]
    #[doc = "software. It is assumed that access to this interface will be locked down"]
    #[doc = "to whitelisted clients by the compositor."]
    #[doc = ""]
    #[doc = "Note! This protocol is deprecated and not intended for production use."]
    #[doc = "For screen lockers, use the ext-session-lock-v1 protocol."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_input_inhibit_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "an input inhibitor is already in use on the compositor"]
            AlreadyInhibited = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyInhibited),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_input_inhibit_manager_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_input_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_input_inhibit_manager_v1#{}.get_inhibitor({})",
                                sender_id,
                                id
                            );
                            self.get_inhibitor(client, sender_id, id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Activates the input inhibitor. As long as the inhibitor is active, the"]
            #[doc = "compositor will not send input events to other clients."]
            fn get_inhibitor(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "While this resource exists, input to clients other than the owner of the"]
    #[doc = "inhibitor resource will not receive input events. Any client which"]
    #[doc = "previously had focus will receive a leave event and will not be given"]
    #[doc = "focus again. The client that owns this resource will receive all input"]
    #[doc = "events normally. The compositor will also disable all of its own input"]
    #[doc = "processing (such as keyboard shortcuts) while the inhibitor is active."]
    #[doc = ""]
    #[doc = "The compositor may continue to send input events to selected clients,"]
    #[doc = "such as an on-screen keyboard (via the input-method protocol)."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_input_inhibitor_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_input_inhibitor_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_input_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zwlr_input_inhibitor_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Destroy the inhibitor and allow other clients to receive input."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
}
#[allow(clippy::module_inception)]
pub mod wlr_layer_shell_unstable_v1 {
    #[doc = "Clients can use this interface to assign the surface_layer role to"]
    #[doc = "wl_surfaces. Such surfaces are assigned to a \"layer\" of the output and"]
    #[doc = "rendered with a defined z-depth respective to each other. They may also be"]
    #[doc = "anchored to the edges and corners of a screen and specify input handling"]
    #[doc = "semantics. This interface should be suitable for the implementation of"]
    #[doc = "many desktop shell components, and a broad number of other applications"]
    #[doc = "that interact with the desktop."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_layer_shell_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface has another role"]
            Role = 0u32,
            #[doc = "layer value is invalid"]
            InvalidLayer = 1u32,
            #[doc = "wl_surface has a buffer attached or committed"]
            AlreadyConstructed = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::InvalidLayer),
                    2u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These values indicate which layers a surface can be rendered in. They"]
        #[doc = "are ordered by z depth, bottom-most first. Traditional shell surfaces"]
        #[doc = "will typically be rendered between the bottom and top layers."]
        #[doc = "Fullscreen shell surfaces are typically rendered at the top layer."]
        #[doc = "Multiple surfaces can share a single layer, and ordering within a"]
        #[doc = "single layer is undefined."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Layer {
            Background = 0u32,
            Bottom = 1u32,
            Top = 2u32,
            Overlay = 3u32,
        }
        impl TryFrom<u32> for Layer {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Background),
                    1u32 => Ok(Self::Bottom),
                    2u32 => Ok(Self::Top),
                    3u32 => Ok(Self::Overlay),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Layer {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_layer_shell_v1";
        pub const VERSION: u32 = 5u32;
        #[doc = "Trait to implement the zwlr_layer_shell_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let output = message.object()?;
                            let layer = message.uint()?;
                            let namespace = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_layer_shell_v1#{}.get_layer_surface({}, {}, {}, {}, \"{}\")",
                                sender_id,
                                id,
                                surface,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                layer,
                                namespace
                            );
                            self.get_layer_surface(
                                client,
                                sender_id,
                                id,
                                surface,
                                output,
                                layer.try_into()?,
                                namespace,
                            )
                            .await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_layer_shell_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Create a layer surface for an existing surface. This assigns the role of"]
            #[doc = "layer_surface, or raises a protocol error if another role is already"]
            #[doc = "assigned."]
            #[doc = ""]
            #[doc = "Creating a layer surface from a wl_surface which has a buffer attached"]
            #[doc = "or committed is a client error, and any attempts by a client to attach"]
            #[doc = "or manipulate a buffer prior to the first layer_surface.configure call"]
            #[doc = "must also be treated as errors."]
            #[doc = ""]
            #[doc = "After creating a layer_surface object and setting it up, the client"]
            #[doc = "must perform an initial commit without any buffer attached."]
            #[doc = "The compositor will reply with a layer_surface.configure event."]
            #[doc = "The client must acknowledge it and is then allowed to attach a buffer"]
            #[doc = "to map the surface."]
            #[doc = ""]
            #[doc = "You may pass NULL for output to allow the compositor to decide which"]
            #[doc = "output to use. Generally this will be the one that the user most"]
            #[doc = "recently interacted with."]
            #[doc = ""]
            #[doc = "Clients can specify a namespace that defines the purpose of the layer"]
            #[doc = "surface."]
            fn get_layer_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
                layer: Layer,
                namespace: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request indicates that the client will not use the layer_shell"]
            #[doc = "object any more. Objects that have been created through this instance"]
            #[doc = "are not affected."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "An interface that may be implemented by a wl_surface, for surfaces that"]
    #[doc = "are designed to be rendered as a layer of a stacked desktop-like"]
    #[doc = "environment."]
    #[doc = ""]
    #[doc = "Layer surface state (layer, size, anchor, exclusive zone,"]
    #[doc = "margin, interactivity) is double-buffered, and will be applied at the"]
    #[doc = "time wl_surface.commit of the corresponding wl_surface is called."]
    #[doc = ""]
    #[doc = "Attaching a null buffer to a layer surface unmaps it."]
    #[doc = ""]
    #[doc = "Unmapping a layer_surface means that the surface cannot be shown by the"]
    #[doc = "compositor until it is explicitly mapped again. The layer_surface"]
    #[doc = "returns to the state it had right after layer_shell.get_layer_surface."]
    #[doc = "The client can re-map the surface by performing a commit without any"]
    #[doc = "buffer attached, waiting for a configure event and handling it as usual."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_layer_surface_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Types of keyboard interaction possible for layer shell surfaces. The"]
        #[doc = "rationale for this is twofold: (1) some applications are not interested"]
        #[doc = "in keyboard events and not allowing them to be focused can improve the"]
        #[doc = "desktop experience; (2) some applications will want to take exclusive"]
        #[doc = "keyboard focus."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum KeyboardInteractivity {
            None = 0u32,
            Exclusive = 1u32,
            OnDemand = 2u32,
        }
        impl TryFrom<u32> for KeyboardInteractivity {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Exclusive),
                    2u32 => Ok(Self::OnDemand),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for KeyboardInteractivity {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "provided surface state is invalid"]
            InvalidSurfaceState = 0u32,
            #[doc = "size is invalid"]
            InvalidSize = 1u32,
            #[doc = "anchor bitfield is invalid"]
            InvalidAnchor = 2u32,
            #[doc = "keyboard interactivity is invalid"]
            InvalidKeyboardInteractivity = 3u32,
            #[doc = "exclusive edge is invalid given the surface anchors"]
            InvalidExclusiveEdge = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSurfaceState),
                    1u32 => Ok(Self::InvalidSize),
                    2u32 => Ok(Self::InvalidAnchor),
                    3u32 => Ok(Self::InvalidKeyboardInteractivity),
                    4u32 => Ok(Self::InvalidExclusiveEdge),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Anchor : u32 { # [doc = "the top edge of the anchor rectangle"] const Top = 1u32 ; # [doc = "the bottom edge of the anchor rectangle"] const Bottom = 2u32 ; # [doc = "the left edge of the anchor rectangle"] const Left = 4u32 ; # [doc = "the right edge of the anchor rectangle"] const Right = 8u32 ; } }
        impl TryFrom<u32> for Anchor {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Anchor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_layer_surface_v1";
        pub const VERSION: u32 = 5u32;
        #[doc = "Trait to implement the zwlr_layer_surface_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let width = message.uint()?;
                            let height = message.uint()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_size({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.set_size(client, sender_id, width, height).await
                        }
                        1u16 => {
                            let anchor = message.uint()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_anchor({})",
                                sender_id,
                                anchor
                            );
                            self.set_anchor(client, sender_id, anchor.try_into()?).await
                        }
                        2u16 => {
                            let zone = message.int()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_exclusive_zone({})",
                                sender_id,
                                zone
                            );
                            self.set_exclusive_zone(client, sender_id, zone).await
                        }
                        3u16 => {
                            let top = message.int()?;
                            let right = message.int()?;
                            let bottom = message.int()?;
                            let left = message.int()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_margin({}, {}, {}, {})",
                                sender_id,
                                top,
                                right,
                                bottom,
                                left
                            );
                            self.set_margin(client, sender_id, top, right, bottom, left)
                                .await
                        }
                        4u16 => {
                            let keyboard_interactivity = message.uint()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_keyboard_interactivity({})",
                                sender_id,
                                keyboard_interactivity
                            );
                            self.set_keyboard_interactivity(
                                client,
                                sender_id,
                                keyboard_interactivity.try_into()?,
                            )
                            .await
                        }
                        5u16 => {
                            let popup = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.get_popup({})",
                                sender_id,
                                popup
                            );
                            self.get_popup(client, sender_id, popup).await
                        }
                        6u16 => {
                            let serial = message.uint()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.ack_configure({})",
                                sender_id,
                                serial
                            );
                            self.ack_configure(client, sender_id, serial).await
                        }
                        7u16 => {
                            tracing::debug!("zwlr_layer_surface_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        8u16 => {
                            let layer = message.uint()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_layer({})",
                                sender_id,
                                layer
                            );
                            self.set_layer(client, sender_id, layer.try_into()?).await
                        }
                        9u16 => {
                            let edge = message.uint()?;
                            tracing::debug!(
                                "zwlr_layer_surface_v1#{}.set_exclusive_edge({})",
                                sender_id,
                                edge
                            );
                            self.set_exclusive_edge(client, sender_id, edge.try_into()?)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Sets the size of the surface in surface-local coordinates. The"]
            #[doc = "compositor will display the surface centered with respect to its"]
            #[doc = "anchors."]
            #[doc = ""]
            #[doc = "If you pass 0 for either value, the compositor will assign it and"]
            #[doc = "inform you of the assignment in the configure event. You must set your"]
            #[doc = "anchor to opposite edges in the dimensions you omit; not doing so is a"]
            #[doc = "protocol error. Both values are 0 by default."]
            #[doc = ""]
            #[doc = "Size is double-buffered, see wl_surface.commit."]
            fn set_size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the compositor anchor the surface to the specified edges"]
            #[doc = "and corners. If two orthogonal edges are specified (e.g. 'top' and"]
            #[doc = "'left'), then the anchor point will be the intersection of the edges"]
            #[doc = "(e.g. the top left corner of the output); otherwise the anchor point"]
            #[doc = "will be centered on that edge, or in the center if none is specified."]
            #[doc = ""]
            #[doc = "Anchor is double-buffered, see wl_surface.commit."]
            fn set_anchor(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                anchor: Anchor,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the compositor avoids occluding an area with other"]
            #[doc = "surfaces. The compositor's use of this information is"]
            #[doc = "implementation-dependent - do not assume that this region will not"]
            #[doc = "actually be occluded."]
            #[doc = ""]
            #[doc = "A positive value is only meaningful if the surface is anchored to one"]
            #[doc = "edge or an edge and both perpendicular edges. If the surface is not"]
            #[doc = "anchored, anchored to only two perpendicular edges (a corner), anchored"]
            #[doc = "to only two parallel edges or anchored to all edges, a positive value"]
            #[doc = "will be treated the same as zero."]
            #[doc = ""]
            #[doc = "A positive zone is the distance from the edge in surface-local"]
            #[doc = "coordinates to consider exclusive."]
            #[doc = ""]
            #[doc = "Surfaces that do not wish to have an exclusive zone may instead specify"]
            #[doc = "how they should interact with surfaces that do. If set to zero, the"]
            #[doc = "surface indicates that it would like to be moved to avoid occluding"]
            #[doc = "surfaces with a positive exclusive zone. If set to -1, the surface"]
            #[doc = "indicates that it would not like to be moved to accommodate for other"]
            #[doc = "surfaces, and the compositor should extend it all the way to the edges"]
            #[doc = "it is anchored to."]
            #[doc = ""]
            #[doc = "For example, a panel might set its exclusive zone to 10, so that"]
            #[doc = "maximized shell surfaces are not shown on top of it. A notification"]
            #[doc = "might set its exclusive zone to 0, so that it is moved to avoid"]
            #[doc = "occluding the panel, but shell surfaces are shown underneath it. A"]
            #[doc = "wallpaper or lock screen might set their exclusive zone to -1, so that"]
            #[doc = "they stretch below or over the panel."]
            #[doc = ""]
            #[doc = "The default value is 0."]
            #[doc = ""]
            #[doc = "Exclusive zone is double-buffered, see wl_surface.commit."]
            fn set_exclusive_zone(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                zone: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests that the surface be placed some distance away from the anchor"]
            #[doc = "point on the output, in surface-local coordinates. Setting this value"]
            #[doc = "for edges you are not anchored to has no effect."]
            #[doc = ""]
            #[doc = "The exclusive zone includes the margin."]
            #[doc = ""]
            #[doc = "Margin is double-buffered, see wl_surface.commit."]
            fn set_margin(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                top: i32,
                right: i32,
                bottom: i32,
                left: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Set how keyboard events are delivered to this surface. By default,"]
            #[doc = "layer shell surfaces do not receive keyboard events; this request can"]
            #[doc = "be used to change this."]
            #[doc = ""]
            #[doc = "This setting is inherited by child surfaces set by the get_popup"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Layer surfaces receive pointer, touch, and tablet events normally. If"]
            #[doc = "you do not want to receive them, set the input region on your surface"]
            #[doc = "to an empty region."]
            #[doc = ""]
            #[doc = "Keyboard interactivity is double-buffered, see wl_surface.commit."]
            fn set_keyboard_interactivity(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                keyboard_interactivity: KeyboardInteractivity,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This assigns an xdg_popup's parent to this layer_surface.  This popup"]
            #[doc = "should have been created via xdg_surface::get_popup with the parent set"]
            #[doc = "to NULL, and this request must be invoked before committing the popup's"]
            #[doc = "initial state."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_popup for more details about what an"]
            #[doc = "xdg_popup is and how it is used."]
            fn get_popup(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                popup: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "When a configure event is received, if a client commits the"]
            #[doc = "surface in response to the configure event, then the client"]
            #[doc = "must make an ack_configure request sometime before the commit"]
            #[doc = "request, passing along the serial of the configure event."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it"]
            #[doc = "can respond to one, it only has to ack the last configure event."]
            #[doc = ""]
            #[doc = "A client is not required to commit immediately after sending"]
            #[doc = "an ack_configure request - it may even ack_configure several times"]
            #[doc = "before its next surface commit."]
            #[doc = ""]
            #[doc = "A client may send multiple ack_configure requests before committing, but"]
            #[doc = "only the last request sent before a commit indicates which configure"]
            #[doc = "event the client really is responding to."]
            fn ack_configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request destroys the layer surface."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Change the layer that the surface is rendered on."]
            #[doc = ""]
            #[doc = "Layer is double-buffered, see wl_surface.commit."]
            fn set_layer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                layer : super :: super :: super :: wlr :: wlr_layer_shell_unstable_v1 :: zwlr_layer_shell_v1 :: Layer,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Requests an edge for the exclusive zone to apply. The exclusive"]
            #[doc = "edge will be automatically deduced from anchor points when possible,"]
            #[doc = "but when the surface is anchored to a corner, it will be necessary"]
            #[doc = "to set it explicitly to disambiguate, as it is not possible to deduce"]
            #[doc = "which one of the two corner edges should be used."]
            #[doc = ""]
            #[doc = "The edge must be one the surface is anchored to, otherwise the"]
            #[doc = "invalid_exclusive_edge protocol error will be raised."]
            fn set_exclusive_edge(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                edge: Anchor,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "The configure event asks the client to resize its surface."]
            #[doc = ""]
            #[doc = "Clients should arrange their surface for the new states, and then send"]
            #[doc = "an ack_configure request with the serial sent in this configure event at"]
            #[doc = "some point before committing the new surface."]
            #[doc = ""]
            #[doc = "The client is free to dismiss all but the last configure event it"]
            #[doc = "received."]
            #[doc = ""]
            #[doc = "The width and height arguments specify the size of the window in"]
            #[doc = "surface-local coordinates."]
            #[doc = ""]
            #[doc = "The size is a hint, in the sense that the client is free to ignore it if"]
            #[doc = "it doesn't resize, pick a smaller size (to satisfy aspect ratio or"]
            #[doc = "resize in steps of NxM pixels). If the client picks a smaller size and"]
            #[doc = "is anchored to two opposite anchors (e.g. 'top' and 'bottom'), the"]
            #[doc = "surface will be centered on this axis."]
            #[doc = ""]
            #[doc = "If the width or height arguments are zero, it means the client should"]
            #[doc = "decide its own window dimension."]
            fn configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_layer_surface_v1#{}.configure({}, {}, {})",
                        sender_id,
                        serial,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "The closed event is sent by the compositor when the surface will no"]
            #[doc = "longer be shown. The output may have been destroyed or the user may"]
            #[doc = "have asked for it to be removed. Further changes to the surface will be"]
            #[doc = "ignored. The client should destroy the resource after receiving this"]
            #[doc = "event, and create a new surface if they so choose."]
            fn closed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_layer_surface_v1#{}.closed()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[doc = "This protocol exposes interfaces to obtain and modify output device"]
#[doc = "configuration."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod wlr_output_management_unstable_v1 {
    #[doc = "This interface is a manager that allows reading and writing the current"]
    #[doc = "output device configuration."]
    #[doc = ""]
    #[doc = "Output devices that display pixels (e.g. a physical monitor or a virtual"]
    #[doc = "output in a window) are represented as heads. Heads cannot be created nor"]
    #[doc = "destroyed by the client, but they can be enabled or disabled and their"]
    #[doc = "properties can be changed. Each head may have one or more available modes."]
    #[doc = ""]
    #[doc = "Whenever a head appears (e.g. a monitor is plugged in), it will be"]
    #[doc = "advertised via the head event. Immediately after the output manager is"]
    #[doc = "bound, all current heads are advertised."]
    #[doc = ""]
    #[doc = "Whenever a head's properties change, the relevant wlr_output_head events"]
    #[doc = "will be sent. Not all head properties will be sent: only properties that"]
    #[doc = "have changed need to."]
    #[doc = ""]
    #[doc = "Whenever a head disappears (e.g. a monitor is unplugged), a"]
    #[doc = "wlr_output_head.finished event will be sent."]
    #[doc = ""]
    #[doc = "After one or more heads appear, change or disappear, the done event will"]
    #[doc = "be sent. It carries a serial which can be used in a create_configuration"]
    #[doc = "request to update heads properties."]
    #[doc = ""]
    #[doc = "The information obtained from this protocol should only be used for output"]
    #[doc = "configuration purposes. This protocol is not designed to be a generic"]
    #[doc = "output property advertisement protocol for regular clients. Instead,"]
    #[doc = "protocols such as xdg-output should be used."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_output_manager_v1";
        pub const VERSION: u32 = 4u32;
        #[doc = "Trait to implement the zwlr_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let serial = message.uint()?;
                            tracing::debug!(
                                "zwlr_output_manager_v1#{}.create_configuration({}, {})",
                                sender_id,
                                id,
                                serial
                            );
                            self.create_configuration(client, sender_id, id, serial)
                                .await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_output_manager_v1#{}.stop()", sender_id,);
                            self.stop(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Create a new output configuration object. This allows to update head"]
            #[doc = "properties."]
            fn create_configuration(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                serial: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Indicates the client no longer wishes to receive events for output"]
            #[doc = "configuration changes. However the compositor may emit further events,"]
            #[doc = "until the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            fn stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "This event introduces a new head. This happens whenever a new head"]
            #[doc = "appears (e.g. a monitor is plugged in) or after the output manager is"]
            #[doc = "bound."]
            fn head(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_manager_v1#{}.head({})", sender_id, head);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(head))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent after all information has been sent after binding to"]
            #[doc = "the output manager object and after any subsequent changes. This applies"]
            #[doc = "to child head and mode objects as well. In other words, this event is"]
            #[doc = "sent whenever a head or mode is created or destroyed and whenever one of"]
            #[doc = "their properties has been changed. Not all state is re-sent each time"]
            #[doc = "the current configuration changes: only the actual changes are sent."]
            #[doc = ""]
            #[doc = "This allows changes to the output configuration to be seen as atomic,"]
            #[doc = "even if they happen via multiple events."]
            #[doc = ""]
            #[doc = "A serial is sent to be used in a future create_configuration request."]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_manager_v1#{}.done({})", sender_id, serial);
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_uint(serial).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the compositor is done sending manager events."]
            #[doc = "The compositor will destroy the object immediately after sending this"]
            #[doc = "event, so it will become invalid and the client should release any"]
            #[doc = "resources associated with it."]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_manager_v1#{}.finished()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "A head is an output device. The difference between a wl_output object and"]
    #[doc = "a head is that heads are advertised even if they are turned off. A head"]
    #[doc = "object only advertises properties and cannot be used directly to change"]
    #[doc = "them."]
    #[doc = ""]
    #[doc = "A head has some read-only properties: modes, name, description and"]
    #[doc = "physical_size. These cannot be changed by clients."]
    #[doc = ""]
    #[doc = "Other properties can be updated via a wlr_output_configuration object."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the"]
    #[doc = "wlr_output_manager.done event. No guarantees are made regarding the order"]
    #[doc = "in which properties are sent."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_head_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AdaptiveSyncState {
            #[doc = "adaptive sync is disabled"]
            Disabled = 0u32,
            #[doc = "adaptive sync is enabled"]
            Enabled = 1u32,
        }
        impl TryFrom<u32> for AdaptiveSyncState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AdaptiveSyncState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_output_head_v1";
        pub const VERSION: u32 = 4u32;
        #[doc = "Trait to implement the zwlr_output_head_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zwlr_output_head_v1#{}.release()", sender_id,);
                            let result = self.release(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "This request indicates that the client will no longer use this head"]
            #[doc = "object."]
            fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "This event describes the head name."]
            #[doc = ""]
            #[doc = "The naming convention is compositor defined, but limited to alphanumeric"]
            #[doc = "characters and dashes (-). Each name is unique among all wlr_output_head"]
            #[doc = "objects, but if a wlr_output_head object is destroyed the same name may"]
            #[doc = "be reused later. The names will also remain consistent across sessions"]
            #[doc = "with the same hardware and software configuration."]
            #[doc = ""]
            #[doc = "Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do"]
            #[doc = "not assume that the name is a reflection of an underlying DRM"]
            #[doc = "connector, X11 connection, etc."]
            #[doc = ""]
            #[doc = "If the compositor implements the xdg-output protocol and this head is"]
            #[doc = "enabled, the xdg_output.name event must report the same name."]
            #[doc = ""]
            #[doc = "The name event is sent after a wlr_output_head object is created. This"]
            #[doc = "event is only sent once per object, and the name does not change over"]
            #[doc = "the lifetime of the wlr_output_head object."]
            fn name(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.name(\"{}\")", sender_id, name);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes a human-readable description of the head."]
            #[doc = ""]
            #[doc = "The description is a UTF-8 string with no convention defined for its"]
            #[doc = "contents. Examples might include 'Foocorp 11\" Display' or 'Virtual X11"]
            #[doc = "output via :1'. However, do not assume that the name is a reflection of"]
            #[doc = "the make, model, serial of the underlying DRM connector or the display"]
            #[doc = "name of the underlying X11 connection, etc."]
            #[doc = ""]
            #[doc = "If the compositor implements xdg-output and this head is enabled,"]
            #[doc = "the xdg_output.description must report the same description."]
            #[doc = ""]
            #[doc = "The description event is sent after a wlr_output_head object is created."]
            #[doc = "This event is only sent once per object, and the description does not"]
            #[doc = "change over the lifetime of the wlr_output_head object."]
            fn description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                description: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.description(\"{}\")",
                        sender_id,
                        description
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(description))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the physical size of the head. This event is only"]
            #[doc = "sent if the head has a physical size (e.g. is not a projector or a"]
            #[doc = "virtual device)."]
            #[doc = ""]
            #[doc = "The physical size event is sent after a wlr_output_head object is created. This"]
            #[doc = "event is only sent once per object, and the physical size does not change over"]
            #[doc = "the lifetime of the wlr_output_head object."]
            fn physical_size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.physical_size({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event introduces a mode for this head. It is sent once per"]
            #[doc = "supported mode."]
            fn mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mode: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.mode({})", sender_id, mode);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(mode))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes whether the head is enabled. A disabled head is not"]
            #[doc = "mapped to a region of the global compositor space."]
            #[doc = ""]
            #[doc = "When a head is disabled, some properties (current_mode, position,"]
            #[doc = "transform and scale) are irrelevant."]
            fn enabled(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                enabled: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.enabled({})", sender_id, enabled);
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_int(enabled).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the mode currently in use for this head. It is only"]
            #[doc = "sent if the output is enabled."]
            fn current_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mode: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.current_mode({})",
                        sender_id,
                        mode
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(mode))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This events describes the position of the head in the global compositor"]
            #[doc = "space. It is only sent if the output is enabled."]
            fn position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.position({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the transformation currently applied to the head."]
            #[doc = "It is only sent if the output is enabled."]
            fn transform(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.transform({})",
                        sender_id,
                        transform
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(transform as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This events describes the scale of the head in the global compositor"]
            #[doc = "space. It is only sent if the output is enabled."]
            fn scale(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                scale: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.scale({})", sender_id, scale);
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_fixed(scale).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 8u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the head is no longer available. The head"]
            #[doc = "object becomes inert. Clients should send a destroy request and release"]
            #[doc = "any resources associated with it."]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.finished()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 9u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the manufacturer of the head."]
            #[doc = ""]
            #[doc = "This must report the same make as the wl_output interface does in its"]
            #[doc = "geometry event."]
            #[doc = ""]
            #[doc = "Together with the model and serial_number events the purpose is to"]
            #[doc = "allow clients to recognize heads from previous sessions and for example"]
            #[doc = "load head-specific configurations back."]
            #[doc = ""]
            #[doc = "It is not guaranteed this event will be ever sent. A reason for that"]
            #[doc = "can be that the compositor does not have information about the make of"]
            #[doc = "the head or the definition of a make is not sensible in the current"]
            #[doc = "setup, for example in a virtual session. Clients can still try to"]
            #[doc = "identify the head by available information from other events but should"]
            #[doc = "be aware that there is an increased risk of false positives."]
            #[doc = ""]
            #[doc = "If sent, the make event is sent after a wlr_output_head object is"]
            #[doc = "created and only sent once per object. The make does not change over"]
            #[doc = "the lifetime of the wlr_output_head object."]
            #[doc = ""]
            #[doc = "It is not recommended to display the make string in UI to users. For"]
            #[doc = "that the string provided by the description event should be preferred."]
            fn make(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                make: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.make(\"{}\")", sender_id, make);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(make))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 10u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the model of the head."]
            #[doc = ""]
            #[doc = "This must report the same model as the wl_output interface does in its"]
            #[doc = "geometry event."]
            #[doc = ""]
            #[doc = "Together with the make and serial_number events the purpose is to"]
            #[doc = "allow clients to recognize heads from previous sessions and for example"]
            #[doc = "load head-specific configurations back."]
            #[doc = ""]
            #[doc = "It is not guaranteed this event will be ever sent. A reason for that"]
            #[doc = "can be that the compositor does not have information about the model of"]
            #[doc = "the head or the definition of a model is not sensible in the current"]
            #[doc = "setup, for example in a virtual session. Clients can still try to"]
            #[doc = "identify the head by available information from other events but should"]
            #[doc = "be aware that there is an increased risk of false positives."]
            #[doc = ""]
            #[doc = "If sent, the model event is sent after a wlr_output_head object is"]
            #[doc = "created and only sent once per object. The model does not change over"]
            #[doc = "the lifetime of the wlr_output_head object."]
            #[doc = ""]
            #[doc = "It is not recommended to display the model string in UI to users. For"]
            #[doc = "that the string provided by the description event should be preferred."]
            fn model(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                model: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_head_v1#{}.model(\"{}\")", sender_id, model);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(model))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 11u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the serial number of the head."]
            #[doc = ""]
            #[doc = "Together with the make and model events the purpose is to allow clients"]
            #[doc = "to recognize heads from previous sessions and for example load head-"]
            #[doc = "specific configurations back."]
            #[doc = ""]
            #[doc = "It is not guaranteed this event will be ever sent. A reason for that"]
            #[doc = "can be that the compositor does not have information about the serial"]
            #[doc = "number of the head or the definition of a serial number is not sensible"]
            #[doc = "in the current setup. Clients can still try to identify the head by"]
            #[doc = "available information from other events but should be aware that there"]
            #[doc = "is an increased risk of false positives."]
            #[doc = ""]
            #[doc = "If sent, the serial number event is sent after a wlr_output_head object"]
            #[doc = "is created and only sent once per object. The serial number does not"]
            #[doc = "change over the lifetime of the wlr_output_head object."]
            #[doc = ""]
            #[doc = "It is not recommended to display the serial_number string in UI to"]
            #[doc = "users. For that the string provided by the description event should be"]
            #[doc = "preferred."]
            fn serial_number(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial_number: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.serial_number(\"{}\")",
                        sender_id,
                        serial_number
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(serial_number))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 12u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes whether adaptive sync is currently enabled for"]
            #[doc = "the head or not. Adaptive sync is also known as Variable Refresh"]
            #[doc = "Rate or VRR."]
            fn adaptive_sync(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state: AdaptiveSyncState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_head_v1#{}.adaptive_sync({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(state as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 13u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "This object describes an output mode."]
    #[doc = ""]
    #[doc = "Some heads don't support output modes, in which case modes won't be"]
    #[doc = "advertised."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the"]
    #[doc = "wlr_output_manager.done event. No guarantees are made regarding the order"]
    #[doc = "in which properties are sent."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_mode_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_output_mode_v1";
        pub const VERSION: u32 = 3u32;
        #[doc = "Trait to implement the zwlr_output_mode_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zwlr_output_mode_v1#{}.release()", sender_id,);
                            let result = self.release(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "This request indicates that the client will no longer use this mode"]
            #[doc = "object."]
            fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "This event describes the mode size. The size is given in physical"]
            #[doc = "hardware units of the output device. This is not necessarily the same as"]
            #[doc = "the output size in the global compositor space. For instance, the output"]
            #[doc = "may be scaled or transformed."]
            fn size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_output_mode_v1#{}.size({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event describes the mode's fixed vertical refresh rate. It is only"]
            #[doc = "sent if the mode has a fixed refresh rate."]
            fn refresh(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                refresh: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_mode_v1#{}.refresh({})", sender_id, refresh);
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_int(refresh).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event advertises this mode as preferred."]
            fn preferred(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_mode_v1#{}.preferred()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the mode is no longer available. The mode"]
            #[doc = "object becomes inert. Clients should send a destroy request and release"]
            #[doc = "any resources associated with it."]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_mode_v1#{}.finished()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "This object is used by the client to describe a full output configuration."]
    #[doc = ""]
    #[doc = "First, the client needs to setup the output configuration. Each head can"]
    #[doc = "be either enabled (and configured) or disabled. It is a protocol error to"]
    #[doc = "send two enable_head or disable_head requests with the same head. It is a"]
    #[doc = "protocol error to omit a head in a configuration."]
    #[doc = ""]
    #[doc = "Then, the client can apply or test the configuration. The compositor will"]
    #[doc = "then reply with a succeeded, failed or cancelled event. Finally the client"]
    #[doc = "should destroy the configuration object."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_configuration_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "head has been configured twice"]
            AlreadyConfiguredHead = 1u32,
            #[doc = "head has not been configured"]
            UnconfiguredHead = 2u32,
            #[doc = "request sent after configuration has been applied or tested"]
            AlreadyUsed = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyConfiguredHead),
                    2u32 => Ok(Self::UnconfiguredHead),
                    3u32 => Ok(Self::AlreadyUsed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_output_configuration_v1";
        pub const VERSION: u32 = 4u32;
        #[doc = "Trait to implement the zwlr_output_configuration_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let head = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_output_configuration_v1#{}.enable_head({}, {})",
                                sender_id,
                                id,
                                head
                            );
                            self.enable_head(client, sender_id, id, head).await
                        }
                        1u16 => {
                            let head = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_output_configuration_v1#{}.disable_head({})",
                                sender_id,
                                head
                            );
                            self.disable_head(client, sender_id, head).await
                        }
                        2u16 => {
                            tracing::debug!("zwlr_output_configuration_v1#{}.apply()", sender_id,);
                            self.apply(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!("zwlr_output_configuration_v1#{}.test()", sender_id,);
                            self.test(client, sender_id).await
                        }
                        4u16 => {
                            tracing::debug!("zwlr_output_configuration_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Enable a head. This request creates a head configuration object that can"]
            #[doc = "be used to change the head's properties."]
            fn enable_head(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Disable a head."]
            fn disable_head(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Apply the new output configuration."]
            #[doc = ""]
            #[doc = "In case the configuration is successfully applied, there is no guarantee"]
            #[doc = "that the new output state matches completely the requested"]
            #[doc = "configuration. For instance, a compositor might round the scale if it"]
            #[doc = "doesn't support fractional scaling."]
            #[doc = ""]
            #[doc = "After this request has been sent, the compositor must respond with an"]
            #[doc = "succeeded, failed or cancelled event. Sending a request that isn't the"]
            #[doc = "destructor is a protocol error."]
            fn apply(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Test the new output configuration. The configuration won't be applied,"]
            #[doc = "but will only be validated."]
            #[doc = ""]
            #[doc = "Even if the compositor succeeds to test a configuration, applying it may"]
            #[doc = "fail."]
            #[doc = ""]
            #[doc = "After this request has been sent, the compositor must respond with an"]
            #[doc = "succeeded, failed or cancelled event. Sending a request that isn't the"]
            #[doc = "destructor is a protocol error."]
            fn test(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Using this request a client can tell the compositor that it is not going"]
            #[doc = "to use the configuration object anymore. Any changes to the outputs"]
            #[doc = "that have not been applied will be discarded."]
            #[doc = ""]
            #[doc = "This request also destroys wlr_output_configuration_head objects created"]
            #[doc = "via this object."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Sent after the compositor has successfully applied the changes or"]
            #[doc = "tested them."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            #[doc = ""]
            #[doc = "If the current configuration has changed, events to describe the changes"]
            #[doc = "will be sent followed by a wlr_output_manager.done event."]
            fn succeeded(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_configuration_v1#{}.succeeded()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Sent if the compositor rejects the changes or failed to apply them. The"]
            #[doc = "compositor should revert any changes made by the apply request that"]
            #[doc = "triggered this event."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_configuration_v1#{}.failed()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Sent if the compositor cancels the configuration because the state of an"]
            #[doc = "output changed and the client has outdated information (e.g. after an"]
            #[doc = "output has been hotplugged)."]
            #[doc = ""]
            #[doc = "The client can create a new configuration with a newer serial and try"]
            #[doc = "again."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            fn cancelled(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_configuration_v1#{}.cancelled()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = "This object is used by the client to update a single head's configuration."]
    #[doc = ""]
    #[doc = "It is a protocol error to set the same property twice."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_configuration_head_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "property has already been set"]
            AlreadySet = 1u32,
            #[doc = "mode doesn't belong to head"]
            InvalidMode = 2u32,
            #[doc = "mode is invalid"]
            InvalidCustomMode = 3u32,
            #[doc = "transform value outside enum"]
            InvalidTransform = 4u32,
            #[doc = "scale negative or zero"]
            InvalidScale = 5u32,
            #[doc = "invalid enum value used in the set_adaptive_sync request"]
            InvalidAdaptiveSyncState = 6u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadySet),
                    2u32 => Ok(Self::InvalidMode),
                    3u32 => Ok(Self::InvalidCustomMode),
                    4u32 => Ok(Self::InvalidTransform),
                    5u32 => Ok(Self::InvalidScale),
                    6u32 => Ok(Self::InvalidAdaptiveSyncState),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_output_configuration_head_v1";
        pub const VERSION: u32 = 4u32;
        #[doc = "Trait to implement the zwlr_output_configuration_head_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let mode = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_output_configuration_head_v1#{}.set_mode({})",
                                sender_id,
                                mode
                            );
                            self.set_mode(client, sender_id, mode).await
                        }
                        1u16 => {
                            let width = message.int()?;
                            let height = message.int()?;
                            let refresh = message.int()?;
                            tracing::debug!(
                                "zwlr_output_configuration_head_v1#{}.set_custom_mode({}, {}, {})",
                                sender_id,
                                width,
                                height,
                                refresh
                            );
                            self.set_custom_mode(client, sender_id, width, height, refresh)
                                .await
                        }
                        2u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            tracing::debug!(
                                "zwlr_output_configuration_head_v1#{}.set_position({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.set_position(client, sender_id, x, y).await
                        }
                        3u16 => {
                            let transform = message.uint()?;
                            tracing::debug!(
                                "zwlr_output_configuration_head_v1#{}.set_transform({})",
                                sender_id,
                                transform
                            );
                            self.set_transform(client, sender_id, transform.try_into()?)
                                .await
                        }
                        4u16 => {
                            let scale = message.fixed()?;
                            tracing::debug!(
                                "zwlr_output_configuration_head_v1#{}.set_scale({})",
                                sender_id,
                                scale
                            );
                            self.set_scale(client, sender_id, scale).await
                        }
                        5u16 => {
                            let state = message.uint()?;
                            tracing::debug!(
                                "zwlr_output_configuration_head_v1#{}.set_adaptive_sync({})",
                                sender_id,
                                state
                            );
                            self.set_adaptive_sync(client, sender_id, state.try_into()?)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "This request sets the head's mode."]
            fn set_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mode: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request assigns a custom mode to the head. The size is given in"]
            #[doc = "physical hardware units of the output device. If set to zero, the"]
            #[doc = "refresh rate is unspecified."]
            #[doc = ""]
            #[doc = "It is a protocol error to set both a mode and a custom mode."]
            fn set_custom_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
                refresh: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request sets the head's position in the global compositor space."]
            fn set_position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request sets the head's transform."]
            fn set_transform(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request sets the head's scale."]
            fn set_scale(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                scale: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "This request enables/disables adaptive sync. Adaptive sync is also"]
            #[doc = "known as Variable Refresh Rate or VRR."]
            fn set_adaptive_sync(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state : super :: super :: super :: wlr :: wlr_output_management_unstable_v1 :: zwlr_output_head_v1 :: AdaptiveSyncState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
}
#[doc = "This protocol allows clients to control power management modes"]
#[doc = "of outputs that are currently part of the compositor space. The"]
#[doc = "intent is to allow special clients like desktop shells to power"]
#[doc = "down outputs when the system is idle."]
#[doc = ""]
#[doc = "To modify outputs not currently part of the compositor space see"]
#[doc = "wlr-output-management."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod wlr_output_power_management_unstable_v1 {
    #[doc = "This interface is a manager that allows creating per-output power"]
    #[doc = "management mode controls."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_power_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_output_power_manager_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_output_power_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_output_power_manager_v1#{}.get_output_power({}, {})",
                                sender_id,
                                id,
                                output
                            );
                            self.get_output_power(client, sender_id, id, output).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_output_power_manager_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Create an output power management mode control that can be used to"]
            #[doc = "adjust the power management mode for a given output."]
            fn get_output_power(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "This object offers requests to set the power management mode of"]
    #[doc = "an output."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_power_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "Output is turned off."]
            Off = 0u32,
            #[doc = "Output is turned on, no power saving"]
            On = 1u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Off),
                    1u32 => Ok(Self::On),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "nonexistent power save mode"]
            InvalidMode = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidMode),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_output_power_v1";
        pub const VERSION: u32 = 1u32;
        #[doc = "Trait to implement the zwlr_output_power_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let mode = message.uint()?;
                            tracing::debug!(
                                "zwlr_output_power_v1#{}.set_mode({})",
                                sender_id,
                                mode
                            );
                            self.set_mode(client, sender_id, mode.try_into()?).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_output_power_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Set an output's power save mode to the given mode. The mode change"]
            #[doc = "is effective immediately. If the output does not support the given"]
            #[doc = "mode a failed event is sent."]
            fn set_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mode: Mode,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the output power management mode control object."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Report the power management mode change of an output."]
            #[doc = ""]
            #[doc = "The mode event is sent after an output changed its power"]
            #[doc = "management mode. The reason can be a client using set_mode or the"]
            #[doc = "compositor deciding to change an output's mode."]
            #[doc = "This event is also sent immediately when the object is created"]
            #[doc = "so the client is informed about the current power management mode."]
            fn mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mode: Mode,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_power_v1#{}.mode({})", sender_id, mode);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(mode as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the output power management mode control"]
            #[doc = "is no longer valid. This can happen for a number of reasons,"]
            #[doc = "including:"]
            #[doc = "- The output doesn't support power management"]
            #[doc = "- Another client already has exclusive power management mode control"]
            #[doc = "for this output"]
            #[doc = "- The output disappeared"]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_output_power_v1#{}.failed()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[doc = "This protocol allows clients to ask the compositor to copy part of the"]
#[doc = "screen content to a client buffer."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod wlr_screencopy_unstable_v1 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_screencopy_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_screencopy_manager_v1";
        pub const VERSION: u32 = 3u32;
        #[doc = "Trait to implement the zwlr_screencopy_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let frame = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let overlay_cursor = message.int()?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_screencopy_manager_v1#{}.capture_output({}, {}, {})",
                                sender_id,
                                frame,
                                overlay_cursor,
                                output
                            );
                            self.capture_output(client, sender_id, frame, overlay_cursor, output)
                                .await
                        }
                        1u16 => {
                            let frame = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let overlay_cursor = message.int()?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            tracing::debug!(
                                "zwlr_screencopy_manager_v1#{}.capture_output_region({}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                frame,
                                overlay_cursor,
                                output,
                                x,
                                y,
                                width,
                                height
                            );
                            self.capture_output_region(
                                client,
                                sender_id,
                                frame,
                                overlay_cursor,
                                output,
                                x,
                                y,
                                width,
                                height,
                            )
                            .await
                        }
                        2u16 => {
                            tracing::debug!("zwlr_screencopy_manager_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Capture the next frame of an entire output."]
            fn capture_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Capture the next frame of an output's region."]
            #[doc = ""]
            #[doc = "The region is given in output logical coordinates, see"]
            #[doc = "xdg_output.logical_size. The region will be clipped to the output's"]
            #[doc = "extents."]
            fn capture_output_region(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                output: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "This object represents a single frame."]
    #[doc = ""]
    #[doc = "When created, a series of buffer events will be sent, each representing a"]
    #[doc = "supported buffer type. The \"buffer_done\" event is sent afterwards to"]
    #[doc = "indicate that all supported buffer types have been enumerated. The client"]
    #[doc = "will then be able to send a \"copy\" request. If the capture is successful,"]
    #[doc = "the compositor will send a \"flags\" followed by a \"ready\" event."]
    #[doc = ""]
    #[doc = "For objects version 2 or lower, wl_shm buffers are always supported, ie."]
    #[doc = "the \"buffer\" event is guaranteed to be sent."]
    #[doc = ""]
    #[doc = "If the capture failed, the \"failed\" event is sent. This can happen anytime"]
    #[doc = "before the \"ready\" event."]
    #[doc = ""]
    #[doc = "Once either a \"ready\" or a \"failed\" event is received, the client should"]
    #[doc = "destroy the frame."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_screencopy_frame_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the object has already been used to copy a wl_buffer"]
            AlreadyUsed = 0u32,
            #[doc = "buffer attributes are invalid"]
            InvalidBuffer = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    1u32 => Ok(Self::InvalidBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInvert = 1u32 ; } }
        impl TryFrom<u32> for Flags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_screencopy_frame_v1";
        pub const VERSION: u32 = 3u32;
        #[doc = "Trait to implement the zwlr_screencopy_frame_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_screencopy_frame_v1#{}.copy({})",
                                sender_id,
                                buffer
                            );
                            self.copy(client, sender_id, buffer).await
                        }
                        1u16 => {
                            tracing::debug!("zwlr_screencopy_frame_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        2u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_screencopy_frame_v1#{}.copy_with_damage({})",
                                sender_id,
                                buffer
                            );
                            self.copy_with_damage(client, sender_id, buffer).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Copy the frame to the supplied buffer. The buffer must have a the"]
            #[doc = "correct size, see zwlr_screencopy_frame_v1.buffer and"]
            #[doc = "zwlr_screencopy_frame_v1.linux_dmabuf. The buffer needs to have a"]
            #[doc = "supported format."]
            #[doc = ""]
            #[doc = "If the frame is successfully copied, a \"flags\" and a \"ready\" events are"]
            #[doc = "sent. Otherwise, a \"failed\" event is sent."]
            fn copy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the frame. This request can be sent at any time by the client."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Same as copy, except it waits until there is damage to copy."]
            fn copy_with_damage(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {
            #[doc = "Provides information about wl_shm buffer parameters that need to be"]
            #[doc = "used for this frame. This event is sent once after the frame is created"]
            #[doc = "if wl_shm buffers are supported."]
            fn buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: super::super::super::core::wayland::wl_shm::Format,
                width: u32,
                height: u32,
                stride: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_screencopy_frame_v1#{}.buffer({}, {}, {}, {})",
                        sender_id,
                        format,
                        width,
                        height,
                        stride
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(format as u32)
                        .put_uint(width)
                        .put_uint(height)
                        .put_uint(stride)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Provides flags about the frame. This event is sent once before the"]
            #[doc = "\"ready\" event."]
            fn flags(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                flags: Flags,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_screencopy_frame_v1#{}.flags({})", sender_id, flags);
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(flags.bits())
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading. This event includes the time at which presentation happened"]
            #[doc = "at."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part"]
            #[doc = "may have an arbitrary offset at start."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_screencopy_frame_v1#{}.ready({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_screencopy_frame_v1#{}.failed()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent right before the ready event when copy_with_damage is"]
            #[doc = "requested. It may be generated multiple times for each copy_with_damage"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "The arguments describe a box around an area that has changed since the"]
            #[doc = "last copy request that was derived from the current screencopy manager"]
            #[doc = "instance."]
            #[doc = ""]
            #[doc = "The union of all regions received between the call to copy_with_damage"]
            #[doc = "and a ready event is the total damage since the prior ready event."]
            fn damage(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_screencopy_frame_v1#{}.damage({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(x)
                        .put_uint(y)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Provides information about linux-dmabuf buffer parameters that need to"]
            #[doc = "be used for this frame. This event is sent once after the frame is"]
            #[doc = "created if linux-dmabuf buffers are supported."]
            fn linux_dmabuf(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: u32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zwlr_screencopy_frame_v1#{}.linux_dmabuf({}, {}, {})",
                        sender_id,
                        format,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(format)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent once after all buffer events have been sent."]
            #[doc = ""]
            #[doc = "The client should proceed to create a buffer of one of the supported"]
            #[doc = "types, and send a \"copy\" request."]
            fn buffer_done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zwlr_screencopy_frame_v1#{}.buffer_done()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod wlr_virtual_pointer_unstable_v1 {
    #[doc = "This protocol allows clients to emulate a physical pointer device. The"]
    #[doc = "requests are mostly mirror opposites of those specified in wl_pointer."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_virtual_pointer_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "client sent invalid axis enumeration value"]
            InvalidAxis = 0u32,
            #[doc = "client sent invalid axis source enumeration value"]
            InvalidAxisSource = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidAxis),
                    1u32 => Ok(Self::InvalidAxisSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        pub const INTERFACE: &'static str = "zwlr_virtual_pointer_v1";
        pub const VERSION: u32 = 2u32;
        #[doc = "Trait to implement the zwlr_virtual_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let time = message.uint()?;
                            let dx = message.fixed()?;
                            let dy = message.fixed()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.motion({}, {}, {})",
                                sender_id,
                                time,
                                dx,
                                dy
                            );
                            self.motion(client, sender_id, time, dx, dy).await
                        }
                        1u16 => {
                            let time = message.uint()?;
                            let x = message.uint()?;
                            let y = message.uint()?;
                            let x_extent = message.uint()?;
                            let y_extent = message.uint()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.motion_absolute({}, {}, {}, {}, {})",
                                sender_id,
                                time,
                                x,
                                y,
                                x_extent,
                                y_extent
                            );
                            self.motion_absolute(client, sender_id, time, x, y, x_extent, y_extent)
                                .await
                        }
                        2u16 => {
                            let time = message.uint()?;
                            let button = message.uint()?;
                            let state = message.uint()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.button({}, {}, {})",
                                sender_id,
                                time,
                                button,
                                state
                            );
                            self.button(client, sender_id, time, button, state.try_into()?)
                                .await
                        }
                        3u16 => {
                            let time = message.uint()?;
                            let axis = message.uint()?;
                            let value = message.fixed()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.axis({}, {}, {})",
                                sender_id,
                                time,
                                axis,
                                value
                            );
                            self.axis(client, sender_id, time, axis.try_into()?, value)
                                .await
                        }
                        4u16 => {
                            tracing::debug!("zwlr_virtual_pointer_v1#{}.frame()", sender_id,);
                            self.frame(client, sender_id).await
                        }
                        5u16 => {
                            let axis_source = message.uint()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.axis_source({})",
                                sender_id,
                                axis_source
                            );
                            self.axis_source(client, sender_id, axis_source.try_into()?)
                                .await
                        }
                        6u16 => {
                            let time = message.uint()?;
                            let axis = message.uint()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.axis_stop({}, {})",
                                sender_id,
                                time,
                                axis
                            );
                            self.axis_stop(client, sender_id, time, axis.try_into()?)
                                .await
                        }
                        7u16 => {
                            let time = message.uint()?;
                            let axis = message.uint()?;
                            let value = message.fixed()?;
                            let discrete = message.int()?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_v1#{}.axis_discrete({}, {}, {}, {})",
                                sender_id,
                                time,
                                axis,
                                value,
                                discrete
                            );
                            self.axis_discrete(
                                client,
                                sender_id,
                                time,
                                axis.try_into()?,
                                value,
                                discrete,
                            )
                            .await
                        }
                        8u16 => {
                            tracing::debug!("zwlr_virtual_pointer_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "The pointer has moved by a relative amount to the previous request."]
            #[doc = ""]
            #[doc = "Values are in the global compositor space."]
            fn motion(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                dx: crate::wire::Fixed,
                dy: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "The pointer has moved in an absolute coordinate frame."]
            #[doc = ""]
            #[doc = "Value of x can range from 0 to x_extent, value of y can range from 0"]
            #[doc = "to y_extent."]
            fn motion_absolute(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                x: u32,
                y: u32,
                x_extent: u32,
                y_extent: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "A button was pressed or released."]
            fn button(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                button: u32,
                state: super::super::super::core::wayland::wl_pointer::ButtonState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Scroll and other axis requests."]
            fn axis(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                axis: super::super::super::core::wayland::wl_pointer::Axis,
                value: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Indicates the set of events that logically belong together."]
            fn frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Source information for scroll and other axis."]
            fn axis_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                axis_source: super::super::super::core::wayland::wl_pointer::AxisSource,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Stop notification for scroll and other axes."]
            fn axis_stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                axis: super::super::super::core::wayland::wl_pointer::Axis,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Discrete step information for scroll and other axes."]
            #[doc = ""]
            #[doc = "This event allows the client to extend data normally sent using the axis"]
            #[doc = "event with discrete value."]
            fn axis_discrete(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                axis: super::super::super::core::wayland::wl_pointer::Axis,
                value: crate::wire::Fixed,
                discrete: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
    #[doc = "This object allows clients to create individual virtual pointer objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_virtual_pointer_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        pub const INTERFACE: &'static str = "zwlr_virtual_pointer_manager_v1";
        pub const VERSION: u32 = 2u32;
        #[doc = "Trait to implement the zwlr_virtual_pointer_manager_v1 interface. See the module level documentation for more info"]
        pub trait ServerHandler: Requests + Events + crate::server::Dispatcher {
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let seat = message.object()?;
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer({}, {})",
                                sender_id,
                                seat.as_ref().map_or("null".to_string(), |v| v.to_string()),
                                id
                            );
                            self.create_virtual_pointer(client, sender_id, seat, id)
                                .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zwlr_virtual_pointer_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        2u16 => {
                            let seat = message.object()?;
                            let output = message.object()?;
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer_with_output({}, {}, {})",
                                sender_id,
                                seat.as_ref().map_or("null".to_string(), |v| v.to_string()),
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                id
                            );
                            self.create_virtual_pointer_with_output(
                                client, sender_id, seat, output, id,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
        }
        pub trait Requests {
            #[doc = "Creates a new virtual pointer. The optional seat is a suggestion to the"]
            #[doc = "compositor."]
            fn create_virtual_pointer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: Option<crate::wire::ObjectId>,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Creates a new virtual pointer. The seat and the output arguments are"]
            #[doc = "optional. If the seat argument is set, the compositor should assign the"]
            #[doc = "input device to the requested seat. If the output argument is set, the"]
            #[doc = "compositor should map the input device to the requested output."]
            fn create_virtual_pointer_with_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: Option<crate::wire::ObjectId>,
                output: Option<crate::wire::ObjectId>,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
        pub trait Events {}
    }
}
