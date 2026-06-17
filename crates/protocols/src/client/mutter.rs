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
