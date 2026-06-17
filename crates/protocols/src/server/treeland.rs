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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_app_id_resolver_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_app_id_resolver_manager_v1#{}.get_resolver({})",
                                sender_id,
                                id
                            );
                            self.get_resolver(connection, sender_id, id).await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn identify_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                request_id: u32,
                pidfd: std::os::fd::OwnedFd,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_app_id_resolver_v1#{}.identify_request({}, {})",
                        sender_id,
                        request_id,
                        std::os::fd::AsRawFd::as_raw_fd(&pidfd)
                    );
                    waynest::Connection::push_fd(connection, pidfd);
                    let payload = waynest::PayloadBuilder::new().put_uint(request_id).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            let app_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let sandbox_engine_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_app_id_resolver_v1#{}.respond({}, \"{}\", \"{}\")",
                                sender_id,
                                request_id,
                                app_id,
                                sandbox_engine_name
                            );
                            self.respond(
                                connection,
                                sender_id,
                                request_id,
                                app_id,
                                sandbox_engine_name,
                            )
                            .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_app_id_resolver_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Start session and keeps sending frame."]
            fn start(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_session_v1#{}.frame({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
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
                    let payload = waynest::PayloadBuilder::new()
                        .put_int(offset_x)
                        .put_int(offset_y)
                        .put_uint(width)
                        .put_uint(height)
                        .put_uint(buffer_flags)
                        .put_uint(flags.into())
                        .put_uint(format)
                        .put_uint(mod_high)
                        .put_uint(mod_low)
                        .put_uint(num_objects)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_session_v1#{}.object({}, {}, {}, {}, {}, {})",
                        sender_id,
                        index,
                        std::os::fd::AsRawFd::as_raw_fd(&fd),
                        size,
                        offset,
                        stride,
                        plane_index
                    );
                    waynest::Connection::push_fd(connection, fd);
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(index)
                        .put_uint(size)
                        .put_uint(offset)
                        .put_uint(stride)
                        .put_uint(plane_index)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent as soon as the frame is presented, indicating it is available for reading. This event"]
            #[doc = "includes the time at which presentation happened at."]
            fn ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_session_v1#{}.ready({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "If the capture failed or if the frame is no longer valid after the \"frame\" event has been emitted, this"]
            #[doc = "event will be used to inform the client to scrap the frame."]
            fn cancel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_capture_unstable_v1 :: treeland_capture_session_v1 :: CancelReason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_session_v1#{}.cancel({})",
                        sender_id,
                        reason
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_capture_session_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_capture_session_v1#{}.start()", sender_id,);
                            self.start(connection, sender_id).await
                        }
                        2u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_usec = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_session_v1#{}.frame_done({}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_usec
                            );
                            self.frame_done(connection, sender_id, tv_sec_hi, tv_sec_lo, tv_usec)
                                .await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Copy capture contents to provided buffer"]
            fn copy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Inform client to prepare buffer."]
            fn buffer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                format: super::super::super::core::wayland::wl_shm::Format,
                width: u32,
                height: u32,
                stride: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_frame_v1#{}.buffer({}, {}, {}, {})",
                        sender_id,
                        format,
                        width,
                        height,
                        stride
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(format.into())
                        .put_uint(width)
                        .put_uint(height)
                        .put_uint(stride)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform client that all buffer formats supported are emitted."]
            fn buffer_done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_frame_v1#{}.buffer_done()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Provides flags about the frame. This event is sent once before the"]
            #[doc = "\"ready\" event."]
            fn flags(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags : super :: super :: super :: treeland :: treeland_capture_unstable_v1 :: treeland_capture_frame_v1 :: Flags,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_frame_v1#{}.flags({})",
                        sender_id,
                        flags
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(flags.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform that buffer is ready for reading"]
            fn ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_frame_v1#{}.ready()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform that frame copy fails."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_capture_frame_v1#{}.failed()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_capture_frame_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_frame_v1#{}.copy({})",
                                sender_id,
                                buffer
                            );
                            self.copy(connection, sender_id, buffer).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event can be called just once. A second call might result in a protocol error cause"]
            #[doc = "we just provide transient"]
            fn capture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                frame: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Often used by a screen recorder."]
            fn create_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                session: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_context_v1#{}.source_ready({}, {}, {}, {}, {})",
                        sender_id,
                        region_x,
                        region_y,
                        region_width,
                        region_height,
                        source_type
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_int(region_x)
                        .put_int(region_y)
                        .put_uint(region_width)
                        .put_uint(region_height)
                        .put_uint(source_type.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "There could a lot of reasons but the most common one is that selector is busy"]
            fn source_failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason: SourceFailure,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_capture_context_v1#{}.source_failed({})",
                        sender_id,
                        reason
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_capture_context_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let source_hint = message.uint()?;
                            let freeze = message.uint()?;
                            let with_cursor = message.uint()?;
                            let mask = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_context_v1#{}.select_source({}, {}, {}, {})",
                                sender_id,
                                source_hint,
                                freeze,
                                with_cursor,
                                mask.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.select_source(
                                connection,
                                sender_id,
                                source_hint.try_into()?,
                                freeze,
                                with_cursor,
                                mask,
                            )
                            .await
                        }
                        2u16 => {
                            let frame = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_context_v1#{}.capture({})",
                                sender_id,
                                frame
                            );
                            self.capture(connection, sender_id, frame).await
                        }
                        3u16 => {
                            let session = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_context_v1#{}.create_session({})",
                                sender_id,
                                session
                            );
                            self.create_session(connection, sender_id, session).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                context: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            tracing::debug!("treeland_capture_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let context = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_capture_manager_v1#{}.get_context({})",
                                sender_id,
                                context
                            );
                            self.get_context(connection, sender_id, context).await
                        }
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
            const VERSION: u32 = 2u32;
            fn get_window_overlap_checker(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a new dde active for a given seat."]
            fn get_treeland_dde_active(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a new multitaskview context for toggle."]
            fn get_treeland_multitaskview(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a new window picker to pick window."]
            fn get_treeland_window_picker(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a new lockscreen context for toggle."]
            fn get_treeland_lockscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the treeland_dde_shell_manager_v1 object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_window_overlap_checker({})",
                                sender_id,
                                id
                            );
                            self.get_window_overlap_checker(connection, sender_id, id)
                                .await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_shell_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_shell_surface(connection, sender_id, id, surface)
                                .await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_dde_active({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_treeland_dde_active(connection, sender_id, id, seat)
                                .await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_multitaskview({})",
                                sender_id,
                                id
                            );
                            self.get_treeland_multitaskview(connection, sender_id, id)
                                .await
                        }
                        4u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_window_picker({})",
                                sender_id,
                                id
                            );
                            self.get_treeland_window_picker(connection, sender_id, id)
                                .await
                        }
                        5u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_lockscreen({})",
                                sender_id,
                                id
                            );
                            self.get_treeland_lockscreen(connection, sender_id, id)
                                .await
                        }
                        6u16 => {
                            let callback = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let wid = message.uint()?;
                            let anchor = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let dx = message.fixed()?;
                            let dy = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.set_xwindow_position_relative({}, {}, {}, {}, {})",
                                sender_id,
                                callback,
                                wid,
                                anchor,
                                dx,
                                dy
                            );
                            self.set_xwindow_position_relative(
                                connection, sender_id, callback, wid, anchor, dx, dy,
                            )
                            .await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroys the treeland_window_overlap_checker object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent when windows overlapped."]
            #[doc = "This event is sent only once."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_window_overlap_checker#{}.enter()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent when windows not overlapped."]
            #[doc = "This event is sent only once."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_window_overlap_checker#{}.leave()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            let width = message.int()?;
                            let height = message.int()?;
                            let anchor = message.uint()?;
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_overlap_checker#{}.update({}, {}, {}, {})",
                                sender_id,
                                width,
                                height,
                                anchor,
                                output
                            );
                            self.update(
                                connection,
                                sender_id,
                                width,
                                height,
                                anchor.try_into()?,
                                output,
                            )
                            .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_overlap_checker#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Assign a role to a shell surface."]
            fn set_role(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                role : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_shell_surface_v1 :: Role,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a switcher."]
            fn set_skip_switcher(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a dock preview."]
            fn set_skip_dock_preview(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a mutitask view."]
            fn set_skip_muti_task_view(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Setting this will determine whether the surface can receive keyboard focus."]
            #[doc = "When set to 0, the surface will not receive keyboard focus even when clicked or activated."]
            #[doc = "When set to 1 (default), the surface will receive keyboard focus normally."]
            fn set_accept_keyboard_focus(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                accept: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_dde_shell_surface_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_surface_position({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.set_surface_position(connection, sender_id, x, y).await
                        }
                        2u16 => {
                            let role = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_role({})",
                                sender_id,
                                role
                            );
                            self.set_role(connection, sender_id, role.try_into()?).await
                        }
                        3u16 => {
                            let y_offset = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_auto_placement({})",
                                sender_id,
                                y_offset
                            );
                            self.set_auto_placement(connection, sender_id, y_offset)
                                .await
                        }
                        4u16 => {
                            let skip = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_skip_switcher({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_switcher(connection, sender_id, skip).await
                        }
                        5u16 => {
                            let skip = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_skip_dock_preview({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_dock_preview(connection, sender_id, skip)
                                .await
                        }
                        6u16 => {
                            let skip = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_skip_muti_task_view({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_muti_task_view(connection, sender_id, skip)
                                .await
                        }
                        7u16 => {
                            let accept = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_accept_keyboard_focus({})",
                                sender_id,
                                accept
                            );
                            self.set_accept_keyboard_focus(connection, sender_id, accept)
                                .await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn active_in(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_active_v1 :: Reason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_active_v1#{}.active_in({})",
                        sender_id,
                        reason
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn active_out(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_active_v1 :: Reason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_dde_active_v1#{}.active_out({})",
                        sender_id,
                        reason
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn start_drag(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dde_active_v1#{}.start_drag()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn drop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dde_active_v1#{}.drop()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_dde_active_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Show or hide the multitaskview."]
            fn toggle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            tracing::debug!("treeland_multitaskview_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_multitaskview_v1#{}.toggle()", sender_id,);
                            self.toggle(connection, sender_id).await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Pick a window to get information."]
            fn pick(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Picked window information."]
            fn window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                pid: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_window_picker_v1#{}.window({})", sender_id, pid);
                    let payload = waynest::PayloadBuilder::new().put_int(pid).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_window_picker_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let hint = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_picker_v1#{}.pick(\"{}\")",
                                sender_id,
                                hint
                            );
                            self.pick(connection, sender_id, hint).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Lock the screen."]
            fn lock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Show shutdown."]
            fn shutdown(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Switch user."]
            fn switch_user(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            tracing::debug!("treeland_lockscreen_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_lockscreen_v1#{}.lock()", sender_id,);
                            self.lock(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_lockscreen_v1#{}.shutdown()", sender_id,);
                            self.shutdown(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_lockscreen_v1#{}.switch_user()", sender_id,);
                            self.switch_user(connection, sender_id).await
                        }
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
            const VERSION: u32 = 2u32;
            #[doc = "Send treeland to Greeter mode."]
            fn switch_to_greeter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set lockscreen user to username. Ignore when username is \"ddm\"."]
            fn switch_to_user(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                username: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Activate treeland session. This will makes treeland try to take"]
            #[doc = "control of screen."]
            fn activate_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Deactivate treeland session. This will release control of the"]
            #[doc = "screen, but not to close the current seats."]
            fn deactivate_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Enable treeland rendering. This is primarily called after"]
            #[doc = "disable_render to resume treeland."]
            fn enable_render(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Disable treeland rendering. This will prevent treeland from"]
            #[doc = "output to DRM device."]
            fn disable_render(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                callback: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroys this treeland_ddm_v1 object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Call ddm to switch current virtual terminal to vtnr. ddm should"]
            #[doc = "take care of the switch and call ioctl respectively."]
            fn switch_to_vt(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                vtnr: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_ddm_v1#{}.switch_to_vt({})", sender_id, vtnr);
                    let payload = waynest::PayloadBuilder::new().put_int(vtnr).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Call ddm to acquire control of VT at vtnr. ddm should call"]
            #[doc = "VT_SETMODE respectively."]
            fn acquire_vt(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                vtnr: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_ddm_v1#{}.acquire_vt({})", sender_id, vtnr);
                    let payload = waynest::PayloadBuilder::new().put_int(vtnr).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_ddm_v1#{}.switch_to_greeter()", sender_id,);
                            self.switch_to_greeter(connection, sender_id).await
                        }
                        1u16 => {
                            let username = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_ddm_v1#{}.switch_to_user(\"{}\")",
                                sender_id,
                                username
                            );
                            self.switch_to_user(connection, sender_id, username).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm_v1#{}.activate_session()", sender_id,);
                            self.activate_session(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm_v1#{}.deactivate_session()", sender_id,);
                            self.deactivate_session(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm_v1#{}.enable_render()", sender_id,);
                            self.enable_render(connection, sender_id).await
                        }
                        5u16 => {
                            let callback = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_ddm_v1#{}.disable_render({})",
                                sender_id,
                                callback
                            );
                            self.disable_render(connection, sender_id, callback).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
            const VERSION: u32 = 2u32;
            #[doc = "Indicates the client no longer wishes to receive events for new toplevels."]
            #[doc = "However the compositor may emit further toplevel_created events, until"]
            #[doc = "the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            fn stop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_dock_preview_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                relative_surface: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_manager_v1#{}.toplevel({})",
                        sender_id,
                        toplevel
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the compositor is done sending events to the"]
            #[doc = "treeland_foreign_toplevel_manager_v1. The server will destroy the object"]
            #[doc = "immediately after sending this request, so it will become invalid and"]
            #[doc = "the client should free any resources associated with it."]
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_manager_v1#{}.finished()",
                        sender_id,
                    );
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_foreign_toplevel_manager_v1#{}.stop()",
                                sender_id,
                            );
                            self.stop(connection, sender_id).await
                        }
                        1u16 => {
                            let relative_surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_manager_v1#{}.get_dock_preview_context({}, {})",
                                sender_id,
                                relative_surface,
                                id
                            );
                            self.get_dock_preview_context(
                                connection,
                                sender_id,
                                relative_surface,
                                id,
                            )
                            .await
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
            #[doc = "the toplevel is demanding attention"]
            Attention = 4u32,
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
                    4u32 => Ok(Self::Attention),
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
            const VERSION: u32 = 2u32;
            #[doc = "Requests that the toplevel be maximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn set_maximized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the toplevel be unmaximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn unset_maximized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the toplevel be minimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn set_minimized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the toplevel be unminimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            fn unset_minimized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Request that this toplevel be activated on the given seat."]
            #[doc = "There is no guarantee the toplevel will be actually activated."]
            fn activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroys the treeland_foreign_toplevel_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the toplevel be unfullscreened. If the fullscreen state"]
            #[doc = "actually changes, this will be indicated by the state event."]
            fn unset_fullscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event will be sent when the compositor has set the process id this window"]
            #[doc = "belongs to. This should be set once before the initial_state is sent."]
            fn pid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                pid: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.pid({})",
                        sender_id,
                        pid
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(pid).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            fn title(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                title: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.title(\"{}\")",
                        sender_id,
                        title
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(title))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted whenever the app-id of the toplevel changes."]
            fn app_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                app_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.app_id(\"{}\")",
                        sender_id,
                        app_id
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.identifier({})",
                        sender_id,
                        identifier
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(identifier).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted whenever the toplevel becomes visible on"]
            #[doc = "the given output. A toplevel may be visible on multiple outputs."]
            fn output_enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.output_enter({})",
                        sender_id,
                        output
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted whenever the toplevel stops being visible on"]
            #[doc = "the given output. It is guaranteed that an entered-output event"]
            #[doc = "with the same output has been emitted before this event."]
            fn output_leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.output_leave({})",
                        sender_id,
                        output
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted immediately after the treeland_foreign_toplevel_handle_v1"]
            #[doc = "is created and each time the toplevel state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            fn state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.state(array[{}])",
                        sender_id,
                        state.len()
                    );
                    let payload = waynest::PayloadBuilder::new().put_array(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after all changes in the toplevel state have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "This allows changes to the treeland_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.done()",
                        sender_id,
                    );
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event means the toplevel has been destroyed. It is guaranteed there"]
            #[doc = "won't be any more events for this treeland_foreign_toplevel_handle_v1. The"]
            #[doc = "toplevel itself becomes inert so any requests will be ignored except the"]
            #[doc = "destroy request."]
            fn closed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.closed()",
                        sender_id,
                    );
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted whenever the parent of the toplevel changes."]
            #[doc = ""]
            #[doc = "No event is emitted when the parent handle is destroyed by the client."]
            fn parent(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                parent: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.parent({})",
                        sender_id,
                        parent
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let payload = waynest::PayloadBuilder::new().put_object(parent).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_foreign_toplevel_handle_v1#{}.set_maximized()",
                                sender_id,
                            );
                            self.set_maximized(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.unset_maximized()",
                                sender_id,
                            );
                            self.unset_maximized(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_minimized()",
                                sender_id,
                            );
                            self.set_minimized(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.unset_minimized()",
                                sender_id,
                            );
                            self.unset_minimized(connection, sender_id).await
                        }
                        4u16 => {
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.activate({})",
                                sender_id,
                                seat
                            );
                            self.activate(connection, sender_id, seat).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.close()",
                                sender_id,
                            );
                            self.close(connection, sender_id).await
                        }
                        6u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_rectangle({}, {}, {}, {}, {})",
                                sender_id,
                                surface,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_rectangle(connection, sender_id, surface, x, y, width, height)
                                .await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        8u16 => {
                            let output = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_fullscreen({})",
                                sender_id,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_fullscreen(connection, sender_id, output).await
                        }
                        9u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.unset_fullscreen()",
                                sender_id,
                            );
                            self.unset_fullscreen(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn show_tooltip(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tooltip: String,
                x: i32,
                y: i32,
                direction: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "close preview box"]
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent after mouse enter preview box."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dock_preview_context_v1#{}.enter()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after mouse leave preview box."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_dock_preview_context_v1#{}.leave()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            let surfaces = message.array()?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let direction = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.show(array[{}], {}, {}, {})",
                                sender_id,
                                surfaces.len(),
                                x,
                                y,
                                direction
                            );
                            self.show(connection, sender_id, surfaces, x, y, direction)
                                .await
                        }
                        1u16 => {
                            let tooltip = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let direction = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.show_tooltip(\"{}\", {}, {}, {})",
                                sender_id,
                                tooltip,
                                x,
                                y,
                                direction
                            );
                            self.show_tooltip(connection, sender_id, tooltip, x, y, direction)
                                .await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.close()",
                                sender_id,
                            );
                            self.close(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows a privileged client to discover seat capabilities"]
#[doc = "and configure compositor-managed mouse, touchpad, and keyboard devices."]
#[doc = ""]
#[doc = "A treeland_input_manager_v1 object is a global manager. It creates"]
#[doc = "settings objects that represent configuration state for existing input"]
#[doc = "devices; those settings objects are not the device objects themselves."]
#[doc = ""]
#[doc = "For mouse and touchpad devices, pointer device settings such as"]
#[doc = "acceleration speed, acceleration profile, natural scrolling, scroll"]
#[doc = "factor, handed mode, double-click interval, send events mode,"]
#[doc = "disable-while-typing and tap-to-click are managed through a"]
#[doc = "treeland_pointer_device_configuration_v1 object created from the"]
#[doc = "device-specific settings interface."]
#[allow(clippy::module_inception)]
pub mod treeland_input_manager_unstable_v1 {
    #[doc = "The treeland_input_manager_v1 interface is a global object used to"]
    #[doc = "create per-device settings objects for input devices exposed by the"]
    #[doc = "compositor."]
    #[doc = ""]
    #[doc = "The compositor advertises device capability changes with the"]
    #[doc = "capability_available and capability_unavailable events."]
    #[doc = ""]
    #[doc = "For each target device or seat-level settings scope, the compositor"]
    #[doc = "may allow at most one live settings object of a given interface"]
    #[doc = "created from one manager object."]
    #[doc = ""]
    #[doc = "virtual-pointer and virtual-keyboard are not included in the list."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_input_manager_v1 {
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct DeviceType : u32 { # [doc = "mouse input device"] const Mouse = 1u32 ; # [doc = "touchpad input device"] const Touchpad = 2u32 ; # [doc = "keyboard input device"] const Keyboard = 4u32 ; } }
        impl From<DeviceType> for u32 {
            fn from(value: DeviceType) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for DeviceType {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for DeviceType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            #[doc = "disabled state"]
            Disabled = 0u32,
            #[doc = "enabled state"]
            Enabled = 1u32,
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
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_input_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandInputManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_input_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This informs the compositor that the treeland input manager object"]
            #[doc = "will no longer be used."]
            #[doc = ""]
            #[doc = "Existing objects created through this interface are not destroyed"]
            #[doc = "automatically. Clients should destroy those objects before destroying"]
            #[doc = "the manager."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a treeland_mouse_settings_v1 object bound to the"]
            #[doc = "specified wl_seat."]
            #[doc = ""]
            #[doc = "The created object acts as a factory for"]
            #[doc = "treeland_pointer_device_configuration_v1 objects that manage"]
            #[doc = "mouse device settings on the target seat."]
            #[doc = ""]
            #[doc = "The seat must currently expose mouse capability. If the seat"]
            #[doc = "does not identify a valid mouse-capable target, the compositor"]
            #[doc = "raises the protocol error."]
            #[doc = ""]
            #[doc = "Note:"]
            #[doc = "1. The treeland_mouse_settings_v1 object must be destroyed"]
            #[doc = "before the associated wl_seat is destroyed."]
            fn get_mouse_settings(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a treeland_touchpad_settings_v1 object bound to the"]
            #[doc = "specified wl_seat."]
            #[doc = ""]
            #[doc = "The created object acts as a factory for"]
            #[doc = "treeland_pointer_device_configuration_v1 objects that manage"]
            #[doc = "touchpad device settings on the target seat."]
            #[doc = ""]
            #[doc = "The seat must currently expose touchpad capability. If the seat"]
            #[doc = "does not identify a valid touchpad-capable target, the"]
            #[doc = "compositor raises the protocol error."]
            #[doc = ""]
            #[doc = "Note:"]
            #[doc = "1. The treeland_touchpad_settings_v1 object must be destroyed"]
            #[doc = "before the associated wl_seat is destroyed."]
            fn get_touchpad_settings(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a treeland_keyboard_settings_v1 object bound to the"]
            #[doc = "specified wl_seat."]
            #[doc = ""]
            #[doc = "The seat must currently expose keyboard capability. If the seat"]
            #[doc = "does not identify a valid keyboard-capable target, the"]
            #[doc = "compositor raises the protocol error."]
            #[doc = ""]
            #[doc = "Note:"]
            #[doc = "1. The treeland_keyboard_settings_v1 object must be destroyed"]
            #[doc = "before the associated wl_seat is destroyed."]
            fn get_keyboard_settings(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted when the compositor detects that a seat"]
            #[doc = "now exposes the specified device capability."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_input_manager_v1 object is created to report"]
            #[doc = "capabilities currently available on a seat."]
            #[doc = ""]
            #[doc = "The compositor may also send this event later when input"]
            #[doc = "devices are hot-plugged and a seat gains the specified"]
            #[doc = "capability."]
            #[doc = ""]
            #[doc = "A client may create a treeland_mouse_settings_v1 object for a"]
            #[doc = "given seat only if the compositor has advertised device_type"]
            #[doc = "mouse capability for that seat. Likewise, a client may create a"]
            #[doc = "treeland_touchpad_settings_v1 or treeland_keyboard_settings_v1"]
            #[doc = "object for a given seat only if the compositor has advertised"]
            #[doc = "device_type touchpad or keyboard capability for that seat."]
            fn capability_available(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: DeviceType,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_input_manager_v1#{}.capability_available({}, {})",
                        sender_id,
                        r#type,
                        seat
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(r#type.into())
                        .put_object(Some(seat))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted when the compositor detects that a seat no"]
            #[doc = "longer exposes the specified device capability."]
            #[doc = ""]
            #[doc = "The compositor may also send this event later when input"]
            #[doc = "devices are hot-unplugged and a seat loses the specified"]
            #[doc = "capability."]
            #[doc = ""]
            #[doc = "If type includes mouse capability, the client must destroy any"]
            #[doc = "treeland_mouse_settings_v1 objects targeting the seat. This"]
            #[doc = "also destroys any treeland_pointer_device_configuration_v1"]
            #[doc = "objects created from those mouse settings objects."]
            #[doc = ""]
            #[doc = "If type includes touchpad capability, the client must destroy"]
            #[doc = "any treeland_touchpad_settings_v1 objects targeting the seat."]
            #[doc = "This also destroys any treeland_pointer_device_configuration_v1"]
            #[doc = "objects created from those touchpad settings objects."]
            #[doc = ""]
            #[doc = "If type includes keyboard capability, the client must destroy any"]
            #[doc = "treeland_keyboard_settings_v1 objects targeting the seat."]
            fn capability_unavailable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: DeviceType,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_input_manager_v1#{}.capability_unavailable({}, {})",
                        sender_id,
                        r#type,
                        seat
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(r#type.into())
                        .put_object(Some(seat))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_input_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_input_manager_v1#{}.get_mouse_settings({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_mouse_settings(connection, sender_id, id, seat)
                                .await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_input_manager_v1#{}.get_touchpad_settings({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_touchpad_settings(connection, sender_id, id, seat)
                                .await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_input_manager_v1#{}.get_keyboard_settings({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_keyboard_settings(connection, sender_id, id, seat)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_pointer_device_configuration_v1 interface represents"]
    #[doc = "pending configuration for pointer devices associated with the parent"]
    #[doc = "device settings object."]
    #[doc = ""]
    #[doc = "This object is created via get_pointer_configuration on a"]
    #[doc = "treeland_mouse_settings_v1 or treeland_touchpad_settings_v1 object."]
    #[doc = "It collects configuration that applies to the associated device"]
    #[doc = "type."]
    #[doc = ""]
    #[doc = "Requests on this object update pending state. The compositor"]
    #[doc = "applies the accumulated settings when the client sends apply."]
    #[doc = "Destroying the object before apply discards any pending state that"]
    #[doc = "has not yet been applied."]
    #[doc = ""]
    #[doc = "The compositor sends configuration events after object creation to"]
    #[doc = "report the current effective settings. It may later send additional"]
    #[doc = "state batches when those effective settings change for"]
    #[doc = "compositor-defined reasons. A done event terminates each event"]
    #[doc = "batch."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_pointer_device_configuration_v1 {
        bitflags::bitflags! { # [doc = "Optional features supported by pointer devices. The compositor"] # [doc = "sends a feature event to indicate which features are supported"] # [doc = "by at least one device associated with the parent settings"] # [doc = "object. A client must only send a set request if the"] # [doc = "corresponding feature has been advertised. It is a protocol error"] # [doc = "to send a set request for a feature that has not been"] # [doc = "advertised."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Feature : u32 { # [doc = "device supports configuring scroll factor"] const ScrollFactor = 1u32 ; # [doc = "device supports configuring handed mode"] const HandedMode = 2u32 ; # [doc = "device supports configuring acceleration speed"] const AccelSpeed = 4u32 ; # [doc = "device supports configuring acceleration profile"] const AccelerationProfile = 8u32 ; # [doc = "device supports configuring event delivery mode"] const SendEventsMode = 16u32 ; # [doc = "device supports configuring natural scrolling"] const NaturalScroll = 32u32 ; # [doc = "device supports disable-while-typing"] const DisableWhileTyping = 64u32 ; # [doc = "device supports tap-to-click"] const TapToClick = 128u32 ; } }
        impl From<Feature> for u32 {
            fn from(value: Feature) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Feature {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Feature {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum HandedMode {
            #[doc = "right-handed mode"]
            Right = 0u32,
            #[doc = "left-handed mode"]
            Left = 1u32,
        }
        impl From<HandedMode> for u32 {
            fn from(value: HandedMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for HandedMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Right),
                    1u32 => Ok(Self::Left),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for HandedMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Pointer acceleration profiles. The semantics correspond to"]
        #[doc = "libinput acceleration profiles."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AccelerationProfile {
            #[doc = "device does not have a configurable acceleration profile"]
            None = 0u32,
            #[doc = "use a flat acceleration profile"]
            Flat = 1u32,
            #[doc = "use an adaptive acceleration profile"]
            Adaptive = 2u32,
            #[doc = "use a custom acceleration profile"]
            Custom = 4u32,
        }
        impl From<AccelerationProfile> for u32 {
            fn from(value: AccelerationProfile) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for AccelerationProfile {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Flat),
                    2u32 => Ok(Self::Adaptive),
                    4u32 => Ok(Self::Custom),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AccelerationProfile {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Modes controlling whether the target pointer device should send"]
        #[doc = "events and whether those events depend on the presence of an"]
        #[doc = "external pointer device on the same seat."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum SendEventsMode {
            #[doc = "send events from this device normally"]
            Enabled = 0u32,
            #[doc = "do not send events from this device"]
            Disabled = 1u32,
            #[doc = "if an external pointer device is plugged in, do not send events from this device; may be available on built-in pointer devices"]
            DisabledOnExternalMouse = 2u32,
        }
        impl From<SendEventsMode> for u32 {
            fn from(value: SendEventsMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for SendEventsMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Enabled),
                    1u32 => Ok(Self::Disabled),
                    2u32 => Ok(Self::DisabledOnExternalMouse),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for SendEventsMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_pointer_device_configuration_v1 interface. See the module level documentation for more info"]
        pub trait TreelandPointerDeviceConfigurationV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_pointer_device_configuration_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this treeland_pointer_device_configuration_v1 object."]
            #[doc = ""]
            #[doc = "Destroying the object discards any pending state that has not"]
            #[doc = "yet been applied."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set a scaling factor applied to scroll axis delta generated"]
            #[doc = "by wheel and other continuous scrolling input sources. This"]
            #[doc = "request requires the scroll_factor feature to be advertised via"]
            #[doc = "the feature event."]
            #[doc = ""]
            #[doc = "The compositor multiplies scroll axis values by the specified"]
            #[doc = "factor before forwarding them to clients. This affects"]
            #[doc = "continuous scroll delta only and does not modify discrete"]
            #[doc = "scroll step events."]
            #[doc = ""]
            #[doc = "This request updates only the pending state associated with"]
            #[doc = "this object. The change takes effect after the next apply"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Sending this request or the corresponding apply request does"]
            #[doc = "not by itself cause a scroll_factor event to be emitted."]
            #[doc = ""]
            #[doc = "delta = factor * _delta"]
            fn set_scroll_factor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                factor: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set whether primary and secondary buttons use left-handed or"]
            #[doc = "right-handed ordering. This request requires the handed_mode"]
            #[doc = "feature to be advertised via the feature event. The change"]
            #[doc = "takes effect on the next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a handed_mode event to be emitted."]
            fn set_handed_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: HandedMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the pointer acceleration speed. This request requires the"]
            #[doc = "accel_speed feature to be advertised via the feature event. The"]
            #[doc = "change takes effect on the next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause an accel_speed event to be emitted."]
            fn set_accel_speed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                accel_speed: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the acceleration profile. This request requires the"]
            #[doc = "acceleration_profile feature to be advertised via the feature"]
            #[doc = "event. The change takes effect on the next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause an acceleration_profile event to be"]
            #[doc = "emitted."]
            fn set_acceleration_profile(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                profile : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_pointer_device_configuration_v1 :: AccelerationProfile,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set how events from the target pointer device are delivered."]
            #[doc = "This request requires the send_events_mode feature to be"]
            #[doc = "advertised via the feature event. The change takes effect on the"]
            #[doc = "next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a send_events_mode event to be emitted."]
            fn set_send_events_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_pointer_device_configuration_v1 :: SendEventsMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Enable or disable natural scrolling. This request requires the"]
            #[doc = "natural_scroll feature to be advertised via the feature event."]
            #[doc = "The change takes effect on the next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a natural_scroll event to be emitted."]
            fn set_natural_scroll(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_input_manager_v1 :: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Enable or disable temporarily disabling pointer devices while"]
            #[doc = "the user is typing. This request requires the"]
            #[doc = "disable_while_typing feature to be advertised via the feature"]
            #[doc = "event. The change takes effect on the next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a disable_while_typing event to be"]
            #[doc = "emitted."]
            fn set_disable_while_typing(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_input_manager_v1 :: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Enable or disable tap-to-click. This request requires the"]
            #[doc = "tap_to_click feature to be advertised via the feature event."]
            #[doc = "The change takes effect on the next apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a tap_to_click event to be emitted."]
            fn set_tap_to_click(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_input_manager_v1 :: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Ask the compositor to apply the pending pointer device"]
            #[doc = "configuration carried by this object."]
            fn apply(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event reports which optional features are supported by at"]
            #[doc = "least one device associated with the parent settings object."]
            #[doc = ""]
            #[doc = "The compositor sends this event as part of the initial state"]
            #[doc = "batch after the treeland_pointer_device_configuration_v1 object"]
            #[doc = "is created."]
            #[doc = ""]
            #[doc = "The compositor may also send this event again when devices are"]
            #[doc = "added to or removed from the target seat and the set of"]
            #[doc = "supported features changes."]
            fn feature(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                feature: Feature,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.feature({})",
                        sender_id,
                        feature
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(feature.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports the current scroll factor used for wheel and"]
            #[doc = "similar scroll input."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_pointer_device_configuration_v1 object is created as"]
            #[doc = "part of the initial state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_scroll_factor do not cause this"]
            #[doc = "event to be emitted automatically."]
            fn scroll_factor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                factor: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.scroll_factor({})",
                        sender_id,
                        factor
                    );
                    let payload = waynest::PayloadBuilder::new().put_fixed(factor).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports whether left-handed or right-handed button"]
            #[doc = "ordering is currently used."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_pointer_device_configuration_v1 object is created as"]
            #[doc = "part of the initial state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_handed_mode do not cause this"]
            #[doc = "event to be emitted automatically."]
            fn handed_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: HandedMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.handed_mode({})",
                        sender_id,
                        mode
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(mode.into()).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports the current pointer acceleration speed."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_pointer_device_configuration_v1 object is created as"]
            #[doc = "part of the initial state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_accel_speed do not cause this"]
            #[doc = "event to be emitted automatically."]
            fn accel_speed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                accel_speed: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.accel_speed({})",
                        sender_id,
                        accel_speed
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_fixed(accel_speed)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports the current acceleration profile."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_pointer_device_configuration_v1 object is created as"]
            #[doc = "part of the initial state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_acceleration_profile do not cause"]
            #[doc = "this event to be emitted automatically."]
            fn acceleration_profile(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                profile : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_pointer_device_configuration_v1 :: AccelerationProfile,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.acceleration_profile({})",
                        sender_id,
                        profile
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(profile.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports the current event delivery mode used for"]
            #[doc = "the target pointer device."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_pointer_device_configuration_v1 object is created as"]
            #[doc = "part of the initial state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_send_events_mode do not cause"]
            #[doc = "this event to be emitted automatically."]
            fn send_events_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_pointer_device_configuration_v1 :: SendEventsMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.send_events_mode({})",
                        sender_id,
                        mode
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(mode.into()).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports whether natural scrolling is enabled."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_pointer_device_configuration_v1 object is created as"]
            #[doc = "part of the initial state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_natural_scroll do not cause this"]
            #[doc = "event to be emitted automatically."]
            fn natural_scroll(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_input_manager_v1 :: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.natural_scroll({})",
                        sender_id,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports whether pointer devices are temporarily"]
            #[doc = "disabled while the user is typing."]
            #[doc = ""]
            #[doc = "The compositor sends this event as part of the initial state"]
            #[doc = "batch if the disable_while_typing feature has been advertised."]
            #[doc = ""]
            #[doc = "Changes requested through set_disable_while_typing do not cause"]
            #[doc = "this event to be emitted automatically."]
            fn disable_while_typing(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_input_manager_v1 :: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.disable_while_typing({})",
                        sender_id,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports whether tap-to-click is enabled."]
            #[doc = ""]
            #[doc = "The compositor sends this event as part of the initial state"]
            #[doc = "batch if the tap_to_click feature has been advertised."]
            #[doc = ""]
            #[doc = "Changes requested through set_tap_to_click do not cause this"]
            #[doc = "event to be emitted automatically."]
            fn tap_to_click(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_input_manager_v1 :: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.tap_to_click({})",
                        sender_id,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the compositor failed to apply one or"]
            #[doc = "more pending pointer device configuration changes carried by"]
            #[doc = "this object."]
            #[doc = ""]
            #[doc = "This event does not destroy the object. The client may continue"]
            #[doc = "using the object and send further requests."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.failed()",
                        sender_id,
                    );
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent once when all pointer device configuration"]
            #[doc = "state events in the current batch have been sent."]
            #[doc = ""]
            #[doc = "The compositor must always end a batch of pointer device"]
            #[doc = "configuration state events with this event, regardless of"]
            #[doc = "whether it sends the initial state or a later update."]
            #[doc = ""]
            #[doc = "The serial is used in a future get_pointer_configuration"]
            #[doc = "request to ensure the client's configuration is based on"]
            #[doc = "up-to-date state."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_pointer_device_configuration_v1#{}.done({})",
                        sender_id,
                        serial
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(serial).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_pointer_device_configuration_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let factor = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_scroll_factor({})",
                                sender_id,
                                factor
                            );
                            self.set_scroll_factor(connection, sender_id, factor).await
                        }
                        2u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_handed_mode({})",
                                sender_id,
                                mode
                            );
                            self.set_handed_mode(connection, sender_id, mode.try_into()?)
                                .await
                        }
                        3u16 => {
                            let accel_speed = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_accel_speed({})",
                                sender_id,
                                accel_speed
                            );
                            self.set_accel_speed(connection, sender_id, accel_speed)
                                .await
                        }
                        4u16 => {
                            let profile = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_acceleration_profile({})",
                                sender_id,
                                profile
                            );
                            self.set_acceleration_profile(
                                connection,
                                sender_id,
                                profile.try_into()?,
                            )
                            .await
                        }
                        5u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_send_events_mode({})",
                                sender_id,
                                mode
                            );
                            self.set_send_events_mode(connection, sender_id, mode.try_into()?)
                                .await
                        }
                        6u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_natural_scroll({})",
                                sender_id,
                                state
                            );
                            self.set_natural_scroll(connection, sender_id, state.try_into()?)
                                .await
                        }
                        7u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_disable_while_typing({})",
                                sender_id,
                                state
                            );
                            self.set_disable_while_typing(connection, sender_id, state.try_into()?)
                                .await
                        }
                        8u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.set_tap_to_click({})",
                                sender_id,
                                state
                            );
                            self.set_tap_to_click(connection, sender_id, state.try_into()?)
                                .await
                        }
                        9u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_pointer_device_configuration_v1#{}.apply()",
                                sender_id,
                            );
                            self.apply(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_mouse_settings_v1 interface acts as a factory for"]
    #[doc = "treeland_pointer_device_configuration_v1 objects that manage mouse"]
    #[doc = "device settings on the target seat."]
    #[doc = ""]
    #[doc = "A treeland_pointer_device_configuration_v1 object is created via"]
    #[doc = "get_pointer_configuration on this object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_mouse_settings_v1 {
        #[doc = "Trait to implement the treeland_mouse_settings_v1 interface. See the module level documentation for more info"]
        pub trait TreelandMouseSettingsV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_mouse_settings_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this treeland_mouse_settings_v1 object."]
            #[doc = ""]
            #[doc = "Existing treeland_pointer_device_configuration_v1 objects"]
            #[doc = "created from this object are also destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a treeland_pointer_device_configuration_v1 object for"]
            #[doc = "this mouse settings scope."]
            #[doc = ""]
            #[doc = "The created object manages mouse device configuration on the"]
            #[doc = "target seat. The serial argument must match the serial from the"]
            #[doc = "most recent done event sent on the previous"]
            #[doc = "treeland_pointer_device_configuration_v1 object for this"]
            #[doc = "treeland_mouse_settings_v1 object. For the first creation, the"]
            #[doc = "serial should be 0."]
            #[doc = ""]
            #[doc = "Only one treeland_pointer_device_configuration_v1 object may"]
            #[doc = "exist for this treeland_mouse_settings_v1 object at a time."]
            #[doc = "Creating a second one without destroying the first is a"]
            #[doc = "protocol error."]
            #[doc = ""]
            #[doc = "If the serial does not match the current state, the compositor"]
            #[doc = "may immediately send a failed event on the new"]
            #[doc = "treeland_pointer_device_configuration_v1 object."]
            fn get_pointer_configuration(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            tracing::debug!("treeland_mouse_settings_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_mouse_settings_v1#{}.get_pointer_configuration({}, {})",
                                sender_id,
                                id,
                                serial
                            );
                            self.get_pointer_configuration(connection, sender_id, id, serial)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_touchpad_settings_v1 interface acts as a factory for"]
    #[doc = "treeland_pointer_device_configuration_v1 objects that manage"]
    #[doc = "touchpad device settings on the target seat."]
    #[doc = ""]
    #[doc = "A treeland_pointer_device_configuration_v1 object is created via"]
    #[doc = "get_pointer_configuration on this object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_touchpad_settings_v1 {
        #[doc = "Trait to implement the treeland_touchpad_settings_v1 interface. See the module level documentation for more info"]
        pub trait TreelandTouchpadSettingsV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_touchpad_settings_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this treeland_touchpad_settings_v1 object."]
            #[doc = ""]
            #[doc = "Existing treeland_pointer_device_configuration_v1 objects"]
            #[doc = "created from this object are also destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a treeland_pointer_device_configuration_v1 object for"]
            #[doc = "this touchpad settings scope."]
            #[doc = ""]
            #[doc = "The created object manages touchpad device configuration on the"]
            #[doc = "target seat. The serial argument must match the serial from the"]
            #[doc = "most recent done event sent on the previous"]
            #[doc = "treeland_pointer_device_configuration_v1 object for this"]
            #[doc = "treeland_touchpad_settings_v1 object. For the first creation,"]
            #[doc = "the serial should be 0."]
            #[doc = ""]
            #[doc = "Only one treeland_pointer_device_configuration_v1 object may"]
            #[doc = "exist for this treeland_touchpad_settings_v1 object at a time."]
            #[doc = "Creating a second one without destroying the first is a"]
            #[doc = "protocol error."]
            #[doc = ""]
            #[doc = "If the serial does not match the current state, the compositor"]
            #[doc = "may immediately send a failed event on the new"]
            #[doc = "treeland_pointer_device_configuration_v1 object."]
            fn get_pointer_configuration(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_touchpad_settings_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_touchpad_settings_v1#{}.get_pointer_configuration({}, {})",
                                sender_id,
                                id,
                                serial
                            );
                            self.get_pointer_configuration(connection, sender_id, id, serial)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_keyboard_settings_v1 interface represents pending"]
    #[doc = "settings for all keyboard associated with the target seat."]
    #[doc = ""]
    #[doc = "Requests on this object update pending keyboard state. The"]
    #[doc = "compositor applies the accumulated settings when the client sends"]
    #[doc = "apply."]
    #[doc = ""]
    #[doc = "The compositor sends keyboard-specific state events after object"]
    #[doc = "creation to report the current effective settings for the target"]
    #[doc = "seat. It may later send additional state batches when those"]
    #[doc = "effective settings change for compositor-defined reasons. A done"]
    #[doc = "event terminates each event batch."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_keyboard_settings_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ToggleState {
            #[doc = "off state"]
            Off = 0u32,
            #[doc = "on state"]
            On = 1u32,
        }
        impl From<ToggleState> for u32 {
            fn from(value: ToggleState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ToggleState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Off),
                    1u32 => Ok(Self::On),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ToggleState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Optional features supported by keyboard devices. The compositor"] # [doc = "sends a feature event to indicate which features are supported"] # [doc = "by at least one keyboard device associated with the target seat."] # [doc = "A client must only send a set request if the corresponding"] # [doc = "feature has been advertised. It is a protocol error to send a"] # [doc = "set request for a feature that has not been advertised."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Feature : u32 { # [doc = "device supports configuring num lock state"] const NumLock = 1u32 ; } }
        impl From<Feature> for u32 {
            fn from(value: Feature) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Feature {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Feature {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_keyboard_settings_v1 interface. See the module level documentation for more info"]
        pub trait TreelandKeyboardSettingsV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_keyboard_settings_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this treeland_keyboard_settings_v1 object."]
            #[doc = ""]
            #[doc = "Destroying the object discards any pending state that has not"]
            #[doc = "yet been applied."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the keyboard repeat rate and delay to apply on the next"]
            #[doc = "apply request."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a repeat event to be emitted."]
            fn set_repeat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rate: i32,
                delay: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the num lock state to apply on the next apply request."]
            #[doc = "This request requires the num_lock feature to be advertised"]
            #[doc = "via the feature event."]
            #[doc = ""]
            #[doc = "This request updates only the pending state tracked by this"]
            #[doc = "object. Sending this request, or the following apply request,"]
            #[doc = "does not by itself cause a num_lock event to be emitted."]
            fn set_num_lock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_keyboard_settings_v1 :: ToggleState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Ask the compositor to apply the pending keyboard settings"]
            #[doc = "carried by this object."]
            fn apply(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event reports which optional features are supported by at"]
            #[doc = "least one keyboard device associated with the target seat."]
            #[doc = ""]
            #[doc = "The compositor sends this event as part of the initial state"]
            #[doc = "batch after the treeland_keyboard_settings_v1 object is created."]
            #[doc = ""]
            #[doc = "The compositor may also send this event again when keyboard"]
            #[doc = "devices are added to or removed from the target seat and the"]
            #[doc = "set of supported features changes."]
            fn feature(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                feature: Feature,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_keyboard_settings_v1#{}.feature({})",
                        sender_id,
                        feature
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(feature.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports the current keyboard repeat configuration."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_keyboard_settings_v1 object is created as part of the"]
            #[doc = "initial keyboard state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_repeat do not cause this event to"]
            #[doc = "be emitted automatically."]
            fn repeat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rate: i32,
                delay: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_keyboard_settings_v1#{}.repeat({}, {})",
                        sender_id,
                        rate,
                        delay
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_int(rate)
                        .put_int(delay)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event reports the current num lock state."]
            #[doc = ""]
            #[doc = "The compositor sends this event immediately after the"]
            #[doc = "treeland_keyboard_settings_v1 object is created as part of the"]
            #[doc = "initial keyboard state batch."]
            #[doc = ""]
            #[doc = "Changes requested through set_num_lock do not cause this event"]
            #[doc = "to be emitted automatically."]
            fn num_lock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state : super :: super :: super :: treeland :: treeland_input_manager_unstable_v1 :: treeland_keyboard_settings_v1 :: ToggleState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_keyboard_settings_v1#{}.num_lock({})",
                        sender_id,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the compositor failed to apply one or"]
            #[doc = "more pending keyboard settings carried by this object."]
            #[doc = ""]
            #[doc = "This event does not destroy the object. The client may continue"]
            #[doc = "using the object and send further requests."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_keyboard_settings_v1#{}.failed()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent once when all keyboard state events in the"]
            #[doc = "current batch have been sent."]
            #[doc = ""]
            #[doc = "The compositor must always end a batch of keyboard state events"]
            #[doc = "with this event, regardless of whether it sends the initial"]
            #[doc = "state or a later update."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_keyboard_settings_v1#{}.done()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_keyboard_settings_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let rate = message.int()?;
                            let delay = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_keyboard_settings_v1#{}.set_repeat({}, {})",
                                sender_id,
                                rate,
                                delay
                            );
                            self.set_repeat(connection, sender_id, rate, delay).await
                        }
                        2u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_keyboard_settings_v1#{}.set_num_lock({})",
                                sender_id,
                                state
                            );
                            self.set_num_lock(connection, sender_id, state.try_into()?)
                                .await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_keyboard_settings_v1#{}.apply()", sender_id,);
                            self.apply(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows privileged clients to subscribe to global keyboard"]
#[doc = "state changes (e.g. Caps Lock, Num Lock)."]
#[doc = ""]
#[doc = "Clients create watcher objects through the manager global, each bound"]
#[doc = "to a seat. The client then configures the watcher with set_modifiers to"]
#[doc = "choose which modifiers to monitor, and set_flags to choose which event"]
#[doc = "types (locked, unlocked, or both) to receive. Configuration changes are"]
#[doc = "double-buffered and take effect atomically when the client sends apply."]
#[doc = "Both set_modifiers and set_flags can be called multiple times before"]
#[doc = "apply; only the values from the most recent calls are committed. This"]
#[doc = "allows changing the configuration at runtime without destroying and"]
#[doc = "recreating the watcher. After apply, the compositor sends a"]
#[doc = "current_state event for each watched modifier containing its"]
#[doc = "current lock state, so the client can synchronize its initial"]
#[doc = "state without waiting for the next physical key press. The"]
#[doc = "compositor then sends"]
#[doc = "state_changed events on the watcher whenever any of the watched"]
#[doc = "modifiers changes lock state globally, regardless of which surface"]
#[doc = "has keyboard focus."]
#[doc = ""]
#[doc = "This protocol is observation-only. Keyboard state changes are never"]
#[doc = "consumed or intercepted by this protocol."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[allow(clippy::module_inception)]
pub mod treeland_keyboard_state_notify_unstable_v1 {
    #[doc = "The treeland_keyboard_state_notify_manager_v1 interface is a"]
    #[doc = "singleton global object used to create watcher objects for monitoring"]
    #[doc = "keyboard state changes on a given seat."]
    #[doc = ""]
    #[doc = "A client may create multiple watcher objects for the same seat if"]
    #[doc = "needed."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_keyboard_state_notify_manager_v1 {
        #[doc = "Trait to implement the treeland_keyboard_state_notify_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandKeyboardStateNotifyManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_keyboard_state_notify_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this"]
            #[doc = "treeland_keyboard_state_notify_manager_v1 object."]
            #[doc = ""]
            #[doc = "Existing watcher objects created through this interface are not"]
            #[doc = "destroyed automatically. Clients should destroy those objects"]
            #[doc = "before destroying the manager."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a treeland_keyboard_state_watcher_v1 object for"]
            #[doc = "monitoring keyboard state changes on the given seat, or on"]
            #[doc = "all seats if seat is null."]
            #[doc = ""]
            #[doc = "The watcher is created with no modifiers and no flags set by"]
            #[doc = "default. The client should use set_modifiers and set_flags on"]
            #[doc = "the watcher to configure what to monitor."]
            #[doc = ""]
            #[doc = "Note:"]
            #[doc = "1. The treeland_keyboard_state_watcher_v1 object must be destroyed"]
            #[doc = "before the associated wl_seat is destroyed."]
            fn get_keyboard_state_watcher(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_keyboard_state_notify_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_keyboard_state_notify_manager_v1#{}.get_keyboard_state_watcher({}, {})",
                                sender_id,
                                id,
                                seat.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.get_keyboard_state_watcher(connection, sender_id, id, seat)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The treeland_keyboard_state_watcher_v1 interface represents a"]
    #[doc = "subscription to keyboard state changes on a specific"]
    #[doc = "seat."]
    #[doc = ""]
    #[doc = "This object is created via get_keyboard_state_watcher on the"]
    #[doc = "treeland_keyboard_state_notify_manager_v1 global. The client"]
    #[doc = "configures the watcher with set_modifiers to choose which modifiers"]
    #[doc = "to monitor and set_flags to choose which state transitions to"]
    #[doc = "receive. Both can be changed at runtime. Configuration changes are"]
    #[doc = "double-buffered: set_modifiers and set_flags only update the pending"]
    #[doc = "state, which is applied atomically when apply is called."]
    #[doc = ""]
    #[doc = "The compositor sends a current_state event for each modifier in"]
    #[doc = "the committed modifier set immediately after apply is called."]
    #[doc = "It then sends"]
    #[doc = "state_changed events whenever any of the watched modifiers"]
    #[doc = "changes lock state, according to the committed modifiers and"]
    #[doc = "flags."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_keyboard_state_watcher_v1 {
        bitflags::bitflags! { # [doc = "Bitfield of keyboard modifiers whose lock state can be"] # [doc = "monitored. Multiple modifier values can be combined using"] # [doc = "bitwise OR."] # [doc = ""] # [doc = "caps_lock: the Caps Lock modifier."] # [doc = "num_lock: the Num Lock modifier."] # [doc = "scroll_lock: the Scroll Lock modifier."] # [doc = ""] # [doc = "Examples:"] # [doc = "caps_lock: monitor only Caps Lock."] # [doc = "caps_lock | num_lock: monitor both Caps Lock and Num Lock."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Modifier : u32 { # [doc = "Caps Lock modifier"] const CapsLock = 1u32 ; # [doc = "Num Lock modifier"] const NumLock = 2u32 ; } }
        impl From<Modifier> for u32 {
            fn from(value: Modifier) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Modifier {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Modifier {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Flags to specify which modifier lock state transitions the"] # [doc = "client wants to be notified about."] # [doc = ""] # [doc = "none: do not notify on any state transition."] # [doc = "locked: notify when a watched modifier becomes locked."] # [doc = "unlocked: notify when a watched modifier becomes unlocked."] # [doc = ""] # [doc = "Examples:"] # [doc = "locked: the client receives events only when a modifier is"] # [doc = "locked."] # [doc = "unlocked: the client receives events only when a modifier is"] # [doc = "unlocked."] # [doc = "locked | unlocked: the client receives events on both."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct WatchFlag : u32 { # [doc = "notify when modifier is locked"] const Locked = 1u32 ; # [doc = "notify when modifier is unlocked"] const Unlocked = 2u32 ; } }
        impl From<WatchFlag> for u32 {
            fn from(value: WatchFlag) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for WatchFlag {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for WatchFlag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "The current lock state of a keyboard modifier."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ModifierState {
            #[doc = "modifier is now unlocked"]
            Unlocked = 0u32,
            #[doc = "modifier is now locked"]
            Locked = 1u32,
        }
        impl From<ModifierState> for u32 {
            fn from(value: ModifierState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ModifierState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unlocked),
                    1u32 => Ok(Self::Locked),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ModifierState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_keyboard_state_watcher_v1 interface. See the module level documentation for more info"]
        pub trait TreelandKeyboardStateWatcherV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_keyboard_state_watcher_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this treeland_keyboard_state_watcher_v1 object and"]
            #[doc = "stop receiving keyboard state notifications."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the keyboard modifiers to watch on this watcher."]
            #[doc = ""]
            #[doc = "The modifiers argument is a bitfield of values from the modifier"]
            #[doc = "enum (e.g. caps_lock = 1, num_lock = 2, scroll_lock = 4)."]
            #[doc = "Multiple modifiers can be combined using bitwise OR."]
            #[doc = ""]
            #[doc = "Calling set_modifiers replaces any previously pending modifiers."]
            #[doc = ""]
            #[doc = "This request only updates the pending configuration. The change"]
            #[doc = "takes effect when apply is called. If set_modifiers is called"]
            #[doc = "multiple times before apply, only the values from the most"]
            #[doc = "recent call are used."]
            fn set_modifiers(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                modifiers: Modifier,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Change the watch flags for this watcher, selecting which modifier"]
            #[doc = "state transitions (locked, unlocked, or both) the client wants"]
            #[doc = "to be notified about."]
            #[doc = ""]
            #[doc = "This request only updates the pending configuration. The change"]
            #[doc = "takes effect when apply is called. If set_flags is called"]
            #[doc = "multiple times before apply, only the values from the most"]
            #[doc = "recent call are used."]
            fn set_flags(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: WatchFlag,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Apply the pending configuration state for this watcher"]
            #[doc = "atomically."]
            #[doc = ""]
            #[doc = "The compositor will use the modifiers and flags set by the most"]
            #[doc = "recent set_modifiers and set_flags calls as the new active"]
            #[doc = "configuration for this watcher. If set_modifiers or set_flags"]
            #[doc = "have not been called since the last apply (or since the watcher"]
            #[doc = "was created), the corresponding pending values remain unchanged."]
            #[doc = ""]
            #[doc = "After this request, the pending configuration is considered"]
            #[doc = "consumed. Subsequent calls to set_modifiers or set_flags will"]
            #[doc = "begin a new pending configuration."]
            #[doc = ""]
            #[doc = "The compositor will send a current_state event for each"]
            #[doc = "modifier in the newly committed modifier set immediately"]
            #[doc = "after processing this request."]
            fn apply(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sent for each modifier in the committed modifier set after"]
            #[doc = "each apply call to provide its current lock state. For"]
            #[doc = "example, if the client watches caps_lock and num_lock, the"]
            #[doc = "compositor will send two current_state events, one for each"]
            #[doc = "modifier."]
            #[doc = ""]
            #[doc = "This event is emitted before any subsequent state_changed"]
            #[doc = "events for the new configuration. It may be sent again"]
            #[doc = "after future apply calls if the client reconfigures the"]
            #[doc = "watcher."]
            fn current_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                modifier: Modifier,
                state: ModifierState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_keyboard_state_watcher_v1#{}.current_state({}, {})",
                        sender_id,
                        modifier,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(modifier.into())
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent when one of the watched modifiers changes"]
            #[doc = "lock state globally, according to the modifiers and flags"]
            #[doc = "currently set on this watcher."]
            #[doc = ""]
            #[doc = "The modifier argument identifies which modifier changed state."]
            #[doc = "The state argument indicates whether the modifier is now locked"]
            #[doc = "or unlocked."]
            fn state_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                modifier: Modifier,
                state: ModifierState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_keyboard_state_watcher_v1#{}.state_changed({}, {})",
                        sender_id,
                        modifier,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(modifier.into())
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_keyboard_state_watcher_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let modifiers = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_keyboard_state_watcher_v1#{}.set_modifiers({})",
                                sender_id,
                                modifiers
                            );
                            self.set_modifiers(connection, sender_id, modifiers.try_into()?)
                                .await
                        }
                        2u16 => {
                            let flags = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_keyboard_state_watcher_v1#{}.set_flags({})",
                                sender_id,
                                flags
                            );
                            self.set_flags(connection, sender_id, flags.try_into()?)
                                .await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_keyboard_state_watcher_v1#{}.apply()",
                                sender_id,
                            );
                            self.apply(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_color_control(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Specifies which output is the primary one identified by their name."]
            fn primary_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_manager_v1#{}.primary_output(\"{}\")",
                        sender_id,
                        output_name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(output_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_manager_v1#{}.set_primary_output(\"{}\")",
                                sender_id,
                                output
                            );
                            self.set_primary_output(connection, sender_id, output).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_manager_v1#{}.get_color_control({}, {})",
                                sender_id,
                                id,
                                output
                            );
                            self.get_color_control(connection, sender_id, id, output)
                                .await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_output_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Brightness settings are applied only after a commit request is made."]
            #[doc = "Setting a value outside the range [0.0, 100.0] is a protocol error."]
            fn set_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                brightness: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn result(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                success: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_color_control_v1#{}.result({})",
                        sender_id,
                        success
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(success).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_color_control_v1#{}.color_temperature({})",
                        sender_id,
                        temperature
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(temperature).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Provides the current brightness setting of the output."]
            #[doc = "Brightness is valued in the range [0.0, 100.0]."]
            #[doc = "This event is sent once after the treeland_output_color_control_v1 object is created,"]
            #[doc = "or right after when a brightness change for the output is successfully commited."]
            fn brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                brightness: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_output_color_control_v1#{}.brightness({})",
                        sender_id,
                        brightness
                    );
                    let payload = waynest::PayloadBuilder::new().put_fixed(brightness).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            let temperature = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.set_color_temperature({})",
                                sender_id,
                                temperature
                            );
                            self.set_color_temperature(connection, sender_id, temperature)
                                .await
                        }
                        1u16 => {
                            let brightness = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.set_brightness({})",
                                sender_id,
                                brightness
                            );
                            self.set_brightness(connection, sender_id, brightness).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.commit()",
                                sender_id,
                            );
                            self.commit(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_output_color_control_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            const VERSION: u32 = 2u32;
            #[doc = "set window background, shadow based on context"]
            fn get_window_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "custom user cursor"]
            fn get_cursor_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "custom treeland and window global font context"]
            fn get_font_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "custom user treeland window appearance global"]
            fn get_appearance_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_manager_v1#{}.get_window_context({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_window_context(connection, sender_id, id, surface)
                                .await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_manager_v1#{}.get_cursor_context({})",
                                sender_id,
                                id
                            );
                            self.get_cursor_context(connection, sender_id, id).await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_manager_v1#{}.get_font_context({})",
                                sender_id,
                                id
                            );
                            self.get_font_context(connection, sender_id, id).await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_manager_v1#{}.get_appearance_context({})",
                                sender_id,
                                id
                            );
                            self.get_appearance_context(connection, sender_id, id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "if only one commit fails validation, the commit will fail"]
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after commit cursor configure."]
            fn verfity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                success: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.verfity({})",
                        sender_id,
                        success
                    );
                    let payload = waynest::PayloadBuilder::new().put_int(success).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after system cursor theme changed."]
            fn theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.theme(\"{}\")",
                        sender_id,
                        name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after system cursor size changed."]
            fn size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_cursor_context_v1#{}.size({})",
                        sender_id,
                        size
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(size).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.set_theme(\"{}\")",
                                sender_id,
                                name
                            );
                            self.set_theme(connection, sender_id, name).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.get_theme()",
                                sender_id,
                            );
                            self.get_theme(connection, sender_id).await
                        }
                        2u16 => {
                            let size = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.set_size({})",
                                sender_id,
                                size
                            );
                            self.set_size(connection, sender_id, size).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.get_size()",
                                sender_id,
                            );
                            self.get_size(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.commit()",
                                sender_id,
                            );
                            self.commit(connection, sender_id).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_cursor_context_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set if system titlebar is enabled"]
            fn set_titlebar(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: EnableMode,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_window_context_v1#{}.set_blend_mode({})",
                                sender_id,
                                mode
                            );
                            self.set_blend_mode(connection, sender_id, mode.try_into()?)
                                .await
                        }
                        1u16 => {
                            let radius = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_window_context_v1#{}.set_round_corner_radius({})",
                                sender_id,
                                radius
                            );
                            self.set_round_corner_radius(connection, sender_id, radius)
                                .await
                        }
                        2u16 => {
                            let radius = message.int()?;
                            let offset_x = message.int()?;
                            let offset_y = message.int()?;
                            let r = message.int()?;
                            let g = message.int()?;
                            let b = message.int()?;
                            let a = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_window_context_v1#{}.set_shadow({}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                radius,
                                offset_x,
                                offset_y,
                                r,
                                g,
                                b,
                                a
                            );
                            self.set_shadow(
                                connection, sender_id, radius, offset_x, offset_y, r, g, b, a,
                            )
                            .await
                        }
                        3u16 => {
                            let width = message.int()?;
                            let r = message.int()?;
                            let g = message.int()?;
                            let b = message.int()?;
                            let a = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_window_context_v1#{}.set_border({}, {}, {}, {}, {})",
                                sender_id,
                                width,
                                r,
                                g,
                                b,
                                a
                            );
                            self.set_border(connection, sender_id, width, r, g, b, a)
                                .await
                        }
                        4u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_window_context_v1#{}.set_titlebar({})",
                                sender_id,
                                mode
                            );
                            self.set_titlebar(connection, sender_id, mode.try_into()?)
                                .await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_window_context_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the system font size"]
            fn get_font_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the system font"]
            fn set_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the system font"]
            fn get_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the system monospace font"]
            fn set_monospace_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the system monospace font"]
            fn get_monospace_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the system font."]
            fn font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.font(\"{}\")",
                        sender_id,
                        font_name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(font_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system monospace font."]
            fn monospace_font(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.monospace_font(\"{}\")",
                        sender_id,
                        font_name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(font_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system font size."]
            fn font_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                font_size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_font_context_v1#{}.font_size({})",
                        sender_id,
                        font_size
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(font_size).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            let size = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.set_font_size({})",
                                sender_id,
                                size
                            );
                            self.set_font_size(connection, sender_id, size).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.get_font_size()",
                                sender_id,
                            );
                            self.get_font_size(connection, sender_id).await
                        }
                        2u16 => {
                            let font_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.set_font(\"{}\")",
                                sender_id,
                                font_name
                            );
                            self.set_font(connection, sender_id, font_name).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.get_font()",
                                sender_id,
                            );
                            self.get_font(connection, sender_id).await
                        }
                        4u16 => {
                            let font_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.set_monospace_font(\"{}\")",
                                sender_id,
                                font_name
                            );
                            self.set_monospace_font(connection, sender_id, font_name)
                                .await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.get_monospace_font()",
                                sender_id,
                            );
                            self.get_monospace_font(connection, sender_id).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_font_context_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get window round corner radius"]
            fn get_round_corner_radius(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the system icon theme"]
            fn set_icon_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                theme: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the system icon theme"]
            fn get_icon_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the system active color"]
            fn set_active_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                color: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the system active color"]
            fn get_active_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the window window opacity"]
            fn set_window_opacity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                opacity: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the window window opacity"]
            fn get_window_opacity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the window theme."]
            fn set_window_theme_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: ThemeType,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the window theme type"]
            fn get_window_theme_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the window titlebar height"]
            fn set_window_titlebar_height(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Get the window titlebar height"]
            fn get_window_titlebar_height(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the context object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send this signal after setting the round corner radius."]
            fn round_corner_radius(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                radius: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.round_corner_radius({})",
                        sender_id,
                        radius
                    );
                    let payload = waynest::PayloadBuilder::new().put_int(radius).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system icon theme."]
            fn icon_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                theme_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.icon_theme(\"{}\")",
                        sender_id,
                        theme_name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(theme_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system active color"]
            fn active_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                active_color: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.active_color(\"{}\")",
                        sender_id,
                        active_color
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(active_color))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system active color"]
            fn window_opacity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                opacity: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.window_opacity({})",
                        sender_id,
                        opacity
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(opacity).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the system theme"]
            fn window_theme_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: ThemeType,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.window_theme_type({})",
                        sender_id,
                        r#type
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(r#type.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Send this signal after setting the window titlebar height"]
            fn window_titlebar_height(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_appearance_context_v1#{}.window_titlebar_height({})",
                        sender_id,
                        height
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(height).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_personalization_appearance_context_v1#{}.set_round_corner_radius({})",
                                sender_id,
                                radius
                            );
                            self.set_round_corner_radius(connection, sender_id, radius)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.get_round_corner_radius()",
                                sender_id,
                            );
                            self.get_round_corner_radius(connection, sender_id).await
                        }
                        2u16 => {
                            let theme = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.set_icon_theme(\"{}\")",
                                sender_id,
                                theme
                            );
                            self.set_icon_theme(connection, sender_id, theme).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.get_icon_theme()",
                                sender_id,
                            );
                            self.get_icon_theme(connection, sender_id).await
                        }
                        4u16 => {
                            let color = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.set_active_color(\"{}\")",
                                sender_id,
                                color
                            );
                            self.set_active_color(connection, sender_id, color).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.get_active_color()",
                                sender_id,
                            );
                            self.get_active_color(connection, sender_id).await
                        }
                        6u16 => {
                            let opacity = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.set_window_opacity({})",
                                sender_id,
                                opacity
                            );
                            self.set_window_opacity(connection, sender_id, opacity)
                                .await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.get_window_opacity()",
                                sender_id,
                            );
                            self.get_window_opacity(connection, sender_id).await
                        }
                        8u16 => {
                            let r#type = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.set_window_theme_type({})",
                                sender_id,
                                r#type
                            );
                            self.set_window_theme_type(connection, sender_id, r#type.try_into()?)
                                .await
                        }
                        9u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.get_window_theme_type()",
                                sender_id,
                            );
                            self.get_window_theme_type(connection, sender_id).await
                        }
                        10u16 => {
                            let height = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.set_window_titlebar_height({})",
                                sender_id,
                                height
                            );
                            self.set_window_titlebar_height(connection, sender_id, height)
                                .await
                        }
                        11u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.get_window_titlebar_height()",
                                sender_id,
                            );
                            self.get_window_titlebar_height(connection, sender_id).await
                        }
                        12u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_appearance_context_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_prelaunch_splash_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let app_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let sandbox_engine_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let icon_buffer = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_prelaunch_splash_manager_v1#{}.create_splash(\"{}\", \"{}\", {})",
                                sender_id,
                                app_id,
                                sandbox_engine_name,
                                icon_buffer
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.create_splash(
                                connection,
                                sender_id,
                                app_id,
                                sandbox_engine_name,
                                icon_buffer,
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
pub mod treeland_prelaunch_splash_v2 {
    #[doc = "This interface is a manager for creating prelaunch splash screens."]
    #[doc = "A prelaunch splash screen is a placeholder surface that is shown"]
    #[doc = "before an application's main window is mapped. This helps to improve"]
    #[doc = "the perceived startup time."]
    #[doc = ""]
    #[doc = "It is particularly useful for application launchers to provide immediate"]
    #[doc = "feedback to the user."]
    #[doc = ""]
    #[doc = "Compared to treeland_prelaunch_splash_manager_v1, this version returns"]
    #[doc = "a splash object from create_splash. The caller can destroy the object"]
    #[doc = "to dismiss the splash, and the compositor can send events (e.g. timeout)"]
    #[doc = "back to the caller through the object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_prelaunch_splash_manager_v2 {
        #[doc = "Trait to implement the treeland_prelaunch_splash_manager_v2 interface. See the module level documentation for more info"]
        pub trait TreelandPrelaunchSplashManagerV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_prelaunch_splash_manager_v2";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Creates a new prelaunch splash screen and returns a splash object."]
            #[doc = ""]
            #[doc = "The `app_id` is a string that identifies the application. The compositor"]
            #[doc = "will use this ID together with `sandbox_engine_name` to match the splash"]
            #[doc = "screen with the actual application window when it appears. This"]
            #[doc = "matching mechanism should also work for XWayland windows."]
            #[doc = ""]
            #[doc = "The `instance_id` identifies a specific application instance, allowing"]
            #[doc = "multiple splashes for non-singleton applications. The compositor can"]
            #[doc = "use this to associate a splash with a particular launch instance."]
            #[doc = ""]
            #[doc = "Callers MUST provide a non-empty `sandbox_engine_name` string which"]
            #[doc = "identifies the sandboxing/container."]
            fn create_splash(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                splash: waynest::ObjectId,
                app_id: String,
                instance_id: String,
                sandbox_engine_name: String,
                icon_buffer: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_prelaunch_splash_manager_v2#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let splash = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let app_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let instance_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let sandbox_engine_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let icon_buffer = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_prelaunch_splash_manager_v2#{}.create_splash({}, \"{}\", \"{}\", \"{}\", {})",
                                sender_id,
                                splash,
                                app_id,
                                instance_id,
                                sandbox_engine_name,
                                icon_buffer
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.create_splash(
                                connection,
                                sender_id,
                                splash,
                                app_id,
                                instance_id,
                                sandbox_engine_name,
                                icon_buffer,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Represents a single prelaunch splash screen created by the manager."]
    #[doc = ""]
    #[doc = "The caller can destroy this object to dismiss the corresponding splash."]
    #[doc = "The compositor may also send events through this object, for example"]
    #[doc = "to notify the caller that the splash was closed due to a timeout."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_prelaunch_splash_v2 {
        #[doc = "Trait to implement the treeland_prelaunch_splash_v2 interface. See the module level documentation for more info"]
        pub trait TreelandPrelaunchSplashV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_prelaunch_splash_v2";
            const VERSION: u32 = 1u32;
            #[doc = "Dismisses the splash screen and destroys this object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            tracing::debug!("treeland_prelaunch_splash_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
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
            const VERSION: u32 = 2u32;
            #[doc = "Inhibit idleness with given application_name and reason_for_inhibit."]
            fn inhibit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                application_name: String,
                reason_for_inhibit: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Uninhibit idleness previously inhibited by inhibit request."]
            fn uninhibit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            let application_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let reason_for_inhibit = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_screensaver_v1#{}.inhibit(\"{}\", \"{}\")",
                                sender_id,
                                application_name,
                                reason_for_inhibit
                            );
                            self.inhibit(
                                connection,
                                sender_id,
                                application_name,
                                reason_for_inhibit,
                            )
                            .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_screensaver_v1#{}.uninhibit()", sender_id,);
                            self.uninhibit(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_screensaver_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
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
            const VERSION: u32 = 2u32;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            let key = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v1#{}.register_shortcut_context(\"{}\", {})",
                                sender_id,
                                key,
                                id
                            );
                            self.register_shortcut_context(connection, sender_id, key, id)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_shortcut_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn shortcut(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_shortcut_context_v1#{}.shortcut()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_shortcut_context_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
    #[doc = "This allows multiple users to use their own set of global shortcuts"]
    #[doc = "on the same system without conflicts."]
    #[doc = "This behavior is transparent to the clients of this interface (i.e.,"]
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
        bitflags::bitflags! { # [doc = "Flags to specify the keybinding mode."] # [doc = "With key_press, the action is triggered on key press."] # [doc = "With key_release, the action is triggered on key release."] # [doc = "With repeat, the action is repeatedly triggered if the key is held down."] # [doc = ""] # [doc = "Examples:"] # [doc = "key_press | repeat: the action is triggered on key press, and repeatedly"] # [doc = "triggered if the key is held down."] # [doc = "key_press | key_release: the action is triggered on both key press and"] # [doc = "key release, auto-repeated events are ignored."] # [doc = "key_press | key_release | repeat: note that treeland repeats both key"] # [doc = "press and key release events."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct KeybindFlag : u32 { # [doc = "bind key press events"] const KeyPress = 1u32 ; # [doc = "bind key release events"] const KeyRelease = 2u32 ; # [doc = "bind autorepeat events."] const Repeat = 4u32 ; } }
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
            InvalidSurface = 4u32,
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
                    4u32 => Ok(Self::InvalidSurface),
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Acquire the shortcut manager for the current client."]
            #[doc = ""]
            #[doc = "This request must be sent before any bind/unbind request can be performed."]
            #[doc = ""]
            #[doc = "Only one client can hold exclusive control of the shortcut manager at a time,"]
            #[doc = "for a given session."]
            #[doc = "If the shortcut manager is already acquired by another client,"]
            #[doc = "the compositor will raise the `occupied` protocol error."]
            fn acquire(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            #[doc = "Each keybind has a flags argument that specifies the exact triggering condition,"]
            #[doc = "see the documentation of the keybind_flag enum for details."]
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Commit the pending bindings."]
            #[doc = ""]
            #[doc = "This request applies all the bind_key, bind_swipe_gesture and bind_hold_gesture"]
            #[doc = "requests sent since the last commit."]
            #[doc = ""]
            #[doc = "After processing this request, the compositor will emit a `commit_success` event"]
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove an existing binding."]
            #[doc = ""]
            #[doc = "Sending this request without first acquiring the shortcut manager"]
            #[doc = "will result in a `not_acquired` protocol error."]
            #[doc = ""]
            #[doc = "The binding to be removed is identified by its unique name."]
            #[doc = "If no binding with the specified name exists, the unbind request has no effect."]
            fn unbind(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Request the compositor to capture the next shortcut input once,"]
            #[doc = "bound to the provided surface."]
            #[doc = ""]
            #[doc = "The capture object is created by this request via new_id."]
            #[doc = "The compositor performs validation after receiving the request,"]
            #[doc = "and may immediately emit a terminal event on the newly created object."]
            #[doc = "Clients must be prepared to handle a capture object that is"]
            #[doc = "failed immediately after creation."]
            #[doc = ""]
            #[doc = "The surface argument must be a wl_surface owned by the requesting client."]
            #[doc = "Otherwise, the compositor will raise the `invalid_surface` protocol error."]
            #[doc = ""]
            #[doc = "The compositor must verify that the provided surface currently has"]
            #[doc = "keyboard focus or is active before starting capture."]
            #[doc = "If this check fails, the compositor must first decide that capture"]
            #[doc = "does not start, and then must emit `failed` with reason"]
            #[doc = "`not_active` on the created object."]
            #[doc = ""]
            #[doc = "The compositor monitors both key_press and key_release events during"]
            #[doc = "capture. The shortcut candidate is established on key_press of a valid"]
            #[doc = "non-modifier key; the terminal event (captured or failed) is sent on"]
            #[doc = "key_release of that non-modifier key. Pure modifier-only key presses"]
            #[doc = "(e.g. Ctrl, Alt, Shift) are silently consumed while waiting for a"]
            #[doc = "non-modifier key. Auto-repeat events are consumed and ignored."]
            #[doc = ""]
            #[doc = "If the next intercepted input is not a valid shortcut input"]
            #[doc = "(for example: unmodified alphanumeric keys such as a, b, c, d,"]
            #[doc = "1, 2, 3, unmodified Escape, or pointer button and wheel events),"]
            #[doc = "the compositor"]
            #[doc = "must fail capture with reason `interrupted`."]
            #[doc = "For either `interrupted` failure or successful `captured`,"]
            #[doc = "the triggering input event must be consumed by the compositor"]
            #[doc = "and must not be forwarded to the client."]
            #[doc = ""]
            #[doc = "The compositor creates a treeland_shortcut_capture_v1 object and starts"]
            #[doc = "a one-shot capture flow for this request."]
            #[doc = ""]
            #[doc = "This request does not modify existing shortcut bindings."]
            #[doc = "This request does not require acquire."]
            fn capture_next_shortcut(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                seat: Option<waynest::ObjectId>,
                capture: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted when a binding registered with action `notify` is activated."]
            #[doc = ""]
            #[doc = "The flags argument indicates the type of key event as defined in the keybind_flag enum."]
            fn activated(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                flags: KeybindFlag,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.activated(\"{}\", {})",
                        sender_id,
                        name,
                        flags
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(flags.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is emitted in response to a commit request,"]
            #[doc = "indicating that the commit was successful."]
            fn commit_success(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.commit_success()",
                        sender_id,
                    );
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.commit_failure(\"{}\", {})",
                        sender_id,
                        name,
                        error
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(error.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_shortcut_manager_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_shortcut_manager_v2#{}.acquire()", sender_id,);
                            self.acquire(connection, sender_id).await
                        }
                        2u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let key = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let flags = message.uint()?;
                            let action = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.bind_key(\"{}\", \"{}\", {}, {})",
                                sender_id,
                                name,
                                key,
                                flags,
                                action
                            );
                            self.bind_key(
                                connection,
                                sender_id,
                                name,
                                key,
                                flags.try_into()?,
                                action.try_into()?,
                            )
                            .await
                        }
                        3u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let finger = message.uint()?;
                            let direction = message.uint()?;
                            let action = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.bind_swipe_gesture(\"{}\", {}, {}, {})",
                                sender_id,
                                name,
                                finger,
                                direction,
                                action
                            );
                            self.bind_swipe_gesture(
                                connection,
                                sender_id,
                                name,
                                finger,
                                direction.try_into()?,
                                action.try_into()?,
                            )
                            .await
                        }
                        4u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let finger = message.uint()?;
                            let action = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.bind_hold_gesture(\"{}\", {}, {})",
                                sender_id,
                                name,
                                finger,
                                action
                            );
                            self.bind_hold_gesture(
                                connection,
                                sender_id,
                                name,
                                finger,
                                action.try_into()?,
                            )
                            .await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_shortcut_manager_v2#{}.commit()", sender_id,);
                            self.commit(connection, sender_id).await
                        }
                        6u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.unbind(\"{}\")",
                                sender_id,
                                name
                            );
                            self.unbind(connection, sender_id, name).await
                        }
                        7u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message.object()?;
                            let capture = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.capture_next_shortcut({}, {}, {})",
                                sender_id,
                                surface,
                                seat.as_ref().map_or("null".to_string(), |v| v.to_string()),
                                capture
                            );
                            self.capture_next_shortcut(
                                connection, sender_id, surface, seat, capture,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This object represents one request to capture the next shortcut input."]
    #[doc = ""]
    #[doc = "After creation, the compositor captures input once and may send at most one"]
    #[doc = "terminal event: either captured or failed."]
    #[doc = ""]
    #[doc = "If the client destroys this object while capture is pending,"]
    #[doc = "no terminal event is sent."]
    #[doc = ""]
    #[doc = "After a terminal event is sent, this object becomes inert."]
    #[doc = "The client should destroy this object promptly after receiving"]
    #[doc = "captured or failed."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_shortcut_capture_v1 {
        #[doc = "Reasons why a one-shot capture could not complete."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum FailedReason {
            #[doc = "another capture is in progress"]
            Busy = 1u32,
            #[doc = "capture was aborted by compositor policy"]
            Aborted = 2u32,
            #[doc = "surface focus or active validation failed"]
            NotActive = 3u32,
            #[doc = "capture was interrupted by a non-shortcut intercepted input"]
            Interrupted = 4u32,
        }
        impl From<FailedReason> for u32 {
            fn from(value: FailedReason) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for FailedReason {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Busy),
                    2u32 => Ok(Self::Aborted),
                    3u32 => Ok(Self::NotActive),
                    4u32 => Ok(Self::Interrupted),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for FailedReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_shortcut_capture_v1 interface. See the module level documentation for more info"]
        pub trait TreelandShortcutCaptureV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_shortcut_capture_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy this capture object."]
            #[doc = ""]
            #[doc = "Destroying the object cancels capture if it is still pending."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The compositor captured one shortcut input for this request."]
            #[doc = ""]
            #[doc = "The key argument uses the same textual format as bind_key."]
            fn captured(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                key: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_capture_v1#{}.captured(\"{}\")",
                        sender_id,
                        key
                    );
                    let payload = waynest::PayloadBuilder::new().put_string(Some(key)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The compositor could not complete this capture request."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason: FailedReason,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_capture_v1#{}.failed({})",
                        sender_id,
                        reason
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_shortcut_capture_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
            const VERSION: u32 = 2u32;
            #[doc = "Create virtual output that can be used when setting screen copy mode for use"]
            #[doc = "on multiple screens. Virtual outputs are created to mirror multiple wl_output"]
            #[doc = "outputs."]
            #[doc = ""]
            #[doc = "The name argument must not be empty. If name is empty, the compositor"]
            #[doc = "emits an error event with code invalid_group_name."]
            #[doc = ""]
            #[doc = "The element of the array is the name of the screen."]
            #[doc = ""]
            #[doc = "The first element of the array outputs is the screen to be copied, and"]
            #[doc = "the subsequent elements are the screens to be mirrored."]
            #[doc = ""]
            #[doc = "The outputs array must contain at least two elements. If the number"]
            #[doc = "of elements is less than two, the compositor emits an error event"]
            #[doc = "with code invalid_screen_number."]
            #[doc = ""]
            #[doc = "If any output name in the outputs array does not exist, the compositor"]
            #[doc = "emits an error event with code invalid_output."]
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Gets a list of virtual output names."]
            #[doc = ""]
            #[doc = "The compositor emits one virtual_output_list event for each request."]
            #[doc = "Clients should avoid issuing a new get_virtual_output_list request"]
            #[doc = "before receiving the corresponding event."]
            fn get_virtual_output_list(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The client obtains the corresponding virtual_output_v1 object"]
            #[doc = "through the virtual output name."]
            fn get_virtual_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sends a list of virtual output names to the client."]
            fn virtual_output_list(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                names: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_virtual_output_manager_v1#{}.virtual_output_list(array[{}])",
                        sender_id,
                        names.len()
                    );
                    let payload = waynest::PayloadBuilder::new().put_array(names).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let outputs = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.create_virtual_output({}, \"{}\", array[{}])",
                                sender_id,
                                id,
                                name,
                                outputs.len()
                            );
                            self.create_virtual_output(connection, sender_id, id, name, outputs)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.get_virtual_output_list()",
                                sender_id,
                            );
                            self.get_virtual_output_list(connection, sender_id).await
                        }
                        2u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.get_virtual_output(\"{}\", {})",
                                sender_id,
                                name,
                                id
                            );
                            self.get_virtual_output(connection, sender_id, name, id)
                                .await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_virtual_output_v1#{}.outputs(\"{}\", array[{}])",
                        sender_id,
                        name,
                        outputs.len()
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_array(outputs)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "When an error occurs, an error event is emitted, terminating the replication"]
            #[doc = "mode request issued by the client."]
            #[doc = ""]
            #[doc = "The code argument uses treeland_virtual_output_v1.error."]
            fn error(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                code : super :: super :: super :: treeland :: treeland_virtual_output_manager_v1 :: treeland_virtual_output_v1 :: Error,
                message: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_virtual_output_v1#{}.error({}, \"{}\")",
                        sender_id,
                        code,
                        message
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(code.into())
                        .put_string(Some(message))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_virtual_output_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Stop monitor the wallpaper color for the given screen."]
            fn unwatch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The client no longer cares about wallpaper_color."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Tell the client that the wallpaper color of the screen it is monitoring has changed."]
            #[doc = "This event will also be sent immediately when the client requests a watch."]
            fn output_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
                isdark: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_color_manager_v1#{}.output_color(\"{}\", {})",
                        sender_id,
                        output,
                        isdark
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(output))
                        .put_uint(isdark)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.watch(\"{}\")",
                                sender_id,
                                output
                            );
                            self.watch(connection, sender_id, output).await
                        }
                        1u16 => {
                            let output = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.unwatch(\"{}\")",
                                sender_id,
                                output
                            );
                            self.unwatch(connection, sender_id, output).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            #[doc = "Note:"]
            #[doc = "1. The treeland_wallpaper_v1 object must be destroyed before"]
            #[doc = "the associated wl_output is destroyed."]
            #[doc = "1. The treeland_wallpaper_v1 object must be destroyed before"]
            #[doc = "the associated wl_surface is destroyed."]
            fn get_treeland_wallpaper(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                output: waynest::ObjectId,
                surface: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_wallpaper_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_manager_v1#{}.get_treeland_wallpaper({}, {}, {})",
                                sender_id,
                                id,
                                output,
                                surface
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.get_treeland_wallpaper(connection, sender_id, id, output, surface)
                                .await
                        }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_v1#{}.failed(\"{}\", {})",
                        sender_id,
                        file_source,
                        error
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(file_source))
                        .put_uint(error.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_v1#{}.changed({}, {}, \"{}\")",
                        sender_id,
                        role,
                        source_type,
                        file_source
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(role.into())
                        .put_uint(source_type.into())
                        .put_string(Some(file_source))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                            tracing::debug!("treeland_wallpaper_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let role = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_v1#{}.set_image_source(\"{}\", {})",
                                sender_id,
                                file_source,
                                role
                            );
                            self.set_image_source(connection, sender_id, file_source, role)
                                .await
                        }
                        2u16 => {
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let role = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_v1#{}.set_video_source(\"{}\", {})",
                                sender_id,
                                file_source,
                                role
                            );
                            self.set_video_source(connection, sender_id, file_source, role)
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_notifier_v1#{}.add({}, \"{}\")",
                        sender_id,
                        source_type,
                        file_source
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(source_type.into())
                        .put_string(Some(file_source))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_notifier_v1#{}.remove(\"{}\")",
                        sender_id,
                        file_source
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(file_source))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_wallpaper_notifier_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
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
            const VERSION: u32 = 2u32;
            #[doc = "Destroys this treeland_wallpaper_shell_v1 object."]
            #[doc = ""]
            #[doc = "Destroying a bound treeland_wallpaper_shell_v1 object while there"]
            #[doc = "are still treeland_wallpaper_surface_v1 objects created from it"]
            #[doc = "is illegal and will result in a protocol error."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                            tracing::debug!("treeland_wallpaper_shell_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let file_source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_shell_v1#{}.get_treeland_wallpaper_surface({}, {}, \"{}\")",
                                sender_id,
                                id,
                                surface,
                                file_source
                            );
                            self.get_treeland_wallpaper_surface(
                                connection,
                                sender_id,
                                id,
                                surface,
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
            const VERSION: u32 = 2u32;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Informs the compositor that the wallpaper surface has completed"]
            #[doc = "its initial rendering and is ready to be displayed."]
            #[doc = ""]
            #[doc = "The client should send this request only after a valid buffer"]
            #[doc = "has been attached and committed to the associated wl_surface."]
            #[doc = ""]
            #[doc = "Before receiving this request, the compositor may keep the"]
            #[doc = "wallpaper hidden or continue displaying a fallback wallpaper."]
            #[doc = ""]
            #[doc = "This request is typically used to avoid visual glitches such as"]
            #[doc = "black frames, incomplete rendering, or uninitialized content"]
            #[doc = "during wallpaper startup."]
            #[doc = ""]
            #[doc = "Calling this request multiple times has no additional effect."]
            fn ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_surface_v1#{}.position({})",
                        sender_id,
                        position
                    );
                    let payload = waynest::PayloadBuilder::new().put_fixed(position).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_surface_v1#{}.pause()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the playback speed of the wallpaper content."]
            #[doc = ""]
            #[doc = "A rate of 1.0 represents normal speed."]
            #[doc = "A rate of 0.0 represents a paused state."]
            fn set_playback_rate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rate: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_surface_v1#{}.set_playback_rate({})",
                        sender_id,
                        rate
                    );
                    let payload = waynest::PayloadBuilder::new().put_fixed(rate).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event instructs the client to resume wallpaper updates or"]
            #[doc = "animations after a pause."]
            fn play(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> treeland_wallpaper_surface_v1#{}.play()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wallpaper_surface_v1#{}.slow_down({})",
                        sender_id,
                        duration
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(duration).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_wallpaper_surface_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let error = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wallpaper_surface_v1#{}.source_failed({})",
                                sender_id,
                                error
                            );
                            self.source_failed(connection, sender_id, error.try_into()?)
                                .await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_wallpaper_surface_v1#{}.ready()", sender_id,);
                            self.ready(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_window_management_v1#{}.show_desktop({})",
                        sender_id,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_window_management_v1#{}.set_desktop({})",
                                sender_id,
                                state
                            );
                            self.set_desktop(connection, sender_id, state).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_window_management_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol provides privileged window management capabilities"]
#[doc = "required by Wine (the Windows API compatibility layer) that are not"]
#[doc = "available through standard Wayland protocols."]
#[doc = ""]
#[doc = "Standard xdg-shell intentionally prevents clients from setting their"]
#[doc = "own screen position or controlling z-order.  Wine applications rely"]
#[doc = "heavily on absolute positioning (SetWindowPos, MoveWindow) and"]
#[doc = "stacking control (HWND_TOPMOST, HWND_TOP) to implement the Windows"]
#[doc = "window management model.  This protocol bridges that gap by allowing"]
#[doc = "a trusted Wine client to request positioning and stacking changes"]
#[doc = "while leaving the compositor the final authority over actual"]
#[doc = "placement."]
#[doc = ""]
#[doc = "A client obtains a treeland_wine_window_control_v1 object for each"]
#[doc = "xdg_toplevel it wishes to manage.  Requests on that object express"]
#[doc = "the client's desired geometry and stacking.  The compositor applies"]
#[doc = "or adjusts them and reports the actual result through events."]
#[doc = ""]
#[doc = "A Wine session is the set of all treeland_wine_window_control_v1"]
#[doc = "objects created through the same treeland_wine_window_manager_v1"]
#[doc = "global binding.  window_id values emitted by the window_id event are"]
#[doc = "unique and valid only within that set.  The compositor must enforce"]
#[doc = "this boundary: a sibling_id supplied to set_z_order that refers to a"]
#[doc = "control object from a different manager binding must be treated as if"]
#[doc = "no matching sibling exists, causing hwnd_insert_after to have no"]
#[doc = "effect."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"should\", \"should not\", and \"may\""]
#[doc = "in this document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod treeland_wine_window_management_v1 {
    #[doc = "Lets a Wine client create per-toplevel control objects.  The"]
    #[doc = "compositor should restrict binding to trusted clients identified"]
    #[doc = "by app_id or an equivalent policy mechanism."]
    #[doc = ""]
    #[doc = "The client must destroy all child"]
    #[doc = "treeland_wine_window_control_v1 objects before destroying this"]
    #[doc = "object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wine_window_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "a control object already exists for the given toplevel"]
            ToplevelAlreadyControlled = 0u32,
            #[doc = "the xdg_toplevel has no associated role or has been destroyed"]
            DefunctToplevel = 1u32,
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
                    0u32 => Ok(Self::ToplevelAlreadyControlled),
                    1u32 => Ok(Self::DefunctToplevel),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wine_window_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWineWindowManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wine_window_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys this manager object.  The client must destroy every"]
            #[doc = "treeland_wine_window_control_v1 created through this manager"]
            #[doc = "before sending this request; otherwise the compositor must"]
            #[doc = "raise an implementation-defined protocol error."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Creates a treeland_wine_window_control_v1 bound to the given"]
            #[doc = "xdg_toplevel.  At most one control object may exist for each"]
            #[doc = "toplevel; requesting a second one is a"]
            #[doc = "toplevel_already_controlled error.  If the xdg_toplevel has"]
            #[doc = "been destroyed or has no configured role the compositor must"]
            #[doc = "raise a defunct_toplevel error."]
            fn get_window_control(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_wine_window_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_manager_v1#{}.get_window_control({}, {})",
                                sender_id,
                                id,
                                toplevel
                            );
                            self.get_window_control(connection, sender_id, id, toplevel)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Controls the screen geometry and stacking order for one"]
    #[doc = "xdg_toplevel surface."]
    #[doc = ""]
    #[doc = "All coordinate values are in the compositor's global logical"]
    #[doc = "coordinate space (the same space used by wl_output.geometry and"]
    #[doc = "xdg_output.logical_position)."]
    #[doc = ""]
    #[doc = "Position requests are not applied immediately.  The"]
    #[doc = "compositor processes them and reports the result via"]
    #[doc = "configure_position.  The client should treat the values in"]
    #[doc = "configure_position as authoritative and update its internal"]
    #[doc = "state accordingly."]
    #[doc = ""]
    #[doc = "If the associated xdg_toplevel is destroyed, this object becomes"]
    #[doc = "inert.  All subsequent requests are ignored and no further events"]
    #[doc = "are emitted.  The client should destroy the inert object."]
    #[doc = ""]
    #[doc = "Window size is managed through the standard xdg_toplevel"]
    #[doc = "configure / xdg_surface.ack_configure mechanism.  This interface"]
    #[doc = "only controls screen position and stacking order."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wine_window_control_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "sibling_id is non-zero for an op that does not accept a sibling"]
            InvalidSibling = 0u32,
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
                    0u32 => Ok(Self::InvalidSibling),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Encodes the Windows SetWindowPos hWndInsertAfter parameter"]
        #[doc = "directly.  The first four values correspond to the four"]
        #[doc = "special HWND constants.  hwnd_insert_after requires a"]
        #[doc = "non-zero sibling_id in set_z_order."]
        #[doc = ""]
        #[doc = "hwnd_topmost and hwnd_notopmost change WS_EX_TOPMOST tier"]
        #[doc = "membership; the compositor must preserve the resulting tier"]
        #[doc = "until explicitly changed.  hwnd_bottom also clears topmost."]
        #[doc = "hwnd_top does not alter tier membership."]
        #[doc = ""]
        #[doc = "This is a superset of what the Wine X11 driver implements:"]
        #[doc = "the X11 driver tracks only the topmost flag and a boolean"]
        #[doc = "\"raise to top\", skipping HWND_BOTTOM and sibling-relative"]
        #[doc = "ordering (see sync_window_position comment in winex11.drv)."]
        #[doc = "This protocol implements all five Windows cases."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ZOrderOp {
            #[doc = "raise to top of current tier; no tier change (HWND_TOP)"]
            HwndTop = 0u32,
            #[doc = "lower to absolute bottom of Z-order; clears topmost (HWND_BOTTOM)"]
            HwndBottom = 1u32,
            #[doc = "enter topmost tier at its top; sets WS_EX_TOPMOST (HWND_TOPMOST)"]
            HwndTopmost = 2u32,
            #[doc = "leave topmost tier; raise to top of normal tier (HWND_NOTOPMOST)"]
            HwndNotopmost = 3u32,
            #[doc = "place directly below the sibling identified by sibling_id"]
            HwndInsertAfter = 4u32,
        }
        impl From<ZOrderOp> for u32 {
            fn from(value: ZOrderOp) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ZOrderOp {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::HwndTop),
                    1u32 => Ok(Self::HwndBottom),
                    2u32 => Ok(Self::HwndTopmost),
                    3u32 => Ok(Self::HwndNotopmost),
                    4u32 => Ok(Self::HwndInsertAfter),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ZOrderOp {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wine_window_control_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWineWindowControlV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wine_window_control_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the control object.  This does not affect the"]
            #[doc = "associated xdg_toplevel or its current position on screen."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the compositor place this window at the"]
            #[doc = "specified position in the compositor's global logical"]
            #[doc = "coordinate space."]
            #[doc = ""]
            #[doc = "The client must supply a non-zero serial that monotonically"]
            #[doc = "increases with each request, starting from 1.  The"]
            #[doc = "compositor echoes this serial in the resulting"]
            #[doc = "configure_position event so the client can correlate"]
            #[doc = "responses to requests."]
            #[doc = ""]
            #[doc = "The requested position must fall within the bounds of an"]
            #[doc = "active screen.  If the compositor determines that"]
            #[doc = "(x, y) is outside all screen regions, it rejects the"]
            #[doc = "request and emits a configure_position event reporting the"]
            #[doc = "current unchanged position."]
            #[doc = ""]
            #[doc = "Even for valid positions the compositor does not guarantee"]
            #[doc = "exact placement: it may adjust the position to respect"]
            #[doc = "window management rules or layer-shell exclusive zones."]
            #[doc = "The client must treat the values in the resulting"]
            #[doc = "configure_position event as authoritative and update its"]
            #[doc = "internal state accordingly."]
            #[doc = ""]
            #[doc = "Window size changes must use the standard xdg_toplevel"]
            #[doc = "configure / xdg_surface.ack_configure flow and are outside"]
            #[doc = "the scope of this protocol."]
            #[doc = ""]
            #[doc = "Corresponds to the position component of the Windows"]
            #[doc = "SetWindowPos and MoveWindow APIs."]
            fn set_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests a Z-order change that directly encodes the Windows"]
            #[doc = "SetWindowPos hWndInsertAfter parameter."]
            #[doc = ""]
            #[doc = "For ops hwnd_top, hwnd_bottom, hwnd_topmost, and"]
            #[doc = "hwnd_notopmost, sibling_id must be 0; a non-zero value is"]
            #[doc = "an invalid_sibling error."]
            #[doc = ""]
            #[doc = "For op hwnd_insert_after, sibling_id must be the window_id"]
            #[doc = "of a control object in the same Wine session as emitted by"]
            #[doc = "the window_id event on that control.  The compositor places"]
            #[doc = "this surface directly below the identified sibling in"]
            #[doc = "Z-order.  If sibling_id is 0 or does not identify a live"]
            #[doc = "surface in the same session, the compositor must ignore the"]
            #[doc = "request and leave the current Z-order unchanged."]
            #[doc = ""]
            #[doc = "hwnd_insert_after must not mix tiers: if the sibling is in"]
            #[doc = "a different tier from this surface, the compositor must clamp"]
            #[doc = "to hwnd_top within the appropriate tier."]
            #[doc = ""]
            #[doc = "The compositor reports the resulting stacking state via"]
            #[doc = "configure_stacking."]
            #[doc = ""]
            #[doc = "Note: hwnd_top and hwnd_insert_after do not change tier"]
            #[doc = "membership and therefore do not emit configure_stacking."]
            fn set_z_order(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                op: ZOrderOp,
                sibling_id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Emitted by the compositor immediately after this control"]
            #[doc = "object is created, before any configure_position or"]
            #[doc = "configure_stacking event."]
            #[doc = ""]
            #[doc = "The id is a non-zero uint32 that uniquely identifies this"]
            #[doc = "toplevel within the current Wine session for the lifetime of"]
            #[doc = "this control object."]
            #[doc = ""]
            #[doc = "The Wine driver must store this mapping (HWND to window_id)"]
            #[doc = "in wineserver shared memory so that other Wine processes in"]
            #[doc = "the same session can look up the sibling_id to use in"]
            #[doc = "set_z_order requests."]
            fn window_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wine_window_control_v1#{}.window_id({})",
                        sender_id,
                        id
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(id).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sent by the compositor to inform the client of the actual"]
            #[doc = "window position in the compositor's global logical coordinate"]
            #[doc = "space."]
            #[doc = ""]
            #[doc = "This event is emitted:"]
            #[doc = "- immediately after window_id on initial creation,"]
            #[doc = "- in response to set_position requests,"]
            #[doc = "- when the user or compositor moves the window."]
            #[doc = ""]
            #[doc = "The serial field echoes the serial from the set_position"]
            #[doc = "request that caused this event.  If the position change was"]
            #[doc = "not initiated by a set_position request (e.g. initial"]
            #[doc = "creation, user drag, or compositor policy), serial is 0."]
            #[doc = ""]
            #[doc = "Window size is communicated via the standard xdg_toplevel"]
            #[doc = "configure event and is not reported here."]
            #[doc = ""]
            #[doc = "The client should use these coordinates to synchronize its"]
            #[doc = "internal window state (e.g. update the wineserver window"]
            #[doc = "rectangles with the correct screen origin)."]
            fn configure_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wine_window_control_v1#{}.configure_position({}, {}, {})",
                        sender_id,
                        x,
                        y,
                        serial
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_uint(serial)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sent by the compositor to report whether this toplevel is"]
            #[doc = "currently in the topmost tier (WS_EX_TOPMOST equivalent)."]
            #[doc = ""]
            #[doc = "This event is emitted:"]
            #[doc = "- immediately after window_id and configure_position on"]
            #[doc = "initial creation, to report the current tier state,"]
            #[doc = "- in response to set_z_order requests that change tier"]
            #[doc = "membership (hwnd_topmost, hwnd_notopmost, hwnd_bottom),"]
            #[doc = "- when the compositor changes tiers on its own initiative."]
            #[doc = ""]
            #[doc = "Note: hwnd_top and hwnd_insert_after do not change tier"]
            #[doc = "membership and therefore do not trigger this event."]
            fn configure_stacking(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                topmost: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wine_window_control_v1#{}.configure_stacking({})",
                        sender_id,
                        topmost
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(topmost).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_wine_window_control_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_control_v1#{}.set_position({}, {}, {})",
                                sender_id,
                                x,
                                y,
                                serial
                            );
                            self.set_position(connection, sender_id, x, y, serial).await
                        }
                        2u16 => {
                            let op = message.uint()?;
                            let sibling_id = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_control_v1#{}.set_z_order({}, {})",
                                sender_id,
                                op,
                                sibling_id
                            );
                            self.set_z_order(connection, sender_id, op.try_into()?, sibling_id)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol provides window state management capabilities required"]
#[doc = "by Wine (the Windows API compatibility layer) that are not available"]
#[doc = "through standard Wayland protocols."]
#[doc = ""]
#[doc = "Standard xdg-shell lacks:"]
#[doc = "- any way to read whether a toplevel is minimized,"]
#[doc = "- any way to programmatically unminimize a toplevel,"]
#[doc = "- any way to request activation without a valid xdg-activation token"]
#[doc = "originating from a prior user interaction,"]
#[doc = "- any way to request a visual attention hint (taskbar flash) without"]
#[doc = "stealing focus."]
#[doc = ""]
#[doc = "This protocol fills those gaps.  A client obtains a"]
#[doc = "treeland_wine_window_state_v1 object for each xdg_toplevel whose"]
#[doc = "state it needs to observe or control.  The compositor remains the"]
#[doc = "final authority: every request is advisory, and the compositor"]
#[doc = "reports the authoritative state through events."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"should\", \"should not\", and \"may\""]
#[doc = "in this document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod treeland_wine_window_state_v1 {
    #[doc = "Lets a Wine client create per-toplevel state objects.  The"]
    #[doc = "compositor should restrict binding to trusted clients identified"]
    #[doc = "by app_id or an equivalent policy mechanism."]
    #[doc = ""]
    #[doc = "The client must destroy all child"]
    #[doc = "treeland_wine_window_state_v1 objects before destroying this"]
    #[doc = "object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wine_window_state_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "a state object already exists for the given toplevel"]
            ToplevelAlreadyBound = 0u32,
            #[doc = "the xdg_toplevel has no associated role or has been destroyed"]
            DefunctToplevel = 1u32,
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
                    0u32 => Ok(Self::ToplevelAlreadyBound),
                    1u32 => Ok(Self::DefunctToplevel),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wine_window_state_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWineWindowStateManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wine_window_state_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys this manager object.  The client must destroy every"]
            #[doc = "treeland_wine_window_state_v1 created through this manager"]
            #[doc = "before sending this request; otherwise the compositor must"]
            #[doc = "raise an implementation-defined protocol error."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Creates a treeland_wine_window_state_v1 bound to the given"]
            #[doc = "xdg_toplevel.  At most one state object may exist for each"]
            #[doc = "toplevel; requesting a second one is a"]
            #[doc = "toplevel_already_bound error.  If the xdg_toplevel has been"]
            #[doc = "destroyed or has no configured role, the compositor must"]
            #[doc = "raise a defunct_toplevel error."]
            #[doc = ""]
            #[doc = "Upon creation, the compositor immediately sends a"]
            #[doc = "state_changed event to report the current state of the"]
            #[doc = "toplevel."]
            fn get_window_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
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
                                "treeland_wine_window_state_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_state_manager_v1#{}.get_window_state({}, {})",
                                sender_id,
                                id,
                                toplevel
                            );
                            self.get_window_state(connection, sender_id, id, toplevel)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Observes and controls the lifecycle state for one xdg_toplevel"]
    #[doc = "surface."]
    #[doc = ""]
    #[doc = "State flags are reported via the state_changed event.  Each"]
    #[doc = "flag corresponds to a Windows window state that xdg-shell does"]
    #[doc = "not expose to clients."]
    #[doc = ""]
    #[doc = "If the associated xdg_toplevel is destroyed, this object"]
    #[doc = "becomes inert.  All subsequent requests are ignored and no"]
    #[doc = "further events are emitted.  The client should destroy the"]
    #[doc = "inert object."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wine_window_state_v1 {
        bitflags::bitflags! { # [doc = "Bitfield of window states that the compositor tracks for"] # [doc = "this toplevel.  These supplement the states delivered by"] # [doc = "xdg_toplevel.configure.  Keyboard focus (activated) is"] # [doc = "excluded because it is already delivered by xdg_toplevel;"] # [doc = "duplicating it here would create two conflicting sources of"] # [doc = "truth."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct State : u32 { # [doc = "the toplevel is minimized (iconified)"] const Minimized = 1u32 ; # [doc = "the toplevel is requesting user attention (taskbar flash active)"] const Attention = 2u32 ; } }
        impl From<State> for u32 {
            fn from(value: State) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for State {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Provides semantic context for an activate request so the"]
        #[doc = "compositor can apply appropriate focus-stealing prevention"]
        #[doc = "policy."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ActivateReason {
            #[doc = "the user explicitly asked to activate this window (e.g. taskbar click)"]
            UserRequest = 0u32,
            #[doc = "the application is activating itself (e.g. SetForegroundWindow without direct user gesture)"]
            Programmatic = 1u32,
            #[doc = "the window is being restored from a minimized or hidden state"]
            Restore = 2u32,
        }
        impl From<ActivateReason> for u32 {
            fn from(value: ActivateReason) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ActivateReason {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::UserRequest),
                    1u32 => Ok(Self::Programmatic),
                    2u32 => Ok(Self::Restore),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ActivateReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_wine_window_state_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWineWindowStateV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_wine_window_state_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the state object.  This does not affect the"]
            #[doc = "associated xdg_toplevel."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the compositor restore this toplevel from the"]
            #[doc = "minimized state."]
            #[doc = ""]
            #[doc = "Standard xdg-shell provides xdg_toplevel.set_minimized but"]
            #[doc = "no way to reverse it.  This request fills that gap."]
            #[doc = ""]
            #[doc = "If the toplevel is not currently minimized, this request has"]
            #[doc = "no effect.  If the compositor honors the request, it emits a"]
            #[doc = "state_changed event with the minimized flag cleared."]
            #[doc = ""]
            #[doc = "This corresponds to the Wine/Windows ShowWindow(SW_RESTORE)"]
            #[doc = "call on a minimized window."]
            fn unminimize(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests that the compositor give keyboard focus to this"]
            #[doc = "toplevel and raise it in the stacking order."]
            #[doc = ""]
            #[doc = "serial is a client-assigned value that is echoed back in"]
            #[doc = "any activate_denied event for this request, allowing the"]
            #[doc = "client to correlate denials when multiple activations are"]
            #[doc = "in flight."]
            #[doc = ""]
            #[doc = "The reason argument informs the compositor about the context"]
            #[doc = "of the activation.  Compositors should generally honor"]
            #[doc = "user_request and restore activations, and may choose to"]
            #[doc = "deny or convert programmatic activations into attention"]
            #[doc = "requests based on focus-stealing prevention policy."]
            #[doc = ""]
            #[doc = "If the compositor accepts the request, no activate_denied"]
            #[doc = "event is sent.  Acceptance does not guarantee immediate"]
            #[doc = "keyboard focus: if another surface (e.g. a layer-shell"]
            #[doc = "surface with exclusive keyboard interactivity) currently"]
            #[doc = "holds focus, the compositor places this toplevel at the"]
            #[doc = "head of the focus fallback queue and activates it once"]
            #[doc = "the exclusive focus is released.  The client will"]
            #[doc = "observe the eventual focus grant through the normal"]
            #[doc = "xdg_toplevel.configure activated state."]
            #[doc = ""]
            #[doc = "If the compositor denies the request, it emits an"]
            #[doc = "activate_denied event carrying the same serial, and may"]
            #[doc = "set the attention flag via state_changed."]
            #[doc = ""]
            #[doc = "Corresponds to the Wine/Windows SetForegroundWindow API."]
            #[doc = "Under X11, Wine sends a _NET_ACTIVE_WINDOW client message"]
            #[doc = "with source indication; this request serves the same role"]
            #[doc = "under Wayland."]
            fn activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                reason: ActivateReason,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Asks the compositor to signal that this toplevel needs user"]
            #[doc = "attention in the taskbar or dock.  This must not steal"]
            #[doc = "keyboard focus."]
            #[doc = ""]
            #[doc = "The compositor forwards this hint to the taskbar via the"]
            #[doc = "foreign toplevel manager protocol.  Whether the taskbar"]
            #[doc = "honors count and timeout_ms is implementation-defined and"]
            #[doc = "not guaranteed by the compositor; clients must not rely on"]
            #[doc = "exact flash behavior."]
            #[doc = ""]
            #[doc = "count specifies how many times to flash.  0 means flash"]
            #[doc = "indefinitely until the window is activated, corresponding"]
            #[doc = "to FLASHW_TIMER in the Windows FlashWindowEx API."]
            #[doc = ""]
            #[doc = "timeout_ms specifies the desired interval between flashes"]
            #[doc = "in milliseconds.  0 requests the compositor or taskbar"]
            #[doc = "default rate.  This value is forwarded as a hint only."]
            #[doc = ""]
            #[doc = "Sending this request replaces any currently active"]
            #[doc = "attention hint.  To stop flashing, use clear_attention."]
            #[doc = ""]
            #[doc = "This request covers the taskbar/tray component"]
            #[doc = "(FLASHW_TRAY) of the Windows FlashWindowEx API."]
            #[doc = "Title-bar caption flashing (FLASHW_CAPTION) is handled"]
            #[doc = "by Wine's client-side decoration layer and is outside"]
            #[doc = "the scope of this protocol."]
            #[doc = ""]
            #[doc = "Under X11, Wine sets _NET_WM_STATE_DEMANDS_ATTENTION;"]
            #[doc = "this request serves the same role under Wayland."]
            fn set_attention(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                count: u32,
                timeout_ms: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Cancels any active attention hint set via set_attention."]
            #[doc = "If no hint is active, this request has no effect."]
            #[doc = ""]
            #[doc = "The compositor emits a state_changed event with the"]
            #[doc = "attention flag cleared."]
            #[doc = ""]
            #[doc = "Corresponds to FlashWindowEx with FLASHW_STOP."]
            fn clear_attention(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Emitted whenever the state flags of the toplevel change."]
            #[doc = ""]
            #[doc = "This event is sent:"]
            #[doc = "- immediately after the treeland_wine_window_state_v1 is"]
            #[doc = "created, to report current state,"]
            #[doc = "- when the user or compositor minimizes or restores the"]
            #[doc = "window,"]
            #[doc = "- when an attention hint is set or cleared."]
            #[doc = ""]
            #[doc = "The state argument is a bitfield of treeland_wine_window_state_v1.state"]
            #[doc = "values.  Flags not present in the bitfield are considered"]
            #[doc = "unset."]
            fn state_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: State,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wine_window_state_v1#{}.state_changed({})",
                        sender_id,
                        state
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Emitted when the compositor denies an activate request."]
            #[doc = "The serial matches the serial field of the activate request"]
            #[doc = "that was denied."]
            #[doc = ""]
            #[doc = "The compositor may have set the attention flag via"]
            #[doc = "state_changed instead.  The client must not retry the"]
            #[doc = "activation automatically."]
            fn activate_denied(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_wine_window_state_v1#{}.activate_denied({})",
                        sender_id,
                        serial
                    );
                    let payload = waynest::PayloadBuilder::new().put_uint(serial).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
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
                                "treeland_wine_window_state_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_state_v1#{}.unminimize()",
                                sender_id,
                            );
                            self.unminimize(connection, sender_id).await
                        }
                        2u16 => {
                            let serial = message.uint()?;
                            let reason = message.uint()?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_state_v1#{}.activate({}, {}, {})",
                                sender_id,
                                serial,
                                reason,
                                seat
                            );
                            self.activate(connection, sender_id, serial, reason.try_into()?, seat)
                                .await
                        }
                        3u16 => {
                            let count = message.uint()?;
                            let timeout_ms = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_state_v1#{}.set_attention({}, {})",
                                sender_id,
                                count,
                                timeout_ms
                            );
                            self.set_attention(connection, sender_id, count, timeout_ms)
                                .await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_wine_window_state_v1#{}.clear_attention()",
                                sender_id,
                            );
                            self.clear_attention(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
