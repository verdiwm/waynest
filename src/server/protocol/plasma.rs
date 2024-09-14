#![allow(async_fn_in_trait)]
pub mod appmenu {
    #[doc = "This interface allows a client to link a window (or wl_surface) to an com.canonical.dbusmenu"]
    #[doc = "interface registered on DBus."]
    pub mod org_kde_kwin_appmenu_manager {
        #[doc = "Trait to implement the org_kde_kwin_appmenu_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinAppmenuManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_appmenu_manager";
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
                        tracing::debug!("org_kde_kwin_appmenu_manager#{}.create()", object.id);
                        self.create(
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "The DBus service name and object path where the appmenu interface is present"]
    #[doc = "The object should be registered on the session bus before sending this request."]
    #[doc = "If not applicable, clients should remove this object."]
    pub mod org_kde_kwin_appmenu {
        #[doc = "Trait to implement the org_kde_kwin_appmenu interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinAppmenu: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_appmenu";
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
                        tracing::debug!("org_kde_kwin_appmenu#{}.set_address()", object.id);
                        self.set_address(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_appmenu#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Set or update the service name and object path."]
            #[doc = "Strings should be formatted in Latin-1 matching the relevant DBus specifications."]
            async fn set_address(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                service_name: String,
                object_path: String,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod blur {
    pub mod org_kde_kwin_blur_manager {
        #[doc = "Trait to implement the org_kde_kwin_blur_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinBlurManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_blur_manager";
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
                        tracing::debug!("org_kde_kwin_blur_manager#{}.create()", object.id);
                        self.create(
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
                        tracing::debug!("org_kde_kwin_blur_manager#{}.unset()", object.id);
                        self.unset(
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn unset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod org_kde_kwin_blur {
        #[doc = "Trait to implement the org_kde_kwin_blur interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinBlur: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_blur";
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
                        tracing::debug!("org_kde_kwin_blur#{}.commit()", object.id);
                        self.commit(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_blur#{}.set_region()", object.id);
                        self.set_region(object, client, message.object()?).await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_kwin_blur#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn commit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn set_region(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod contrast {
    pub mod org_kde_kwin_contrast_manager {
        #[doc = "Trait to implement the org_kde_kwin_contrast_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinContrastManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_contrast_manager";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("org_kde_kwin_contrast_manager#{}.create()", object.id);
                        self.create(
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
                        tracing::debug!("org_kde_kwin_contrast_manager#{}.unset()", object.id);
                        self.unset(
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn unset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod org_kde_kwin_contrast {
        #[doc = "Trait to implement the org_kde_kwin_contrast interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinContrast: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_contrast";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.commit()", object.id);
                        self.commit(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.set_region()", object.id);
                        self.set_region(object, client, message.object()?).await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.set_contrast()", object.id);
                        self.set_contrast(object, client, message.fixed()?).await
                    }
                    3u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.set_intensity()", object.id);
                        self.set_intensity(object, client, message.fixed()?).await
                    }
                    4u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.set_saturation()", object.id);
                        self.set_saturation(object, client, message.fixed()?).await
                    }
                    5u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    6u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.set_frost()", object.id);
                        self.set_frost(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    7u16 => {
                        tracing::debug!("org_kde_kwin_contrast#{}.unset_frost()", object.id);
                        self.unset_frost(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn commit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn set_region(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
            async fn set_contrast(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                contrast: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn set_intensity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                intensity: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn set_saturation(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                saturation: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "enables 'frost' variant of contrast effect."]
            #[doc = ""]
            #[doc = "'frost' is an enhanced version of the contrast effect that"]
            #[doc = "uses different colour arithmetic to get backgrounds simultaneously"]
            #[doc = "higher in contrast and (apparent) transparency."]
            #[doc = ""]
            #[doc = "r, g, b, a are channels from 0-255, indicating a colour to use in contrast calculation."]
            #[doc = "should be based off of the \"main\" background colour of the surface."]
            async fn set_frost(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                red: i32,
                green: i32,
                blue: i32,
                alpha: i32,
            ) -> crate::server::Result<()>;
            async fn unset_frost(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod fullscreen_shell {
    #[doc = "Displays a single surface per output."]
    #[doc = ""]
    #[doc = "This interface provides a mechanism for a single client to display"]
    #[doc = "simple full-screen surfaces.  While there technically may be multiple"]
    #[doc = "clients bound to this interface, only one of those clients should be"]
    #[doc = "shown at a time."]
    #[doc = ""]
    #[doc = "To present a surface, the client uses either the present_surface or"]
    #[doc = "present_surface_for_mode requests.  Presenting a surface takes effect"]
    #[doc = "on the next wl_surface.commit.  See the individual requests for"]
    #[doc = "details about scaling and mode switches."]
    #[doc = ""]
    #[doc = "The client can have at most one surface per output at any time."]
    #[doc = "Requesting a surface be presented on an output that already has a"]
    #[doc = "surface replaces the previously presented surface.  Presenting a null"]
    #[doc = "surface removes its content and effectively disables the output."]
    #[doc = "Exactly what happens when an output is \"disabled\" is"]
    #[doc = "compositor-specific.  The same surface may be presented on multiple"]
    #[doc = "outputs simultaneously."]
    #[doc = ""]
    #[doc = "Once a surface is presented on an output, it stays on that output"]
    #[doc = "until either the client removes it or the compositor destroys the"]
    #[doc = "output.  This way, the client can update the output's contents by"]
    #[doc = "simply attaching a new buffer."]
    pub mod _wl_fullscreen_shell {
        #[doc = "Various capabilities that can be advertised by the compositor.  They"]
        #[doc = "are advertised one-at-a-time when the wl_fullscreen_shell interface is"]
        #[doc = "bound.  See the wl_fullscreen_shell.capability event for more details."]
        #[doc = ""]
        #[doc = "ARBITRARY_MODE:"]
        #[doc = "This is a hint to the client that indicates that the compositor is"]
        #[doc = "capable of setting practically any mode on its outputs.  If this"]
        #[doc = "capability is provided, wl_fullscreen_shell.present_surface_for_mode"]
        #[doc = "will almost never fail and clients should feel free to set whatever"]
        #[doc = "mode they like.  If the compositor does not advertise this, it may"]
        #[doc = "still support some modes that are not advertised through wl_global.mode"]
        #[doc = "but it is less likely."]
        #[doc = ""]
        #[doc = "CURSOR_PLANE:"]
        #[doc = "This is a hint to the client that indicates that the compositor can"]
        #[doc = "handle a cursor surface from the client without actually compositing."]
        #[doc = "This may be because of a hardware cursor plane or some other mechanism."]
        #[doc = "If the compositor does not advertise this capability then setting"]
        #[doc = "wl_pointer.cursor may degrade performance or be ignored entirely.  If"]
        #[doc = "CURSOR_PLANE is not advertised, it is recommended that the client draw"]
        #[doc = "its own cursor and set wl_pointer.cursor(NULL)."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Capability {
            #[doc = "compositor is capable of almost any output mode"]
            ArbitraryModes = 1u32,
            #[doc = "compositor has a separate cursor plane"]
            CursorPlane = 2u32,
        }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::ArbitraryModes),
                    2u32 => Ok(Self::CursorPlane),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Hints to indicate to the compositor how to deal with a conflict"]
        #[doc = "between the dimensions of the surface and the dimensions of the"]
        #[doc = "output. The compositor is free to ignore this parameter."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PresentMethod {
            #[doc = "no preference, apply default policy"]
            Default = 0u32,
            #[doc = "center the surface on the output"]
            Center = 1u32,
            #[doc = "scale the surface, preserving aspect ratio, to the largest size that will fit on the output"]
            Zoom = 2u32,
            #[doc = "scale the surface, preserving aspect ratio, to fully fill the output cropping if needed"]
            ZoomCrop = 3u32,
            #[doc = "scale the surface to the size of the output ignoring aspect ratio"]
            Stretch = 4u32,
        }
        impl TryFrom<u32> for PresentMethod {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Default),
                    1u32 => Ok(Self::Center),
                    2u32 => Ok(Self::Zoom),
                    3u32 => Ok(Self::ZoomCrop),
                    4u32 => Ok(Self::Stretch),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "These errors can be emitted in response to wl_fullscreen_shell requests"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "present_method is not known"]
            InvalidMethod = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidMethod),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the _wl_fullscreen_shell interface. See the module level documentation for more info"]
        pub trait WlFullscreenShell: crate::server::Dispatcher {
            const INTERFACE: &'static str = "_wl_fullscreen_shell";
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
                        tracing::debug!("_wl_fullscreen_shell#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("_wl_fullscreen_shell#{}.present_surface()", object.id);
                        self.present_surface(
                            object,
                            client,
                            message.object()?,
                            message.uint()?,
                            message.object()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "_wl_fullscreen_shell#{}.present_surface_for_mode()",
                            object.id
                        );
                        self.present_surface_for_mode(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Release the binding from the wl_fullscreen_shell interface"]
            #[doc = ""]
            #[doc = "This destroys the server-side object and frees this binding.  If"]
            #[doc = "the client binds to wl_fullscreen_shell multiple times, it may wish"]
            #[doc = "to free some of those bindings."]
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Present a surface on the given output."]
            #[doc = ""]
            #[doc = "If the output is null, the compositor will present the surface on"]
            #[doc = "whatever display (or displays) it thinks best.  In particular, this"]
            #[doc = "may replace any or all surfaces currently presented so it should"]
            #[doc = "not be used in combination with placing surfaces on specific"]
            #[doc = "outputs."]
            #[doc = ""]
            #[doc = "The method parameter is a hint to the compositor for how the surface"]
            #[doc = "is to be presented.  In particular, it tells the compostior how to"]
            #[doc = "handle a size mismatch between the presented surface and the"]
            #[doc = "output.  The compositor is free to ignore this parameter."]
            #[doc = ""]
            #[doc = "The \"zoom\", \"zoom_crop\", and \"stretch\" methods imply a scaling"]
            #[doc = "operation on the surface.  This will override any kind of output"]
            #[doc = "scaling, so the buffer_scale property of the surface is effectively"]
            #[doc = "ignored."]
            async fn present_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: Option<crate::wire::ObjectId>,
                method: u32,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
            #[doc = "Presents a surface on the given output for a particular mode."]
            #[doc = ""]
            #[doc = "If the current size of the output differs from that of the surface,"]
            #[doc = "the compositor will attempt to change the size of the output to"]
            #[doc = "match the surface.  The result of the mode-switch operation will be"]
            #[doc = "returned via the provided wl_fullscreen_shell_mode_feedback object."]
            #[doc = ""]
            #[doc = "If the current output mode matches the one requested or if the"]
            #[doc = "compositor successfully switches the mode to match the surface,"]
            #[doc = "then the mode_successful event will be sent and the output will"]
            #[doc = "contain the contents of the given surface.  If the compositor"]
            #[doc = "cannot match the output size to the surface size, the mode_failed"]
            #[doc = "will be sent and the output will contain the contents of the"]
            #[doc = "previously presented surface (if any).  If another surface is"]
            #[doc = "presented on the given output before either of these has a chance"]
            #[doc = "to happen, the present_cancelled event will be sent."]
            #[doc = ""]
            #[doc = "Due to race conditions and other issues unknown to the client, no"]
            #[doc = "mode-switch operation is guaranteed to succeed.  However, if the"]
            #[doc = "mode is one advertised by wl_output.mode or if the compositor"]
            #[doc = "advertises the ARBITRARY_MODES capability, then the client should"]
            #[doc = "expect that the mode-switch operation will usually succeed."]
            #[doc = ""]
            #[doc = "If the size of the presented surface changes, the resulting output"]
            #[doc = "is undefined.  The compositor may attempt to change the output mode"]
            #[doc = "to compensate.  However, there is no guarantee that a suitable mode"]
            #[doc = "will be found and the client has no way to be notified of success"]
            #[doc = "or failure."]
            #[doc = ""]
            #[doc = "The framerate parameter specifies the desired framerate for the"]
            #[doc = "output in mHz.  The compositor is free to ignore this parameter.  A"]
            #[doc = "value of 0 indicates that the client has no preference."]
            #[doc = ""]
            #[doc = "If the value of wl_output.scale differs from wl_surface.buffer_scale,"]
            #[doc = "then the compositor may choose a mode that matches either the buffer"]
            #[doc = "size or the surface size.  In either case, the surface will fill the"]
            #[doc = "output."]
            async fn present_surface_for_mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                framerate: i32,
                feedback: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Advertises a single capability of the compositor."]
            #[doc = ""]
            #[doc = "When the wl_fullscreen_shell interface is bound, this event is emitted"]
            #[doc = "once for each capability advertised.  Valid capabilities are given by"]
            #[doc = "the wl_fullscreen_shell.capability enum.  If clients want to take"]
            #[doc = "advantage of any of these capabilities, they should use a"]
            #[doc = "wl_display.sync request immediately after binding to ensure that they"]
            #[doc = "receive all the capability events."]
            async fn capability(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                capability: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> _wl_fullscreen_shell#{}.capability()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(capability)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    pub mod _wl_fullscreen_shell_mode_feedback {
        #[doc = "Trait to implement the _wl_fullscreen_shell_mode_feedback interface. See the module level documentation for more info"]
        pub trait WlFullscreenShellModeFeedback: crate::server::Dispatcher {
            const INTERFACE: &'static str = "_wl_fullscreen_shell_mode_feedback";
            const VERSION: u32 = 1u32;
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
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This event indicates that the attempted mode switch operation was"]
            #[doc = "successful.  A surface of the size requested in the mode switch"]
            #[doc = "will fill the output without scaling."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            async fn mode_successful(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> _wl_fullscreen_shell_mode_feedback#{}.mode_successful()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates that the attempted mode switch operation"]
            #[doc = "failed. This may be because the requested output mode is not"]
            #[doc = "possible or it may mean that the compositor does not want to allow it."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            async fn mode_failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> _wl_fullscreen_shell_mode_feedback#{}.mode_failed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates that the attempted mode switch operation was"]
            #[doc = "cancelled.  Most likely this is because the client requested a"]
            #[doc = "second mode switch before the first one completed."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            async fn present_cancelled(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> _wl_fullscreen_shell_mode_feedback#{}.present_cancelled()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod idle {
    #[doc = "This interface allows to monitor user idle time on a given seat. The interface"]
    #[doc = "allows to register timers which trigger after no user activity was registered"]
    #[doc = "on the seat for a given interval. It notifies when user activity resumes."]
    #[doc = ""]
    #[doc = "This is useful for applications wanting to perform actions when the user is not"]
    #[doc = "interacting with the system, e.g. chat applications setting the user as away, power"]
    #[doc = "management features to dim screen, etc.."]
    pub mod org_kde_kwin_idle {
        #[doc = "Trait to implement the org_kde_kwin_idle interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinIdle: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_idle";
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
                        tracing::debug!("org_kde_kwin_idle#{}.get_idle_timeout()", object.id);
                        self.get_idle_timeout(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn get_idle_timeout(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                timeout: u32,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod org_kde_kwin_idle_timeout {
        #[doc = "Trait to implement the org_kde_kwin_idle_timeout interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinIdleTimeout: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_idle_timeout";
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
                        tracing::debug!("org_kde_kwin_idle_timeout#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "org_kde_kwin_idle_timeout#{}.simulate_user_activity()",
                            object.id
                        );
                        self.simulate_user_activity(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn simulate_user_activity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn idle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_idle_timeout#{}.idle()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn resumed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_idle_timeout#{}.resumed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod keystate {
    #[doc = "Keeps track of the states of the different keys that have a state attached to it."]
    pub mod org_kde_kwin_keystate {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Key {
            Capslock = 0u32,
            Numlock = 1u32,
            Scrolllock = 2u32,
        }
        impl TryFrom<u32> for Key {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Capslock),
                    1u32 => Ok(Self::Numlock),
                    2u32 => Ok(Self::Scrolllock),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            Unlocked = 0u32,
            Latched = 1u32,
            Locked = 2u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unlocked),
                    1u32 => Ok(Self::Latched),
                    2u32 => Ok(Self::Locked),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_keystate interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinKeystate: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_keystate";
            const VERSION: u32 = 4u32;
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
                        tracing::debug!("org_kde_kwin_keystate#{}.fetch_states()", object.id);
                        self.fetch_states(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_keystate#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn fetch_states(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn state_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                key: u32,
                state: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_keystate#{}.state_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(key)
                    .put_uint(state)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod outputmanagement {
    #[doc = "This interface enables clients to set properties of output devices for screen"]
    #[doc = "configuration purposes via the server. To this end output devices are referenced"]
    #[doc = "by global org_kde_kwin_outputdevice objects."]
    #[doc = ""]
    #[doc = "outputmanagement (wl_global)"]
    #[doc = "--------------------------"]
    #[doc = "request:"]
    #[doc = "* create_configuration -> outputconfiguration (wl_resource)"]
    #[doc = ""]
    #[doc = "outputconfiguration (wl_resource)"]
    #[doc = "--------------------------"]
    #[doc = "requests:"]
    #[doc = "* enable(outputdevice, bool)"]
    #[doc = "* mode(outputdevice, mode_id)"]
    #[doc = "* transformation(outputdevice, flag)"]
    #[doc = "* position(outputdevice, x, y)"]
    #[doc = "* apply"]
    #[doc = ""]
    #[doc = "events:"]
    #[doc = "* applied"]
    #[doc = "* failed"]
    #[doc = ""]
    #[doc = "The server registers one outputmanagement object as a global object. In order"]
    #[doc = "to configure outputs a client requests create_configuration, which provides a"]
    #[doc = "resource referencing an outputconfiguration for one-time configuration. That"]
    #[doc = "way the server knows which requests belong together and can group them by that."]
    #[doc = ""]
    #[doc = "On the outputconfiguration object the client calls for each output whether the"]
    #[doc = "output should be enabled, which mode should be set (by referencing the mode from"]
    #[doc = "the list of announced modes) and the output's global position. Once all outputs"]
    #[doc = "are configured that way, the client calls apply."]
    #[doc = "At that point and not earlier the server should try to apply the configuration."]
    #[doc = "If this succeeds the server emits the applied signal, otherwise the failed"]
    #[doc = "signal, such that the configuring client is noticed about the success of its"]
    #[doc = "configuration request."]
    #[doc = ""]
    #[doc = "Through this design the interface enables atomic output configuration changes if"]
    #[doc = "internally supported by the server."]
    pub mod org_kde_kwin_outputmanagement {
        #[doc = "Trait to implement the org_kde_kwin_outputmanagement interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinOutputmanagement: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_outputmanagement";
            const VERSION: u32 = 4u32;
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
                            "org_kde_kwin_outputmanagement#{}.create_configuration()",
                            object.id
                        );
                        self.create_configuration(
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
            #[doc = "Request an outputconfiguration object through which the client can configure"]
            #[doc = "output devices."]
            async fn create_configuration(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "outputconfiguration is a client-specific resource that can be used to ask"]
    #[doc = "the server to apply changes to available output devices."]
    #[doc = ""]
    #[doc = "The client receives a list of output devices from the registry. When it wants"]
    #[doc = "to apply new settings, it creates a configuration object from the"]
    #[doc = "outputmanagement global, writes changes through this object's enable, scale,"]
    #[doc = "transform and mode calls. It then asks the server to apply these settings in"]
    #[doc = "an atomic fashion, for example through Linux' DRM interface."]
    #[doc = ""]
    #[doc = "The server signals back whether the new settings have applied successfully"]
    #[doc = "or failed to apply. outputdevice objects are updated after the changes have been"]
    #[doc = "applied to the hardware and before the server side sends the applied event."]
    pub mod org_kde_kwin_outputconfiguration {
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_outputconfiguration interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinOutputconfiguration: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_outputconfiguration";
            const VERSION: u32 = 4u32;
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
                        tracing::debug!("org_kde_kwin_outputconfiguration#{}.enable()", object.id);
                        self.enable(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_outputconfiguration#{}.mode()", object.id);
                        self.mode(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "org_kde_kwin_outputconfiguration#{}.transform()",
                            object.id
                        );
                        self.transform(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!(
                            "org_kde_kwin_outputconfiguration#{}.position()",
                            object.id
                        );
                        self.position(
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
                    4u16 => {
                        tracing::debug!("org_kde_kwin_outputconfiguration#{}.scale()", object.id);
                        self.scale(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!("org_kde_kwin_outputconfiguration#{}.apply()", object.id);
                        self.apply(object, client).await
                    }
                    6u16 => {
                        tracing::debug!("org_kde_kwin_outputconfiguration#{}.scalef()", object.id);
                        self.scalef(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.fixed()?,
                        )
                        .await
                    }
                    7u16 => {
                        tracing::debug!(
                            "org_kde_kwin_outputconfiguration#{}.colorcurves()",
                            object.id
                        );
                        self.colorcurves(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.array()?,
                            message.array()?,
                            message.array()?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!("org_kde_kwin_outputconfiguration#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    9u16 => {
                        tracing::debug!(
                            "org_kde_kwin_outputconfiguration#{}.overscan()",
                            object.id
                        );
                        self.overscan(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    10u16 => {
                        tracing::debug!(
                            "org_kde_kwin_outputconfiguration#{}.set_vrr_policy()",
                            object.id
                        );
                        self.set_vrr_policy(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Mark the output as enabled or disabled."]
            async fn enable(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                enable: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the mode for a given output by its mode size (width and height) and refresh rate."]
            async fn mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                mode_id: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the transformation for a given output."]
            async fn transform(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                transform: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the position for this output device. (x,y) describe the top-left corner"]
            #[doc = "of the output in global space, whereby the origin (0,0) of the global space"]
            #[doc = "has to be aligned with the top-left corner of the most left and in case this"]
            #[doc = "does not define a single one the top output."]
            #[doc = ""]
            #[doc = "There may be no gaps or overlaps between outputs, i.e. the outputs are"]
            #[doc = "stacked horizontally, vertically, or both on each other."]
            async fn position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the scaling factor for this output device."]
            async fn scale(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                scale: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Asks the server to apply property changes requested through this outputconfiguration"]
            #[doc = "object to all outputs on the server side."]
            async fn apply(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the scaling factor for this output device."]
            #[doc = "Sending both scale and scalef is undefined."]
            async fn scalef(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                scale: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "Set color curves of output devices through RGB color ramps. Allows color"]
            #[doc = "correction of output device from user space."]
            #[doc = ""]
            #[doc = "These are the raw values. A compositor might opt to adjust these values"]
            #[doc = "internally, for example to shift color temperature at night."]
            async fn colorcurves(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                red: Vec<u8>,
                green: Vec<u8>,
                blue: Vec<u8>,
            ) -> crate::server::Result<()>;
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Set the overscan value of this output device with a value in percent."]
            async fn overscan(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                overscan: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Set what policy the compositor should employ regarding its use of"]
            #[doc = "variable refresh rate."]
            async fn set_vrr_policy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                policy: VrrPolicy,
            ) -> crate::server::Result<()>;
            #[doc = "Sent after the server has successfully applied the changes."]
            #[doc = "."]
            async fn applied(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_kwin_outputconfiguration#{}.applied()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent if the server rejects the changes or failed to apply them."]
            async fn failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputconfiguration#{}.failed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod org_kde_kwin_outputdevice {
    #[doc = "An outputdevice describes a display device available to the compositor."]
    #[doc = "outputdevice is similar to wl_output, but focuses on output"]
    #[doc = "configuration management."]
    #[doc = ""]
    #[doc = "A client can query all global outputdevice objects to enlist all"]
    #[doc = "available display devices, even those that may currently not be"]
    #[doc = "represented by the compositor as a wl_output."]
    #[doc = ""]
    #[doc = "The client sends configuration changes to the server through the"]
    #[doc = "outputconfiguration interface, and the server applies the configuration"]
    #[doc = "changes to the hardware and signals changes to the outputdevices"]
    #[doc = "accordingly."]
    #[doc = ""]
    #[doc = "This object is published as global during start up for every available"]
    #[doc = "display devices, or when one later becomes available, for example by"]
    #[doc = "being hotplugged via a physical connector."]
    pub mod org_kde_kwin_outputdevice {
        #[doc = "This enumeration describes how the physical pixels on an output are"]
        #[doc = "laid out."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Subpixel {
            Unknown = 0u32,
            None = 1u32,
            HorizontalRgb = 2u32,
            HorizontalBgr = 3u32,
            VerticalRgb = 4u32,
            VerticalBgr = 5u32,
        }
        impl TryFrom<u32> for Subpixel {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unknown),
                    1u32 => Ok(Self::None),
                    2u32 => Ok(Self::HorizontalRgb),
                    3u32 => Ok(Self::HorizontalBgr),
                    4u32 => Ok(Self::VerticalRgb),
                    5u32 => Ok(Self::VerticalBgr),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "This describes the transform, that a compositor will apply to a"]
        #[doc = "surface to compensate for the rotation or mirroring of an"]
        #[doc = "output device."]
        #[doc = ""]
        #[doc = "The flipped values correspond to an initial flip around a"]
        #[doc = "vertical axis followed by rotation."]
        #[doc = ""]
        #[doc = "The purpose is mainly to allow clients to render accordingly and"]
        #[doc = "tell the compositor, so that for fullscreen surfaces, the"]
        #[doc = "compositor is still able to scan out directly client surfaces."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Transform {
            Normal = 0u32,
            _90 = 1u32,
            _180 = 2u32,
            _270 = 3u32,
            Flipped = 4u32,
            Flipped90 = 5u32,
            Flipped180 = 6u32,
            Flipped270 = 7u32,
        }
        impl TryFrom<u32> for Transform {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::_90),
                    2u32 => Ok(Self::_180),
                    3u32 => Ok(Self::_270),
                    4u32 => Ok(Self::Flipped),
                    5u32 => Ok(Self::Flipped90),
                    6u32 => Ok(Self::Flipped180),
                    7u32 => Ok(Self::Flipped270),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "These flags describe properties of an output mode. They are"]
        #[doc = "used in the flags bitfield of the mode event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "indicates this is the current mode"]
            Current = 1u32,
            #[doc = "indicates this is the preferred mode"]
            Preferred = 2u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Current),
                    2u32 => Ok(Self::Preferred),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Describes whether a device is enabled, i.e. device is used to"]
        #[doc = "display content by the compositor. This wraps a boolean around"]
        #[doc = "an int to avoid a boolean trap."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Enablement {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl TryFrom<u32> for Enablement {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        bitflags::bitflags! { # [doc = "Describes what capabilities this device has."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "if this outputdevice can use overscan"] const Overscan = 1u32 ; # [doc = "if this outputdevice supports variable refresh rate"] const Vrr = 2u32 ; } }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_outputdevice interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinOutputdevice: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_outputdevice";
            const VERSION: u32 = 4u32;
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
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "The geometry event describes geometric properties of the output."]
            #[doc = "The event is sent when binding to the output object and whenever"]
            #[doc = "any of the properties change."]
            async fn geometry(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                physical_width: i32,
                physical_height: i32,
                subpixel: i32,
                make: String,
                model: String,
                transform: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.geometry()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(physical_width)
                    .put_int(physical_height)
                    .put_int(subpixel)
                    .put_string(Some(make))
                    .put_string(Some(model))
                    .put_int(transform)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The mode event describes an available mode for the output."]
            #[doc = ""]
            #[doc = "When the client binds to the outputdevice object, the server sends this"]
            #[doc = "event once for every available mode the outputdevice can be operated by."]
            #[doc = ""]
            #[doc = "There will always be at least one event sent out on initial binding,"]
            #[doc = "which represents the current mode."]
            #[doc = ""]
            #[doc = "Later on if an output changes its mode the event is sent again, whereby"]
            #[doc = "this event represents the mode that has now become current. In other"]
            #[doc = "words, the current mode is always represented by the latest event sent"]
            #[doc = "with the current flag set."]
            #[doc = ""]
            #[doc = "The size of a mode is given in physical hardware units of the output device."]
            #[doc = "This is not necessarily the same as the output size in the global compositor"]
            #[doc = "space. For instance, the output may be scaled, as described in"]
            #[doc = "org_kde_kwin_outputdevice.scale, or transformed, as described in"]
            #[doc = "org_kde_kwin_outputdevice.transform."]
            #[doc = ""]
            #[doc = "The id can be used to refer to a mode when calling set_mode on an"]
            #[doc = "org_kde_kwin_outputconfiguration object."]
            async fn mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flags: u32,
                width: i32,
                height: i32,
                refresh: i32,
                mode_id: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.mode()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(flags)
                    .put_int(width)
                    .put_int(height)
                    .put_int(refresh)
                    .put_int(mode_id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all other properties have been"]
            #[doc = "sent on binding to the output object as well as after any"]
            #[doc = "other output property change have been applied later on."]
            #[doc = "This allows to see changes to the output properties as atomic,"]
            #[doc = "even if multiple events successively announce them."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. If it is not sent, the client should assume a"]
            #[doc = "scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients track the"]
            #[doc = "current output of a surface, and if it is on a scaled"]
            #[doc = "output it should use wl_surface.set_buffer_scale with"]
            #[doc = "the scale of the output. That way the compositor can"]
            #[doc = "avoid scaling the surface, and the client can supply"]
            #[doc = "a higher detail image."]
            async fn scale(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                factor: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.scale()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(factor).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The edid event encapsulates the EDID data for the outputdevice."]
            #[doc = ""]
            #[doc = "The event is sent when binding to the output object. The EDID"]
            #[doc = "data may be empty, in which case this event is sent anyway."]
            #[doc = "If the EDID information is empty, you can fall back to the name"]
            #[doc = "et al. properties of the outputdevice."]
            async fn edid(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                raw: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.edid()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(raw))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The enabled event notifies whether this output is currently"]
            #[doc = "enabled and used for displaying content by the server."]
            #[doc = "The event is sent when binding to the output object and"]
            #[doc = "whenever later on an output changes its state by becoming"]
            #[doc = "enabled or disabled."]
            async fn enabled(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                enabled: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.enabled()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(enabled).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The uuid can be used to identify the output. It's controlled by"]
            #[doc = "the server entirely. The server should make sure the uuid is"]
            #[doc = "persistent across restarts. An empty uuid is considered invalid."]
            async fn uuid(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                uuid: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.uuid()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(uuid))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. If it is not sent, the client should assume a"]
            #[doc = "scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients track the"]
            #[doc = "current output of a surface, and if it is on a scaled"]
            #[doc = "output it should use wl_surface.set_buffer_scale with"]
            #[doc = "the scale of the output. That way the compositor can"]
            #[doc = "avoid scaling the surface, and the client can supply"]
            #[doc = "a higher detail image."]
            #[doc = ""]
            #[doc = "wl_output will keep the output scale as an integer. In every situation except"]
            #[doc = "configuring the window manager you want to use that."]
            async fn scalef(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                factor: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.scalef()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fixed(factor).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Describes the color intensity profile of the output."]
            #[doc = "Commonly used for gamma/color correction."]
            #[doc = ""]
            #[doc = "The array contains all color ramp values of the output."]
            #[doc = "For example on 8bit screens there are 256 of them."]
            #[doc = ""]
            #[doc = "The array elements are unsigned 16bit integers."]
            async fn colorcurves(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                red: Vec<u8>,
                green: Vec<u8>,
                blue: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.colorcurves()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_array(red)
                    .put_array(green)
                    .put_array(blue)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Serial ID of the monitor, sent on startup before the first done event."]
            async fn serial_number(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial_number: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.serial_number()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(serial_number))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "EISA ID of the monitor, sent on startup before the first done event."]
            async fn eisa_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                eisa_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.eisa_id()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(eisa_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "What capabilities this device has, sent on startup before the first"]
            #[doc = "done event."]
            async fn capabilities(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flags: Capability,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.capabilities()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(flags.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 11u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Overscan value of the monitor in percent, sent on startup before the"]
            #[doc = "first done event."]
            async fn overscan(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                overscan: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.overscan()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(overscan)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 12u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "What policy the compositor will employ regarding its use of variable"]
            #[doc = "refresh rate."]
            async fn vrr_policy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                vrr_policy: VrrPolicy,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_outputdevice#{}.vrr_policy()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(vrr_policy as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 13u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod remote_access {
    pub mod org_kde_kwin_remote_access_manager {
        #[doc = "Trait to implement the org_kde_kwin_remote_access_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinRemoteAccessManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_remote_access_manager";
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
                            "org_kde_kwin_remote_access_manager#{}.get_buffer()",
                            object.id
                        );
                        self.get_buffer(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "org_kde_kwin_remote_access_manager#{}.release()",
                            object.id
                        );
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn get_buffer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
                internal_buffer_id: i32,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn buffer_ready(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: i32,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_kwin_remote_access_manager#{}.buffer_ready()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(id)
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    pub mod org_kde_kwin_remote_buffer {
        #[doc = "Trait to implement the org_kde_kwin_remote_buffer interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinRemoteBuffer: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_remote_buffer";
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
                        tracing::debug!("org_kde_kwin_remote_buffer#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn gbm_handle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                fd: rustix::fd::OwnedFd,
                width: u32,
                height: u32,
                stride: u32,
                format: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_remote_buffer#{}.gbm_handle()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fd(fd)
                    .put_uint(width)
                    .put_uint(height)
                    .put_uint(stride)
                    .put_uint(format)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod server_decoration_palette {
    #[doc = "This interface allows a client to alter the palette of a server side decoration."]
    pub mod org_kde_kwin_server_decoration_palette_manager {
        #[doc = "Trait to implement the org_kde_kwin_server_decoration_palette_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecorationPaletteManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_palette_manager";
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
                            "org_kde_kwin_server_decoration_palette_manager#{}.create()",
                            object.id
                        );
                        self.create(
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This interface allows a client to alter the palette of a server side decoration."]
    pub mod org_kde_kwin_server_decoration_palette {
        #[doc = "Trait to implement the org_kde_kwin_server_decoration_palette interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecorationPalette: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_palette";
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
                            "org_kde_kwin_server_decoration_palette#{}.set_palette()",
                            object.id
                        );
                        self.set_palette(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "org_kde_kwin_server_decoration_palette#{}.release()",
                            object.id
                        );
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Color scheme that should be applied to the window decoration."]
            #[doc = "Absolute file path, or name of palette in the user's config directory."]
            #[doc = "The server may choose not to follow the requested style."]
            async fn set_palette(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                palette: String,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod server_decoration {
    #[doc = "This interface allows to coordinate whether the server should create"]
    #[doc = "a server-side window decoration around a wl_surface representing a"]
    #[doc = "shell surface (wl_shell_surface or similar). By announcing support"]
    #[doc = "for this interface the server indicates that it supports server"]
    #[doc = "side decorations."]
    #[doc = ""]
    #[doc = "Use in conjunction with zxdg_decoration_manager_v1 is undefined."]
    pub mod org_kde_kwin_server_decoration_manager {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "Undecorated: The surface is not decorated at all, neither server nor client-side. An example is a popup surface which should not be decorated."]
            None = 0u32,
            #[doc = "Client-side decoration: The decoration is part of the surface and the client."]
            Client = 1u32,
            #[doc = "Server-side decoration: The server embeds the surface into a decoration frame."]
            Server = 2u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Client),
                    2u32 => Ok(Self::Server),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_server_decoration_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecorationManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_manager";
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
                            "org_kde_kwin_server_decoration_manager#{}.create()",
                            object.id
                        );
                        self.create(
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
            #[doc = "When a client creates a server-side decoration object it indicates"]
            #[doc = "that it supports the protocol. The client is supposed to tell the"]
            #[doc = "server whether it wants server-side decorations or will provide"]
            #[doc = "client-side decorations."]
            #[doc = ""]
            #[doc = "If the client does not create a server-side decoration object for"]
            #[doc = "a surface the server interprets this as lack of support for this"]
            #[doc = "protocol and considers it as client-side decorated. Nevertheless a"]
            #[doc = "client-side decorated surface should use this protocol to indicate"]
            #[doc = "to the server that it does not want a server-side deco."]
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event is emitted directly after binding the interface. It contains"]
            #[doc = "the default mode for the decoration. When a new server decoration object"]
            #[doc = "is created this new object will be in the default mode until the first"]
            #[doc = "request_mode is requested."]
            #[doc = ""]
            #[doc = "The server may change the default mode at any time."]
            async fn default_mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_kwin_server_decoration_manager#{}.default_mode()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(mode).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    pub mod org_kde_kwin_server_decoration {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "Undecorated: The surface is not decorated at all, neither server nor client-side. An example is a popup surface which should not be decorated."]
            None = 0u32,
            #[doc = "Client-side decoration: The decoration is part of the surface and the client."]
            Client = 1u32,
            #[doc = "Server-side decoration: The server embeds the surface into a decoration frame."]
            Server = 2u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Client),
                    2u32 => Ok(Self::Server),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_server_decoration interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecoration: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration";
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
                        tracing::debug!("org_kde_kwin_server_decoration#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "org_kde_kwin_server_decoration#{}.request_mode()",
                            object.id
                        );
                        self.request_mode(object, client, message.uint()?).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn request_mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: u32,
            ) -> crate::server::Result<()>;
            #[doc = "This event is emitted directly after the decoration is created and"]
            #[doc = "represents the base decoration policy by the server. E.g. a server"]
            #[doc = "which wants all surfaces to be client-side decorated will send Client,"]
            #[doc = "a server which wants server-side decoration will send Server."]
            #[doc = ""]
            #[doc = "The client can request a different mode through the decoration request."]
            #[doc = "The server will acknowledge this by another event with the same mode. So"]
            #[doc = "even if a server prefers server-side decoration it's possible to force a"]
            #[doc = "client-side decoration."]
            #[doc = ""]
            #[doc = "The server may emit this event at any time. In this case the client can"]
            #[doc = "again request a different mode. It's the responsibility of the server to"]
            #[doc = "prevent a feedback loop."]
            async fn mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_server_decoration#{}.mode()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(mode).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod shadow {
    pub mod org_kde_kwin_shadow_manager {
        #[doc = "Trait to implement the org_kde_kwin_shadow_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinShadowManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_shadow_manager";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("org_kde_kwin_shadow_manager#{}.create()", object.id);
                        self.create(
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
                        tracing::debug!("org_kde_kwin_shadow_manager#{}.unset()", object.id);
                        self.unset(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_kwin_shadow_manager#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn unset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the org_kde_kwin_shadow_manager object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod org_kde_kwin_shadow {
        #[doc = "Trait to implement the org_kde_kwin_shadow interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinShadow: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_shadow";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.commit()", object.id);
                        self.commit(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_left()", object.id);
                        self.attach_left(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_top_left()", object.id);
                        self.attach_top_left(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_top()", object.id);
                        self.attach_top(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_top_right()", object.id);
                        self.attach_top_right(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_right()", object.id);
                        self.attach_right(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    6u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_bottom_right()", object.id);
                        self.attach_bottom_right(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    7u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_bottom()", object.id);
                        self.attach_bottom(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.attach_bottom_left()", object.id);
                        self.attach_bottom_left(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    9u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.set_left_offset()", object.id);
                        self.set_left_offset(object, client, message.fixed()?).await
                    }
                    10u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.set_top_offset()", object.id);
                        self.set_top_offset(object, client, message.fixed()?).await
                    }
                    11u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.set_right_offset()", object.id);
                        self.set_right_offset(object, client, message.fixed()?)
                            .await
                    }
                    12u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.set_bottom_offset()", object.id);
                        self.set_bottom_offset(object, client, message.fixed()?)
                            .await
                    }
                    13u16 => {
                        tracing::debug!("org_kde_kwin_shadow#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn commit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn attach_left(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_top_left(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_top(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_top_right(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_right(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_bottom_right(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_bottom(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn attach_bottom_left(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn set_left_offset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                offset: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn set_top_offset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                offset: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn set_right_offset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                offset: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn set_bottom_offset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                offset: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the org_kde_kwin_shadow object. If the org_kde_kwin_shadow is"]
            #[doc = "still set on a wl_surface the shadow will be immediately removed."]
            #[doc = "Prefer to first call the request unset on the org_kde_kwin_shadow_manager and"]
            #[doc = "commit the wl_surface to apply the change."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod slide {
    pub mod org_kde_kwin_slide_manager {
        #[doc = "Trait to implement the org_kde_kwin_slide_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinSlideManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_slide_manager";
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
                        tracing::debug!("org_kde_kwin_slide_manager#{}.create()", object.id);
                        self.create(
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
                        tracing::debug!("org_kde_kwin_slide_manager#{}.unset()", object.id);
                        self.unset(
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn unset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "Ask the compositor to move the surface from a location to another"]
    #[doc = "with a slide animation."]
    #[doc = ""]
    #[doc = "The from argument provides a clue about where the slide animation"]
    #[doc = "begins, offset is the distance from screen edge to begin the animation."]
    pub mod org_kde_kwin_slide {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Location {
            Left = 0u32,
            Top = 1u32,
            Right = 2u32,
            Bottom = 3u32,
        }
        impl TryFrom<u32> for Location {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Left),
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Right),
                    3u32 => Ok(Self::Bottom),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_slide interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinSlide: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_slide";
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
                        tracing::debug!("org_kde_kwin_slide#{}.commit()", object.id);
                        self.commit(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_slide#{}.set_location()", object.id);
                        self.set_location(object, client, message.uint()?).await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_kwin_slide#{}.set_offset()", object.id);
                        self.set_offset(object, client, message.int()?).await
                    }
                    3u16 => {
                        tracing::debug!("org_kde_kwin_slide#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn commit(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn set_location(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                location: u32,
            ) -> crate::server::Result<()>;
            async fn set_offset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                offset: i32,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod surface_extension {
    pub mod qt_surface_extension {
        #[doc = "Trait to implement the qt_surface_extension interface. See the module level documentation for more info"]
        pub trait QtSurfaceExtension: crate::server::Dispatcher {
            const INTERFACE: &'static str = "qt_surface_extension";
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
                            "qt_surface_extension#{}.get_extended_surface()",
                            object.id
                        );
                        self.get_extended_surface(
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
            async fn get_extended_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod qt_extended_surface {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Orientation {
            PrimaryOrientation = 0u32,
            PortraitOrientation = 1u32,
            LandscapeOrientation = 2u32,
            InvertedPortraitOrientation = 4u32,
            InvertedLandscapeOrientation = 8u32,
        }
        impl TryFrom<u32> for Orientation {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PrimaryOrientation),
                    1u32 => Ok(Self::PortraitOrientation),
                    2u32 => Ok(Self::LandscapeOrientation),
                    4u32 => Ok(Self::InvertedPortraitOrientation),
                    8u32 => Ok(Self::InvertedLandscapeOrientation),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Windowflag {
            OverridesSystemGestures = 1u32,
            StaysOnTop = 2u32,
            BypassWindowManager = 4u32,
        }
        impl TryFrom<u32> for Windowflag {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::OverridesSystemGestures),
                    2u32 => Ok(Self::StaysOnTop),
                    4u32 => Ok(Self::BypassWindowManager),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the qt_extended_surface interface. See the module level documentation for more info"]
        pub trait QtExtendedSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "qt_extended_surface";
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
                            "qt_extended_surface#{}.update_generic_property()",
                            object.id
                        );
                        self.update_generic_property(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.array()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "qt_extended_surface#{}.set_content_orientation_mask()",
                            object.id
                        );
                        self.set_content_orientation_mask(object, client, message.int()?)
                            .await
                    }
                    2u16 => {
                        tracing::debug!("qt_extended_surface#{}.set_window_flags()", object.id);
                        self.set_window_flags(object, client, message.int()?).await
                    }
                    3u16 => {
                        tracing::debug!("qt_extended_surface#{}.raise()", object.id);
                        self.raise(object, client).await
                    }
                    4u16 => {
                        tracing::debug!("qt_extended_surface#{}.lower()", object.id);
                        self.lower(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn update_generic_property(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                value: Vec<u8>,
            ) -> crate::server::Result<()>;
            async fn set_content_orientation_mask(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                orientation: i32,
            ) -> crate::server::Result<()>;
            async fn set_window_flags(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flags: i32,
            ) -> crate::server::Result<()>;
            async fn raise(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn lower(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn onscreen_visibility(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                visible: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> qt_extended_surface#{}.onscreen_visibility()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(visible).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn set_generic_property(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                value: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> qt_extended_surface#{}.set_generic_property()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .put_array(value)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn close(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> qt_extended_surface#{}.close()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod text_input_unstable_v2 {
    #[doc = "The zwp_text_input_v2 interface represents text input and input methods"]
    #[doc = "associated with a seat. It provides enter/leave events to follow the"]
    #[doc = "text input focus for a seat."]
    #[doc = ""]
    #[doc = "Requests are used to enable/disable the text-input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about the entered text is sent to the text-input object"]
    #[doc = "via the pre-edit and commit events. Using this interface removes the need"]
    #[doc = "for applications to directly process hardware key events and compose text"]
    #[doc = "out of them."]
    #[doc = ""]
    #[doc = "Text is valid UTF-8 encoded, indices and lengths are in bytes. Indices"]
    #[doc = "have to always point to the first byte of an UTF-8 encoded code point."]
    #[doc = "Lengths are not allowed to contain just a part of an UTF-8 encoded code"]
    #[doc = "point."]
    #[doc = ""]
    #[doc = "State is sent by the state requests (set_surrounding_text,"]
    #[doc = "set_content_type, set_cursor_rectangle and set_preferred_language) and"]
    #[doc = "an update_state request. After an enter or an input_method_change event"]
    #[doc = "all state information is invalidated and needs to be resent from the"]
    #[doc = "client. A reset or entering a new widget on client side also"]
    #[doc = "invalidates all current state information."]
    pub mod zwp_text_input_v2 {
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behaviour"] const None = 0u32 ; # [doc = "suggest word completions"] const AutoCompletion = 1u32 ; # [doc = "suggest word corrections"] const AutoCorrection = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
        impl TryFrom<u32> for ContentHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
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
            #[doc = "input a password (combine with password or sensitive_data hint)"]
            Password = 8u32,
            #[doc = "input a date"]
            Date = 9u32,
            #[doc = "input a time"]
            Time = 10u32,
            #[doc = "input a date and time"]
            Datetime = 11u32,
            #[doc = "input for a terminal"]
            Terminal = 12u32,
        }
        impl TryFrom<u32> for ContentPurpose {
            type Error = crate::wire::DecodeError;
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
                    9u32 => Ok(Self::Date),
                    10u32 => Ok(Self::Time),
                    11u32 => Ok(Self::Datetime),
                    12u32 => Ok(Self::Terminal),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Defines the reason for sending an updated state."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum UpdateState {
            #[doc = "updated state because it changed"]
            Change = 0u32,
            #[doc = "full state after enter or input_method_changed event"]
            Full = 1u32,
            #[doc = "full state after reset"]
            Reset = 2u32,
            #[doc = "full state after switching focus to a different widget on client side"]
            Enter = 3u32,
        }
        impl TryFrom<u32> for UpdateState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Change),
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Reset),
                    3u32 => Ok(Self::Enter),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum InputPanelVisibility {
            #[doc = "the input panel (virtual keyboard) is hidden"]
            Hidden = 0u32,
            #[doc = "the input panel (virtual keyboard) is visible"]
            Visible = 1u32,
        }
        impl TryFrom<u32> for InputPanelVisibility {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Hidden),
                    1u32 => Ok(Self::Visible),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PreeditStyle {
            #[doc = "default style for composing text"]
            Default = 0u32,
            #[doc = "composing text should be shown the same as non-composing text"]
            None = 1u32,
            #[doc = "composing text might be bold"]
            Active = 2u32,
            #[doc = "composing text might be cursive"]
            Inactive = 3u32,
            #[doc = "composing text might have a different background color"]
            Highlight = 4u32,
            #[doc = "composing text might be underlined"]
            Underline = 5u32,
            #[doc = "composing text should be shown the same as selected text"]
            Selection = 6u32,
            #[doc = "composing text might be underlined with a red wavy line"]
            Incorrect = 7u32,
        }
        impl TryFrom<u32> for PreeditStyle {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Default),
                    1u32 => Ok(Self::None),
                    2u32 => Ok(Self::Active),
                    3u32 => Ok(Self::Inactive),
                    4u32 => Ok(Self::Highlight),
                    5u32 => Ok(Self::Underline),
                    6u32 => Ok(Self::Selection),
                    7u32 => Ok(Self::Incorrect),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TextDirection {
            #[doc = "automatic text direction based on text and language"]
            Auto = 0u32,
            #[doc = "left-to-right"]
            Ltr = 1u32,
            #[doc = "right-to-left"]
            Rtl = 2u32,
        }
        impl TryFrom<u32> for TextDirection {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Auto),
                    1u32 => Ok(Self::Ltr),
                    2u32 => Ok(Self::Rtl),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwp_text_input_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zwp_text_input_v2";
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
                        tracing::debug!("zwp_text_input_v2#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.enable()", object.id);
                        self.enable(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.disable()", object.id);
                        self.disable(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.show_input_panel()", object.id);
                        self.show_input_panel(object, client).await
                    }
                    4u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.hide_input_panel()", object.id);
                        self.hide_input_panel(object, client).await
                    }
                    5u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.set_surrounding_text()", object.id);
                        self.set_surrounding_text(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    6u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.set_content_type()", object.id);
                        self.set_content_type(
                            object,
                            client,
                            message.uint()?.try_into()?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    7u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.set_cursor_rectangle()", object.id);
                        self.set_cursor_rectangle(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.set_preferred_language()", object.id);
                        self.set_preferred_language(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    9u16 => {
                        tracing::debug!("zwp_text_input_v2#{}.update_state()", object.id);
                        self.update_state(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_text_input object. Also disables all surfaces enabled"]
            #[doc = "through this wp_text_input object"]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Enable text input in a surface (usually when a text entry inside of it"]
            #[doc = "has focus)."]
            #[doc = ""]
            #[doc = "This can be called before or after a surface gets text (or keyboard)"]
            #[doc = "focus via the enter event. Text input to a surface is only active"]
            #[doc = "when it has the current text (or keyboard) focus and is enabled."]
            async fn enable(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Disable text input in a surface (typically when there is no focus on any"]
            #[doc = "text entry inside the surface)."]
            async fn disable(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Requests input panels (virtual keyboard) to show."]
            #[doc = ""]
            #[doc = "This should be used for example to show a virtual keyboard again"]
            #[doc = "(with a tap) after it was closed by pressing on a close button on the"]
            #[doc = "keyboard."]
            async fn show_input_panel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Requests input panels (virtual keyboard) to hide."]
            async fn hide_input_panel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the plain surrounding text around the input position. Text is"]
            #[doc = "UTF-8 encoded. Cursor is the byte offset within the surrounding text."]
            #[doc = "Anchor is the byte offset of the selection anchor within the"]
            #[doc = "surrounding text. If there is no selected text, anchor is the same as"]
            #[doc = "cursor."]
            #[doc = ""]
            #[doc = "Make sure to always send some text before and after the cursor"]
            #[doc = "except when the cursor is at the beginning or end of text."]
            #[doc = ""]
            #[doc = "When there was a configure_surrounding_text event take the"]
            #[doc = "before_cursor and after_cursor arguments into account for picking how"]
            #[doc = "much surrounding text to send."]
            #[doc = ""]
            #[doc = "There is a maximum length of wayland messages so text can not be"]
            #[doc = "longer than 4000 bytes."]
            async fn set_surrounding_text(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                text: String,
                cursor: i32,
                anchor: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some"]
            #[doc = "of the behavior."]
            #[doc = ""]
            #[doc = "When no content type is explicitly set, a normal content purpose with"]
            #[doc = "none hint should be assumed."]
            async fn set_content_type(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                hint: ContentHint,
                purpose: ContentPurpose,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the cursor outline as a x, y, width, height rectangle in surface"]
            #[doc = "local coordinates."]
            #[doc = ""]
            #[doc = "Allows the compositor to put a window with word suggestions near the"]
            #[doc = "cursor."]
            async fn set_cursor_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets a specific language. This allows for example a virtual keyboard to"]
            #[doc = "show a language specific layout. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            #[doc = ""]
            #[doc = "It could be used for example in a word processor to indicate language of"]
            #[doc = "currently edited document or in an instant message application which"]
            #[doc = "tracks languages of contacts."]
            async fn set_preferred_language(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                language: String,
            ) -> crate::server::Result<()>;
            #[doc = "Allows to atomically send state updates from client."]
            #[doc = ""]
            #[doc = "This request should follow after a batch of state updating requests"]
            #[doc = "like set_surrounding_text, set_content_type, set_cursor_rectangle and"]
            #[doc = "set_preferred_language."]
            #[doc = ""]
            #[doc = "The flags field indicates why an updated state is sent to the input"]
            #[doc = "method."]
            #[doc = ""]
            #[doc = "Reset should be used by an editor widget after the text was changed"]
            #[doc = "outside of the normal input method flow."]
            #[doc = ""]
            #[doc = "For \"change\" it is enough to send the changed state, else the full"]
            #[doc = "state should be send."]
            #[doc = ""]
            #[doc = "Serial should be set to the serial from the last enter or"]
            #[doc = "input_method_changed event."]
            #[doc = ""]
            #[doc = "To make sure to not receive outdated input method events after a"]
            #[doc = "reset or switching to a new widget wl_display_sync() should be used"]
            #[doc = "after update_state in these cases."]
            async fn update_state(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                reason: UpdateState,
            ) -> crate::server::Result<()>;
            #[doc = "Notification that this seat's text-input focus is on a certain surface."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            async fn enter(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.enter()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notification that this seat's text-input focus is no longer on"]
            #[doc = "a certain surface."]
            #[doc = ""]
            #[doc = "The leave notification is sent before the enter notification"]
            #[doc = "for the new focus."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            async fn leave(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.leave()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notification that the visibility of the input panel (virtual keyboard)"]
            #[doc = "changed."]
            #[doc = ""]
            #[doc = "The rectangle x, y, width, height defines the area overlapped by the"]
            #[doc = "input panel (virtual keyboard) on the surface having the text"]
            #[doc = "focus in surface local coordinates."]
            #[doc = ""]
            #[doc = "That can be used to make sure widgets are visible and not covered by"]
            #[doc = "a virtual keyboard."]
            async fn input_panel_state(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                state: InputPanelVisibility,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.input_panel_state()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(state as u32)
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when a new composing text (pre-edit) should be set around the"]
            #[doc = "current cursor position. Any previously set composing text should"]
            #[doc = "be removed."]
            #[doc = ""]
            #[doc = "The commit text can be used to replace the composing text in some cases"]
            #[doc = "(for example when losing focus)."]
            #[doc = ""]
            #[doc = "The text input should also handle all preedit_style and preedit_cursor"]
            #[doc = "events occurring directly before preedit_string."]
            async fn preedit_string(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                text: String,
                commit: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.preedit_string()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(text))
                    .put_string(Some(commit))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets styling information on composing text. The style is applied for"]
            #[doc = "length bytes from index relative to the beginning of the composing"]
            #[doc = "text (as byte offset). Multiple styles can be applied to a composing"]
            #[doc = "text by sending multiple preedit_styling events."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            async fn preedit_styling(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: u32,
                length: u32,
                style: PreeditStyle,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.preedit_styling()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(index)
                    .put_uint(length)
                    .put_uint(style as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets the cursor position inside the composing text (as byte"]
            #[doc = "offset) relative to the start of the composing text. When index is a"]
            #[doc = "negative number no cursor is shown."]
            #[doc = ""]
            #[doc = "When no preedit_cursor event is sent the cursor will be at the end of"]
            #[doc = "the composing text by default."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            async fn preedit_cursor(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.preedit_cursor()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(index).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit). It could be also an empty text"]
            #[doc = "when some text should be removed (see delete_surrounding_text) or when"]
            #[doc = "the input cursor should be moved (see cursor_position)."]
            #[doc = ""]
            #[doc = "Any previously set composing text should be removed."]
            async fn commit_string(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                text: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.commit_string()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(text))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when the cursor or anchor position should be modified."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The text between anchor and index should be selected."]
            async fn cursor_position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: i32,
                anchor: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.cursor_position()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(index)
                    .put_int(anchor)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted. BeforeLength and afterLength is the length (in bytes) of text"]
            #[doc = "before and after the current cursor position (excluding the selection)"]
            #[doc = "to delete."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "or preedit_string event."]
            async fn delete_surrounding_text(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                before_length: u32,
                after_length: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> zwp_text_input_v2#{}.delete_surrounding_text()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(before_length)
                    .put_uint(after_length)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Transfer an array of 0-terminated modifiers names. The position in"]
            #[doc = "the array is the index of the modifier as used in the modifiers"]
            #[doc = "bitmask in the keysym event."]
            async fn modifiers_map(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                map: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.modifiers_map()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_array(map).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when a key event was sent. Key events should not be used"]
            #[doc = "for normal text input operations, which should be done with"]
            #[doc = "commit_string, delete_surrounding_text, etc. The key event follows"]
            #[doc = "the wl_keyboard key event convention. Sym is a XKB keysym, state a"]
            #[doc = "wl_keyboard key_state. Modifiers are a mask for effective modifiers"]
            #[doc = "(where the modifier indices are set by the modifiers_map event)"]
            async fn keysym(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                time: u32,
                sym: u32,
                state: u32,
                modifiers: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.keysym()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(sym)
                    .put_uint(state)
                    .put_uint(modifiers)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets the language of the input text. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            async fn language(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                language: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.language()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(language))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 11u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets the text direction of input text."]
            #[doc = ""]
            #[doc = "It is mainly needed for showing input cursor on correct side of the"]
            #[doc = "editor when there is no input yet done and making sure neutral"]
            #[doc = "direction text is laid out properly."]
            async fn text_direction(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                direction: TextDirection,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.text_direction()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(direction as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 12u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Configure what amount of surrounding text is expected by the"]
            #[doc = "input method. The surrounding text will be sent in the"]
            #[doc = "set_surrounding_text request on the following state information updates."]
            async fn configure_surrounding_text(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                before_cursor: i32,
                after_cursor: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> zwp_text_input_v2#{}.configure_surrounding_text()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(before_cursor)
                    .put_int(after_cursor)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 13u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The input method changed on compositor side, which invalidates all"]
            #[doc = "current state information. New state information should be sent from"]
            #[doc = "the client via state requests (set_surrounding_text,"]
            #[doc = "set_content_hint, ...) and update_state."]
            async fn input_method_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                flags: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> zwp_text_input_v2#{}.input_method_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(flags)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 14u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    pub mod zwp_text_input_manager_v2 {
        #[doc = "Trait to implement the zwp_text_input_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zwp_text_input_manager_v2";
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
                        tracing::debug!("zwp_text_input_manager_v2#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("zwp_text_input_manager_v2#{}.get_text_input()", object.id);
                        self.get_text_input(
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
            #[doc = "Destroy the wp_text_input_manager object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Creates a new text-input object for a given seat."]
            async fn get_text_input(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod text {
    #[doc = "An object used for text input. Adds support for text input and input"]
    #[doc = "methods to applications. A text-input object is created from a"]
    #[doc = "wl_text_input_manager and corresponds typically to a text entry in an"]
    #[doc = "application."]
    #[doc = "Requests are used to activate/deactivate the text-input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about entered text is sent to the text-input object via"]
    #[doc = "the pre-edit and commit events. Using this interface removes the need"]
    #[doc = "for applications to directly process hardware key events and compose text"]
    #[doc = "out of them."]
    #[doc = ""]
    #[doc = "Text is generally UTF-8 encoded, indices and lengths are in bytes."]
    #[doc = ""]
    #[doc = "Serials are used to synchronize the state between the text input and"]
    #[doc = "an input method. New serials are sent by the text input in the"]
    #[doc = "commit_state request and are used by the input method to indicate"]
    #[doc = "the known text input state in events like preedit_string, commit_string,"]
    #[doc = "and keysym. The text input can then ignore events from the input method"]
    #[doc = "which are based on an outdated state (for example after a reset)."]
    pub mod wl_text_input {
        #[doc = "Content hint is a bitmask to allow to modify the behavior of the text"]
        #[doc = "input."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ContentHint {
            #[doc = "no special behaviour"]
            None = 0u32,
            #[doc = "auto completion, correction and capitalization"]
            Default = 7u32,
            #[doc = "hidden and sensitive text"]
            Password = 192u32,
            #[doc = "suggest word completions"]
            AutoCompletion = 1u32,
            #[doc = "suggest word corrections"]
            AutoCorrection = 2u32,
            #[doc = "switch to uppercase letters at the start of a sentence"]
            AutoCapitalization = 4u32,
            #[doc = "prefer lowercase letters"]
            Lowercase = 8u32,
            #[doc = "prefer uppercase letters"]
            Uppercase = 16u32,
            #[doc = "prefer casing for titles and headings (can be language dependent)"]
            Titlecase = 32u32,
            #[doc = "characters should be hidden"]
            HiddenText = 64u32,
            #[doc = "typed text should not be stored"]
            SensitiveData = 128u32,
            #[doc = "just latin characters should be entered"]
            Latin = 256u32,
            #[doc = "the text input is multiline"]
            Multiline = 512u32,
        }
        impl TryFrom<u32> for ContentHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    7u32 => Ok(Self::Default),
                    192u32 => Ok(Self::Password),
                    1u32 => Ok(Self::AutoCompletion),
                    2u32 => Ok(Self::AutoCorrection),
                    4u32 => Ok(Self::AutoCapitalization),
                    8u32 => Ok(Self::Lowercase),
                    16u32 => Ok(Self::Uppercase),
                    32u32 => Ok(Self::Titlecase),
                    64u32 => Ok(Self::HiddenText),
                    128u32 => Ok(Self::SensitiveData),
                    256u32 => Ok(Self::Latin),
                    512u32 => Ok(Self::Multiline),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
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
            #[doc = "input a password (combine with password or sensitive_data hint)"]
            Password = 8u32,
            #[doc = "input a date"]
            Date = 9u32,
            #[doc = "input a time"]
            Time = 10u32,
            #[doc = "input a date and time"]
            Datetime = 11u32,
            #[doc = "input for a terminal"]
            Terminal = 12u32,
        }
        impl TryFrom<u32> for ContentPurpose {
            type Error = crate::wire::DecodeError;
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
                    9u32 => Ok(Self::Date),
                    10u32 => Ok(Self::Time),
                    11u32 => Ok(Self::Datetime),
                    12u32 => Ok(Self::Terminal),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PreeditStyle {
            #[doc = "default style for composing text"]
            Default = 0u32,
            #[doc = "style should be the same as in non-composing text"]
            None = 1u32,
            Active = 2u32,
            Inactive = 3u32,
            Highlight = 4u32,
            Underline = 5u32,
            Selection = 6u32,
            Incorrect = 7u32,
        }
        impl TryFrom<u32> for PreeditStyle {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Default),
                    1u32 => Ok(Self::None),
                    2u32 => Ok(Self::Active),
                    3u32 => Ok(Self::Inactive),
                    4u32 => Ok(Self::Highlight),
                    5u32 => Ok(Self::Underline),
                    6u32 => Ok(Self::Selection),
                    7u32 => Ok(Self::Incorrect),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TextDirection {
            #[doc = "automatic text direction based on text and language"]
            Auto = 0u32,
            #[doc = "left-to-right"]
            Ltr = 1u32,
            #[doc = "right-to-left"]
            Rtl = 2u32,
        }
        impl TryFrom<u32> for TextDirection {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Auto),
                    1u32 => Ok(Self::Ltr),
                    2u32 => Ok(Self::Rtl),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_text_input interface. See the module level documentation for more info"]
        pub trait WlTextInput: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_text_input";
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
                        tracing::debug!("wl_text_input#{}.activate()", object.id);
                        self.activate(
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
                        tracing::debug!("wl_text_input#{}.deactivate()", object.id);
                        self.deactivate(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("wl_text_input#{}.show_input_panel()", object.id);
                        self.show_input_panel(object, client).await
                    }
                    3u16 => {
                        tracing::debug!("wl_text_input#{}.hide_input_panel()", object.id);
                        self.hide_input_panel(object, client).await
                    }
                    4u16 => {
                        tracing::debug!("wl_text_input#{}.reset()", object.id);
                        self.reset(object, client).await
                    }
                    5u16 => {
                        tracing::debug!("wl_text_input#{}.set_surrounding_text()", object.id);
                        self.set_surrounding_text(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    6u16 => {
                        tracing::debug!("wl_text_input#{}.set_content_type()", object.id);
                        self.set_content_type(object, client, message.uint()?, message.uint()?)
                            .await
                    }
                    7u16 => {
                        tracing::debug!("wl_text_input#{}.set_cursor_rectangle()", object.id);
                        self.set_cursor_rectangle(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!("wl_text_input#{}.set_preferred_language()", object.id);
                        self.set_preferred_language(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    9u16 => {
                        tracing::debug!("wl_text_input#{}.commit_state()", object.id);
                        self.commit_state(object, client, message.uint()?).await
                    }
                    10u16 => {
                        tracing::debug!("wl_text_input#{}.invoke_action()", object.id);
                        self.invoke_action(object, client, message.uint()?, message.uint()?)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Requests the text-input object to be activated (typically when the"]
            #[doc = "text entry gets focus)."]
            #[doc = "The seat argument is a wl_seat which maintains the focus for this"]
            #[doc = "activation. The surface argument is a wl_surface assigned to the"]
            #[doc = "text-input object and tracked for focus lost. The enter event"]
            #[doc = "is emitted on successful activation."]
            async fn activate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                seat: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Requests the text-input object to be deactivated (typically when the"]
            #[doc = "text entry lost focus). The seat argument is a wl_seat which was used"]
            #[doc = "for activation."]
            async fn deactivate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                seat: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Requests input panels (virtual keyboard) to show."]
            async fn show_input_panel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Requests input panels (virtual keyboard) to hide."]
            async fn hide_input_panel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Should be called by an editor widget when the input state should be"]
            #[doc = "reset, for example after the text was changed outside of the normal"]
            #[doc = "input method flow."]
            async fn reset(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the plain surrounding text around the input position. Text is"]
            #[doc = "UTF-8 encoded. Cursor is the byte offset within the"]
            #[doc = "surrounding text. Anchor is the byte offset of the"]
            #[doc = "selection anchor within the surrounding text. If there is no selected"]
            #[doc = "text anchor is the same as cursor."]
            async fn set_surrounding_text(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                text: String,
                cursor: u32,
                anchor: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some"]
            #[doc = "of the behavior."]
            #[doc = ""]
            #[doc = "When no content type is explicitly set, a normal content purpose with"]
            #[doc = "default hints (auto completion, auto correction, auto capitalization)"]
            #[doc = "should be assumed."]
            async fn set_content_type(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                hint: u32,
                purpose: u32,
            ) -> crate::server::Result<()>;
            async fn set_cursor_rectangle(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets a specific language. This allows for example a virtual keyboard to"]
            #[doc = "show a language specific layout. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            #[doc = ""]
            #[doc = "It could be used for example in a word processor to indicate language of"]
            #[doc = "currently edited document or in an instant message application which tracks"]
            #[doc = "languages of contacts."]
            async fn set_preferred_language(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                language: String,
            ) -> crate::server::Result<()>;
            async fn commit_state(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
            ) -> crate::server::Result<()>;
            async fn invoke_action(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                button: u32,
                index: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Notify the text-input object when it received focus. Typically in"]
            #[doc = "response to an activate request."]
            async fn enter(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.enter()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify the text-input object when it lost focus. Either in response"]
            #[doc = "to a deactivate request or when the assigned surface lost focus or was"]
            #[doc = "destroyed."]
            async fn leave(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.leave()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Transfer an array of 0-terminated modifiers names. The position in"]
            #[doc = "the array is the index of the modifier as used in the modifiers"]
            #[doc = "bitmask in the keysym event."]
            async fn modifiers_map(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                map: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.modifiers_map()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_array(map).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when the visibility state of the input panel changed."]
            async fn input_panel_state(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                state: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.input_panel_state()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(state).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when a new composing text (pre-edit) should be set around the"]
            #[doc = "current cursor position. Any previously set composing text should"]
            #[doc = "be removed."]
            #[doc = ""]
            #[doc = "The commit text can be used to replace the preedit text on reset"]
            #[doc = "(for example on unfocus)."]
            #[doc = ""]
            #[doc = "The text input should also handle all preedit_style and preedit_cursor"]
            #[doc = "events occurring directly before preedit_string."]
            async fn preedit_string(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                text: String,
                commit: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.preedit_string()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(Some(text))
                    .put_string(Some(commit))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets styling information on composing text. The style is applied for"]
            #[doc = "length bytes from index relative to the beginning of the composing"]
            #[doc = "text (as byte offset). Multiple styles can"]
            #[doc = "be applied to a composing text by sending multiple preedit_styling"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            async fn preedit_styling(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: u32,
                length: u32,
                style: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.preedit_styling()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(index)
                    .put_uint(length)
                    .put_uint(style)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets the cursor position inside the composing text (as byte"]
            #[doc = "offset) relative to the start of the composing text. When index is a"]
            #[doc = "negative number no cursor is shown."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            async fn preedit_cursor(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.preedit_cursor()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(index).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit). It could be also an empty text"]
            #[doc = "when some text should be removed (see delete_surrounding_text) or when"]
            #[doc = "the input cursor should be moved (see cursor_position)."]
            #[doc = ""]
            #[doc = "Any previously set composing text should be removed."]
            async fn commit_string(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                text: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.commit_string()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(Some(text))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when the cursor or anchor position should be modified."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            async fn cursor_position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: i32,
                anchor: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.cursor_position()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(index)
                    .put_int(anchor)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted."]
            #[doc = ""]
            #[doc = "Index is relative to the current cursor (in bytes)."]
            #[doc = "Length is the length of deleted text (in bytes)."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            async fn delete_surrounding_text(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                index: i32,
                length: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.delete_surrounding_text()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(index)
                    .put_uint(length)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify when a key event was sent. Key events should not be used"]
            #[doc = "for normal text input operations, which should be done with"]
            #[doc = "commit_string, delete_surrounding_text, etc. The key event follows"]
            #[doc = "the wl_keyboard key event convention. Sym is a XKB keysym, state a"]
            #[doc = "wl_keyboard key_state. Modifiers are a mask for effective modifiers"]
            #[doc = "(where the modifier indices are set by the modifiers_map event)"]
            async fn keysym(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                time: u32,
                sym: u32,
                state: u32,
                modifiers: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.keysym()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(sym)
                    .put_uint(state)
                    .put_uint(modifiers)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets the language of the input text. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            async fn language(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                language: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.language()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(Some(language))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 11u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sets the text direction of input text."]
            #[doc = ""]
            #[doc = "It is mainly needed for showing input cursor on correct side of the"]
            #[doc = "editor when there is no input yet done and making sure neutral"]
            #[doc = "direction text is laid out properly."]
            async fn text_direction(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial: u32,
                direction: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_text_input#{}.text_direction()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(direction)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 12u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    pub mod wl_text_input_manager {
        #[doc = "Trait to implement the wl_text_input_manager interface. See the module level documentation for more info"]
        pub trait WlTextInputManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_text_input_manager";
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
                        tracing::debug!("wl_text_input_manager#{}.create_text_input()", object.id);
                        self.create_text_input(
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
            #[doc = "Creates a new text-input object."]
            async fn create_text_input(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod wl_eglstream_controller {
    pub mod wl_eglstream_controller {
        #[doc = "- dont_care: Using this enum will tell the server to make its own"]
        #[doc = "decisions regarding present mode."]
        #[doc = ""]
        #[doc = "- fifo:      Tells the server to use a fifo present mode. The decision to"]
        #[doc = "use fifo synchronous is left up to the server."]
        #[doc = ""]
        #[doc = "- mailbox:   Tells the server to use a mailbox present mode."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PresentMode {
            #[doc = "Let the Server decide present mode"]
            DontCare = 0u32,
            #[doc = "Use a fifo present mode"]
            Fifo = 1u32,
            #[doc = "Use a mailbox mode"]
            Mailbox = 2u32,
        }
        impl TryFrom<u32> for PresentMode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::DontCare),
                    1u32 => Ok(Self::Fifo),
                    2u32 => Ok(Self::Mailbox),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "- present_mode: Must be one of wl_eglstream_controller_present_mode. Tells the"]
        #[doc = "server the desired present mode that should be used."]
        #[doc = ""]
        #[doc = "- fifo_length:  Only valid when the present_mode attrib is provided and its"]
        #[doc = "value is specified as fifo. Tells the server the desired fifo"]
        #[doc = "length to be used when the desired present_mode is fifo."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Attrib {
            #[doc = "Tells the server the desired present mode"]
            PresentMode = 0u32,
            #[doc = "Tells the server the desired fifo length when the desired presenation_mode is fifo."]
            FifoLength = 1u32,
        }
        impl TryFrom<u32> for Attrib {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PresentMode),
                    1u32 => Ok(Self::FifoLength),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_eglstream_controller interface. See the module level documentation for more info"]
        pub trait WlEglstreamController: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_eglstream_controller";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "wl_eglstream_controller#{}.attach_eglstream_consumer()",
                            object.id
                        );
                        self.attach_eglstream_consumer(
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
                            "wl_eglstream_controller#{}.attach_eglstream_consumer_attribs()",
                            object.id
                        );
                        self.attach_eglstream_consumer_attribs(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.array()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Creates the corresponding server side EGLStream from the given wl_buffer"]
            #[doc = "and attaches a consumer to it."]
            async fn attach_eglstream_consumer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                wl_surface: crate::wire::ObjectId,
                wl_resource: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Creates the corresponding server side EGLStream from the given wl_buffer"]
            #[doc = "and attaches a consumer to it using the given attributes."]
            async fn attach_eglstream_consumer_attribs(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                wl_surface: crate::wire::ObjectId,
                wl_resource: crate::wire::ObjectId,
                attribs: Vec<u8>,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod dpms {
    #[doc = "The Dpms manager allows to get a org_kde_kwin_dpms for a given wl_output."]
    #[doc = "The org_kde_kwin_dpms provides the currently used VESA Display Power Management"]
    #[doc = "Signaling state (see https://en.wikipedia.org/wiki/VESA_Display_Power_Management_Signaling )."]
    #[doc = "In addition it allows to request a state change. A compositor is not obliged to honor it"]
    #[doc = "and will normally automatically switch back to on state."]
    pub mod org_kde_kwin_dpms_manager {
        #[doc = "Trait to implement the org_kde_kwin_dpms_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinDpmsManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_dpms_manager";
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
                        tracing::debug!("org_kde_kwin_dpms_manager#{}.get()", object.id);
                        self.get(
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
            #[doc = "Factory request to get the org_kde_kwin_dpms for a given wl_output."]
            async fn get(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "This interface provides information about the VESA DPMS state for a wl_output."]
    #[doc = "It gets created through the request get on the org_kde_kwin_dpms_manager interface."]
    #[doc = ""]
    #[doc = "On creating the resource the server will push whether DPSM is supported for the output,"]
    #[doc = "the currently used DPMS state and notifies the client through the done event once all"]
    #[doc = "states are pushed. Whenever a state changes the set of changes is committed with the"]
    #[doc = "done event."]
    pub mod org_kde_kwin_dpms {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            On = 0u32,
            Standby = 1u32,
            Suspend = 2u32,
            Off = 3u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::On),
                    1u32 => Ok(Self::Standby),
                    2u32 => Ok(Self::Suspend),
                    3u32 => Ok(Self::Off),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_dpms interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinDpms: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_dpms";
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
                        tracing::debug!("org_kde_kwin_dpms#{}.set()", object.id);
                        self.set(object, client, message.uint()?).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_dpms#{}.release()", object.id);
                        self.release(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Requests that the compositor puts the wl_output into the passed mode. The compositor"]
            #[doc = "is not obliged to change the state. In addition the compositor might leave the mode"]
            #[doc = "whenever it seems suitable. E.g. the compositor might return to On state on user input."]
            #[doc = ""]
            #[doc = "The client should not assume that the mode changed after requesting a new mode."]
            #[doc = "Instead the client should listen for the mode event."]
            async fn set(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: u32,
            ) -> crate::server::Result<()>;
            async fn release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This event gets pushed on binding the resource and indicates whether the wl_output"]
            #[doc = "supports DPMS. There are operation modes of a Wayland server where DPMS might not"]
            #[doc = "make sense (e.g. nested compositors)."]
            async fn supported(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                supported: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_dpms#{}.supported()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(supported)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This mode gets pushed on binding the resource and provides the currently used"]
            #[doc = "DPMS mode. It also gets pushed if DPMS is not supported for the wl_output, in that"]
            #[doc = "case the value will be On."]
            #[doc = ""]
            #[doc = "The event is also pushed whenever the state changes."]
            async fn mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_dpms#{}.mode()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(mode).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event gets pushed on binding the resource once all other states are pushed."]
            #[doc = ""]
            #[doc = "In addition it gets pushed whenever a state changes to tell the client that all"]
            #[doc = "state changes have been pushed."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_kwin_dpms#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod fake_input {
    #[doc = "This interface allows other processes to provide fake input events."]
    #[doc = "Purpose is on the one hand side to provide testing facilities like XTest on X11."]
    #[doc = "But also to support use case like kdeconnect's mouse pad interface."]
    #[doc = ""]
    #[doc = "A compositor should not trust the input received from this interface."]
    #[doc = "Clients should not expect that the compositor honors the requests from this"]
    #[doc = "interface."]
    pub mod org_kde_kwin_fake_input {
        #[doc = "Trait to implement the org_kde_kwin_fake_input interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinFakeInput: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_kwin_fake_input";
            const VERSION: u32 = 5u32;
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
                        tracing::debug!("org_kde_kwin_fake_input#{}.authenticate()", object.id);
                        self.authenticate(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.pointer_motion()", object.id);
                        self.pointer_motion(object, client, message.fixed()?, message.fixed()?)
                            .await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.button()", object.id);
                        self.button(object, client, message.uint()?, message.uint()?)
                            .await
                    }
                    3u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.axis()", object.id);
                        self.axis(object, client, message.uint()?, message.fixed()?)
                            .await
                    }
                    4u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.touch_down()", object.id);
                        self.touch_down(
                            object,
                            client,
                            message.uint()?,
                            message.fixed()?,
                            message.fixed()?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.touch_motion()", object.id);
                        self.touch_motion(
                            object,
                            client,
                            message.uint()?,
                            message.fixed()?,
                            message.fixed()?,
                        )
                        .await
                    }
                    6u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.touch_up()", object.id);
                        self.touch_up(object, client, message.uint()?).await
                    }
                    7u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.touch_cancel()", object.id);
                        self.touch_cancel(object, client).await
                    }
                    8u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.touch_frame()", object.id);
                        self.touch_frame(object, client).await
                    }
                    9u16 => {
                        tracing::debug!(
                            "org_kde_kwin_fake_input#{}.pointer_motion_absolute()",
                            object.id
                        );
                        self.pointer_motion_absolute(
                            object,
                            client,
                            message.fixed()?,
                            message.fixed()?,
                        )
                        .await
                    }
                    10u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.keyboard_key()", object.id);
                        self.keyboard_key(object, client, message.uint()?, message.uint()?)
                            .await
                    }
                    11u16 => {
                        tracing::debug!("org_kde_kwin_fake_input#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "A client should use this request to tell the compositor why it wants to"]
            #[doc = "use this interface. The compositor might use the information to decide"]
            #[doc = "whether it wants to grant the request. The data might also be passed to"]
            #[doc = "the user to decide whether the application should get granted access to"]
            #[doc = "this very privileged interface."]
            async fn authenticate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                application: String,
                reason: String,
            ) -> crate::server::Result<()>;
            async fn pointer_motion(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                delta_x: crate::wire::Fixed,
                delta_y: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn button(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                button: u32,
                state: u32,
            ) -> crate::server::Result<()>;
            async fn axis(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                axis: u32,
                value: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "A client should use this request to send touch down event at specific"]
            #[doc = "coordinates."]
            async fn touch_down(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: u32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "A client should use this request to send touch motion to specific position."]
            async fn touch_motion(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: u32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "A client should use this request to send touch up event."]
            async fn touch_up(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: u32,
            ) -> crate::server::Result<()>;
            #[doc = "A client should use this request to cancel the current"]
            #[doc = "touch event."]
            async fn touch_cancel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "A client should use this request to send touch frame event."]
            async fn touch_frame(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn pointer_motion_absolute(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn keyboard_key(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                button: u32,
                state: u32,
            ) -> crate::server::Result<()>;
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod kde_lockscreen_overlay_v1 {
    #[doc = "Allows a client to request a surface to be visible when the system is locked."]
    #[doc = ""]
    #[doc = "This is meant to be used for specific high urgency cases like phone calls or alarms."]
    pub mod kde_lockscreen_overlay_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the client provided an invalid surface state"]
            InvalidSurfaceState = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSurfaceState),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the kde_lockscreen_overlay_v1 interface. See the module level documentation for more info"]
        pub trait KdeLockscreenOverlayV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_lockscreen_overlay_v1";
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
                        tracing::debug!("kde_lockscreen_overlay_v1#{}.allow()", object.id);
                        self.allow(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("kde_lockscreen_overlay_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the compositor that the surface could be shown when the screen is locked. This request should be called while the surface is unmapped."]
            async fn allow(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This won't affect the surface previously marked with the allow request."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod kde_output_device_v2 {
    #[doc = "An output device describes a display device available to the compositor."]
    #[doc = "output_device is similar to wl_output, but focuses on output"]
    #[doc = "configuration management."]
    #[doc = ""]
    #[doc = "A client can query all global output_device objects to enlist all"]
    #[doc = "available display devices, even those that may currently not be"]
    #[doc = "represented by the compositor as a wl_output."]
    #[doc = ""]
    #[doc = "The client sends configuration changes to the server through the"]
    #[doc = "outputconfiguration interface, and the server applies the configuration"]
    #[doc = "changes to the hardware and signals changes to the output devices"]
    #[doc = "accordingly."]
    #[doc = ""]
    #[doc = "This object is published as global during start up for every available"]
    #[doc = "display devices, or when one later becomes available, for example by"]
    #[doc = "being hotplugged via a physical connector."]
    pub mod kde_output_device_v2 {
        #[doc = "This enumeration describes how the physical pixels on an output are"]
        #[doc = "laid out."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Subpixel {
            Unknown = 0u32,
            None = 1u32,
            HorizontalRgb = 2u32,
            HorizontalBgr = 3u32,
            VerticalRgb = 4u32,
            VerticalBgr = 5u32,
        }
        impl TryFrom<u32> for Subpixel {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unknown),
                    1u32 => Ok(Self::None),
                    2u32 => Ok(Self::HorizontalRgb),
                    3u32 => Ok(Self::HorizontalBgr),
                    4u32 => Ok(Self::VerticalRgb),
                    5u32 => Ok(Self::VerticalBgr),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "This describes the transform, that a compositor will apply to a"]
        #[doc = "surface to compensate for the rotation or mirroring of an"]
        #[doc = "output device."]
        #[doc = ""]
        #[doc = "The flipped values correspond to an initial flip around a"]
        #[doc = "vertical axis followed by rotation."]
        #[doc = ""]
        #[doc = "The purpose is mainly to allow clients to render accordingly and"]
        #[doc = "tell the compositor, so that for fullscreen surfaces, the"]
        #[doc = "compositor is still able to scan out directly client surfaces."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Transform {
            Normal = 0u32,
            _90 = 1u32,
            _180 = 2u32,
            _270 = 3u32,
            Flipped = 4u32,
            Flipped90 = 5u32,
            Flipped180 = 6u32,
            Flipped270 = 7u32,
        }
        impl TryFrom<u32> for Transform {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::_90),
                    2u32 => Ok(Self::_180),
                    3u32 => Ok(Self::_270),
                    4u32 => Ok(Self::Flipped),
                    5u32 => Ok(Self::Flipped90),
                    6u32 => Ok(Self::Flipped180),
                    7u32 => Ok(Self::Flipped270),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        bitflags::bitflags! { # [doc = "Describes what capabilities this device has."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "if this output_device can use overscan"] const Overscan = 1u32 ; # [doc = "if this outputdevice supports variable refresh rate"] const Vrr = 2u32 ; # [doc = "if setting the rgb range is possible"] const RgbRange = 4u32 ; # [doc = "if this outputdevice supports high dynamic range"] const HighDynamicRange = 8u32 ; # [doc = "if this outputdevice supports a wide color gamut"] const WideColorGamut = 16u32 ; # [doc = "if this outputdevice supports autorotation"] const AutoRotate = 32u32 ; # [doc = "if this outputdevice supports icc profiles"] const IccProfile = 64u32 ; } }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Whether full or limited color range should be used"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum RgbRange {
            Automatic = 0u32,
            Full = 1u32,
            Limited = 2u32,
        }
        impl TryFrom<u32> for RgbRange {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Automatic),
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Limited),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AutoRotatePolicy {
            Never = 0u32,
            InTabletMode = 1u32,
            Always = 2u32,
        }
        impl TryFrom<u32> for AutoRotatePolicy {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::InTabletMode),
                    2u32 => Ok(Self::Always),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ColorProfileSource {
            SRgb = 0u32,
            Icc = 1u32,
            Edid = 2u32,
        }
        impl TryFrom<u32> for ColorProfileSource {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SRgb),
                    1u32 => Ok(Self::Icc),
                    2u32 => Ok(Self::Edid),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the kde_output_device_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputDeviceV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_output_device_v2";
            const VERSION: u32 = 8u32;
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
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "The geometry event describes geometric properties of the output."]
            #[doc = "The event is sent when binding to the output object and whenever"]
            #[doc = "any of the properties change."]
            async fn geometry(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                physical_width: i32,
                physical_height: i32,
                subpixel: i32,
                make: String,
                model: String,
                transform: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.geometry()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(physical_width)
                    .put_int(physical_height)
                    .put_int(subpixel)
                    .put_string(Some(make))
                    .put_string(Some(model))
                    .put_int(transform)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event describes the mode currently in use for this head. It is only"]
            #[doc = "sent if the output is enabled."]
            async fn current_mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.current_mode()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(mode))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The mode event describes an available mode for the output."]
            #[doc = ""]
            #[doc = "When the client binds to the output_device object, the server sends this"]
            #[doc = "event once for every available mode the output_device can be operated by."]
            #[doc = ""]
            #[doc = "There will always be at least one event sent out on initial binding,"]
            #[doc = "which represents the current mode."]
            #[doc = ""]
            #[doc = "Later if an output changes, its mode event is sent again for the"]
            #[doc = "eventual added modes and lastly the current mode. In other words, the"]
            #[doc = "current mode is always represented by the latest event sent with the current"]
            #[doc = "flag set."]
            #[doc = ""]
            #[doc = "The size of a mode is given in physical hardware units of the output device."]
            #[doc = "This is not necessarily the same as the output size in the global compositor"]
            #[doc = "space. For instance, the output may be scaled, as described in"]
            #[doc = "kde_output_device_v2.scale, or transformed, as described in"]
            #[doc = "kde_output_device_v2.transform."]
            async fn mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                mode: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.mode()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(mode))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all other properties have been"]
            #[doc = "sent on binding to the output object as well as after any"]
            #[doc = "other output property change have been applied later on."]
            #[doc = "This allows to see changes to the output properties as atomic,"]
            #[doc = "even if multiple events successively announce them."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. If it is not sent, the client should assume a"]
            #[doc = "scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients track the"]
            #[doc = "current output of a surface, and if it is on a scaled"]
            #[doc = "output it should use wl_surface.set_buffer_scale with"]
            #[doc = "the scale of the output. That way the compositor can"]
            #[doc = "avoid scaling the surface, and the client can supply"]
            #[doc = "a higher detail image."]
            async fn scale(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                factor: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.scale()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fixed(factor).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The edid event encapsulates the EDID data for the outputdevice."]
            #[doc = ""]
            #[doc = "The event is sent when binding to the output object. The EDID"]
            #[doc = "data may be empty, in which case this event is sent anyway."]
            #[doc = "If the EDID information is empty, you can fall back to the name"]
            #[doc = "et al. properties of the outputdevice."]
            async fn edid(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                raw: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.edid()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(raw))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The enabled event notifies whether this output is currently"]
            #[doc = "enabled and used for displaying content by the server."]
            #[doc = "The event is sent when binding to the output object and"]
            #[doc = "whenever later on an output changes its state by becoming"]
            #[doc = "enabled or disabled."]
            async fn enabled(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                enabled: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.enabled()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(enabled).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The uuid can be used to identify the output. It's controlled by"]
            #[doc = "the server entirely. The server should make sure the uuid is"]
            #[doc = "persistent across restarts. An empty uuid is considered invalid."]
            async fn uuid(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                uuid: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.uuid()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(uuid))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Serial ID of the monitor, sent on startup before the first done event."]
            async fn serial_number(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                serial_number: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.serial_number()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(serial_number))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "EISA ID of the monitor, sent on startup before the first done event."]
            async fn eisa_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                eisa_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.eisa_id()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(eisa_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "What capabilities this device has, sent on startup before the first"]
            #[doc = "done event."]
            async fn capabilities(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flags: Capability,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.capabilities()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(flags.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Overscan value of the monitor in percent, sent on startup before the"]
            #[doc = "first done event."]
            async fn overscan(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                overscan: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.overscan()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(overscan)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 11u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "What policy the compositor will employ regarding its use of variable"]
            #[doc = "refresh rate."]
            async fn vrr_policy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                vrr_policy: VrrPolicy,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.vrr_policy()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(vrr_policy as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 12u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "What rgb range the compositor is using for this output"]
            async fn rgb_range(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                rgb_range: RgbRange,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.rgb_range()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(rgb_range as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 13u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Name of the output, it's useful to cross-reference to an zxdg_output_v1 and ultimately QScreen"]
            async fn name(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.name()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 14u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Whether or not high dynamic range is enabled for this output"]
            async fn high_dynamic_range(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                hdr_enabled: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.high_dynamic_range()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(hdr_enabled)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 15u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "If high dynamic range is used, this value defines the brightness in nits for content"]
            #[doc = "that's in standard dynamic range format. Note that while the value is in nits, that"]
            #[doc = "doesn't necessarily translate to the same brightness on the screen."]
            async fn sdr_brightness(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                sdr_brightness: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.sdr_brightness()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(sdr_brightness)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 16u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Whether or not the use of a wide color gamut is enabled for this output"]
            async fn wide_color_gamut(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                wcg_enabled: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.wide_color_gamut()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(wcg_enabled)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 17u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn auto_rotate_policy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                policy: AutoRotatePolicy,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.auto_rotate_policy()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(policy as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 18u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn icc_profile_path(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                profile_path: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.icc_profile_path()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(profile_path))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 19u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn brightness_metadata(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                max_peak_brightness: u32,
                max_frame_average_brightness: u32,
                min_brightness: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> kde_output_device_v2#{}.brightness_metadata()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(max_peak_brightness)
                    .put_uint(max_frame_average_brightness)
                    .put_uint(min_brightness)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 20u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn brightness_overrides(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                max_peak_brightness: i32,
                max_average_brightness: i32,
                min_brightness: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> kde_output_device_v2#{}.brightness_overrides()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(max_peak_brightness)
                    .put_int(max_average_brightness)
                    .put_int(min_brightness)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 21u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This can be used to provide the colors users assume sRGB applications should have based on the"]
            #[doc = "default experience on many modern sRGB screens."]
            async fn sdr_gamut_wideness(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                gamut_wideness: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.sdr_gamut_wideness()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(gamut_wideness)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 22u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn color_profile_source(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                source: ColorProfileSource,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> kde_output_device_v2#{}.color_profile_source()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(source as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 23u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This is the brightness modifier of the output. It doesn't specify"]
            #[doc = "any absolute values, but is merely a multiplier on top of other"]
            #[doc = "brightness values, like sdr_brightness and brightness_metadata."]
            #[doc = "0 is the minimum brightness (not completely dark) and 10000 is"]
            #[doc = "the maximum brightness."]
            #[doc = "This is currently only supported / meaningful while HDR is active."]
            async fn brightness(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                brightness: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_v2#{}.brightness()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(brightness)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 24u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "This object describes an output mode."]
    #[doc = ""]
    #[doc = "Some heads don't support output modes, in which case modes won't be"]
    #[doc = "advertised."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the"]
    #[doc = "kde_output_device.done event. No guarantees are made regarding the order"]
    #[doc = "in which properties are sent."]
    pub mod kde_output_device_mode_v2 {
        #[doc = "Trait to implement the kde_output_device_mode_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputDeviceModeV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_output_device_mode_v2";
            const VERSION: u32 = 1u32;
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
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This event describes the mode size. The size is given in physical"]
            #[doc = "hardware units of the output device. This is not necessarily the same as"]
            #[doc = "the output size in the global compositor space. For instance, the output"]
            #[doc = "may be scaled or transformed."]
            async fn size(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_mode_v2#{}.size()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event describes the mode's fixed vertical refresh rate. It is only"]
            #[doc = "sent if the mode has a fixed refresh rate."]
            async fn refresh(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                refresh: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_mode_v2#{}.refresh()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(refresh).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event advertises this mode as preferred."]
            async fn preferred(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_mode_v2#{}.preferred()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The compositor will destroy the object immediately after sending this"]
            #[doc = "event, so it will become invalid and the client should release any"]
            #[doc = "resources associated with it."]
            async fn removed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_device_mode_v2#{}.removed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod kde_output_management_v2 {
    #[doc = "This interface enables clients to set properties of output devices for screen"]
    #[doc = "configuration purposes via the server. To this end output devices are referenced"]
    #[doc = "by global kde_output_device_v2 objects."]
    #[doc = ""]
    #[doc = "outputmanagement (wl_global)"]
    #[doc = "--------------------------"]
    #[doc = "request:"]
    #[doc = "* create_configuration -> outputconfiguration (wl_resource)"]
    #[doc = ""]
    #[doc = "outputconfiguration (wl_resource)"]
    #[doc = "--------------------------"]
    #[doc = "requests:"]
    #[doc = "* enable(outputdevice, bool)"]
    #[doc = "* mode(outputdevice, mode)"]
    #[doc = "* transformation(outputdevice, flag)"]
    #[doc = "* position(outputdevice, x, y)"]
    #[doc = "* apply"]
    #[doc = ""]
    #[doc = "events:"]
    #[doc = "* applied"]
    #[doc = "* failed"]
    #[doc = ""]
    #[doc = "The server registers one outputmanagement object as a global object. In order"]
    #[doc = "to configure outputs a client requests create_configuration, which provides a"]
    #[doc = "resource referencing an outputconfiguration for one-time configuration. That"]
    #[doc = "way the server knows which requests belong together and can group them by that."]
    #[doc = ""]
    #[doc = "On the outputconfiguration object the client calls for each output whether the"]
    #[doc = "output should be enabled, which mode should be set (by referencing the mode from"]
    #[doc = "the list of announced modes) and the output's global position. Once all outputs"]
    #[doc = "are configured that way, the client calls apply."]
    #[doc = "At that point and not earlier the server should try to apply the configuration."]
    #[doc = "If this succeeds the server emits the applied signal, otherwise the failed"]
    #[doc = "signal, such that the configuring client is noticed about the success of its"]
    #[doc = "configuration request."]
    #[doc = ""]
    #[doc = "Through this design the interface enables atomic output configuration changes if"]
    #[doc = "internally supported by the server."]
    pub mod kde_output_management_v2 {
        #[doc = "Trait to implement the kde_output_management_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputManagementV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_output_management_v2";
            const VERSION: u32 = 9u32;
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
                            "kde_output_management_v2#{}.create_configuration()",
                            object.id
                        );
                        self.create_configuration(
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
            #[doc = "Request an outputconfiguration object through which the client can configure"]
            #[doc = "output devices."]
            async fn create_configuration(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "outputconfiguration is a client-specific resource that can be used to ask"]
    #[doc = "the server to apply changes to available output devices."]
    #[doc = ""]
    #[doc = "The client receives a list of output devices from the registry. When it wants"]
    #[doc = "to apply new settings, it creates a configuration object from the"]
    #[doc = "outputmanagement global, writes changes through this object's enable, scale,"]
    #[doc = "transform and mode calls. It then asks the server to apply these settings in"]
    #[doc = "an atomic fashion, for example through Linux' DRM interface."]
    #[doc = ""]
    #[doc = "The server signals back whether the new settings have applied successfully"]
    #[doc = "or failed to apply. outputdevice objects are updated after the changes have been"]
    #[doc = "applied to the hardware and before the server side sends the applied event."]
    pub mod kde_output_configuration_v2 {
        #[doc = "These error can be emitted in response to kde_output_configuration_v2 requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the config is already applied"]
            AlreadyApplied = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyApplied),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Whether this output should use full or limited rgb."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum RgbRange {
            Automatic = 0u32,
            Full = 1u32,
            Limited = 2u32,
        }
        impl TryFrom<u32> for RgbRange {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Automatic),
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Limited),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AutoRotatePolicy {
            Never = 0u32,
            InTabletMode = 1u32,
            Always = 2u32,
        }
        impl TryFrom<u32> for AutoRotatePolicy {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::InTabletMode),
                    2u32 => Ok(Self::Always),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ColorProfileSource {
            SRgb = 0u32,
            Icc = 1u32,
            Edid = 2u32,
        }
        impl TryFrom<u32> for ColorProfileSource {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SRgb),
                    1u32 => Ok(Self::Icc),
                    2u32 => Ok(Self::Edid),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the kde_output_configuration_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputConfigurationV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_output_configuration_v2";
            const VERSION: u32 = 9u32;
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
                        tracing::debug!("kde_output_configuration_v2#{}.enable()", object.id);
                        self.enable(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.mode()", object.id);
                        self.mode(
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
                        tracing::debug!("kde_output_configuration_v2#{}.transform()", object.id);
                        self.transform(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.position()", object.id);
                        self.position(
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
                    4u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.scale()", object.id);
                        self.scale(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.fixed()?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.apply()", object.id);
                        self.apply(object, client).await
                    }
                    6u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    7u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.overscan()", object.id);
                        self.overscan(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_vrr_policy()",
                            object.id
                        );
                        self.set_vrr_policy(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    9u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_rgb_range()",
                            object.id
                        );
                        self.set_rgb_range(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    10u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_primary_output()",
                            object.id
                        );
                        self.set_primary_output(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    11u16 => {
                        tracing::debug!("kde_output_configuration_v2#{}.set_priority()", object.id);
                        self.set_priority(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    12u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_high_dynamic_range()",
                            object.id
                        );
                        self.set_high_dynamic_range(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    13u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_sdr_brightness()",
                            object.id
                        );
                        self.set_sdr_brightness(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    14u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_wide_color_gamut()",
                            object.id
                        );
                        self.set_wide_color_gamut(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    15u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_auto_rotate_policy()",
                            object.id
                        );
                        self.set_auto_rotate_policy(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    16u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_icc_profile_path()",
                            object.id
                        );
                        self.set_icc_profile_path(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    17u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_brightness_overrides()",
                            object.id
                        );
                        self.set_brightness_overrides(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    18u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_sdr_gamut_wideness()",
                            object.id
                        );
                        self.set_sdr_gamut_wideness(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    19u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_color_profile_source()",
                            object.id
                        );
                        self.set_color_profile_source(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    20u16 => {
                        tracing::debug!(
                            "kde_output_configuration_v2#{}.set_brightness()",
                            object.id
                        );
                        self.set_brightness(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Mark the output as enabled or disabled."]
            async fn enable(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                enable: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the mode for a given output."]
            async fn mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                mode: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the transformation for a given output."]
            async fn transform(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                transform: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the position for this output device. (x,y) describe the top-left corner"]
            #[doc = "of the output in global space, whereby the origin (0,0) of the global space"]
            #[doc = "has to be aligned with the top-left corner of the most left and in case this"]
            #[doc = "does not define a single one the top output."]
            #[doc = ""]
            #[doc = "There may be no gaps or overlaps between outputs, i.e. the outputs are"]
            #[doc = "stacked horizontally, vertically, or both on each other."]
            async fn position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the scaling factor for this output device."]
            async fn scale(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                scale: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            #[doc = "Asks the server to apply property changes requested through this outputconfiguration"]
            #[doc = "object to all outputs on the server side."]
            #[doc = ""]
            #[doc = "The output configuration can be applied only once. The already_applied protocol error"]
            #[doc = "will be posted if the apply request is called the second time."]
            async fn apply(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Set the overscan value of this output device with a value in percent."]
            async fn overscan(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                overscan: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Set what policy the compositor should employ regarding its use of"]
            #[doc = "variable refresh rate."]
            async fn set_vrr_policy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                policy: VrrPolicy,
            ) -> crate::server::Result<()>;
            #[doc = "Whether full or limited color range should be used"]
            async fn set_rgb_range(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                rgb_range: RgbRange,
            ) -> crate::server::Result<()>;
            async fn set_primary_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "The order of outputs can be used to assign desktop environment components to a specific screen,"]
            #[doc = "see kde_output_order_v1 for details. The priority is 1-based for outputs that will be enabled after"]
            #[doc = "this changeset is applied, all outputs that are disabled need to have the index set to zero."]
            async fn set_priority(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                priority: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets whether or not the output should be set to HDR mode."]
            async fn set_high_dynamic_range(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                enable_hdr: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the brightness of standard dynamic range content in nits. Only has an effect while the output is in HDR mode."]
            #[doc = "Note that while the value is in nits, that doesn't necessarily translate to the same brightness on the screen."]
            async fn set_sdr_brightness(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                sdr_brightness: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Whether or not the output should use a wide color gamut"]
            async fn set_wide_color_gamut(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                enable_wcg: u32,
            ) -> crate::server::Result<()>;
            async fn set_auto_rotate_policy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                policy: AutoRotatePolicy,
            ) -> crate::server::Result<()>;
            async fn set_icc_profile_path(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                profile_path: String,
            ) -> crate::server::Result<()>;
            async fn set_brightness_overrides(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                max_peak_brightness: i32,
                max_frame_average_brightness: i32,
                min_brightness: i32,
            ) -> crate::server::Result<()>;
            #[doc = "This can be used to provide the colors users assume sRGB applications should have based on the"]
            #[doc = "default experience on many modern sRGB screens."]
            async fn set_sdr_gamut_wideness(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                gamut_wideness: u32,
            ) -> crate::server::Result<()>;
            async fn set_color_profile_source(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                color_profile_source: ColorProfileSource,
            ) -> crate::server::Result<()>;
            #[doc = "Set the brightness modifier of the output. It doesn't specify"]
            #[doc = "any absolute values, but is merely a multiplier on top of other"]
            #[doc = "brightness values, like sdr_brightness and brightness_metadata."]
            #[doc = "0 is the minimum brightness (not completely dark) and 10000 is"]
            #[doc = "the maximum brightness."]
            #[doc = "This is currently only supported / meaningful while HDR is active."]
            async fn set_brightness(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                outputdevice: crate::wire::ObjectId,
                brightness: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sent after the server has successfully applied the changes."]
            #[doc = "."]
            async fn applied(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_configuration_v2#{}.applied()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent if the server rejects the changes or failed to apply them."]
            async fn failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_configuration_v2#{}.failed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod kde_output_order_v1 {
    #[doc = "Announce the order in which desktop environment components should be placed on outputs."]
    #[doc = "The compositor will send the list of outputs when the global is bound and whenever there is a change."]
    pub mod kde_output_order_v1 {
        #[doc = "Trait to implement the kde_output_order_v1 interface. See the module level documentation for more info"]
        pub trait KdeOutputOrderV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_output_order_v1";
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
                        tracing::debug!("kde_output_order_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Specifies the output identified by their wl_output.name."]
            async fn output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output_name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_order_v1#{}.output()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(output_name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Specifies that the output list is complete. On the next output event, a new list begins."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_output_order_v1#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod kde_primary_output_v1 {
    #[doc = "Protocol for telling which is the primary display among the selection of enabled outputs."]
    pub mod kde_primary_output_v1 {
        #[doc = "Trait to implement the kde_primary_output_v1 interface. See the module level documentation for more info"]
        pub trait KdePrimaryOutputV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_primary_output_v1";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!("kde_primary_output_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Specifies which output is the primary one identified by their uuid. See kde_output_device_v2 uuid event for more information about it."]
            async fn primary_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output_name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> kde_primary_output_v1#{}.primary_output()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(output_name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod kde_screen_edge_v1 {
    #[doc = "This interface allows clients to associate actions with screen edges. For"]
    #[doc = "example, showing a surface by moving the pointer to a screen edge."]
    #[doc = ""]
    #[doc = "Potential ways to trigger the screen edge are subject to compositor"]
    #[doc = "policies. As an example, the compositor may consider the screen edge to be"]
    #[doc = "triggered if the pointer hits its associated screen border. Other ways may"]
    #[doc = "include using touchscreen or touchpad gestures."]
    pub mod kde_screen_edge_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the specified border value is invalid"]
            InvalidBorder = 0u32,
            #[doc = "the surface has invalid role"]
            InvalidRole = 1u32,
            #[doc = "the surface already has a screen edge"]
            AlreadyConstructed = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidBorder),
                    1u32 => Ok(Self::InvalidRole),
                    2u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "These values describe possible screen borders."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Border {
            #[doc = "top screen edge"]
            Top = 1u32,
            #[doc = "bottom screen edge"]
            Bottom = 2u32,
            #[doc = "left screen edge"]
            Left = 3u32,
            #[doc = "right screen edge"]
            Right = 4u32,
        }
        impl TryFrom<u32> for Border {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Bottom),
                    3u32 => Ok(Self::Left),
                    4u32 => Ok(Self::Right),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the kde_screen_edge_manager_v1 interface. See the module level documentation for more info"]
        pub trait KdeScreenEdgeManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_screen_edge_manager_v1";
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
                        tracing::debug!("kde_screen_edge_manager_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "kde_screen_edge_manager_v1#{}.get_auto_hide_screen_edge()",
                            object.id
                        );
                        self.get_auto_hide_screen_edge(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the screen edge manager. This doesn't destroy objects created"]
            #[doc = "with this manager."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Create a new auto hide screen edge object associated with the specified"]
            #[doc = "surface and the border."]
            #[doc = ""]
            #[doc = "Creating a kde_auto_hide_screen_edge_v1 object does not change the"]
            #[doc = "visibility of the surface. The kde_auto_hide_screen_edge_v1.activate"]
            #[doc = "request must be issued in order to hide the surface."]
            #[doc = ""]
            #[doc = "The \"border\" argument must be a valid enum entry, otherwise the"]
            #[doc = "invalid_border protocol error is raised."]
            #[doc = ""]
            #[doc = "The invalid_role protocol error will be raised if the specified surface"]
            #[doc = "does not have layer_surface role."]
            async fn get_auto_hide_screen_edge(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                border: Border,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "The auto hide screen edge object allows to hide the surface and make it"]
    #[doc = "visible by triggering the screen edge. The screen edge is inactive and"]
    #[doc = "the surface is visible by default."]
    #[doc = ""]
    #[doc = "This interface can be used to implement user interface elements such as"]
    #[doc = "auto-hide panels or docks."]
    #[doc = ""]
    #[doc = "kde_auto_hide_screen_edge_v1.activate activates the screen edge and makes"]
    #[doc = "the surface hidden. The surface can be made visible by triggering the"]
    #[doc = "screen edge or calling kde_auto_hide_screen_edge_v1.deactivate."]
    #[doc = ""]
    #[doc = "If the screen edge has been triggered, it won't be re-activated again."]
    #[doc = "Another kde_auto_hide_screen_edge_v1.activate request must be made by the"]
    #[doc = "client to activate the screen edge."]
    pub mod kde_auto_hide_screen_edge_v1 {
        #[doc = "Trait to implement the kde_auto_hide_screen_edge_v1 interface. See the module level documentation for more info"]
        pub trait KdeAutoHideScreenEdgeV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "kde_auto_hide_screen_edge_v1";
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
                        tracing::debug!("kde_auto_hide_screen_edge_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("kde_auto_hide_screen_edge_v1#{}.deactivate()", object.id);
                        self.deactivate(object, client).await
                    }
                    2u16 => {
                        tracing::debug!("kde_auto_hide_screen_edge_v1#{}.activate()", object.id);
                        self.activate(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the auto hide screen edge object. If the screen edge is active,"]
            #[doc = "it will be deactivated and the surface will be made visible."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Deactivate the screen edge. The surface will be made visible."]
            async fn deactivate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Activate the screen edge. The surface will be hidden until the screen"]
            #[doc = "edge is triggered."]
            async fn activate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod org_kde_plasma_virtual_desktop {
    pub mod org_kde_plasma_virtual_desktop_management {
        #[doc = "Trait to implement the org_kde_plasma_virtual_desktop_management interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaVirtualDesktopManagement: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_virtual_desktop_management";
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
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "org_kde_plasma_virtual_desktop_management#{}.get_virtual_desktop()",
                            object.id
                        );
                        self.get_virtual_desktop(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing :: debug ! ("org_kde_plasma_virtual_desktop_management#{}.request_create_virtual_desktop()" , object . id);
                        self.request_create_virtual_desktop(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing :: debug ! ("org_kde_plasma_virtual_desktop_management#{}.request_remove_virtual_desktop()" , object . id);
                        self.request_remove_virtual_desktop(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Given the id of a particular virtual desktop, get the corresponding org_kde_plasma_virtual_desktop which represents only the desktop with that id."]
            async fn get_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                desktop_id: String,
            ) -> crate::server::Result<()>;
            #[doc = "Ask the server to create a new virtual desktop, and position it at a specified position. If the position is zero or less, it will be positioned at the beginning, if the position is the count or more, it will be positioned at the end."]
            async fn request_create_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                position: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Ask the server to get rid of a virtual desktop, the server may or may not acconsent to the request."]
            async fn request_remove_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                desktop_id: String,
            ) -> crate::server::Result<()>;
            async fn desktop_created(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                desktop_id: String,
                position: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop_management#{}.desktop_created()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(desktop_id))
                    .put_uint(position)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn desktop_removed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                desktop_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop_management#{}.desktop_removed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(desktop_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all other properties has been"]
            #[doc = "sent after binding to the desktop manager object and after any"]
            #[doc = "other property changes done after that. This allows"]
            #[doc = "changes to the org_kde_plasma_virtual_desktop_management properties to be seen as"]
            #[doc = "atomic, even if they happen via multiple events."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop_management#{}.done()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn rows(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                rows: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop_management#{}.rows()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(rows).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    pub mod org_kde_plasma_virtual_desktop {
        #[doc = "Trait to implement the org_kde_plasma_virtual_desktop interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaVirtualDesktop: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_virtual_desktop";
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
                            "org_kde_plasma_virtual_desktop#{}.request_activate()",
                            object.id
                        );
                        self.request_activate(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Request the server to set the status of this desktop to active: The server is free to consent or deny the request. This will be the new \"current\" virtual desktop of the system."]
            async fn request_activate(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The format of the id is decided by the compositor implementation. A desktop id univocally identifies a virtual desktop and must be guaranteed to never exist two desktops with the same id. The format of the string id is up to the server implementation."]
            async fn desktop_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                desktop_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop#{}.desktop_id()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(desktop_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn name(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_virtual_desktop#{}.name()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The desktop will be the new \"current\" desktop of the system. The server may support either one virtual desktop active at a time, or other combinations such as one virtual desktop active per screen."]
            #[doc = "Windows associated to this virtual desktop will be shown."]
            async fn activated(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop#{}.activated()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Windows that were associated only to this desktop will be hidden."]
            async fn deactivated(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_virtual_desktop#{}.deactivated()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all other properties has been"]
            #[doc = "sent after binding to the desktop object and after any"]
            #[doc = "other property changes done after that. This allows"]
            #[doc = "changes to the org_kde_plasma_virtual_desktop properties to be seen as"]
            #[doc = "atomic, even if they happen via multiple events."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_virtual_desktop#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This virtual desktop has just been removed by the server:"]
            #[doc = "All windows will lose the association to this desktop."]
            async fn removed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_virtual_desktop#{}.removed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod plasma_shell {
    #[doc = "This interface is used by KF5 powered Wayland shells to communicate with"]
    #[doc = "the compositor and can only be bound one time."]
    pub mod org_kde_plasma_shell {
        #[doc = "Trait to implement the org_kde_plasma_shell interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaShell: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_shell";
            const VERSION: u32 = 8u32;
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
                        tracing::debug!("org_kde_plasma_shell#{}.get_surface()", object.id);
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
            #[doc = "Create a shell surface for an existing surface."]
            #[doc = ""]
            #[doc = "Only one shell surface can be associated with a given"]
            #[doc = "surface."]
            async fn get_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide the shell user interface."]
    #[doc = ""]
    #[doc = "It provides requests to set surface roles, assign an output"]
    #[doc = "or set the position in output coordinates."]
    #[doc = ""]
    #[doc = "On the server side the object is automatically destroyed when"]
    #[doc = "the related wl_surface is destroyed.  On client side,"]
    #[doc = "org_kde_plasma_surface.destroy() must be called before"]
    #[doc = "destroying the wl_surface object."]
    pub mod org_kde_plasma_surface {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Role {
            Normal = 0u32,
            Desktop = 1u32,
            Panel = 2u32,
            Onscreendisplay = 3u32,
            Notification = 4u32,
            Tooltip = 5u32,
            Criticalnotification = 6u32,
            Appletpopup = 7u32,
        }
        impl TryFrom<u32> for Role {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::Desktop),
                    2u32 => Ok(Self::Panel),
                    3u32 => Ok(Self::Onscreendisplay),
                    4u32 => Ok(Self::Notification),
                    5u32 => Ok(Self::Tooltip),
                    6u32 => Ok(Self::Criticalnotification),
                    7u32 => Ok(Self::Appletpopup),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PanelBehavior {
            AlwaysVisible = 1u32,
            AutoHide = 2u32,
            WindowsCanCover = 3u32,
            WindowsGoBelow = 4u32,
        }
        impl TryFrom<u32> for PanelBehavior {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlwaysVisible),
                    2u32 => Ok(Self::AutoHide),
                    3u32 => Ok(Self::WindowsCanCover),
                    4u32 => Ok(Self::WindowsGoBelow),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Request panel_auto_hide performed on a surface which does not correspond to an auto-hide panel."]
            PanelNotAutoHide = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PanelNotAutoHide),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_plasma_surface interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_surface";
            const VERSION: u32 = 8u32;
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
                        tracing::debug!("org_kde_plasma_surface#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("org_kde_plasma_surface#{}.set_output()", object.id);
                        self.set_output(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("org_kde_plasma_surface#{}.set_position()", object.id);
                        self.set_position(object, client, message.int()?, message.int()?)
                            .await
                    }
                    3u16 => {
                        tracing::debug!("org_kde_plasma_surface#{}.set_role()", object.id);
                        self.set_role(object, client, message.uint()?).await
                    }
                    4u16 => {
                        tracing::debug!(
                            "org_kde_plasma_surface#{}.set_panel_behavior()",
                            object.id
                        );
                        self.set_panel_behavior(object, client, message.uint()?)
                            .await
                    }
                    5u16 => {
                        tracing::debug!("org_kde_plasma_surface#{}.set_skip_taskbar()", object.id);
                        self.set_skip_taskbar(object, client, message.uint()?).await
                    }
                    6u16 => {
                        tracing::debug!(
                            "org_kde_plasma_surface#{}.panel_auto_hide_hide()",
                            object.id
                        );
                        self.panel_auto_hide_hide(object, client).await
                    }
                    7u16 => {
                        tracing::debug!(
                            "org_kde_plasma_surface#{}.panel_auto_hide_show()",
                            object.id
                        );
                        self.panel_auto_hide_show(object, client).await
                    }
                    8u16 => {
                        tracing::debug!(
                            "org_kde_plasma_surface#{}.set_panel_takes_focus()",
                            object.id
                        );
                        self.set_panel_takes_focus(object, client, message.uint()?)
                            .await
                    }
                    9u16 => {
                        tracing::debug!("org_kde_plasma_surface#{}.set_skip_switcher()", object.id);
                        self.set_skip_switcher(object, client, message.uint()?)
                            .await
                    }
                    10u16 => {
                        tracing::debug!("org_kde_plasma_surface#{}.open_under_cursor()", object.id);
                        self.open_under_cursor(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "The org_kde_plasma_surface interface is removed from the"]
            #[doc = "wl_surface object that was turned into a shell surface with the"]
            #[doc = "org_kde_plasma_shell.get_surface request."]
            #[doc = "The shell surface role is lost and wl_surface is unmapped."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Assign an output to this shell surface."]
            #[doc = "The compositor will use this information to set the position"]
            #[doc = "when org_kde_plasma_surface.set_position request is"]
            #[doc = "called."]
            async fn set_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Move the surface to new coordinates."]
            #[doc = ""]
            #[doc = "Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output"]
            #[doc = "is 1970,50 in global coordinates space."]
            #[doc = ""]
            #[doc = "Use org_kde_plasma_surface.set_output to assign an output"]
            #[doc = "to this surface."]
            async fn set_position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Assign a role to a shell surface."]
            #[doc = ""]
            #[doc = "The compositor handles surfaces depending on their role."]
            #[doc = "See the explanation below."]
            #[doc = ""]
            #[doc = "This request fails if the surface already has a role, this means"]
            #[doc = "the surface role may be assigned only once."]
            #[doc = ""]
            #[doc = "== Surfaces with splash role =="]
            #[doc = ""]
            #[doc = "Splash surfaces are placed above every other surface during the"]
            #[doc = "shell startup phase."]
            #[doc = ""]
            #[doc = "The surfaces are placed according to the output coordinates."]
            #[doc = "No size is imposed to those surfaces, the shell has to resize"]
            #[doc = "them according to output size."]
            #[doc = ""]
            #[doc = "These surfaces are meant to hide the desktop during the startup"]
            #[doc = "phase so that the user will always see a ready to work desktop."]
            #[doc = ""]
            #[doc = "A shell might not create splash surfaces if the compositor reveals"]
            #[doc = "the desktop in an alternative fashion, for example with a fade"]
            #[doc = "in effect."]
            #[doc = ""]
            #[doc = "That depends on how much time the desktop usually need to prepare"]
            #[doc = "the workspace or specific design decisions."]
            #[doc = "This specification doesn't impose any particular design."]
            #[doc = ""]
            #[doc = "When the startup phase is finished, the shell will send the"]
            #[doc = "org_kde_plasma.desktop_ready request to the compositor."]
            #[doc = ""]
            #[doc = "== Surfaces with desktop role =="]
            #[doc = ""]
            #[doc = "Desktop surfaces are placed below all other surfaces and are used"]
            #[doc = "to show the actual desktop view with icons, search results or"]
            #[doc = "controls the user will interact with. What to show depends on the"]
            #[doc = "shell implementation."]
            #[doc = ""]
            #[doc = "The surfaces are placed according to the output coordinates."]
            #[doc = "No size is imposed to those surfaces, the shell has to resize"]
            #[doc = "them according to output size."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the desktop role."]
            #[doc = ""]
            #[doc = "== Surfaces with dashboard role =="]
            #[doc = ""]
            #[doc = "Dashboard surfaces are placed above desktop surfaces and are used to"]
            #[doc = "show additional widgets and controls."]
            #[doc = ""]
            #[doc = "The surfaces are placed according to the output coordinates."]
            #[doc = "No size is imposed to those surfaces, the shell has to resize"]
            #[doc = "them according to output size."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the dashboard role."]
            #[doc = ""]
            #[doc = "== Surfaces with config role =="]
            #[doc = ""]
            #[doc = "A configuration surface is shown when the user wants to configure"]
            #[doc = "panel or desktop views."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the config role."]
            #[doc = ""]
            #[doc = "TODO: This should grab the input like popup menus, right?"]
            #[doc = ""]
            #[doc = "== Surfaces with overlay role =="]
            #[doc = ""]
            #[doc = "Overlays are special surfaces that shows for a limited amount"]
            #[doc = "of time.  Such surfaces are useful to display things like volume,"]
            #[doc = "brightness and status changes."]
            #[doc = ""]
            #[doc = "Compositors may decide to show those surfaces in a layer above"]
            #[doc = "all surfaces, even full screen ones if so is desired."]
            #[doc = ""]
            #[doc = "== Surfaces with notification role =="]
            #[doc = ""]
            #[doc = "Notification surfaces display informative content for a limited"]
            #[doc = "amount of time.  The compositor may decide to show them in a corner"]
            #[doc = "depending on the configuration."]
            #[doc = ""]
            #[doc = "These surfaces are shown in a layer above all other surfaces except"]
            #[doc = "for full screen ones."]
            #[doc = ""]
            #[doc = "== Surfaces with lock role =="]
            #[doc = ""]
            #[doc = "The lock surface is shown by the compositor when the session is"]
            #[doc = "locked, users interact with it to unlock the session."]
            #[doc = ""]
            #[doc = "Compositors should move lock surfaces to 0,0 in output"]
            #[doc = "coordinates space and hide all other surfaces for security sake."]
            #[doc = "For the same reason it is recommended that clients make the"]
            #[doc = "lock surface as big as the screen."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the lock role."]
            async fn set_role(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                role: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Set flags bitmask as described by the flag enum."]
            #[doc = "Pass 0 to unset any flag, the surface will adjust its behavior to"]
            #[doc = "the default."]
            #[doc = ""]
            #[doc = "Deprecated in Plasma 6. Setting this flag will have no effect. Applications should use layer shell where appropriate."]
            async fn set_panel_behavior(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flag: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Setting this bit to the window, will make it say it prefers to not be listed in the taskbar. Taskbar implementations may or may not follow this hint."]
            async fn set_skip_taskbar(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                skip: u32,
            ) -> crate::server::Result<()>;
            #[doc = "A panel surface with panel_behavior auto_hide can perform this request to hide the panel"]
            #[doc = "on a screen edge without unmapping it. The compositor informs the client about the panel"]
            #[doc = "being hidden with the event auto_hidden_panel_hidden."]
            #[doc = ""]
            #[doc = "The compositor will restore the visibility state of the"]
            #[doc = "surface when the pointer touches the screen edge the panel borders. Once the compositor restores"]
            #[doc = "the visibility the event auto_hidden_panel_shown will be sent. This event will also be sent"]
            #[doc = "if the compositor is unable to hide the panel."]
            #[doc = ""]
            #[doc = "The client can also request to show the panel again with the request panel_auto_hide_show."]
            async fn panel_auto_hide_hide(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "A panel surface with panel_behavior auto_hide can perform this request to show the panel"]
            #[doc = "again which got hidden with panel_auto_hide_hide."]
            async fn panel_auto_hide_show(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "By default various org_kde_plasma_surface roles do not take focus and cannot be"]
            #[doc = "activated. With this request the compositor can be instructed to pass focus also to this"]
            #[doc = "org_kde_plasma_surface."]
            async fn set_panel_takes_focus(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                takes_focus: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a switcher."]
            async fn set_skip_switcher(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                skip: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Request the initial position of this surface to be under the current"]
            #[doc = "cursor position. Has to be called before attaching any buffer to this surface."]
            async fn open_under_cursor(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "An auto-hiding panel got hidden by the compositor."]
            async fn auto_hidden_panel_hidden(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_surface#{}.auto_hidden_panel_hidden()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "An auto-hiding panel got shown by the compositor."]
            async fn auto_hidden_panel_shown(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_surface#{}.auto_hidden_panel_shown()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod plasma_window_management {
    #[doc = "This interface manages application windows."]
    #[doc = "It provides requests to show and hide the desktop and emits"]
    #[doc = "an event every time a window is created so that the client can"]
    #[doc = "use it to manage the window."]
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    pub mod org_kde_plasma_window_management {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            Active = 1u32,
            Minimized = 2u32,
            Maximized = 4u32,
            Fullscreen = 8u32,
            KeepAbove = 16u32,
            KeepBelow = 32u32,
            OnAllDesktops = 64u32,
            DemandsAttention = 128u32,
            Closeable = 256u32,
            Minimizable = 512u32,
            Maximizable = 1024u32,
            Fullscreenable = 2048u32,
            Skiptaskbar = 4096u32,
            Shadeable = 8192u32,
            Shaded = 16384u32,
            Movable = 32768u32,
            Resizable = 65536u32,
            VirtualDesktopChangeable = 131072u32,
            Skipswitcher = 262144u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Active),
                    2u32 => Ok(Self::Minimized),
                    4u32 => Ok(Self::Maximized),
                    8u32 => Ok(Self::Fullscreen),
                    16u32 => Ok(Self::KeepAbove),
                    32u32 => Ok(Self::KeepBelow),
                    64u32 => Ok(Self::OnAllDesktops),
                    128u32 => Ok(Self::DemandsAttention),
                    256u32 => Ok(Self::Closeable),
                    512u32 => Ok(Self::Minimizable),
                    1024u32 => Ok(Self::Maximizable),
                    2048u32 => Ok(Self::Fullscreenable),
                    4096u32 => Ok(Self::Skiptaskbar),
                    8192u32 => Ok(Self::Shadeable),
                    16384u32 => Ok(Self::Shaded),
                    32768u32 => Ok(Self::Movable),
                    65536u32 => Ok(Self::Resizable),
                    131072u32 => Ok(Self::VirtualDesktopChangeable),
                    262144u32 => Ok(Self::Skipswitcher),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ShowDesktop {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl TryFrom<u32> for ShowDesktop {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the org_kde_plasma_window_management interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaWindowManagement: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_window_management";
            const VERSION: u32 = 17u32;
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
                            "org_kde_plasma_window_management#{}.show_desktop()",
                            object.id
                        );
                        self.show_desktop(object, client, message.uint()?).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window_management#{}.get_window()",
                            object.id
                        );
                        self.get_window(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window_management#{}.get_window_by_uuid()",
                            object.id
                        );
                        self.get_window_by_uuid(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window_management#{}.get_stacking_order()",
                            object.id
                        );
                        self.get_stacking_order(
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
            #[doc = "Tell the compositor to show/hide the desktop."]
            async fn show_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                state: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Deprecated: use get_window_by_uuid"]
            async fn get_window(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                internal_window_id: u32,
            ) -> crate::server::Result<()>;
            async fn get_window_by_uuid(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                internal_window_uuid: String,
            ) -> crate::server::Result<()>;
            async fn get_stacking_order(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                stacking_order: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event will be sent whenever the show desktop mode changes. E.g. when it is entered"]
            #[doc = "or left."]
            #[doc = ""]
            #[doc = "On binding the interface the current state is sent."]
            async fn show_desktop_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                state: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window_management#{}.show_desktop_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(state).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent immediately after a window is mapped."]
            async fn window(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window_management#{}.window()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(id).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when stacking order changed and on bind."]
            #[doc = ""]
            #[doc = "With version 17 this event is deprecated and will no longer be sent."]
            async fn stacking_order_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                ids: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window_management#{}.stacking_order_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_array(ids).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when stacking order changed and on bind."]
            #[doc = ""]
            #[doc = "With version 17 this event is deprecated and will no longer be sent."]
            async fn stacking_order_uuid_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                uuids: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window_management#{}.stacking_order_uuid_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(uuids))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent immediately after a window is mapped."]
            async fn window_with_uuid(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: u32,
                uuid: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window_management#{}.window_with_uuid()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(id)
                    .put_string(Some(uuid))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when stacking order changed."]
            async fn stacking_order_changed_2(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window_management#{}.stacking_order_changed_2()",
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
    #[doc = "Manages and control an application window."]
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    pub mod org_kde_plasma_window {
        #[doc = "Trait to implement the org_kde_plasma_window interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaWindow: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_window";
            const VERSION: u32 = 17u32;
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
                        tracing::debug!("org_kde_plasma_window#{}.set_state()", object.id);
                        self.set_state(object, client, message.uint()?, message.uint()?)
                            .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.set_virtual_desktop()",
                            object.id
                        );
                        self.set_virtual_desktop(object, client, message.uint()?)
                            .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.set_minimized_geometry()",
                            object.id
                        );
                        self.set_minimized_geometry(
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
                    3u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.unset_minimized_geometry()",
                            object.id
                        );
                        self.unset_minimized_geometry(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4u16 => {
                        tracing::debug!("org_kde_plasma_window#{}.close()", object.id);
                        self.close(object, client).await
                    }
                    5u16 => {
                        tracing::debug!("org_kde_plasma_window#{}.request_move()", object.id);
                        self.request_move(object, client).await
                    }
                    6u16 => {
                        tracing::debug!("org_kde_plasma_window#{}.request_resize()", object.id);
                        self.request_resize(object, client).await
                    }
                    7u16 => {
                        tracing::debug!("org_kde_plasma_window#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    8u16 => {
                        tracing::debug!("org_kde_plasma_window#{}.get_icon()", object.id);
                        self.get_icon(object, client, message.fd()?).await
                    }
                    9u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.request_enter_virtual_desktop()",
                            object.id
                        );
                        self.request_enter_virtual_desktop(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    10u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.request_enter_new_virtual_desktop()",
                            object.id
                        );
                        self.request_enter_new_virtual_desktop(object, client).await
                    }
                    11u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.request_leave_virtual_desktop()",
                            object.id
                        );
                        self.request_leave_virtual_desktop(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    12u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.request_enter_activity()",
                            object.id
                        );
                        self.request_enter_activity(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    13u16 => {
                        tracing::debug!(
                            "org_kde_plasma_window#{}.request_leave_activity()",
                            object.id
                        );
                        self.request_leave_activity(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    14u16 => {
                        tracing::debug!("org_kde_plasma_window#{}.send_to_output()", object.id);
                        self.send_to_output(
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
            #[doc = "Set window state."]
            #[doc = ""]
            #[doc = "Values for state argument are described by org_kde_plasma_window_management.state"]
            #[doc = "and can be used together in a bitfield. The flags bitfield describes which flags are"]
            #[doc = "supposed to be set, the state bitfield the value for the set flags"]
            async fn set_state(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flags: u32,
                state: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Deprecated: use enter_virtual_desktop"]
            #[doc = "Maps the window to a different virtual desktop."]
            #[doc = ""]
            #[doc = "To show the window on all virtual desktops, call the"]
            #[doc = "org_kde_plasma_window.set_state request and specify a on_all_desktops"]
            #[doc = "state in the bitfield."]
            async fn set_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                number: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the geometry of the taskbar entry for this window."]
            #[doc = "The geometry is relative to a panel in particular."]
            async fn set_minimized_geometry(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                panel: crate::wire::ObjectId,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Remove the task geometry information for a particular panel."]
            async fn unset_minimized_geometry(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                panel: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Close this window."]
            async fn close(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Request an interactive move for this window."]
            async fn request_move(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Request an interactive resize for this window."]
            async fn request_resize(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Removes the resource bound for this org_kde_plasma_window."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The compositor will write the window icon into the provided file descriptor."]
            #[doc = "The data is a serialized QIcon with QDataStream."]
            async fn get_icon(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()>;
            #[doc = "Make the window enter a virtual desktop. A window can enter more"]
            #[doc = "than one virtual desktop. if the id is empty or invalid, no action will be performed."]
            async fn request_enter_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()>;
            #[doc = "RFC: do this with an empty id to request_enter_virtual_desktop?"]
            #[doc = "Make the window enter a new virtual desktop. If the server consents the request,"]
            #[doc = "it will create a new virtual desktop and assign the window to it."]
            async fn request_enter_new_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Make the window exit a virtual desktop. If it exits all desktops it will be considered on all of them."]
            async fn request_leave_virtual_desktop(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()>;
            #[doc = "Make the window enter an activity. A window can enter more activity. If the id is empty or invalid, no action will be performed."]
            async fn request_enter_activity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()>;
            #[doc = "Make the window exit a an activity. If it exits all activities it will be considered on all of them."]
            async fn request_leave_activity(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()>;
            #[doc = "Requests this window to be displayed in a specific output."]
            async fn send_to_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event will be sent as soon as the window title is changed."]
            async fn title_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                title: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.title_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(title))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent as soon as the application"]
            #[doc = "identifier is changed."]
            async fn app_id_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                app_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.app_id_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent as soon as the window state changes."]
            #[doc = ""]
            #[doc = "Values for state argument are described by org_kde_plasma_window_management.state."]
            async fn state_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                flags: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.state_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(flags).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "DEPRECATED: use virtual_desktop_entered and virtual_desktop_left instead"]
            #[doc = "This event will be sent when a window is moved to another"]
            #[doc = "virtual desktop."]
            #[doc = ""]
            #[doc = "It is not sent if it becomes visible on all virtual desktops though."]
            async fn virtual_desktop_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                number: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window#{}.virtual_desktop_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(number).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent whenever the themed icon name changes. May be null."]
            async fn themed_icon_name_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window#{}.themed_icon_name_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent immediately after the window is closed"]
            #[doc = "and its surface is unmapped."]
            async fn unmapped(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.unmapped()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent immediately after all initial state been sent to the client."]
            #[doc = "If the Plasma window is already unmapped, the unmapped event will be sent before the"]
            #[doc = "initial_state event."]
            async fn initial_state(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.initial_state()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent whenever the parent window of this org_kde_plasma_window changes."]
            #[doc = "The passed parent is another org_kde_plasma_window and this org_kde_plasma_window is a"]
            #[doc = "transient window to the parent window. If the parent argument is null, this"]
            #[doc = "org_kde_plasma_window does not have a parent window."]
            async fn parent_window(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                parent: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.parent_window()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(parent)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent whenever the window geometry of this org_kde_plasma_window changes."]
            #[doc = "The coordinates are in absolute coordinates of the windowing system."]
            async fn geometry(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                width: u32,
                height: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.geometry()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_uint(width)
                    .put_uint(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent whenever the icon of the window changes, but there is no themed"]
            #[doc = "icon name. Common examples are Xwayland windows which have a pixmap based icon."]
            #[doc = ""]
            #[doc = "The client can request the icon using get_icon."]
            async fn icon_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.icon_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when the compositor has set the process id this window belongs to."]
            #[doc = "This should be set once before the initial_state is sent."]
            async fn pid_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                pid: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.pid_changed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(pid).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when the window has entered a new virtual desktop. The window can be on more than one desktop, or none: then is considered on all of them."]
            async fn virtual_desktop_entered(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window#{}.virtual_desktop_entered()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 11u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when the window left a virtual desktop. If the window leaves all desktops, it can be considered on all."]
            #[doc = "If the window gets manually added on all desktops, the server has to send virtual_desktop_left for every previous desktop it was in for the window to be really considered on all desktops."]
            async fn virtual_desktop_left(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                is: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window#{}.virtual_desktop_left()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(is))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 12u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent after the application menu"]
            #[doc = "for the window has changed."]
            async fn application_menu(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                service_name: String,
                object_path: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.application_menu()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(service_name))
                    .put_string(Some(object_path))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 13u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when the window has entered an activity. The window can be on more than one activity, or none: then is considered on all of them."]
            async fn activity_entered(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.activity_entered()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 14u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when the window left an activity. If the window leaves all activities, it will be considered on all."]
            #[doc = "If the window gets manually added on all activities, the server has to send activity_left for every previous activity it was in for the window to be really considered on all activities."]
            async fn activity_left(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_window#{}.activity_left()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 15u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent when the X11 resource name of the window has changed."]
            #[doc = "This is only set for XWayland windows."]
            async fn resource_name_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                resource_name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_window#{}.resource_name_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(resource_name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 16u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "The activation manager interface provides a way to get notified"]
    #[doc = "when an application is about to be activated."]
    pub mod org_kde_plasma_activation_feedback {
        #[doc = "Trait to implement the org_kde_plasma_activation_feedback interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaActivationFeedback: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_activation_feedback";
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
                            "org_kde_plasma_activation_feedback#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the activation manager object. The activation objects introduced"]
            #[doc = "by this manager object will be unaffected."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Will be issued when an app is set to be activated. It offers"]
            #[doc = "an instance of org_kde_plasma_activation that will tell us the app_id"]
            #[doc = "and the extent of the activation."]
            async fn activation(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> org_kde_plasma_activation_feedback#{}.activation()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    pub mod org_kde_plasma_activation {
        #[doc = "Trait to implement the org_kde_plasma_activation interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaActivation: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_activation";
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
                        tracing::debug!("org_kde_plasma_activation#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the org_kde_plasma_activation object will no"]
            #[doc = "longer be used."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn app_id(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                app_id: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_activation#{}.app_id()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn finished(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_activation#{}.finished()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "When this object is created, the compositor sends a window event for"]
    #[doc = "each window in the stacking order, and afterwards sends the done event"]
    #[doc = "and destroys this object."]
    pub mod org_kde_plasma_stacking_order {
        #[doc = "Trait to implement the org_kde_plasma_stacking_order interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaStackingOrder: crate::server::Dispatcher {
            const INTERFACE: &'static str = "org_kde_plasma_stacking_order";
            const VERSION: u32 = 17u32;
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
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn window(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                uuid: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_stacking_order#{}.window()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(uuid))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> org_kde_plasma_stacking_order#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod zkde_screencast_unstable_v1 {
    pub mod zkde_screencast_unstable_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Pointer {
            #[doc = "No cursor"]
            Hidden = 1u32,
            #[doc = "Render the cursor on the stream"]
            Embedded = 2u32,
            #[doc = "Send metadata about where the cursor is through PipeWire"]
            Metadata = 4u32,
        }
        impl TryFrom<u32> for Pointer {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Hidden),
                    2u32 => Ok(Self::Embedded),
                    4u32 => Ok(Self::Metadata),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zkde_screencast_unstable_v1 interface. See the module level documentation for more info"]
        pub trait ZkdeScreencastUnstableV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zkde_screencast_unstable_v1";
            const VERSION: u32 = 3u32;
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
                            "zkde_screencast_unstable_v1#{}.stream_output()",
                            object.id
                        );
                        self.stream_output(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "zkde_screencast_unstable_v1#{}.stream_window()",
                            object.id
                        );
                        self.stream_window(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("zkde_screencast_unstable_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    3u16 => {
                        tracing::debug!(
                            "zkde_screencast_unstable_v1#{}.stream_virtual_output()",
                            object.id
                        );
                        self.stream_virtual_output(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.fixed()?,
                            message.uint()?,
                        )
                        .await
                    }
                    4u16 => {
                        tracing::debug!(
                            "zkde_screencast_unstable_v1#{}.stream_region()",
                            object.id
                        );
                        self.stream_region(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.uint()?,
                            message.uint()?,
                            message.fixed()?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn stream_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                stream: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                pointer: u32,
            ) -> crate::server::Result<()>;
            async fn stream_window(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                stream: crate::wire::ObjectId,
                window_uuid: String,
                pointer: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the zkde_screencast_unstable_v1 object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn stream_virtual_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                stream: crate::wire::ObjectId,
                name: String,
                width: i32,
                height: i32,
                scale: crate::wire::Fixed,
                pointer: u32,
            ) -> crate::server::Result<()>;
            async fn stream_region(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                stream: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: u32,
                height: u32,
                scale: crate::wire::Fixed,
                pointer: u32,
            ) -> crate::server::Result<()>;
        }
    }
    pub mod zkde_screencast_stream_unstable_v1 {
        #[doc = "Trait to implement the zkde_screencast_stream_unstable_v1 interface. See the module level documentation for more info"]
        pub trait ZkdeScreencastStreamUnstableV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zkde_screencast_stream_unstable_v1";
            const VERSION: u32 = 3u32;
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
                        tracing::debug!("zkde_screencast_stream_unstable_v1#{}.close()", object.id);
                        self.close(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn close(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn closed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> zkde_screencast_stream_unstable_v1#{}.closed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn created(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                node: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> zkde_screencast_stream_unstable_v1#{}.created()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(node).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            async fn failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                error: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> zkde_screencast_stream_unstable_v1#{}.failed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(error))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
