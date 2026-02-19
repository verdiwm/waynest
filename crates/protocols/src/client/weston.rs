#[allow(clippy::module_inception)]
pub mod ivi_application {
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_surface {
        #[doc = "Trait to implement the ivi_surface interface. See the module level documentation for more info"]
        pub trait IviSurface
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "ivi_surface";
            const VERSION: u32 = 1u32;
            #[doc = "This removes the link from ivi_id to wl_surface and destroys ivi_surface."]
            #[doc = "The ID, ivi_id, is free and can be used for surface_create again."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> ivi_surface#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The configure event asks the client to resize its surface."]
            #[doc = ""]
            #[doc = "The size is a hint, in the sense that the client is free to"]
            #[doc = "ignore it if it doesn't resize, pick a smaller size (to"]
            #[doc = "satisfy aspect ratio or resize in steps of NxM pixels)."]
            #[doc = ""]
            #[doc = "The client is free to dismiss all but the last configure"]
            #[doc = "event it received."]
            #[doc = ""]
            #[doc = "The width and height arguments specify the size of the window"]
            #[doc = "in surface-local coordinates."]
            fn configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
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
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "ivi_surface#{}.configure({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.configure(connection, sender_id, width, height).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface is exposed as a global singleton."]
    #[doc = "This interface is implemented by servers that provide IVI-style user interfaces."]
    #[doc = "It allows clients to associate an ivi_surface with wl_surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_application {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "given ivi_id is assigned to another wl_surface"]
            IviId = 1u32,
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
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::IviId),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_application interface. See the module level documentation for more info"]
        pub trait IviApplication
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "ivi_application";
            const VERSION: u32 = 1u32;
            #[doc = "This request gives the wl_surface the role of an IVI Surface. Creating more than"]
            #[doc = "one ivi_surface for a wl_surface is not allowed. Note, that this still allows the"]
            #[doc = "following example:"]
            #[doc = ""]
            #[doc = "1. create a wl_surface"]
            #[doc = "2. create ivi_surface for the wl_surface"]
            #[doc = "3. destroy the ivi_surface"]
            #[doc = "4. create ivi_surface for the wl_surface (with the same or another ivi_id as before)"]
            #[doc = ""]
            #[doc = "surface_create will create an interface:ivi_surface with numeric ID; ivi_id in"]
            #[doc = "ivi compositor. These ivi_ids are defined as unique in the system to identify"]
            #[doc = "it inside of ivi compositor. The ivi compositor implements business logic how to"]
            #[doc = "set properties of the surface with ivi_id according to the status of the system."]
            #[doc = "E.g. a unique ID for Car Navigation application is used for implementing special"]
            #[doc = "logic of the application about where it shall be located."]
            #[doc = "The server regards the following cases as protocol errors and disconnects the client."]
            #[doc = "- wl_surface already has another role."]
            #[doc = "- ivi_id is already assigned to another wl_surface."]
            #[doc = ""]
            #[doc = "If client destroys ivi_surface or wl_surface which is assigned to the ivi_surface,"]
            #[doc = "ivi_id which is assigned to the ivi_surface is free for reuse."]
            fn surface_create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                ivi_id: u32,
                surface: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> ivi_application#{}.surface_create({}, {}, {})",
                        sender_id,
                        ivi_id,
                        surface,
                        id
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(ivi_id)
                        .put_object(Some(surface))
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
}
#[allow(clippy::module_inception)]
pub mod ivi_hmi_controller {
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_hmi_controller {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum LayoutMode {
            Tiling = 0u32,
            SideBySide = 1u32,
            FullScreen = 2u32,
            Random = 3u32,
        }
        impl From<LayoutMode> for u32 {
            fn from(value: LayoutMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for LayoutMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Tiling),
                    1u32 => Ok(Self::SideBySide),
                    2u32 => Ok(Self::FullScreen),
                    3u32 => Ok(Self::Random),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for LayoutMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Home {
            Off = 0u32,
            On = 1u32,
        }
        impl From<Home> for u32 {
            fn from(value: Home) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Home {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Off),
                    1u32 => Ok(Self::On),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Home {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_hmi_controller interface. See the module level documentation for more info"]
        pub trait IviHmiController
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "ivi_hmi_controller";
            const VERSION: u32 = 1u32;
            fn ui_ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> ivi_hmi_controller#{}.ui_ready()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Reference protocol to control a surface by server."]
            #[doc = "To control a surface by server, it gives seat to the server"]
            #[doc = "to e.g. control Home screen. Home screen has several workspaces"]
            #[doc = "to group launchers of wayland application. These workspaces"]
            #[doc = "are drawn on a horizontally long surface to be controlled"]
            #[doc = "by motion of input device. E.g. A motion from right to left"]
            #[doc = "happens, the viewport of surface is controlled in the ivi-shell"]
            #[doc = "by using ivi-layout. client can recognizes the end of controlling"]
            #[doc = "by event \"workspace_end_control\"."]
            fn workspace_control(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> ivi_hmi_controller#{}.workspace_control({}, {})",
                        sender_id,
                        seat,
                        serial
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(seat))
                        .put_uint(serial)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "hmi-controller loaded to ivi-shall implements 4 types of layout"]
            #[doc = "as a reference; tiling, side by side, full_screen, and random."]
            fn switch_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                layout_mode: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> ivi_hmi_controller#{}.switch_mode({})",
                        sender_id,
                        layout_mode
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(layout_mode).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "home screen is a reference implementation of launcher to launch"]
            #[doc = "wayland applications. The home screen has several workspaces to"]
            #[doc = "group wayland applications. By defining the following keys in"]
            #[doc = "weston.ini, user can add launcher icon to launch a wayland application"]
            #[doc = "to a workspace."]
            #[doc = "[ivi-launcher]"]
            #[doc = "workspace-id=0"]
            #[doc = ": id of workspace to add a launcher"]
            #[doc = "icon-id=4001"]
            #[doc = ": ivi id of ivi_surface to draw an icon"]
            #[doc = "icon=/home/user/review/build-ivi-shell/data/icon_ivi_flower.png"]
            #[doc = ": path to icon image"]
            #[doc = "path=/home/user/review/build-ivi-shell/weston-dnd"]
            #[doc = ": path to wayland application"]
            fn home(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                home: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> ivi_hmi_controller#{}.home({})", sender_id, home);
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(home).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn workspace_end_control(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                is_controlled: i32,
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
                            let is_controlled = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "ivi_hmi_controller#{}.workspace_end_control({})",
                                sender_id,
                                is_controlled
                            );
                            self.workspace_end_control(connection, sender_id, is_controlled)
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
pub mod text_cursor_position {
    #[allow(clippy::too_many_arguments)]
    pub mod text_cursor_position {
        #[doc = "Trait to implement the text_cursor_position interface. See the module level documentation for more info"]
        pub trait TextCursorPosition
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "text_cursor_position";
            const VERSION: u32 = 1u32;
            fn notify(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                x: waynest::Fixed,
                y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> text_cursor_position#{}.notify({}, {}, {})",
                        sender_id,
                        surface,
                        x,
                        y
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .put_fixed(x)
                        .put_fixed(y)
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
}
#[doc = "This protocol specifies a set of interfaces used to provide"]
#[doc = "content-protection for e.g. HDCP, and protect surface contents on the"]
#[doc = "secured outputs and prevent from appearing in screenshots or from being"]
#[doc = "visible on non-secure outputs."]
#[doc = ""]
#[doc = "A secure-output is defined as an output that is secured by some"]
#[doc = "content-protection mechanism e.g. HDCP, and meets at least the minimum"]
#[doc = "required content-protection level requested by a client."]
#[doc = ""]
#[doc = "The term content-protection is defined in terms of HDCP type 0 and"]
#[doc = "HDCP type 1, but this may be extended in future."]
#[doc = ""]
#[doc = "This protocol is not intended for implementing Digital Rights Management on"]
#[doc = "general (e.g. Desktop) systems, and would only be useful for closed systems."]
#[doc = "As the server is the one responsible for implementing"]
#[doc = "the content-protection, the client can only trust the content-protection as"]
#[doc = "much they can trust the server."]
#[doc = ""]
#[doc = "In order to protect the content and prevent surface contents from appearing"]
#[doc = "in screenshots or from being visible on non-secure outputs, a client must"]
#[doc = "first bind the global interface \"weston_content_protection\" which, if a"]
#[doc = "compositor supports secure output, is exposed by the registry."]
#[doc = "Using the bound global object, the client uses the \"get_protection\" request"]
#[doc = "to instantiate an interface extension for a wl_surface object."]
#[doc = "This extended interface will then allow surfaces to request for"]
#[doc = "content-protection, and also to censor the visibility of the surface on"]
#[doc = "non-secure outputs. Client applications should not wait for the protection"]
#[doc = "to change, as it might never change in case the content-protection cannot be"]
#[doc = "achieved. Alternatively, clients can use a timeout and start showing the"]
#[doc = "content in lower quality."]
#[doc = ""]
#[doc = "Censored visibility is defined as the compositor censoring the protected"]
#[doc = "content on non-secure outputs. Censoring may include artificially reducing"]
#[doc = "image quality or replacing the protected content completely with"]
#[doc = "placeholder graphics."]
#[doc = ""]
#[doc = "Censored visibility is controlled by protection mode, set by the client."]
#[doc = "In \"relax\" mode, the compositor may show protected content on non-secure"]
#[doc = "outputs. It will be up to the client to adapt to secure and non-secure"]
#[doc = "presentation. In \"enforce\" mode, the compositor will censor the parts of"]
#[doc = "protected content that would otherwise show on non-secure outputs."]
#[allow(clippy::module_inception)]
pub mod weston_content_protection {
    #[doc = "The global interface weston_content_protection is used for exposing the"]
    #[doc = "content protection capabilities to a client. It provides a way for clients"]
    #[doc = "to request their wl_surface contents to not be displayed on an output"]
    #[doc = "below their required level of content-protection."]
    #[doc = "Using this interface clients can request for a weston_protected_surface"]
    #[doc = "which is an extension to the wl_surface to provide content-protection, and"]
    #[doc = "set the censored-visibility on the non-secured-outputs."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_content_protection {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a protected surface associated"]
            SurfaceExists = 0u32,
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
                    0u32 => Ok(Self::SurfaceExists),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_content_protection interface. See the module level documentation for more info"]
        pub trait WestonContentProtection
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_content_protection";
            const VERSION: u32 = 1u32;
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other objects,"]
            #[doc = "protected_surface objects included."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_content_protection#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Instantiate an interface extension for the given wl_surface to"]
            #[doc = "provide surface protection. If the given wl_surface already has"]
            #[doc = "a weston_protected_surface associated, the surface_exists protocol"]
            #[doc = "error is raised."]
            fn get_protection(
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
                        "-> weston_content_protection#{}.get_protection({}, {})",
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
    #[doc = "An additional interface to a wl_surface object, which allows a client to"]
    #[doc = "request the minimum level of content-protection, request to change the"]
    #[doc = "visibility of their contents, and receive notifications about changes in"]
    #[doc = "content-protection."]
    #[doc = ""]
    #[doc = "A protected surface has a 'status' associated with it, that indicates"]
    #[doc = "what type of protection it is currently providing, specified by"]
    #[doc = "content-type. Updates to this status are sent to the client"]
    #[doc = "via the 'status' event. Before the first status event is sent, the client"]
    #[doc = "should assume that the status is 'unprotected'."]
    #[doc = ""]
    #[doc = "A client can request a content protection level to be the minimum for an"]
    #[doc = "output to be considered secure, using the 'set_type' request."]
    #[doc = "It is responsibility of the client to monitor the actual"]
    #[doc = "content-protection level achieved via the 'status' event, and make"]
    #[doc = "decisions as to what content to show based on this."]
    #[doc = ""]
    #[doc = "The server should make its best effort to achieve the desired"]
    #[doc = "content-protection level on all of the outputs the client's contents are"]
    #[doc = "being displayed on. Any changes to the content protection status should be"]
    #[doc = "reported to the client, even if they are below the requested"]
    #[doc = "content-protection level. If the client's contents are being displayed on"]
    #[doc = "multiple outputs, the lowest content protection level achieved should be"]
    #[doc = "reported."]
    #[doc = ""]
    #[doc = "A client can also request that its content only be displayed on outputs"]
    #[doc = "that are considered secure. The 'enforce/relax' requests can achieve this."]
    #[doc = "In enforce mode, the content is censored for non-secure outputs."]
    #[doc = "The implementation of censored-visibility is compositor-defined."]
    #[doc = "In relax mode there are no such limitation. On an attempt to show the"]
    #[doc = "client on unsecured output, compositor would keep on showing the content"]
    #[doc = "and send the 'status' event to the client. Client can take a call to"]
    #[doc = "downgrade the content."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the protected_surface is destroyed,"]
    #[doc = "the protected_surface becomes inert."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_protected_surface {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "provided type was not valid"]
            InvalidType = 0u32,
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
                    0u32 => Ok(Self::InvalidType),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Description of a particular type of content protection."]
        #[doc = ""]
        #[doc = "A server may not necessarily support all of these types."]
        #[doc = ""]
        #[doc = "Note that there is no ordering between enum members unless specified."]
        #[doc = "Over time, different types of content protection may be added, which"]
        #[doc = "may be considered less secure than what is already here."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Type {
            #[doc = "no protection required"]
            Unprotected = 0u32,
            #[doc = "HDCP type 0"]
            Hdcp0 = 1u32,
            #[doc = "HDCP type 1. This is a more secure than HDCP type 0."]
            Hdcp1 = 2u32,
        }
        impl From<Type> for u32 {
            fn from(value: Type) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Type {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unprotected),
                    1u32 => Ok(Self::Hdcp0),
                    2u32 => Ok(Self::Hdcp1),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_protected_surface interface. See the module level documentation for more info"]
        pub trait WestonProtectedSurface
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_protected_surface";
            const VERSION: u32 = 1u32;
            #[doc = "If the protected_surface is destroyed, the wl_surface desired protection"]
            #[doc = "level returns to unprotected, as if set_type request was sent with type"]
            #[doc = "as 'unprotected'."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_protected_surface#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Informs the server about the type of content. The level of"]
            #[doc = "content-protection depends upon the content-type set by the client"]
            #[doc = "through this request. Initially, this is set to 'unprotected'."]
            #[doc = ""]
            #[doc = "If the requested value is not a valid content_type enum value, the"]
            #[doc = "'invalid_type' protocol error is raised. It is not an error to request"]
            #[doc = "a valid protection type the compositor does not implement or cannot"]
            #[doc = "achieve."]
            #[doc = ""]
            #[doc = "The requested content protection is double-buffered, see"]
            #[doc = "wl_surface.commit."]
            fn set_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: Type,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_protected_surface#{}.set_type({})",
                        sender_id,
                        r#type
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(r#type.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Censor the visibility of the wl_surface contents on non-secure outputs."]
            #[doc = "See weston_protected_surface for the description."]
            #[doc = ""]
            #[doc = "The force constrain mode is double-buffered, see wl_surface.commit"]
            fn enforce(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_protected_surface#{}.enforce()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Do not enforce censored-visibility of the wl_surface contents on"]
            #[doc = "non-secure-outputs. See weston_protected_surface for the description."]
            #[doc = ""]
            #[doc = "The relax mode is selected by default, if no explicit request is made"]
            #[doc = "for enforcing the censored-visibility."]
            #[doc = ""]
            #[doc = "The relax mode is double-buffered, see wl_surface.commit"]
            fn relax(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_protected_surface#{}.relax()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent to the client to inform about the actual protection"]
            #[doc = "level for its surface in the relax mode."]
            #[doc = ""]
            #[doc = "The 'type' argument indicates what that current level of content"]
            #[doc = "protection that the server has currently established."]
            #[doc = ""]
            #[doc = "The 'status' event is first sent, when a weston_protected_surface is"]
            #[doc = "created."]
            #[doc = ""]
            #[doc = "Until this event is sent for the first time, the client should assume"]
            #[doc = "that its contents are not secure, and the type is 'unprotected'."]
            #[doc = ""]
            #[doc = "Possible reasons the content protection status can change is due to"]
            #[doc = "change in censored-visibility mode from enforced to relaxed, a new"]
            #[doc = "connector being added, movement of window to another output, or,"]
            #[doc = "the client attaching a buffer too large for what the server may secure."]
            #[doc = "However, it is not limited to these reasons."]
            #[doc = ""]
            #[doc = "A client may want to listen to this event and lower the resolution of"]
            #[doc = "their content until it can successfully be shown securely."]
            #[doc = ""]
            #[doc = "In case of \"enforce\" mode, the client will not get any status event."]
            #[doc = "If the mode is then changed to \"relax\", the client will receive the"]
            #[doc = "status event."]
            fn status(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: Type,
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
                            let r#type = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_protected_surface#{}.status({})",
                                sender_id,
                                r#type
                            );
                            self.status(connection, sender_id, r#type.try_into()?).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_debug {
    #[doc = "This is a generic debugging interface for Weston internals, the global"]
    #[doc = "object advertized through wl_registry."]
    #[doc = ""]
    #[doc = "WARNING: This interface by design allows a denial-of-service attack. It"]
    #[doc = "should not be offered in production, or proper authorization mechanisms"]
    #[doc = "must be enforced."]
    #[doc = ""]
    #[doc = "The idea is for a client to provide a file descriptor that the server"]
    #[doc = "uses for printing debug information. The server uses the file"]
    #[doc = "descriptor in blocking writes mode, which exposes the denial-of-service"]
    #[doc = "risk. The blocking mode is necessary to ensure all debug messages can"]
    #[doc = "be easily printed in place. It also ensures message ordering if a"]
    #[doc = "client subscribes to more than one debug stream."]
    #[doc = ""]
    #[doc = "The available debugging features depend on the server."]
    #[doc = ""]
    #[doc = "A debug stream can be one-shot where the server prints the requested"]
    #[doc = "information and then closes it, or continuous where server keeps on"]
    #[doc = "printing until the client stops it. Or anything in between."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_debug_v1 {
        #[doc = "Trait to implement the weston_debug_v1 interface. See the module level documentation for more info"]
        pub trait WestonDebugV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_debug_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the factory object, but does not affect any other objects."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_debug_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Subscribe to a named debug stream. The server will start printing"]
            #[doc = "to the given file descriptor."]
            #[doc = ""]
            #[doc = "If the named debug stream is a one-shot dump, the server will send"]
            #[doc = "weston_debug_stream_v1.complete event once all requested data has"]
            #[doc = "been printed. Otherwise, the server will continue streaming debug"]
            #[doc = "prints until the subscription object is destroyed."]
            #[doc = ""]
            #[doc = "If the debug stream name is unknown to the server, the server will"]
            #[doc = "immediately respond with weston_debug_stream_v1.failure event."]
            fn subscribe(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                streamfd: std::os::fd::BorrowedFd,
                stream: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_debug_v1#{}.subscribe(\"{}\", {}, {})",
                        sender_id,
                        name,
                        std::os::fd::AsRawFd::as_raw_fd(&streamfd),
                        stream
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_fd(streamfd)
                        .put_object(Some(stream))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Advertises an available debug scope which the client may be able to"]
            #[doc = "bind to. No information is provided by the server about the content"]
            #[doc = "contained within the debug streams provided by the scope, once a"]
            #[doc = "client has subscribed."]
            fn available(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                description: Option<String>,
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
                            let description = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_debug_v1#{}.available(\"{}\", \"{}\")",
                                sender_id,
                                name,
                                description
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.available(connection, sender_id, name, description)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Represents one subscribed debug stream, created with"]
    #[doc = "weston_debug_v1.subscribe. When the object is created, it is associated"]
    #[doc = "with a given file descriptor. The server will continue writing to the"]
    #[doc = "file descriptor until the object is destroyed or the server sends an"]
    #[doc = "event through the object."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_debug_stream_v1 {
        #[doc = "Trait to implement the weston_debug_stream_v1 interface. See the module level documentation for more info"]
        pub trait WestonDebugStreamV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_debug_stream_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroys the object, which causes the server to stop writing into"]
            #[doc = "and closes the associated file descriptor if it was not closed"]
            #[doc = "already."]
            #[doc = ""]
            #[doc = "Use a wl_display.sync if the clients needs to guarantee the file"]
            #[doc = "descriptor is closed before continuing."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_debug_stream_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The server has successfully finished writing to and has closed the"]
            #[doc = "associated file descriptor."]
            #[doc = ""]
            #[doc = "This event is delivered only for one-shot debug streams where the"]
            #[doc = "server dumps some data and stop. This is never delivered for"]
            #[doc = "continuous debbug streams because they by definition never complete."]
            fn complete(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The server has stopped writing to and has closed the"]
            #[doc = "associated file descriptor. The data already written to the file"]
            #[doc = "descriptor is correct, but it may be truncated."]
            #[doc = ""]
            #[doc = "This event may be delivered at any time and for any kind of debug"]
            #[doc = "stream. It may be due to a failure in or shutdown of the server."]
            #[doc = "The message argument may provide a hint of the reason."]
            fn failure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: Option<String>,
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
                            tracing::debug!("weston_debug_stream_v1#{}.complete()", sender_id,);
                            self.complete(connection, sender_id).await
                        }
                        1u16 => {
                            let message = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_debug_stream_v1#{}.failure(\"{}\")",
                                sender_id,
                                message
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.failure(connection, sender_id, message).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_desktop {
    #[doc = "Traditional user interfaces can rely on this interface to define the"]
    #[doc = "foundations of typical desktops. Currently it's possible to set up"]
    #[doc = "background, panels and locking surfaces."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_desktop_shell {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Cursor {
            None = 0u32,
            ResizeTop = 1u32,
            ResizeBottom = 2u32,
            Arrow = 3u32,
            ResizeLeft = 4u32,
            ResizeTopLeft = 5u32,
            ResizeBottomLeft = 6u32,
            Move = 7u32,
            ResizeRight = 8u32,
            ResizeTopRight = 9u32,
            ResizeBottomRight = 10u32,
            Busy = 11u32,
        }
        impl From<Cursor> for u32 {
            fn from(value: Cursor) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Cursor {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::ResizeTop),
                    2u32 => Ok(Self::ResizeBottom),
                    3u32 => Ok(Self::Arrow),
                    4u32 => Ok(Self::ResizeLeft),
                    5u32 => Ok(Self::ResizeTopLeft),
                    6u32 => Ok(Self::ResizeBottomLeft),
                    7u32 => Ok(Self::Move),
                    8u32 => Ok(Self::ResizeRight),
                    9u32 => Ok(Self::ResizeTopRight),
                    10u32 => Ok(Self::ResizeBottomRight),
                    11u32 => Ok(Self::Busy),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Cursor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PanelPosition {
            Top = 0u32,
            Bottom = 1u32,
            Left = 2u32,
            Right = 3u32,
        }
        impl From<PanelPosition> for u32 {
            fn from(value: PanelPosition) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PanelPosition {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Top),
                    1u32 => Ok(Self::Bottom),
                    2u32 => Ok(Self::Left),
                    3u32 => Ok(Self::Right),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PanelPosition {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "an invalid argument was provided in a request"]
            InvalidArgument = 0u32,
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
                    0u32 => Ok(Self::InvalidArgument),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_desktop_shell interface. See the module level documentation for more info"]
        pub trait WestonDesktopShell
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_desktop_shell";
            const VERSION: u32 = 1u32;
            fn set_background(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.set_background({}, {})",
                        sender_id,
                        output,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
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
            fn set_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.set_panel({}, {})",
                        sender_id,
                        output,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
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
            fn set_lock_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.set_lock_surface({})",
                        sender_id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn unlock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_desktop_shell#{}.unlock()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The surface set by this request will receive a fake"]
            #[doc = "pointer.enter event during grabs at position 0, 0 and is"]
            #[doc = "expected to set an appropriate cursor image as described by"]
            #[doc = "the grab_cursor event sent just before the enter event."]
            fn set_grab_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.set_grab_surface({})",
                        sender_id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tell the server, that enough desktop elements have been drawn"]
            #[doc = "to make the desktop look ready for use. During start-up, the"]
            #[doc = "server can wait for this request with a black screen before"]
            #[doc = "starting to fade in the desktop, for instance. If the client"]
            #[doc = "parts of a desktop take a long time to initialize, we avoid"]
            #[doc = "showing temporary garbage."]
            fn desktop_ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_desktop_shell#{}.desktop_ready()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tell the shell which side of the screen the panel is"]
            #[doc = "located. This is so that new windows do not overlap the panel"]
            #[doc = "and maximized windows maximize properly."]
            fn set_panel_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                position: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.set_panel_position({})",
                        sender_id,
                        position
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(position).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                edges: u32,
                surface: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Tell the client we want it to create and set the lock surface, which is"]
            #[doc = "a GUI asking the user to unlock the screen. The lock surface is"]
            #[doc = "announced with 'set_lock_surface'. Whether or not the client actually"]
            #[doc = "implements locking, it MUST send 'unlock' request to let the normal"]
            #[doc = "desktop resume."]
            fn prepare_lock_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event will be sent immediately before a fake enter event on the"]
            #[doc = "grab surface."]
            fn grab_cursor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                cursor: u32,
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
                            let edges = message.uint()?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_desktop_shell#{}.configure({}, {}, {}, {})",
                                sender_id,
                                edges,
                                surface,
                                width,
                                height
                            );
                            self.configure(connection, sender_id, edges, surface, width, height)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_desktop_shell#{}.prepare_lock_surface()",
                                sender_id,
                            );
                            self.prepare_lock_surface(connection, sender_id).await
                        }
                        2u16 => {
                            let cursor = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_desktop_shell#{}.grab_cursor({})",
                                sender_id,
                                cursor
                            );
                            self.grab_cursor(connection, sender_id, cursor).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Only one client can bind this interface at a time."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_screensaver {
        #[doc = "Trait to implement the weston_screensaver interface. See the module level documentation for more info"]
        pub trait WestonScreensaver
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_screensaver";
            const VERSION: u32 = 1u32;
            #[doc = "A screensaver surface is normally hidden, and only visible after an"]
            #[doc = "idle timeout."]
            fn set_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_screensaver#{}.set_surface({}, {})",
                        sender_id,
                        surface,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
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
pub mod weston_direct_display {
    #[doc = "Weston extension to instruct the compositor to avoid any import"]
    #[doc = "of the dmabuf created by 'linux-dmabuf' protocol other than the display"]
    #[doc = "controller."]
    #[doc = ""]
    #[doc = "Compositors are already going to use direct scan-out as much as possible but"]
    #[doc = "there's no assurance that while doing so, they won't first import the dmabuf"]
    #[doc = "in to the GPU. This extension assures the client that the compositor will"]
    #[doc = "never attempt to import in to the GPU and pass it directly to the display"]
    #[doc = "controller."]
    #[doc = ""]
    #[doc = "Clients can make use of this extension to pass the dmabuf buffer to the"]
    #[doc = "display controller, potentially increasing the performance and lowering the"]
    #[doc = "bandwidth usage."]
    #[doc = ""]
    #[doc = "Lastly, clients can make use of this extension in tandem with content-protection"]
    #[doc = "one thus avoiding any GPU interaction and providing a secure-content path."]
    #[doc = "Also, in some cases, the memory where dmabuf are allocated are in specially"]
    #[doc = "crafted memory zone which would be seen as an illegal memory access when the"]
    #[doc = "GPU will attempt to read it."]
    #[doc = ""]
    #[doc = "WARNING: This interface by design might break screenshoting functionality"]
    #[doc = "as compositing might be involved while doing that. Also, do note, that in"]
    #[doc = "case the dmabufer provided can't be imported by KMS, the client connection"]
    #[doc = "will be terminated."]
    #[doc = ""]
    #[doc = "WARNING: This extension requires 'linux-dmabuf' protocol and"]
    #[doc = "'zwp_linux_buffer_params_v1' be already created by 'zwp_linux_buffer_v1'."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_direct_display_v1 {
        #[doc = "Trait to implement the weston_direct_display_v1 interface. See the module level documentation for more info"]
        pub trait WestonDirectDisplayV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_direct_display_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request tells the compositor not to import the dmabuf to the GPU"]
            #[doc = "in order to bypass it entirely, such that the buffer will be directly"]
            #[doc = "scanned-out by the display controller. If HW is not capable/or there"]
            #[doc = "aren't any available resources to directly scan-out the buffer, a"]
            #[doc = "placeholder should be installed in-place by the compositor. The"]
            #[doc = "compositor may perform checks on the dmabuf and refuse to create a"]
            #[doc = "wl_buffer if the dmabuf seems unusable for being used directly."]
            #[doc = ""]
            #[doc = "Assumes that 'zwp_linux_buffer_params_v1' was already created"]
            #[doc = "by 'zwp_linux_dmabuf_v1_create_params'."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                dmabuf: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_direct_display_v1#{}.enable({})",
                        sender_id,
                        dmabuf
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(dmabuf))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroys the factory object, but does not affect any other objects."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_direct_display_v1#{}.destroy()", sender_id,);
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
pub mod weston_output_capture {
    #[doc = "The global interface exposing Weston screenshooting functionality"]
    #[doc = "intended for single shots."]
    #[doc = ""]
    #[doc = "This is a privileged inteface."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_capture_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid source enum value"]
            InvalidSource = 0u32,
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
                    0u32 => Ok(Self::InvalidSource),
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
        pub enum Source {
            #[doc = "use hardware writeback"]
            Writeback = 0u32,
            #[doc = "copy from framebuffer, desktop area"]
            Framebuffer = 1u32,
            #[doc = "copy whole framebuffer, including borders"]
            FullFramebuffer = 2u32,
            #[doc = "copy from blending space"]
            Blending = 3u32,
        }
        impl From<Source> for u32 {
            fn from(value: Source) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Source {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Writeback),
                    1u32 => Ok(Self::Framebuffer),
                    2u32 => Ok(Self::FullFramebuffer),
                    3u32 => Ok(Self::Blending),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_capture_v1 interface. See the module level documentation for more info"]
        pub trait WestonCaptureV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_capture_v1";
            const VERSION: u32 = 2u32;
            #[doc = "Affects no other protocol objects in any way."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_capture_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This creates a weston_capture_source_v1 object corresponding to the"]
            #[doc = "given wl_output. The object delivers information for allocating"]
            #[doc = "suitable buffers, and exposes the capture function."]
            #[doc = ""]
            #[doc = "The object will be using the given pixel source for capturing images."]
            #[doc = "If the source is not available, all attempts to capture will fail"]
            #[doc = "gracefully."]
            #[doc = ""]
            #[doc = "'writeback' source will use hardware writeback feature of DRM KMS for"]
            #[doc = "capturing. This may allow hardware planes to remain used"]
            #[doc = "during the capture. This source is often not available."]
            #[doc = ""]
            #[doc = "'framebuffer' source copies the contents of the final framebuffer."]
            #[doc = "Using this source temporarily disables all use of hardware planes and"]
            #[doc = "DRM KMS color pipeline features. This source is always available."]
            #[doc = ""]
            #[doc = "'full_framebuffer' is otherwise the same as 'framebuffer' except it"]
            #[doc = "will include also any borders (decorations) that the framebuffer may"]
            #[doc = "contain."]
            #[doc = ""]
            #[doc = "'blending' source copies the contents of the intermediate blending"]
            #[doc = "buffer, which should be in linear-light format.  Using this source"]
            #[doc = "temporarily disables all use of hardware planes. This source is only"]
            #[doc = "available when a blending buffer exists, e.g. when color management"]
            #[doc = "is active on the output."]
            #[doc = ""]
            #[doc = "If the pixel source is not one of the defined enumeration values,"]
            #[doc = "'invalid_source' protocol error is raised."]
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
                source: Source,
                capture_source_new_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_capture_v1#{}.create({}, {}, {})",
                        sender_id,
                        output,
                        source,
                        capture_source_new_id
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
                        .put_uint(source.into())
                        .put_object(Some(capture_source_new_id))
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
    #[doc = "An object representing image capturing functionality for a single"]
    #[doc = "source. When created, it sends the initial events if and only if the"]
    #[doc = "output still exists and the specified pixel source is available on"]
    #[doc = "the output."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_capture_source_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the wl_buffer is not writable"]
            BadBuffer = 0u32,
            #[doc = "capture requested again before previous retired"]
            Sequence = 1u32,
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
                    0u32 => Ok(Self::BadBuffer),
                    1u32 => Ok(Self::Sequence),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_capture_source_v1 interface. See the module level documentation for more info"]
        pub trait WestonCaptureSourceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_capture_source_v1";
            const VERSION: u32 = 2u32;
            #[doc = "If a capture is on-going on this object, this will cancel it and"]
            #[doc = "make the image buffer contents undefined."]
            #[doc = ""]
            #[doc = "This object is destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_capture_source_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "If the given wl_buffer is compatible, the associated output will go"]
            #[doc = "through a repaint some time after this request has been processed,"]
            #[doc = "and that repaint will execute the capture."]
            #[doc = "Once the capture is complete, 'complete' event is emitted."]
            #[doc = ""]
            #[doc = "If the given wl_buffer is incompatible, the event 'retry' is"]
            #[doc = "emitted."]
            #[doc = ""]
            #[doc = "If the capture fails or the buffer type is unsupported, the event"]
            #[doc = "'failed' is emitted."]
            #[doc = ""]
            #[doc = "The client must wait for one of these events before attempting"]
            #[doc = "'capture' on this object again. If 'capture' is requested again before"]
            #[doc = "any of those events, 'sequence' protocol error is raised."]
            #[doc = ""]
            #[doc = "The wl_buffer object will not emit wl_buffer.release event due to"]
            #[doc = "this request."]
            #[doc = ""]
            #[doc = "The wl_buffer must refer to compositor-writable storage. If buffer"]
            #[doc = "storage is not writable, either the protocol error bad_buffer or"]
            #[doc = "wl_shm.error.invalid_fd is raised."]
            #[doc = ""]
            #[doc = "If the wl_buffer is destroyed before any event is emitted, the buffer"]
            #[doc = "contents become undefined."]
            #[doc = ""]
            #[doc = "A compositor is required to implement capture into wl_shm buffers."]
            #[doc = "Other buffer types may or may not be supported."]
            fn capture(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_capture_source_v1#{}.capture({})",
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
            #[doc = "This event delivers one pixel format that can be used for the"]
            #[doc = "image buffer. Any buffer is incompatible if it does not have"]
            #[doc = "a pixel format delivered by one of this events."]
            #[doc = ""]
            #[doc = "The format modifier is linear (DRM_FORMAT_MOD_LINEAR)."]
            #[doc = ""]
            #[doc = "This is an initial event, and sent whenever the supported formats"]
            #[doc = "change."]
            #[doc = ""]
            #[doc = "This event may be send multiple times, followed by a format_done event."]
            fn format(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                drm_format: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event delivers the size that should be used for the"]
            #[doc = "image buffer. Any buffer is incompatible if it does not have"]
            #[doc = "this size."]
            #[doc = ""]
            #[doc = "For wl_shm the row alignment of the buffer must be 4 bytes, and it must"]
            #[doc = "not contain further row padding. Otherwise the buffer is unsupported."]
            #[doc = ""]
            #[doc = "This is an initial event, and sent whenever the required size"]
            #[doc = "changes."]
            fn size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "has successfully completed."]
            #[doc = ""]
            #[doc = "If the buffer used in the shot is a dmabuf, the client also needs to"]
            #[doc = "wait for any implicit fences on it before accessing the contents."]
            fn complete(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "cannot succeed due to an incompatible buffer. The client has already"]
            #[doc = "received the events delivering the new buffer parameters. The client"]
            #[doc = "should retry the capture with the new buffer parameters."]
            fn retry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "has failed for reasons other than an incompatible buffer. The reasons"]
            #[doc = "may include: unsupported buffer type, unsupported buffer stride,"]
            #[doc = "unsupported image source, the image source (output) was removed, or"]
            #[doc = "compositor policy denied the capture."]
            #[doc = ""]
            #[doc = "The string 'msg' may contain a human-readable explanation of the"]
            #[doc = "failure to aid debugging."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                msg: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is sent after all formats have been sent."]
            fn formats_done(
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
                            let drm_format = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_capture_source_v1#{}.format({})",
                                sender_id,
                                drm_format
                            );
                            self.format(connection, sender_id, drm_format).await
                        }
                        1u16 => {
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_capture_source_v1#{}.size({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.size(connection, sender_id, width, height).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("weston_capture_source_v1#{}.complete()", sender_id,);
                            self.complete(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("weston_capture_source_v1#{}.retry()", sender_id,);
                            self.retry(connection, sender_id).await
                        }
                        4u16 => {
                            let msg = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_capture_source_v1#{}.failed(\"{}\")",
                                sender_id,
                                msg.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.failed(connection, sender_id, msg).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_capture_source_v1#{}.formats_done()",
                                sender_id,
                            );
                            self.formats_done(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_test {
    #[doc = "Internal testing facilities for the weston compositor."]
    #[doc = ""]
    #[doc = "It can't be stressed enough that these should never ever be used"]
    #[doc = "outside of running weston's tests.  The weston-test.so module should"]
    #[doc = "never be installed."]
    #[doc = ""]
    #[doc = "These requests may allow clients to do very bad things."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_test {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid coordinate"]
            TouchUpWithCoordinate = 0u32,
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
                    0u32 => Ok(Self::TouchUpWithCoordinate),
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
        pub enum Breakpoint {
            #[doc = "after output repaint (filter type: wl_output)"]
            PostRepaint = 0u32,
            #[doc = "after output latch (filter type: wl_output)"]
            PostLatch = 1u32,
        }
        impl From<Breakpoint> for u32 {
            fn from(value: Breakpoint) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Breakpoint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PostRepaint),
                    1u32 => Ok(Self::PostLatch),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Breakpoint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_test interface. See the module level documentation for more info"]
        pub trait WestonTest
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_test";
            const VERSION: u32 = 1u32;
            fn move_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.move_surface({}, {}, {})",
                        sender_id,
                        surface,
                        x,
                        y
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .put_int(x)
                        .put_int(y)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn move_pointer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.move_pointer({}, {}, {}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec,
                        x,
                        y
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .put_int(x)
                        .put_int(y)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn send_button(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                button: i32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.send_button({}, {}, {}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec,
                        button,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .put_int(button)
                        .put_uint(state)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn send_axis(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                axis: u32,
                value: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.send_axis({}, {}, {}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec,
                        axis,
                        value
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .put_uint(axis)
                        .put_fixed(value)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn activate_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.activate_surface({})",
                        sender_id,
                        surface
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_object(surface).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn send_key(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                key: u32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.send_key({}, {}, {}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec,
                        key,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .put_uint(key)
                        .put_uint(state)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn device_release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                device: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.device_release(\"{}\")",
                        sender_id,
                        device
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(device))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn device_add(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                device: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_test#{}.device_add(\"{}\")", sender_id, device);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(device))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn send_touch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                touch_id: i32,
                x: waynest::Fixed,
                y: waynest::Fixed,
                touch_type: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.send_touch({}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec,
                        touch_id,
                        x,
                        y,
                        touch_type
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .put_int(touch_id)
                        .put_fixed(x)
                        .put_fixed(y)
                        .put_uint(touch_type)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the compositor pauses execution at a certain point. When"]
            #[doc = "execution is paused, the compositor will signal the shared semaphore"]
            #[doc = "to the client."]
            fn client_break(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                breakpoint: Breakpoint,
                resource_id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_test#{}.client_break({}, {})",
                        sender_id,
                        breakpoint,
                        resource_id
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(breakpoint.into())
                        .put_uint(resource_id)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn pointer_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: waynest::Fixed,
                y: waynest::Fixed,
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
                            let x = message.fixed()?;
                            let y = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_test#{}.pointer_position({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.pointer_position(connection, sender_id, x, y).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This is a global singleton interface for Weston internal tests."]
    #[doc = ""]
    #[doc = "This interface allows a test client to trigger compositor-side"]
    #[doc = "test procedures. This is useful for cases, where the actual tests"]
    #[doc = "are in compositor plugins, but they also require the presence of"]
    #[doc = "a particular client."]
    #[doc = ""]
    #[doc = "This interface is implemented by the compositor plugins containing"]
    #[doc = "the testing code."]
    #[doc = ""]
    #[doc = "A test client starts a test with the \"run\" request. It must not send"]
    #[doc = "another \"run\" request until receiving the \"finished\" event. If the"]
    #[doc = "compositor-side test succeeds, the \"finished\" event is sent. If the"]
    #[doc = "compositor-side test fails, the compositor should send the protocol"]
    #[doc = "error \"test_failed\", but it may also exit with an error (e.g. SEGV)."]
    #[doc = ""]
    #[doc = "Unknown test name will raise \"unknown_test\" protocol error."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_test_runner {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "compositor test failed"]
            TestFailed = 0u32,
            #[doc = "unrecognized test name"]
            UnknownTest = 1u32,
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
                    0u32 => Ok(Self::TestFailed),
                    1u32 => Ok(Self::UnknownTest),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_test_runner interface. See the module level documentation for more info"]
        pub trait WestonTestRunner
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_test_runner";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_test_runner#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn run(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                test_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_test_runner#{}.run(\"{}\")", sender_id, test_name);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(test_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
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
                            #[cfg(feature = "tracing")]
                            tracing::debug!("weston_test_runner#{}.finished()", sender_id,);
                            self.finished(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_touch_calibration {
    #[doc = "This is the global interface for calibrating a touchscreen input"]
    #[doc = "coordinate transformation. It is recommended to make this interface"]
    #[doc = "privileged."]
    #[doc = ""]
    #[doc = "This interface can be used by a client to show a calibration pattern and"]
    #[doc = "receive uncalibrated touch coordinates, facilitating the computation of"]
    #[doc = "a calibration transformation that will align actual touch positions"]
    #[doc = "on screen with their expected coordinates."]
    #[doc = ""]
    #[doc = "Immediately after being bound by a client, the compositor sends the"]
    #[doc = "touch_device events."]
    #[doc = ""]
    #[doc = "The client chooses a touch device from the touch_device events, creates a"]
    #[doc = "wl_surface and then a weston_touch_calibrator for the wl_surface and the"]
    #[doc = "chosen touch device. The client waits for the compositor to send a"]
    #[doc = "configure event before it starts drawing the first calibration pattern."]
    #[doc = "After receiving the configure event, the client will iterate drawing a"]
    #[doc = "pattern, getting touch input via weston_touch_calibrator, and converting"]
    #[doc = "pixel coordinates to expected touch coordinates with"]
    #[doc = "weston_touch_calibrator.convert until it has enough correspondences to"]
    #[doc = "compute the calibration transformation or the compositor cancels the"]
    #[doc = "calibration."]
    #[doc = ""]
    #[doc = "Once the client has successfully computed a new calibration, it can use"]
    #[doc = "weston_touch_calibration.save request to load the new calibration into"]
    #[doc = "the compositor. The compositor may take this new calibration into use and"]
    #[doc = "may write it into persistent storage."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_touch_calibration {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the given wl_surface already has a role"]
            InvalidSurface = 0u32,
            #[doc = "the given device is not valid"]
            InvalidDevice = 1u32,
            #[doc = "a calibrator has already been created"]
            AlreadyExists = 2u32,
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
                    0u32 => Ok(Self::InvalidSurface),
                    1u32 => Ok(Self::InvalidDevice),
                    2u32 => Ok(Self::AlreadyExists),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_touch_calibration interface. See the module level documentation for more info"]
        pub trait WestonTouchCalibration
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_touch_calibration";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the binding to the global interface, does not affect any"]
            #[doc = "objects already created through this interface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_touch_calibration#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This gives the calibrator role to the surface and ties it with the"]
            #[doc = "given touch input device."]
            #[doc = ""]
            #[doc = "If the surface already has a role, then invalid_surface error is raised."]
            #[doc = ""]
            #[doc = "If the device string is not one advertised with touch_device event's"]
            #[doc = "device argument, then invalid_device error is raised."]
            #[doc = ""]
            #[doc = "If a weston_touch_calibrator protocol object exists in the compositor"]
            #[doc = "already, then already_exists error is raised. This limitation is"]
            #[doc = "compositor-wide and not specific to any particular client."]
            fn create_calibrator(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                device: String,
                cal: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_touch_calibration#{}.create_calibrator({}, \"{}\", {})",
                        sender_id,
                        surface,
                        device,
                        cal
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .put_string(Some(device))
                        .put_object(Some(cal))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request asks the compositor to save the calibration data for the"]
            #[doc = "given touch input device. The compositor may ignore this request."]
            #[doc = ""]
            #[doc = "If the device string is not one advertised with touch_device event's"]
            #[doc = "device argument, then invalid_device error is raised."]
            #[doc = ""]
            #[doc = "The array must contain exactly six 'float' (the 32-bit floating"]
            #[doc = "point format used by the C language on the system) numbers. For a 3x3"]
            #[doc = "calibration matrix in the form"]
            #[doc = "@code"]
            #[doc = "( a b c )"]
            #[doc = "( d e f )"]
            #[doc = "( 0 0 1 )"]
            #[doc = "@endcode"]
            #[doc = "the array must contain the values { a, b, c, d, e, f }. For the"]
            #[doc = "definition of the coordinate spaces, see"]
            #[doc = "libinput_device_config_calibration_set_matrix()."]
            fn save(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                device: String,
                matrix: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_touch_calibration#{}.save(\"{}\", array[{}])",
                        sender_id,
                        device,
                        matrix.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(device))
                        .put_array(matrix)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "When a client binds to weston_touch_calibration, one touch_device event"]
            #[doc = "is sent for each touchscreen that is available to be calibrated. This"]
            #[doc = "is the only time the event is sent. Touch devices added in the"]
            #[doc = "compositor will not generate events for existing"]
            #[doc = "weston_touch_calibration objects."]
            #[doc = ""]
            #[doc = "An event carries the touch device identification and the associated"]
            #[doc = "output or head (display connector) name."]
            #[doc = ""]
            #[doc = "On platforms using udev, the device identification is the udev sys"]
            #[doc = "path. It is an absolute path and starts with the sys mount point."]
            fn touch_device(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                device: String,
                head: String,
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
                            let device = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let head = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibration#{}.touch_device(\"{}\", \"{}\")",
                                sender_id,
                                device,
                                head
                            );
                            self.touch_device(connection, sender_id, device, head).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "On creation, this object is tied to a specific touch device. The"]
    #[doc = "compositor sends a configure event which the client must obey with the"]
    #[doc = "associated wl_surface."]
    #[doc = ""]
    #[doc = "Once the client has committed content to the surface, the compositor can"]
    #[doc = "grab the touch input device, prevent it from emitting normal touch"]
    #[doc = "events, show the surface on the correct output, and relay input events"]
    #[doc = "from the touch device via this protocol object."]
    #[doc = ""]
    #[doc = "Touch events from other touch devices than the one tied to this object"]
    #[doc = "must generate wrong_touch events on at least touch-down and must not"]
    #[doc = "generate normal or calibration touch events."]
    #[doc = ""]
    #[doc = "At any time, the compositor can choose to cancel the calibration"]
    #[doc = "procedure by sending the cancel_calibration event. This should also be"]
    #[doc = "used if the touch device disappears or anything else prevents the"]
    #[doc = "calibration from continuing on the compositor side."]
    #[doc = ""]
    #[doc = "If the wl_surface is destroyed, the compositor must cancel the"]
    #[doc = "calibration."]
    #[doc = ""]
    #[doc = "The touch event coordinates and conversion results are delivered in"]
    #[doc = "calibration units. The calibration units cover the device coordinate"]
    #[doc = "range exactly. Calibration units are in the closed interval [0.0, 1.0]"]
    #[doc = "mapped into 32-bit unsigned integers. An integer can be converted into a"]
    #[doc = "real value by dividing by 2^32-1. A calibration matrix must be computed"]
    #[doc = "from the [0.0, 1.0] real values, but the matrix elements do not need to"]
    #[doc = "fall into that range."]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_touch_calibrator {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface size does not match"]
            BadSize = 0u32,
            #[doc = "requested operation is not possible without mapping the surface"]
            NotMapped = 1u32,
            #[doc = "surface-local coordinates are out of bounds"]
            BadCoordinates = 2u32,
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
                    0u32 => Ok(Self::BadSize),
                    1u32 => Ok(Self::NotMapped),
                    2u32 => Ok(Self::BadCoordinates),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_touch_calibrator interface. See the module level documentation for more info"]
        pub trait WestonTouchCalibrator
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_touch_calibrator";
            const VERSION: u32 = 1u32;
            #[doc = "This unmaps the surface if it was mapped. The input device grab"]
            #[doc = "is dropped, if it was present. The surface loses its role as a"]
            #[doc = "calibrator."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> weston_touch_calibrator#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request asks the compositor to convert the surface-local"]
            #[doc = "coordinates into the expected touch input coordinates appropriate for"]
            #[doc = "the associated touch device. The intention is that a client uses this"]
            #[doc = "request to convert marker positions that the user is supposed to touch"]
            #[doc = "during calibration."]
            #[doc = ""]
            #[doc = "If the compositor has cancelled the calibration, the conversion result"]
            #[doc = "shall be zeroes and no errors will be raised."]
            #[doc = ""]
            #[doc = "The coordinates given as arguments to this request are relative to"]
            #[doc = "the associated wl_surface."]
            #[doc = ""]
            #[doc = "If a client asks for conversion before it has committed valid"]
            #[doc = "content to the wl_surface, the not_mapped error is raised."]
            #[doc = ""]
            #[doc = "If the coordinates x, y are outside of the wl_surface content, the"]
            #[doc = "bad_coordinates error is raised."]
            fn convert(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                reply: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> weston_touch_calibrator#{}.convert({}, {}, {})",
                        sender_id,
                        x,
                        y,
                        reply
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_object(Some(reply))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event tells the client what size to make the surface. The client"]
            #[doc = "must obey the size exactly on the next commit with a wl_buffer."]
            #[doc = ""]
            #[doc = "This event shall be sent once as a response to creating a"]
            #[doc = "weston_touch_calibrator object."]
            fn configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This is sent when the compositor wants to cancel the calibration and"]
            #[doc = "drop the touch device grab. The compositor unmaps the surface, if it"]
            #[doc = "was mapped."]
            #[doc = ""]
            #[doc = "The weston_touch_calibrator object will not send any more events. The"]
            #[doc = "client should destroy it."]
            fn cancel_calibration(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "For whatever reason, a touch event resulting from a user action cannot"]
            #[doc = "be used for calibration. The client should show feedback to the user"]
            #[doc = "that the touch was rejected."]
            #[doc = ""]
            #[doc = "Possible causes for this event include the user touching a wrong"]
            #[doc = "touchscreen when there are multiple ones present. This is particularly"]
            #[doc = "useful when the touchscreens are cloned and there is no other way to"]
            #[doc = "identify which screen the user should be touching."]
            #[doc = ""]
            #[doc = "Another cause could be a touch device that sends coordinates beyond its"]
            #[doc = "declared range. If motion takes a touch point outside the range, the"]
            #[doc = "compositor should also send 'cancel' event to undo the touch-down."]
            fn invalid_touch(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new touch point has appeared on the surface. This touch point is"]
            #[doc = "assigned a unique ID. Future events from this touch point reference"]
            #[doc = "this ID. The ID ceases to be valid after a touch up event and may be"]
            #[doc = "reused in the future."]
            #[doc = ""]
            #[doc = "For the coordinate units, see weston_touch_calibrator."]
            fn down(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                time: u32,
                id: i32,
                x: u32,
                y: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The touch point has disappeared. No further events will be sent for"]
            #[doc = "this touch point and the touch point's ID is released and may be"]
            #[doc = "reused in a future touch down event."]
            fn up(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                time: u32,
                id: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A touch point has changed coordinates."]
            #[doc = ""]
            #[doc = "For the coordinate units, see weston_touch_calibrator."]
            fn motion(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                time: u32,
                id: i32,
                x: u32,
                y: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Indicates the end of a set of events that logically belong together."]
            #[doc = "A client is expected to accumulate the data in all events within the"]
            #[doc = "frame before proceeding."]
            #[doc = ""]
            #[doc = "A wl_touch.frame terminates at least one event but otherwise no"]
            #[doc = "guarantee is provided about the set of events within a frame. A client"]
            #[doc = "must assume that any state not updated in a frame is unchanged from the"]
            #[doc = "previously known state."]
            fn frame(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sent if the compositor decides the touch stream is a global"]
            #[doc = "gesture. No further events are sent to the clients from that"]
            #[doc = "particular gesture. Touch cancellation applies to all touch points"]
            #[doc = "currently active on this client's surface. The client is"]
            #[doc = "responsible for finalizing the touch points, future touch points on"]
            #[doc = "this surface may reuse the touch point ID."]
            fn cancel(
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
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibrator#{}.configure({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.configure(connection, sender_id, width, height).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibrator#{}.cancel_calibration()",
                                sender_id,
                            );
                            self.cancel_calibration(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibrator#{}.invalid_touch()",
                                sender_id,
                            );
                            self.invalid_touch(connection, sender_id).await
                        }
                        3u16 => {
                            let time = message.uint()?;
                            let id = message.int()?;
                            let x = message.uint()?;
                            let y = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibrator#{}.down({}, {}, {}, {})",
                                sender_id,
                                time,
                                id,
                                x,
                                y
                            );
                            self.down(connection, sender_id, time, id, x, y).await
                        }
                        4u16 => {
                            let time = message.uint()?;
                            let id = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibrator#{}.up({}, {})",
                                sender_id,
                                time,
                                id
                            );
                            self.up(connection, sender_id, time, id).await
                        }
                        5u16 => {
                            let time = message.uint()?;
                            let id = message.int()?;
                            let x = message.uint()?;
                            let y = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_calibrator#{}.motion({}, {}, {}, {})",
                                sender_id,
                                time,
                                id,
                                x,
                                y
                            );
                            self.motion(connection, sender_id, time, id, x, y).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("weston_touch_calibrator#{}.frame()", sender_id,);
                            self.frame(connection, sender_id).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("weston_touch_calibrator#{}.cancel()", sender_id,);
                            self.cancel(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod weston_touch_coordinate {
        #[doc = "Trait to implement the weston_touch_coordinate interface. See the module level documentation for more info"]
        pub trait WestonTouchCoordinate
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "weston_touch_coordinate";
            const VERSION: u32 = 1u32;
            #[doc = "This event returns the conversion result from surface coordinates to"]
            #[doc = "the expected touch device coordinates."]
            #[doc = ""]
            #[doc = "For details, see weston_touch_calibrator.convert. For the coordinate"]
            #[doc = "units, see weston_touch_calibrator."]
            #[doc = ""]
            #[doc = "This event destroys the weston_touch_coordinate object."]
            fn result(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: u32,
                y: u32,
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
                            let x = message.uint()?;
                            let y = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "weston_touch_coordinate#{}.result({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.result(connection, sender_id, x, y).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
