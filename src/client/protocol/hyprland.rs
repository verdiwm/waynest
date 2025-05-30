#![allow(async_fn_in_trait)]
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the hyprland_ctm_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandCtmControlManagerV1 {
            const INTERFACE: &'static str = "hyprland_ctm_control_manager_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
            #[doc = "This event is sent if another manager was bound by any client"]
            #[doc = "at the time the current manager was bound."]
            #[doc = "Any set_ctm_for_output requests from a blocked manager will be"]
            #[doc = "silently ignored by the compositor."]
            #[doc = ""]
            #[doc = "The client should destroy the manager after receiving this event."]
            async fn blocked(&self) -> crate::client::Result<()>;
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
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_focus_grab_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandFocusGrabManagerV1 {
            const INTERFACE: &'static str = "hyprland_focus_grab_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
    #[allow(clippy::too_many_arguments)]
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
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
            #[doc = "Sent when an active grab is cancelled by the compositor,"]
            #[doc = "regardless of cause."]
            async fn cleared(&self) -> crate::client::Result<()>;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
    #[allow(clippy::too_many_arguments)]
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
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
            #[doc = "The keystroke was pressed."]
            #[doc = ""]
            #[doc = "tv_ values hold the timestamp of the occurrence."]
            async fn pressed(
                &self,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
            #[doc = "The keystroke was released."]
            #[doc = ""]
            #[doc = "tv_ values hold the timestamp of the occurrence."]
            async fn released(
                &self,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod hyprland_lock_notify_v1 {
    #[doc = "This interface allows clients to monitor whether the wayland session is"]
    #[doc = "locked or unlocked."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_lock_notifier_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_lock_notifier_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandLockNotifierV1 {
            const INTERFACE: &'static str = "hyprland_lock_notifier_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the manager object. All objects created via this interface"]
            #[doc = "remain valid."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_lock_notifier_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a new lock notification object."]
            #[doc = ""]
            #[doc = "If the session is already locked when calling this method,"]
            #[doc = "the locked event shall be sent immediately."]
            async fn get_lock_notification(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_lock_notifier_v1#{}.get_lock_notification()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This interface is used by the compositor to send lock notification events"]
    #[doc = "to clients."]
    #[doc = ""]
    #[doc = "Typically the \"locked\" and \"unlocked\" events are emitted when a client"]
    #[doc = "locks/unlocks the session via ext-session-lock, but the compositor may"]
    #[doc = "choose to send notifications for any other locking mechanisms."]
    #[doc = ""]
    #[doc = "The compositor must notfiy after possible transition periods"]
    #[doc = "between locked and unlocked states of the session."]
    #[doc = "In the context of ext-session-lock, that means the \"locked\" event is"]
    #[doc = "expected to be sent after the session-lock client has presented"]
    #[doc = "a lock screen frame on every output, which corresponds to the \"locked\""]
    #[doc = "event of ext-session-lock."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_lock_notification_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_lock_notification_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandLockNotificationV1 {
            const INTERFACE: &'static str = "hyprland_lock_notification_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the notification object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_lock_notification_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent when the wayland session is locked."]
            #[doc = ""]
            #[doc = "It's a compositor protocol error to send this event twice without an"]
            #[doc = "unlock event in-between."]
            async fn locked(&self) -> crate::client::Result<()>;
            #[doc = "This event is sent when the wayland session is unlocked."]
            #[doc = ""]
            #[doc = "It's a compositor protocol error to send this event twice without an"]
            #[doc = "locked event in-between. It's a compositor protocol error to send this"]
            #[doc = "event prior to any locked event."]
            async fn unlocked(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol exposes hyprland-specific wl_surface properties."]
#[allow(clippy::module_inception)]
pub mod hyprland_surface_v1 {
    #[doc = "This interface allows a client to create hyprland surface objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_surface_manager_v1 {
        use futures_util::SinkExt;
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
        pub trait HyprlandSurfaceManagerV1 {
            const INTERFACE: &'static str = "hyprland_surface_manager_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a hyprland surface object for the given wayland surface."]
            #[doc = ""]
            #[doc = "If the wl_surface already has an associated hyprland_surface_v1 object,"]
            #[doc = "even from a different manager, creation is a protocol error."]
            async fn get_hyprland_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_surface_manager_v1#{}.get_hyprland_surface()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the surface manager."]
            #[doc = "This does not destroy existing surface objects."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_surface_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This interface allows access to hyprland-specific properties of a wl_surface."]
    #[doc = ""]
    #[doc = "Once the wl_surface has been destroyed, the hyprland surface object must be"]
    #[doc = "destroyed as well. All other operations are a protocol error."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_surface_v1 {
        use futures_util::SinkExt;
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
        pub trait HyprlandSurfaceV1 {
            const INTERFACE: &'static str = "hyprland_surface_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Sets a multiplier for the overall opacity of the surface."]
            #[doc = "This multiplier applies to visual effects such as blur behind the surface"]
            #[doc = "in addition to the surface's content."]
            #[doc = ""]
            #[doc = "The default value is 1.0."]
            #[doc = "Setting a value outside of the range 0.0 - 1.0 (inclusive) is a protocol error."]
            #[doc = "Does not take effect until wl_surface.commit is called."]
            async fn set_opacity(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                opacity: crate::wire::Fixed,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_surface_v1#{}.set_opacity()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fixed(opacity)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the hyprland surface object, resetting properties provided"]
            #[doc = "by this interface to their default values on the next wl_surface.commit."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_surface_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the region of the surface that contains visible content."]
            #[doc = "Visible content refers to content that has an alpha value greater than zero."]
            #[doc = ""]
            #[doc = "The visible region is an optimization hint for the compositor that lets it"]
            #[doc = "avoid drawing parts of the surface that are not visible. Setting a visible region"]
            #[doc = "that does not contain all content in the surface may result in missing content"]
            #[doc = "not being drawn."]
            #[doc = ""]
            #[doc = "The visible region is specified in buffer-local coordinates."]
            #[doc = ""]
            #[doc = "The compositor ignores the parts of the visible region that fall outside of the surface."]
            #[doc = "When all parts of the region fall outside of the buffer geometry, the compositor may"]
            #[doc = "avoid rendering the surface entirely."]
            #[doc = ""]
            #[doc = "The initial value for the visible region is empty. Setting the"]
            #[doc = "visible region has copy semantics, and the wl_region object can be destroyed immediately."]
            #[doc = "A NULL wl_region causes the visible region to be set to empty."]
            #[doc = ""]
            #[doc = "Does not take effect until wl_surface.commit is called."]
            async fn set_visible_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> hyprland_surface_v1#{}.set_visible_region()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(region)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
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
#[allow(clippy::module_inception)]
pub mod hyprland_toplevel_export_v1 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    #[allow(clippy::too_many_arguments)]
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
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
                tracing::debug!(
                    "-> hyprland_toplevel_export_manager_v1#{}.capture_toplevel_with_wlr_toplevel_handle()",
                    object_id
                );
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
    #[allow(clippy::too_many_arguments)]
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
        pub trait HyprlandToplevelExportFrameV1 {
            const INTERFACE: &'static str = "hyprland_toplevel_export_frame_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
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
            #[doc = "Provides information about wl_shm buffer parameters that need to be"]
            #[doc = "used for this frame. This event is sent once after the frame is created"]
            #[doc = "if wl_shm buffers are supported."]
            async fn buffer(
                &self,
                format: super::super::super::core::wayland::wl_shm::Format,
                width: u32,
                height: u32,
                stride: u32,
            ) -> crate::client::Result<()>;
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
            async fn damage(
                &self,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Provides flags about the frame. This event is sent once before the"]
            #[doc = "\"ready\" event."]
            async fn flags(&self, flags: Flags) -> crate::client::Result<()>;
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
            async fn ready(
                &self,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            async fn failed(&self) -> crate::client::Result<()>;
            #[doc = "Provides information about linux-dmabuf buffer parameters that need to"]
            #[doc = "be used for this frame. This event is sent once after the frame is"]
            #[doc = "created if linux-dmabuf buffers are supported."]
            async fn linux_dmabuf(
                &self,
                format: u32,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent once after all buffer events have been sent."]
            #[doc = ""]
            #[doc = "The client should proceed to create a buffer of one of the supported"]
            #[doc = "types, and send a \"copy\" request."]
            async fn buffer_done(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol allows clients to retrieve the mapping of toplevels to hyprland window addresses."]
#[allow(clippy::module_inception)]
pub mod hyprland_toplevel_mapping_v1 {
    #[doc = "This object is a manager which offers requests to retrieve a window address"]
    #[doc = "for a toplevel."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_toplevel_mapping_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_toplevel_mapping_manager_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandToplevelMappingManagerV1 {
            const INTERFACE: &'static str = "hyprland_toplevel_mapping_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Get the window address for a toplevel."]
            async fn get_window_for_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                handle: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_mapping_manager_v1#{}.get_window_for_toplevel()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(handle))
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Get the window address for a wlr toplevel."]
            async fn get_window_for_toplevel_wlr(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                handle: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_mapping_manager_v1#{}.get_window_for_toplevel_wlr()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(handle))
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "All objects created by the manager will still remain valid, until their appropriate destroy"]
            #[doc = "request has been called."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_mapping_manager_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object represents a mapping of a (wlr) toplevel to a window address."]
    #[doc = ""]
    #[doc = "Once created, the `window_address` event will be sent containing the address of the window"]
    #[doc = "associated with the toplevel."]
    #[doc = "Should the mapping fail, the `failed` event will be sent."]
    #[allow(clippy::too_many_arguments)]
    pub mod hyprland_toplevel_window_mapping_handle_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the hyprland_toplevel_window_mapping_handle_v1 interface. See the module level documentation for more info"]
        pub trait HyprlandToplevelWindowMappingHandleV1 {
            const INTERFACE: &'static str = "hyprland_toplevel_window_mapping_handle_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the handle. This request can be sent at any time by the client."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> hyprland_toplevel_window_mapping_handle_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The full 64bit window address. The `address` field contains the lower 32 bits whilst the"]
            #[doc = "`address_hi` contains the upper 32 bits"]
            async fn window_address(
                &self,
                address_hi: u32,
                address: u32,
            ) -> crate::client::Result<()>;
            #[doc = "The mapping of the toplevel to a window address failed. Most likely the window does not"]
            #[doc = "exist (anymore)."]
            async fn failed(&self) -> crate::client::Result<()>;
        }
    }
}
