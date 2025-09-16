#[allow(clippy::module_inception)]
pub mod treeland_dde_shell_v1 {
    #[doc = ""]
    #[doc = "This interface allows DDE change some treeland function."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dde_shell_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_dde_shell_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdeShellManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_dde_shell_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_window_overlap_checker({})",
                                sender_id,
                                id
                            );
                            self.get_window_overlap_checker(client, sender_id, id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_shell_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_shell_surface(client, sender_id, id, surface).await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_dde_active({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_treeland_dde_active(client, sender_id, id, seat)
                                .await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_multitaskview({})",
                                sender_id,
                                id
                            );
                            self.get_treeland_multitaskview(client, sender_id, id).await
                        }
                        4u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_window_picker({})",
                                sender_id,
                                id
                            );
                            self.get_treeland_window_picker(client, sender_id, id).await
                        }
                        5u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_dde_shell_manager_v1#{}.get_treeland_lockscreen({})",
                                sender_id,
                                id
                            );
                            self.get_treeland_lockscreen(client, sender_id, id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn get_window_overlap_checker(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Create a shell surface for an existing wl_surface."]
            #[doc = ""]
            #[doc = "Only one shell surface can be associated with a given surface."]
            #[doc = ""]
            #[doc = "Recommended for use with xdg_surface."]
            #[doc = ""]
            fn get_shell_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Create a new dde active for a given seat."]
            #[doc = ""]
            fn get_treeland_dde_active(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Create a new multitaskview context for toggle."]
            #[doc = ""]
            fn get_treeland_multitaskview(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Create a new window picker to pick window."]
            #[doc = ""]
            fn get_treeland_window_picker(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Create a new lockscreen context for toggle."]
            #[doc = ""]
            fn get_treeland_lockscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "A treeland_dde_shell_handle_v1 object represents an opened toplevel window. Each"]
    #[doc = "app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_window_overlap_checker {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [doc = ""] # [doc = "same layershell"] # [doc = ""] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Anchor : u32 { # [doc = "the top edge of the anchor rectangle"] const Top = 1u32 ; # [doc = "the bottom edge of the anchor rectangle"] const Bottom = 2u32 ; # [doc = "the left edge of the anchor rectangle"] const Left = 4u32 ; # [doc = "the right edge of the anchor rectangle"] const Right = 8u32 ; } }
        impl TryFrom<u32> for Anchor {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Anchor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_window_overlap_checker interface. See the module level documentation for more info"]
        pub trait TreelandWindowOverlapChecker: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_window_overlap_checker";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let width = message.int()?;
                            let height = message.int()?;
                            let anchor = message.uint()?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_window_overlap_checker#{}.update({}, {}, {}, {})",
                                sender_id,
                                width,
                                height,
                                anchor,
                                output
                            );
                            self.update(
                                client,
                                sender_id,
                                width,
                                height,
                                anchor.try_into()?,
                                output,
                            )
                            .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "treeland_window_overlap_checker#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This interface is used to receive the detected surface."]
            #[doc = "When the xdgshell window in the workspace overlaps with the detected window,"]
            #[doc = "an event will be sent to notify the client to process it."]
            #[doc = "The window position will only be recorded when this interface is called."]
            #[doc = "If the window moves, this interface needs to be called again."]
            #[doc = ""]
            fn update(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
                anchor: Anchor,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroys the treeland_window_overlap_checker object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is sent when windows overlapped."]
            #[doc = "This event is sent only once."]
            #[doc = ""]
            fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_window_overlap_checker#{}.enter()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is sent when windows not overlapped."]
            #[doc = "This event is sent only once."]
            #[doc = ""]
            fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_window_overlap_checker#{}.leave()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide the shell user interface."]
    #[doc = ""]
    #[doc = "It provides requests to set surface role, set skip, set the position"]
    #[doc = "set auto placement in output coordinates."]
    #[doc = ""]
    #[doc = "On the server side the object is automatically destroyed when"]
    #[doc = "the related wl_surface is destroyed.  On client side,"]
    #[doc = "treeland_dde_shell_surface_v1.destroy() must be called before"]
    #[doc = "destroying the wl_surface object."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dde_shell_surface_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "These values indicate which roles a surface can be rendered in, They"]
        #[doc = "are ordered by z depth."]
        #[doc = ""]
        #[doc = "Displayed below wlr-layer-shell, at the overlay level of the workspace."]
        #[doc = ""]
        #[doc = "Multiple surfaces can share a single role, and ordering within a single"]
        #[doc = "role is undefined."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Role {
            Overlay = 1u32,
        }
        impl TryFrom<u32> for Role {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Overlay),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Role {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_dde_shell_surface_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdeShellSurfaceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_dde_shell_surface_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_surface_position({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.set_surface_position(client, sender_id, x, y).await
                        }
                        2u16 => {
                            let role = message.uint()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_role({})",
                                sender_id,
                                role
                            );
                            self.set_role(client, sender_id, role.try_into()?).await
                        }
                        3u16 => {
                            let y_offset = message.uint()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_auto_placement({})",
                                sender_id,
                                y_offset
                            );
                            self.set_auto_placement(client, sender_id, y_offset).await
                        }
                        4u16 => {
                            let skip = message.uint()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_skip_switcher({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_switcher(client, sender_id, skip).await
                        }
                        5u16 => {
                            let skip = message.uint()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_skip_dock_preview({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_dock_preview(client, sender_id, skip).await
                        }
                        6u16 => {
                            let skip = message.uint()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_skip_muti_task_view({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_muti_task_view(client, sender_id, skip).await
                        }
                        7u16 => {
                            let accept = message.uint()?;
                            tracing::debug!(
                                "treeland_dde_shell_surface_v1#{}.set_accept_keyboard_focus({})",
                                sender_id,
                                accept
                            );
                            self.set_accept_keyboard_focus(client, sender_id, accept)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "The treeland_dde_shell_surface_v1 interface is removed from the"]
            #[doc = "wl_surface object that was turned into a shell surface with the"]
            #[doc = "treeland_shell_v1.get_treeland_dde_shell_surface request."]
            #[doc = ""]
            #[doc = "The shell surface role is lost and wl_surface is unmapped."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Move the surface to new coordinates."]
            #[doc = ""]
            #[doc = "Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output"]
            #[doc = "is 1970,50 in global coordinates space."]
            #[doc = ""]
            fn set_surface_position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Assign a role to a shell surface."]
            #[doc = ""]
            fn set_role(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                role : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_shell_surface_v1 :: Role,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Set the vertical alignment of the surface within the cursor width."]
            #[doc = ""]
            #[doc = "Do not use it together with set_surface_position to avoid exceptions."]
            #[doc = ""]
            #[doc = "The position of the surface will be controlled by the compositor after the"]
            #[doc = "request, including preventing it from being displayed beyond the edge of"]
            #[doc = "the output."]
            #[doc = ""]
            fn set_auto_placement(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                y_offset: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a switcher."]
            #[doc = ""]
            fn set_skip_switcher(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                skip: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a dock preview."]
            #[doc = ""]
            fn set_skip_dock_preview(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                skip: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a mutitask view."]
            #[doc = ""]
            fn set_skip_muti_task_view(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                skip: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Setting this will determine whether the surface can receive keyboard focus."]
            #[doc = "When set to 0, the surface will not receive keyboard focus even when clicked or activated."]
            #[doc = "When set to 1 (default), the surface will receive keyboard focus normally."]
            #[doc = ""]
            fn set_accept_keyboard_focus(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                accept: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "An interface used to monitor special events."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dde_active_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Reason {
            Mouse = 0u32,
            Wheel = 1u32,
        }
        impl TryFrom<u32> for Reason {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Mouse),
                    1u32 => Ok(Self::Wheel),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Reason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_dde_active_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDdeActiveV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_dde_active_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_dde_active_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn active_in(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_active_v1 :: Reason,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_dde_active_v1#{}.active_in({})",
                        sender_id,
                        reason
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(reason as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            fn active_out(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                reason : super :: super :: super :: treeland :: treeland_dde_shell_v1 :: treeland_dde_active_v1 :: Reason,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_dde_active_v1#{}.active_out({})",
                        sender_id,
                        reason
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(reason as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            fn start_drag(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_dde_active_v1#{}.start_drag()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            fn drop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_dde_active_v1#{}.drop()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "An interface used to control multitaskview."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_multitaskview_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_multitaskview_v1 interface. See the module level documentation for more info"]
        pub trait TreelandMultitaskviewV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_multitaskview_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_multitaskview_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            tracing::debug!("treeland_multitaskview_v1#{}.toggle()", sender_id,);
                            self.toggle(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Show or hide the multitaskview."]
            #[doc = ""]
            fn toggle(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "An interface used to pick window and return credentials."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_window_picker_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_window_picker_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWindowPickerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_window_picker_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_window_picker_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let hint = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_window_picker_v1#{}.pick(\"{}\")",
                                sender_id,
                                hint
                            );
                            self.pick(client, sender_id, hint).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Pick a window to get information."]
            #[doc = ""]
            fn pick(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                hint: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Picked window information."]
            #[doc = ""]
            fn window(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                pid: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_window_picker_v1#{}.window({})", sender_id, pid);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(pid).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "An interface used to operate lockscreen."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_lockscreen_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_lockscreen_v1 interface. See the module level documentation for more info"]
        pub trait TreelandLockscreenV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_lockscreen_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_lockscreen_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            tracing::debug!("treeland_lockscreen_v1#{}.lock()", sender_id,);
                            self.lock(client, sender_id).await
                        }
                        2u16 => {
                            tracing::debug!("treeland_lockscreen_v1#{}.shutdown()", sender_id,);
                            self.shutdown(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!("treeland_lockscreen_v1#{}.switch_user()", sender_id,);
                            self.switch_user(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Lock the screen."]
            #[doc = ""]
            fn lock(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Show shutdown."]
            #[doc = ""]
            fn shutdown(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Switch user."]
            #[doc = ""]
            fn switch_user(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_ddm {
    #[doc = ""]
    #[doc = "This object is primarily used for establish connection between"]
    #[doc = "treeland and ddm."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_ddm {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_ddm interface. See the module level documentation for more info"]
        pub trait TreelandDdm: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_ddm";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_ddm#{}.switch_to_greeter()", sender_id,);
                            self.switch_to_greeter(client, sender_id).await
                        }
                        1u16 => {
                            let username = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_ddm#{}.switch_to_user(\"{}\")",
                                sender_id,
                                username
                            );
                            self.switch_to_user(client, sender_id, username).await
                        }
                        2u16 => {
                            tracing::debug!("treeland_ddm#{}.activate_session()", sender_id,);
                            self.activate_session(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!("treeland_ddm#{}.deactivate_session()", sender_id,);
                            self.deactivate_session(client, sender_id).await
                        }
                        4u16 => {
                            tracing::debug!("treeland_ddm#{}.enable_render()", sender_id,);
                            self.enable_render(client, sender_id).await
                        }
                        5u16 => {
                            let callback = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_ddm#{}.disable_render({})",
                                sender_id,
                                callback
                            );
                            self.disable_render(client, sender_id, callback).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Send treeland to Greeter mode."]
            #[doc = ""]
            fn switch_to_greeter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Set lockscreen user to username. Ignore when username is \"ddm\"."]
            #[doc = ""]
            fn switch_to_user(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                username: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Activate treeland session. This will makes treeland try to take"]
            #[doc = "control of screen."]
            #[doc = ""]
            fn activate_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Deactivate treeland session. This will release control of the"]
            #[doc = "screen, but not to close the current seats."]
            #[doc = ""]
            fn deactivate_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Enable treeland rendering. This is primarily called after"]
            #[doc = "disable_render to resume treeland."]
            #[doc = ""]
            fn enable_render(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Disable treeland rendering. This will prevent treeland from"]
            #[doc = "output to DRM device."]
            #[doc = ""]
            fn disable_render(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                callback: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Call ddm to switch current virtual terminal to vtnr. ddm should"]
            #[doc = "take care of the switch and call ioctl respectively."]
            #[doc = ""]
            fn switch_to_vt(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                vtnr: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_ddm#{}.switch_to_vt({})", sender_id, vtnr);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(vtnr).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Call ddm to acquire control of VT at vtnr. ddm should call"]
            #[doc = "VT_SETMODE respectively."]
            #[doc = ""]
            fn acquire_vt(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                vtnr: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_ddm#{}.acquire_vt({})", sender_id, vtnr);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(vtnr).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_foreign_toplevel_manager_v1 {
    #[doc = ""]
    #[doc = "This interface allows a client to get toplevel some info."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_foreign_toplevel_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_foreign_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandForeignToplevelManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_foreign_toplevel_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_manager_v1#{}.stop()",
                                sender_id,
                            );
                            self.stop(client, sender_id).await
                        }
                        1u16 => {
                            let relative_surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_foreign_toplevel_manager_v1#{}.get_dock_preview_context({}, {})",
                                sender_id,
                                relative_surface,
                                id
                            );
                            self.get_dock_preview_context(client, sender_id, relative_surface, id)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Indicates the client no longer wishes to receive events for new toplevels."]
            #[doc = "However the compositor may emit further toplevel_created events, until"]
            #[doc = "the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            #[doc = ""]
            fn stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn get_dock_preview_context(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                relative_surface: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is emitted whenever a new toplevel window is created. It"]
            #[doc = "is emitted for all toplevels, regardless of the app that has created"]
            #[doc = "them."]
            #[doc = ""]
            #[doc = "All initial details of the toplevel(title, app_id, states, etc.) will"]
            #[doc = "be sent immediately after this event via the corresponding events in"]
            #[doc = "treeland_foreign_toplevel_handle_v1."]
            #[doc = ""]
            fn toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_manager_v1#{}.toplevel({})",
                        sender_id,
                        toplevel
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event indicates that the compositor is done sending events to the"]
            #[doc = "treeland_foreign_toplevel_manager_v1. The server will destroy the object"]
            #[doc = "immediately after sending this request, so it will become invalid and"]
            #[doc = "the client should free any resources associated with it."]
            #[doc = ""]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_manager_v1#{}.finished()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "A treeland_foreign_toplevel_handle_v1 object represents an opened toplevel window. Each"]
    #[doc = "app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_foreign_toplevel_handle_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "The different states that a toplevel can have. These have the same meaning"]
        #[doc = "as the states with the same names defined in xdg-toplevel"]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            #[doc = "the toplevel is maximized"]
            Maximized = 0u32,
            #[doc = "the toplevel is minimized"]
            Minimized = 1u32,
            #[doc = "the toplevel is active"]
            Activated = 2u32,
            #[doc = "the toplevel is fullscreen"]
            Fullscreen = 3u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Maximized),
                    1u32 => Ok(Self::Minimized),
                    2u32 => Ok(Self::Activated),
                    3u32 => Ok(Self::Fullscreen),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
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
        pub enum Error {
            #[doc = "the provided rectangle is invalid"]
            InvalidRectangle = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidRectangle),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait TreelandForeignToplevelHandleV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_foreign_toplevel_handle_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_maximized()",
                                sender_id,
                            );
                            self.set_maximized(client, sender_id).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.unset_maximized()",
                                sender_id,
                            );
                            self.unset_maximized(client, sender_id).await
                        }
                        2u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_minimized()",
                                sender_id,
                            );
                            self.set_minimized(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.unset_minimized()",
                                sender_id,
                            );
                            self.unset_minimized(client, sender_id).await
                        }
                        4u16 => {
                            let seat = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.activate({})",
                                sender_id,
                                seat
                            );
                            self.activate(client, sender_id, seat).await
                        }
                        5u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.close()",
                                sender_id,
                            );
                            self.close(client, sender_id).await
                        }
                        6u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_rectangle({}, {}, {}, {}, {})",
                                sender_id,
                                surface,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_rectangle(client, sender_id, surface, x, y, width, height)
                                .await
                        }
                        7u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        8u16 => {
                            let output = message.object()?;
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.set_fullscreen({})",
                                sender_id,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_fullscreen(client, sender_id, output).await
                        }
                        9u16 => {
                            tracing::debug!(
                                "treeland_foreign_toplevel_handle_v1#{}.unset_fullscreen()",
                                sender_id,
                            );
                            self.unset_fullscreen(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Requests that the toplevel be maximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            #[doc = ""]
            fn set_maximized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Requests that the toplevel be unmaximized. If the maximized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            #[doc = ""]
            fn unset_maximized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Requests that the toplevel be minimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            #[doc = ""]
            fn set_minimized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Requests that the toplevel be unminimized. If the minimized state actually"]
            #[doc = "changes, this will be indicated by the state event."]
            #[doc = ""]
            fn unset_minimized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request that this toplevel be activated on the given seat."]
            #[doc = "There is no guarantee the toplevel will be actually activated."]
            #[doc = ""]
            fn activate(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Send a request to the toplevel to close itself. The compositor would"]
            #[doc = "typically use a shell-specific method to carry out this request, for"]
            #[doc = "example by sending the xdg_toplevel.close event. However, this gives"]
            #[doc = "no guarantees the toplevel will actually be destroyed. If and when"]
            #[doc = "this happens, the treeland_foreign_toplevel_handle_v1.closed event will"]
            #[doc = "be emitted."]
            #[doc = ""]
            fn close(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The rectangle of the surface specified in this request corresponds to"]
            #[doc = "the place where the app using this protocol represents the given toplevel."]
            #[doc = "It can be used by the compositor as a hint for some operations, e.g"]
            #[doc = "minimizing. The client is however not required to set this, in which"]
            #[doc = "case the compositor is free to decide some default value."]
            #[doc = ""]
            #[doc = "If the client specifies more than one rectangle, only the last one is"]
            #[doc = "considered."]
            #[doc = ""]
            #[doc = "The dimensions are given in surface-local coordinates."]
            #[doc = "Setting width=height=0 removes the already-set rectangle."]
            #[doc = ""]
            fn set_rectangle(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroys the treeland_foreign_toplevel_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be called either when the client does not want to"]
            #[doc = "use the toplevel anymore or after the closed event to finalize the"]
            #[doc = "destruction of the object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Requests that the toplevel be fullscreened on the given output. If the"]
            #[doc = "fullscreen state and/or the outputs the toplevel is visible on actually"]
            #[doc = "change, this will be indicated by the state and output_enter/leave"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "The output parameter is only a hint to the compositor. Also, if output"]
            #[doc = "is NULL, the compositor should decide which output the toplevel will be"]
            #[doc = "fullscreened on, if at all."]
            #[doc = ""]
            fn set_fullscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Requests that the toplevel be unfullscreened. If the fullscreen state"]
            #[doc = "actually changes, this will be indicated by the state event."]
            #[doc = ""]
            fn unset_fullscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event will be sent when the compositor has set the process id this window"]
            #[doc = "belongs to. This should be set once before the initial_state is sent."]
            #[doc = ""]
            fn pid(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                pid: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.pid({})",
                        sender_id,
                        pid
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(pid).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            #[doc = ""]
            fn title(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                title: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.title(\"{}\")",
                        sender_id,
                        title
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(title))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the app-id of the toplevel changes."]
            #[doc = ""]
            fn app_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                app_id: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.app_id(\"{}\")",
                        sender_id,
                        app_id
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "The identifier of each top level and its handle must be unique."]
            #[doc = "Two different top layers cannot have the same identifier."]
            #[doc = "This identifier is only valid as long as the top level is mapped."]
            #[doc = "Identifiers must not be reused if the top level is not mapped."]
            #[doc = "The compositor must not reuse identifiers to ensure there are no races when"]
            #[doc = "identifiers are shared between processes."]
            #[doc = ""]
            fn identifier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                identifier: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.identifier({})",
                        sender_id,
                        identifier
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(identifier)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel becomes visible on"]
            #[doc = "the given output. A toplevel may be visible on multiple outputs."]
            #[doc = ""]
            fn output_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.output_enter({})",
                        sender_id,
                        output
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel stops being visible on"]
            #[doc = "the given output. It is guaranteed that an entered-output event"]
            #[doc = "with the same output has been emitted before this event."]
            #[doc = ""]
            fn output_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.output_leave({})",
                        sender_id,
                        output
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted immediately after the treeland_foreign_toplevel_handle_v1"]
            #[doc = "is created and each time the toplevel state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            #[doc = ""]
            fn state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.state(array[{}])",
                        sender_id,
                        state.len()
                    );
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_array(state).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is sent after all changes in the toplevel state have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "This allows changes to the treeland_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            #[doc = ""]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.done()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event means the toplevel has been destroyed. It is guaranteed there"]
            #[doc = "won't be any more events for this treeland_foreign_toplevel_handle_v1. The"]
            #[doc = "toplevel itself becomes inert so any requests will be ignored except the"]
            #[doc = "destroy request."]
            #[doc = ""]
            fn closed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.closed()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 8u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the parent of the toplevel changes."]
            #[doc = ""]
            #[doc = "No event is emitted when the parent handle is destroyed by the client."]
            #[doc = ""]
            fn parent(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                parent: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_foreign_toplevel_handle_v1#{}.parent({})",
                        sender_id,
                        parent
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_object(parent)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 9u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "This interface allows dock set windows preview."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_dock_preview_context_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Direction {
            #[doc = "top"]
            Top = 0u32,
            #[doc = "right"]
            Right = 1u32,
            #[doc = "bottom"]
            Bottom = 2u32,
            #[doc = "left"]
            Left = 3u32,
        }
        impl TryFrom<u32> for Direction {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Top),
                    1u32 => Ok(Self::Right),
                    2u32 => Ok(Self::Bottom),
                    3u32 => Ok(Self::Left),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Direction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_dock_preview_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandDockPreviewContextV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_dock_preview_context_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let surfaces = message.array()?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let direction = message.uint()?;
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.show(array[{}], {}, {}, {})",
                                sender_id,
                                surfaces.len(),
                                x,
                                y,
                                direction
                            );
                            self.show(client, sender_id, surfaces, x, y, direction)
                                .await
                        }
                        1u16 => {
                            let tooltip = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let direction = message.uint()?;
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.show_tooltip(\"{}\", {}, {}, {})",
                                sender_id,
                                tooltip,
                                x,
                                y,
                                direction
                            );
                            self.show_tooltip(client, sender_id, tooltip, x, y, direction)
                                .await
                        }
                        2u16 => {
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.close()",
                                sender_id,
                            );
                            self.close(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!(
                                "treeland_dock_preview_context_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "X and Y are relative to the relative_surface."]
            #[doc = "surfaces wl_array is identifiers."]
            #[doc = ""]
            fn show(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surfaces: Vec<u8>,
                x: i32,
                y: i32,
                direction: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn show_tooltip(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tooltip: String,
                x: i32,
                y: i32,
                direction: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "close preview box"]
            #[doc = ""]
            fn close(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroy the context object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is sent after mouse enter preview box."]
            #[doc = ""]
            fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_dock_preview_context_v1#{}.enter()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is sent after mouse leave preview box."]
            #[doc = ""]
            fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_dock_preview_context_v1#{}.leave()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_output_manager_v1 {
    #[doc = ""]
    #[doc = "Protocol for telling which is the primary display among the selection of enabled"]
    #[doc = "outputs."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_output_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandOutputManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_output_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let output = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_output_manager_v1#{}.set_primary_output(\"{}\")",
                                sender_id,
                                output
                            );
                            self.set_primary_output(client, sender_id, output).await
                        }
                        1u16 => {
                            tracing::debug!("treeland_output_manager_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn set_primary_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Specifies which output is the primary one identified by their name."]
            #[doc = ""]
            fn primary_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output_name: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_output_manager_v1#{}.primary_output(\"{}\")",
                        sender_id,
                        output_name
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(output_name))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_shortcut_manager_v1 {
    #[doc = ""]
    #[doc = "This interface allows a client to get some shell's info."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_shortcut_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_shortcut_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandShortcutManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_shortcut_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let key = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_shortcut_manager_v1#{}.register_shortcut_context(\"{}\", {})",
                                sender_id,
                                key,
                                id
                            );
                            self.register_shortcut_context(client, sender_id, key, id)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "The format of the shortcut key is 'Modify+Key', such as 'Ctrl+Alt+T'."]
            #[doc = "If the format is wrong, the synthesizer will give a \"format error\". If the shortcut"]
            #[doc = "key is already registered,"]
            #[doc = "the compositor will give a \"register error\" and issue a destruction to the context."]
            #[doc = ""]
            fn register_shortcut_context(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                key: String,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "This interface allows a client to listen a shortcut action."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_shortcut_context_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "shortcut register failed"]
            RegisterFailed = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::RegisterFailed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_shortcut_context_v1 interface. See the module level documentation for more info"]
        pub trait TreelandShortcutContextV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_shortcut_context_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_shortcut_context_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the context object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn shortcut(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> treeland_shortcut_context_v1#{}.shortcut()", sender_id,);
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
#[allow(clippy::module_inception)]
pub mod treeland_virtual_output_manager_v1 {
    #[doc = ""]
    #[doc = "This interface is a manager that allows the creation of copied output."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_virtual_output_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_virtual_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandVirtualOutputManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_virtual_output_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let outputs = message.array()?;
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.create_virtual_output({}, \"{}\", array[{}])",
                                sender_id,
                                id,
                                name,
                                outputs.len()
                            );
                            self.create_virtual_output(client, sender_id, id, name, outputs)
                                .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.get_virtual_output_list()",
                                sender_id,
                            );
                            self.get_virtual_output_list(client, sender_id).await
                        }
                        2u16 => {
                            let name = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_virtual_output_manager_v1#{}.get_virtual_output(\"{}\", {})",
                                sender_id,
                                name,
                                id
                            );
                            self.get_virtual_output(client, sender_id, name, id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Create virtual output that can be used when setting screen copy mode for use"]
            #[doc = "on multiple screens. Virtual outputs are created to mirror multiple wl_output"]
            #[doc = "outputs."]
            #[doc = ""]
            #[doc = "The element of the array is the name of the screen."]
            #[doc = ""]
            #[doc = "The first element of the array outputs is the screen to be copied, and"]
            #[doc = "the subsequent elements are the screens to be mirrored."]
            #[doc = ""]
            #[doc = "The client calling this interface will not generate an additional wl_output"]
            #[doc = "object on the client."]
            #[doc = ""]
            fn create_virtual_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                name: String,
                outputs: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Gets a list of virtual output names."]
            #[doc = ""]
            fn get_virtual_output_list(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The client obtains the corresponding virtual_output_v1 object"]
            #[doc = "through the virtual output name."]
            #[doc = ""]
            fn get_virtual_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sends a list of virtual output names to the client."]
            #[doc = ""]
            fn virtual_output_list(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                names: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_virtual_output_manager_v1#{}.virtual_output_list(array[{}])",
                        sender_id,
                        names.len()
                    );
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_array(names).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "A treeland_virtual_output_v1 represents a set virtual screen output object."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_virtual_output_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Group name is empty"]
            InvalidGroupName = 0u32,
            #[doc = "The number of screens applying for copy mode is less than 2"]
            InvalidScreenNumber = 1u32,
            #[doc = "Output does not exist"]
            InvalidOutput = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidGroupName),
                    1u32 => Ok(Self::InvalidScreenNumber),
                    2u32 => Ok(Self::InvalidOutput),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_virtual_output_v1 interface. See the module level documentation for more info"]
        pub trait TreelandVirtualOutputV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_virtual_output_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("treeland_virtual_output_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the output."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is sent to the client when any screen in the array changes."]
            #[doc = ""]
            #[doc = "The element of the array is the name of the screen."]
            #[doc = ""]
            #[doc = "The first element of the array outputs is the screen to be copied, and"]
            #[doc = "the subsequent elements are the screens to be mirrored."]
            #[doc = ""]
            #[doc = "When the primary screen (the screen being copied) is removed, a successor"]
            #[doc = "is selected from the queue as the primary screen, and the queue information"]
            #[doc = "is updated."]
            #[doc = ""]
            fn outputs(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
                outputs: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_virtual_output_v1#{}.outputs(\"{}\", array[{}])",
                        sender_id,
                        name,
                        outputs.len()
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_array(outputs)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "When an error occurs, an error event is emitted, terminating the replication"]
            #[doc = "mode request issued by the client."]
            #[doc = ""]
            fn error(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                code: u32,
                message: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_virtual_output_v1#{}.error({}, \"{}\")",
                        sender_id,
                        code,
                        message
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(code)
                        .put_string(Some(message))
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
#[allow(clippy::module_inception)]
pub mod treeland_wallpaper_color_v1 {
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_wallpaper_color_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the treeland_wallpaper_color_manager_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWallpaperColorManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_wallpaper_color_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let output = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.watch(\"{}\")",
                                sender_id,
                                output
                            );
                            self.watch(client, sender_id, output).await
                        }
                        1u16 => {
                            let output = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.unwatch(\"{}\")",
                                sender_id,
                                output
                            );
                            self.unwatch(client, sender_id, output).await
                        }
                        2u16 => {
                            tracing::debug!(
                                "treeland_wallpaper_color_manager_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Monitor the wallpaper color of a given screen."]
            #[doc = ""]
            fn watch(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Stop monitor the wallpaper color for the given screen."]
            #[doc = ""]
            fn unwatch(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The client no longer cares about wallpaper_color."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Tell the client that the wallpaper color of the screen it is monitoring has changed."]
            #[doc = "This event will also be sent immediately when the client requests a watch."]
            #[doc = ""]
            fn output_color(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: String,
                isdark: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_wallpaper_color_manager_v1#{}.output_color(\"{}\", {})",
                        sender_id,
                        output,
                        isdark
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(output))
                        .put_uint(isdark)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod treeland_window_management_v1 {
    #[doc = ""]
    #[doc = "This interface manages application windows."]
    #[doc = "It provides requests to show and hide the desktop and emits"]
    #[doc = "an event every time a window is created so that the client can"]
    #[doc = "use it to manage the window."]
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod treeland_window_management_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum DesktopState {
            Normal = 0u32,
            Show = 1u32,
            PreviewShow = 2u32,
        }
        impl TryFrom<u32> for DesktopState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::Show),
                    2u32 => Ok(Self::PreviewShow),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DesktopState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the treeland_window_management_v1 interface. See the module level documentation for more info"]
        pub trait TreelandWindowManagementV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "treeland_window_management_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let state = message.uint()?;
                            tracing::debug!(
                                "treeland_window_management_v1#{}.set_desktop({})",
                                sender_id,
                                state
                            );
                            self.set_desktop(client, sender_id, state).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "treeland_window_management_v1#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Tell the compositor to show/hide the desktop."]
            #[doc = ""]
            fn set_desktop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event will be sent whenever the show desktop mode changes. E.g. when it is"]
            #[doc = "entered"]
            #[doc = "or left."]
            #[doc = ""]
            #[doc = "On binding the interface the current state is sent."]
            #[doc = ""]
            fn show_desktop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> treeland_window_management_v1#{}.show_desktop({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(state).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
