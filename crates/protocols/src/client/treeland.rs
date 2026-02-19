#[allow(clippy::module_inception)]
pub mod treeland_app_id_resolver_v1 {
    #[doc = "Create a resolver object. Typically exactly one privileged helper"]
    #[doc = "(a Wayland client with DBus access) binds this interface and serves"]
    #[doc = "identification requests coming from the compositor."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_app_id_resolver_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "A resolver is already registered in this session with the manager"]
            ResolverAlreadyExists = 1u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::ResolverAlreadyExists),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_app_id_resolver_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandAppIdResolverManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_app_id_resolver_manager_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_app_id_resolver_manager_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create or bind a resolver object. Only one resolver may be registered"]
            #[doc = "per session. Treeland is a multi-user compositor; different user"]
            #[doc = "sessions may each register their own resolver. If a resolver is"]
            #[doc = "already bound in the same session, the compositor will report an"]
            #[doc = "error on the manager and will NOT create a new resolver object for"]
            #[doc = "this request."]
            fn get_resolver(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_app_id_resolver_manager_v1#{}.get_resolver({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The compositor sends identify_request with a unique request_id and a pidfd."]
    #[doc = "The client must answer exactly once via respond(request_id, app_id). If"]
    #[doc = "resolution fails respond with an empty string."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_app_id_resolver_v1 {
        #[doc = "Trait to implement the treeland_app_id_resolver_v1 interface. See the module level documentation for more info"]
        pub trait TreelandAppIdResolverV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_app_id_resolver_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Respond to an identify_request. The sandbox_engine_name must be provided and"]
            #[doc = "matches the context in which the process is running (container, sandbox, etc)."]
            #[doc = "If resolution fails, respond with an empty app_id."]
            fn respond(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                request_id: u32,
                app_id: String,
                sandbox_engine_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_app_id_resolver_v1#{}.respond({}, \"{}\", \"{}\")",
                        sender_id,
                        request_id,
                        app_id,
                        sandbox_engine_name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(request_id)
                        .put_string(Some(app_id))
                        .put_string(Some(sandbox_engine_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_app_id_resolver_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn identify_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                request_id: u32,
                pidfd: std::os::fd::OwnedFd,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let request_id = message.uint()?;
                            let pidfd = waynest::Connection::fd(connection)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_app_id_resolver_v1#{}.identify_request({}, {})",
                                sender_id,
                                request_id,
                                std::os::fd::AsRawFd::as_raw_fd(&pidfd)
                            );
                            self.identify_request(connection, sender_id, request_id, pidfd)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows authorized application to capture output contents or window"]
#[doc = "contents(useful for window streaming)."]
#[allow(clippy::module_inception)]
pub mod treeland_capture_unstable_v1 {
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_capture_session_v1 {
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
        impl From<CancelReason> for u32 {
            fn from(value: CancelReason) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for CancelReason {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Temporary),
                    1u32 => Ok(Self::Permanent),
                    2u32 => Ok(Self::Resizing),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for CancelReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "clients should copy frame before processing"] const Transient = 1u32 ; } }
        impl From<Flags> for u32 {
            fn from(value: Flags) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Flags {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_capture_session_v1 interface. See the module level documentation for more info"]
        pub trait TreelandCaptureSessionV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_capture_session_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Unreferences the frame. This request must be called as soon as it's no longer valid."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_session_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Start session and keeps sending frame."]
            fn start(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_session_v1#{}.start()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This is the ACK to the current \"ready\" event. The next \"frame\" event will be sent only when current"]
            #[doc = "\"ready\" event is acknowledged. The timestamp should be the same as the one sent in \"ready\" event."]
            #[doc = "If the frame has the \"transient\" flag, all objects sent before become invalid after this event."]
            fn frame_done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_usec: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_session_v1#{}.frame_done({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_usec
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_usec)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Main event supplying the client with information about the frame. If the capture didn't fail, this event is always"]
            #[doc = "emitted first before any other events."]
            #[doc = "When mask is provided, x and y should be offset relative to mask surface origin. Otherwise offset_x and offset_y should always"]
            #[doc = "be zero."]
            fn frame(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                offset_x: i32,
                offset_y: i32,
                width: u32,
                height: u32,
                buffer_flags: u32,
                flags : super :: super :: super :: treeland :: treeland_capture_unstable_v1 :: treeland_capture_session_v1 :: Flags,
                format: u32,
                mod_high: u32,
                mod_low: u32,
                num_objects: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn object(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: u32,
                fd: std::os::fd::OwnedFd,
                size: u32,
                offset: u32,
                stride: u32,
                plane_index: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent as soon as the frame is presented, indicating it is available for reading. This event"]
            #[doc = "includes the time at which presentation happened at."]
            fn ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "If the capture failed or if the frame is no longer valid after the \"frame\" event has been emitted, this"]
            #[doc = "event will be used to inform the client to scrap the frame."]
            fn cancel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_capture_unstable_v1 :: treeland_capture_session_v1 :: CancelReason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let offset_x = message.int()?;
                            let offset_y = message.int()?;
                            let width = message.uint()?;
                            let height = message.uint()?;
                            let buffer_flags = message.uint()?;
                            let flags = message.uint()?;
                            let format = message.uint()?;
                            let mod_high = message.uint()?;
                            let mod_low = message.uint()?;
                            let num_objects = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_session_v1#{}.frame({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                offset_x,
                                offset_y,
                                width,
                                height,
                                buffer_flags,
                                flags,
                                format,
                                mod_high,
                                mod_low,
                                num_objects
                            );
                            self.frame(
                                connection,
                                sender_id,
                                offset_x,
                                offset_y,
                                width,
                                height,
                                buffer_flags,
                                flags.try_into()?,
                                format,
                                mod_high,
                                mod_low,
                                num_objects,
                            )
                            .await
                        }
                        1u16 => {
                            let index = message.uint()?;
                            let fd = waynest::Connection::fd(connection)?;
                            let size = message.uint()?;
                            let offset = message.uint()?;
                            let stride = message.uint()?;
                            let plane_index = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_session_v1#{}.object({}, {}, {}, {}, {}, {})",
                                sender_id,
                                index,
                                std::os::fd::AsRawFd::as_raw_fd(&fd),
                                size,
                                offset,
                                stride,
                                plane_index
                            );
                            self.object(
                                connection,
                                sender_id,
                                index,
                                fd,
                                size,
                                offset,
                                stride,
                                plane_index,
                            )
                            .await
                        }
                        2u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_nsec = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_session_v1#{}.ready({}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_nsec
                            );
                            self.ready(connection, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec)
                                .await
                        }
                        3u16 => {
                            let reason = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_session_v1#{}.cancel({})",
                                sender_id,
                                reason
                            );
                            self.cancel(connection, sender_id, reason.try_into()?).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_capture_frame_v1 {
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInverted = 1u32 ; } }
        impl From<Flags> for u32 {
            fn from(value: Flags) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Flags {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_capture_frame_v1 interface. See the module level documentation for more info"]
        pub trait TreelandCaptureFrameV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_capture_frame_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the context. This request can be sent at any time by the client."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_frame_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Copy capture contents to provided buffer"]
            fn copy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_frame_v1#{}.copy({})",
                        sender_id,
                        buffer
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(buffer))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform client to prepare buffer."]
            fn buffer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                format: super::super::super::core::wayland::wl_shm::Format,
                width: u32,
                height: u32,
                stride: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Inform client that all buffer formats supported are emitted."]
            fn buffer_done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Provides flags about the frame. This event is sent once before the"]
            #[doc = "\"ready\" event."]
            fn flags(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags : super :: super :: super :: treeland :: treeland_capture_unstable_v1 :: treeland_capture_frame_v1 :: Flags,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Inform that buffer is ready for reading"]
            fn ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Inform that frame copy fails."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let format = message.uint()?;
                            let width = message.uint()?;
                            let height = message.uint()?;
                            let stride = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_frame_v1#{}.buffer({}, {}, {}, {})",
                                sender_id,
                                format,
                                width,
                                height,
                                stride
                            );
                            self.buffer(
                                connection,
                                sender_id,
                                format.try_into()?,
                                width,
                                height,
                                stride,
                            )
                            .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_frame_v1#{}.buffer_done()",
                                sender_id,
                            );
                            self.buffer_done(connection, sender_id).await
                        }
                        2u16 => {
                            let flags = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_frame_v1#{}.flags({})",
                                sender_id,
                                flags
                            );
                            self.flags(connection, sender_id, flags.try_into()?).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_capture_frame_v1#{}.ready()", sender_id,);
                            self.ready(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_capture_frame_v1#{}.failed()", sender_id,);
                            self.failed(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_capture_context_v1 {
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct SourceType : u32 { # [doc = "output source type"] const Output = 1u32 ; # [doc = "window source type"] const Window = 2u32 ; # [doc = "region source type"] const Region = 4u32 ; } }
        impl From<SourceType> for u32 {
            fn from(value: SourceType) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for SourceType {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for SourceType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum SourceFailure {
            #[doc = "selector is occupied by other context"]
            SelectorBusy = 1u32,
            #[doc = "User cancel this context from compositor"]
            UserCancel = 2u32,
            #[doc = "Source has been destroyed"]
            SourceDestroyed = 3u32,
            #[doc = "other failure"]
            Other = 4u32,
        }
        impl From<SourceFailure> for u32 {
            fn from(value: SourceFailure) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for SourceFailure {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::SelectorBusy),
                    2u32 => Ok(Self::UserCancel),
                    3u32 => Ok(Self::SourceDestroyed),
                    4u32 => Ok(Self::Other),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for SourceFailure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_capture_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandCaptureContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_capture_context_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the context. This request can be sent at any time by the client."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_context_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Selector is provided by compositor. Client can provide source hint to hint compositor"]
            #[doc = "to provide certain kinds of source."]
            fn select_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                source_hint: SourceType,
                freeze: u32,
                with_cursor: u32,
                mask: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_context_v1#{}.select_source({}, {}, {}, {})",
                        sender_id,
                        source_hint,
                        freeze,
                        with_cursor,
                        mask.as_ref().map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(source_hint.into())
                        .put_uint(freeze)
                        .put_uint(with_cursor)
                        .put_object(mask)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event can be called just once. A second call might result in a protocol error cause"]
            #[doc = "we just provide transient"]
            fn capture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                frame: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_context_v1#{}.capture({})",
                        sender_id,
                        frame
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(frame))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Often used by a screen recorder."]
            fn create_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                session: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_context_v1#{}.create_session({})",
                        sender_id,
                        session
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(session))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event supplies the client some information about the capture source, including"]
            #[doc = "the capture region relative to mask and source type."]
            fn source_ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                region_x: i32,
                region_y: i32,
                region_width: u32,
                region_height: u32,
                source_type: SourceType,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "There could a lot of reasons but the most common one is that selector is busy"]
            fn source_failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason: SourceFailure,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let region_x = message.int()?;
                            let region_y = message.int()?;
                            let region_width = message.uint()?;
                            let region_height = message.uint()?;
                            let source_type = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_context_v1#{}.source_ready({}, {}, {}, {}, {})",
                                sender_id,
                                region_x,
                                region_y,
                                region_width,
                                region_height,
                                source_type
                            );
                            self.source_ready(
                                connection,
                                sender_id,
                                region_x,
                                region_y,
                                region_width,
                                region_height,
                                source_type.try_into()?,
                            )
                            .await
                        }
                        1u16 => {
                            let reason = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_context_v1#{}.source_failed({})",
                                sender_id,
                                reason
                            );
                            self.source_failed(connection, sender_id, reason.try_into()?)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_capture_manager_v1 {
        #[doc = "Trait to implement the treeland_capture_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandCaptureManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_capture_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the treeland_capture_manager_v1 object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_manager_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn get_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                context: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_manager_v1#{}.get_context({})",
                        sender_id,
                        context
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(context))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_dde_shell_v1 {
    #[doc = "This interface allows DDE change some treeland function."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dde_shell_manager_v1 {
        #[doc = "Trait to implement the treeland_dde_shell_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdeShellManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_dde_shell_manager_v1";
            const VERSION: u32 = 1u32;
            fn get_window_overlap_checker(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.get_window_overlap_checker({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a shell surface for an existing wl_surface."]
            #[doc = ""]
            #[doc = "Only one shell surface can be associated with a given surface."]
            #[doc = ""]
            #[doc = "Recommended for use with xdg_surface."]
            fn get_shell_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.get_shell_surface({}, {})",
                        sender_id,
                        id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a new dde active for a given seat."]
            fn get_treeland_dde_active(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.get_treeland_dde_active({}, {})",
                        sender_id,
                        id,
                        seat
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(seat))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a new multitaskview context for toggle."]
            fn get_treeland_multitaskview(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.get_treeland_multitaskview({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a new window picker to pick window."]
            fn get_treeland_window_picker(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.get_treeland_window_picker({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a new lockscreen context for toggle."]
            fn get_treeland_lockscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.get_treeland_lockscreen({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Move the top-left point of a XWayland surface"]
            #[doc = "specified with Window ID, to a position that has"]
            #[doc = "(dx, dy) offset from the top-left point of a wl_surface"]
            #[doc = ""]
            #[doc = "Once the server finished process it send \"done\" to the"]
            #[doc = "callback. The callback data 0 means no error occured,"]
            #[doc = "1 means something went wrong and the move cannot be"]
            #[doc = "done."]
            fn set_xwindow_position_relative(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                callback: waynest::ObjectId,
                wid: u32,
                anchor: waynest::ObjectId,
                dx: waynest::Fixed,
                dy: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_manager_v1#{}.set_xwindow_position_relative({}, {}, {}, {}, {})",
                        sender_id,
                        callback,
                        wid,
                        anchor,
                        dx,
                        dy
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(callback))
                        .put_uint(wid)
                        .put_object(Some(anchor))
                        .put_fixed(dx)
                        .put_fixed(dy)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A treeland_dde_shell_handle_v1 object represents an opened toplevel window. Each"]
    #[doc = "app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_window_overlap_checker {
        bitflags::bitflags! { # [doc = "same layershell"] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Anchor : u32 { # [doc = "the top edge of the anchor rectangle"] const Top = 1u32 ; # [doc = "the bottom edge of the anchor rectangle"] const Bottom = 2u32 ; # [doc = "the left edge of the anchor rectangle"] const Left = 4u32 ; # [doc = "the right edge of the anchor rectangle"] const Right = 8u32 ; } }
        impl From<Anchor> for u32 {
            fn from(value: Anchor) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Anchor {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Anchor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_window_overlap_checker interface. See the module level documentation for more info"]
        pub trait TreelandWindowOverlapChecker
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_window_overlap_checker";
            const VERSION: u32 = 1u32;
            #[doc = "This interface is used to receive the detected surface."]
            #[doc = "When the xdgshell window in the workspace overlaps with the detected window,"]
            #[doc = "an event will be sent to notify the client to process it."]
            #[doc = "The window position will only be recorded when this interface is called."]
            #[doc = "If the window moves, this interface needs to be called again."]
            fn update(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
                anchor: Anchor,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_window_overlap_checker#{}.update({}, {}, {}, {})",
                        sender_id,
                        width,
                        height,
                        anchor,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .put_uint(anchor.into())
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroys the treeland_window_overlap_checker object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_window_overlap_checker#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent when windows overlapped."]
            #[doc = "This event is sent only once."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent when windows not overlapped."]
            #[doc = "This event is sent only once."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_overlap_checker#{}.enter()",
                                sender_id,
                            );
                            self.enter(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_overlap_checker#{}.leave()",
                                sender_id,
                            );
                            self.leave(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide the shell user interface."]
    #[doc = ""]
    #[doc = "It provides requests to set surface role, set skip, set the position"]
    #[doc = "set auto placement in output coordinates."]
    #[doc = ""]
    #[doc = "On the server side the object is automatically destroyed when"]
    #[doc = "the related wl_surface is destroyed.  On client side,"]
    #[doc = "treeland_dde_shell_surface_v1.destroy() must be called before"]
    #[doc = "destroying the wl_surface object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dde_shell_surface_v1 {
        #[doc = "These values indicate which roles a surface can be rendered in, They"]
        #[doc = "are ordered by z depth."]
        #[doc = ""]
        #[doc = "Displayed below wlr-layer-shell, at the overlay level of the workspace."]
        #[doc = ""]
        #[doc = "Multiple surfaces can share a single role, and ordering within a single"]
        #[doc = "role is undefined."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Role {
            Overlay = 1u32,
        }
        impl From<Role> for u32 {
            fn from(value: Role) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Role {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Overlay),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Role {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_dde_shell_surface_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdeShellSurfaceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_dde_shell_surface_v1";
            const VERSION: u32 = 1u32;
            #[doc = "The treeland_dde_shell_surface_v1 interface is removed from the"]
            #[doc = "wl_surface object that was turned into a shell surface with the"]
            #[doc = "treeland_shell_v1.get_treeland_dde_shell_surface request."]
            #[doc = ""]
            #[doc = "The shell surface role is lost and wl_surface is unmapped."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dde_shell_surface_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Move the surface to new coordinates."]
            #[doc = ""]
            #[doc = "Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output"]
            #[doc = "is 1970,50 in global coordinates space."]
            fn set_surface_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_surface_position({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_int(x).put_int(y).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Assign a role to a shell surface."]
            fn set_role(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                role : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_shell_surface_v1 :: Role,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_role({})",
                        sender_id,
                        role
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(role.into()).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the vertical alignment of the surface within the cursor width."]
            #[doc = ""]
            #[doc = "Do not use it together with set_surface_position to avoid exceptions."]
            #[doc = ""]
            #[doc = "The position of the surface will be controlled by the compositor after the"]
            #[doc = "request, including preventing it from being displayed beyond the edge of"]
            #[doc = "the output."]
            fn set_auto_placement(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                y_offset: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_auto_placement({})",
                        sender_id,
                        y_offset
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(y_offset).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a switcher."]
            fn set_skip_switcher(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_skip_switcher({})",
                        sender_id,
                        skip
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(skip).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a dock preview."]
            fn set_skip_dock_preview(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_skip_dock_preview({})",
                        sender_id,
                        skip
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(skip).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a mutitask view."]
            fn set_skip_muti_task_view(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_skip_muti_task_view({})",
                        sender_id,
                        skip
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(skip).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Setting this will determine whether the surface can receive keyboard focus."]
            #[doc = "When set to 0, the surface will not receive keyboard focus even when clicked or activated."]
            #[doc = "When set to 1 (default), the surface will receive keyboard focus normally."]
            fn set_accept_keyboard_focus(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                accept: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_shell_surface_v1#{}.set_accept_keyboard_focus({})",
                        sender_id,
                        accept
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(accept).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An interface used to monitor special events."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dde_active_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Reason {
            Mouse = 0u32,
            Wheel = 1u32,
        }
        impl From<Reason> for u32 {
            fn from(value: Reason) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Reason {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Mouse),
                    1u32 => Ok(Self::Wheel),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Reason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_dde_active_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdeActiveV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_dde_active_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dde_active_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn active_in(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_active_v1 :: Reason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn active_out(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_active_v1 :: Reason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn start_drag(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn drop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let reason = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_active_v1#{}.active_in({})",
                                sender_id,
                                reason
                            );
                            self.active_in(connection, sender_id, reason.try_into()?)
                                .await
                        }
                        1u16 => {
                            let reason = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_active_v1#{}.active_out({})",
                                sender_id,
                                reason
                            );
                            self.active_out(connection, sender_id, reason.try_into()?)
                                .await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_dde_active_v1#{}.start_drag()", sender_id,);
                            self.start_drag(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_dde_active_v1#{}.drop()", sender_id,);
                            self.drop(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An interface used to control multitaskview."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_multitaskview_v1 {
        #[doc = "Trait to implement the treeland_multitaskview_v1 interface. See the module level documentation for more info"]
        pub trait TreelandMultitaskviewV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_multitaskview_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_multitaskview_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Show or hide the multitaskview."]
            fn toggle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_multitaskview_v1#{}.toggle()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An interface used to pick window and return credentials."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_window_picker_v1 {
        #[doc = "Trait to implement the treeland_window_picker_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWindowPickerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_window_picker_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_window_picker_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Pick a window to get information."]
            fn pick(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_window_picker_v1#{}.pick(\"{}\")",
                        sender_id,
                        hint
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(hint))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Picked window information."]
            fn window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                pid: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let pid = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_picker_v1#{}.window({})",
                                sender_id,
                                pid
                            );
                            self.window(connection, sender_id, pid).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An interface used to operate lockscreen."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_lockscreen_v1 {
        #[doc = "Trait to implement the treeland_lockscreen_v1 interface. See the module level documentation for more info"]
        pub trait TreelandLockscreenV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_lockscreen_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_lockscreen_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Lock the screen."]
            fn lock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_lockscreen_v1#{}.lock()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Show shutdown."]
            fn shutdown(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_lockscreen_v1#{}.shutdown()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Switch user."]
            fn switch_user(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_lockscreen_v1#{}.switch_user()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_ddm_v1 {
    #[doc = "This is the treeland - ddm private communication protocol."]
    #[doc = ""]
    #[doc = "This object is primarily used for establish connection between"]
    #[doc = "treeland and ddm."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_ddm_v1 {
        #[doc = "Trait to implement the treeland_ddm_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdmV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_ddm_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Send treeland to Greeter mode."]
            fn switch_to_greeter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_ddm_v1#{}.switch_to_greeter()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set lockscreen user to username. Ignore when username is \"ddm\"."]
            fn switch_to_user(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                username: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_ddm_v1#{}.switch_to_user(\"{}\")",
                        sender_id,
                        username
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(username))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Activate treeland session. This will makes treeland try to take"]
            #[doc = "control of screen."]
            fn activate_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_ddm_v1#{}.activate_session()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Deactivate treeland session. This will release control of the"]
            #[doc = "screen, but not to close the current seats."]
            fn deactivate_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_ddm_v1#{}.deactivate_session()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Enable treeland rendering. This is primarily called after"]
            #[doc = "disable_render to resume treeland."]
            fn enable_render(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_ddm_v1#{}.enable_render()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Disable treeland rendering. This will prevent treeland from"]
            #[doc = "output to DRM device."]
            fn disable_render(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                callback: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_ddm_v1#{}.disable_render({})",
                        sender_id,
                        callback
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(callback))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Call ddm to switch current virtual terminal to vtnr. ddm should"]
            #[doc = "take care of the switch and call ioctl respectively."]
            fn switch_to_vt(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                vtnr: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Call ddm to acquire control of VT at vtnr. ddm should call"]
            #[doc = "VT_SETMODE respectively."]
            fn acquire_vt(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                vtnr: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let vtnr = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm_v1#{}.switch_to_vt({})", sender_id, vtnr);
                            self.switch_to_vt(connection, sender_id, vtnr).await
                        }
                        1u16 => {
                            let vtnr = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm_v1#{}.acquire_vt({})", sender_id, vtnr);
                            self.acquire_vt(connection, sender_id, vtnr).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_foreign_toplevel_manager_v1 {
    #[doc = "This interface allows a client to get toplevel some info."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_foreign_toplevel_manager_v1 {
        #[doc = "Trait to implement the treeland_foreign_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandForeignToplevelManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_foreign_toplevel_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Indicates the client no longer wishes to receive events for new toplevels."]
            #[doc = "However the compositor may emit further toplevel_created events, until"]
            #[doc = "the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            fn stop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_manager_v1#{}.stop()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn get_dock_preview_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                relative_surface: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_manager_v1#{}.get_dock_preview_context({}, {})",
                        sender_id,
                        relative_surface,
                        id
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(relative_surface))
                        .put_object(Some(id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted whenever a new toplevel window is created. It"]
            #[doc = "is emitted for all toplevels, regardless of the app that has created"]
            #[doc = "them."]
            #[doc = ""]
            #[doc = "All initial details of the toplevel(title, app_id, states, etc.) will"]
            #[doc = "be sent immediately after this event via the corresponding events in"]
            #[doc = "treeland_foreign_toplevel_handle_v1."]
            fn toplevel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the compositor is done sending events to the"]
            #[doc = "treeland_foreign_toplevel_manager_v1. The server will destroy the object"]
            #[doc = "immediately after sending this request, so it will become invalid and"]
            #[doc = "the client should free any resources associated with it."]
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_manager_v1#{}.toplevel({})",
                                sender_id,
                                toplevel
                            );
                            self.toplevel(connection, sender_id, toplevel).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_manager_v1#{}.finished()",
                                sender_id,
                            );
                            self.finished(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A treeland_foreign_toplevel_handle_v1 object represents an opened toplevel window. Each"]
    #[doc = "app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_foreign_toplevel_handle_v1 {
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
        impl From<State> for u32 {
            fn from(value: State) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for State {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Maximized),
                    1u32 => Ok(Self::Minimized),
                    2u32 => Ok(Self::Activated),
                    3u32 => Ok(Self::Fullscreen),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidRectangle),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait TreelandForeignToplevelHandleV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_foreign_toplevel_handle_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Requests that the toplevel be maximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn set_maximized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.set_maximized()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests that the toplevel be unmaximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn unset_maximized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.unset_maximized()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests that the toplevel be minimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn set_minimized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.set_minimized()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests that the toplevel be unminimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn unset_minimized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.unset_minimized()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that this toplevel be activated on the given seat."]
            #[doc = "There is no guarantee the toplevel will be actually activated."]
            fn activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.activate({})",
                        sender_id,
                        seat
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(seat))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send a request to the toplevel to close itself. The compositor would"]
            #[doc = "typically use a shell-specific method to carry out this request, for"]
            #[doc = "example by sending the xdg_toplevel.close event. However, this gives"]
            #[doc = "no guarantees the toplevel will actually be destroyed. If and when"]
            #[doc = "this happens, the treeland_foreign_toplevel_handle_v1.closed event will"]
            #[doc = "be emitted."]
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.close()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
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
            fn set_rectangle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.set_rectangle({}, {}, {}, {}, {})",
                        sender_id,
                        surface,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroys the treeland_foreign_toplevel_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.set_fullscreen({})",
                        sender_id,
                        output
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_object(output).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests that the toplevel be unfullscreened. If the fullscreen state"]
            #[doc = "actually changes, this will be indicated by the state event."]
            fn unset_fullscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.unset_fullscreen()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the compositor has set the process id this window"]
            #[doc = "belongs to. This should be set once before the initial_state is sent."]
            fn pid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                pid: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            fn title(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                title: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted whenever the app-id of the toplevel changes."]
            fn app_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                app_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The identifier of each top level and its handle must be unique."]
            #[doc = "Two different top layers cannot have the same identifier."]
            #[doc = "This identifier is only valid as long as the top level is mapped."]
            #[doc = "Identifiers must not be reused if the top level is not mapped."]
            #[doc = "The compositor must not reuse identifiers to ensure there are no races when"]
            #[doc = "identifiers are shared between processes."]
            fn identifier(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                identifier: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted whenever the toplevel becomes visible on"]
            #[doc = "the given output. A toplevel may be visible on multiple outputs."]
            fn output_enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted whenever the toplevel stops being visible on"]
            #[doc = "the given output. It is guaranteed that an entered-output event"]
            #[doc = "with the same output has been emitted before this event."]
            fn output_leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted immediately after the treeland_foreign_toplevel_handle_v1"]
            #[doc = "is created and each time the toplevel state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            fn state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent after all changes in the toplevel state have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "This allows changes to the treeland_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event means the toplevel has been destroyed. It is guaranteed there"]
            #[doc = "won't be any more events for this treeland_foreign_toplevel_handle_v1. The"]
            #[doc = "toplevel itself becomes inert so any requests will be ignored except the"]
            #[doc = "destroy request."]
            fn closed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted whenever the parent of the toplevel changes."]
            #[doc = ""]
            #[doc = "No event is emitted when the parent handle is destroyed by the client."]
            fn parent(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                parent: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let pid = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.pid({})",
                                sender_id,
                                pid
                            );
                            self.pid(connection, sender_id, pid).await
                        }
                        1u16 => {
                            let title = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.title(\"{}\")",
                                sender_id,
                                title
                            );
                            self.title(connection, sender_id, title).await
                        }
                        2u16 => {
                            let app_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.app_id(\"{}\")",
                                sender_id,
                                app_id
                            );
                            self.app_id(connection, sender_id, app_id).await
                        }
                        3u16 => {
                            let identifier = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.identifier({})",
                                sender_id,
                                identifier
                            );
                            self.identifier(connection, sender_id, identifier).await
                        }
                        4u16 => {
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.output_enter({})",
                                sender_id,
                                output
                            );
                            self.output_enter(connection, sender_id, output).await
                        }
                        5u16 => {
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.output_leave({})",
                                sender_id,
                                output
                            );
                            self.output_leave(connection, sender_id, output).await
                        }
                        6u16 => {
                            let state = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.state(array[{}])",
                                sender_id,
                                state.len()
                            );
                            self.state(connection, sender_id, state).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.done()",
                                sender_id,
                            );
                            self.done(connection, sender_id).await
                        }
                        8u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.closed()",
                                sender_id,
                            );
                            self.closed(connection, sender_id).await
                        }
                        9u16 => {
                            let parent = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.parent({})",
                                sender_id,
                                parent
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.parent(connection, sender_id, parent).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows dock set windows preview."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dock_preview_context_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Direction {
            #[doc = "top"]
            Top = 0u32,
            #[doc = "right"]
            Right = 1u32,
            #[doc = "bottom"]
            Bottom = 2u32,
            #[doc = "left"]
            Left = 3u32,
        }
        impl From<Direction> for u32 {
            fn from(value: Direction) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Direction {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Top),
                    1u32 => Ok(Self::Right),
                    2u32 => Ok(Self::Bottom),
                    3u32 => Ok(Self::Left),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Direction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_dock_preview_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDockPreviewContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_dock_preview_context_v1";
            const VERSION: u32 = 1u32;
            #[doc = "X and Y are relative to the relative_surface."]
            #[doc = "surfaces wl_array is identifiers."]
            fn show(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surfaces: Vec<u8>,
                x: i32,
                y: i32,
                direction: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dock_preview_context_v1#{}.show(array[{}], {}, {}, {})",
                        sender_id,
                        surfaces.len(),
                        x,
                        y,
                        direction
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_array(surfaces)
                        .put_int(x)
                        .put_int(y)
                        .put_uint(direction)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn show_tooltip(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tooltip: String,
                x: i32,
                y: i32,
                direction: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dock_preview_context_v1#{}.show_tooltip(\"{}\", {}, {}, {})",
                        sender_id,
                        tooltip,
                        x,
                        y,
                        direction
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(tooltip))
                        .put_int(x)
                        .put_int(y)
                        .put_uint(direction)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "close preview box"]
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dock_preview_context_v1#{}.close()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dock_preview_context_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after mouse enter preview box."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent after mouse leave preview box."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.enter()",
                                sender_id,
                            );
                            self.enter(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.leave()",
                                sender_id,
                            );
                            self.leave(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_output_manager_v1 {
    #[doc = "Protocol for telling which is the primary display among the selection of enabled"]
    #[doc = "outputs."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_output_manager_v1 {
        #[doc = "Trait to implement the treeland_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandOutputManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_output_manager_v1";
            const VERSION: u32 = 2u32;
            fn set_primary_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_manager_v1#{}.set_primary_output(\"{}\")",
                        sender_id,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn get_color_control(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_manager_v1#{}.get_color_control({}, {})",
                        sender_id,
                        id,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_output_manager_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Specifies which output is the primary one identified by their name."]
            fn primary_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let output_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_manager_v1#{}.primary_output(\"{}\")",
                                sender_id,
                                output_name
                            );
                            self.primary_output(connection, sender_id, output_name)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Protocol for controlling color temperature and brightness settings of a specific output."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_output_color_control_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Invalid color temperature value provided."]
            InvalidColorTemperature = 1u32,
            #[doc = "Invalid brightness value provided."]
            InvalidBrightness = 2u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidColorTemperature),
                    2u32 => Ok(Self::InvalidBrightness),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_output_color_control_v1 interface. See the module level documentation for more info"]
        pub trait TreelandOutputColorControlV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_output_color_control_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Color temperature settings are applied only after a commit request is made."]
            #[doc = "Setting a value outside the range [1000, 20000] is a protocol error."]
            fn set_color_temperature(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                temperature: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_color_control_v1#{}.set_color_temperature({})",
                        sender_id,
                        temperature
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(temperature).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Brightness settings are applied only after a commit request is made."]
            #[doc = "Setting a value outside the range [0.0, 100.0] is a protocol error."]
            fn set_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                brightness: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_color_control_v1#{}.set_brightness({})",
                        sender_id,
                        brightness
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_fixed(brightness).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_output_color_control_v1#{}.commit()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_color_control_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn result(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                success: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Provides the current color temperature setting of the output."]
            #[doc = "Color temperature is valued in the range [1000, 20000]."]
            #[doc = "Color temperature is defined as the corresponding temperature (in Kelvin) of the current white point"]
            #[doc = "of the display on a Planckian locus."]
            #[doc = "With the current implementation, the neutral temperature is 6600K."]
            #[doc = "This event is sent once after the treeland_output_color_control_v1 object is created,"]
            #[doc = "or right after when a color temperature change for the output is successfully commited."]
            fn color_temperature(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                temperature: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Provides the current brightness setting of the output."]
            #[doc = "Brightness is valued in the range [0.0, 100.0]."]
            #[doc = "This event is sent once after the treeland_output_color_control_v1 object is created,"]
            #[doc = "or right after when a brightness change for the output is successfully commited."]
            fn brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                brightness: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let success = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.result({})",
                                sender_id,
                                success
                            );
                            self.result(connection, sender_id, success).await
                        }
                        1u16 => {
                            let temperature = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.color_temperature({})",
                                sender_id,
                                temperature
                            );
                            self.color_temperature(connection, sender_id, temperature)
                                .await
                        }
                        2u16 => {
                            let brightness = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.brightness({})",
                                sender_id,
                                brightness
                            );
                            self.brightness(connection, sender_id, brightness).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_personalization_manager_v1 {
    #[doc = "This interface allows a client to customized display effects."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_personalization_manager_v1 {
        #[doc = "Trait to implement the treeland_personalization_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPersonalizationManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_personalization_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "set window background, shadow based on context"]
            fn get_window_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_manager_v1#{}.get_window_context({}, {})",
                        sender_id,
                        id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "custom user wallpaper"]
            fn get_wallpaper_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_manager_v1#{}.get_wallpaper_context({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "custom user cursor"]
            fn get_cursor_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_manager_v1#{}.get_cursor_context({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "custom treeland and window global font context"]
            fn get_font_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_manager_v1#{}.get_font_context({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "custom user treeland window appearance global"]
            fn get_appearance_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_manager_v1#{}.get_appearance_context({})",
                        sender_id,
                        id
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows a client personalization wallpaper."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_personalization_wallpaper_context_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Options {
            #[doc = "whether to show a preview of the picture"]
            Preview = 1u32,
            #[doc = "configure screen background"]
            Background = 2u32,
            #[doc = "configure screen wallpaper"]
            Lockscreen = 4u32,
        }
        impl From<Options> for u32 {
            fn from(value: Options) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Options {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Preview),
                    2u32 => Ok(Self::Background),
                    4u32 => Ok(Self::Lockscreen),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Options {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_personalization_wallpaper_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPersonalizationWallpaperContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_personalization_wallpaper_context_v1";
            const VERSION: u32 = 1u32;
            fn set_fd(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                fd: std::os::fd::BorrowedFd,
                metadata: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.set_fd({}, \"{}\")",
                        sender_id,
                        std::os::fd::AsRawFd::as_raw_fd(&fd),
                        metadata
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_fd(fd)
                        .put_string(Some(metadata))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_identifier(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                identifier: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.set_identifier(\"{}\")",
                        sender_id,
                        identifier
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(identifier))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.set_output(\"{}\")",
                        sender_id,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_on(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                options: Options,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.set_on({})",
                        sender_id,
                        options
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(options.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_isdark(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                isdark: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.set_isdark({})",
                        sender_id,
                        isdark
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(isdark).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.commit()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "get the current user's wallpaper"]
            fn get_metadata(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.get_metadata()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after getting the user's wallpaper."]
            fn metadata(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                metadata: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let metadata = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.metadata(\"{}\")",
                                sender_id,
                                metadata
                            );
                            self.metadata(connection, sender_id, metadata).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows a client personalization cursor."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_personalization_cursor_context_v1 {
        #[doc = "Trait to implement the treeland_personalization_cursor_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPersonalizationCursorContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_personalization_cursor_context_v1";
            const VERSION: u32 = 1u32;
            fn set_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.set_theme(\"{}\")",
                        sender_id,
                        name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn get_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.get_theme()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.set_size({})",
                        sender_id,
                        size
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(size).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn get_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.get_size()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "if only one commit fails validation, the commit will fail"]
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.commit()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after commit cursor configure."]
            fn verfity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                success: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after system cursor theme changed."]
            fn theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after system cursor size changed."]
            fn size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let success = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.verfity({})",
                                sender_id,
                                success
                            );
                            self.verfity(connection, sender_id, success).await
                        }
                        1u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.theme(\"{}\")",
                                sender_id,
                                name
                            );
                            self.theme(connection, sender_id, name).await
                        }
                        2u16 => {
                            let size = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.size({})",
                                sender_id,
                                size
                            );
                            self.size(connection, sender_id, size).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows a client personalization window."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_personalization_window_context_v1 {
        #[doc = "Window blend mode defines how compositor composite window's surface over other"]
        #[doc = "surfaces."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum BlendMode {
            #[doc = "Normal blend mode, just composite over background with alpha channel"]
            Transparent = 0u32,
            #[doc = "Composite window over wallpaper"]
            Wallpaper = 1u32,
            #[doc = "Blur the content of the window background"]
            Blur = 2u32,
        }
        impl From<BlendMode> for u32 {
            fn from(value: BlendMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for BlendMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Transparent),
                    1u32 => Ok(Self::Wallpaper),
                    2u32 => Ok(Self::Blur),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for BlendMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Set window enable mode"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum EnableMode {
            #[doc = "force enable titlebar"]
            Enable = 0u32,
            #[doc = "force disable titlebar"]
            Disable = 1u32,
        }
        impl From<EnableMode> for u32 {
            fn from(value: EnableMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for EnableMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Enable),
                    1u32 => Ok(Self::Disable),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for EnableMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_personalization_window_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPersonalizationWindowContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_personalization_window_context_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Set window background blend mode"]
            fn set_blend_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: BlendMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_window_context_v1#{}.set_blend_mode({})",
                        sender_id,
                        mode
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(mode.into()).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request will set window round corner radius, invoking this request means user"]
            #[doc = "want to"]
            #[doc = "manage window round corner radius by itself. If not invoked, window round corner"]
            #[doc = "radius will"]
            #[doc = "be decided by compositor."]
            fn set_round_corner_radius(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                radius: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_window_context_v1#{}.set_round_corner_radius({})",
                        sender_id,
                        radius
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(radius).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set window shadow's radius, offset and color, invoking this request indicates that"]
            #[doc = "client want to manage"]
            #[doc = "the window shadow by itself. If not invoked, window shadow will be decided by the"]
            #[doc = "compositor"]
            fn set_shadow(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                radius: i32,
                offset_x: i32,
                offset_y: i32,
                r: i32,
                g: i32,
                b: i32,
                a: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_window_context_v1#{}.set_shadow({}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        radius,
                        offset_x,
                        offset_y,
                        r,
                        g,
                        b,
                        a
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(radius)
                        .put_int(offset_x)
                        .put_int(offset_y)
                        .put_int(r)
                        .put_int(g)
                        .put_int(b)
                        .put_int(a)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set window border width and color"]
            fn set_border(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                r: i32,
                g: i32,
                b: i32,
                a: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_window_context_v1#{}.set_border({}, {}, {}, {}, {})",
                        sender_id,
                        width,
                        r,
                        g,
                        b,
                        a
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(r)
                        .put_int(g)
                        .put_int(b)
                        .put_int(a)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set if system titlebar is enabled"]
            fn set_titlebar(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: EnableMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_window_context_v1#{}.set_titlebar({})",
                        sender_id,
                        mode
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(mode.into()).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_window_context_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows set treeland window global font settings."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_personalization_font_context_v1 {
        #[doc = "Trait to implement the treeland_personalization_font_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPersonalizationFontContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_personalization_font_context_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Set the system font size"]
            fn set_font_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.set_font_size({})",
                        sender_id,
                        size
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(size).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the system font size"]
            fn get_font_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.get_font_size()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the system font"]
            fn set_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.set_font(\"{}\")",
                        sender_id,
                        font_name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(font_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the system font"]
            fn get_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.get_font()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the system monospace font"]
            fn set_monospace_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.set_monospace_font(\"{}\")",
                        sender_id,
                        font_name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(font_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the system monospace font"]
            fn get_monospace_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.get_monospace_font()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system font."]
            fn font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system monospace font."]
            fn monospace_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system font size."]
            fn font_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let font_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.font(\"{}\")",
                                sender_id,
                                font_name
                            );
                            self.font(connection, sender_id, font_name).await
                        }
                        1u16 => {
                            let font_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.monospace_font(\"{}\")",
                                sender_id,
                                font_name
                            );
                            self.monospace_font(connection, sender_id, font_name).await
                        }
                        2u16 => {
                            let font_size = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.font_size({})",
                                sender_id,
                                font_size
                            );
                            self.font_size(connection, sender_id, font_size).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows set treeland window global appearance settings."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_personalization_appearance_context_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ThemeType {
            #[doc = "window auto theme"]
            Auto = 1u32,
            #[doc = "window light theme"]
            Light = 2u32,
            #[doc = "window dark theme"]
            Dark = 4u32,
        }
        impl From<ThemeType> for u32 {
            fn from(value: ThemeType) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ThemeType {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Auto),
                    2u32 => Ok(Self::Light),
                    4u32 => Ok(Self::Dark),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ThemeType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Wrong round corner radius"]
            InvalidRoundCornerRadius = 0u32,
            #[doc = "Wrong icon theme"]
            InvalidIconTheme = 1u32,
            #[doc = "Wrong active color"]
            InvalidActiveColor = 2u32,
            #[doc = "Wrong window opacity"]
            InvalidWindowOpacity = 4u32,
            #[doc = "Wrong theme type"]
            InvalidWindowThemeType = 8u32,
            #[doc = "Wrong window titlebar height"]
            InvalidWindowTitlebarHeight = 16u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidRoundCornerRadius),
                    1u32 => Ok(Self::InvalidIconTheme),
                    2u32 => Ok(Self::InvalidActiveColor),
                    4u32 => Ok(Self::InvalidWindowOpacity),
                    8u32 => Ok(Self::InvalidWindowThemeType),
                    16u32 => Ok(Self::InvalidWindowTitlebarHeight),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_personalization_appearance_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPersonalizationAppearanceContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_personalization_appearance_context_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Set window round corner radius"]
            fn set_round_corner_radius(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                radius: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.set_round_corner_radius({})",
                        sender_id,
                        radius
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(radius).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get window round corner radius"]
            fn get_round_corner_radius(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.get_round_corner_radius()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the system icon theme"]
            fn set_icon_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                theme: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.set_icon_theme(\"{}\")",
                        sender_id,
                        theme
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(theme))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the system icon theme"]
            fn get_icon_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.get_icon_theme()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the system active color"]
            fn set_active_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                color: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.set_active_color(\"{}\")",
                        sender_id,
                        color
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(color))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the system active color"]
            fn get_active_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.get_active_color()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the window window opacity"]
            fn set_window_opacity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                opacity: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.set_window_opacity({})",
                        sender_id,
                        opacity
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(opacity).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the window window opacity"]
            fn get_window_opacity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.get_window_opacity()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the window theme."]
            fn set_window_theme_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: ThemeType,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.set_window_theme_type({})",
                        sender_id,
                        r#type
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(r#type.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the window theme type"]
            fn get_window_theme_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.get_window_theme_type()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the window titlebar height"]
            fn set_window_titlebar_height(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.set_window_titlebar_height({})",
                        sender_id,
                        height
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(height).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the window titlebar height"]
            fn get_window_titlebar_height(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.get_window_titlebar_height()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the round corner radius."]
            fn round_corner_radius(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                radius: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system icon theme."]
            fn icon_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                theme_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system active color"]
            fn active_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                active_color: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system active color"]
            fn window_opacity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                opacity: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system theme"]
            fn window_theme_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: ThemeType,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the window titlebar height"]
            fn window_titlebar_height(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let radius = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.round_corner_radius({})",
                                sender_id,
                                radius
                            );
                            self.round_corner_radius(connection, sender_id, radius)
                                .await
                        }
                        1u16 => {
                            let theme_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.icon_theme(\"{}\")",
                                sender_id,
                                theme_name
                            );
                            self.icon_theme(connection, sender_id, theme_name).await
                        }
                        2u16 => {
                            let active_color = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.active_color(\"{}\")",
                                sender_id,
                                active_color
                            );
                            self.active_color(connection, sender_id, active_color).await
                        }
                        3u16 => {
                            let opacity = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.window_opacity({})",
                                sender_id,
                                opacity
                            );
                            self.window_opacity(connection, sender_id, opacity).await
                        }
                        4u16 => {
                            let r#type = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.window_theme_type({})",
                                sender_id,
                                r#type
                            );
                            self.window_theme_type(connection, sender_id, r#type.try_into()?)
                                .await
                        }
                        5u16 => {
                            let height = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.window_titlebar_height({})",
                                sender_id,
                                height
                            );
                            self.window_titlebar_height(connection, sender_id, height)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_prelaunch_splash_v1 {
    #[doc = "This interface is a manager for creating prelaunch splash screens."]
    #[doc = "A prelaunch splash screen is a placeholder surface that is shown"]
    #[doc = "before an application's main window is mapped. This helps to improve"]
    #[doc = "the perceived startup time."]
    #[doc = ""]
    #[doc = "It is particularly useful for application launchers to provide immediate"]
    #[doc = "feedback to the user."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_prelaunch_splash_manager_v1 {
        #[doc = "Trait to implement the treeland_prelaunch_splash_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPrelaunchSplashManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_prelaunch_splash_manager_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_prelaunch_splash_manager_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Creates a new prelaunch splash screen."]
            #[doc = ""]
            #[doc = "The `app_id` is a string that identifies the application. The compositor"]
            #[doc = "will use this ID together with `sandbox_engine_name` to match the splash"]
            #[doc = "screen with the actual application window when it appears. This"]
            #[doc = "matching mechanism should also work for XWayland windows."]
            #[doc = ""]
            #[doc = "Callers MUST provide a non-empty `sandbox_engine_name` string which"]
            #[doc = "identifies the sandboxing/container."]
            #[doc = ""]
            #[doc = "If there is already an active (not-yet-completed) splash for the same"]
            #[doc = "`sandbox_engine_name` and `app_id`, the compositor will silently ignore"]
            #[doc = "this request (no new splash will be created and no error is raised)."]
            fn create_splash(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                app_id: String,
                sandbox_engine_name: String,
                icon_buffer: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_prelaunch_splash_manager_v1#{}.create_splash(\"{}\", \"{}\", {})",
                        sender_id,
                        app_id,
                        sandbox_engine_name,
                        icon_buffer
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .put_string(Some(sandbox_engine_name))
                        .put_object(icon_buffer)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_screensaver_v1 {
    #[doc = "This object implements a simple idle inhibit protocol used"]
    #[doc = "to implement org.freedesktop.ScreenSaver D-Bus interface."]
    #[doc = ""]
    #[doc = "Call inhibit to prevent treeland from entering idle state."]
    #[doc = "Call uninhibit or disconnect from the global to release"]
    #[doc = "the inhibit."]
    #[doc = ""]
    #[doc = "If the client disconnects from the compositor, the inhibit"]
    #[doc = "associated with that client is automatically released."]
    #[doc = ""]
    #[doc = "There can be only one inhibit per client per time. Calling"]
    #[doc = "inhibit multiple times will raise an error. Call uninhibit"]
    #[doc = "before inhibit to update application_name and reason"]
    #[doc = "recorded."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently"]
    #[doc = "in the testing phase. Backward compatible changes may be"]
    #[doc = "added together with the corresponding interface version"]
    #[doc = "bump. Backward incompatible changes can only be done by"]
    #[doc = "creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_screensaver_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Trying to uninhibit but no active inhibit existed"]
            NotYetInhibited = 0u32,
            #[doc = "Trying to inhibit with an active inhibit existed"]
            AlreadyInhibited = 1u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NotYetInhibited),
                    1u32 => Ok(Self::AlreadyInhibited),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_screensaver_v1 interface. See the module level documentation for more info"]
        pub trait TreelandScreensaverV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_screensaver_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Inhibit idleness with given application_name and reason_for_inhibit."]
            fn inhibit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                application_name: String,
                reason_for_inhibit: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_screensaver_v1#{}.inhibit(\"{}\", \"{}\")",
                        sender_id,
                        application_name,
                        reason_for_inhibit
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(application_name))
                        .put_string(Some(reason_for_inhibit))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Uninhibit idleness previously inhibited by inhibit request."]
            fn uninhibit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_screensaver_v1#{}.uninhibit()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_shortcut_manager_v1 {
    #[doc = "This interface allows a client to get some shell's info."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_shortcut_manager_v1 {
        #[doc = "Trait to implement the treeland_shortcut_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandShortcutManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_shortcut_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "The format of the shortcut key is 'Modify+Key', such as 'Ctrl+Alt+T'."]
            #[doc = "If the format is wrong, the synthesizer will give a \"format error\". If the shortcut"]
            #[doc = "key is already registered,"]
            #[doc = "the compositor will give a \"register error\" and issue a destruction to the context."]
            fn register_shortcut_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                key: String,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v1#{}.register_shortcut_context(\"{}\", {})",
                        sender_id,
                        key,
                        id
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(key))
                        .put_object(Some(id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows a client to listen a shortcut action."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_shortcut_context_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "shortcut register failed"]
            RegisterFailed = 1u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::RegisterFailed),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_shortcut_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandShortcutContextV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_shortcut_context_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_shortcut_context_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn shortcut(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_context_v1#{}.shortcut()",
                                sender_id,
                            );
                            self.shortcut(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_shortcut_manager_v2 {
    #[doc = "This interface allows privileged clients to register global shortcuts."]
    #[doc = ""]
    #[doc = "In treeland, global shortcuts are managed in a per-user context."]
    #[doc = "Shortcuts for different users are isolated, and will not interfere with each other."]
    #[doc = "This allows multiple users to use their own set of global Shortcuts"]
    #[doc = "on the same system without conflicts."]
    #[doc = "This behavior is transparent to the clients of this interface (i.e"]
    #[doc = "the user context used by this protocol is the same as that of the client.)"]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_shortcut_manager_v2 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Direction {
            Down = 1u32,
            Left = 2u32,
            Up = 3u32,
            Right = 4u32,
        }
        impl From<Direction> for u32 {
            fn from(value: Direction) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Direction {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Down),
                    2u32 => Ok(Self::Left),
                    3u32 => Ok(Self::Up),
                    4u32 => Ok(Self::Right),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Direction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Compositor actions that can be assigned to a shortcut."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Action {
            Notify = 1u32,
            Workspace1 = 2u32,
            Workspace2 = 3u32,
            Workspace3 = 4u32,
            Workspace4 = 5u32,
            Workspace5 = 6u32,
            Workspace6 = 7u32,
            PrevWorkspace = 8u32,
            NextWorkspace = 9u32,
            ShowDesktop = 10u32,
            Maximize = 11u32,
            CancelMaximize = 12u32,
            MoveWindow = 13u32,
            CloseWindow = 14u32,
            ShowWindowMenu = 15u32,
            OpenMultitaskView = 16u32,
            CloseMultitaskView = 17u32,
            ToggleMultitaskView = 18u32,
            ToggleFpsDisplay = 19u32,
            Lockscreen = 20u32,
            ShutdownMenu = 21u32,
            Quit = 22u32,
            TaskswitchEnter = 23u32,
            TaskswitchNext = 24u32,
            TaskswitchPrev = 25u32,
            TaskswitchSameappNext = 26u32,
            TaskswitchSameappPrev = 27u32,
        }
        impl From<Action> for u32 {
            fn from(value: Action) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Action {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Notify),
                    2u32 => Ok(Self::Workspace1),
                    3u32 => Ok(Self::Workspace2),
                    4u32 => Ok(Self::Workspace3),
                    5u32 => Ok(Self::Workspace4),
                    6u32 => Ok(Self::Workspace5),
                    7u32 => Ok(Self::Workspace6),
                    8u32 => Ok(Self::PrevWorkspace),
                    9u32 => Ok(Self::NextWorkspace),
                    10u32 => Ok(Self::ShowDesktop),
                    11u32 => Ok(Self::Maximize),
                    12u32 => Ok(Self::CancelMaximize),
                    13u32 => Ok(Self::MoveWindow),
                    14u32 => Ok(Self::CloseWindow),
                    15u32 => Ok(Self::ShowWindowMenu),
                    16u32 => Ok(Self::OpenMultitaskView),
                    17u32 => Ok(Self::CloseMultitaskView),
                    18u32 => Ok(Self::ToggleMultitaskView),
                    19u32 => Ok(Self::ToggleFpsDisplay),
                    20u32 => Ok(Self::Lockscreen),
                    21u32 => Ok(Self::ShutdownMenu),
                    22u32 => Ok(Self::Quit),
                    23u32 => Ok(Self::TaskswitchEnter),
                    24u32 => Ok(Self::TaskswitchNext),
                    25u32 => Ok(Self::TaskswitchPrev),
                    26u32 => Ok(Self::TaskswitchSameappNext),
                    27u32 => Ok(Self::TaskswitchSameappPrev),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Action {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Flags to specify the keybinding mode."] # [doc = "with key_press, the action is triggered on key press."] # [doc = "with key_release, the action is triggered on key release."] # [doc = "with repeat, the action is repeatedly triggered if the key is held down."] # [doc = ""] # [doc = "Examples:"] # [doc = "key_press | repeat: the action is triggered on key press, and repeatedly"] # [doc = "triggered if the key is held down."] # [doc = "key_press | key_release: the action is triggered on both key press and"] # [doc = "key release, auto-repeated events are ignored."] # [doc = "key_press | key_release | repeat: note that treeland repeats both key"] # [doc = "press and key release events."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct KeybindFlag : u32 { # [doc = "bind key press events"] const KeyPress = 1u32 ; # [doc = "bind key release events"] const KeyRelease = 2u32 ; # [doc = "bind autorepeat events."] const Repeat = 4u32 ; } }
        impl From<KeybindFlag> for u32 {
            fn from(value: KeybindFlag) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for KeybindFlag {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for KeybindFlag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Error codes indicating the reason of a binding failure."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum BindError {
            NameConflict = 1u32,
            DuplicateBinding = 2u32,
            InvalidArgument = 3u32,
            InternalError = 4u32,
        }
        impl From<BindError> for u32 {
            fn from(value: BindError) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for BindError {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NameConflict),
                    2u32 => Ok(Self::DuplicateBinding),
                    3u32 => Ok(Self::InvalidArgument),
                    4u32 => Ok(Self::InternalError),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for BindError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            Occupied = 1u32,
            NotAcquired = 2u32,
            InvalidCommit = 3u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Occupied),
                    2u32 => Ok(Self::NotAcquired),
                    3u32 => Ok(Self::InvalidCommit),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_shortcut_manager_v2 interface. See the module level documentation for more info"]
        pub trait TreelandShortcutManagerV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_shortcut_manager_v2";
            const VERSION: u32 = 2u32;
            #[doc = "Destroy the shortcut manager."]
            #[doc = "Existing shortcuts created through this interface remain valid."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_shortcut_manager_v2#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Acquire the shortcut manager for the current client."]
            #[doc = ""]
            #[doc = "This request must be sent before any bind/unbind request can be performed."]
            #[doc = ""]
            #[doc = "Only one client hold exclusive control of the shortcut manager at a time,"]
            #[doc = "for a given session."]
            #[doc = "If the shortcut manager is already acquired by another client, an protocol error"]
            fn acquire(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_shortcut_manager_v2#{}.acquire()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Bind a key sequence to a compositor action."]
            #[doc = ""]
            #[doc = "The key sequence is specified in the string format used by QKeySequence,"]
            #[doc = "see https://doc.qt.io/qt-6/qkeysequence.html#toString for details."]
            #[doc = ""]
            #[doc = "Sending this request without first acquiring the shortcut manager"]
            #[doc = "will result in a `not_acquired` protocol error."]
            #[doc = ""]
            #[doc = "The name argument must be unique among all existing bindings."]
            #[doc = "If a binding with the same name already exists, the bind_key request will fail."]
            #[doc = ""]
            #[doc = "The action argument specifies the compositor action to be executed"]
            #[doc = "when the key sequence is activated."]
            #[doc = ""]
            #[doc = "Each keybind has a flags argument to specify the exact condition of triggering,"]
            #[doc = "see documentation of keybind_flag enum for details."]
            #[doc = ""]
            #[doc = "If a binding with the same key sequence and action already exists,"]
            #[doc = "its flags will be updated to the new value."]
            #[doc = ""]
            #[doc = "Note that the binding will not take effect until a commit request is sent."]
            fn bind_key(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                key: String,
                flags: KeybindFlag,
                action: Action,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.bind_key(\"{}\", \"{}\", {}, {})",
                        sender_id,
                        name,
                        key,
                        flags,
                        action
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_string(Some(key))
                        .put_uint(flags.into())
                        .put_uint(action.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Bind a swipe gesture to a compositor action."]
            #[doc = ""]
            #[doc = "Sending this request without first acquiring the shortcut manager"]
            #[doc = "will result in a `not_acquired` protocol error."]
            #[doc = ""]
            #[doc = "The name argument must be unique among all existing bindings."]
            #[doc = "If a binding with the same name already exists, the bind_swipe_gesture request will fail."]
            #[doc = ""]
            #[doc = "The direction argument specifies the direction towards which the swipe gesture must be performed."]
            #[doc = "If this argument is not one of the defined enum values, the bind_swipe_gesture request will fail."]
            #[doc = ""]
            #[doc = "The action argument specifies the compositor action to be executed"]
            #[doc = "when the swipe gesture is activated."]
            #[doc = "If a binding with the same gesture and action already exists,"]
            #[doc = "the bind_swipe_gesture request will fail."]
            #[doc = ""]
            #[doc = "Note that the binding will not take effect until a commit request is sent."]
            fn bind_swipe_gesture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                finger: u32,
                direction: Direction,
                action: Action,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.bind_swipe_gesture(\"{}\", {}, {}, {})",
                        sender_id,
                        name,
                        finger,
                        direction,
                        action
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(finger)
                        .put_uint(direction.into())
                        .put_uint(action.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Bind a hold gesture to a compositor action."]
            #[doc = ""]
            #[doc = "Sending this request without first acquiring the shortcut manager"]
            #[doc = "will result in a `not_acquired` protocol error."]
            #[doc = ""]
            #[doc = "The name argument must be unique among all existing bindings."]
            #[doc = "If a binding with the same name already exists, the bind_hold_gesture request will fail."]
            #[doc = ""]
            #[doc = "The action argument specifies the compositor action to be executed"]
            #[doc = "when the hold gesture is activated."]
            #[doc = "If a binding with the same gesture and action already exists,"]
            #[doc = "the bind_hold_gesture request will fail."]
            #[doc = ""]
            #[doc = "Note that the binding will not take effect until a commit request is sent."]
            fn bind_hold_gesture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                finger: u32,
                action: Action,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.bind_hold_gesture(\"{}\", {}, {})",
                        sender_id,
                        name,
                        finger,
                        action
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(finger)
                        .put_uint(action.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Commit the pending bindings."]
            #[doc = ""]
            #[doc = "This request applies all the bind_key, bind_swipe_gesture and bind_hold_gesture"]
            #[doc = "requests sent since the last commit."]
            #[doc = ""]
            #[doc = "After processing this request, the compositor will emit a `commit_status` event"]
            #[doc = "if the commit was successful or `commit_failure` event if the commit failed."]
            #[doc = ""]
            #[doc = "On a successful commit, all the pending bindings will take effect."]
            #[doc = "On a failed commit, none of the pending bindings will take effect."]
            #[doc = ""]
            #[doc = "Sending this request without first acquiring the shortcut manager"]
            #[doc = "will result in a `not_acquired` protocol error."]
            #[doc = ""]
            #[doc = "Sending further commit requests before `commit_success` or `commit_failure`"]
            #[doc = "is sent is a protocol error."]
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_shortcut_manager_v2#{}.commit()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Remove an existing binding."]
            #[doc = ""]
            #[doc = "The binding to be removed is identified by its unique name."]
            #[doc = "If no binding with the specified name exists, the unbind request has no effect."]
            fn unbind(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.unbind(\"{}\")",
                        sender_id,
                        name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted when a binding registered with action `notify` is activated."]
            #[doc = ""]
            #[doc = "the flags argument indicates the type of key event as defined in keybind_flag enum."]
            fn activated(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                flags: KeybindFlag,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted in response to a commit request,"]
            #[doc = "indicating that the commit was successful."]
            fn commit_success(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted in response to a commit request,"]
            #[doc = "indicating that the commit has failed."]
            #[doc = ""]
            #[doc = "The error argument indicates the first error that caused the commit to fail."]
            fn commit_failure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                error: BindError,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let flags = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.activated(\"{}\", {})",
                                sender_id,
                                name,
                                flags
                            );
                            self.activated(connection, sender_id, name, flags.try_into()?)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.commit_success()",
                                sender_id,
                            );
                            self.commit_success(connection, sender_id).await
                        }
                        2u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let error = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.commit_failure(\"{}\", {})",
                                sender_id,
                                name,
                                error
                            );
                            self.commit_failure(connection, sender_id, name, error.try_into()?)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_virtual_output_manager_v1 {
    #[doc = "This interface is a manager that allows the creation of copied output."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_virtual_output_manager_v1 {
        #[doc = "Trait to implement the treeland_virtual_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandVirtualOutputManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_virtual_output_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Create virtual output that can be used when setting screen copy mode for use"]
            #[doc = "on multiple screens. Virtual outputs are created to mirror multiple wl_output"]
            #[doc = "outputs."]
            #[doc = ""]
            #[doc = "The element of the array is the name of the screen."]
            #[doc = ""]
            #[doc = "The first element of the array outputs is the screen to be copied, and"]
            #[doc = "the subsequent elements are the screens to be mirrored."]
            #[doc = ""]
            #[doc = "The client calling this interface will not generate an additional wl_output"]
            #[doc = "object on the client."]
            fn create_virtual_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                name: String,
                outputs: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_virtual_output_manager_v1#{}.create_virtual_output({}, \"{}\", array[{}])",
                        sender_id,
                        id,
                        name,
                        outputs.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_string(Some(name))
                        .put_array(outputs)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Gets a list of virtual output names."]
            fn get_virtual_output_list(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_virtual_output_manager_v1#{}.get_virtual_output_list()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The client obtains the corresponding virtual_output_v1 object"]
            #[doc = "through the virtual output name."]
            fn get_virtual_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_virtual_output_manager_v1#{}.get_virtual_output(\"{}\", {})",
                        sender_id,
                        name,
                        id
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_object(Some(id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sends a list of virtual output names to the client."]
            fn virtual_output_list(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                names: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let names = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.virtual_output_list(array[{}])",
                                sender_id,
                                names.len()
                            );
                            self.virtual_output_list(connection, sender_id, names).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A treeland_virtual_output_v1 represents a set virtual screen output object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_virtual_output_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Group name is empty"]
            InvalidGroupName = 0u32,
            #[doc = "The number of screens applying for copy mode is less than 2"]
            InvalidScreenNumber = 1u32,
            #[doc = "Output does not exist"]
            InvalidOutput = 2u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidGroupName),
                    1u32 => Ok(Self::InvalidScreenNumber),
                    2u32 => Ok(Self::InvalidOutput),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_virtual_output_v1 interface. See the module level documentation for more info"]
        pub trait TreelandVirtualOutputV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_virtual_output_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the output."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_virtual_output_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent to the client when any screen in the array changes."]
            #[doc = ""]
            #[doc = "The element of the array is the name of the screen."]
            #[doc = ""]
            #[doc = "The first element of the array outputs is the screen to be copied, and"]
            #[doc = "the subsequent elements are the screens to be mirrored."]
            #[doc = ""]
            #[doc = "When the primary screen (the screen being copied) is removed, a successor"]
            #[doc = "is selected from the queue as the primary screen, and the queue information"]
            #[doc = "is updated."]
            fn outputs(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                outputs: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "When an error occurs, an error event is emitted, terminating the replication"]
            #[doc = "mode request issued by the client."]
            fn error(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                code: u32,
                message: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let outputs = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_v1#{}.outputs(\"{}\", array[{}])",
                                sender_id,
                                name,
                                outputs.len()
                            );
                            self.outputs(connection, sender_id, name, outputs).await
                        }
                        1u16 => {
                            let code = message.uint()?;
                            let message = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_v1#{}.error({}, \"{}\")",
                                sender_id,
                                code,
                                message
                            );
                            self.error(connection, sender_id, code, message).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_wallpaper_color_v1 {
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_color_manager_v1 {
        #[doc = "Trait to implement the treeland_wallpaper_color_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperColorManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wallpaper_color_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Monitor the wallpaper color of a given screen."]
            fn watch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_color_manager_v1#{}.watch(\"{}\")",
                        sender_id,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Stop monitor the wallpaper color for the given screen."]
            fn unwatch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_color_manager_v1#{}.unwatch(\"{}\")",
                        sender_id,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The client no longer cares about wallpaper_color."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_color_manager_v1#{}.destroy()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tell the client that the wallpaper color of the screen it is monitoring has changed."]
            #[doc = "This event will also be sent immediately when the client requests a watch."]
            fn output_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
                isdark: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let output = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let isdark = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.output_color(\"{}\", {})",
                                sender_id,
                                output,
                                isdark
                            );
                            self.output_color(connection, sender_id, output, isdark)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_wallpaper_manager_unstable_v1 {
    #[doc = "The treeland_wallpaper_manager_v1 interface is a global object used to"]
    #[doc = "create wallpaper objects for specific outputs."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_manager_v1 {
        #[doc = "Trait to implement the treeland_wallpaper_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wallpaper_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this treeland_wallpaper_manager_v1 object."]
            #[doc = ""]
            #[doc = "Destroying a bound treeland_wallpaper_manager_v1 object while there"]
            #[doc = "are wallpapers still alive created by this treeland_wallpaper_manager_v1"]
            #[doc = "object instance is illegal and will result in a protocol error."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_manager_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Creates a treeland_wallpaper_v1 object bound to the specified wl_output."]
            #[doc = ""]
            #[doc = "The compositor may enforce that only one wallpaper exists for a given"]
            #[doc = "(output, role) combination."]
            #[doc = ""]
            #[doc = "When a surface is associated, the compositor will track the workspace"]
            #[doc = "the surface belongs to. Whenever the workspace changes, the compositor"]
            #[doc = "may notify the client of the current wallpaper file path used by that"]
            #[doc = "workspace."]
            #[doc = ""]
            #[doc = "Note: The treeland_wallpaper_v1 object must be destroyed before"]
            #[doc = "the associated wl_surface is destroyed."]
            fn get_treeland_wallpaper(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                output: waynest::ObjectId,
                surface: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_manager_v1#{}.get_treeland_wallpaper({}, {}, {})",
                        sender_id,
                        id,
                        output,
                        surface
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(output))
                        .put_object(surface)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_wallpaper_v1 interface represents a wallpaper instance"]
    #[doc = "created by the compositor for a specific output, role."]
    #[doc = ""]
    #[doc = "The wallpaper source provided by the client is stored and managed on"]
    #[doc = "the compositor side. Once a wallpaper source has been set,"]
    #[doc = "the compositor may cache, reuse, or persist the source independently"]
    #[doc = "of the client."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wallpaper file has already been used and is cached on the wallpaper side"]
            AlreadyUsed = 0u32,
            #[doc = "The specified wallpaper source is invalid, unsupported, or failed to load."]
            InvalidSource = 1u32,
            #[doc = "Permission denied when opening the specified wallpaper source"]
            PermissionDenied = 2u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    1u32 => Ok(Self::InvalidSource),
                    2u32 => Ok(Self::PermissionDenied),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum WallpaperSourceType {
            #[doc = "The wallpaper source is an image"]
            Image = 0u32,
            #[doc = "The wallpaper source is a video"]
            Video = 1u32,
        }
        impl From<WallpaperSourceType> for u32 {
            fn from(value: WallpaperSourceType) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for WallpaperSourceType {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Image),
                    1u32 => Ok(Self::Video),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for WallpaperSourceType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum WallpaperRole {
            #[doc = "Wallpaper for the desktop environment"]
            Desktop = 1u32,
            #[doc = "Wallpaper for the lock screen"]
            Lockscreen = 2u32,
        }
        impl From<WallpaperRole> for u32 {
            fn from(value: WallpaperRole) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for WallpaperRole {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Desktop),
                    2u32 => Ok(Self::Lockscreen),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for WallpaperRole {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wallpaper_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wallpaper_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the treeland_wallpaper_v1 object and releases all resources"]
            #[doc = "associated with it on the compositor side."]
            #[doc = ""]
            #[doc = "After calling this request, the client must not use the wallpaper"]
            #[doc = "object anymore."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets an image as the wallpaper source, apply to the current"]
            #[doc = "wl_output and the active workspace. except for lockscreen"]
            #[doc = "wallpapers, only one wallpaper can be set per wl_output."]
            #[doc = "Supported formats:"]
            #[doc = ""]
            #[doc = "- JPG / JPEG (Joint Photographic Experts Group)"]
            #[doc = ""]
            #[doc = "The compositor will attempt to load and display the specified image"]
            #[doc = "file. If the file cannot be accessed, decoded, or is in an unsupported"]
            #[doc = "format, the compositor will emit a failed event with an appropriate"]
            #[doc = "error code."]
            #[doc = ""]
            #[doc = "Access Control: This protocol accepts file_source such"]
            #[doc = "as /usr/share/wallpapers/xxx  or /var/cache/wallpapers/xxxx provided"]
            #[doc = "by the client. the compositor will perform file verification."]
            fn set_image_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                file_source: String,
                role: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_v1#{}.set_image_source(\"{}\", {})",
                        sender_id,
                        file_source,
                        role
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(file_source))
                        .put_uint(role)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets a video file as the wallpaper source, apply to the current"]
            #[doc = "wl_output and the active workspace. except for lockscreen"]
            #[doc = "wallpapers, only one wallpaper can be set per wl_output."]
            #[doc = "Supported formats:"]
            #[doc = ""]
            #[doc = "- MP4"]
            #[doc = "- AVI"]
            #[doc = "- MOV"]
            #[doc = ""]
            #[doc = "The compositor is responsible for decoding and presenting the video."]
            #[doc = "Playback behavior such as looping, synchronization, and performance"]
            #[doc = "characteristics are compositor-defined."]
            #[doc = ""]
            #[doc = "If the video source cannot be opened or decoded, a failed event"]
            #[doc = "will be emitted."]
            #[doc = ""]
            #[doc = "Access Control: This protocol accepts file_source such"]
            #[doc = "as /usr/share/wallpapers/xxx or /var/cache/wallpapers/xxxx provided"]
            #[doc = "by the client. the compositor will perform file verification."]
            fn set_video_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                file_source: String,
                role: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_v1#{}.set_video_source(\"{}\", {})",
                        sender_id,
                        file_source,
                        role
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(file_source))
                        .put_uint(role)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted when a wallpaper-related request fails."]
            #[doc = ""]
            #[doc = "The failure may occur during source loading, validation, or runtime"]
            #[doc = "setup. The error code provides additional information about the cause"]
            #[doc = "of the failure."]
            #[doc = ""]
            #[doc = "Possible error values include:"]
            #[doc = "- already_used: The wallpaper source is already configured. This is"]
            #[doc = "not a fatal error and serves as a notification to the client."]
            #[doc = "- invalid_source: The specified wallpaper source is invalid,"]
            #[doc = "unsupported, or could not be processed. This is a fatal error and"]
            #[doc = "the client should verify the source before retrying."]
            #[doc = "- Permission denied, check file permissions."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                file_source: String,
                error: Error,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted by the compositor to notify the client that the"]
            #[doc = "wallpaper source associated with this treeland_wallpaper_v1 object"]
            #[doc = "has changed."]
            #[doc = ""]
            #[doc = "The event may be emitted during initial object setup or when the"]
            #[doc = "wallpaper is changed due to external factors not initiated by this"]
            #[doc = "client, such as compositor policy decisions, workspace switches"]
            #[doc = "(except for lockscreen wallpapers) that affect the active"]
            #[doc = "wallpaper, or other system components."]
            #[doc = ""]
            #[doc = "Because the desktop wallpaper is designed to support different"]
            #[doc = "workspaces. if the client is a surface, changes to its displayed"]
            #[doc = "workspace will also send (reference set_reference_surface)."]
            #[doc = ""]
            #[doc = "The event provides the current wallpaper source type and the"]
            #[doc = "corresponding source file path."]
            fn changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                role: WallpaperRole,
                source_type: WallpaperSourceType,
                file_source: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let error = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_v1#{}.failed(\"{}\", {})",
                                sender_id,
                                file_source,
                                error
                            );
                            self.failed(connection, sender_id, file_source, error.try_into()?)
                                .await
                        }
                        1u16 => {
                            let role = message.uint()?;
                            let source_type = message.uint()?;
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_v1#{}.changed({}, {}, \"{}\")",
                                sender_id,
                                role,
                                source_type,
                                file_source
                            );
                            self.changed(
                                connection,
                                sender_id,
                                role.try_into()?,
                                source_type.try_into()?,
                                file_source,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_wallpaper_shell_unstable_v1 {
    #[doc = "The treeland_wallpaper_notifier_v1 interface provides notifications"]
    #[doc = "about the availability and lifetime of wallpaper sources managed by"]
    #[doc = "the compositor."]
    #[doc = ""]
    #[doc = "This interface is purely event-driven. Clients receive events when"]
    #[doc = "wallpaper sources are added, become active, or are removed, and may"]
    #[doc = "update their internal state or user interface accordingly."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_notifier_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum WallpaperSourceType {
            #[doc = "The wallpaper source is an image"]
            Image = 0u32,
            #[doc = "The wallpaper source is a video"]
            Video = 1u32,
        }
        impl From<WallpaperSourceType> for u32 {
            fn from(value: WallpaperSourceType) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for WallpaperSourceType {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Image),
                    1u32 => Ok(Self::Video),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for WallpaperSourceType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wallpaper_notifier_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperNotifierV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wallpaper_notifier_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys this treeland_wallpaper_notifier_v1 object."]
            #[doc = ""]
            #[doc = "Destroying a bound treeland_wallpaper_notifier_v1 object while"]
            #[doc = "wallpaper surface objects created from it are still alive is"]
            #[doc = "illegal and will result in a protocol error."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_notifier_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent by the compositor when a new wallpaper source"]
            #[doc = "is added or becomes active."]
            #[doc = ""]
            #[doc = "The source_type argument describes the type of the wallpaper"]
            #[doc = "source. The meaning of the file_source argument depends on the"]
            #[doc = "reported source_type."]
            fn add(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                source_type: WallpaperSourceType,
                file_source: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the wallpaper source identified by"]
            #[doc = "file_source is no longer available."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should discard any internal"]
            #[doc = "state associated with the wallpaper source and must not reference"]
            #[doc = "it anymore."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                file_source: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let source_type = message.uint()?;
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_notifier_v1#{}.add({}, \"{}\")",
                                sender_id,
                                source_type,
                                file_source
                            );
                            self.add(connection, sender_id, source_type.try_into()?, file_source)
                                .await
                        }
                        1u16 => {
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_notifier_v1#{}.remove(\"{}\")",
                                sender_id,
                                file_source
                            );
                            self.remove(connection, sender_id, file_source).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_wallpaper_shell_v1 interface is a global object exposed by"]
    #[doc = "the compositor that allows a client to assign the wallpaper role to"]
    #[doc = "a wl_surface and create wallpaper surface objects."]
    #[doc = ""]
    #[doc = "A wallpaper surface represents content intended to be displayed as"]
    #[doc = "the desktop wallpaper, such as a static image or a dynamic media"]
    #[doc = "source, and is associated with exactly one wl_surface."]
    #[doc = ""]
    #[doc = "This interface follows the shell pattern used by other Wayland"]
    #[doc = "protocols: it assigns a specific role to a wl_surface and manages"]
    #[doc = "the lifetime and behavior of wallpaper surfaces."]
    #[doc = ""]
    #[doc = "This interface is a singleton. At most one client may bind to"]
    #[doc = "treeland_wallpaper_shell_v1 at any given time."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_shell_v1 {
        #[doc = "Trait to implement the treeland_wallpaper_shell_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperShellV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wallpaper_shell_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys this treeland_wallpaper_shell_v1 object."]
            #[doc = ""]
            #[doc = "Destroying a bound treeland_wallpaper_shell_v1 object while there"]
            #[doc = "are still treeland_wallpaper_surface_v1 objects created from it"]
            #[doc = "is illegal and will result in a protocol error."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_shell_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Creates a treeland_wallpaper_surface_v1 object and assigns the"]
            #[doc = "wallpaper role to the given wl_surface."]
            #[doc = ""]
            #[doc = "The provided wl_surface must not already have a role, and while"]
            #[doc = "it is used as a wallpaper surface, it must not be assigned any"]
            #[doc = "other role."]
            #[doc = ""]
            #[doc = "The file_source argument specifies the initial wallpaper source"]
            #[doc = "identifier. Its interpretation depends on the wallpaper source"]
            #[doc = "type selected by the compositor."]
            fn get_treeland_wallpaper_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
                file_source: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_shell_v1#{}.get_treeland_wallpaper_surface({}, {}, \"{}\")",
                        sender_id,
                        id,
                        surface,
                        file_source
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(surface))
                        .put_string(Some(file_source))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_wallpaper_surface_v1 interface controls the behavior of a"]
    #[doc = "wallpaper surface."]
    #[doc = ""]
    #[doc = "A wallpaper surface is bound to exactly one wl_surface and defines how"]
    #[doc = "that surface should be sized, positioned, and synchronized with the"]
    #[doc = "compositor's wallpaper management logic."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "The specified wallpaper source is invalid, unsupported, or failed to load."]
            InvalidSource = 1u32,
            #[doc = "Permission denied when opening the specified wallpaper source"]
            PermissionDenied = 2u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidSource),
                    2u32 => Ok(Self::PermissionDenied),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wallpaper_surface_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperSurfaceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wallpaper_surface_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the wallpaper surface object and release its association with"]
            #[doc = "the underlying wl_surface."]
            #[doc = ""]
            #[doc = "After calling this request, the wallpaper surface object becomes"]
            #[doc = "invalid and must not be used again."]
            #[doc = ""]
            #[doc = "This request should only be sent when the client intends to"]
            #[doc = "permanently stop using the wallpaper surface, such as when the"]
            #[doc = "client is shutting down or after receiving the"]
            #[doc = "treeland_wallpaper_produce_v1.removed event for this surface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_surface_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Reports to the compositor that a wallpaper-related operation has"]
            #[doc = "failed on the client side."]
            #[doc = ""]
            #[doc = "This request may be sent by the client when it fails to open,"]
            #[doc = "load, or otherwise process the configured wallpaper source."]
            #[doc = "The compositor may use this information for diagnostics, policy"]
            #[doc = "decisions, or to adjust internal state."]
            #[doc = ""]
            #[doc = "Possible error values include:"]
            #[doc = "- invalid_source: The specified wallpaper source is invalid,"]
            #[doc = "unsupported, or could not be processed. This is a fatal error and"]
            #[doc = "indicates that the source should be verified before retrying."]
            #[doc = "- permission_denied: The client does not have permission to access"]
            #[doc = "the specified wallpaper source."]
            fn source_failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                error: Error,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_surface_v1#{}.source_failed({})",
                        sender_id,
                        error
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(error.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event provides a position value associated with the wallpaper."]
            #[doc = ""]
            #[doc = "The semantic meaning of the position value is compositor-defined."]
            #[doc = "It may represent a scroll offset, animation progress, or playback"]
            #[doc = "position. The value is expressed as a fixed-point number[0, 1.0]."]
            fn position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                position: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event instructs the client to pause wallpaper updates or"]
            #[doc = "animations."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should stop advancing any"]
            #[doc = "time-based or animated wallpaper content until a play event is"]
            #[doc = "received."]
            fn pause(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the playback speed of the wallpaper content."]
            #[doc = ""]
            #[doc = "A rate of 1.0 represents normal speed."]
            #[doc = "A rate of 0.0 represents a paused state."]
            fn set_playback_rate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rate: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event instructs the client to resume wallpaper updates or"]
            #[doc = "animations after a pause."]
            fn play(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event instructs the client to progressively reduce the playback"]
            #[doc = "or animation update rate of the wallpaper content until it comes to"]
            #[doc = "a complete stop."]
            #[doc = ""]
            #[doc = "The slowdown should be smooth and continuous rather than abrupt."]
            #[doc = ""]
            #[doc = "The duration argument specifies the amount of time, in milliseconds,"]
            #[doc = "over which the slowdown should occur. After this duration has elapsed,"]
            #[doc = "the client should consider the wallpaper paused."]
            #[doc = ""]
            #[doc = "A duration of 0 means the client should pause immediately."]
            fn slow_down(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                duration: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let position = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_surface_v1#{}.position({})",
                                sender_id,
                                position
                            );
                            self.position(connection, sender_id, position).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_wallpaper_surface_v1#{}.pause()", sender_id,);
                            self.pause(connection, sender_id).await
                        }
                        2u16 => {
                            let rate = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_surface_v1#{}.set_playback_rate({})",
                                sender_id,
                                rate
                            );
                            self.set_playback_rate(connection, sender_id, rate).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_wallpaper_surface_v1#{}.play()", sender_id,);
                            self.play(connection, sender_id).await
                        }
                        4u16 => {
                            let duration = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_surface_v1#{}.slow_down({})",
                                sender_id,
                                duration
                            );
                            self.slow_down(connection, sender_id, duration).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_window_management_v1 {
    #[doc = "This interface manages application windows."]
    #[doc = "It provides requests to show and hide the desktop and emits"]
    #[doc = "an event every time a window is created so that the client can"]
    #[doc = "use it to manage the window."]
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_window_management_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum DesktopState {
            Normal = 0u32,
            Show = 1u32,
            PreviewShow = 2u32,
        }
        impl From<DesktopState> for u32 {
            fn from(value: DesktopState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for DesktopState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::Show),
                    2u32 => Ok(Self::PreviewShow),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DesktopState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_window_management_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWindowManagementV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_window_management_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Tell the compositor to show/hide the desktop."]
            fn set_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_window_management_v1#{}.set_desktop({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_window_management_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent whenever the show desktop mode changes. E.g. when it is"]
            #[doc = "entered"]
            #[doc = "or left."]
            #[doc = ""]
            #[doc = "On binding the interface the current state is sent."]
            fn show_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_management_v1#{}.show_desktop({})",
                                sender_id,
                                state
                            );
                            self.show_desktop(connection, sender_id, state).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
