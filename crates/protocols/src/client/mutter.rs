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
            const VERSION: u32 = 7u32;
            fn get_gtk_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                gtk_surface: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_shell1#{}.get_gtk_surface({}, {})",
                        sender_id,
                        gtk_surface,
                        surface
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(gtk_surface))
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_startup_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                startup_id: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_shell1#{}.set_startup_id(\"{}\")",
                        sender_id,
                        startup_id
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(startup_id)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn system_bell(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_shell1#{}.system_bell({})",
                        sender_id,
                        surface
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let payload = waynest::PayloadBuilder::new().put_object(surface).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn notify_launch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                startup_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_shell1#{}.notify_launch(\"{}\")",
                        sender_id,
                        startup_id
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(startup_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn capabilities(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                capabilities: u32,
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
                            let capabilities = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_shell1#{}.capabilities({})",
                                sender_id,
                                capabilities
                            );
                            self.capabilities(connection, sender_id, capabilities).await
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
            const VERSION: u32 = 7u32;
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_surface1#{}.set_dbus_properties(\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")",
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
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(application_id)
                        .put_string(app_menu_path)
                        .put_string(menubar_path)
                        .put_string(window_object_path)
                        .put_string(application_object_path)
                        .put_string(unique_bus_name)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_modal(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> gtk_surface1#{}.set_modal()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn unset_modal(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> gtk_surface1#{}.unset_modal()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn present(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                time: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> gtk_surface1#{}.present({})", sender_id, time);
                    let payload = waynest::PayloadBuilder::new().put_uint(time).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn request_focus(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                startup_id: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_surface1#{}.request_focus(\"{}\")",
                        sender_id,
                        startup_id
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(startup_id)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> gtk_surface1#{}.release()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn titlebar_gesture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                seat: waynest::ObjectId,
                gesture: Gesture,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_surface1#{}.titlebar_gesture({}, {}, {})",
                        sender_id,
                        serial,
                        seat,
                        gesture
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_object(Some(seat))
                        .put_uint(gesture.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_a11y_properties(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                a11y_dbus_name: String,
                toplevel_object_path: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> gtk_surface1#{}.set_a11y_properties(\"{}\", \"{}\")",
                        sender_id,
                        a11y_dbus_name,
                        toplevel_object_path
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(a11y_dbus_name))
                        .put_string(Some(toplevel_object_path))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                states: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn configure_edges(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                constraints: Vec<u8>,
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
                            let states = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_surface1#{}.configure(array[{}])",
                                sender_id,
                                states.len()
                            );
                            self.configure(connection, sender_id, states).await
                        }
                        1u16 => {
                            let constraints = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "gtk_surface1#{}.configure_edges(array[{}])",
                                sender_id,
                                constraints.len()
                            );
                            self.configure_edges(connection, sender_id, constraints)
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> mutter_x11_interop#{}.destroy()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_x11_parent(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                xwindow: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> mutter_x11_interop#{}.set_x11_parent({}, {})",
                        sender_id,
                        surface,
                        xwindow
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .put_uint(xwindow)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_manager_v1#{}.destroy()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_session_manager_v1#{}.get_session({}, {}, \"{}\")",
                        sender_id,
                        id,
                        reason,
                        session
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_uint(reason.into())
                        .put_string(session)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.destroy()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Remove the session, making it no longer available for restoration. A"]
            #[doc = "compositor should in response to this request remove the data related to"]
            #[doc = "this session from its storage."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.remove()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_session_v1#{}.add_toplevel({}, {}, \"{}\")",
                        sender_id,
                        id,
                        toplevel,
                        name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(toplevel))
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_session_v1#{}.restore_toplevel({}, {}, \"{}\")",
                        sender_id,
                        id,
                        toplevel,
                        name
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(toplevel))
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Emitted at most once some time after getting a new session object. It"]
            #[doc = "means that no previous state was restored, and a new session was created."]
            #[doc = "The passed id can be used to restore previous sessions."]
            fn created(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Emitted at most once some time after getting a new session object. It"]
            #[doc = "means that previous state was at least partially restored. The same id"]
            #[doc = "can again be used to restore previous sessions."]
            fn restored(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Emitted at most once, if the session was taken over by some other"]
            #[doc = "client. When this happens, the session and all its toplevel session"]
            #[doc = "objects become inert, and should be destroyed."]
            fn replaced(
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
                            let id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_session_v1#{}.created(\"{}\")", sender_id, id);
                            self.created(connection, sender_id, id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_session_v1#{}.restored()", sender_id,);
                            self.restored(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_session_v1#{}.replaced()", sender_id,);
                            self.replaced(connection, sender_id).await
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
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_toplevel_session_v1#{}.destroy()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Remove a specified surface from the session and render any corresponding"]
            #[doc = "xx_toplevel_session_v1 object inert. The compositor should remove any"]
            #[doc = "data related to the toplevel in the corresponding session from its internal"]
            #[doc = "storage."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_toplevel_session_v1#{}.remove()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_toplevel_session_v1#{}.restored({})",
                                sender_id,
                                surface
                            );
                            self.restored(connection, sender_id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows compositors to act as input methods and to send text"]
#[doc = "to applications. A text input object is used to manage state of what are"]
#[doc = "typically text entry fields in the application."]
#[doc = ""]
#[doc = "This document adheres to the RFC 2119 when using words like \"must\","]
#[doc = "\"should\", \"may\", etc."]
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
pub mod text_input_unstable_v3 {
    #[doc = "The zwp_text_input_v3 interface represents text input and input methods"]
    #[doc = "associated with a seat. It provides enter/leave events to follow the"]
    #[doc = "text input focus for a seat."]
    #[doc = ""]
    #[doc = "Requests are used to enable/disable the text-input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about the entered text is sent to the text-input object"]
    #[doc = "via the preedit_string and commit_string events."]
    #[doc = ""]
    #[doc = "Text is valid UTF-8 encoded, indices and lengths are in bytes. Indices"]
    #[doc = "must not point to middle bytes inside a code point: they must either"]
    #[doc = "point to the first byte of a code point or to the end of the buffer."]
    #[doc = "Lengths must be measured between two valid indices."]
    #[doc = ""]
    #[doc = "Focus moving throughout surfaces will result in the emission of"]
    #[doc = "zwp_text_input_v3.enter and zwp_text_input_v3.leave events. The focused"]
    #[doc = "surface must commit zwp_text_input_v3.enable and"]
    #[doc = "zwp_text_input_v3.disable requests as the keyboard focus moves across"]
    #[doc = "editable and non-editable elements of the UI. Those two requests are not"]
    #[doc = "expected to be paired with each other, the compositor must be able to"]
    #[doc = "handle consecutive series of the same request."]
    #[doc = ""]
    #[doc = "State is sent by the state requests (set_surrounding_text,"]
    #[doc = "set_content_type and set_cursor_rectangle) and a commit request. After an"]
    #[doc = "enter event or disable request all state information is invalidated and"]
    #[doc = "needs to be resent by the client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_v3 {
        #[doc = "Reason for the change of surrounding text or cursor posision."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ChangeCause {
            #[doc = "input method caused the change"]
            InputMethod = 0u32,
            #[doc = "something else than the input method caused the change"]
            Other = 1u32,
        }
        impl From<ChangeCause> for u32 {
            fn from(value: ChangeCause) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ChangeCause {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InputMethod),
                    1u32 => Ok(Self::Other),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ChangeCause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behavior"] const None = 0u32 ; # [doc = "suggest word completions"] const Completion = 1u32 ; # [doc = "suggest word corrections"] const Spellcheck = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just Latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; # [doc = "an on-screen way to fill in the input is already provided by the client"] const OnScreenInputProvided = 1024u32 ; # [doc = "prefer not offering emoji support"] const NoEmoji = 2048u32 ; # [doc = "the text input will display preedit text in place"] const PreeditShown = 4096u32 ; } }
        impl From<ContentHint> for u32 {
            fn from(value: ContentHint) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for ContentHint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ContentHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "The content purpose allows to specify the primary purpose of a text"]
        #[doc = "input."]
        #[doc = ""]
        #[doc = "This allows an input method to show special purpose input panels with"]
        #[doc = "extra characters or to disallow some characters."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ContentPurpose {
            #[doc = "default input, allowing all characters"]
            Normal = 0u32,
            #[doc = "allow only alphabetic characters"]
            Alpha = 1u32,
            #[doc = "allow only digits"]
            Digits = 2u32,
            #[doc = "input a number (including decimal separator and sign)"]
            Number = 3u32,
            #[doc = "input a phone number"]
            Phone = 4u32,
            #[doc = "input an URL"]
            Url = 5u32,
            #[doc = "input an email address"]
            Email = 6u32,
            #[doc = "input a name of a person"]
            Name = 7u32,
            #[doc = "input a password (combine with sensitive_data hint)"]
            Password = 8u32,
            #[doc = "input is a numeric password (combine with sensitive_data hint)"]
            Pin = 9u32,
            #[doc = "input a date"]
            Date = 10u32,
            #[doc = "input a time"]
            Time = 11u32,
            #[doc = "input a date and time"]
            Datetime = 12u32,
            #[doc = "input for a terminal"]
            Terminal = 13u32,
        }
        impl From<ContentPurpose> for u32 {
            fn from(value: ContentPurpose) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ContentPurpose {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::Alpha),
                    2u32 => Ok(Self::Digits),
                    3u32 => Ok(Self::Number),
                    4u32 => Ok(Self::Phone),
                    5u32 => Ok(Self::Url),
                    6u32 => Ok(Self::Email),
                    7u32 => Ok(Self::Name),
                    8u32 => Ok(Self::Password),
                    9u32 => Ok(Self::Pin),
                    10u32 => Ok(Self::Date),
                    11u32 => Ok(Self::Time),
                    12u32 => Ok(Self::Datetime),
                    13u32 => Ok(Self::Terminal),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ContentPurpose {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "an invalid or duplicate action was specified"]
            InvalidAction = 0u32,
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
                    0u32 => Ok(Self::InvalidAction),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "A possible action to perform on a text input."]
        #[doc = ""]
        #[doc = "The submit action is intended for input entries that expect some sort of"]
        #[doc = "activation after user interaction, e.g. the URL entry in a browser."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Action {
            #[doc = "no action"]
            None = 0u32,
            #[doc = "the action is submitted"]
            Submit = 1u32,
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
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Submit),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Action {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Style hints for the preedit string."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PreeditHint {
            #[doc = "simple pre-edit text style, typically underlined"]
            Whole = 1u32,
            #[doc = "hint for a selected piece of text, e.g. per-character navigation and composition"]
            Selection = 2u32,
            #[doc = "predicted text, not typed by the user"]
            Prediction = 3u32,
            #[doc = "prefixed text not being currently edited, e.g. prior to a 'selection' section"]
            Prefix = 4u32,
            #[doc = "suffixed text not being currently edited, e.g. after a 'selection' section"]
            Suffix = 5u32,
            #[doc = "spelling error"]
            SpellingError = 6u32,
            #[doc = "wrong composition, e.g. user input that can not be transliterated"]
            ComposeError = 7u32,
        }
        impl From<PreeditHint> for u32 {
            fn from(value: PreeditHint) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PreeditHint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Whole),
                    2u32 => Ok(Self::Selection),
                    3u32 => Ok(Self::Prediction),
                    4u32 => Ok(Self::Prefix),
                    5u32 => Ok(Self::Suffix),
                    6u32 => Ok(Self::SpellingError),
                    7u32 => Ok(Self::ComposeError),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PreeditHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_text_input_v3 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV3
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "zwp_text_input_v3";
            const VERSION: u32 = 2u32;
            #[doc = "Destroy the wp_text_input object. Also disables all surfaces enabled"]
            #[doc = "through this wp_text_input object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_v3#{}.destroy()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests text input on the surface previously obtained from the enter"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "This request must be issued every time the focused text input changes"]
            #[doc = "to a new one, including within the current surface. Use"]
            #[doc = "zwp_text_input_v3.disable when there is no longer any input focus on"]
            #[doc = "the current surface."]
            #[doc = ""]
            #[doc = "Clients must not enable more than one text input on the single seat"]
            #[doc = "and should disable the current text input before enabling the new one."]
            #[doc = "Requests to enable a text input when another text input is enabled"]
            #[doc = "on the same seat must be ignored by compositor."]
            #[doc = ""]
            #[doc = "This request resets all state associated with previous enable, disable,"]
            #[doc = "set_surrounding_text, set_text_change_cause, set_content_type, and"]
            #[doc = "set_cursor_rectangle requests, as well as the state associated with"]
            #[doc = "preedit_string, commit_string, and delete_surrounding_text events."]
            #[doc = ""]
            #[doc = "The set_surrounding_text, set_content_type and set_cursor_rectangle"]
            #[doc = "requests must follow if the text input supports the necessary"]
            #[doc = "functionality."]
            #[doc = ""]
            #[doc = "State set with this request is double-buffered. It will get applied on"]
            #[doc = "the next zwp_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The changes must be applied by the compositor after issuing a"]
            #[doc = "zwp_text_input_v3.commit request."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_v3#{}.enable()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Explicitly disable text input on the current surface (typically when"]
            #[doc = "there is no focus on any text entry inside the surface)."]
            #[doc = ""]
            #[doc = "State set with this request is double-buffered. It will get applied on"]
            #[doc = "the next zwp_text_input_v3.commit request."]
            fn disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_v3#{}.disable()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the surrounding plain text around the input, excluding the preedit"]
            #[doc = "text."]
            #[doc = ""]
            #[doc = "The client should notify the compositor of any changes in any of the"]
            #[doc = "values carried with this request, including changes caused by handling"]
            #[doc = "incoming text-input events as well as changes caused by other"]
            #[doc = "mechanisms like keyboard typing."]
            #[doc = ""]
            #[doc = "If the client is unaware of the text around the cursor, it should not"]
            #[doc = "issue this request, to signify lack of support to the compositor."]
            #[doc = ""]
            #[doc = "Text is UTF-8 encoded, and should include the cursor position, the"]
            #[doc = "complete selection and additional characters before and after them."]
            #[doc = "There is a maximum length of wayland messages, so text can not be"]
            #[doc = "longer than 4000 bytes."]
            #[doc = ""]
            #[doc = "Cursor is the byte offset of the cursor within text buffer."]
            #[doc = ""]
            #[doc = "Anchor is the byte offset of the selection anchor within text buffer."]
            #[doc = "If there is no selected text, anchor is the same as cursor."]
            #[doc = ""]
            #[doc = "If any preedit text is present, it is replaced with a cursor for the"]
            #[doc = "purpose of this event."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial state for affected fields is empty, meaning that the text"]
            #[doc = "input does not support sending surrounding text. If the empty values"]
            #[doc = "get applied, subsequent attempts to change them may have no effect."]
            fn set_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                cursor: i32,
                anchor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> zwp_text_input_v3#{}.set_surrounding_text(\"{}\", {}, {})",
                        sender_id,
                        text,
                        cursor,
                        anchor
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_string(Some(text))
                        .put_int(cursor)
                        .put_int(anchor)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tells the compositor why the text surrounding the cursor changed."]
            #[doc = ""]
            #[doc = "Whenever the client detects an external change in text, cursor, or"]
            #[doc = "anchor posision, it must issue this request to the compositor. This"]
            #[doc = "request is intended to give the input method a chance to update the"]
            #[doc = "preedit text in an appropriate way, e.g. by removing it when the user"]
            #[doc = "starts typing with a keyboard."]
            #[doc = ""]
            #[doc = "cause describes the source of the change."]
            #[doc = ""]
            #[doc = "The value set with this request is double-buffered. It must be applied"]
            #[doc = "and reset to initial at the next zwp_text_input_v3.commit request."]
            #[doc = ""]
            #[doc = "The initial value of cause is input_method."]
            fn set_text_change_cause(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                cause: ChangeCause,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> zwp_text_input_v3#{}.set_text_change_cause({})",
                        sender_id,
                        cause
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(cause.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some of"]
            #[doc = "the behavior."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request."]
            #[doc = "Subsequent attempts to update them may have no effect. The values"]
            #[doc = "remain valid until the next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial value for hint is none, and the initial value for purpose"]
            #[doc = "is normal."]
            fn set_content_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: ContentHint,
                purpose: ContentPurpose,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> zwp_text_input_v3#{}.set_content_type({}, {})",
                        sender_id,
                        hint,
                        purpose
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_uint(hint.into())
                        .put_uint(purpose.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Marks an area around the cursor as a x, y, width, height rectangle in"]
            #[doc = "surface local coordinates."]
            #[doc = ""]
            #[doc = "Allows the compositor to put a window with word suggestions near the"]
            #[doc = "cursor, without obstructing the text being input."]
            #[doc = ""]
            #[doc = "If the client is unaware of the position of edited text, it should not"]
            #[doc = "issue this request, to signify lack of support to the compositor."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial values describing a cursor rectangle are empty. That means"]
            #[doc = "the text input does not support describing the cursor area. If the"]
            #[doc = "empty values get applied, subsequent attempts to change them may have"]
            #[doc = "no effect."]
            #[doc = ""]
            #[doc = "As of version 2, the zwp_text_input_v3.commit request does not apply"]
            #[doc = "values sent with this request. Instead, it stores them in a separate"]
            #[doc = "\"committed\" area. The committed values, if still valid, get applied on"]
            #[doc = "the next wl_surface.commit request on the surface with text-input focus."]
            #[doc = "Both committed and applied values get invalidated on:"]
            #[doc = ""]
            #[doc = "- the next committed enable or disable request, or"]
            #[doc = "- a change of the focused surface of the text-input (leave or enter events)."]
            #[doc = ""]
            #[doc = "This double stage application allows the compositor to position"]
            #[doc = "the input method popup in the same frame as the contents"]
            #[doc = "of the text on the surface are updated."]
            fn set_cursor_rectangle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> zwp_text_input_v3#{}.set_cursor_rectangle({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Atomically applies state changes recently sent to the compositor."]
            #[doc = ""]
            #[doc = "The commit request establishes and updates the state of the client, and"]
            #[doc = "must be issued after any changes to apply them."]
            #[doc = ""]
            #[doc = "Text input state (enabled status, content purpose, content hint,"]
            #[doc = "surrounding text and change cause, cursor rectangle) is conceptually"]
            #[doc = "double-buffered within the context of a text input, i.e. between a"]
            #[doc = "committed enable request and the following committed enable or disable"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Protocol requests modify the pending state, as opposed to the current"]
            #[doc = "state in use by the input method. A commit request atomically applies"]
            #[doc = "all pending state, replacing the current state. After commit, the new"]
            #[doc = "pending state is as documented for each related request."]
            #[doc = ""]
            #[doc = "Requests are applied in the order of arrival."]
            #[doc = ""]
            #[doc = "Neither current nor pending state are modified unless noted otherwise."]
            #[doc = ""]
            #[doc = "The compositor must count the number of commit requests coming from"]
            #[doc = "each zwp_text_input_v3 object and use the count as the serial in done"]
            #[doc = "events."]
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_v3#{}.commit()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the actions available for this text input."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request."]
            #[doc = ""]
            #[doc = "If the available_actions array contains the none action, or contains the"]
            #[doc = "same action multiple times, the compositor must raise the invalid_action"]
            #[doc = "protocol error."]
            #[doc = ""]
            #[doc = "Initially, no actions are available."]
            fn set_available_actions(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                available_actions: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> zwp_text_input_v3#{}.set_available_actions(array[{}])",
                        sender_id,
                        available_actions.len()
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_array(available_actions)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests an input panel to be shown (e.g. a on-screen keyboard)."]
            #[doc = ""]
            #[doc = "This request only hints the desired interaction pattern from the"]
            #[doc = "client side, and its effect may be ignored by compositors given"]
            #[doc = "other environmental factors. Repeated calls will be ignored."]
            fn show_input_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_v3#{}.show_input_panel()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Requests an input panel to be hidden."]
            #[doc = ""]
            #[doc = "This request only hints the desired interaction pattern from the"]
            #[doc = "client side, and its effect may be ignored by compositors given"]
            #[doc = "other environmental factors. Repeated calls will be ignored."]
            fn hide_input_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_v3#{}.hide_input_panel()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notification that this seat's text-input focus is on a certain surface."]
            #[doc = ""]
            #[doc = "If client has created multiple text input objects, compositor must send"]
            #[doc = "this event to all of them."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus. This event sets the current surface for the"]
            #[doc = "text-input object."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notification that this seat's text-input focus is no longer on a"]
            #[doc = "certain surface. The client should reset any preedit string previously"]
            #[doc = "set."]
            #[doc = ""]
            #[doc = "The leave notification clears the current surface. It is sent before"]
            #[doc = "the enter notification for the new focus. After leave event, compositor"]
            #[doc = "must ignore requests from any text input instances until next enter"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notify when a new composing text (pre-edit) should be set at the"]
            #[doc = "current cursor position. Any previously set composing text must be"]
            #[doc = "removed. Any previously existing selected text must be removed."]
            #[doc = ""]
            #[doc = "The argument text contains the pre-edit string buffer."]
            #[doc = ""]
            #[doc = "The parameters cursor_begin and cursor_end are counted in bytes"]
            #[doc = "relative to the beginning of the submitted text buffer. Cursor should"]
            #[doc = "be hidden when both are equal to -1."]
            #[doc = ""]
            #[doc = "They could be represented by the client as a line if both values are"]
            #[doc = "the same, or as a text highlight otherwise."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string, and cursor_begin,"]
            #[doc = "cursor_end and cursor_hidden are all 0."]
            fn preedit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: Option<String>,
                cursor_begin: i32,
                cursor_end: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit)."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string."]
            fn commit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted."]
            #[doc = ""]
            #[doc = "Before_length and after_length are the number of bytes before and after"]
            #[doc = "the current cursor index (excluding the selection) to delete."]
            #[doc = ""]
            #[doc = "If a preedit text is present, in effect before_length is counted from"]
            #[doc = "the beginning of it, and after_length from its end (see done event"]
            #[doc = "sequence)."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial values of both before_length and after_length are 0."]
            fn delete_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                before_length: u32,
                after_length: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Instruct the application to apply changes to state requested by the"]
            #[doc = "preedit_string, commit_string delete_surrounding_text, and action"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "The state relating to these events is double-buffered, and each one"]
            #[doc = "modifies the pending state. This event replaces the current state with"]
            #[doc = "the pending state."]
            #[doc = ""]
            #[doc = "The application must proceed by evaluating the changes in the following"]
            #[doc = "order:"]
            #[doc = ""]
            #[doc = "1. Replace existing preedit string with the cursor."]
            #[doc = "2. Delete requested surrounding text."]
            #[doc = "3. Insert commit string with the cursor at its end."]
            #[doc = "4. Calculate surrounding text to send."]
            #[doc = "5. Insert new preedit text in cursor position."]
            #[doc = "6. Place cursor inside preedit text."]
            #[doc = "7. Perform the requested action."]
            #[doc = ""]
            #[doc = "The serial number reflects the last state of the zwp_text_input_v3"]
            #[doc = "object known to the compositor. The value of the serial argument must"]
            #[doc = "be equal to the number of commit requests already issued on that object."]
            #[doc = ""]
            #[doc = "When the client receives a done event with a serial different than the"]
            #[doc = "number of past commit requests, it must proceed with evaluating and"]
            #[doc = "applying the changes as normal, except it should not change the current"]
            #[doc = "state of the zwp_text_input_v3 object. All pending state requests"]
            #[doc = "(set_surrounding_text, set_content_type and set_cursor_rectangle) on"]
            #[doc = "the zwp_text_input_v3 object should be sent and committed after"]
            #[doc = "receiving a zwp_text_input_v3.done event with a matching serial."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "An action was performed on this text input."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of action is none."]
            fn action(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                action: Action,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notify the application of language used by the input method."]
            #[doc = ""]
            #[doc = "This event will be sent on creation if known and for all subsequent changes."]
            #[doc = ""]
            #[doc = "The language should be specified as an IETF BCP 47 tag."]
            #[doc = "Setting an empty string will reset any known language back to the default unknown state."]
            fn language(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                language: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notify of contextual hints for the pre-edit string. This"]
            #[doc = "event is always sent together with a zwp_text_input_v3.preedit_string"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The parameters start and end are counted in bytes relative to the"]
            #[doc = "beginning of the text buffer submitted through"]
            #[doc = "zwp_text_input_v3.preedit_string, and represent the substring in the"]
            #[doc = "pre-edit text affected by the hint."]
            #[doc = ""]
            #[doc = "Multiple events may be submitted if the preedit string has different"]
            #[doc = "sections. The extent of hints may overlap. The parts of the preedit"]
            #[doc = "string that are not covered by any zwp_text_input_v3.preedit_hint event,"]
            #[doc = "the text will be considered unhinted. This is also the case if no"]
            #[doc = "preedit_hint event is sent."]
            #[doc = ""]
            #[doc = "Clients should provide recognizable visuals to these hints. if they are"]
            #[doc = "unable to comply with this requisition, it may be preferable for them"]
            #[doc = "keep the preedit_shown content hint disabled."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset on the next zwp_text_input_v3.done event."]
            fn preedit_hint(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                start: u32,
                end: u32,
                hint: PreeditHint,
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
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v3#{}.enter({})", sender_id, surface);
                            self.enter(connection, sender_id, surface).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v3#{}.leave({})", sender_id, surface);
                            self.leave(connection, sender_id, surface).await
                        }
                        2u16 => {
                            let text = message.string()?;
                            let cursor_begin = message.int()?;
                            let cursor_end = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v3#{}.preedit_string(\"{}\", {}, {})",
                                sender_id,
                                text.as_ref().map_or("null".to_string(), |v| v.to_string()),
                                cursor_begin,
                                cursor_end
                            );
                            self.preedit_string(
                                connection,
                                sender_id,
                                text,
                                cursor_begin,
                                cursor_end,
                            )
                            .await
                        }
                        3u16 => {
                            let text = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v3#{}.commit_string(\"{}\")",
                                sender_id,
                                text.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.commit_string(connection, sender_id, text).await
                        }
                        4u16 => {
                            let before_length = message.uint()?;
                            let after_length = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v3#{}.delete_surrounding_text({}, {})",
                                sender_id,
                                before_length,
                                after_length
                            );
                            self.delete_surrounding_text(
                                connection,
                                sender_id,
                                before_length,
                                after_length,
                            )
                            .await
                        }
                        5u16 => {
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v3#{}.done({})", sender_id, serial);
                            self.done(connection, sender_id, serial).await
                        }
                        6u16 => {
                            let action = message.uint()?;
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v3#{}.action({}, {})",
                                sender_id,
                                action,
                                serial
                            );
                            self.action(connection, sender_id, action.try_into()?, serial)
                                .await
                        }
                        7u16 => {
                            let language = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v3#{}.language(\"{}\")",
                                sender_id,
                                language
                            );
                            self.language(connection, sender_id, language).await
                        }
                        8u16 => {
                            let start = message.uint()?;
                            let end = message.uint()?;
                            let hint = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v3#{}.preedit_hint({}, {}, {})",
                                sender_id,
                                start,
                                end,
                                hint
                            );
                            self.preedit_hint(connection, sender_id, start, end, hint.try_into()?)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_manager_v3 {
        #[doc = "Trait to implement the zwp_text_input_manager_v3 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV3
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "zwp_text_input_manager_v3";
            const VERSION: u32 = 2u32;
            #[doc = "Destroy the wp_text_input_manager object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> zwp_text_input_manager_v3#{}.destroy()", sender_id,);
                    let payload = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Creates a new text-input object for a given seat."]
            fn get_text_input(
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
                        "-> zwp_text_input_manager_v3#{}.get_text_input({}, {})",
                        sender_id,
                        id,
                        seat
                    );
                    let payload = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
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
