#![allow(async_fn_in_trait)]
#[doc = "This protocol allows a client to control outputs' color transform matrix (CTM)."]
#[doc = ""]
#[doc = "This protocol is privileged and should not be exposed to unprivileged clients."]
pub mod hyprland_ctm_control_v1 {
    #[doc = "This object is a manager which offers requests to control CTMs."]
    #[doc = ""]
    #[doc = "If any changes are done, once this object is destroyed, CTMs are reset back to"]
    #[doc = "an identity matrix."]
    pub mod hyprland_ctm_control_manager_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the hyprland_ctm_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandCtmControlManagerV1 {
            const INTERFACE: &'static str = "hyprland_ctm_control_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
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
            async fn set_ctm_for_output(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
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
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_ctm_control_manager_v1#{}.set_ctm_for_output()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .put_fixed(mat0)
                    .put_fixed(mat1)
                    .put_fixed(mat2)
                    .put_fixed(mat3)
                    .put_fixed(mat4)
                    .put_fixed(mat5)
                    .put_fixed(mat6)
                    .put_fixed(mat7)
                    .put_fixed(mat8)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Commits the pending state(s) set by set_ctm_for_output."]
            async fn commit(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_ctm_control_manager_v1#{}.commit()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            #[doc = ""]
            #[doc = "The CTMs of all outputs will be reset to an identity matrix."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_ctm_control_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows clients to limit input focus to a specific set"]
#[doc = "of surfaces and receive a notification when the limiter is removed as"]
#[doc = "detailed below."]
pub mod hyprland_focus_grab_v1 {
    #[doc = "This interface allows a client to create surface grab objects."]
    pub mod hyprland_focus_grab_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_focus_grab_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandFocusGrabManagerV1 {
            const INTERFACE: &'static str = "hyprland_focus_grab_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a surface grab object."]
            async fn create_grab(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                grab: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_focus_grab_manager_v1#{}.create_grab()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(grab))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the focus grab manager."]
            #[doc = "This doesn't destroy existing focus grab objects."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_focus_grab_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
    pub mod hyprland_focus_grab_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_focus_grab_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandFocusGrabV1 {
            const INTERFACE: &'static str = "hyprland_focus_grab_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Add a surface to the whitelist. Destroying the surface is treated the"]
            #[doc = "same as an explicit call to remove_surface and duplicate additions are"]
            #[doc = "ignored."]
            #[doc = ""]
            #[doc = "Does not take effect until commit is called."]
            async fn add_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_focus_grab_v1#{}.add_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Remove a surface from the whitelist. Destroying the surface is treated"]
            #[doc = "the same as an explicit call to this function."]
            #[doc = ""]
            #[doc = "If the grab was active and the removed surface was entered by the"]
            #[doc = "keyboard, another surface will be entered on commit."]
            #[doc = ""]
            #[doc = "Does not take effect until commit is called."]
            async fn remove_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_focus_grab_v1#{}.remove_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Commit pending changes to the surface whitelist."]
            #[doc = ""]
            #[doc = "If the list previously had no entries and now has at least one, the grab"]
            #[doc = "will start. If it previously had entries and now has none, the grab will"]
            #[doc = "become inert."]
            async fn commit(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_focus_grab_v1#{}.commit()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the grab object and remove the grab if active."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_focus_grab_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows a client to register triggerable actions,"]
#[doc = "meant to be global shortcuts."]
pub mod hyprland_global_shortcuts_v1 {
    #[doc = "This object is a manager which offers requests to create global shortcuts."]
    pub mod hyprland_global_shortcuts_manager_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the hyprland_global_shortcuts_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandGlobalShortcutsManagerV1 {
            const INTERFACE: &'static str = "hyprland_global_shortcuts_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Register a new global shortcut."]
            #[doc = ""]
            #[doc = "A global shortcut is anonymous, meaning the app does not know what key(s) trigger it."]
            #[doc = ""]
            #[doc = "The shortcut's keybinding shall be dealt with by the compositor."]
            #[doc = ""]
            #[doc = "In the case of a duplicate app_id + id combination, the already_taken protocol error is raised."]
            async fn register_shortcut(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                shortcut: crate::wire::ObjectId,
                id: String,
                app_id: String,
                description: String,
                trigger_description: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_global_shortcuts_manager_v1#{}.register_shortcut()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(shortcut))
                    .put_string(Some(id))
                    .put_string(Some(app_id))
                    .put_string(Some(description))
                    .put_string(Some(trigger_description))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_global_shortcuts_manager_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object represents a single shortcut."]
    pub mod hyprland_global_shortcut_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_global_shortcut_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandGlobalShortcutV1 {
            const INTERFACE: &'static str = "hyprland_global_shortcut_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the shortcut. Can be sent at any time by the client."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_global_shortcut_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows clients to ask for exporting another toplevel's"]
#[doc = "surface(s) to a buffer."]
#[doc = ""]
#[doc = "Particularly useful for sharing a single window."]
pub mod hyprland_toplevel_export_v1 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    pub mod hyprland_toplevel_export_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_toplevel_export_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandToplevelExportManagerV1 {
            const INTERFACE: &'static str = "hyprland_toplevel_export_manager_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
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
            async fn capture_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                handle: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_export_manager_v1#{}.capture_toplevel()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .put_int(overlay_cursor)
                    .put_uint(handle)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_export_manager_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle."]
            async fn capture_toplevel_with_wlr_toplevel_handle(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
                overlay_cursor: i32,
                handle: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing :: debug ! ("-> hyprland_toplevel_export_manager_v1#{}.capture_toplevel_with_wlr_toplevel_handle()" , object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .put_int(overlay_cursor)
                    .put_object(Some(handle))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
    pub mod hyprland_toplevel_export_frame_v1 {
        use futures_util::SinkExt;
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
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInvert = 1u32 ; } }
        impl TryFrom<u32> for Flags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the hyprland_toplevel_export_frame_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandToplevelExportFrameV1 {
            const INTERFACE: &'static str = "hyprland_toplevel_export_frame_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
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
            async fn copy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
                ignore_damage: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_toplevel_export_frame_v1#{}.copy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .put_int(ignore_damage)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the frame. This request can be sent at any time by the client."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_export_frame_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
