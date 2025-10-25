#[allow(clippy::module_inception)]
pub mod gtk {
    #[doc = "gtk_shell is a protocol extension providing additional features for"]
    #[doc = "clients implementing it."]
    #[allow(clippy::too_many_arguments)]
    pub mod gtk_shell1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Capability {
            GlobalAppMenu = 1u32,
            GlobalMenuBar = 2u32,
            DesktopIcons = 3u32,
        }
        impl From<Capability> for u32 {
            fn from(value: Capability) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Capability {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::GlobalAppMenu),
                    2u32 => Ok(Self::GlobalMenuBar),
                    3u32 => Ok(Self::DesktopIcons),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the gtk_shell1 interface. See the module level documentation for more info"]
        pub trait GtkShell1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "gtk_shell1";
            const VERSION: u32 = 6u32;
            fn get_gtk_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                gtk_surface: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_startup_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                startup_id: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn system_bell(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn notify_launch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                startup_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn capabilities(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                capabilities: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> gtk_shell1#{}.capabilities({})", sender_id, capabilities);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(capabilities)
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
                            let gtk_surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_shell1#{}.get_gtk_surface({}, {})",
                                sender_id,
                                gtk_surface,
                                surface
                            );
                            self.get_gtk_surface(connection, sender_id, gtk_surface, surface)
                                .await
                        }
                        1u16 => {
                            let startup_id = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_shell1#{}.set_startup_id(\"{}\")",
                                sender_id,
                                startup_id
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_startup_id(connection, sender_id, startup_id).await
                        }
                        2u16 => {
                            let surface = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_shell1#{}.system_bell({})",
                                sender_id,
                                surface
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.system_bell(connection, sender_id, surface).await
                        }
                        3u16 => {
                            let startup_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_shell1#{}.notify_launch(\"{}\")",
                                sender_id,
                                startup_id
                            );
                            self.notify_launch(connection, sender_id, startup_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod gtk_surface1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            Tiled = 1u32,
            TiledTop = 2u32,
            TiledRight = 3u32,
            TiledBottom = 4u32,
            TiledLeft = 5u32,
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
                    1u32 => Ok(Self::Tiled),
                    2u32 => Ok(Self::TiledTop),
                    3u32 => Ok(Self::TiledRight),
                    4u32 => Ok(Self::TiledBottom),
                    5u32 => Ok(Self::TiledLeft),
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
        pub enum EdgeConstraint {
            ResizableTop = 1u32,
            ResizableRight = 2u32,
            ResizableBottom = 3u32,
            ResizableLeft = 4u32,
        }
        impl From<EdgeConstraint> for u32 {
            fn from(value: EdgeConstraint) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for EdgeConstraint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::ResizableTop),
                    2u32 => Ok(Self::ResizableRight),
                    3u32 => Ok(Self::ResizableBottom),
                    4u32 => Ok(Self::ResizableLeft),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for EdgeConstraint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Gesture {
            DoubleClick = 1u32,
            RightClick = 2u32,
            MiddleClick = 3u32,
        }
        impl From<Gesture> for u32 {
            fn from(value: Gesture) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Gesture {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::DoubleClick),
                    2u32 => Ok(Self::RightClick),
                    3u32 => Ok(Self::MiddleClick),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Gesture {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            InvalidGesture = 0u32,
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
                    0u32 => Ok(Self::InvalidGesture),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the gtk_surface1 interface. See the module level documentation for more info"]
        pub trait GtkSurface1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "gtk_surface1";
            const VERSION: u32 = 6u32;
            fn set_dbus_properties(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                application_id: Option<String>,
                app_menu_path: Option<String>,
                menubar_path: Option<String>,
                window_object_path: Option<String>,
                application_object_path: Option<String>,
                unique_bus_name: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_modal(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn unset_modal(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn present(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                time: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn request_focus(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                startup_id: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn titlebar_gesture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                seat: waynest::ObjectId,
                gesture: Gesture,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                states: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_surface1#{}.configure(array[{}])",
                        sender_id,
                        states.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_array(states).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn configure_edges(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                constraints: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_surface1#{}.configure_edges(array[{}])",
                        sender_id,
                        constraints.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_array(constraints)
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
                            let application_id = message.string()?;
                            let app_menu_path = message.string()?;
                            let menubar_path = message.string()?;
                            let window_object_path = message.string()?;
                            let application_object_path = message.string()?;
                            let unique_bus_name = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_surface1#{}.set_dbus_properties(\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")",
                                sender_id,
                                application_id
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                app_menu_path
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                menubar_path
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                window_object_path
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                application_object_path
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                unique_bus_name
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_dbus_properties(
                                connection,
                                sender_id,
                                application_id,
                                app_menu_path,
                                menubar_path,
                                window_object_path,
                                application_object_path,
                                unique_bus_name,
                            )
                            .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("gtk_surface1#{}.set_modal()", sender_id,);
                            self.set_modal(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("gtk_surface1#{}.unset_modal()", sender_id,);
                            self.unset_modal(connection, sender_id).await
                        }
                        3u16 => {
                            let time = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("gtk_surface1#{}.present({})", sender_id, time);
                            self.present(connection, sender_id, time).await
                        }
                        4u16 => {
                            let startup_id = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_surface1#{}.request_focus(\"{}\")",
                                sender_id,
                                startup_id
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.request_focus(connection, sender_id, startup_id).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("gtk_surface1#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        6u16 => {
                            let serial = message.uint()?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let gesture = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_surface1#{}.titlebar_gesture({}, {}, {})",
                                sender_id,
                                serial,
                                seat,
                                gesture
                            );
                            self.titlebar_gesture(
                                connection,
                                sender_id,
                                serial,
                                seat,
                                gesture.try_into()?,
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
#[doc = "This protocol is intended to be used by the portal backend to map Wayland"]
#[doc = "dialogs as modal dialogs on top of X11 windows."]
#[allow(clippy::module_inception)]
pub mod mutter_x11_interop {
    #[allow(clippy::too_many_arguments)]
    pub mod mutter_x11_interop {
        #[doc = "Trait to implement the mutter_x11_interop interface. See the module level documentation for more info"]
        pub trait MutterX11Interop
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "mutter_x11_interop";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_x11_parent(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                xwindow: u32,
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
                            tracing::debug!("mutter_x11_interop#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let xwindow = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "mutter_x11_interop#{}.set_x11_parent({}, {})",
                                sender_id,
                                surface,
                                xwindow
                            );
                            self.set_x11_parent(connection, sender_id, surface, xwindow)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This description provides a high-level overview of the interplay between"]
#[doc = "the interfaces defined this protocol. For details, see the protocol"]
#[doc = "specification."]
#[doc = ""]
#[doc = "The xx_session_manager protocol declares interfaces necessary to"]
#[doc = "allow clients to restore toplevel state from previous executions. The"]
#[doc = "xx_session_manager_v1.get_session request can be used to obtain a"]
#[doc = "xx_session_v1 resource representing the state of a set of toplevels."]
#[doc = ""]
#[doc = "Clients may obtain the session string to use in future calls through"]
#[doc = "the xx_session_v1.created event. Compositors will use this string"]
#[doc = "as an identifiable token for future runs, possibly storing data about"]
#[doc = "the related toplevels in persistent storage."]
#[doc = ""]
#[doc = "Toplevels are managed through the xx_session_v1.add_toplevel and"]
#[doc = "xx_session_toplevel_v1.remove pair of requests. Clients will explicitly"]
#[doc = "request a toplevel to be restored according to prior state through the"]
#[doc = "xx_session_v1.restore_toplevel request before the toplevel is mapped."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[allow(clippy::module_inception)]
pub mod xx_session_management_v1 {
    #[doc = "The xx_session_manager interface defines base requests for creating and"]
    #[doc = "managing a session for an application. Sessions persist across application"]
    #[doc = "and compositor restarts unless explicitly destroyed. A session is created"]
    #[doc = "for the purpose of maintaining an application's xdg_toplevel surfaces"]
    #[doc = "across compositor or application restarts. The compositor should remember"]
    #[doc = "as many states as possible for surfaces in a given session, but there is"]
    #[doc = "no requirement for which states must be remembered."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_session_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "a requested session is already in use"]
            InUse = 1u32,
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
                    1u32 => Ok(Self::InUse),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "The reason may determine in what way a session restores the window"]
        #[doc = "management state of associated toplevels."]
        #[doc = ""]
        #[doc = "For example newly launched applications might be launched on the active"]
        #[doc = "workspace with restored size and position, while a recovered"]
        #[doc = "applications might restore additional state such as active workspace and"]
        #[doc = "stacking order."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Reason {
            Launch = 1u32,
            Recover = 2u32,
            SessionRestore = 3u32,
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
                    1u32 => Ok(Self::Launch),
                    2u32 => Ok(Self::Recover),
                    3u32 => Ok(Self::SessionRestore),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Reason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_session_manager_v1 interface. See the module level documentation for more info"]
        pub trait XxSessionManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_session_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This has no effect other than to destroy the xx_session_manager object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a session object corresponding to either an existing session"]
            #[doc = "identified by the given session identifier string or a new session."]
            #[doc = "While the session object exists, the session is considered to be \"in"]
            #[doc = "use\"."]
            #[doc = ""]
            #[doc = "If a identifier string represents a session that is currently actively"]
            #[doc = "in use by the the same client, an 'in_use' error is raised. If some"]
            #[doc = "other client is currently using the same session, the new session will"]
            #[doc = "replace managing the associated state."]
            #[doc = ""]
            #[doc = "NULL is passed to initiate a new session. If an id is passed which does"]
            #[doc = "not represent a valid session, the compositor treats it as if NULL had"]
            #[doc = "been passed."]
            #[doc = ""]
            #[doc = "A client is allowed to have any number of in use sessions at the same"]
            #[doc = "time."]
            fn get_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                reason: Reason,
                session: Option<String>,
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
                            tracing::debug!("xx_session_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let reason = message.uint()?;
                            let session = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_session_manager_v1#{}.get_session({}, {}, \"{}\")",
                                sender_id,
                                id,
                                reason,
                                session
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.get_session(connection, sender_id, id, reason.try_into()?, session)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A xx_session_v1 object represents a session for an application. While the"]
    #[doc = "object exists, all surfaces which have been added to the session will"]
    #[doc = "have states stored by the compositor which can be reapplied at a later"]
    #[doc = "time. Two sessions cannot exist for the same identifier string."]
    #[doc = ""]
    #[doc = "States for surfaces added to a session are automatically updated by the"]
    #[doc = "compositor when they are changed."]
    #[doc = ""]
    #[doc = "Surfaces which have been added to a session are automatically removed from"]
    #[doc = "the session if xdg_toplevel.destroy is called for the surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_session_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "restore cannot be performed after initial toplevel commit"]
            InvalidRestore = 1u32,
            #[doc = "toplevel name is already in used"]
            NameInUse = 2u32,
            #[doc = "toplevel was already mapped when restored"]
            AlreadyMapped = 3u32,
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
                    1u32 => Ok(Self::InvalidRestore),
                    2u32 => Ok(Self::NameInUse),
                    3u32 => Ok(Self::AlreadyMapped),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_session_v1 interface. See the module level documentation for more info"]
        pub trait XxSessionV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_session_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy a session object, preserving the current state but not continuing"]
            #[doc = "to make further updates if state changes occur. This makes the associated"]
            #[doc = "xx_toplevel_session_v1 objects inert."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove the session, making it no longer available for restoration. A"]
            #[doc = "compositor should in response to this request remove the data related to"]
            #[doc = "this session from its storage."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Attempt to add a given surface to the session. The passed name is used"]
            #[doc = "to identify what window is being restored, and may be used store window"]
            #[doc = "specific state within the session."]
            #[doc = ""]
            #[doc = "Calling this with a toplevel that is already managed by the session with"]
            #[doc = "the same associated will raise an in_use error."]
            fn add_toplevel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Inform the compositor that the toplevel associated with the passed name"]
            #[doc = "should have its window management state restored."]
            #[doc = ""]
            #[doc = "Calling this with a toplevel that is already managed by the session with"]
            #[doc = "the same associated will raise an in_use error."]
            #[doc = ""]
            #[doc = "This request must be called prior to the first commit on the associated"]
            #[doc = "wl_surface, otherwise an already_mapped error is raised."]
            #[doc = ""]
            #[doc = "As part of the initial configure sequence, if the toplevel was"]
            #[doc = "successfully restored, a xx_toplevel_session_v1.restored event is"]
            #[doc = "emitted. See the xx_toplevel_session_v1.restored event for further"]
            #[doc = "details."]
            fn restore_toplevel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Emitted at most once some time after getting a new session object. It"]
            #[doc = "means that no previous state was restored, and a new session was created."]
            #[doc = "The passed id can be used to restore previous sessions."]
            fn created(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.created(\"{}\")", sender_id, id);
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Emitted at most once some time after getting a new session object. It"]
            #[doc = "means that previous state was at least partially restored. The same id"]
            #[doc = "can again be used to restore previous sessions."]
            fn restored(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.restored()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Emitted at most once, if the session was taken over by some other"]
            #[doc = "client. When this happens, the session and all its toplevel session"]
            #[doc = "objects become inert, and should be destroyed."]
            fn replaced(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.replaced()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
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
                            tracing::debug!("xx_session_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_session_v1#{}.remove()", sender_id,);
                            self.remove(connection, sender_id).await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_session_v1#{}.add_toplevel({}, {}, \"{}\")",
                                sender_id,
                                id,
                                toplevel,
                                name
                            );
                            self.add_toplevel(connection, sender_id, id, toplevel, name)
                                .await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_session_v1#{}.restore_toplevel({}, {}, \"{}\")",
                                sender_id,
                                id,
                                toplevel,
                                name
                            );
                            self.restore_toplevel(connection, sender_id, id, toplevel, name)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod xx_toplevel_session_v1 {
        #[doc = "Trait to implement the xx_toplevel_session_v1 interface. See the module level documentation for more info"]
        pub trait XxToplevelSessionV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_toplevel_session_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the object. This has no effect window management of the"]
            #[doc = "associated toplevel."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove a specified surface from the session and render any corresponding"]
            #[doc = "xx_toplevel_session_v1 object inert. The compositor should remove any"]
            #[doc = "data related to the toplevel in the corresponding session from its internal"]
            #[doc = "storage."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The \"restored\" event is emitted prior to the first"]
            #[doc = "xdg_toplevel.configure for the toplevel. It will only be emitted after"]
            #[doc = "xx_session_v1.restore_toplevel, and the initial empty surface state has"]
            #[doc = "been applied, and it indicates that the surface's session is being"]
            #[doc = "restored with this configure event."]
            fn restored(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_toplevel_session_v1#{}.restored({})",
                        sender_id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
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
                            tracing::debug!("xx_toplevel_session_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_toplevel_session_v1#{}.remove()", sender_id,);
                            self.remove(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
