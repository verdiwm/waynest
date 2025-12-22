#[allow(clippy::module_inception)]
pub mod treeland_ddm {
    #[doc = "This object is primarily used for establish connection between"]
    #[doc = "treeland and ddm."]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_ddm {
        #[doc = "Trait to implement the treeland_ddm interface. See the module level documentation for more info"]
        pub trait TreelandDdm
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_ddm";
            const VERSION: u32 = 1u32;
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
                    tracing::debug!("-> treeland_ddm#{}.switch_to_vt({})", sender_id, vtnr);
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(vtnr).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    tracing::debug!("-> treeland_ddm#{}.acquire_vt({})", sender_id, vtnr);
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(vtnr).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                            tracing::debug!("treeland_ddm#{}.switch_to_greeter()", sender_id,);
                            self.switch_to_greeter(connection, sender_id).await
                        }
                        1u16 => {
                            let username = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_ddm#{}.switch_to_user(\"{}\")",
                                sender_id,
                                username
                            );
                            self.switch_to_user(connection, sender_id, username).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm#{}.activate_session()", sender_id,);
                            self.activate_session(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm#{}.deactivate_session()", sender_id,);
                            self.deactivate_session(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("treeland_ddm#{}.enable_render()", sender_id,);
                            self.enable_render(connection, sender_id).await
                        }
                        5u16 => {
                            let callback = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_ddm#{}.disable_render({})",
                                sender_id,
                                callback
                            );
                            self.disable_render(connection, sender_id, callback).await
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(pid).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(title))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(identifier).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_array(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_object(parent).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Gets a list of virtual output names."]
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_array(names).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
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
            #[doc = "When an error occurs, an error event is emitted, terminating the replication"]
            #[doc = "mode request issued by the client."]
            fn error(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                code: u32,
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(code)
                        .put_string(Some(message))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output))
                        .put_uint(isdark)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                pidfd: std::os::fd::BorrowedFd,
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(request_id)
                        .put_fd(pidfd)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(reason.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(pid).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(success).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(temperature).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_fixed(brightness).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
            const VERSION: u32 = 1u32;
            #[doc = "set window background, shadow based on context"]
            fn get_window_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "custom user wallpaper"]
            fn get_wallpaper_context(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
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
                                "treeland_personalization_manager_v1#{}.get_wallpaper_context({})",
                                sender_id,
                                id
                            );
                            self.get_wallpaper_context(connection, sender_id, id).await
                        }
                        2u16 => {
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
                        3u16 => {
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
                        4u16 => {
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
                fd: std::os::fd::OwnedFd,
                metadata: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_identifier(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                identifier: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_on(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                options: Options,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_isdark(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                isdark: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "get the current user's wallpaper"]
            fn get_metadata(
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
            #[doc = "Send this signal after getting the user's wallpaper."]
            fn metadata(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                metadata: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_personalization_wallpaper_context_v1#{}.metadata(\"{}\")",
                        sender_id,
                        metadata
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
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
                            let fd = waynest::Connection::fd(connection)?;
                            let metadata = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.set_fd({}, \"{}\")",
                                sender_id,
                                std::os::fd::AsRawFd::as_raw_fd(&fd),
                                metadata
                            );
                            self.set_fd(connection, sender_id, fd, metadata).await
                        }
                        1u16 => {
                            let identifier = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.set_identifier(\"{}\")",
                                sender_id,
                                identifier
                            );
                            self.set_identifier(connection, sender_id, identifier).await
                        }
                        2u16 => {
                            let output = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.set_output(\"{}\")",
                                sender_id,
                                output
                            );
                            self.set_output(connection, sender_id, output).await
                        }
                        3u16 => {
                            let options = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.set_on({})",
                                sender_id,
                                options
                            );
                            self.set_on(connection, sender_id, options.try_into()?)
                                .await
                        }
                        4u16 => {
                            let isdark = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.set_isdark({})",
                                sender_id,
                                isdark
                            );
                            self.set_isdark(connection, sender_id, isdark).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.commit()",
                                sender_id,
                            );
                            self.commit(connection, sender_id).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.get_metadata()",
                                sender_id,
                            );
                            self.get_metadata(connection, sender_id).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_personalization_wallpaper_context_v1#{}.destroy()",
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(success).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(size).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(font_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(font_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(font_size).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(radius).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(theme_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(active_color))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(opacity).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(r#type.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(height).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
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
pub mod treeland_screensaver {
    #[doc = "This object implements a simple idle inhibit protocol."]
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
    pub mod treeland_screensaver {
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
        #[doc = "Trait to implement the treeland_screensaver interface. See the module level documentation for more info"]
        pub trait TreelandScreensaver
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "treeland_screensaver";
            const VERSION: u32 = 1u32;
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
                                "treeland_screensaver#{}.inhibit(\"{}\", \"{}\")",
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
                            tracing::debug!("treeland_screensaver#{}.uninhibit()", sender_id,);
                            self.uninhibit(connection, sender_id).await
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
        #[doc = "Keybinding modes."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum KeybindMode {
            KeyRelease = 1u32,
            KeyPress = 2u32,
            KeyPressRepeat = 3u32,
        }
        impl From<KeybindMode> for u32 {
            fn from(value: KeybindMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for KeybindMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::KeyRelease),
                    2u32 => Ok(Self::KeyPress),
                    3u32 => Ok(Self::KeyPressRepeat),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for KeybindMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
            const VERSION: u32 = 1u32;
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
            #[doc = "Only one client hold exclusive control of the shortcut manager at a time,"]
            #[doc = "for a given session."]
            #[doc = "If the shortcut manager is already acquired by another client, an protocol error"]
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
            #[doc = "The protocol provides three keybinding modes:"]
            #[doc = "- key_release: the action is triggered when the key sequence is released."]
            #[doc = "- key_press: the action is triggered when the key sequence is pressed."]
            #[doc = "- key_press_repeat: the action is triggered when the key sequence is pressed,"]
            #[doc = "and repeatedly triggered if the key sequence is held down."]
            #[doc = ""]
            #[doc = "If a binding with the same key sequence and action already exists,"]
            #[doc = "the bind_key request will fail."]
            #[doc = ""]
            #[doc = "Note that the binding will not take effect until a commit request is sent."]
            fn bind_key(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                key: String,
                mode: KeybindMode,
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove an existing binding."]
            #[doc = ""]
            #[doc = "The binding to be removed is identified by its unique name."]
            #[doc = "If no binding with the specified name exists, the unbind request has no effect."]
            fn unbind(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted when a binding registered with action `notify` is activated."]
            #[doc = ""]
            #[doc = "If the binding is activated due to auto-repeat, the repeat argument will be non-zero."]
            fn activated(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                repeat: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> treeland_shortcut_manager_v2#{}.activated(\"{}\", {})",
                        sender_id,
                        name,
                        repeat
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(repeat)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
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
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(error.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
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
                            let mode = message.uint()?;
                            let action = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "treeland_shortcut_manager_v2#{}.bind_key(\"{}\", \"{}\", {}, {})",
                                sender_id,
                                name,
                                key,
                                mode,
                                action
                            );
                            self.bind_key(
                                connection,
                                sender_id,
                                name,
                                key,
                                mode.try_into()?,
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
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
