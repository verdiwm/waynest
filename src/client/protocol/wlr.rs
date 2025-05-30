#![allow(async_fn_in_trait)]
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_data_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlManagerV1 {
            const INTERFACE: &'static str = "zwlr_data_control_manager_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a new data source."]
            async fn create_data_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_data_control_manager_v1#{}.create_data_source()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a data device that can be used to manage a seat's selection."]
            async fn get_data_device(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_data_control_manager_v1#{}.get_data_device()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_data_control_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This interface allows a client to manage a seat's selection."]
    #[doc = ""]
    #[doc = "When the seat is destroyed, this object becomes inert."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_device_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_data_control_device_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlDeviceV1 {
            const INTERFACE: &'static str = "zwlr_data_control_device_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request asks the compositor to set the selection to the data from"]
            #[doc = "the source on behalf of the client."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "set_primary_selection requests. Attempting to use a previously used"]
            #[doc = "source is a protocol error."]
            #[doc = ""]
            #[doc = "To unset the selection, set the source to NULL."]
            async fn set_selection(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_data_control_device_v1#{}.set_selection()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the data device object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_data_control_device_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn set_primary_selection(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_data_control_device_v1#{}.set_primary_selection()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The data_offer event introduces a new wlr_data_control_offer object,"]
            #[doc = "which will subsequently be used in either the"]
            #[doc = "wlr_data_control_device.selection event (for the regular clipboard"]
            #[doc = "selections) or the wlr_data_control_device.primary_selection event (for"]
            #[doc = "the primary clipboard selections). Immediately following the"]
            #[doc = "wlr_data_control_device.data_offer event, the new data_offer object"]
            #[doc = "will send out wlr_data_control_offer.offer events to describe the MIME"]
            #[doc = "types it offers."]
            async fn data_offer(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
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
            async fn selection(
                &self,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()>;
            #[doc = "This data control object is no longer valid and should be destroyed by"]
            #[doc = "the client."]
            async fn finished(&self) -> crate::client::Result<()>;
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
            async fn primary_selection(
                &self,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "The wlr_data_control_source object is the source side of a"]
    #[doc = "wlr_data_control_offer. It is created by the source client in a data"]
    #[doc = "transfer and provides a way to describe the offered data and a way to"]
    #[doc = "respond to requests to transfer the data."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_source_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_data_control_source_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlSourceV1 {
            const INTERFACE: &'static str = "zwlr_data_control_source_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request adds a MIME type to the set of MIME types advertised to"]
            #[doc = "targets. Can be called several times to offer multiple types."]
            #[doc = ""]
            #[doc = "Calling this after wlr_data_control_device.set_selection is a protocol"]
            #[doc = "error."]
            async fn offer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_data_control_source_v1#{}.offer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the data source object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_data_control_source_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request for data from the client. Send the data as the specified MIME"]
            #[doc = "type over the passed file descriptor, then close it."]
            async fn send(
                &self,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()>;
            #[doc = "This data source is no longer valid. The data source has been replaced"]
            #[doc = "by another data source."]
            #[doc = ""]
            #[doc = "The client should clean up and destroy this data source."]
            async fn cancelled(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "A wlr_data_control_offer represents a piece of data offered for transfer"]
    #[doc = "by another client (the source client). The offer describes the different"]
    #[doc = "MIME types that the data can be converted to and provides the mechanism"]
    #[doc = "for transferring the data directly from the source client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_data_control_offer_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_data_control_offer_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlOfferV1 {
            const INTERFACE: &'static str = "zwlr_data_control_offer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
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
            async fn receive(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_data_control_offer_v1#{}.receive()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .put_fd(fd)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the data offer object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_data_control_offer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sent immediately after creating the wlr_data_control_offer object."]
            #[doc = "One event per offered MIME type."]
            async fn offer(&self, mime_type: String) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_export_dmabuf_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrExportDmabufManagerV1 {
            const INTERFACE: &'static str = "zwlr_export_dmabuf_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Capture the next frame of an entire output."]
            async fn capture_output(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_export_dmabuf_manager_v1#{}.capture_output()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .put_int(overlay_cursor)
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_export_dmabuf_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_export_dmabuf_frame_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrExportDmabufFrameV1 {
            const INTERFACE: &'static str = "zwlr_export_dmabuf_frame_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Unreferences the frame. This request must be called as soon as its no"]
            #[doc = "longer used."]
            #[doc = ""]
            #[doc = "It can be called at any time by the client. The client will still have"]
            #[doc = "to close any FDs it has been given."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_export_dmabuf_frame_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Main event supplying the client with information about the frame. If the"]
            #[doc = "capture didn't fail, this event is always emitted first before any other"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "This event is followed by a number of \"object\" as specified by the"]
            #[doc = "\"num_objects\" argument."]
            async fn frame(
                &self,
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
            ) -> crate::client::Result<()>;
            #[doc = "Event which serves to supply the client with the file descriptors"]
            #[doc = "containing the data for each object."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must always close the file"]
            #[doc = "descriptor as soon as they're done with it and even if the frame fails."]
            async fn object(
                &self,
                index: u32,
                fd: rustix::fd::OwnedFd,
                size: u32,
                offset: u32,
                stride: u32,
                plane_index: u32,
            ) -> crate::client::Result<()>;
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
            async fn ready(
                &self,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
            #[doc = "If the capture failed or if the frame is no longer valid after the"]
            #[doc = "\"frame\" event has been emitted, this event will be used to inform the"]
            #[doc = "client to scrap the frame."]
            #[doc = ""]
            #[doc = "If the failure is temporary, the client may capture again the same"]
            #[doc = "source. If the failure is permanent, any further attempts to capture the"]
            #[doc = "same source will fail again."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy this object."]
            async fn cancel(&self, reason: CancelReason) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_foreign_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrForeignToplevelManagerV1 {
            const INTERFACE: &'static str = "zwlr_foreign_toplevel_manager_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Indicates the client no longer wishes to receive events for new toplevels."]
            #[doc = "However the compositor may emit further toplevel_created events, until"]
            #[doc = "the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            async fn stop(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_foreign_toplevel_manager_v1#{}.stop()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is emitted whenever a new toplevel window is created. It"]
            #[doc = "is emitted for all toplevels, regardless of the app that has created"]
            #[doc = "them."]
            #[doc = ""]
            #[doc = "All initial details of the toplevel(title, app_id, states, etc.) will"]
            #[doc = "be sent immediately after this event via the corresponding events in"]
            #[doc = "zwlr_foreign_toplevel_handle_v1."]
            async fn toplevel(&self, toplevel: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event indicates that the compositor is done sending events to the"]
            #[doc = "zwlr_foreign_toplevel_manager_v1. The server will destroy the object"]
            #[doc = "immediately after sending this request, so it will become invalid and"]
            #[doc = "the client should free any resources associated with it."]
            async fn finished(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "A zwlr_foreign_toplevel_handle_v1 object represents an opened toplevel"]
    #[doc = "window. Each app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Each toplevel has a list of outputs it is visible on, conveyed to the"]
    #[doc = "client with the output_enter and output_leave events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_foreign_toplevel_handle_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrForeignToplevelHandleV1 {
            const INTERFACE: &'static str = "zwlr_foreign_toplevel_handle_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Requests that the toplevel be maximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            async fn set_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.set_maximized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the toplevel be unmaximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            async fn unset_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.unset_maximized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the toplevel be minimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            async fn set_minimized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.set_minimized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the toplevel be unminimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            async fn unset_minimized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.unset_minimized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request that this toplevel be activated on the given seat."]
            #[doc = "There is no guarantee the toplevel will be actually activated."]
            async fn activate(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.activate()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Send a request to the toplevel to close itself. The compositor would"]
            #[doc = "typically use a shell-specific method to carry out this request, for"]
            #[doc = "example by sending the xdg_toplevel.close event. However, this gives"]
            #[doc = "no guarantees the toplevel will actually be destroyed. If and when"]
            #[doc = "this happens, the zwlr_foreign_toplevel_handle_v1.closed event will"]
            #[doc = "be emitted."]
            async fn close(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_foreign_toplevel_handle_v1#{}.close()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn set_rectangle(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.set_rectangle()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the zwlr_foreign_toplevel_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_foreign_toplevel_handle_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the toplevel be fullscreened on the given output. If the"]
            #[doc = "fullscreen state and/or the outputs the toplevel is visible on actually"]
            #[doc = "change, this will be indicated by the state and output_enter/leave"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "The output parameter is only a hint to the compositor. Also, if output"]
            #[doc = "is NULL, the compositor should decide which output the toplevel will be"]
            #[doc = "fullscreened on, if at all."]
            async fn set_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.set_fullscreen()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the toplevel be unfullscreened. If the fullscreen state"]
            #[doc = "actually changes, this will be indicated by the state event."]
            async fn unset_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_foreign_toplevel_handle_v1#{}.unset_fullscreen()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            async fn title(&self, title: String) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the app-id of the toplevel changes."]
            async fn app_id(&self, app_id: String) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel becomes visible on"]
            #[doc = "the given output. A toplevel may be visible on multiple outputs."]
            async fn output_enter(
                &self,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel stops being visible on"]
            #[doc = "the given output. It is guaranteed that an entered-output event"]
            #[doc = "with the same output has been emitted before this event."]
            async fn output_leave(
                &self,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted immediately after the zlw_foreign_toplevel_handle_v1"]
            #[doc = "is created and each time the toplevel state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            async fn state(&self, state: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "This event is sent after all changes in the toplevel state have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "This allows changes to the zwlr_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "This event means the toplevel has been destroyed. It is guaranteed there"]
            #[doc = "won't be any more events for this zwlr_foreign_toplevel_handle_v1. The"]
            #[doc = "toplevel itself becomes inert so any requests will be ignored except the"]
            #[doc = "destroy request."]
            async fn closed(&self) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the parent of the toplevel changes."]
            #[doc = ""]
            #[doc = "No event is emitted when the parent handle is destroyed by the client."]
            async fn parent(
                &self,
                parent: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_gamma_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrGammaControlManagerV1 {
            const INTERFACE: &'static str = "zwlr_gamma_control_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a gamma control that can be used to adjust gamma tables for the"]
            #[doc = "provided output."]
            async fn get_gamma_control(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_gamma_control_manager_v1#{}.get_gamma_control()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_gamma_control_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_gamma_control_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrGammaControlV1 {
            const INTERFACE: &'static str = "zwlr_gamma_control_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Set the gamma table. The file descriptor can be memory-mapped to provide"]
            #[doc = "the raw gamma table, which contains successive gamma ramps for the red,"]
            #[doc = "green and blue channels. Each gamma ramp is an array of 16-byte unsigned"]
            #[doc = "integers which has the same length as the gamma size."]
            #[doc = ""]
            #[doc = "The file descriptor data must have the same length as three times the"]
            #[doc = "gamma size."]
            async fn set_gamma(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_gamma_control_v1#{}.set_gamma()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fd(fd).build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the gamma control object. If the object is still valid, this"]
            #[doc = "restores the original gamma tables."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_gamma_control_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Advertise the size of each gamma ramp."]
            #[doc = ""]
            #[doc = "This event is sent immediately when the gamma control object is created."]
            async fn gamma_size(&self, size: u32) -> crate::client::Result<()>;
            #[doc = "This event indicates that the gamma control is no longer valid. This"]
            #[doc = "can happen for a number of reasons, including:"]
            #[doc = "- The output doesn't support gamma tables"]
            #[doc = "- Setting the gamma tables failed"]
            #[doc = "- Another client already has exclusive gamma control for this output"]
            #[doc = "- The compositor has transferred gamma control to another client"]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            async fn failed(&self) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_input_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrInputInhibitManagerV1 {
            const INTERFACE: &'static str = "zwlr_input_inhibit_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Activates the input inhibitor. As long as the inhibitor is active, the"]
            #[doc = "compositor will not send input events to other clients."]
            async fn get_inhibitor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_input_inhibit_manager_v1#{}.get_inhibitor()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_input_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrInputInhibitorV1 {
            const INTERFACE: &'static str = "zwlr_input_inhibitor_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the inhibitor and allow other clients to receive input."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_input_inhibitor_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_layer_shell_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrLayerShellV1 {
            const INTERFACE: &'static str = "zwlr_layer_shell_v1";
            const VERSION: u32 = 5u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
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
            async fn get_layer_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
                layer: Layer,
                namespace: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_shell_v1#{}.get_layer_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(output)
                    .put_uint(layer as u32)
                    .put_string(Some(namespace))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request indicates that the client will not use the layer_shell"]
            #[doc = "object any more. Objects that have been created through this instance"]
            #[doc = "are not affected."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_shell_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_layer_surface_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrLayerSurfaceV1 {
            const INTERFACE: &'static str = "zwlr_layer_surface_v1";
            const VERSION: u32 = 5u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
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
            async fn set_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.set_size()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(width)
                    .put_uint(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the compositor anchor the surface to the specified edges"]
            #[doc = "and corners. If two orthogonal edges are specified (e.g. 'top' and"]
            #[doc = "'left'), then the anchor point will be the intersection of the edges"]
            #[doc = "(e.g. the top left corner of the output); otherwise the anchor point"]
            #[doc = "will be centered on that edge, or in the center if none is specified."]
            #[doc = ""]
            #[doc = "Anchor is double-buffered, see wl_surface.commit."]
            async fn set_anchor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                anchor: Anchor,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.set_anchor()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(anchor.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn set_exclusive_zone(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                zone: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_layer_surface_v1#{}.set_exclusive_zone()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(zone).build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests that the surface be placed some distance away from the anchor"]
            #[doc = "point on the output, in surface-local coordinates. Setting this value"]
            #[doc = "for edges you are not anchored to has no effect."]
            #[doc = ""]
            #[doc = "The exclusive zone includes the margin."]
            #[doc = ""]
            #[doc = "Margin is double-buffered, see wl_surface.commit."]
            async fn set_margin(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                top: i32,
                right: i32,
                bottom: i32,
                left: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.set_margin()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(top)
                    .put_int(right)
                    .put_int(bottom)
                    .put_int(left)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn set_keyboard_interactivity(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                keyboard_interactivity: KeyboardInteractivity,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_layer_surface_v1#{}.set_keyboard_interactivity()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(keyboard_interactivity as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This assigns an xdg_popup's parent to this layer_surface.  This popup"]
            #[doc = "should have been created via xdg_surface::get_popup with the parent set"]
            #[doc = "to NULL, and this request must be invoked before committing the popup's"]
            #[doc = "initial state."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_popup for more details about what an"]
            #[doc = "xdg_popup is and how it is used."]
            async fn get_popup(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                popup: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.get_popup()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(popup))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn ack_configure(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.ack_configure()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request destroys the layer surface."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Change the layer that the surface is rendered on."]
            #[doc = ""]
            #[doc = "Layer is double-buffered, see wl_surface.commit."]
            async fn set_layer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                layer : super :: super :: super :: wlr :: wlr_layer_shell_unstable_v1 :: zwlr_layer_shell_v1 :: Layer,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_layer_surface_v1#{}.set_layer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests an edge for the exclusive zone to apply. The exclusive"]
            #[doc = "edge will be automatically deduced from anchor points when possible,"]
            #[doc = "but when the surface is anchored to a corner, it will be necessary"]
            #[doc = "to set it explicitly to disambiguate, as it is not possible to deduce"]
            #[doc = "which one of the two corner edges should be used."]
            #[doc = ""]
            #[doc = "The edge must be one the surface is anchored to, otherwise the"]
            #[doc = "invalid_exclusive_edge protocol error will be raised."]
            async fn set_exclusive_edge(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                edge: Anchor,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_layer_surface_v1#{}.set_exclusive_edge()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(edge.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn configure(
                &self,
                serial: u32,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
            #[doc = "The closed event is sent by the compositor when the surface will no"]
            #[doc = "longer be shown. The output may have been destroyed or the user may"]
            #[doc = "have asked for it to be removed. Further changes to the surface will be"]
            #[doc = "ignored. The client should destroy the resource after receiving this"]
            #[doc = "event, and create a new surface if they so choose."]
            async fn closed(&self) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputManagerV1 {
            const INTERFACE: &'static str = "zwlr_output_manager_v1";
            const VERSION: u32 = 4u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a new output configuration object. This allows to update head"]
            #[doc = "properties."]
            async fn create_configuration(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_manager_v1#{}.create_configuration()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Indicates the client no longer wishes to receive events for output"]
            #[doc = "configuration changes. However the compositor may emit further events,"]
            #[doc = "until the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            async fn stop(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_manager_v1#{}.stop()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event introduces a new head. This happens whenever a new head"]
            #[doc = "appears (e.g. a monitor is plugged in) or after the output manager is"]
            #[doc = "bound."]
            async fn head(&self, head: crate::wire::ObjectId) -> crate::client::Result<()>;
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
            async fn done(&self, serial: u32) -> crate::client::Result<()>;
            #[doc = "This event indicates that the compositor is done sending manager events."]
            #[doc = "The compositor will destroy the object immediately after sending this"]
            #[doc = "event, so it will become invalid and the client should release any"]
            #[doc = "resources associated with it."]
            async fn finished(&self) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_output_head_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputHeadV1 {
            const INTERFACE: &'static str = "zwlr_output_head_v1";
            const VERSION: u32 = 4u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request indicates that the client will no longer use this head"]
            #[doc = "object."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_head_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn name(&self, name: String) -> crate::client::Result<()>;
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
            async fn description(&self, description: String) -> crate::client::Result<()>;
            #[doc = "This event describes the physical size of the head. This event is only"]
            #[doc = "sent if the head has a physical size (e.g. is not a projector or a"]
            #[doc = "virtual device)."]
            #[doc = ""]
            #[doc = "The physical size event is sent after a wlr_output_head object is created. This"]
            #[doc = "event is only sent once per object, and the physical size does not change over"]
            #[doc = "the lifetime of the wlr_output_head object."]
            async fn physical_size(&self, width: i32, height: i32) -> crate::client::Result<()>;
            #[doc = "This event introduces a mode for this head. It is sent once per"]
            #[doc = "supported mode."]
            async fn mode(&self, mode: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event describes whether the head is enabled. A disabled head is not"]
            #[doc = "mapped to a region of the global compositor space."]
            #[doc = ""]
            #[doc = "When a head is disabled, some properties (current_mode, position,"]
            #[doc = "transform and scale) are irrelevant."]
            async fn enabled(&self, enabled: i32) -> crate::client::Result<()>;
            #[doc = "This event describes the mode currently in use for this head. It is only"]
            #[doc = "sent if the output is enabled."]
            async fn current_mode(&self, mode: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This events describes the position of the head in the global compositor"]
            #[doc = "space. It is only sent if the output is enabled."]
            async fn position(&self, x: i32, y: i32) -> crate::client::Result<()>;
            #[doc = "This event describes the transformation currently applied to the head."]
            #[doc = "It is only sent if the output is enabled."]
            async fn transform(
                &self,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::client::Result<()>;
            #[doc = "This events describes the scale of the head in the global compositor"]
            #[doc = "space. It is only sent if the output is enabled."]
            async fn scale(&self, scale: crate::wire::Fixed) -> crate::client::Result<()>;
            #[doc = "This event indicates that the head is no longer available. The head"]
            #[doc = "object becomes inert. Clients should send a destroy request and release"]
            #[doc = "any resources associated with it."]
            async fn finished(&self) -> crate::client::Result<()>;
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
            async fn make(&self, make: String) -> crate::client::Result<()>;
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
            async fn model(&self, model: String) -> crate::client::Result<()>;
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
            async fn serial_number(&self, serial_number: String) -> crate::client::Result<()>;
            #[doc = "This event describes whether adaptive sync is currently enabled for"]
            #[doc = "the head or not. Adaptive sync is also known as Variable Refresh"]
            #[doc = "Rate or VRR."]
            async fn adaptive_sync(&self, state: AdaptiveSyncState) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_output_mode_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputModeV1 {
            const INTERFACE: &'static str = "zwlr_output_mode_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request indicates that the client will no longer use this mode"]
            #[doc = "object."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_mode_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event describes the mode size. The size is given in physical"]
            #[doc = "hardware units of the output device. This is not necessarily the same as"]
            #[doc = "the output size in the global compositor space. For instance, the output"]
            #[doc = "may be scaled or transformed."]
            async fn size(&self, width: i32, height: i32) -> crate::client::Result<()>;
            #[doc = "This event describes the mode's fixed vertical refresh rate. It is only"]
            #[doc = "sent if the mode has a fixed refresh rate."]
            async fn refresh(&self, refresh: i32) -> crate::client::Result<()>;
            #[doc = "This event advertises this mode as preferred."]
            async fn preferred(&self) -> crate::client::Result<()>;
            #[doc = "This event indicates that the mode is no longer available. The mode"]
            #[doc = "object becomes inert. Clients should send a destroy request and release"]
            #[doc = "any resources associated with it."]
            async fn finished(&self) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_output_configuration_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputConfigurationV1 {
            const INTERFACE: &'static str = "zwlr_output_configuration_v1";
            const VERSION: u32 = 4u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Enable a head. This request creates a head configuration object that can"]
            #[doc = "be used to change the head's properties."]
            async fn enable_head(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_v1#{}.enable_head()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(head))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Disable a head."]
            async fn disable_head(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_v1#{}.disable_head()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(head))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn apply(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_configuration_v1#{}.apply()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Test the new output configuration. The configuration won't be applied,"]
            #[doc = "but will only be validated."]
            #[doc = ""]
            #[doc = "Even if the compositor succeeds to test a configuration, applying it may"]
            #[doc = "fail."]
            #[doc = ""]
            #[doc = "After this request has been sent, the compositor must respond with an"]
            #[doc = "succeeded, failed or cancelled event. Sending a request that isn't the"]
            #[doc = "destructor is a protocol error."]
            async fn test(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_configuration_v1#{}.test()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Using this request a client can tell the compositor that it is not going"]
            #[doc = "to use the configuration object anymore. Any changes to the outputs"]
            #[doc = "that have not been applied will be discarded."]
            #[doc = ""]
            #[doc = "This request also destroys wlr_output_configuration_head objects created"]
            #[doc = "via this object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_configuration_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sent after the compositor has successfully applied the changes or"]
            #[doc = "tested them."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            #[doc = ""]
            #[doc = "If the current configuration has changed, events to describe the changes"]
            #[doc = "will be sent followed by a wlr_output_manager.done event."]
            async fn succeeded(&self) -> crate::client::Result<()>;
            #[doc = "Sent if the compositor rejects the changes or failed to apply them. The"]
            #[doc = "compositor should revert any changes made by the apply request that"]
            #[doc = "triggered this event."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            async fn failed(&self) -> crate::client::Result<()>;
            #[doc = "Sent if the compositor cancels the configuration because the state of an"]
            #[doc = "output changed and the client has outdated information (e.g. after an"]
            #[doc = "output has been hotplugged)."]
            #[doc = ""]
            #[doc = "The client can create a new configuration with a newer serial and try"]
            #[doc = "again."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            async fn cancelled(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "This object is used by the client to update a single head's configuration."]
    #[doc = ""]
    #[doc = "It is a protocol error to set the same property twice."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_configuration_head_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_output_configuration_head_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputConfigurationHeadV1 {
            const INTERFACE: &'static str = "zwlr_output_configuration_head_v1";
            const VERSION: u32 = 4u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request sets the head's mode."]
            async fn set_mode(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mode: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_head_v1#{}.set_mode()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(mode))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request assigns a custom mode to the head. The size is given in"]
            #[doc = "physical hardware units of the output device. If set to zero, the"]
            #[doc = "refresh rate is unspecified."]
            #[doc = ""]
            #[doc = "It is a protocol error to set both a mode and a custom mode."]
            async fn set_custom_mode(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
                refresh: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_head_v1#{}.set_custom_mode()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .put_int(refresh)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the head's position in the global compositor space."]
            async fn set_position(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_head_v1#{}.set_position()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the head's transform."]
            async fn set_transform(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_head_v1#{}.set_transform()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(transform as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the head's scale."]
            async fn set_scale(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                scale: crate::wire::Fixed,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_head_v1#{}.set_scale()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fixed(scale).build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request enables/disables adaptive sync. Adaptive sync is also"]
            #[doc = "known as Variable Refresh Rate or VRR."]
            async fn set_adaptive_sync(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                state : super :: super :: super :: wlr :: wlr_output_management_unstable_v1 :: zwlr_output_head_v1 :: AdaptiveSyncState,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_configuration_head_v1#{}.set_adaptive_sync()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(state as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_output_power_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputPowerManagerV1 {
            const INTERFACE: &'static str = "zwlr_output_power_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create an output power management mode control that can be used to"]
            #[doc = "adjust the power management mode for a given output."]
            async fn get_output_power(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_output_power_manager_v1#{}.get_output_power()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_power_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object offers requests to set the power management mode of"]
    #[doc = "an output."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_output_power_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_output_power_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputPowerV1 {
            const INTERFACE: &'static str = "zwlr_output_power_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Set an output's power save mode to the given mode. The mode change"]
            #[doc = "is effective immediately. If the output does not support the given"]
            #[doc = "mode a failed event is sent."]
            async fn set_mode(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mode: Mode,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_power_v1#{}.set_mode()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(mode as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the output power management mode control object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_output_power_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Report the power management mode change of an output."]
            #[doc = ""]
            #[doc = "The mode event is sent after an output changed its power"]
            #[doc = "management mode. The reason can be a client using set_mode or the"]
            #[doc = "compositor deciding to change an output's mode."]
            #[doc = "This event is also sent immediately when the object is created"]
            #[doc = "so the client is informed about the current power management mode."]
            async fn mode(&self, mode: Mode) -> crate::client::Result<()>;
            #[doc = "This event indicates that the output power management mode control"]
            #[doc = "is no longer valid. This can happen for a number of reasons,"]
            #[doc = "including:"]
            #[doc = "- The output doesn't support power management"]
            #[doc = "- Another client already has exclusive power management mode control"]
            #[doc = "for this output"]
            #[doc = "- The output disappeared"]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            async fn failed(&self) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_screencopy_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrScreencopyManagerV1 {
            const INTERFACE: &'static str = "zwlr_screencopy_manager_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Capture the next frame of an entire output."]
            async fn capture_output(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_screencopy_manager_v1#{}.capture_output()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .put_int(overlay_cursor)
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Capture the next frame of an output's region."]
            #[doc = ""]
            #[doc = "The region is given in output logical coordinates, see"]
            #[doc = "xdg_output.logical_size. The region will be clipped to the output's"]
            #[doc = "extents."]
            async fn capture_output_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                output: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_screencopy_manager_v1#{}.capture_output_region()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .put_int(overlay_cursor)
                    .put_object(Some(output))
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_screencopy_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object represents a single frame."]
    #[doc = ""]
    #[doc = "When created, a series of buffer events will be sent, each representing a"]
    #[doc = "supported buffer type. The \"buffer_done\" event is sent afterwards to"]
    #[doc = "indicate that all supported buffer types have been enumerated. The client"]
    #[doc = "will then be able to send a \"copy\" request. If the capture is successful,"]
    #[doc = "the compositor will send a \"flags\" event followed by a \"ready\" event."]
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
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_screencopy_frame_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrScreencopyFrameV1 {
            const INTERFACE: &'static str = "zwlr_screencopy_frame_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Copy the frame to the supplied buffer. The buffer must have the"]
            #[doc = "correct size, see zwlr_screencopy_frame_v1.buffer and"]
            #[doc = "zwlr_screencopy_frame_v1.linux_dmabuf. The buffer needs to have a"]
            #[doc = "supported format."]
            #[doc = ""]
            #[doc = "If the frame is successfully copied, \"flags\" and \"ready\" events are"]
            #[doc = "sent. Otherwise, a \"failed\" event is sent."]
            async fn copy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_screencopy_frame_v1#{}.copy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the frame. This request can be sent at any time by the client."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_screencopy_frame_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Same as copy, except it waits until there is damage to copy."]
            async fn copy_with_damage(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_screencopy_frame_v1#{}.copy_with_damage()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Provides information about wl_shm buffer parameters that need to be"]
            #[doc = "used for this frame. This event is sent once after the frame is created"]
            #[doc = "if wl_shm buffers are supported."]
            async fn buffer(
                &self,
                format: super::super::super::core::wayland::wl_shm::Format,
                width: u32,
                height: u32,
                stride: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Provides flags about the frame. This event is sent once before the"]
            #[doc = "\"ready\" event."]
            async fn flags(&self, flags: Flags) -> crate::client::Result<()>;
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading. This event includes the time at which the presentation took place."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part"]
            #[doc = "may have an arbitrary offset at start."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            async fn ready(
                &self,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            async fn failed(&self) -> crate::client::Result<()>;
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
            async fn damage(
                &self,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Provides information about linux-dmabuf buffer parameters that need to"]
            #[doc = "be used for this frame. This event is sent once after the frame is"]
            #[doc = "created if linux-dmabuf buffers are supported."]
            async fn linux_dmabuf(
                &self,
                format: u32,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent once after all buffer events have been sent."]
            #[doc = ""]
            #[doc = "The client should proceed to create a buffer of one of the supported"]
            #[doc = "types, and send a \"copy\" request."]
            async fn buffer_done(&self) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod wlr_virtual_pointer_unstable_v1 {
    #[doc = "This protocol allows clients to emulate a physical pointer device. The"]
    #[doc = "requests are mostly mirror opposites of those specified in wl_pointer."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_virtual_pointer_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the zwlr_virtual_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrVirtualPointerV1 {
            const INTERFACE: &'static str = "zwlr_virtual_pointer_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "The pointer has moved by a relative amount to the previous request."]
            #[doc = ""]
            #[doc = "Values are in the global compositor space."]
            async fn motion(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                time: u32,
                dx: crate::wire::Fixed,
                dy: crate::wire::Fixed,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.motion()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_fixed(dx)
                    .put_fixed(dy)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The pointer has moved in an absolute coordinate frame."]
            #[doc = ""]
            #[doc = "Value of x can range from 0 to x_extent, value of y can range from 0"]
            #[doc = "to y_extent."]
            async fn motion_absolute(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                time: u32,
                x: u32,
                y: u32,
                x_extent: u32,
                y_extent: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.motion_absolute()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(x)
                    .put_uint(y)
                    .put_uint(x_extent)
                    .put_uint(y_extent)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "A button was pressed or released."]
            async fn button(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                time: u32,
                button: u32,
                state: super::super::super::core::wayland::wl_pointer::ButtonState,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.button()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(button)
                    .put_uint(state as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Scroll and other axis requests."]
            async fn axis(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                time: u32,
                axis: super::super::super::core::wayland::wl_pointer::Axis,
                value: crate::wire::Fixed,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.axis()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis as u32)
                    .put_fixed(value)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Indicates the set of events that logically belong together."]
            async fn frame(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.frame()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Source information for scroll and other axis."]
            async fn axis_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                axis_source: super::super::super::core::wayland::wl_pointer::AxisSource,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.axis_source()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(axis_source as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Stop notification for scroll and other axes."]
            async fn axis_stop(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                time: u32,
                axis: super::super::super::core::wayland::wl_pointer::Axis,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.axis_stop()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Discrete step information for scroll and other axes."]
            #[doc = ""]
            #[doc = "This event allows the client to extend data normally sent using the axis"]
            #[doc = "event with discrete value."]
            async fn axis_discrete(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                time: u32,
                axis: super::super::super::core::wayland::wl_pointer::Axis,
                value: crate::wire::Fixed,
                discrete: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.axis_discrete()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis as u32)
                    .put_fixed(value)
                    .put_int(discrete)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object allows clients to create individual virtual pointer objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwlr_virtual_pointer_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwlr_virtual_pointer_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrVirtualPointerManagerV1 {
            const INTERFACE: &'static str = "zwlr_virtual_pointer_manager_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Creates a new virtual pointer. The optional seat is a suggestion to the"]
            #[doc = "compositor."]
            async fn create_virtual_pointer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: Option<crate::wire::ObjectId>,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(seat)
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwlr_virtual_pointer_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Creates a new virtual pointer. The seat and the output arguments are"]
            #[doc = "optional. If the seat argument is set, the compositor should assign the"]
            #[doc = "input device to the requested seat. If the output argument is set, the"]
            #[doc = "compositor should map the input device to the requested output."]
            async fn create_virtual_pointer_with_output(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: Option<crate::wire::ObjectId>,
                output: Option<crate::wire::ObjectId>,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer_with_output()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(seat)
                    .put_object(output)
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
