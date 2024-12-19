#![allow(async_fn_in_trait)]
#[allow(clippy::module_inception)]
pub mod ivi_application {
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_surface {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ivi_surface interface. See the module level documentation for more info"]
        pub trait IviSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_surface";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ivi_surface#{}.destroy()", object.id,);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This removes the link from ivi_id to wl_surface and destroys ivi_surface."]
            #[doc = "The ID, ivi_id, is free and can be used for surface_create again."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            #[doc = "in surface local coordinates."]
            async fn configure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_surface#{}.configure({}, {})",
                    object.id,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "This interface is exposed as a global singleton."]
    #[doc = "This interface is implemented by servers that provide IVI-style user interfaces."]
    #[doc = "It allows clients to associate an ivi_surface with wl_surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_application {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "given ivi_id is assigned to another wl_surface"]
            IviId = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::IviId),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_application interface. See the module level documentation for more info"]
        pub trait IviApplication: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_application";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let ivi_id = message.uint()?;
                        let surface = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ivi_application#{}.surface_create({}, {}, {})",
                            object.id,
                            ivi_id,
                            surface,
                            id
                        );
                        self.surface_create(object, client, ivi_id, surface, id)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
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
            #[doc = "If client destroys ivi_surface or wl_surface which is assigne to the ivi_surface,"]
            #[doc = "ivi_id which is assigned to the ivi_surface is free for reuse."]
            async fn surface_create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                ivi_id: u32,
                surface: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ivi_input {
    #[doc = "This includes handling the existence of seats, seat capabilities,"]
    #[doc = "seat acceptance and input focus."]
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_input {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ivi_input interface. See the module level documentation for more info"]
        pub trait IviInput: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_input";
            const VERSION: u32 = 2u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let surface = message.uint()?;
                        let device = message.uint()?;
                        let enabled = message.int()?;
                        tracing::debug!(
                            "ivi_input#{}.set_input_focus({}, {}, {})",
                            object.id,
                            surface,
                            device,
                            enabled
                        );
                        self.set_input_focus(object, client, surface, device, enabled)
                            .await
                    }
                    1u16 => {
                        let surface = message.uint()?;
                        let seat = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let accepted = message.int()?;
                        tracing::debug!(
                            "ivi_input#{}.set_input_acceptance({}, \"{}\", {})",
                            object.id,
                            surface,
                            seat,
                            accepted
                        );
                        self.set_input_acceptance(object, client, surface, seat, accepted)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Set input focus state of surface in ivi compositor. If the surface has input"]
            #[doc = "focus, all non-graphical inputs (e.g. keyboard) are directed to the application"]
            #[doc = "providing the content for this surface."]
            #[doc = "Multiple surfaces can have input focus at a time."]
            #[doc = "If argument enabled is ILM_TRUE, input focus for this surface is enabled."]
            #[doc = "If argument enabled is not ILM_TRUE, the input focus from this surface is removed."]
            async fn set_input_focus(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: u32,
                device: u32,
                enabled: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Set input acceptance of one seat for a surface. Surfaces may"]
            #[doc = "accept input acceptance from multiple seats at once."]
            #[doc = "If argument 'accepted' is ILM_TRUE, the given seat's name will"]
            #[doc = "be added to the list of accepted seats."]
            #[doc = "If argument 'accepted' is not ILM_TRUE, the given seat's name"]
            #[doc = "will be removed from the list of accepted seats."]
            async fn set_input_acceptance(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: u32,
                seat: String,
                accepted: i32,
            ) -> crate::server::Result<()>;
            async fn seat_created(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                capabilities: u32,
                is_default: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_input#{}.seat_created(\"{}\", {}, {})",
                    object.id,
                    name,
                    capabilities,
                    is_default
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .put_uint(capabilities)
                    .put_int(is_default)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn seat_capabilities(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                capabilities: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_input#{}.seat_capabilities(\"{}\", {})",
                    object.id,
                    name,
                    capabilities
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .put_uint(capabilities)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn seat_destroyed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_input#{}.seat_destroyed(\"{}\")", object.id, name);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The new input focus state is provided in argument enabled:"]
            #[doc = "If enabled is ILM_TRUE, this surface now has input focus enabled."]
            #[doc = "If enabled is not ILM_TRUE, this surface no longer has input focus."]
            async fn input_focus(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: u32,
                device: u32,
                enabled: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_input#{}.input_focus({}, {}, {})",
                    object.id,
                    surface,
                    device,
                    enabled
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface)
                    .put_uint(device)
                    .put_int(enabled)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A surface has changed its input acceptance for a specific seat."]
            #[doc = "If argument 'accepted' is ILM_TRUE, the surface now accepts"]
            #[doc = "the seat."]
            #[doc = "If argument 'accepted' is not ILM_TRUE, the surface no longer"]
            #[doc = "accepts the seat."]
            async fn input_acceptance(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: u32,
                seat: String,
                accepted: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_input#{}.input_acceptance({}, \"{}\", {})",
                    object.id,
                    surface,
                    seat,
                    accepted
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface)
                    .put_string(Some(seat))
                    .put_int(accepted)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ivi_wm {
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_wm_screen {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the layer with given id does not exist"]
            NoLayer = 0u32,
            #[doc = "the output is already destroyed"]
            NoScreen = 1u32,
            #[doc = "the given parameter is not valid"]
            BadParam = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoLayer),
                    1u32 => Ok(Self::NoScreen),
                    2u32 => Ok(Self::BadParam),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_wm_screen interface. See the module level documentation for more info"]
        pub trait IviWmScreen: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_wm_screen";
            const VERSION: u32 = 2u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ivi_wm_screen#{}.destroy()", object.id,);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("ivi_wm_screen#{}.clear()", object.id,);
                        self.clear(object, client).await
                    }
                    2u16 => {
                        let layer_id = message.uint()?;
                        tracing::debug!("ivi_wm_screen#{}.add_layer({})", object.id, layer_id);
                        self.add_layer(object, client, layer_id).await
                    }
                    3u16 => {
                        let layer_id = message.uint()?;
                        tracing::debug!("ivi_wm_screen#{}.remove_layer({})", object.id, layer_id);
                        self.remove_layer(object, client, layer_id).await
                    }
                    4u16 => {
                        let buffer = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let screenshot = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ivi_wm_screen#{}.screenshot({}, {})",
                            object.id,
                            buffer,
                            screenshot
                        );
                        self.screenshot(object, client, buffer, screenshot).await
                    }
                    5u16 => {
                        let param = message.int()?;
                        tracing::debug!("ivi_wm_screen#{}.get({})", object.id, param);
                        self.get(object, client, param).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Request to destroy the ivi_wm_screen."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "A screen has no content assigned to itself, it is a container for layers."]
            #[doc = "This request removes all layers from the screen render order."]
            #[doc = "Note: the layers are not destroyed, they are just no longer contained by"]
            #[doc = "the screen."]
            async fn clear(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "A screen has no content assigned to itself, it is a container for layers."]
            #[doc = "This request adds a layers to the topmost position of the screen render order."]
            #[doc = "The added layer will cover all other layers of the screen."]
            async fn add_layer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "A screen has no content assigned to itself, it is a container for layers."]
            #[doc = "This request removes a layer."]
            async fn remove_layer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "An ivi_screenshot object is created which will receive the screenshot"]
            #[doc = "data of the specified output."]
            async fn screenshot(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
                screenshot: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor sends the requested parameter."]
            async fn get(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                param: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sent immediately after creating the ivi_wm_screen object."]
            async fn screen_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_wm_screen#{}.screen_id({})", object.id, id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(id).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A layer is added to the render order lisf of the screen"]
            async fn layer_added(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_wm_screen#{}.layer_added({})", object.id, layer_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent immediately after creating the ivi_wm_screen object."]
            async fn connector_name(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                process_name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm_screen#{}.connector_name(\"{}\")",
                    object.id,
                    process_name
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(process_name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The error event is sent out when an error has occurred."]
            async fn error(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                error: u32,
                message: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm_screen#{}.error({}, \"{}\")",
                    object.id,
                    error,
                    message
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(error)
                    .put_string(Some(message))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "An ivi_screenshot object receives a single \"done\" or \"error\" event."]
    #[doc = "The server will destroy this resource after the event has been send,"]
    #[doc = "so the client shall then destroy its proxy too."]
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_screenshot {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "screenshot file could not be created"]
            IoError = 0u32,
            #[doc = "screenshot can not be read"]
            NotSupported = 1u32,
            #[doc = "output has been destroyed"]
            NoOutput = 2u32,
            #[doc = "surface has been destroyed"]
            NoSurface = 3u32,
            #[doc = "surface has no content"]
            NoContent = 4u32,
            #[doc = "bad buffer input"]
            BadBuffer = 5u32,
            #[doc = "internal allocation failed"]
            NoMemory = 6u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::IoError),
                    1u32 => Ok(Self::NotSupported),
                    2u32 => Ok(Self::NoOutput),
                    3u32 => Ok(Self::NoSurface),
                    4u32 => Ok(Self::NoContent),
                    5u32 => Ok(Self::BadBuffer),
                    6u32 => Ok(Self::NoMemory),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_screenshot interface. See the module level documentation for more info"]
        pub trait IviScreenshot: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_screenshot";
            const VERSION: u32 = 2u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                _object: &crate::server::Object,
                _client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This event notifies the filling data to buffer is done. The client"]
            #[doc = "can handle the buffer. This also provide the time of dumping data."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                timestamp: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_screenshot#{}.done({})", object.id, timestamp);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(timestamp)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The error event is sent when the screenshot could not be created."]
            async fn error(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                error: Error,
                message: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_screenshot#{}.error({}, \"{}\")",
                    object.id,
                    error,
                    message
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(error as u32)
                    .put_string(Some(message))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_wm {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Sync {
            Add = 0u32,
            Remove = 1u32,
        }
        impl TryFrom<u32> for Sync {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Add),
                    1u32 => Ok(Self::Remove),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Sync {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "The HMI controller can request different types of parameters of an"] # [doc = "ivi-object."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Param : u32 { const Opacity = 1u32 ; const Visibility = 2u32 ; const Size = 4u32 ; const RenderOrder = 8u32 ; } }
        impl TryFrom<u32> for Param {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Param {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "If a surface is restricted type, visible contents of the surface is strictly"]
        #[doc = "controlled by the compositor. Its content is not allowed to be go out of"]
        #[doc = "its destination region. If the application resizes its buffers or uses"]
        #[doc = "wp_viewporter protocol to scale its contents, the old destination region"]
        #[doc = "would causes visible glitches."]
        #[doc = "To avoid these issues, the controller process mark a surface as desktop"]
        #[doc = "compatible. Source and destination regions of a desktop compatible"]
        #[doc = "surface will be modified accordingly,when application sends a request"]
        #[doc = "for resizing or scaling its contents. Therefore, applications contents"]
        #[doc = "will be drawn according to application's wishes."]
        #[doc = "On the other hand, source and destination regions will be strictly"]
        #[doc = "enforced, when the surface's type is restricted. The default type for"]
        #[doc = "a surface is ivi."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum SurfaceType {
            #[doc = "strictly controlled"]
            Restricted = 0u32,
            #[doc = "free to resize and scale"]
            Desktop = 1u32,
        }
        impl TryFrom<u32> for SurfaceType {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Restricted),
                    1u32 => Ok(Self::Desktop),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for SurfaceType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum SurfaceError {
            #[doc = "the surface with given id does not exist"]
            NoSurface = 0u32,
            #[doc = "the given parameter is not valid"]
            BadParam = 1u32,
            #[doc = "the request is not supported"]
            NotSupported = 2u32,
        }
        impl TryFrom<u32> for SurfaceError {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoSurface),
                    1u32 => Ok(Self::BadParam),
                    2u32 => Ok(Self::NotSupported),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for SurfaceError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum LayerError {
            #[doc = "the surface with given id does not exist"]
            NoSurface = 0u32,
            #[doc = "the layer with given id does not exist"]
            NoLayer = 1u32,
            #[doc = "the given parameter is not valid"]
            BadParam = 2u32,
        }
        impl TryFrom<u32> for LayerError {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoSurface),
                    1u32 => Ok(Self::NoLayer),
                    2u32 => Ok(Self::BadParam),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for LayerError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_wm interface. See the module level documentation for more info"]
        pub trait IviWm: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_wm";
            const VERSION: u32 = 2u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ivi_wm#{}.commit_changes()", object.id,);
                        self.commit_changes(object, client).await
                    }
                    1u16 => {
                        let output = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("ivi_wm#{}.create_screen({}, {})", object.id, output, id);
                        self.create_screen(object, client, output, id).await
                    }
                    2u16 => {
                        let surface_id = message.uint()?;
                        let visibility = message.uint()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_surface_visibility({}, {})",
                            object.id,
                            surface_id,
                            visibility
                        );
                        self.set_surface_visibility(object, client, surface_id, visibility)
                            .await
                    }
                    3u16 => {
                        let layer_id = message.uint()?;
                        let visibility = message.uint()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_layer_visibility({}, {})",
                            object.id,
                            layer_id,
                            visibility
                        );
                        self.set_layer_visibility(object, client, layer_id, visibility)
                            .await
                    }
                    4u16 => {
                        let surface_id = message.uint()?;
                        let opacity = message.fixed()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_surface_opacity({}, {})",
                            object.id,
                            surface_id,
                            opacity
                        );
                        self.set_surface_opacity(object, client, surface_id, opacity)
                            .await
                    }
                    5u16 => {
                        let layer_id = message.uint()?;
                        let opacity = message.fixed()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_layer_opacity({}, {})",
                            object.id,
                            layer_id,
                            opacity
                        );
                        self.set_layer_opacity(object, client, layer_id, opacity)
                            .await
                    }
                    6u16 => {
                        let surface_id = message.uint()?;
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_surface_source_rectangle({}, {}, {}, {}, {})",
                            object.id,
                            surface_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.set_surface_source_rectangle(
                            object, client, surface_id, x, y, width, height,
                        )
                        .await
                    }
                    7u16 => {
                        let layer_id = message.uint()?;
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_layer_source_rectangle({}, {}, {}, {}, {})",
                            object.id,
                            layer_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.set_layer_source_rectangle(
                            object, client, layer_id, x, y, width, height,
                        )
                        .await
                    }
                    8u16 => {
                        let surface_id = message.uint()?;
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_surface_destination_rectangle({}, {}, {}, {}, {})",
                            object.id,
                            surface_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.set_surface_destination_rectangle(
                            object, client, surface_id, x, y, width, height,
                        )
                        .await
                    }
                    9u16 => {
                        let layer_id = message.uint()?;
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_layer_destination_rectangle({}, {}, {}, {}, {})",
                            object.id,
                            layer_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.set_layer_destination_rectangle(
                            object, client, layer_id, x, y, width, height,
                        )
                        .await
                    }
                    10u16 => {
                        let surface_id = message.uint()?;
                        let sync_state = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.surface_sync({}, {})",
                            object.id,
                            surface_id,
                            sync_state
                        );
                        self.surface_sync(object, client, surface_id, sync_state)
                            .await
                    }
                    11u16 => {
                        let layer_id = message.uint()?;
                        let sync_state = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.layer_sync({}, {})",
                            object.id,
                            layer_id,
                            sync_state
                        );
                        self.layer_sync(object, client, layer_id, sync_state).await
                    }
                    12u16 => {
                        let surface_id = message.uint()?;
                        let param = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.surface_get({}, {})",
                            object.id,
                            surface_id,
                            param
                        );
                        self.surface_get(object, client, surface_id, param).await
                    }
                    13u16 => {
                        let layer_id = message.uint()?;
                        let param = message.int()?;
                        tracing::debug!("ivi_wm#{}.layer_get({}, {})", object.id, layer_id, param);
                        self.layer_get(object, client, layer_id, param).await
                    }
                    14u16 => {
                        let buffer = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let screenshot = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let surface_id = message.uint()?;
                        tracing::debug!(
                            "ivi_wm#{}.surface_screenshot({}, {}, {})",
                            object.id,
                            buffer,
                            screenshot,
                            surface_id
                        );
                        self.surface_screenshot(object, client, buffer, screenshot, surface_id)
                            .await
                    }
                    15u16 => {
                        let surface_id = message.uint()?;
                        let r#type = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.set_surface_type({}, {})",
                            object.id,
                            surface_id,
                            r#type
                        );
                        self.set_surface_type(object, client, surface_id, r#type)
                            .await
                    }
                    16u16 => {
                        let layer_id = message.uint()?;
                        tracing::debug!("ivi_wm#{}.layer_clear({})", object.id, layer_id);
                        self.layer_clear(object, client, layer_id).await
                    }
                    17u16 => {
                        let layer_id = message.uint()?;
                        let surface_id = message.uint()?;
                        tracing::debug!(
                            "ivi_wm#{}.layer_add_surface({}, {})",
                            object.id,
                            layer_id,
                            surface_id
                        );
                        self.layer_add_surface(object, client, layer_id, surface_id)
                            .await
                    }
                    18u16 => {
                        let layer_id = message.uint()?;
                        let surface_id = message.uint()?;
                        tracing::debug!(
                            "ivi_wm#{}.layer_remove_surface({}, {})",
                            object.id,
                            layer_id,
                            surface_id
                        );
                        self.layer_remove_surface(object, client, layer_id, surface_id)
                            .await
                    }
                    19u16 => {
                        let layer_id = message.uint()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "ivi_wm#{}.create_layout_layer({}, {}, {})",
                            object.id,
                            layer_id,
                            width,
                            height
                        );
                        self.create_layout_layer(object, client, layer_id, width, height)
                            .await
                    }
                    20u16 => {
                        let layer_id = message.uint()?;
                        tracing::debug!("ivi_wm#{}.destroy_layout_layer({})", object.id, layer_id);
                        self.destroy_layout_layer(object, client, layer_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "All requests are not applied directly to scene object, so a controller"]
            #[doc = "can set different properties and apply the changes all at once."]
            #[doc = "Note: there's an exception to this. Creation and destruction of"]
            #[doc = "scene objects is executed immediately."]
            async fn commit_changes(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Ask the ivi-wm to create a ivi-screen for given wl_output."]
            async fn create_screen(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "If visibility argument is 0, the surface in the ivi compositor is set to invisible."]
            #[doc = "If visibility argument is not 0, the surface in the ivi compositor is set to visible."]
            async fn set_surface_visibility(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                visibility: u32,
            ) -> crate::server::Result<()>;
            #[doc = "If visibility argument is 0, the layer in the ivi compositor is set to invisible."]
            #[doc = "If visibility argument is not 0, the layer in the ivi compositor is set to visible."]
            async fn set_layer_visibility(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                visibility: u32,
            ) -> crate::server::Result<()>;
            #[doc = "The valid range for opacity is 0.0 (fully transparent) to 1.0 (fully opaque)."]
            async fn set_surface_opacity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                opacity: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "The valid range for opacity is 0.0 (fully transparent) to 1.0 (fully opaque)."]
            async fn set_layer_opacity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                opacity: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "The source rectangle defines the part of the surface content, that is used for"]
            #[doc = "compositing the surface. It can be used, if valid content of the surface is smaller"]
            #[doc = "than the surface. Effectively it can be used to zoom the content of the surface."]
            #[doc = "If a parameter is less than 0, that value is not changed."]
            #[doc = "x:      horizontal start position of scanout area within the surface"]
            #[doc = "y:      vertical start position of scanout area within the surface"]
            #[doc = "width:  width of scanout area within the surface"]
            #[doc = "height: height of scanout area within the surface"]
            async fn set_surface_source_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "The source rectangle defines the part of the layer content, that is used for"]
            #[doc = "compositing the screen. It can be used, if valid content of the layer is smaller"]
            #[doc = "than the layer. Effectively it can be used to zoom the content of the layer."]
            #[doc = "If a parameter is less than 0, that value is not changed."]
            #[doc = "x:      horizontal start position of scanout area within the layer"]
            #[doc = "y:      vertical start position of scanout area within the layer"]
            #[doc = "width:  width of scanout area within the layer"]
            #[doc = "height: height of scanout area within the layer"]
            async fn set_layer_source_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "The destination rectangle defines the position and size of a surface on a layer."]
            #[doc = "The surface will be scaled to this rectangle for rendering."]
            #[doc = "If a parameter is less than 0, that value is not changed."]
            #[doc = "x:      horizontal start position of surface within the layer"]
            #[doc = "y:      vertical start position of surface within the layer"]
            #[doc = "width : width of surface within the layer"]
            #[doc = "height: height of surface within the layer"]
            async fn set_surface_destination_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "The destination rectangle defines the position and size of a layer on a screen."]
            #[doc = "The layer will be scaled to this rectangle for rendering."]
            #[doc = "If a parameter is less than 0, that value is not changed."]
            #[doc = "x:      horizontal start position of layer within the screen"]
            #[doc = "y:      vertical start position of layer within the screen"]
            #[doc = "width : width of surface within the screen"]
            #[doc = "height: height of surface within the screen"]
            async fn set_layer_destination_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor sends the properties of the surface."]
            #[doc = "If sync_state argument is 0, compositor sends the properties continously."]
            #[doc = "If sync_state argument is not 0, compositor stops sending the properties"]
            #[doc = "continously."]
            async fn surface_sync(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                sync_state: i32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor sends the properties of the layer."]
            #[doc = "If sync_state argument is 0, compositor sends the properties continously."]
            #[doc = "If sync_state argument is not 0, compositor stops sending the properties"]
            #[doc = "continously."]
            async fn layer_sync(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                sync_state: i32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor sends the requested parameter."]
            async fn surface_get(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                param: i32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor sends the requested parameter."]
            async fn layer_get(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                param: i32,
            ) -> crate::server::Result<()>;
            #[doc = "An ivi_screenshot object is created which will receive an image of the"]
            #[doc = "buffer currently attached to the surface with the given id. If there"]
            #[doc = "is no surface with such name the server will respond with an"]
            #[doc = "ivi_screenshot.error event."]
            async fn surface_screenshot(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
                screenshot: crate::wire::ObjectId,
                surface_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor changes the type of the surface."]
            async fn set_surface_type(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                r#type: i32,
            ) -> crate::server::Result<()>;
            #[doc = "A layer has no content assigned to itself, it is a container for surfaces."]
            #[doc = "This request removes all surfaces from the layer render order."]
            async fn layer_clear(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "A layer has no content assigned to itself, it is a container for surfaces."]
            #[doc = "This request adds a surface to the topmost position of the layer render order."]
            #[doc = "The added surface will cover all other surfaces of the layer."]
            async fn layer_add_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                surface_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "A layer has no content assigned to itself, it is a container for surfaces."]
            #[doc = "This request removes one surfaces from the layer render order."]
            #[doc = "Note: the surface is not destroyed, it is just no longer contained by"]
            #[doc = "the layer."]
            async fn layer_remove_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                surface_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor creates an ivi_layout_layer"]
            async fn create_layout_layer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "After this request, compositor destroyes an existing ivi_layout_layer."]
            async fn destroy_layout_layer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "The new visibility state is provided in argument visibility."]
            #[doc = "If visibility is 0, the surface has become invisible."]
            #[doc = "If visibility is not 0, the surface has become visible."]
            async fn surface_visibility(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                visibility: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_visibility({}, {})",
                    object.id,
                    surface_id,
                    visibility
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .put_int(visibility)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The new visibility state is provided in argument visibility."]
            #[doc = "If visibility is 0, the layer has become invisible."]
            #[doc = "If visibility is not 0, the layer has become visible."]
            async fn layer_visibility(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                visibility: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.layer_visibility({}, {})",
                    object.id,
                    layer_id,
                    visibility
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .put_int(visibility)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The new opacity state is provided in argument opacity."]
            #[doc = "The valid range for opactiy is 0.0 (fully transparent) to 1.0 (fully opaque)."]
            async fn surface_opacity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                opacity: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_opacity({}, {})",
                    object.id,
                    surface_id,
                    opacity
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .put_fixed(opacity)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The new opacity state is provided in argument opacity."]
            #[doc = "The valid range for opactiy is 0.0 (fully transparent) to 1.0 (fully opaque)."]
            async fn layer_opacity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                opacity: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.layer_opacity({}, {})",
                    object.id,
                    layer_id,
                    opacity
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .put_fixed(opacity)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The scanout region of the surface content has changed."]
            #[doc = "The new values for source rectangle are provided by"]
            #[doc = "x:      new horizontal start position of scanout area within the surface"]
            #[doc = "y:      new vertical start position of scanout area within the surface"]
            #[doc = "width:  new width of scanout area within the surface"]
            #[doc = "height: new height of scanout area within the surface"]
            async fn surface_source_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_source_rectangle({}, {}, {}, {}, {})",
                    object.id,
                    surface_id,
                    x,
                    y,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The scanout region of the layer content has changed."]
            #[doc = "The new values for source rectangle are provided by"]
            #[doc = "x:      new horizontal start position of scanout area within the layer"]
            #[doc = "y:      new vertical start position of scanout area within the layer"]
            #[doc = "width:  new width of scanout area within the layer"]
            #[doc = "height: new height of scanout area within the layer"]
            async fn layer_source_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.layer_source_rectangle({}, {}, {}, {}, {})",
                    object.id,
                    layer_id,
                    x,
                    y,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The new values for source rectangle are provided by"]
            #[doc = "x:      new horizontal start position of surface within the layer"]
            #[doc = "y:      new vertical start position of surface within the layer"]
            #[doc = "width : new width of surface within the layer"]
            #[doc = "height: new height of surface within the layer"]
            async fn surface_destination_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_destination_rectangle({}, {}, {}, {}, {})",
                    object.id,
                    surface_id,
                    x,
                    y,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The new values for source rectangle are provided by"]
            #[doc = "x:      new horizontal start position of layer within the screen"]
            #[doc = "y:      new vertical start position of layer within the screen"]
            #[doc = "width : new width of layer within the screen"]
            #[doc = "height: new height of layer within the screen"]
            async fn layer_destination_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.layer_destination_rectangle({}, {}, {}, {}, {})",
                    object.id,
                    layer_id,
                    x,
                    y,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn surface_created(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_wm#{}.surface_created({})", object.id, surface_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn layer_created(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_wm#{}.layer_created({})", object.id, layer_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn surface_destroyed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_wm#{}.surface_destroyed({})", object.id, surface_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn layer_destroyed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_wm#{}.layer_destroyed({})", object.id, layer_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 11u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The error event is sent out when an error has occurred."]
            async fn surface_error(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                object_id: u32,
                error: u32,
                message: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_error({}, {}, \"{}\")",
                    object.id,
                    object_id,
                    error,
                    message
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(object_id)
                    .put_uint(error)
                    .put_string(Some(message))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 12u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The error event is sent out when an error has occurred."]
            async fn layer_error(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                object_id: u32,
                error: u32,
                message: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.layer_error({}, {}, \"{}\")",
                    object.id,
                    object_id,
                    error,
                    message
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(object_id)
                    .put_uint(error)
                    .put_string(Some(message))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 13u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The client providing content for this surface modified size of the surface."]
            #[doc = "The modified surface size is provided by arguments width and height."]
            async fn surface_size(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_size({}, {}, {})",
                    object.id,
                    surface_id,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 14u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The information contained in this event is essential for monitoring, debugging,"]
            #[doc = "logging and tracing support in IVI systems."]
            async fn surface_stats(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface_id: u32,
                frame_count: u32,
                pid: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.surface_stats({}, {}, {})",
                    object.id,
                    surface_id,
                    frame_count,
                    pid
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(surface_id)
                    .put_uint(frame_count)
                    .put_uint(pid)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 15u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A surface is added to the render order of the layer"]
            async fn layer_surface_added(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layer_id: u32,
                surface_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_wm#{}.layer_surface_added({}, {})",
                    object.id,
                    layer_id,
                    surface_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(layer_id)
                    .put_uint(surface_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 16u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
