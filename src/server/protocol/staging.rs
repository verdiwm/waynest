#![allow(unused)]
#![allow(async_fn_in_trait)]
pub mod alpha_modifier_v1 {
    #[doc = "This interface allows a client to set a factor for the alpha values on a"]
    #[doc = "surface, which can be used to offload such operations to the compositor,"]
    #[doc = "which can in turn for example offload them to KMS."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_alpha_modifier_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface already has a alpha modifier object"]
            AlreadyConstructed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_alpha_modifier_v1 interface. See the module level documentation for more info"]
        pub trait WpAlphaModifierV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_alpha_modifier_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_alpha_modifier_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("wp_alpha_modifier_v1#{}.get_surface()", object.id);
                        self.get_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the alpha modifier manager. This doesn't destroy objects"]
            #[doc = "created with the manager."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create a new alpha modifier surface object associated with the"]
            #[doc = "given wl_surface. If there is already such an object associated with"]
            #[doc = "the wl_surface, the already_constructed error will be raised."]
            async fn get_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This interface allows the client to set a factor for the alpha values on"]
    #[doc = "a surface, which can be used to offload such operations to the compositor."]
    #[doc = "The default factor is UINT32_MAX."]
    #[doc = ""]
    #[doc = "This object has to be destroyed before the associated wl_surface. Once the"]
    #[doc = "wl_surface is destroyed, all request on this object will raise the"]
    #[doc = "no_surface error."]
    pub mod wp_alpha_modifier_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface was destroyed"]
            NoSurface = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoSurface),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_alpha_modifier_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpAlphaModifierSurfaceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_alpha_modifier_surface_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_alpha_modifier_surface_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_alpha_modifier_surface_v1#{}.set_multiplier()",
                            object.id
                        );
                        self.set_multiplier(object, client, message.uint()?).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This destroys the object, and is equivalent to set_multiplier with"]
            #[doc = "a value of UINT32_MAX, with the same double-buffered semantics as"]
            #[doc = "set_multiplier."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the alpha multiplier for the surface. The alpha multiplier is"]
            #[doc = "double-buffered state, see wl_surface.commit for details."]
            #[doc = ""]
            #[doc = "This factor is applied in the compositor's blending space, as an"]
            #[doc = "additional step after the processing of per-pixel alpha values for the"]
            #[doc = "wl_surface. The exact meaning of the factor is thus undefined, unless"]
            #[doc = "the blending space is specified in a different extension."]
            #[doc = ""]
            #[doc = "This multiplier is applied even if the buffer attached to the"]
            #[doc = "wl_surface doesn't have an alpha channel; in that case an alpha value"]
            #[doc = "of one is used instead."]
            #[doc = ""]
            #[doc = "Zero means completely transparent, UINT32_MAX means completely opaque."]
            async fn set_multiplier(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                factor: u32,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod content_type_v1 {
    #[doc = "This interface allows a client to describe the kind of content a surface"]
    #[doc = "will display, to allow the compositor to optimize its behavior for it."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_content_type_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface already has a content type object"]
            AlreadyConstructed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_content_type_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpContentTypeManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_content_type_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_content_type_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_content_type_manager_v1#{}.get_surface_content_type()",
                            object.id
                        );
                        self.get_surface_content_type(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the content type manager. This doesn't destroy objects created"]
            #[doc = "with the manager."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create a new content type object associated with the given surface."]
            #[doc = ""]
            #[doc = "Creating a wp_content_type_v1 from a wl_surface which already has one"]
            #[doc = "attached is a client error: already_constructed."]
            async fn get_surface_content_type(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "The content type object allows the compositor to optimize for the kind"]
    #[doc = "of content shown on the surface. A compositor may for example use it to"]
    #[doc = "set relevant drm properties like \"content type\"."]
    #[doc = ""]
    #[doc = "The client may request to switch to another content type at any time."]
    #[doc = "When the associated surface gets destroyed, this object becomes inert and"]
    #[doc = "the client should destroy it."]
    pub mod wp_content_type_v1 {
        #[doc = "These values describe the available content types for a surface."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Type {
            None = 0u32,
            Photo = 1u32,
            Video = 2u32,
            Game = 3u32,
        }
        impl TryFrom<u32> for Type {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Photo),
                    2u32 => Ok(Self::Video),
                    3u32 => Ok(Self::Game),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_content_type_v1 interface. See the module level documentation for more info"]
        pub trait WpContentTypeV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_content_type_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_content_type_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("wp_content_type_v1#{}.set_content_type()", object.id);
                        self.set_content_type(object, client, message.uint()?.try_into()?)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Switch back to not specifying the content type of this surface. This is"]
            #[doc = "equivalent to setting the content type to none, including double"]
            #[doc = "buffering semantics. See set_content_type for details."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Set the surface content type. This informs the compositor that the"]
            #[doc = "client believes it is displaying buffers matching this content type."]
            #[doc = ""]
            #[doc = "This is purely a hint for the compositor, which can be used to adjust"]
            #[doc = "its behavior or hardware settings to fit the presented content best."]
            #[doc = ""]
            #[doc = "The content type is double-buffered state, see wl_surface.commit for"]
            #[doc = "details."]
            async fn set_content_type(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                content_type: Type,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod cursor_shape_v1 {
    #[doc = "This global offers an alternative, optional way to set cursor images. This"]
    #[doc = "new way uses enumerated cursors instead of a wl_surface like"]
    #[doc = "wl_pointer.set_cursor does."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_cursor_shape_manager_v1 {
        #[doc = "Trait to implement the wp_cursor_shape_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpCursorShapeManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_cursor_shape_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_cursor_shape_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("wp_cursor_shape_manager_v1#{}.get_pointer()", object.id);
                        self.get_pointer(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "wp_cursor_shape_manager_v1#{}.get_tablet_tool_v2()",
                            object.id
                        );
                        self.get_tablet_tool_v2(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the cursor shape manager."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Obtain a wp_cursor_shape_device_v1 for a wl_pointer object."]
            async fn get_pointer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                cursor_shape_device: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object."]
            async fn get_tablet_tool_v2(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                cursor_shape_device: crate::wire::ObjectId,
                tablet_tool: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This interface allows clients to set the cursor shape."]
    pub mod wp_cursor_shape_device_v1 {
        #[doc = "This enum describes cursor shapes."]
        #[doc = ""]
        #[doc = "The names are taken from the CSS W3C specification:"]
        #[doc = "https://w3c.github.io/csswg-drafts/css-ui/#cursor"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Shape {
            #[doc = "default cursor"]
            Default = 1u32,
            #[doc = "a context menu is available for the object under the cursor"]
            ContextMenu = 2u32,
            #[doc = "help is available for the object under the cursor"]
            Help = 3u32,
            #[doc = "pointer that indicates a link or another interactive element"]
            Pointer = 4u32,
            #[doc = "progress indicator"]
            Progress = 5u32,
            #[doc = "program is busy, user should wait"]
            Wait = 6u32,
            #[doc = "a cell or set of cells may be selected"]
            Cell = 7u32,
            #[doc = "simple crosshair"]
            Crosshair = 8u32,
            #[doc = "text may be selected"]
            Text = 9u32,
            #[doc = "vertical text may be selected"]
            VerticalText = 10u32,
            #[doc = "drag-and-drop: alias of/shortcut to something is to be created"]
            Alias = 11u32,
            #[doc = "drag-and-drop: something is to be copied"]
            Copy = 12u32,
            #[doc = "drag-and-drop: something is to be moved"]
            Move = 13u32,
            #[doc = "drag-and-drop: the dragged item cannot be dropped at the current cursor location"]
            NoDrop = 14u32,
            #[doc = "drag-and-drop: the requested action will not be carried out"]
            NotAllowed = 15u32,
            #[doc = "drag-and-drop: something can be grabbed"]
            Grab = 16u32,
            #[doc = "drag-and-drop: something is being grabbed"]
            Grabbing = 17u32,
            #[doc = "resizing: the east border is to be moved"]
            EResize = 18u32,
            #[doc = "resizing: the north border is to be moved"]
            NResize = 19u32,
            #[doc = "resizing: the north-east corner is to be moved"]
            NeResize = 20u32,
            #[doc = "resizing: the north-west corner is to be moved"]
            NwResize = 21u32,
            #[doc = "resizing: the south border is to be moved"]
            SResize = 22u32,
            #[doc = "resizing: the south-east corner is to be moved"]
            SeResize = 23u32,
            #[doc = "resizing: the south-west corner is to be moved"]
            SwResize = 24u32,
            #[doc = "resizing: the west border is to be moved"]
            WResize = 25u32,
            #[doc = "resizing: the east and west borders are to be moved"]
            EwResize = 26u32,
            #[doc = "resizing: the north and south borders are to be moved"]
            NsResize = 27u32,
            #[doc = "resizing: the north-east and south-west corners are to be moved"]
            NeswResize = 28u32,
            #[doc = "resizing: the north-west and south-east corners are to be moved"]
            NwseResize = 29u32,
            #[doc = "resizing: that the item/column can be resized horizontally"]
            ColResize = 30u32,
            #[doc = "resizing: that the item/row can be resized vertically"]
            RowResize = 31u32,
            #[doc = "something can be scrolled in any direction"]
            AllScroll = 32u32,
            #[doc = "something can be zoomed in"]
            ZoomIn = 33u32,
            #[doc = "something can be zoomed out"]
            ZoomOut = 34u32,
        }
        impl TryFrom<u32> for Shape {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Default),
                    2u32 => Ok(Self::ContextMenu),
                    3u32 => Ok(Self::Help),
                    4u32 => Ok(Self::Pointer),
                    5u32 => Ok(Self::Progress),
                    6u32 => Ok(Self::Wait),
                    7u32 => Ok(Self::Cell),
                    8u32 => Ok(Self::Crosshair),
                    9u32 => Ok(Self::Text),
                    10u32 => Ok(Self::VerticalText),
                    11u32 => Ok(Self::Alias),
                    12u32 => Ok(Self::Copy),
                    13u32 => Ok(Self::Move),
                    14u32 => Ok(Self::NoDrop),
                    15u32 => Ok(Self::NotAllowed),
                    16u32 => Ok(Self::Grab),
                    17u32 => Ok(Self::Grabbing),
                    18u32 => Ok(Self::EResize),
                    19u32 => Ok(Self::NResize),
                    20u32 => Ok(Self::NeResize),
                    21u32 => Ok(Self::NwResize),
                    22u32 => Ok(Self::SResize),
                    23u32 => Ok(Self::SeResize),
                    24u32 => Ok(Self::SwResize),
                    25u32 => Ok(Self::WResize),
                    26u32 => Ok(Self::EwResize),
                    27u32 => Ok(Self::NsResize),
                    28u32 => Ok(Self::NeswResize),
                    29u32 => Ok(Self::NwseResize),
                    30u32 => Ok(Self::ColResize),
                    31u32 => Ok(Self::RowResize),
                    32u32 => Ok(Self::AllScroll),
                    33u32 => Ok(Self::ZoomIn),
                    34u32 => Ok(Self::ZoomOut),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the specified shape value is invalid"]
            InvalidShape = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidShape),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_cursor_shape_device_v1 interface. See the module level documentation for more info"]
        pub trait WpCursorShapeDeviceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_cursor_shape_device_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_cursor_shape_device_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("wp_cursor_shape_device_v1#{}.set_shape()", object.id);
                        self.set_shape(object, client, message.uint()?, message.uint()?.try_into()?)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the cursor shape device."]
            #[doc = ""]
            #[doc = "The device cursor shape remains unchanged."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the device cursor to the specified shape. The compositor will"]
            #[doc = "change the cursor image based on the specified shape."]
            #[doc = ""]
            #[doc = "The cursor actually changes only if the input device focus is one of"]
            #[doc = "the requesting client's surfaces. If any, the previous cursor image"]
            #[doc = "(surface or shape) is replaced."]
            #[doc = ""]
            #[doc = "The \"shape\" argument must be a valid enum entry, otherwise the"]
            #[doc = "invalid_shape protocol error is raised."]
            #[doc = ""]
            #[doc = "This is similar to the wl_pointer.set_cursor and"]
            #[doc = "zwp_tablet_tool_v2.set_cursor requests, but this request accepts a"]
            #[doc = "shape instead of contents in the form of a surface. Clients can mix"]
            #[doc = "set_cursor and set_shape requests."]
            #[doc = ""]
            #[doc = "The serial parameter must match the latest wl_pointer.enter or"]
            #[doc = "zwp_tablet_tool_v2.proximity_in serial number sent to the client."]
            #[doc = "Otherwise the request will be ignored."]
            async fn set_shape(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                shape: Shape,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod drm_lease_v1 {
    #[doc = "This protocol is used by Wayland compositors which act as Direct"]
    #[doc = "Rendering Manager (DRM) masters to lease DRM resources to Wayland"]
    #[doc = "clients."]
    #[doc = ""]
    #[doc = "The compositor will advertise one wp_drm_lease_device_v1 global for each"]
    #[doc = "DRM node. Some time after a client binds to the wp_drm_lease_device_v1"]
    #[doc = "global, the compositor will send a drm_fd event followed by zero, one or"]
    #[doc = "more connector events. After all currently available connectors have been"]
    #[doc = "sent, the compositor will send a wp_drm_lease_device_v1.done event."]
    #[doc = ""]
    #[doc = "When the list of connectors available for lease changes the compositor"]
    #[doc = "will send wp_drm_lease_device_v1.connector events for added connectors and"]
    #[doc = "wp_drm_lease_connector_v1.withdrawn events for removed connectors,"]
    #[doc = "followed by a wp_drm_lease_device_v1.done event."]
    #[doc = ""]
    #[doc = "The compositor will indicate when a device is gone by removing the global"]
    #[doc = "via a wl_registry.global_remove event. Upon receiving this event, the"]
    #[doc = "client should destroy any matching wp_drm_lease_device_v1 object."]
    #[doc = ""]
    #[doc = "To destroy a wp_drm_lease_device_v1 object, the client must first issue"]
    #[doc = "a release request. Upon receiving this request, the compositor will"]
    #[doc = "immediately send a released event and destroy the object. The client must"]
    #[doc = "continue to process and discard drm_fd and connector events until it"]
    #[doc = "receives the released event. Upon receiving the released event, the"]
    #[doc = "client can safely cleanup any client-side resources."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_drm_lease_device_v1 {
        #[doc = "Trait to implement the wp_drm_lease_device_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseDeviceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_device_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "wp_drm_lease_device_v1#{}.create_lease_request()",
                            object.id
                        );
                        self.create_lease_request(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("wp_drm_lease_device_v1#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Creates a lease request object."]
            #[doc = ""]
            #[doc = "See the documentation for wp_drm_lease_request_v1 for details."]
            async fn create_lease_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Indicates the client no longer wishes to use this object. In response"]
            #[doc = "the compositor will immediately send the released event and destroy"]
            #[doc = "this object. It can however not guarantee that the client won't receive"]
            #[doc = "connector events before the released event. The client must not send any"]
            #[doc = "requests after this one, doing so will raise a wl_display error."]
            #[doc = "Existing connectors, lease request and leases will not be affected."]
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The compositor will send this event when the wp_drm_lease_device_v1"]
            #[doc = "global is bound, although there are no guarantees as to how long this"]
            #[doc = "takes - the compositor might need to wait until regaining DRM master."]
            #[doc = "The included fd is a non-master DRM file descriptor opened for this"]
            #[doc = "device and the compositor must not authenticate it."]
            #[doc = "The purpose of this event is to give the client the ability to"]
            #[doc = "query DRM and discover information which may help them pick the"]
            #[doc = "appropriate DRM device or select the appropriate connectors therein."]
            async fn drm_fd(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_device_v1#{}.drm_fd()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fd(fd).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor will use this event to advertise connectors available for"]
            #[doc = "lease by clients. This object may be passed into a lease request to"]
            #[doc = "indicate the client would like to lease that connector, see"]
            #[doc = "wp_drm_lease_request_v1.request_connector for details. While the"]
            #[doc = "compositor will make a best effort to not send disconnected connectors,"]
            #[doc = "no guarantees can be made."]
            #[doc = ""]
            #[doc = "The compositor must send the drm_fd event before sending connectors."]
            #[doc = "After the drm_fd event it will send all available connectors but may"]
            #[doc = "send additional connectors at any time."]
            async fn connector(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_device_v1#{}.connector()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor will send this event to indicate that it has sent all"]
            #[doc = "currently available connectors after the client binds to the global or"]
            #[doc = "when it updates the connector list, for example on hotplug, drm master"]
            #[doc = "change or when a leased connector becomes available again. It will"]
            #[doc = "similarly send this event to group wp_drm_lease_connector_v1.withdrawn"]
            #[doc = "events of connectors of this device."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_device_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent in response to the release request and indicates"]
            #[doc = "that the compositor is done sending connector events."]
            #[doc = "The compositor will destroy this object immediately after sending the"]
            #[doc = "event and it will become invalid. The client should release any"]
            #[doc = "resources associated with this device after receiving this event."]
            async fn released(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_device_v1#{}.released()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "Represents a DRM connector which is available for lease. These objects are"]
    #[doc = "created via wp_drm_lease_device_v1.connector events, and should be passed"]
    #[doc = "to lease requests via wp_drm_lease_request_v1.request_connector."]
    #[doc = "Immediately after the wp_drm_lease_connector_v1 object is created the"]
    #[doc = "compositor will send a name, a description, a connector_id and a done"]
    #[doc = "event. When the description is updated the compositor will send a"]
    #[doc = "description event followed by a done event."]
    pub mod wp_drm_lease_connector_v1 {
        #[doc = "Trait to implement the wp_drm_lease_connector_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseConnectorV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_connector_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_drm_lease_connector_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "The client may send this request to indicate that it will not use this"]
            #[doc = "connector. Clients are encouraged to send this after receiving the"]
            #[doc = "\"withdrawn\" event so that the server can release the resources"]
            #[doc = "associated with this connector offer. Neither existing lease requests"]
            #[doc = "nor leases will be affected."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The compositor sends this event once the connector is created to"]
            #[doc = "indicate the name of this connector. This will not change for the"]
            #[doc = "duration of the Wayland session, but is not guaranteed to be consistent"]
            #[doc = "between sessions."]
            #[doc = ""]
            #[doc = "If the compositor supports wl_output version 4 and this connector"]
            #[doc = "corresponds to a wl_output, the compositor should use the same name as"]
            #[doc = "for the wl_output."]
            async fn name(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_connector_v1#{}.name()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor sends this event once the connector is created to provide"]
            #[doc = "a human-readable description for this connector, which may be presented"]
            #[doc = "to the user. The compositor may send this event multiple times over the"]
            #[doc = "lifetime of this object to reflect changes in the description."]
            async fn description(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                description: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_connector_v1#{}.description()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor sends this event once the connector is created to"]
            #[doc = "indicate the DRM object ID which represents the underlying connector"]
            #[doc = "that is being offered. Note that the final lease may include additional"]
            #[doc = "object IDs, such as CRTCs and planes."]
            async fn connector_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                connector_id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_connector_v1#{}.connector_id()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(connector_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all properties of a connector have been sent."]
            #[doc = "This allows changes to the properties to be seen as atomic even if they"]
            #[doc = "happen via multiple events."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_connector_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent to indicate that the compositor will no longer honor requests for"]
            #[doc = "DRM leases which include this connector. The client may still issue a"]
            #[doc = "lease request including this connector, but the compositor will send"]
            #[doc = "wp_drm_lease_v1.finished without issuing a lease fd. Compositors are"]
            #[doc = "encouraged to send this event when they lose access to connector, for"]
            #[doc = "example when the connector is hot-unplugged, when the connector gets"]
            #[doc = "leased to a client or when the compositor loses DRM master."]
            async fn withdrawn(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_connector_v1#{}.withdrawn()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A client that wishes to lease DRM resources will attach the list of"]
    #[doc = "connectors advertised with wp_drm_lease_device_v1.connector that they"]
    #[doc = "wish to lease, then use wp_drm_lease_request_v1.submit to submit the"]
    #[doc = "request."]
    pub mod wp_drm_lease_request_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "requested a connector from a different lease device"]
            WrongDevice = 0u32,
            #[doc = "requested a connector twice"]
            DuplicateConnector = 1u32,
            #[doc = "requested a lease without requesting a connector"]
            EmptyLease = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::WrongDevice),
                    1u32 => Ok(Self::DuplicateConnector),
                    2u32 => Ok(Self::EmptyLease),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_drm_lease_request_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseRequestV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_request_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "wp_drm_lease_request_v1#{}.request_connector()",
                            object.id
                        );
                        self.request_connector(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("wp_drm_lease_request_v1#{}.submit()", object.id);
                        self.submit(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Indicates that the client would like to lease the given connector."]
            #[doc = "This is only used as a suggestion, the compositor may choose to"]
            #[doc = "include any resources in the lease it issues, or change the set of"]
            #[doc = "leased resources at any time. Compositors are however encouraged to"]
            #[doc = "include the requested connector and other resources necessary"]
            #[doc = "to drive the connected output in the lease."]
            #[doc = ""]
            #[doc = "Requesting a connector that was created from a different lease device"]
            #[doc = "than this lease request raises the wrong_device error. Requesting a"]
            #[doc = "connector twice will raise the duplicate_connector error."]
            async fn request_connector(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                connector: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Submits the lease request and creates a new wp_drm_lease_v1 object."]
            #[doc = "After calling submit the compositor will immediately destroy this"]
            #[doc = "object, issuing any more requests will cause a wl_display error."]
            #[doc = "The compositor doesn't make any guarantees about the events of the"]
            #[doc = "lease object, clients cannot expect an immediate response."]
            #[doc = "Not requesting any connectors before submitting the lease request"]
            #[doc = "will raise the empty_lease error."]
            async fn submit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "A DRM lease object is used to transfer the DRM file descriptor to the"]
    #[doc = "client and manage the lifetime of the lease."]
    #[doc = ""]
    #[doc = "Some time after the wp_drm_lease_v1 object is created, the compositor"]
    #[doc = "will reply with the lease request's result. If the lease request is"]
    #[doc = "granted, the compositor will send a lease_fd event. If the lease request"]
    #[doc = "is denied, the compositor will send a finished event without a lease_fd"]
    #[doc = "event."]
    pub mod wp_drm_lease_v1 {
        #[doc = "Trait to implement the wp_drm_lease_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_drm_lease_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "The client should send this to indicate that it no longer wishes to use"]
            #[doc = "this lease. The compositor should use drmModeRevokeLease on the"]
            #[doc = "appropriate file descriptor, if necessary."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This event returns a file descriptor suitable for use with DRM-related"]
            #[doc = "ioctls. The client should use drmModeGetLease to enumerate the DRM"]
            #[doc = "objects which have been leased to them. The compositor guarantees it"]
            #[doc = "will not use the leased DRM objects itself until it sends the finished"]
            #[doc = "event. If the compositor cannot or will not grant a lease for the"]
            #[doc = "requested connectors, it will not send this event, instead sending the"]
            #[doc = "finished event."]
            #[doc = ""]
            #[doc = "The compositor will send this event at most once during this objects"]
            #[doc = "lifetime."]
            async fn lease_fd(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                leased_fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_v1#{}.lease_fd()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fd(leased_fd).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor uses this event to either reject a lease request, or if"]
            #[doc = "it previously sent a lease_fd, to notify the client that the lease has"]
            #[doc = "been revoked. If the client requires a new lease, they should destroy"]
            #[doc = "this object and submit a new lease request. The compositor will send"]
            #[doc = "no further events for this object after sending the finish event."]
            #[doc = "Compositors should revoke the lease when any of the leased resources"]
            #[doc = "become unavailable, namely when a hot-unplug occurs or when the"]
            #[doc = "compositor loses DRM master."]
            async fn finished(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_drm_lease_v1#{}.finished()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[doc = "The purpose of this protocol is to provide protocol object handles for"]
#[doc = "toplevels, possibly originating from another client."]
#[doc = ""]
#[doc = "This protocol is intentionally minimalistic and expects additional"]
#[doc = "functionality (e.g. creating a screencopy source from a toplevel handle,"]
#[doc = "getting information about the state of the toplevel) to be implemented"]
#[doc = "in extension protocols."]
#[doc = ""]
#[doc = "The compositor may choose to restrict this protocol to a special client"]
#[doc = "launched by the compositor itself or expose it to all clients,"]
#[doc = "this is compositor policy."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\",  \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod ext_foreign_toplevel_list_v1 {
    #[doc = "A toplevel is defined as a surface with a role similar to xdg_toplevel."]
    #[doc = "XWayland surfaces may be treated like toplevels in this protocol."]
    #[doc = ""]
    #[doc = "After a client binds the ext_foreign_toplevel_list_v1, each mapped"]
    #[doc = "toplevel window will be sent using the ext_foreign_toplevel_list_v1.toplevel"]
    #[doc = "event."]
    #[doc = ""]
    #[doc = "Clients which only care about the current state can perform a roundtrip after"]
    #[doc = "binding this global."]
    #[doc = ""]
    #[doc = "For each instance of ext_foreign_toplevel_list_v1, the compositor must"]
    #[doc = "create a new ext_foreign_toplevel_handle_v1 object for each mapped toplevel."]
    #[doc = ""]
    #[doc = "If a compositor implementation sends the ext_foreign_toplevel_list_v1.finished"]
    #[doc = "event after the global is bound, the compositor must not send any"]
    #[doc = "ext_foreign_toplevel_list_v1.toplevel events."]
    pub mod ext_foreign_toplevel_list_v1 {
        #[doc = "Trait to implement the ext_foreign_toplevel_list_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelListV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_foreign_toplevel_list_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_foreign_toplevel_list_v1#{}.stop()", object.id);
                        self.stop(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("ext_foreign_toplevel_list_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events for new toplevels."]
            #[doc = ""]
            #[doc = "The Wayland protocol is asynchronous, meaning the compositor may send"]
            #[doc = "further toplevel events until the stop request is processed."]
            #[doc = "The client should wait for a ext_foreign_toplevel_list_v1.finished"]
            #[doc = "event before destroying this object."]
            async fn stop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This request should be called either when the client will no longer"]
            #[doc = "use the ext_foreign_toplevel_list_v1 or after the finished event"]
            #[doc = "has been received to allow destruction of the object."]
            #[doc = ""]
            #[doc = "If a client wishes to destroy this object it should send a"]
            #[doc = "ext_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished"]
            #[doc = "event, then destroy the handles and then this object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This event is emitted whenever a new toplevel window is created. It is"]
            #[doc = "emitted for all toplevels, regardless of the app that has created them."]
            #[doc = ""]
            #[doc = "All initial properties of the toplevel (identifier, title, app_id) will be sent"]
            #[doc = "immediately after this event using the corresponding events for"]
            #[doc = "ext_foreign_toplevel_handle_v1. The compositor will use the"]
            #[doc = "ext_foreign_toplevel_handle_v1.done event to indicate when all data has"]
            #[doc = "been sent."]
            async fn toplevel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                toplevel: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_list_v1#{}.toplevel()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates that the compositor is done sending events"]
            #[doc = "to this object. The client should destroy the object."]
            #[doc = "See ext_foreign_toplevel_list_v1.destroy for more information."]
            #[doc = ""]
            #[doc = "The compositor must not send any more toplevel events after this event."]
            async fn finished(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_list_v1#{}.finished()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A ext_foreign_toplevel_handle_v1 object represents a mapped toplevel"]
    #[doc = "window. A single app may have multiple mapped toplevels."]
    pub mod ext_foreign_toplevel_handle_v1 {
        #[doc = "Trait to implement the ext_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelHandleV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_foreign_toplevel_handle_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_foreign_toplevel_handle_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This request should be used when the client will no longer use the handle"]
            #[doc = "or after the closed event has been received to allow destruction of the"]
            #[doc = "object."]
            #[doc = ""]
            #[doc = "When a handle is destroyed, a new handle may not be created by the server"]
            #[doc = "until the toplevel is unmapped and then remapped. Destroying a toplevel handle"]
            #[doc = "is not recommended unless the client is cleaning up child objects"]
            #[doc = "before destroying the ext_foreign_toplevel_list_v1 object, the toplevel"]
            #[doc = "was closed or the toplevel handle will not be used in the future."]
            #[doc = ""]
            #[doc = "Other protocols which extend the ext_foreign_toplevel_handle_v1"]
            #[doc = "interface should require destructors for extension interfaces be"]
            #[doc = "called before allowing the toplevel handle to be destroyed."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The server will emit no further events on the ext_foreign_toplevel_handle_v1"]
            #[doc = "after this event. Any requests received aside from the destroy request must"]
            #[doc = "be ignored. Upon receiving this event, the client should destroy the handle."]
            #[doc = ""]
            #[doc = "Other protocols which extend the ext_foreign_toplevel_handle_v1"]
            #[doc = "interface must also ignore requests other than destructors."]
            async fn closed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_handle_v1#{}.closed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all changes in the toplevel state have"]
            #[doc = "been sent."]
            #[doc = ""]
            #[doc = "This allows changes to the ext_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be atomically applied. Other protocols which extend the"]
            #[doc = "ext_foreign_toplevel_handle_v1 interface may use this event to also"]
            #[doc = "atomically apply any pending state."]
            #[doc = ""]
            #[doc = "This event must not be sent after the ext_foreign_toplevel_handle_v1.closed"]
            #[doc = "event."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_handle_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The title of the toplevel has changed."]
            #[doc = ""]
            #[doc = "The configured state must not be applied immediately. See"]
            #[doc = "ext_foreign_toplevel_handle_v1.done for details."]
            async fn title(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                title: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_handle_v1#{}.title()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(title))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The app id of the toplevel has changed."]
            #[doc = ""]
            #[doc = "The configured state must not be applied immediately. See"]
            #[doc = "ext_foreign_toplevel_handle_v1.done for details."]
            async fn app_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                app_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_handle_v1#{}.app_id()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This identifier is used to check if two or more toplevel handles belong"]
            #[doc = "to the same toplevel."]
            #[doc = ""]
            #[doc = "The identifier is useful for command line tools or privileged clients"]
            #[doc = "which may need to reference an exact toplevel across processes or"]
            #[doc = "instances of the ext_foreign_toplevel_list_v1 global."]
            #[doc = ""]
            #[doc = "The compositor must only send this event when the handle is created."]
            #[doc = ""]
            #[doc = "The identifier must be unique per toplevel and it's handles. Two different"]
            #[doc = "toplevels must not have the same identifier. The identifier is only valid"]
            #[doc = "as long as the toplevel is mapped. If the toplevel is unmapped the identifier"]
            #[doc = "must not be reused. An identifier must not be reused by the compositor to"]
            #[doc = "ensure there are no races when sharing identifiers between processes."]
            #[doc = ""]
            #[doc = "An identifier is a string that contains up to 32 printable ASCII bytes."]
            #[doc = "An identifier must not be an empty string. It is recommended that a"]
            #[doc = "compositor includes an opaque generation value in identifiers. How the"]
            #[doc = "generation value is used when generating the identifier is implementation"]
            #[doc = "dependent."]
            async fn identifier(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                identifier: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_foreign_toplevel_handle_v1#{}.identifier()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(identifier))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod ext_idle_notify_v1 {
    #[doc = "This interface allows clients to monitor user idle status."]
    #[doc = ""]
    #[doc = "After binding to this global, clients can create ext_idle_notification_v1"]
    #[doc = "objects to get notified when the user is idle for a given amount of time."]
    pub mod ext_idle_notifier_v1 {
        #[doc = "Trait to implement the ext_idle_notifier_v1 interface. See the module level documentation for more info"]
        pub trait ExtIdleNotifierV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_idle_notifier_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_idle_notifier_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_idle_notifier_v1#{}.get_idle_notification()",
                            object.id
                        );
                        self.get_idle_notification(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the manager object. All objects created via this interface"]
            #[doc = "remain valid."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create a new idle notification object."]
            #[doc = ""]
            #[doc = "The notification object has a minimum timeout duration and is tied to a"]
            #[doc = "seat. The client will be notified if the seat is inactive for at least"]
            #[doc = "the provided timeout. See ext_idle_notification_v1 for more details."]
            #[doc = ""]
            #[doc = "A zero timeout is valid and means the client wants to be notified as"]
            #[doc = "soon as possible when the seat is inactive."]
            async fn get_idle_notification(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                timeout: u32,
                seat: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This interface is used by the compositor to send idle notification events"]
    #[doc = "to clients."]
    #[doc = ""]
    #[doc = "Initially the notification object is not idle. The notification object"]
    #[doc = "becomes idle when no user activity has happened for at least the timeout"]
    #[doc = "duration, starting from the creation of the notification object. User"]
    #[doc = "activity may include input events or a presence sensor, but is"]
    #[doc = "compositor-specific. If an idle inhibitor is active (e.g. another client"]
    #[doc = "has created a zwp_idle_inhibitor_v1 on a visible surface), the compositor"]
    #[doc = "must not make the notification object idle."]
    #[doc = ""]
    #[doc = "When the notification object becomes idle, an idled event is sent. When"]
    #[doc = "user activity starts again, the notification object stops being idle,"]
    #[doc = "a resumed event is sent and the timeout is restarted."]
    pub mod ext_idle_notification_v1 {
        #[doc = "Trait to implement the ext_idle_notification_v1 interface. See the module level documentation for more info"]
        pub trait ExtIdleNotificationV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_idle_notification_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_idle_notification_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the notification object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This event is sent when the notification object becomes idle."]
            #[doc = ""]
            #[doc = "It's a compositor protocol error to send this event twice without a"]
            #[doc = "resumed event in-between."]
            async fn idled(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_idle_notification_v1#{}.idled()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent when the notification object stops being idle."]
            #[doc = ""]
            #[doc = "It's a compositor protocol error to send this event twice without an"]
            #[doc = "idled event in-between. It's a compositor protocol error to send this"]
            #[doc = "event prior to any idled event."]
            async fn resumed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_idle_notification_v1#{}.resumed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol serves as an intermediary between capturing protocols and"]
#[doc = "potential image capture sources such as outputs and toplevels."]
#[doc = ""]
#[doc = "This protocol may be extended to support more image capture sources in the"]
#[doc = "future, thereby adding those image capture sources to other protocols that"]
#[doc = "use the image capture source object without having to modify those"]
#[doc = "protocols."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod ext_image_capture_source_v1 {
    #[doc = "The image capture source object is an opaque descriptor for a capturable"]
    #[doc = "resource.  This resource may be any sort of entity from which an image"]
    #[doc = "may be derived."]
    #[doc = ""]
    #[doc = "Note, because ext_image_capture_source_v1 objects are created from multiple"]
    #[doc = "independent factory interfaces, the ext_image_capture_source_v1 interface is"]
    #[doc = "frozen at version 1."]
    pub mod ext_image_capture_source_v1 {
        #[doc = "Trait to implement the ext_image_capture_source_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCaptureSourceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_image_capture_source_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_image_capture_source_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the image capture source. This request may be sent at any time"]
            #[doc = "by the client."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "A manager for creating image capture source objects for wl_output objects."]
    pub mod ext_output_image_capture_source_manager_v1 {
        #[doc = "Trait to implement the ext_output_image_capture_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtOutputImageCaptureSourceManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_output_image_capture_source_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "ext_output_image_capture_source_manager_v1#{}.create_source()",
                            object.id
                        );
                        self.create_source(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_output_image_capture_source_manager_v1#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Creates a source object for an output. Images captured from this source"]
            #[doc = "will show the same content as the output. Some elements may be omitted,"]
            #[doc = "such as cursors and overlays that have been marked as transparent to"]
            #[doc = "capturing."]
            async fn create_source(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                source: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "A manager for creating image capture source objects for"]
    #[doc = "ext_foreign_toplevel_handle_v1 objects."]
    pub mod ext_foreign_toplevel_image_capture_source_manager_v1 {
        #[doc = "Trait to implement the ext_foreign_toplevel_image_capture_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelImageCaptureSourceManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_foreign_toplevel_image_capture_source_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing :: debug ! ("ext_foreign_toplevel_image_capture_source_manager_v1#{}.create_source()" , object . id);
                        self.create_source(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_foreign_toplevel_image_capture_source_manager_v1#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Creates a source object for a foreign toplevel handle. Images captured"]
            #[doc = "from this source will show the same content as the toplevel."]
            async fn create_source(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                source: crate::wire::ObjectId,
                toplevel_handle: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
#[doc = "This protocol allows clients to ask the compositor to capture image sources"]
#[doc = "such as outputs and toplevels into user submitted buffers."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod ext_image_copy_capture_v1 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    pub mod ext_image_copy_capture_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid option flag"]
            InvalidOption = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidOption),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Options : u32 { # [doc = "paint cursors onto captured frames"] const PaintCursors = 1u32 ; } }
        impl TryFrom<u32> for Options {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the ext_image_copy_capture_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_image_copy_capture_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_manager_v1#{}.create_session()",
                            object.id
                        );
                        self.create_session(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_manager_v1#{}.create_pointer_cursor_session()",
                            object.id
                        );
                        self.create_pointer_cursor_session(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_manager_v1#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a capturing session for an image capture source."]
            #[doc = ""]
            #[doc = "If the paint_cursors option is set, cursors shall be composited onto"]
            #[doc = "the captured frame. The cursor must not be composited onto the frame"]
            #[doc = "if this flag is not set."]
            #[doc = ""]
            #[doc = "If the options bitfield is invalid, the invalid_option protocol error"]
            #[doc = "is sent."]
            async fn create_session(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                session: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                options: Options,
            ) -> crate::server::Result<()>;
            #[doc = "Create a cursor capturing session for the pointer of an image capture"]
            #[doc = "source."]
            async fn create_pointer_cursor_session(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                session: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the manager object."]
            #[doc = ""]
            #[doc = "Other objects created via this interface are unaffected."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This object represents an active image copy capture session."]
    #[doc = ""]
    #[doc = "After a capture session is created, buffer constraint events will be"]
    #[doc = "emitted from the compositor to tell the client which buffer types and"]
    #[doc = "formats are supported for reading from the session. The compositor may"]
    #[doc = "re-send buffer constraint events whenever they change."]
    #[doc = ""]
    #[doc = "The advertise buffer constraints, the compositor must send in no"]
    #[doc = "particular order: zero or more shm_format and dmabuf_format events, zero"]
    #[doc = "or one dmabuf_device event, and exactly one buffer_size event. Then the"]
    #[doc = "compositor must send a done event."]
    #[doc = ""]
    #[doc = "When the client has received all the buffer constraints, it can create a"]
    #[doc = "buffer accordingly, attach it to the capture session using the"]
    #[doc = "attach_buffer request, set the buffer damage using the damage_buffer"]
    #[doc = "request and then send the capture request."]
    pub mod ext_image_copy_capture_session_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "create_frame sent before destroying previous frame"]
            DuplicateFrame = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::DuplicateFrame),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_image_copy_capture_session_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureSessionV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_image_copy_capture_session_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.create_frame()",
                            object.id
                        );
                        self.create_frame(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a capture frame for this session."]
            #[doc = ""]
            #[doc = "At most one frame object can exist for a given session at any time. If"]
            #[doc = "a client sends a create_frame request before a previous frame object"]
            #[doc = "has been destroyed, the duplicate_frame protocol error is raised."]
            async fn create_frame(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                frame: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect ext_image_copy_capture_frame_v1 objects created by"]
            #[doc = "this object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Provides the dimensions of the source image in buffer pixel coordinates."]
            #[doc = ""]
            #[doc = "The client must attach buffers that match this size."]
            async fn buffer_size(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                width: u32,
                height: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.buffer_size()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(width)
                    .put_uint(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Provides the format that must be used for shared-memory buffers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            async fn shm_format(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                format: super::super::wayland::wl_shm::Format,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.shm_format()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(format as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event advertises the device buffers must be allocated on for"]
            #[doc = "dma-buf buffers."]
            #[doc = ""]
            #[doc = "In general the device is a DRM node. The DRM node type (primary vs."]
            #[doc = "render) is unspecified. Clients must not rely on the compositor sending"]
            #[doc = "a particular node type. Clients cannot check two devices for equality"]
            #[doc = "by comparing the dev_t value."]
            async fn dmabuf_device(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                device: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.dmabuf_device()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_array(device).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Provides the format that must be used for dma-buf buffers."]
            #[doc = ""]
            #[doc = "The client may choose any of the modifiers advertised in the array of"]
            #[doc = "64-bit unsigned integers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            async fn dmabuf_format(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                format: u32,
                modifiers: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.dmabuf_format()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(format)
                    .put_array(modifiers)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent once when all buffer constraint events have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "The compositor must always end a batch of buffer constraint events with"]
            #[doc = "this event, regardless of whether it sends the initial constraints or"]
            #[doc = "an update."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_image_copy_capture_session_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates that the capture session has stopped and is no"]
            #[doc = "longer available. This can happen in a number of cases, e.g. when the"]
            #[doc = "underlying source is destroyed, if the user decides to end the image"]
            #[doc = "capture, or if an unrecoverable runtime error has occurred."]
            #[doc = ""]
            #[doc = "The client should destroy the session after receiving this event."]
            async fn stopped(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.stopped()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "This object represents an image capture frame."]
    #[doc = ""]
    #[doc = "The client should attach a buffer, damage the buffer, and then send a"]
    #[doc = "capture request."]
    #[doc = ""]
    #[doc = "If the capture is successful, the compositor must send the frame metadata"]
    #[doc = "(transform, damage, presentation_time in any order) followed by the ready"]
    #[doc = "event."]
    #[doc = ""]
    #[doc = "If the capture fails, the compositor must send the failed event."]
    pub mod ext_image_copy_capture_frame_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "capture sent without attach_buffer"]
            NoBuffer = 1u32,
            #[doc = "invalid buffer damage"]
            InvalidBufferDamage = 2u32,
            #[doc = "capture request has been sent"]
            AlreadyCaptured = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NoBuffer),
                    2u32 => Ok(Self::InvalidBufferDamage),
                    3u32 => Ok(Self::AlreadyCaptured),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum FailureReason {
            Unknown = 0u32,
            BufferConstraints = 1u32,
            Stopped = 2u32,
        }
        impl TryFrom<u32> for FailureReason {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unknown),
                    1u32 => Ok(Self::BufferConstraints),
                    2u32 => Ok(Self::Stopped),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_image_copy_capture_frame_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureFrameV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_image_copy_capture_frame_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_image_copy_capture_frame_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_frame_v1#{}.attach_buffer()",
                            object.id
                        );
                        self.attach_buffer(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_frame_v1#{}.damage_buffer()",
                            object.id
                        );
                        self.damage_buffer(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("ext_image_copy_capture_frame_v1#{}.capture()", object.id);
                        self.capture(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Attach a buffer to the session."]
            #[doc = ""]
            #[doc = "The wl_buffer.release request is unused."]
            #[doc = ""]
            #[doc = "The new buffer replaces any previously attached buffer."]
            #[doc = ""]
            #[doc = "This request must not be sent after capture, or else the"]
            #[doc = "already_captured protocol error is raised."]
            async fn attach_buffer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Apply damage to the buffer which is to be captured next. This request"]
            #[doc = "may be sent multiple times to describe a region."]
            #[doc = ""]
            #[doc = "The client indicates the accumulated damage since this wl_buffer was"]
            #[doc = "last captured. During capture, the compositor will update the buffer"]
            #[doc = "with at least the union of the region passed by the client and the"]
            #[doc = "region advertised by ext_image_copy_capture_frame_v1.damage."]
            #[doc = ""]
            #[doc = "When a wl_buffer is captured for the first time, or when the client"]
            #[doc = "doesn't track damage, the client must damage the whole buffer."]
            #[doc = ""]
            #[doc = "This is for optimisation purposes. The compositor may use this"]
            #[doc = "information to reduce copying."]
            #[doc = ""]
            #[doc = "These coordinates originate from the upper left corner of the buffer."]
            #[doc = ""]
            #[doc = "If x or y are strictly negative, or if width or height are negative or"]
            #[doc = "zero, the invalid_buffer_damage protocol error is raised."]
            #[doc = ""]
            #[doc = "This request must not be sent after capture, or else the"]
            #[doc = "already_captured protocol error is raised."]
            async fn damage_buffer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Capture a frame."]
            #[doc = ""]
            #[doc = "Unless this is the first successful captured frame performed in this"]
            #[doc = "session, the compositor may wait an indefinite amount of time for the"]
            #[doc = "source content to change before performing the copy."]
            #[doc = ""]
            #[doc = "This request may only be sent once, or else the already_captured"]
            #[doc = "protocol error is raised. A buffer must be attached before this request"]
            #[doc = "is sent, or else the no_buffer protocol error is raised."]
            async fn capture(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This event is sent before the ready event and holds the transform that"]
            #[doc = "the compositor has applied to the buffer contents."]
            async fn transform(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                transform: super::super::wayland::wl_output::Transform,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_frame_v1#{}.transform()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(transform as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent before the ready event. It may be generated multiple"]
            #[doc = "times to describe a region."]
            #[doc = ""]
            #[doc = "The first captured frame in a session will always carry full damage."]
            #[doc = "Subsequent frames' damaged regions describe which parts of the buffer"]
            #[doc = "have changed since the last ready event."]
            #[doc = ""]
            #[doc = "These coordinates originate in the upper left corner of the buffer."]
            async fn damage(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_image_copy_capture_frame_v1#{}.damage()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates the time at which the frame is presented to the"]
            #[doc = "output in system monotonic time. This event is sent before the ready"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]."]
            async fn presentation_time(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_frame_v1#{}.presentation_time()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(tv_sec_hi)
                    .put_uint(tv_sec_lo)
                    .put_uint(tv_nsec)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading."]
            #[doc = ""]
            #[doc = "The buffer may be re-used by the client after this event."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            async fn ready(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_image_copy_capture_frame_v1#{}.ready()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            async fn failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                reason: FailureReason,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_image_copy_capture_frame_v1#{}.failed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(reason as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "This object represents a cursor capture session. It extends the base"]
    #[doc = "capture session with cursor-specific metadata."]
    pub mod ext_image_copy_capture_cursor_session_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "get_captuerer_session sent twice"]
            DuplicateSession = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::DuplicateSession),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_image_copy_capture_cursor_session_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureCursorSessionV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_image_copy_capture_cursor_session_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_cursor_session_v1#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_cursor_session_v1#{}.get_capture_session()",
                            object.id
                        );
                        self.get_capture_session(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect ext_image_copy_capture_frame_v1 objects created by"]
            #[doc = "this object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Gets the image copy capture session for this cursor session."]
            #[doc = ""]
            #[doc = "The session will produce frames of the cursor image. The compositor may"]
            #[doc = "pause the session when the cursor leaves the captured area."]
            #[doc = ""]
            #[doc = "This request must not be sent more than once, or else the"]
            #[doc = "duplicate_session protocol error is raised."]
            async fn get_capture_session(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                session: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Sent when a cursor enters the captured area. It shall be generated"]
            #[doc = "before the \"position\" and \"hotspot\" events when and only when a cursor"]
            #[doc = "enters the area."]
            #[doc = ""]
            #[doc = "The cursor enters the captured area when the cursor image intersects"]
            #[doc = "with the captured area. Note, this is different from e.g."]
            #[doc = "wl_pointer.enter."]
            async fn enter(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_cursor_session_v1#{}.enter()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent when a cursor leaves the captured area. No \"position\" or \"hotspot\""]
            #[doc = "event is generated for the cursor until the cursor enters the captured"]
            #[doc = "area again."]
            async fn leave(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_cursor_session_v1#{}.leave()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Cursors outside the image capture source do not get captured and no"]
            #[doc = "event will be generated for them."]
            #[doc = ""]
            #[doc = "The given position is the position of the cursor's hotspot and it is"]
            #[doc = "relative to the main buffer's top left corner in transformed buffer"]
            #[doc = "pixel coordinates. The coordinates may be negative or greater than the"]
            #[doc = "main buffer size."]
            async fn position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_cursor_session_v1#{}.position()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The hotspot describes the offset between the cursor image and the"]
            #[doc = "position of the input device."]
            #[doc = ""]
            #[doc = "The given coordinates are the hotspot's offset from the origin in"]
            #[doc = "buffer coordinates."]
            #[doc = ""]
            #[doc = "Clients should not apply the hotspot immediately: the hotspot becomes"]
            #[doc = "effective when the next ext_image_copy_capture_frame_v1.ready event is received."]
            #[doc = ""]
            #[doc = "Compositors may delay this event until the client captures a new frame."]
            async fn hotspot(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_cursor_session_v1#{}.hotspot()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows for a privileged Wayland client to lock the session"]
#[doc = "and display arbitrary graphics while the session is locked."]
#[doc = ""]
#[doc = "The compositor may choose to restrict this protocol to a special client"]
#[doc = "launched by the compositor itself or expose it to all privileged clients,"]
#[doc = "this is compositor policy."]
#[doc = ""]
#[doc = "The client is responsible for performing authentication and informing the"]
#[doc = "compositor when the session should be unlocked. If the client dies while"]
#[doc = "the session is locked the session remains locked, possibly permanently"]
#[doc = "depending on compositor policy."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\",  \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the"]
#[doc = "testing phase. Backward compatible changes may be added together with"]
#[doc = "the corresponding interface version bump. Backward incompatible changes"]
#[doc = "can only be done by creating a new major version of the extension."]
pub mod ext_session_lock_v1 {
    #[doc = "This interface is used to request that the session be locked."]
    pub mod ext_session_lock_manager_v1 {
        #[doc = "Trait to implement the ext_session_lock_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_session_lock_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_session_lock_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("ext_session_lock_manager_v1#{}.lock()", object.id);
                        self.lock(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This informs the compositor that the session lock manager object will"]
            #[doc = "no longer be used. Existing objects created through this interface"]
            #[doc = "remain valid."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This request creates a session lock and asks the compositor to lock the"]
            #[doc = "session. The compositor will send either the ext_session_lock_v1.locked"]
            #[doc = "or ext_session_lock_v1.finished event on the created object in"]
            #[doc = "response to this request."]
            async fn lock(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "In response to the creation of this object the compositor must send"]
    #[doc = "either the locked or finished event."]
    #[doc = ""]
    #[doc = "The locked event indicates that the session is locked. This means"]
    #[doc = "that the compositor must stop rendering and providing input to normal"]
    #[doc = "clients. Instead the compositor must blank all outputs with an opaque"]
    #[doc = "color such that their normal content is fully hidden."]
    #[doc = ""]
    #[doc = "The only surfaces that should be rendered while the session is locked"]
    #[doc = "are the lock surfaces created through this interface and optionally,"]
    #[doc = "at the compositor's discretion, special privileged surfaces such as"]
    #[doc = "input methods or portions of desktop shell UIs."]
    #[doc = ""]
    #[doc = "The locked event must not be sent until a new \"locked\" frame (either"]
    #[doc = "from a session lock surface or the compositor blanking the output) has"]
    #[doc = "been presented on all outputs and no security sensitive normal/unlocked"]
    #[doc = "content is possibly visible."]
    #[doc = ""]
    #[doc = "The finished event should be sent immediately on creation of this"]
    #[doc = "object if the compositor decides that the locked event will not be sent."]
    #[doc = ""]
    #[doc = "The compositor may wait for the client to create and render session lock"]
    #[doc = "surfaces before sending the locked event to avoid displaying intermediate"]
    #[doc = "blank frames. However, it must impose a reasonable time limit if"]
    #[doc = "waiting and send the locked event as soon as the hard requirements"]
    #[doc = "described above can be met if the time limit expires. Clients should"]
    #[doc = "immediately create lock surfaces for all outputs on creation of this"]
    #[doc = "object to make this possible."]
    #[doc = ""]
    #[doc = "This behavior of the locked event is required in order to prevent"]
    #[doc = "possible race conditions with clients that wish to suspend the system"]
    #[doc = "or similar after locking the session. Without these semantics, clients"]
    #[doc = "triggering a suspend after receiving the locked event would race with"]
    #[doc = "the first \"locked\" frame being presented and normal/unlocked frames"]
    #[doc = "might be briefly visible as the system is resumed if the suspend"]
    #[doc = "operation wins the race."]
    #[doc = ""]
    #[doc = "If the client dies while the session is locked, the compositor must not"]
    #[doc = "unlock the session in response. It is acceptable for the session to be"]
    #[doc = "permanently locked if this happens. The compositor may choose to continue"]
    #[doc = "to display the lock surfaces the client had mapped before it died or"]
    #[doc = "alternatively fall back to a solid color, this is compositor policy."]
    #[doc = ""]
    #[doc = "Compositors may also allow a secure way to recover the session, the"]
    #[doc = "details of this are compositor policy. Compositors may allow a new"]
    #[doc = "client to create a ext_session_lock_v1 object and take responsibility"]
    #[doc = "for unlocking the session, they may even start a new lock client"]
    #[doc = "instance automatically."]
    pub mod ext_session_lock_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "attempted to destroy session lock while locked"]
            InvalidDestroy = 0u32,
            #[doc = "unlock requested but locked event was never sent"]
            InvalidUnlock = 1u32,
            #[doc = "given wl_surface already has a role"]
            Role = 2u32,
            #[doc = "given output already has a lock surface"]
            DuplicateOutput = 3u32,
            #[doc = "given wl_surface has a buffer attached or committed"]
            AlreadyConstructed = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidDestroy),
                    1u32 => Ok(Self::InvalidUnlock),
                    2u32 => Ok(Self::Role),
                    3u32 => Ok(Self::DuplicateOutput),
                    4u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_session_lock_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_session_lock_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_session_lock_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("ext_session_lock_v1#{}.get_lock_surface()", object.id);
                        self.get_lock_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("ext_session_lock_v1#{}.unlock_and_destroy()", object.id);
                        self.unlock_and_destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This informs the compositor that the lock object will no longer be"]
            #[doc = "used. Existing objects created through this interface remain valid."]
            #[doc = ""]
            #[doc = "After this request is made, lock surfaces created through this object"]
            #[doc = "should be destroyed by the client as they will no longer be used by"]
            #[doc = "the compositor."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request if the locked event was"]
            #[doc = "sent, the unlock_and_destroy request must be used instead."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The client is expected to create lock surfaces for all outputs"]
            #[doc = "currently present and any new outputs as they are advertised. These"]
            #[doc = "won't be displayed by the compositor unless the lock is successful"]
            #[doc = "and the locked event is sent."]
            #[doc = ""]
            #[doc = "Providing a wl_surface which already has a role or already has a buffer"]
            #[doc = "attached or committed is a protocol error, as is attaching/committing"]
            #[doc = "a buffer before the first ext_session_lock_surface_v1.configure event."]
            #[doc = ""]
            #[doc = "Attempting to create more than one lock surface for a given output"]
            #[doc = "is a duplicate_output protocol error."]
            async fn get_lock_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This request indicates that the session should be unlocked, for"]
            #[doc = "example because the user has entered their password and it has been"]
            #[doc = "verified by the client."]
            #[doc = ""]
            #[doc = "This request also informs the compositor that the lock object will"]
            #[doc = "no longer be used and should be destroyed. Existing objects created"]
            #[doc = "through this interface remain valid."]
            #[doc = ""]
            #[doc = "After this request is made, lock surfaces created through this object"]
            #[doc = "should be destroyed by the client as they will no longer be used by"]
            #[doc = "the compositor."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request if the locked event has"]
            #[doc = "not been sent. In that case, the lock object must be destroyed using"]
            #[doc = "the destroy request."]
            #[doc = ""]
            #[doc = "Note that a correct client that wishes to exit directly after unlocking"]
            #[doc = "the session must use the wl_display.sync request to ensure the server"]
            #[doc = "receives and processes the unlock_and_destroy request. Otherwise"]
            #[doc = "there is no guarantee that the server has unlocked the session due"]
            #[doc = "to the asynchronous nature of the Wayland protocol. For example,"]
            #[doc = "the server might terminate the client with a protocol error before"]
            #[doc = "it processes the unlock_and_destroy request."]
            async fn unlock_and_destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This client is now responsible for displaying graphics while the"]
            #[doc = "session is locked and deciding when to unlock the session."]
            #[doc = ""]
            #[doc = "The locked event must not be sent until a new \"locked\" frame has been"]
            #[doc = "presented on all outputs and no security sensitive normal/unlocked"]
            #[doc = "content is possibly visible."]
            #[doc = ""]
            #[doc = "If this event is sent, making the destroy request is a protocol error,"]
            #[doc = "the lock object must be destroyed using the unlock_and_destroy request."]
            async fn locked(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_session_lock_v1#{}.locked()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor has decided that the session lock should be destroyed"]
            #[doc = "as it will no longer be used by the compositor. Exactly when this"]
            #[doc = "event is sent is compositor policy, but it must never be sent more"]
            #[doc = "than once for a given session lock object."]
            #[doc = ""]
            #[doc = "This might be sent because there is already another ext_session_lock_v1"]
            #[doc = "object held by a client, or the compositor has decided to deny the"]
            #[doc = "request to lock the session for some other reason. This might also"]
            #[doc = "be sent because the compositor implements some alternative, secure"]
            #[doc = "way to authenticate and unlock the session."]
            #[doc = ""]
            #[doc = "The finished event should be sent immediately on creation of this"]
            #[doc = "object if the compositor decides that the locked event will not"]
            #[doc = "be sent."]
            #[doc = ""]
            #[doc = "If the locked event is sent on creation of this object the finished"]
            #[doc = "event may still be sent at some later time in this object's"]
            #[doc = "lifetime. This is compositor policy."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should make either the destroy"]
            #[doc = "request or the unlock_and_destroy request, depending on whether or"]
            #[doc = "not the locked event was received on this object."]
            async fn finished(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_session_lock_v1#{}.finished()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "The client may use lock surfaces to display a screensaver, render a"]
    #[doc = "dialog to enter a password and unlock the session, or however else it"]
    #[doc = "sees fit."]
    #[doc = ""]
    #[doc = "On binding this interface the compositor will immediately send the"]
    #[doc = "first configure event. After making the ack_configure request in"]
    #[doc = "response to this event the client should attach and commit the first"]
    #[doc = "buffer. Committing the surface before acking the first configure is a"]
    #[doc = "protocol error. Committing the surface with a null buffer at any time"]
    #[doc = "is a protocol error."]
    #[doc = ""]
    #[doc = "The compositor is free to handle keyboard/pointer focus for lock"]
    #[doc = "surfaces however it chooses. A reasonable way to do this would be to"]
    #[doc = "give the first lock surface created keyboard focus and change keyboard"]
    #[doc = "focus if the user clicks on other surfaces."]
    pub mod ext_session_lock_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface committed before first ack_configure request"]
            CommitBeforeFirstAck = 0u32,
            #[doc = "surface committed with a null buffer"]
            NullBuffer = 1u32,
            #[doc = "failed to match ack'd width/height"]
            DimensionsMismatch = 2u32,
            #[doc = "serial provided in ack_configure is invalid"]
            InvalidSerial = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::CommitBeforeFirstAck),
                    1u32 => Ok(Self::NullBuffer),
                    2u32 => Ok(Self::DimensionsMismatch),
                    3u32 => Ok(Self::InvalidSerial),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_session_lock_surface_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockSurfaceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_session_lock_surface_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_session_lock_surface_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_session_lock_surface_v1#{}.ack_configure()",
                            object.id
                        );
                        self.ack_configure(object, client, message.uint()?).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This informs the compositor that the lock surface object will no"]
            #[doc = "longer be used."]
            #[doc = ""]
            #[doc = "It is recommended for a lock client to destroy lock surfaces if"]
            #[doc = "their corresponding wl_output global is removed."]
            #[doc = ""]
            #[doc = "If a lock surface on an active output is destroyed before the"]
            #[doc = "ext_session_lock_v1.unlock_and_destroy event is sent, the compositor"]
            #[doc = "must fall back to rendering a solid color."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "When a configure event is received, if a client commits the surface"]
            #[doc = "in response to the configure event, then the client must make an"]
            #[doc = "ack_configure request sometime before the commit request, passing"]
            #[doc = "along the serial of the configure event."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it can"]
            #[doc = "respond to one, it only has to ack the last configure event."]
            #[doc = ""]
            #[doc = "A client is not required to commit immediately after sending an"]
            #[doc = "ack_configure request - it may even ack_configure several times"]
            #[doc = "before its next surface commit."]
            #[doc = ""]
            #[doc = "A client may send multiple ack_configure requests before committing,"]
            #[doc = "but only the last request sent before a commit indicates which"]
            #[doc = "configure event the client really is responding to."]
            #[doc = ""]
            #[doc = "Sending an ack_configure request consumes the configure event"]
            #[doc = "referenced by the given serial, as well as all older configure events"]
            #[doc = "sent on this object."]
            #[doc = ""]
            #[doc = "It is a protocol error to issue multiple ack_configure requests"]
            #[doc = "referencing the same configure event or to issue an ack_configure"]
            #[doc = "request referencing a configure event older than the last configure"]
            #[doc = "event acked for a given lock surface."]
            async fn ack_configure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
            ) -> crate::server::Result<()>;
            #[doc = "This event is sent once on binding the interface and may be sent again"]
            #[doc = "at the compositor's discretion, for example if output geometry changes."]
            #[doc = ""]
            #[doc = "The width and height are in surface-local coordinates and are exact"]
            #[doc = "requirements. Failing to match these surface dimensions in the next"]
            #[doc = "commit after acking a configure is a protocol error."]
            async fn configure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                width: u32,
                height: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_session_lock_surface_v1#{}.configure()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(width)
                    .put_uint(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[doc = "The transient seat protocol can be used by privileged clients to create"]
#[doc = "independent seats that will be removed from the compositor when the client"]
#[doc = "destroys its transient seat."]
#[doc = ""]
#[doc = "This protocol is intended for use with virtual input protocols such as"]
#[doc = "\"virtual_keyboard_unstable_v1\" or \"wlr_virtual_pointer_unstable_v1\", both"]
#[doc = "of which allow the user to select a seat."]
#[doc = ""]
#[doc = "The \"wl_seat\" global created by this protocol does not generate input events"]
#[doc = "on its own, or have any capabilities except those assigned to it by other"]
#[doc = "protocol extensions, such as the ones mentioned above."]
#[doc = ""]
#[doc = "For example, a remote desktop server can create a seat with virtual inputs"]
#[doc = "for each remote user by following these steps for each new connection:"]
#[doc = "* Create a transient seat"]
#[doc = "* Wait for the transient seat to be created"]
#[doc = "* Locate a \"wl_seat\" global with a matching name"]
#[doc = "* Create virtual inputs using the resulting \"wl_seat\" global"]
pub mod ext_transient_seat_v1 {
    #[doc = "The transient seat manager creates short-lived seats."]
    pub mod ext_transient_seat_manager_v1 {
        #[doc = "Trait to implement the ext_transient_seat_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtTransientSeatManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_transient_seat_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_transient_seat_manager_v1#{}.create()", object.id);
                        self.create(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("ext_transient_seat_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a new seat that is removed when the client side transient seat"]
            #[doc = "object is destroyed."]
            #[doc = ""]
            #[doc = "The actual seat may be removed sooner, in which case the transient seat"]
            #[doc = "object shall become inert."]
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                seat: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the manager."]
            #[doc = ""]
            #[doc = "All objects created by the manager will remain valid until they are"]
            #[doc = "destroyed themselves."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "When the transient seat handle is destroyed, the seat itself will also be"]
    #[doc = "destroyed."]
    pub mod ext_transient_seat_v1 {
        #[doc = "Trait to implement the ext_transient_seat_v1 interface. See the module level documentation for more info"]
        pub trait ExtTransientSeatV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ext_transient_seat_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ext_transient_seat_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "When the transient seat object is destroyed by the client, the"]
            #[doc = "associated seat created by the compositor is also destroyed."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This event advertises the global name for the wl_seat to be used with"]
            #[doc = "wl_registry_bind."]
            #[doc = ""]
            #[doc = "It is sent exactly once, immediately after the transient seat is created"]
            #[doc = "and the new \"wl_seat\" global is advertised, if and only if the creation"]
            #[doc = "of the transient seat was allowed."]
            async fn ready(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                global_name: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_transient_seat_v1#{}.ready()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(global_name)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The event informs the client that the compositor denied its request to"]
            #[doc = "create a transient seat."]
            #[doc = ""]
            #[doc = "It is sent exactly once, immediately after the transient seat object is"]
            #[doc = "created, if and only if the creation of the transient seat was denied."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            async fn denied(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ext_transient_seat_v1#{}.denied()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows a compositor to suggest for surfaces to render at"]
#[doc = "fractional scales."]
#[doc = ""]
#[doc = "A client can submit scaled content by utilizing wp_viewport. This is done by"]
#[doc = "creating a wp_viewport object for the surface and setting the destination"]
#[doc = "rectangle to the surface size before the scale factor is applied."]
#[doc = ""]
#[doc = "The buffer size is calculated by multiplying the surface size by the"]
#[doc = "intended scale."]
#[doc = ""]
#[doc = "The wl_surface buffer scale should remain set to 1."]
#[doc = ""]
#[doc = "If a surface has a surface-local size of 100 px by 50 px and wishes to"]
#[doc = "submit buffers with a scale of 1.5, then a buffer of 150px by 75 px should"]
#[doc = "be used and the wp_viewport destination rectangle should be 100 px by 50 px."]
#[doc = ""]
#[doc = "For toplevel surfaces, the size is rounded halfway away from zero. The"]
#[doc = "rounding algorithm for subsurface position and size is not defined."]
pub mod fractional_scale_v1 {
    #[doc = "A global interface for requesting surfaces to use fractional scales."]
    pub mod wp_fractional_scale_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a fractional_scale object associated"]
            FractionalScaleExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::FractionalScaleExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_fractional_scale_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpFractionalScaleManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_fractional_scale_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_fractional_scale_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_fractional_scale_manager_v1#{}.get_fractional_scale()",
                            object.id
                        );
                        self.get_fractional_scale(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will not be using this protocol"]
            #[doc = "object anymore. This does not affect any other objects,"]
            #[doc = "wp_fractional_scale_v1 objects included."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create an add-on object for the the wl_surface to let the compositor"]
            #[doc = "request fractional scales. If the given wl_surface already has a"]
            #[doc = "wp_fractional_scale_v1 object associated, the fractional_scale_exists"]
            #[doc = "protocol error is raised."]
            async fn get_fractional_scale(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "An additional interface to a wl_surface object which allows the compositor"]
    #[doc = "to inform the client of the preferred scale."]
    pub mod wp_fractional_scale_v1 {
        #[doc = "Trait to implement the wp_fractional_scale_v1 interface. See the module level documentation for more info"]
        pub trait WpFractionalScaleV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_fractional_scale_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_fractional_scale_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the fractional scale object. When this object is destroyed,"]
            #[doc = "preferred_scale events will no longer be sent."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Notification of a new preferred scale for this surface that the"]
            #[doc = "compositor suggests that the client should use."]
            #[doc = ""]
            #[doc = "The sent scale is the numerator of a fraction with a denominator of 120."]
            async fn preferred_scale(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                scale: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wp_fractional_scale_v1#{}.preferred_scale()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(scale).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows clients to request explicit synchronization for"]
#[doc = "buffers. It is tied to the Linux DRM synchronization object framework."]
#[doc = ""]
#[doc = "Synchronization refers to co-ordination of pipelined operations performed"]
#[doc = "on buffers. Most GPU clients will schedule an asynchronous operation to"]
#[doc = "render to the buffer, then immediately send the buffer to the compositor"]
#[doc = "to be attached to a surface."]
#[doc = ""]
#[doc = "With implicit synchronization, ensuring that the rendering operation is"]
#[doc = "complete before the compositor displays the buffer is an implementation"]
#[doc = "detail handled by either the kernel or userspace graphics driver."]
#[doc = ""]
#[doc = "By contrast, with explicit synchronization, DRM synchronization object"]
#[doc = "timeline points mark when the asynchronous operations are complete. When"]
#[doc = "submitting a buffer, the client provides a timeline point which will be"]
#[doc = "waited on before the compositor accesses the buffer, and another timeline"]
#[doc = "point that the compositor will signal when it no longer needs to access the"]
#[doc = "buffer contents for the purposes of the surface commit."]
#[doc = ""]
#[doc = "Linux DRM synchronization objects are documented at:"]
#[doc = "https://dri.freedesktop.org/docs/drm/gpu/drm-mm.html#drm-sync-objects"]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod linux_drm_syncobj_v1 {
    #[doc = "This global is a factory interface, allowing clients to request"]
    #[doc = "explicit synchronization for buffers on a per-surface basis."]
    #[doc = ""]
    #[doc = "See wp_linux_drm_syncobj_surface_v1 for more information."]
    pub mod wp_linux_drm_syncobj_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a synchronization object associated"]
            SurfaceExists = 0u32,
            #[doc = "the timeline object could not be imported"]
            InvalidTimeline = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SurfaceExists),
                    1u32 => Ok(Self::InvalidTimeline),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_linux_drm_syncobj_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_linux_drm_syncobj_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_linux_drm_syncobj_manager_v1#{}.get_surface()",
                            object.id
                        );
                        self.get_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "wp_linux_drm_syncobj_manager_v1#{}.import_timeline()",
                            object.id
                        );
                        self.import_timeline(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.fd()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this explicit synchronization factory object. Other objects"]
            #[doc = "shall not be affected by this request."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Instantiate an interface extension for the given wl_surface to provide"]
            #[doc = "explicit synchronization."]
            #[doc = ""]
            #[doc = "If the given wl_surface already has an explicit synchronization object"]
            #[doc = "associated, the surface_exists protocol error is raised."]
            #[doc = ""]
            #[doc = "Graphics APIs, like EGL or Vulkan, that manage the buffer queue and"]
            #[doc = "commits of a wl_surface themselves, are likely to be using this"]
            #[doc = "extension internally. If a client is using such an API for a"]
            #[doc = "wl_surface, it should not directly use this extension on that surface,"]
            #[doc = "to avoid raising a surface_exists protocol error."]
            async fn get_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Import a DRM synchronization object timeline."]
            #[doc = ""]
            #[doc = "If the FD cannot be imported, the invalid_timeline error is raised."]
            async fn import_timeline(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This object represents an explicit synchronization object timeline"]
    #[doc = "imported by the client to the compositor."]
    pub mod wp_linux_drm_syncobj_timeline_v1 {
        #[doc = "Trait to implement the wp_linux_drm_syncobj_timeline_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjTimelineV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_timeline_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_linux_drm_syncobj_timeline_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the synchronization object timeline. Other objects are not"]
            #[doc = "affected by this request, in particular timeline points set by"]
            #[doc = "set_acquire_point and set_release_point are not unset."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This object is an add-on interface for wl_surface to enable explicit"]
    #[doc = "synchronization."]
    #[doc = ""]
    #[doc = "Each surface can be associated with only one object of this interface at"]
    #[doc = "any time."]
    #[doc = ""]
    #[doc = "Explicit synchronization is guaranteed to be supported for buffers"]
    #[doc = "created with any version of the linux-dmabuf protocol. Compositors are"]
    #[doc = "free to support explicit synchronization for additional buffer types."]
    #[doc = "If at surface commit time the attached buffer does not support explicit"]
    #[doc = "synchronization, an unsupported_buffer error is raised."]
    #[doc = ""]
    #[doc = "As long as the wp_linux_drm_syncobj_surface_v1 object is alive, the"]
    #[doc = "compositor may ignore implicit synchronization for buffers attached and"]
    #[doc = "committed to the wl_surface. The delivery of wl_buffer.release events"]
    #[doc = "for buffers attached to the surface becomes undefined."]
    #[doc = ""]
    #[doc = "Clients must set both acquire and release points if and only if a"]
    #[doc = "non-null buffer is attached in the same surface commit. See the"]
    #[doc = "no_buffer, no_acquire_point and no_release_point protocol errors."]
    #[doc = ""]
    #[doc = "If at surface commit time the acquire and release DRM syncobj timelines"]
    #[doc = "are identical, the acquire point value must be strictly less than the"]
    #[doc = "release point value, or else the conflicting_points protocol error is"]
    #[doc = "raised."]
    pub mod wp_linux_drm_syncobj_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the associated wl_surface was destroyed"]
            NoSurface = 1u32,
            #[doc = "the buffer does not support explicit synchronization"]
            UnsupportedBuffer = 2u32,
            #[doc = "no buffer was attached"]
            NoBuffer = 3u32,
            #[doc = "no acquire timeline point was set"]
            NoAcquirePoint = 4u32,
            #[doc = "no release timeline point was set"]
            NoReleasePoint = 5u32,
            #[doc = "acquire and release timeline points are in conflict"]
            ConflictingPoints = 6u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NoSurface),
                    2u32 => Ok(Self::UnsupportedBuffer),
                    3u32 => Ok(Self::NoBuffer),
                    4u32 => Ok(Self::NoAcquirePoint),
                    5u32 => Ok(Self::NoReleasePoint),
                    6u32 => Ok(Self::ConflictingPoints),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_linux_drm_syncobj_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjSurfaceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_surface_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_linux_drm_syncobj_surface_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_linux_drm_syncobj_surface_v1#{}.set_acquire_point()",
                            object.id
                        );
                        self.set_acquire_point(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "wp_linux_drm_syncobj_surface_v1#{}.set_release_point()",
                            object.id
                        );
                        self.set_release_point(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this surface synchronization object."]
            #[doc = ""]
            #[doc = "Any timeline point set by this object with set_acquire_point or"]
            #[doc = "set_release_point since the last commit may be discarded by the"]
            #[doc = "compositor. Any timeline point set by this object before the last"]
            #[doc = "commit will not be affected."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Set the timeline point that must be signalled before the compositor may"]
            #[doc = "sample from the buffer attached with wl_surface.attach."]
            #[doc = ""]
            #[doc = "The 64-bit unsigned value combined from point_hi and point_lo is the"]
            #[doc = "point value."]
            #[doc = ""]
            #[doc = "The acquire point is double-buffered state, and will be applied on the"]
            #[doc = "next wl_surface.commit request for the associated surface. Thus, it"]
            #[doc = "applies only to the buffer that is attached to the surface at commit"]
            #[doc = "time."]
            #[doc = ""]
            #[doc = "If an acquire point has already been attached during the same commit"]
            #[doc = "cycle, the new point replaces the old one."]
            #[doc = ""]
            #[doc = "If the associated wl_surface was destroyed, a no_surface error is"]
            #[doc = "raised."]
            #[doc = ""]
            #[doc = "If at surface commit time there is a pending acquire timeline point set"]
            #[doc = "but no pending buffer attached, a no_buffer error is raised. If at"]
            #[doc = "surface commit time there is a pending buffer attached but no pending"]
            #[doc = "acquire timeline point set, the no_acquire_point protocol error is"]
            #[doc = "raised."]
            async fn set_acquire_point(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                timeline: crate::wire::ObjectId,
                point_hi: u32,
                point_lo: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Set the timeline point that must be signalled by the compositor when it"]
            #[doc = "has finished its usage of the buffer attached with wl_surface.attach"]
            #[doc = "for the relevant commit."]
            #[doc = ""]
            #[doc = "Once the timeline point is signaled, and assuming the associated buffer"]
            #[doc = "is not pending release from other wl_surface.commit requests, no"]
            #[doc = "additional explicit or implicit synchronization with the compositor is"]
            #[doc = "required to safely re-use the buffer."]
            #[doc = ""]
            #[doc = "Note that clients cannot rely on the release point being always"]
            #[doc = "signaled after the acquire point: compositors may release buffers"]
            #[doc = "without ever reading from them. In addition, the compositor may use"]
            #[doc = "different presentation paths for different commits, which may have"]
            #[doc = "different release behavior. As a result, the compositor may signal the"]
            #[doc = "release points in a different order than the client committed them."]
            #[doc = ""]
            #[doc = "Because signaling a timeline point also signals every previous point,"]
            #[doc = "it is generally not safe to use the same timeline object for the"]
            #[doc = "release points of multiple buffers. The out-of-order signaling"]
            #[doc = "described above may lead to a release point being signaled before the"]
            #[doc = "compositor has finished reading. To avoid this, it is strongly"]
            #[doc = "recommended that each buffer should use a separate timeline for its"]
            #[doc = "release points."]
            #[doc = ""]
            #[doc = "The 64-bit unsigned value combined from point_hi and point_lo is the"]
            #[doc = "point value."]
            #[doc = ""]
            #[doc = "The release point is double-buffered state, and will be applied on the"]
            #[doc = "next wl_surface.commit request for the associated surface. Thus, it"]
            #[doc = "applies only to the buffer that is attached to the surface at commit"]
            #[doc = "time."]
            #[doc = ""]
            #[doc = "If a release point has already been attached during the same commit"]
            #[doc = "cycle, the new point replaces the old one."]
            #[doc = ""]
            #[doc = "If the associated wl_surface was destroyed, a no_surface error is"]
            #[doc = "raised."]
            #[doc = ""]
            #[doc = "If at surface commit time there is a pending release timeline point set"]
            #[doc = "but no pending buffer attached, a no_buffer error is raised. If at"]
            #[doc = "surface commit time there is a pending buffer attached but no pending"]
            #[doc = "release timeline point set, the no_release_point protocol error is"]
            #[doc = "raised."]
            async fn set_release_point(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                timeline: crate::wire::ObjectId,
                point_hi: u32,
                point_lo: u32,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod security_context_v1 {
    #[doc = "This interface allows a client to register a new Wayland connection to"]
    #[doc = "the compositor and attach a security context to it."]
    #[doc = ""]
    #[doc = "This is intended to be used by sandboxes. Sandbox engines attach a"]
    #[doc = "security context to all connections coming from inside the sandbox. The"]
    #[doc = "compositor can then restrict the features that the sandboxed connections"]
    #[doc = "can use."]
    #[doc = ""]
    #[doc = "Compositors should forbid nesting multiple security contexts by not"]
    #[doc = "exposing wp_security_context_manager_v1 global to clients with a security"]
    #[doc = "context attached, or by sending the nested protocol error. Nested"]
    #[doc = "security contexts are dangerous because they can potentially allow"]
    #[doc = "privilege escalation of a sandboxed client."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_security_context_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "listening socket FD is invalid"]
            InvalidListenFd = 1u32,
            #[doc = "nested security contexts are forbidden"]
            Nested = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidListenFd),
                    2u32 => Ok(Self::Nested),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_security_context_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpSecurityContextManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_security_context_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_security_context_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_security_context_manager_v1#{}.create_listener()",
                            object.id
                        );
                        self.create_listener(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.fd()?,
                            message.fd()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the manager. This doesn't destroy objects created with the"]
            #[doc = "manager."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Creates a new security context with a socket listening FD."]
            #[doc = ""]
            #[doc = "The compositor will accept new client connections on listen_fd."]
            #[doc = "listen_fd must be ready to accept new connections when this request is"]
            #[doc = "sent by the client. In other words, the client must call bind(2) and"]
            #[doc = "listen(2) before sending the FD."]
            #[doc = ""]
            #[doc = "close_fd is a FD closed by the client when the compositor should stop"]
            #[doc = "accepting new connections on listen_fd."]
            #[doc = ""]
            #[doc = "The compositor must continue to accept connections on listen_fd when"]
            #[doc = "the Wayland client which created the security context disconnects."]
            #[doc = ""]
            #[doc = "After sending this request, closing listen_fd and close_fd remains the"]
            #[doc = "only valid operation on them."]
            async fn create_listener(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                listen_fd: rustix::fd::OwnedFd,
                close_fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "The security context allows a client to register a new client and attach"]
    #[doc = "security context metadata to the connections."]
    #[doc = ""]
    #[doc = "When both are set, the combination of the application ID and the sandbox"]
    #[doc = "engine must uniquely identify an application. The same application ID"]
    #[doc = "will be used across instances (e.g. if the application is restarted, or"]
    #[doc = "if the application is started multiple times)."]
    #[doc = ""]
    #[doc = "When both are set, the combination of the instance ID and the sandbox"]
    #[doc = "engine must uniquely identify a running instance of an application."]
    pub mod wp_security_context_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "security context has already been committed"]
            AlreadyUsed = 1u32,
            #[doc = "metadata has already been set"]
            AlreadySet = 2u32,
            #[doc = "metadata is invalid"]
            InvalidMetadata = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyUsed),
                    2u32 => Ok(Self::AlreadySet),
                    3u32 => Ok(Self::InvalidMetadata),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_security_context_v1 interface. See the module level documentation for more info"]
        pub trait WpSecurityContextV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_security_context_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_security_context_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_security_context_v1#{}.set_sandbox_engine()",
                            object.id
                        );
                        self.set_sandbox_engine(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("wp_security_context_v1#{}.set_app_id()", object.id);
                        self.set_app_id(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("wp_security_context_v1#{}.set_instance_id()", object.id);
                        self.set_instance_id(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4u16 => {
                        tracing::debug!("wp_security_context_v1#{}.commit()", object.id);
                        self.commit(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the security context object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Attach a unique sandbox engine name to the security context. The name"]
            #[doc = "should follow the reverse-DNS style (e.g. \"org.flatpak\")."]
            #[doc = ""]
            #[doc = "A list of well-known engines is maintained at:"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md"]
            #[doc = ""]
            #[doc = "It is a protocol error to call this request twice. The already_set"]
            #[doc = "error is sent in this case."]
            async fn set_sandbox_engine(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
            ) -> crate::server::Result<()>;
            #[doc = "Attach an application ID to the security context."]
            #[doc = ""]
            #[doc = "The application ID is an opaque, sandbox-specific identifier for an"]
            #[doc = "application. See the well-known engines document for more details:"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md"]
            #[doc = ""]
            #[doc = "The compositor may use the application ID to group clients belonging to"]
            #[doc = "the same security context application."]
            #[doc = ""]
            #[doc = "Whether this request is optional or not depends on the sandbox engine used."]
            #[doc = ""]
            #[doc = "It is a protocol error to call this request twice. The already_set"]
            #[doc = "error is sent in this case."]
            async fn set_app_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                app_id: String,
            ) -> crate::server::Result<()>;
            #[doc = "Attach an instance ID to the security context."]
            #[doc = ""]
            #[doc = "The instance ID is an opaque, sandbox-specific identifier for a running"]
            #[doc = "instance of an application. See the well-known engines document for"]
            #[doc = "more details:"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md"]
            #[doc = ""]
            #[doc = "Whether this request is optional or not depends on the sandbox engine used."]
            #[doc = ""]
            #[doc = "It is a protocol error to call this request twice. The already_set"]
            #[doc = "error is sent in this case."]
            async fn set_instance_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                instance_id: String,
            ) -> crate::server::Result<()>;
            #[doc = "Atomically register the new client and attach the security context"]
            #[doc = "metadata."]
            #[doc = ""]
            #[doc = "If the provided metadata is inconsistent or does not match with out of"]
            #[doc = "band metadata (see"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md),"]
            #[doc = "the invalid_metadata error may be sent eventually."]
            #[doc = ""]
            #[doc = "It's a protocol error to send any request other than \"destroy\" after"]
            #[doc = "this request. In this case, the already_used error is sent."]
            async fn commit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
#[doc = "This protocol extension allows clients to create single-pixel buffers."]
#[doc = ""]
#[doc = "Compositors supporting this protocol extension should also support the"]
#[doc = "viewporter protocol extension. Clients may use viewporter to scale a"]
#[doc = "single-pixel buffer to a desired size."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod single_pixel_buffer_v1 {
    #[doc = "The wp_single_pixel_buffer_manager_v1 interface is a factory for"]
    #[doc = "single-pixel buffers."]
    pub mod wp_single_pixel_buffer_manager_v1 {
        #[doc = "Trait to implement the wp_single_pixel_buffer_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpSinglePixelBufferManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_single_pixel_buffer_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "wp_single_pixel_buffer_manager_v1#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_single_pixel_buffer_manager_v1#{}.create_u32_rgba_buffer()",
                            object.id
                        );
                        self.create_u32_rgba_buffer(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_single_pixel_buffer_manager_v1 object."]
            #[doc = ""]
            #[doc = "The child objects created via this interface are unaffected."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create a single-pixel buffer from four 32-bit RGBA values."]
            #[doc = ""]
            #[doc = "Unless specified in another protocol extension, the RGBA values use"]
            #[doc = "pre-multiplied alpha."]
            #[doc = ""]
            #[doc = "The width and height of the buffer are 1."]
            async fn create_u32_rgba_buffer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                r: u32,
                g: u32,
                b: u32,
                a: u32,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod tearing_control_v1 {
    #[doc = "For some use cases like games or drawing tablets it can make sense to"]
    #[doc = "reduce latency by accepting tearing with the use of asynchronous page"]
    #[doc = "flips. This global is a factory interface, allowing clients to inform"]
    #[doc = "which type of presentation the content of their surfaces is suitable for."]
    #[doc = ""]
    #[doc = "Graphics APIs like EGL or Vulkan, that manage the buffer queue and commits"]
    #[doc = "of a wl_surface themselves, are likely to be using this extension"]
    #[doc = "internally. If a client is using such an API for a wl_surface, it should"]
    #[doc = "not directly use this extension on that surface, to avoid raising a"]
    #[doc = "tearing_control_exists protocol error."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_tearing_control_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a tearing object associated"]
            TearingControlExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::TearingControlExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_tearing_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpTearingControlManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_tearing_control_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wp_tearing_control_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "wp_tearing_control_manager_v1#{}.get_tearing_control()",
                            object.id
                        );
                        self.get_tearing_control(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this tearing control factory object. Other objects, including"]
            #[doc = "wp_tearing_control_v1 objects created by this factory, are not affected"]
            #[doc = "by this request."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Instantiate an interface extension for the given wl_surface to request"]
            #[doc = "asynchronous page flips for presentation."]
            #[doc = ""]
            #[doc = "If the given wl_surface already has a wp_tearing_control_v1 object"]
            #[doc = "associated, the tearing_control_exists protocol error is raised."]
            async fn get_tearing_control(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "An additional interface to a wl_surface object, which allows the client"]
    #[doc = "to hint to the compositor if the content on the surface is suitable for"]
    #[doc = "presentation with tearing."]
    #[doc = "The default presentation hint is vsync. See presentation_hint for more"]
    #[doc = "details."]
    #[doc = ""]
    #[doc = "If the associated wl_surface is destroyed, this object becomes inert and"]
    #[doc = "should be destroyed."]
    pub mod wp_tearing_control_v1 {
        #[doc = "This enum provides information for if submitted frames from the client"]
        #[doc = "may be presented with tearing."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PresentationHint {
            Vsync = 0u32,
            Async = 1u32,
        }
        impl TryFrom<u32> for PresentationHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Vsync),
                    1u32 => Ok(Self::Async),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_tearing_control_v1 interface. See the module level documentation for more info"]
        pub trait WpTearingControlV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wp_tearing_control_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "wp_tearing_control_v1#{}.set_presentation_hint()",
                            object.id
                        );
                        self.set_presentation_hint(object, client, message.uint()?.try_into()?)
                            .await
                    }
                    1u16 => {
                        tracing::debug!("wp_tearing_control_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Set the presentation hint for the associated wl_surface. This state is"]
            #[doc = "double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The compositor is free to dynamically respect or ignore this hint based"]
            #[doc = "on various conditions like hardware capabilities, surface state and"]
            #[doc = "user preferences."]
            async fn set_presentation_hint(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                hint: PresentationHint,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy this surface tearing object and revert the presentation hint to"]
            #[doc = "vsync. The change will be applied on the next wl_surface.commit."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
#[doc = "The way for a client to pass focus to another toplevel is as follows."]
#[doc = ""]
#[doc = "The client that intends to activate another toplevel uses the"]
#[doc = "xdg_activation_v1.get_activation_token request to get an activation token."]
#[doc = "This token is then forwarded to the client, which is supposed to activate"]
#[doc = "one of its surfaces, through a separate band of communication."]
#[doc = ""]
#[doc = "One established way of doing this is through the XDG_ACTIVATION_TOKEN"]
#[doc = "environment variable of a newly launched child process. The child process"]
#[doc = "should unset the environment variable again right after reading it out in"]
#[doc = "order to avoid propagating it to other child processes."]
#[doc = ""]
#[doc = "Another established way exists for Applications implementing the D-Bus"]
#[doc = "interface org.freedesktop.Application, which should get their token under"]
#[doc = "activation-token on their platform_data."]
#[doc = ""]
#[doc = "In general activation tokens may be transferred across clients through"]
#[doc = "means not described in this protocol."]
#[doc = ""]
#[doc = "The client to be activated will then pass the token"]
#[doc = "it received to the xdg_activation_v1.activate request. The compositor can"]
#[doc = "then use this token to decide how to react to the activation request."]
#[doc = ""]
#[doc = "The token the activating client gets may be ineffective either already at"]
#[doc = "the time it receives it, for example if it was not focused, for focus"]
#[doc = "stealing prevention. The activating client will have no way to discover"]
#[doc = "the validity of the token, and may still forward it to the to be activated"]
#[doc = "client."]
#[doc = ""]
#[doc = "The created activation token may optionally get information attached to it"]
#[doc = "that can be used by the compositor to identify the application that we"]
#[doc = "intend to activate. This can for example be used to display a visual hint"]
#[doc = "about what application is being started."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod xdg_activation_v1 {
    #[doc = "A global interface used for informing the compositor about applications"]
    #[doc = "being activated or started, or for applications to request to be"]
    #[doc = "activated."]
    pub mod xdg_activation_v1 {
        #[doc = "Trait to implement the xdg_activation_v1 interface. See the module level documentation for more info"]
        pub trait XdgActivationV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_activation_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_activation_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_activation_v1#{}.get_activation_token()", object.id);
                        self.get_activation_token(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("xdg_activation_v1#{}.activate()", object.id);
                        self.activate(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the xdg_activation object will no longer be"]
            #[doc = "used."]
            #[doc = ""]
            #[doc = "The child objects created via this interface are unaffected and should"]
            #[doc = "be destroyed separately."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Creates an xdg_activation_token_v1 object that will provide"]
            #[doc = "the initiating client with a unique token for this activation. This"]
            #[doc = "token should be offered to the clients to be activated."]
            async fn get_activation_token(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Requests surface activation. It's up to the compositor to display"]
            #[doc = "this information as desired, for example by placing the surface above"]
            #[doc = "the rest."]
            #[doc = ""]
            #[doc = "The compositor may know who requested this by checking the activation"]
            #[doc = "token and might decide not to follow through with the activation if it's"]
            #[doc = "considered unwanted."]
            #[doc = ""]
            #[doc = "Compositors can ignore unknown activation tokens when an invalid"]
            #[doc = "token is passed."]
            async fn activate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                token: String,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "An object for setting up a token and receiving a token handle that can"]
    #[doc = "be passed as an activation token to another client."]
    #[doc = ""]
    #[doc = "The object is created using the xdg_activation_v1.get_activation_token"]
    #[doc = "request. This object should then be populated with the app_id, surface"]
    #[doc = "and serial information and committed. The compositor shall then issue a"]
    #[doc = "done event with the token. In case the request's parameters are invalid,"]
    #[doc = "the compositor will provide an invalid token."]
    pub mod xdg_activation_token_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "The token has already been used previously"]
            AlreadyUsed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_activation_token_v1 interface. See the module level documentation for more info"]
        pub trait XdgActivationTokenV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_activation_token_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_activation_token_v1#{}.set_serial()", object.id);
                        self.set_serial(
                            object,
                            client,
                            message.uint()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("xdg_activation_token_v1#{}.set_app_id()", object.id);
                        self.set_app_id(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("xdg_activation_token_v1#{}.set_surface()", object.id);
                        self.set_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("xdg_activation_token_v1#{}.commit()", object.id);
                        self.commit(object, client).await
                    }
                    4u16 => {
                        tracing::debug!("xdg_activation_token_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Provides information about the seat and serial event that requested the"]
            #[doc = "token."]
            #[doc = ""]
            #[doc = "The serial can come from an input or focus event. For instance, if a"]
            #[doc = "click triggers the launch of a third-party client, the launcher client"]
            #[doc = "should send a set_serial request with the serial and seat from the"]
            #[doc = "wl_pointer.button event."]
            #[doc = ""]
            #[doc = "Some compositors might refuse to activate toplevels when the token"]
            #[doc = "doesn't have a valid and recent enough event serial."]
            #[doc = ""]
            #[doc = "Must be sent before commit. This information is optional."]
            async fn set_serial(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                seat: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "The requesting client can specify an app_id to associate the token"]
            #[doc = "being created with it."]
            #[doc = ""]
            #[doc = "Must be sent before commit. This information is optional."]
            async fn set_app_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                app_id: String,
            ) -> crate::server::Result<()>;
            #[doc = "This request sets the surface requesting the activation. Note, this is"]
            #[doc = "different from the surface that will be activated."]
            #[doc = ""]
            #[doc = "Some compositors might refuse to activate toplevels when the token"]
            #[doc = "doesn't have a requesting surface."]
            #[doc = ""]
            #[doc = "Must be sent before commit. This information is optional."]
            async fn set_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Requests an activation token based on the different parameters that"]
            #[doc = "have been offered through set_serial, set_surface and set_app_id."]
            async fn commit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Notify the compositor that the xdg_activation_token_v1 object will no"]
            #[doc = "longer be used. The received token stays valid."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The 'done' event contains the unique token of this activation request"]
            #[doc = "and notifies that the provider is done."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                token: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xdg_activation_token_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(token))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod xdg_dialog_v1 {
    #[doc = "The xdg_wm_dialog_v1 interface is exposed as a global object allowing"]
    #[doc = "to register surfaces with a xdg_toplevel role as \"dialogs\" relative to"]
    #[doc = "another toplevel."]
    #[doc = ""]
    #[doc = "The compositor may let this relation influence how the surface is"]
    #[doc = "placed, displayed or interacted with."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod xdg_wm_dialog_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the xdg_toplevel object has already been used to create a xdg_dialog_v1"]
            AlreadyUsed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_wm_dialog_v1 interface. See the module level documentation for more info"]
        pub trait XdgWmDialogV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_wm_dialog_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_wm_dialog_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_wm_dialog_v1#{}.get_xdg_dialog()", object.id);
                        self.get_xdg_dialog(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the xdg_wm_dialog_v1 object. This does not affect"]
            #[doc = "the xdg_dialog_v1 objects generated through it."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Creates a xdg_dialog_v1 object for the given toplevel. See the interface"]
            #[doc = "description for more details."]
            #[doc = ""]
            #[doc = "Compositors must raise an already_used error if clients attempt to"]
            #[doc = "create multiple xdg_dialog_v1 objects for the same xdg_toplevel."]
            async fn get_xdg_dialog(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "A xdg_dialog_v1 object is an ancillary object tied to a xdg_toplevel. Its"]
    #[doc = "purpose is hinting the compositor that the toplevel is a \"dialog\" (e.g. a"]
    #[doc = "temporary window) relative to another toplevel (see"]
    #[doc = "xdg_toplevel.set_parent). If the xdg_toplevel is destroyed, the xdg_dialog_v1"]
    #[doc = "becomes inert."]
    #[doc = ""]
    #[doc = "Through this object, the client may provide additional hints about"]
    #[doc = "the purpose of the secondary toplevel. This interface has no effect"]
    #[doc = "on toplevels that are not attached to a parent toplevel."]
    pub mod xdg_dialog_v1 {
        #[doc = "Trait to implement the xdg_dialog_v1 interface. See the module level documentation for more info"]
        pub trait XdgDialogV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_dialog_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_dialog_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_dialog_v1#{}.set_modal()", object.id);
                        self.set_modal(object, client).await
                    }
                    2u16 => {
                        tracing::debug!("xdg_dialog_v1#{}.unset_modal()", object.id);
                        self.unset_modal(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the xdg_dialog_v1 object. If this object is destroyed"]
            #[doc = "before the related xdg_toplevel, the compositor should unapply its"]
            #[doc = "effects."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Hints that the dialog has \"modal\" behavior. Modal dialogs typically"]
            #[doc = "require to be fully addressed by the user (i.e. closed) before resuming"]
            #[doc = "interaction with the parent toplevel, and may require a distinct"]
            #[doc = "presentation."]
            #[doc = ""]
            #[doc = "Clients must implement the logic to filter events in the parent"]
            #[doc = "toplevel on their own."]
            #[doc = ""]
            #[doc = "Compositors may choose any policy in event delivery to the parent"]
            #[doc = "toplevel, from delivering all events unfiltered to using them for"]
            #[doc = "internal consumption."]
            async fn set_modal(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Drops the hint that this dialog has \"modal\" behavior. See"]
            #[doc = "xdg_dialog_v1.set_modal for more details."]
            async fn unset_modal(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod xdg_toplevel_drag_v1 {
    #[doc = "This protocol enhances normal drag and drop with the ability to move a"]
    #[doc = "window at the same time. This allows having detachable parts of a window"]
    #[doc = "that when dragged out of it become a new window and can be dragged over"]
    #[doc = "an existing window to be reattached."]
    #[doc = ""]
    #[doc = "A typical workflow would be when the user starts dragging on top of a"]
    #[doc = "detachable part of a window, the client would create a wl_data_source and"]
    #[doc = "a xdg_toplevel_drag_v1 object and start the drag as normal via"]
    #[doc = "wl_data_device.start_drag. Once the client determines that the detachable"]
    #[doc = "window contents should be detached from the originating window, it creates"]
    #[doc = "a new xdg_toplevel with these contents and issues a"]
    #[doc = "xdg_toplevel_drag_v1.attach request before mapping it. From now on the new"]
    #[doc = "window is moved by the compositor during the drag as if the client called"]
    #[doc = "xdg_toplevel.move."]
    #[doc = ""]
    #[doc = "Dragging an existing window is similar. The client creates a"]
    #[doc = "xdg_toplevel_drag_v1 object and attaches the existing toplevel before"]
    #[doc = "starting the drag."]
    #[doc = ""]
    #[doc = "Clients use the existing drag and drop mechanism to detect when a window"]
    #[doc = "can be docked or undocked. If the client wants to snap a window into a"]
    #[doc = "parent window it should delete or unmap the dragged top-level. If the"]
    #[doc = "contents should be detached again it attaches a new toplevel as described"]
    #[doc = "above. If a drag operation is cancelled without being dropped, clients"]
    #[doc = "should revert to the previous state, deleting any newly created windows"]
    #[doc = "as appropriate. When a drag operation ends as indicated by"]
    #[doc = "wl_data_source.dnd_drop_performed the dragged toplevel window's final"]
    #[doc = "position is determined as if a xdg_toplevel_move operation ended."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod xdg_toplevel_drag_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "data_source already used for toplevel drag"]
            InvalidSource = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_drag_manager_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelDragManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_toplevel_drag_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_toplevel_drag_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "xdg_toplevel_drag_manager_v1#{}.get_xdg_toplevel_drag()",
                            object.id
                        );
                        self.get_xdg_toplevel_drag(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,"]
            #[doc = "including xdg_toplevel_drag_v1 objects created by this factory, are not"]
            #[doc = "affected by this request."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create an xdg_toplevel_drag for a drag and drop operation that is going"]
            #[doc = "to be started with data_source."]
            #[doc = ""]
            #[doc = "This request can only be made on sources used in drag-and-drop, so it"]
            #[doc = "must be performed before wl_data_device.start_drag. Attempting to use"]
            #[doc = "the source other than for drag-and-drop such as in"]
            #[doc = "wl_data_device.set_selection will raise an invalid_source error."]
            #[doc = ""]
            #[doc = "Destroying data_source while a toplevel is attached to the"]
            #[doc = "xdg_toplevel_drag is undefined."]
            async fn get_xdg_toplevel_drag(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                data_source: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod xdg_toplevel_drag_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "valid toplevel already attached"]
            ToplevelAttached = 0u32,
            #[doc = "drag has not ended"]
            OngoingDrag = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::ToplevelAttached),
                    1u32 => Ok(Self::OngoingDrag),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_drag_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelDragV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_toplevel_drag_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_toplevel_drag_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_toplevel_drag_v1#{}.attach()", object.id);
                        self.attach(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this xdg_toplevel_drag_v1 object. This request must only be"]
            #[doc = "called after the underlying wl_data_source drag has ended, as indicated"]
            #[doc = "by the dnd_drop_performed or cancelled events. In any other case an"]
            #[doc = "ongoing_drag error is raised."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Request that the window will be moved with the cursor during the drag"]
            #[doc = "operation. The offset is a hint to the compositor how the toplevel"]
            #[doc = "should be positioned relative to the cursor hotspot in surface local"]
            #[doc = "coordinates. For example it might only be used when an unmapped window"]
            #[doc = "is attached. The attached window does not participate in the selection"]
            #[doc = "of the drag target."]
            #[doc = ""]
            #[doc = "If the toplevel is unmapped while it is attached, it is automatically"]
            #[doc = "detached from the drag. In this case this request has to be called again"]
            #[doc = "if the window should be attached after it is remapped."]
            #[doc = ""]
            #[doc = "This request can be called multiple times but issuing it while a"]
            #[doc = "toplevel with an active role is attached raises a toplevel_attached"]
            #[doc = "error."]
            async fn attach(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                toplevel: crate::wire::ObjectId,
                x_offset: i32,
                y_offset: i32,
            ) -> crate::server::Result<()>;
        }
    }
}
#[doc = "This protocol allows clients to set icons for their toplevel surfaces"]
#[doc = "either via the XDG icon stock (using an icon name), or from pixel data."]
#[doc = ""]
#[doc = "A toplevel icon represents the individual toplevel (unlike the application"]
#[doc = "or launcher icon, which represents the application as a whole), and may be"]
#[doc = "shown in window switchers, window overviews and taskbars that list"]
#[doc = "individual windows."]
#[doc = ""]
#[doc = "This document adheres to RFC 2119 when using words like \"must\","]
#[doc = "\"should\", \"may\", etc."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod xdg_toplevel_icon_v1 {
    #[doc = "This interface allows clients to create toplevel window icons and set"]
    #[doc = "them on toplevel windows to be displayed to the user."]
    pub mod xdg_toplevel_icon_manager_v1 {
        #[doc = "Trait to implement the xdg_toplevel_icon_manager_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelIconManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_toplevel_icon_manager_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_toplevel_icon_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_toplevel_icon_manager_v1#{}.create_icon()", object.id);
                        self.create_icon(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("xdg_toplevel_icon_manager_v1#{}.set_icon()", object.id);
                        self.set_icon(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.object()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the toplevel icon manager."]
            #[doc = "This does not destroy objects created with the manager."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Creates a new icon object. This icon can then be attached to a"]
            #[doc = "xdg_toplevel via the 'set_icon' request."]
            async fn create_icon(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This request assigns the icon 'icon' to 'toplevel', or clears the"]
            #[doc = "toplevel icon if 'icon' was null."]
            #[doc = "This state is double-buffered and is applied on the next"]
            #[doc = "wl_surface.commit of the toplevel."]
            #[doc = ""]
            #[doc = "After making this call, the xdg_toplevel_icon_v1 provided as 'icon'"]
            #[doc = "can be destroyed by the client without 'toplevel' losing its icon."]
            #[doc = "The xdg_toplevel_icon_v1 is immutable from this point, and any"]
            #[doc = "future attempts to change it must raise the"]
            #[doc = "'xdg_toplevel_icon_v1.immutable' protocol error."]
            #[doc = ""]
            #[doc = "The compositor must set the toplevel icon from either the pixel data"]
            #[doc = "the icon provides, or by loading a stock icon using the icon name."]
            #[doc = "See the description of 'xdg_toplevel_icon_v1' for details."]
            #[doc = ""]
            #[doc = "If 'icon' is set to null, the icon of the respective toplevel is reset"]
            #[doc = "to its default icon (usually the icon of the application, derived from"]
            #[doc = "its desktop-entry file, or a placeholder icon)."]
            #[doc = "If this request is passed an icon with no pixel buffers or icon name"]
            #[doc = "assigned, the icon must be reset just like if 'icon' was null."]
            async fn set_icon(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                toplevel: crate::wire::ObjectId,
                icon: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
            #[doc = "This event indicates an icon size the compositor prefers to be"]
            #[doc = "available if the client has scalable icons and can render to any size."]
            #[doc = ""]
            #[doc = "When the 'xdg_toplevel_icon_manager_v1' object is created, the"]
            #[doc = "compositor may send one or more 'icon_size' events to describe the list"]
            #[doc = "of preferred icon sizes. If the compositor has no size preference, it"]
            #[doc = "may not send any 'icon_size' event, and it is up to the client to"]
            #[doc = "decide a suitable icon size."]
            #[doc = ""]
            #[doc = "A sequence of 'icon_size' events must be finished with a 'done' event."]
            #[doc = "If the compositor has no size preferences, it must still send the"]
            #[doc = "'done' event, without any preceding 'icon_size' events."]
            async fn icon_size(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                size: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_manager_v1#{}.icon_size()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(size).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all 'icon_size' events have been sent."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_manager_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "This interface defines a toplevel icon."]
    #[doc = "An icon can have a name, and multiple buffers."]
    #[doc = "In order to be applied, the icon must have either a name, or at least"]
    #[doc = "one buffer assigned. Applying an empty icon (with no buffer or name) to"]
    #[doc = "a toplevel should reset its icon to the default icon."]
    #[doc = ""]
    #[doc = "It is up to compositor policy whether to prefer using a buffer or loading"]
    #[doc = "an icon via its name. See 'set_name' and 'add_buffer' for details."]
    pub mod xdg_toplevel_icon_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the provided buffer does not satisfy requirements"]
            InvalidBuffer = 1u32,
            #[doc = "the icon has already been assigned to a toplevel and must not be changed"]
            Immutable = 2u32,
            #[doc = "the provided buffer has been destroyed before the toplevel icon"]
            NoBuffer = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidBuffer),
                    2u32 => Ok(Self::Immutable),
                    3u32 => Ok(Self::NoBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_icon_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelIconV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xdg_toplevel_icon_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xdg_toplevel_icon_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_toplevel_icon_v1#{}.set_name()", object.id);
                        self.set_name(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("xdg_toplevel_icon_v1#{}.add_buffer()", object.id);
                        self.add_buffer(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the 'xdg_toplevel_icon_v1' object."]
            #[doc = "The icon must still remain set on every toplevel it was assigned to,"]
            #[doc = "until the toplevel icon is reset explicitly."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This request assigns an icon name to this icon."]
            #[doc = "Any previously set name is overridden."]
            #[doc = ""]
            #[doc = "The compositor must resolve 'icon_name' according to the lookup rules"]
            #[doc = "described in the XDG icon theme specification[1] using the"]
            #[doc = "environment's current icon theme."]
            #[doc = ""]
            #[doc = "If the compositor does not support icon names or cannot resolve"]
            #[doc = "'icon_name' according to the XDG icon theme specification it must"]
            #[doc = "fall back to using pixel buffer data instead."]
            #[doc = ""]
            #[doc = "If this request is made after the icon has been assigned to a toplevel"]
            #[doc = "via 'set_icon', a 'immutable' error must be raised."]
            #[doc = ""]
            #[doc = "[1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html"]
            async fn set_name(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                icon_name: String,
            ) -> crate::server::Result<()>;
            #[doc = "This request adds pixel data supplied as wl_buffer to the icon."]
            #[doc = ""]
            #[doc = "The client should add pixel data for all icon sizes and scales that"]
            #[doc = "it can provide, or which are explicitly requested by the compositor"]
            #[doc = "via 'icon_size' events on xdg_toplevel_icon_manager_v1."]
            #[doc = ""]
            #[doc = "The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm"]
            #[doc = "and must be a square (width and height being equal)."]
            #[doc = "If any of these buffer requirements are not fulfilled, a 'invalid_buffer'"]
            #[doc = "error must be raised."]
            #[doc = ""]
            #[doc = "If this icon instance already has a buffer of the same size and scale"]
            #[doc = "from a previous 'add_buffer' request, data from the last request"]
            #[doc = "overrides the preexisting pixel data."]
            #[doc = ""]
            #[doc = "The wl_buffer must be kept alive for as long as the xdg_toplevel_icon"]
            #[doc = "it is associated with is not destroyed, otherwise a 'no_buffer' error"]
            #[doc = "is raised. The buffer contents must not be modified after it was"]
            #[doc = "assigned to the icon."]
            #[doc = ""]
            #[doc = "If this request is made after the icon has been assigned to a toplevel"]
            #[doc = "via 'set_icon', a 'immutable' error must be raised."]
            async fn add_buffer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
                scale: i32,
            ) -> crate::server::Result<()>;
        }
    }
}
