#[doc = "This protocol allows a client to control outputs' color transform matrix (CTM)."]
#[doc = ""]
#[doc = "This protocol is privileged and should not be exposed to unprivileged clients."]
#[allow(clippy::module_inception)]
pub mod hyprland_ctm_control_v1 {
    #[doc = "This object is a manager which offers requests to control CTMs."]
    #[doc = ""]
    #[doc = "If any changes are done, once this object is destroyed, CTMs are reset back to"]
    #[doc = "an identity matrix."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_ctm_control_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the matrix values are invalid."]
            InvalidMatrix = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidMatrix),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the hyprland_ctm_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandCtmControlManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_ctm_control_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let mat0 = message.fixed()?;
                            let mat1 = message.fixed()?;
                            let mat2 = message.fixed()?;
                            let mat3 = message.fixed()?;
                            let mat4 = message.fixed()?;
                            let mat5 = message.fixed()?;
                            let mat6 = message.fixed()?;
                            let mat7 = message.fixed()?;
                            let mat8 = message.fixed()?;
                            tracing::debug!(
                                "hyprland_ctm_control_manager_v1#{}.set_ctm_for_output({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                output,
                                mat0,
                                mat1,
                                mat2,
                                mat3,
                                mat4,
                                mat5,
                                mat6,
                                mat7,
                                mat8
                            );
                            self.set_ctm_for_output(
                                client, sender_id, output, mat0, mat1, mat2, mat3, mat4, mat5,
                                mat6, mat7, mat8,
                            )
                            .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "hyprland_ctm_control_manager_v1#{}.commit()",
                                sender_id,
                            );
                            self.commit(client, sender_id).await
                        }
                        2u16 => {
                            tracing::debug!(
                                "hyprland_ctm_control_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Set a CTM for a wl_output."]
            #[doc = ""]
            #[doc = "This state is not applied immediately; clients must call .commit to"]
            #[doc = "apply any pending changes."]
            #[doc = ""]
            #[doc = "The provided values describe a 3x3 Row-Major CTM with values in the range of [0, ∞)"]
            #[doc = ""]
            #[doc = "Passing values outside of the range will raise an invalid_matrix error."]
            #[doc = ""]
            #[doc = "The default value of the CTM is an identity matrix."]
            #[doc = ""]
            #[doc = "If an output doesn't get a CTM set with set_ctm_for_output and commit is called,"]
            #[doc = "that output will get its CTM reset to an identity matrix."]
            fn set_ctm_for_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                mat0: crate::wire::Fixed,
                mat1: crate::wire::Fixed,
                mat2: crate::wire::Fixed,
                mat3: crate::wire::Fixed,
                mat4: crate::wire::Fixed,
                mat5: crate::wire::Fixed,
                mat6: crate::wire::Fixed,
                mat7: crate::wire::Fixed,
                mat8: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Commits the pending state(s) set by set_ctm_for_output."]
            fn commit(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            #[doc = ""]
            #[doc = "The CTMs of all outputs will be reset to an identity matrix."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[doc = "This protocol allows clients to limit input focus to a specific set"]
#[doc = "of surfaces and receive a notification when the limiter is removed as"]
#[doc = "detailed below."]
#[allow(clippy::module_inception)]
pub mod hyprland_focus_grab_v1 {
    #[doc = "This interface allows a client to create surface grab objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_focus_grab_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the hyprland_focus_grab_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandFocusGrabManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_focus_grab_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let grab = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "hyprland_focus_grab_manager_v1#{}.create_grab({})",
                                sender_id,
                                grab
                            );
                            self.create_grab(client, sender_id, grab).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "hyprland_focus_grab_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Create a surface grab object."]
            fn create_grab(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                grab: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroy the focus grab manager."]
            #[doc = "This doesn't destroy existing focus grab objects."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = "This interface restricts input focus to a specified whitelist of"]
    #[doc = "surfaces as long as the focus grab object exists and has at least"]
    #[doc = "one comitted surface."]
    #[doc = ""]
    #[doc = "Mouse and touch events inside a whitelisted surface will be passed"]
    #[doc = "to the surface normally, while events outside of a whitelisted surface"]
    #[doc = "will clear the grab object. Keyboard events will be passed to the client"]
    #[doc = "and a compositor-picked surface in the whitelist will receive a"]
    #[doc = "wl_keyboard::enter event if a whitelisted surface is not already entered."]
    #[doc = ""]
    #[doc = "Upon meeting implementation-defined criteria usually meaning a mouse or"]
    #[doc = "touch input outside of any whitelisted surfaces, the compositor will"]
    #[doc = "clear the whitelist, rendering the grab inert and sending the cleared"]
    #[doc = "event. The same will happen if another focus grab or similar action"]
    #[doc = "is started at the compositor's discretion."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_focus_grab_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the hyprland_focus_grab_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandFocusGrabV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_focus_grab_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "hyprland_focus_grab_v1#{}.add_surface({})",
                                sender_id,
                                surface
                            );
                            self.add_surface(client, sender_id, surface).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "hyprland_focus_grab_v1#{}.remove_surface({})",
                                sender_id,
                                surface
                            );
                            self.remove_surface(client, sender_id, surface).await
                        }
                        2u16 => {
                            tracing::debug!("hyprland_focus_grab_v1#{}.commit()", sender_id,);
                            self.commit(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!("hyprland_focus_grab_v1#{}.destroy()", sender_id,);
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Add a surface to the whitelist. Destroying the surface is treated the"]
            #[doc = "same as an explicit call to remove_surface and duplicate additions are"]
            #[doc = "ignored."]
            #[doc = ""]
            #[doc = "Does not take effect until commit is called."]
            fn add_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Remove a surface from the whitelist. Destroying the surface is treated"]
            #[doc = "the same as an explicit call to this function."]
            #[doc = ""]
            #[doc = "If the grab was active and the removed surface was entered by the"]
            #[doc = "keyboard, another surface will be entered on commit."]
            #[doc = ""]
            #[doc = "Does not take effect until commit is called."]
            fn remove_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Commit pending changes to the surface whitelist."]
            #[doc = ""]
            #[doc = "If the list previously had no entries and now has at least one, the grab"]
            #[doc = "will start. If it previously had entries and now has none, the grab will"]
            #[doc = "become inert."]
            fn commit(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroy the grab object and remove the grab if active."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Sent when an active grab is cancelled by the compositor,"]
            #[doc = "regardless of cause."]
            fn cleared(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> hyprland_focus_grab_v1#{}.cleared()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[doc = "This protocol allows a client to register triggerable actions,"]
#[doc = "meant to be global shortcuts."]
#[allow(clippy::module_inception)]
pub mod hyprland_global_shortcuts_v1 {
    #[doc = "This object is a manager which offers requests to create global shortcuts."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_global_shortcuts_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the app_id + id combination has already been registered."]
            AlreadyTaken = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyTaken),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the hyprland_global_shortcuts_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandGlobalShortcutsManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_global_shortcuts_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let shortcut = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let id = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let app_id = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let description = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let trigger_description = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "hyprland_global_shortcuts_manager_v1#{}.register_shortcut({}, \"{}\", \"{}\", \"{}\", \"{}\")",
                                sender_id,
                                shortcut,
                                id,
                                app_id,
                                description,
                                trigger_description
                            );
                            self.register_shortcut(
                                client,
                                sender_id,
                                shortcut,
                                id,
                                app_id,
                                description,
                                trigger_description,
                            )
                            .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "hyprland_global_shortcuts_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Register a new global shortcut."]
            #[doc = ""]
            #[doc = "A global shortcut is anonymous, meaning the app does not know what key(s) trigger it."]
            #[doc = ""]
            #[doc = "The shortcut's keybinding shall be dealt with by the compositor."]
            #[doc = ""]
            #[doc = "In the case of a duplicate app_id + id combination, the already_taken protocol error is raised."]
            fn register_shortcut(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                shortcut: crate::wire::ObjectId,
                id: String,
                app_id: String,
                description: String,
                trigger_description: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = "This object represents a single shortcut."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_global_shortcut_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the hyprland_global_shortcut_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandGlobalShortcutV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_global_shortcut_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            tracing::debug!("hyprland_global_shortcut_v1#{}.destroy()", sender_id,);
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Destroys the shortcut. Can be sent at any time by the client."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "The keystroke was pressed."]
            #[doc = ""]
            #[doc = "tv_ values hold the timestamp of the occurrence."]
            fn pressed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_global_shortcut_v1#{}.pressed({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "The keystroke was released."]
            #[doc = ""]
            #[doc = "tv_ values hold the timestamp of the occurrence."]
            fn released(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_global_shortcut_v1#{}.released({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[doc = "This protocol exposes hyprland-specific wl_surface properties."]
#[allow(clippy::module_inception)]
pub mod hyprland_surface_v1 {
    #[doc = "This interface allows a client to create hyprland surface objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_surface_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface already has a hyprland surface object"]
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the hyprland_surface_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandSurfaceManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_surface_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "hyprland_surface_manager_v1#{}.get_hyprland_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_hyprland_surface(client, sender_id, id, surface)
                                .await
                        }
                        1u16 => {
                            tracing::debug!("hyprland_surface_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Create a hyprland surface object for the given wayland surface."]
            #[doc = ""]
            #[doc = "If the wl_surface already has an associated hyprland_surface_v1 object,"]
            #[doc = "even from a different manager, creation is a protocol error."]
            fn get_hyprland_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroy the surface manager."]
            #[doc = "This does not destroy existing surface objects."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = "This interface allows access to hyprland-specific properties of a wl_surface."]
    #[doc = ""]
    #[doc = "Once the wl_surface has been destroyed, the hyprland surface object must be"]
    #[doc = "destroyed as well. All other operations are a protocol error."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_surface_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface was destroyed"]
            NoSurface = 0u32,
            #[doc = "given opacity was not in the range 0.0 - 1.0 (inclusive)"]
            OutOfRange = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoSurface),
                    1u32 => Ok(Self::OutOfRange),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the hyprland_surface_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandSurfaceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_surface_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let opacity = message.fixed()?;
                            tracing::debug!(
                                "hyprland_surface_v1#{}.set_opacity({})",
                                sender_id,
                                opacity
                            );
                            self.set_opacity(client, sender_id, opacity).await
                        }
                        1u16 => {
                            tracing::debug!("hyprland_surface_v1#{}.destroy()", sender_id,);
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Sets a multiplier for the overall opacity of the surface."]
            #[doc = "This multiplier applies to visual effects such as blur behind the surface"]
            #[doc = "in addition to the surface's content."]
            #[doc = ""]
            #[doc = "The default value is 1.0."]
            #[doc = "Setting a value outside of the range 0.0 - 1.0 (inclusive) is a protocol error."]
            #[doc = "Does not take effect until wl_surface.commit is called."]
            fn set_opacity(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                opacity: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroy the hyprland surface object, resetting properties provided"]
            #[doc = "by this interface to their default values on the next wl_surface.commit."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[doc = "This protocol allows clients to ask for exporting another toplevel's"]
#[doc = "surface(s) to a buffer."]
#[doc = ""]
#[doc = "Particularly useful for sharing a single window."]
#[allow(clippy::module_inception)]
pub mod hyprland_toplevel_export_v1 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_toplevel_export_manager_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the hyprland_toplevel_export_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandToplevelExportManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_toplevel_export_manager_v1";
            const VERSION: u32 = 2u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let frame = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let overlay_cursor = message.int()?;
                            let handle = message.uint()?;
                            tracing::debug!(
                                "hyprland_toplevel_export_manager_v1#{}.capture_toplevel({}, {}, {})",
                                sender_id,
                                frame,
                                overlay_cursor,
                                handle
                            );
                            self.capture_toplevel(client, sender_id, frame, overlay_cursor, handle)
                                .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "hyprland_toplevel_export_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(client, sender_id).await
                        }
                        2u16 => {
                            let frame = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let overlay_cursor = message.int()?;
                            let handle = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "hyprland_toplevel_export_manager_v1#{}.capture_toplevel_with_wlr_toplevel_handle({}, {}, {})",
                                sender_id,
                                frame,
                                overlay_cursor,
                                handle
                            );
                            self.capture_toplevel_with_wlr_toplevel_handle(
                                client,
                                sender_id,
                                frame,
                                overlay_cursor,
                                handle,
                            )
                            .await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Capture the next frame of a toplevel. (window)"]
            #[doc = ""]
            #[doc = "The captured frame will not contain any server-side decorations and will"]
            #[doc = "ignore the compositor-set geometry, like e.g. rounded corners."]
            #[doc = ""]
            #[doc = "It will contain all the subsurfaces and popups, however the latter will be clipped"]
            #[doc = "to the geometry of the base surface."]
            #[doc = ""]
            #[doc = "The handle parameter refers to the address of the window as seen in `hyprctl clients`."]
            #[doc = "For example, for d161e7b0 it would be 3512854448."]
            fn capture_toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                handle: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle."]
            fn capture_toplevel_with_wlr_toplevel_handle(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                handle: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = "This object represents a single frame."]
    #[doc = ""]
    #[doc = "When created, a series of buffer events will be sent, each representing a"]
    #[doc = "supported buffer type. The \"buffer_done\" event is sent afterwards to"]
    #[doc = "indicate that all supported buffer types have been enumerated. The client"]
    #[doc = "will then be able to send a \"copy\" request. If the capture is successful,"]
    #[doc = "the compositor will send a \"flags\" followed by a \"ready\" event."]
    #[doc = ""]
    #[doc = "wl_shm buffers are always supported, ie. the \"buffer\" event is guaranteed to be sent."]
    #[doc = ""]
    #[doc = "If the capture failed, the \"failed\" event is sent. This can happen anytime"]
    #[doc = "before the \"ready\" event."]
    #[doc = ""]
    #[doc = "Once either a \"ready\" or a \"failed\" event is received, the client should"]
    #[doc = "destroy the frame."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_toplevel_export_frame_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the object has already been used to copy a wl_buffer"]
            AlreadyUsed = 0u32,
            #[doc = "buffer attributes are invalid"]
            InvalidBuffer = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    1u32 => Ok(Self::InvalidBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInvert = 1u32 ; } }
        impl TryFrom<u32> for Flags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the hyprland_toplevel_export_frame_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandToplevelExportFrameV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "hyprland_toplevel_export_frame_v1";
            const VERSION: u32 = 2u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode {
                        0u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let ignore_damage = message.int()?;
                            tracing::debug!(
                                "hyprland_toplevel_export_frame_v1#{}.copy({}, {})",
                                sender_id,
                                buffer,
                                ignore_damage
                            );
                            self.copy(client, sender_id, buffer, ignore_damage).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "hyprland_toplevel_export_frame_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(client, sender_id).await
                        }
                        _ => Err(crate::server::error::Error::UnknownOpcode),
                    }
                }
            }
            #[doc = "Copy the frame to the supplied buffer. The buffer must have the"]
            #[doc = "correct size, see hyprland_toplevel_export_frame_v1.buffer and"]
            #[doc = "hyprland_toplevel_export_frame_v1.linux_dmabuf. The buffer needs to have a"]
            #[doc = "supported format."]
            #[doc = ""]
            #[doc = "If the frame is successfully copied, a \"flags\" and a \"ready\" event is"]
            #[doc = "sent. Otherwise, a \"failed\" event is sent."]
            #[doc = ""]
            #[doc = "This event will wait for appropriate damage to be copied, unless the ignore_damage"]
            #[doc = "arg is set to a non-zero value."]
            fn copy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
                ignore_damage: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Destroys the frame. This request can be sent at any time by the client."]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = "Provides information about wl_shm buffer parameters that need to be"]
            #[doc = "used for this frame. This event is sent once after the frame is created"]
            #[doc = "if wl_shm buffers are supported."]
            fn buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: super::super::super::core::wayland::wl_shm::Format,
                width: u32,
                height: u32,
                stride: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.buffer({}, {}, {}, {})",
                        sender_id,
                        format,
                        width,
                        height,
                        stride
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(format as u32)
                        .put_uint(width)
                        .put_uint(height)
                        .put_uint(stride)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent right before the ready event when ignore_damage was"]
            #[doc = "not set. It may be generated multiple times for each copy"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "The arguments describe a box around an area that has changed since the"]
            #[doc = "last copy request that was derived from the current screencopy manager"]
            #[doc = "instance."]
            #[doc = ""]
            #[doc = "The union of all regions received between the call to copy"]
            #[doc = "and a ready event is the total damage since the prior ready event."]
            fn damage(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.damage({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(x)
                        .put_uint(y)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Provides flags about the frame. This event is sent once before the"]
            #[doc = "\"ready\" event."]
            fn flags(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                flags: Flags,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.flags({})",
                        sender_id,
                        flags
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(flags.bits())
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading. This event includes the time at which presentation happened"]
            #[doc = "at."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part"]
            #[doc = "may have an arbitrary offset at start."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.ready({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.failed()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "Provides information about linux-dmabuf buffer parameters that need to"]
            #[doc = "be used for this frame. This event is sent once after the frame is"]
            #[doc = "created if linux-dmabuf buffers are supported."]
            fn linux_dmabuf(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: u32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.linux_dmabuf({}, {}, {})",
                        sender_id,
                        format,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(format)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = "This event is sent once after all buffer events have been sent."]
            #[doc = ""]
            #[doc = "The client should proceed to create a buffer of one of the supported"]
            #[doc = "types, and send a \"copy\" request."]
            fn buffer_done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> hyprland_toplevel_export_frame_v1#{}.buffer_done()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
