#[doc = ""]
#[doc = "This protocols provides way to toggle various accessibility features"]
#[doc = "in the COSMIC desktop environment for shell components."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod cosmic_a11y_v1 {
    #[doc = ""]
    #[doc = "Manager to toggle accessibility features."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod cosmic_a11y_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ActiveState {
            #[doc = "function is disabled"]
            Disabled = 0u32,
            #[doc = "function is enabled"]
            Enabled = 1u32,
        }
        impl TryFrom<u32> for ActiveState {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ActiveState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Filter {
            #[doc = "No screen filter is set"]
            Disabled = 0u32,
            #[doc = "A custom or unknown screen filter"]
            Unknown = 1u32,
            #[doc = "Greyscale colors"]
            Greyscale = 2u32,
            #[doc = "Daltonize for Protanopia"]
            DaltonizeProtanopia = 3u32,
            #[doc = "Daltonize for Deuteranopia"]
            DaltonizeDeuteranopia = 4u32,
            #[doc = "Daltonize for Tritanopia"]
            DaltonizeTritanopia = 5u32,
        }
        impl TryFrom<u32> for Filter {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Unknown),
                    2u32 => Ok(Self::Greyscale),
                    3u32 => Ok(Self::DaltonizeProtanopia),
                    4u32 => Ok(Self::DaltonizeDeuteranopia),
                    5u32 => Ok(Self::DaltonizeTritanopia),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Filter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "A deprecated request or value was used"]
            Deprecated = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Deprecated),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the cosmic_a11y_manager_v1 interface. See the module level documentation for more info"]
        pub trait CosmicA11yManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "cosmic_a11y_manager_v1";
            const VERSION: u32 = 3u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let active = message.uint()?;
                            tracing::debug!(
                                "cosmic_a11y_manager_v1#{}.set_magnifier({})",
                                sender_id,
                                active
                            );
                            self.set_magnifier(client, sender_id, active.try_into()?)
                                .await
                        }
                        1u16 => {
                            let inverted = message.uint()?;
                            let filter = message.uint()?;
                            tracing::debug!(
                                "cosmic_a11y_manager_v1#{}.set_screen_filter({}, {})",
                                sender_id,
                                inverted,
                                filter
                            );
                            self.set_screen_filter(
                                client,
                                sender_id,
                                inverted.try_into()?,
                                filter.try_into()?,
                            )
                            .await
                        }
                        2u16 => {
                            let inverted = message.uint()?;
                            let filter = message.uint()?;
                            let filter_state = message.uint()?;
                            tracing::debug!(
                                "cosmic_a11y_manager_v1#{}.set_screen_filter2({}, {}, {})",
                                sender_id,
                                inverted,
                                filter,
                                filter_state
                            );
                            self.set_screen_filter2(
                                client,
                                sender_id,
                                inverted.try_into()?,
                                filter.try_into()?,
                                filter_state.try_into()?,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Sets the state of the screen magnifier."]
            #[doc = ""]
            #[doc = "The client must not assume any requested changes are actually applied and should wait"]
            #[doc = "until the next magnifier event before updating it's UI."]
            #[doc = ""]
            fn set_magnifier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                active: ActiveState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Set the parameters for screen filtering."]
            #[doc = ""]
            #[doc = "If the filter is set to unknown, the compositor MUST not change the current state"]
            #[doc = "of the filter. This is to allow clients to update the inverted state, even if they"]
            #[doc = "don't know about the current active filter."]
            #[doc = ""]
            #[doc = "The client must not assume any requested changes are actually applied and should wait"]
            #[doc = "until the next screen_filter event before updating it's UI."]
            #[doc = ""]
            #[doc = "Send this request will raised a \"deprecated\" protocol error, if version 3 or higher was bound."]
            #[doc = "Use `set_screen_filter2` instead."]
            #[doc = ""]
            fn set_screen_filter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                inverted: ActiveState,
                filter: Filter,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Set the parameters for screen filtering."]
            #[doc = ""]
            #[doc = "If the filter is set to unknown, the compositor MUST not change the currently set"]
            #[doc = "filter. This is to allow clients to update the inverted state or toggle the screen filter,"]
            #[doc = "even if they don't know about the currently selected filter."]
            #[doc = ""]
            #[doc = "The client must not assume any requested changes are actually applied and should wait"]
            #[doc = "until the next screen_filter event before updating it's UI."]
            #[doc = ""]
            #[doc = "The \"deprecated\" protocol error is raised, if \"disabled\" is set for \"filter\"."]
            #[doc = ""]
            fn set_screen_filter2(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                inverted: ActiveState,
                filter: Filter,
                filter_state: ActiveState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "State of the screen magnifier."]
            #[doc = ""]
            #[doc = "This event will be emitted by the compositor when binding the protocol"]
            #[doc = "and whenever the state changes."]
            #[doc = ""]
            fn magnifier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                active: ActiveState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> cosmic_a11y_manager_v1#{}.magnifier({})",
                        sender_id,
                        active
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_uint(active as u32).build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Parameters used for screen filtering."]
            #[doc = ""]
            #[doc = "This event will be emitted by the compositor when binding the protocol"]
            #[doc = "and whenever the state changes."]
            #[doc = ""]
            #[doc = "If a screen filter is used not known to the protocol or the bound version"]
            #[doc = "filter will be set to unknown."]
            #[doc = ""]
            #[doc = "Since version 3 this event will not be emitted anymore, instead use `screen_filter2`."]
            #[doc = ""]
            fn screen_filter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                inverted: ActiveState,
                filter: Filter,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> cosmic_a11y_manager_v1#{}.screen_filter({}, {})",
                        sender_id,
                        inverted,
                        filter
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(inverted as u32)
                        .put_uint(filter as u32)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Parameters used for screen filtering."]
            #[doc = ""]
            #[doc = "This event will be emitted by the compositor when binding the protocol"]
            #[doc = "and whenever the state changes."]
            #[doc = ""]
            #[doc = "If a screen filter is used not known to the protocol or the bound version"]
            #[doc = "filter will be set to unknown."]
            #[doc = ""]
            #[doc = "The compositor must never send \"disabled\" as the \"filter\" argument."]
            #[doc = ""]
            fn screen_filter2(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                inverted: ActiveState,
                filter: Filter,
                filter_state: ActiveState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> cosmic_a11y_manager_v1#{}.screen_filter2({}, {}, {})",
                        sender_id,
                        inverted,
                        filter,
                        filter_state
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(inverted as u32)
                        .put_uint(filter as u32)
                        .put_uint(filter_state as u32)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
        }
    }
}
#[doc = ""]
#[doc = "This protocol provides a relatively straightforward mapping of AtpsiDevice"]
#[doc = "in the at-spi2-core library, so it's possible to add a Wayland backend for it."]
#[doc = ""]
#[doc = "This provides a way for screen reader key bindings to work."]
#[doc = ""]
#[doc = "This is a temporary solution until a better protocol is available for this purpose."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod cosmic_atspi_v1 {
    #[doc = ""]
    #[doc = "Manager for adding grabs and monitoring key input."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod cosmic_atspi_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the cosmic_atspi_manager_v1 interface. See the module level documentation for more info"]
        pub trait CosmicAtspiManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "cosmic_atspi_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("cosmic_atspi_manager_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let mods = message.uint()?;
                            let virtual_mods = message.array()?;
                            let key = message.uint()?;
                            tracing::debug!(
                                "cosmic_atspi_manager_v1#{}.add_key_grab({}, array[{}], {})",
                                sender_id,
                                mods,
                                virtual_mods.len(),
                                key
                            );
                            self.add_key_grab(client, sender_id, mods, virtual_mods, key)
                                .await
                        }
                        2u16 => {
                            let mods = message.uint()?;
                            let virtual_mods = message.array()?;
                            let key = message.uint()?;
                            tracing::debug!(
                                "cosmic_atspi_manager_v1#{}.remove_key_grab({}, array[{}], {})",
                                sender_id,
                                mods,
                                virtual_mods.len(),
                                key
                            );
                            self.remove_key_grab(client, sender_id, mods, virtual_mods, key)
                                .await
                        }
                        3u16 => {
                            tracing::debug!(
                                "cosmic_atspi_manager_v1#{}.grab_keyboard()",
                                sender_id,
                            );
                            self.grab_keyboard(client, sender_id).await
                        }
                        4u16 => {
                            tracing::debug!(
                                "cosmic_atspi_manager_v1#{}.ungrab_keyboard()",
                                sender_id,
                            );
                            self.ungrab_keyboard(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Any grabs that are still active will be disabled."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Grab the given key combination, so it will not be sent to clients."]
            #[doc = ""]
            fn add_key_grab(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                mods: u32,
                virtual_mods: Vec<u8>,
                key: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Disables a grab added with add_key_grab."]
            #[doc = ""]
            fn remove_key_grab(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                mods: u32,
                virtual_mods: Vec<u8>,
                key: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Grab keyboard, so key input will not be sent to clients."]
            #[doc = ""]
            fn grab_keyboard(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Disables a grab added with grab_keyboard."]
            #[doc = ""]
            fn ungrab_keyboard(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Produces an fd that can be used with libei to monitor keyboard input."]
            #[doc = ""]
            fn key_events_eis(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> cosmic_atspi_manager_v1#{}.key_events_eis({})",
                        sender_id,
                        fd.as_raw_fd()
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_fd(fd).build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
        }
    }
}
#[doc = ""]
#[doc = "This protocols `extends ext-image-capture-source-v1` with additional capture"]
#[doc = "sources."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod cosmic_image_source_unstable_v1 {
    #[doc = ""]
    #[doc = "A manager for creating image source objects for wl_output objects."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_workspace_image_capture_source_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the zcosmic_workspace_image_capture_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicWorkspaceImageCaptureSourceManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_workspace_image_capture_source_manager_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let source = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_workspace_image_capture_source_manager_v1#{}.create_source({}, {})",
                                sender_id,
                                source,
                                output
                            );
                            self.create_source(client, sender_id, source, output).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zcosmic_workspace_image_capture_source_manager_v1#{}.destroy()",
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
            #[doc = "Creates a source object for a workspaces. Images captured from this source"]
            #[doc = "will show the same content as the workspace. Some elements may be omitted,"]
            #[doc = "such as cursors and overlays that have been marked as transparent to"]
            #[doc = "capturing."]
            #[doc = ""]
            fn create_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                source: crate::ObjectId,
                output: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[doc = ""]
#[doc = "This protocol serves as an extension to wlr-output-management."]
#[doc = ""]
#[doc = "It primarily adds explicit output mirroring,"]
#[doc = "while upstream is figuring out how to best support that."]
#[doc = ""]
#[doc = "It was designed against version 4 of wlr-output-management, but tries"]
#[doc = "it's best to be forward compatible."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod cosmic_output_management_unstable_v1 {
    #[doc = ""]
    #[doc = "This interface provides extension points for wlr-output-management types."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "object already created"]
            AlreadyExtended = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyExtended),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_output_manager_v1";
            const VERSION: u32 = 3u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let extended = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let head = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_output_manager_v1#{}.get_head({}, {})",
                                sender_id,
                                extended,
                                head
                            );
                            self.get_head(client, sender_id, extended, head).await
                        }
                        1u16 => {
                            let extended = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let config = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_output_manager_v1#{}.get_configuration({}, {})",
                                sender_id,
                                extended,
                                config
                            );
                            self.get_configuration(client, sender_id, extended, config)
                                .await
                        }
                        2u16 => {
                            let extended = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let config_head = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_output_manager_v1#{}.get_configuration_head({}, {})",
                                sender_id,
                                extended,
                                config_head
                            );
                            self.get_configuration_head(client, sender_id, extended, config_head)
                                .await
                        }
                        3u16 => {
                            tracing::debug!("zcosmic_output_manager_v1#{}.release()", sender_id,);
                            let result = self.release(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        4u16 => {
                            let head = message.object()?;
                            tracing::debug!(
                                "zcosmic_output_manager_v1#{}.set_xwayland_primary({})",
                                sender_id,
                                head.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_xwayland_primary(client, sender_id, head).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Gets an extension object for zwlr_output_head_v1."]
            #[doc = ""]
            #[doc = "As soon as the extended output is created, events will be dispatched with an accompanying"]
            #[doc = "`done`-event delivered to the matching `zwlr_output_manager_v1` afterwards."]
            #[doc = ""]
            #[doc = "Any further updates will produce new events, if properties of the zcosmic_output_head_v1 change,"]
            #[doc = "just like for the original `zwlr_output_head_v1`. Events should be handled as atomic, as denoted"]
            #[doc = "by `zwlr_output_manager_v1::done`."]
            #[doc = ""]
            #[doc = "Trying to create more than one zcosmic_output_head_v1 per zwlr_output_head_v1 will raise an"]
            #[doc = "\"already_extended\" error."]
            #[doc = ""]
            fn get_head(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                extended: crate::ObjectId,
                head: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Gets an extension object for zwlr_output_configuration_v1."]
            #[doc = ""]
            #[doc = "Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1"]
            #[doc = "will raise an \"already_extended\" error."]
            #[doc = ""]
            fn get_configuration(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                extended: crate::ObjectId,
                config: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Gets an extension object for zwlr_output_configuration_head_v1."]
            #[doc = ""]
            #[doc = "Trying to create more than one zcosmic_output_configuration_head_v1 per"]
            #[doc = "zwlr_output_configuration_head_v1 will raise an \"already_extended\" error."]
            #[doc = ""]
            fn get_configuration_head(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                extended: crate::ObjectId,
                config_head: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroys this global. All previously created objects remain valid."]
            #[doc = ""]
            fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This requests a head to be advertised as the primary output via randr to Xwayland."]
            #[doc = ""]
            #[doc = "No head has to be marked primary, if `null` is passed Xwayland won't advertise a primary output."]
            #[doc = "Sending a disabled head will be ignored to avoid races."]
            #[doc = ""]
            fn set_xwayland_primary(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                head: Option<crate::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "Extension to zwlr_output_head_v1."]
    #[doc = ""]
    #[doc = "Adds additional read-only properties."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the wlr_output_manager.done event."]
    #[doc = "No guarantees are made regarding the order in which properties are sent."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_head_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AdaptiveSyncAvailability {
            #[doc = "adaptive sync is not supported"]
            Unsupported = 0u32,
            #[doc = "automatic adaptive_sync is unavailable"]
            RequiresModeset = 1u32,
            #[doc = "adaptive sync is supported in all states"]
            Supported = 2u32,
        }
        impl TryFrom<u32> for AdaptiveSyncAvailability {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unsupported),
                    1u32 => Ok(Self::RequiresModeset),
                    2u32 => Ok(Self::Supported),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AdaptiveSyncAvailability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AdaptiveSyncStateExt {
            #[doc = "adaptive sync is disabled"]
            Disabled = 0u32,
            #[doc = "adaptive sync will be actived automatically"]
            Automatic = 1u32,
            #[doc = "adaptive sync is forced to be always active"]
            Always = 2u32,
        }
        impl TryFrom<u32> for AdaptiveSyncStateExt {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Automatic),
                    2u32 => Ok(Self::Always),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AdaptiveSyncStateExt {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_output_head_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputHeadV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_output_head_v1";
            const VERSION: u32 = 3u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zcosmic_output_head_v1#{}.release()", sender_id,);
                            let result = self.release(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Using this request a client can tell the compositor that it is not interested"]
            #[doc = "in the head object anymore."]
            #[doc = ""]
            fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This events describes the scale of the head in the global compositor"]
            #[doc = "space multiplied by 1000 for additional precision."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            #[doc = ""]
            fn scale_1000(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                scale_1000: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_output_head_v1#{}.scale_1000({})",
                        sender_id,
                        scale_1000
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_int(scale_1000).build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This events describes that the head is mirroring another."]
            #[doc = "In these cases `name` contains the unique name of the matching `zwlr_output_head_v1`."]
            #[doc = "If the name is null, no head is being mirrored onto this one."]
            #[doc = ""]
            #[doc = "For mirrored heads the `position`-event is meaningless."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            #[doc = ""]
            fn mirroring(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                name: Option<String>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_output_head_v1#{}.mirroring(\"{}\")",
                        sender_id,
                        name.as_ref().map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_string(name).build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This events describes if adaptive_sync is available for this head."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            #[doc = ""]
            fn adaptive_sync_available(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                available: AdaptiveSyncAvailability,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_output_head_v1#{}.adaptive_sync_available({})",
                        sender_id,
                        available
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(available as u32)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This events describes the adaptive_sync state of this head."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            #[doc = ""]
            fn adaptive_sync_ext(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state: AdaptiveSyncStateExt,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_output_head_v1#{}.adaptive_sync_ext({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_uint(state as u32).build();
                    client
                        .send_message(crate::Message::new(sender_id, 3u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event describes if this head is advertised as the primary output via randr to Xwayland."]
            #[doc = ""]
            #[doc = "At most one output is marked primary, but it is not guaranteed that any output is marked."]
            #[doc = "It is only sent if the output is enabled."]
            #[doc = ""]
            fn xwayland_primary(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_output_head_v1#{}.xwayland_primary({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_uint(state).build();
                    client
                        .send_message(crate::Message::new(sender_id, 4u16, payload, fds))
                        .await
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "Extension to zwlr_output_configuration_v1."]
    #[doc = ""]
    #[doc = "Adds additional parameters to be tested/applyed via the original zwlr_output_configuration_v1."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_configuration_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "underlying configuration has already been used"]
            AlreadyFinished = 1u32,
            #[doc = "mirrored head is not enabled"]
            MirroredHeadBusy = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyFinished),
                    2u32 => Ok(Self::MirroredHeadBusy),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_output_configuration_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputConfigurationV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_output_configuration_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let head = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let mirroring = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_output_configuration_v1#{}.mirror_head({}, {}, {})",
                                sender_id,
                                id,
                                head,
                                mirroring
                            );
                            self.mirror_head(client, sender_id, id, head, mirroring)
                                .await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zcosmic_output_configuration_v1#{}.release()",
                                sender_id,
                            );
                            let result = self.release(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Enable a head mirroring another."]
            #[doc = ""]
            #[doc = "This request creates a head configuration object that can be used to change the head's properties."]
            #[doc = ""]
            #[doc = "This is an alternative to `zwlr_output_configuration_v1::enable_head` or `zwlr_output_configuration_v1::disable_head`"]
            #[doc = "Using either with the same `head` argument will result in an `already_configured_head` error on the original"]
            #[doc = "`zwlr_output_configuration_v1` object."]
            #[doc = ""]
            #[doc = "All properties are still required to be set to the resulting `zwlr_output_configuration_head` by the client"]
            #[doc = "as denoted in the original protocol. Some like `set_position` however might be ignored in mirroring configurations."]
            #[doc = ""]
            #[doc = "Trying to set a disabled or mirroring head as `mirroring` or calling `disable_head`/`mirror_head` after using a head"]
            #[doc = "as a `mirroring` argument will raise a `mirrored_head_busy` protocol error."]
            #[doc = ""]
            fn mirror_head(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                id: crate::ObjectId,
                head: crate::ObjectId,
                mirroring: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Using this request a client can tell the compositor that it is not going"]
            #[doc = "to use the configuration object anymore. Any changes to the outputs"]
            #[doc = "will still be attached to the original `zwlr_output_configuration_head_v1`"]
            #[doc = "if it isn't destroyed."]
            #[doc = ""]
            fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event indicates that the configuration is no longer available."]
            #[doc = ""]
            #[doc = "This usually happens when the original configuration was `cancelled`, `suceeded` or `failed`."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            #[doc = ""]
            #[doc = "The configration object becomes inert and any requests other than `destroy` will be ignored."]
            #[doc = ""]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_output_configuration_v1#{}.finished()",
                        sender_id,
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "Extension to zwlr_output_configuration_head_v1."]
    #[doc = ""]
    #[doc = "Adds additional/alternative parameters to the original zwlr_output_configuration_head_v1."]
    #[doc = ""]
    #[doc = "Once the original `zwlr_output_configuration_head_v1` is destroyed this object will"]
    #[doc = "become inert and all requests except `release` will be ignored."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_configuration_head_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the zcosmic_output_configuration_head_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputConfigurationHeadV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_output_configuration_head_v1";
            const VERSION: u32 = 2u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let scale_1000 = message.int()?;
                            tracing::debug!(
                                "zcosmic_output_configuration_head_v1#{}.set_scale_1000({})",
                                sender_id,
                                scale_1000
                            );
                            self.set_scale_1000(client, sender_id, scale_1000).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zcosmic_output_configuration_head_v1#{}.release()",
                                sender_id,
                            );
                            let result = self.release(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        2u16 => {
                            let state = message.uint()?;
                            tracing::debug!(
                                "zcosmic_output_configuration_head_v1#{}.set_adaptive_sync_ext({})",
                                sender_id,
                                state
                            );
                            self.set_adaptive_sync_ext(client, sender_id, state.try_into()?)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request sets the head's scale multiplied by 1000 for additional precision."]
            #[doc = ""]
            #[doc = "This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_scale`."]
            #[doc = "Using `set_scale` and `set_scale_1000` at once will thus raise an `already_set` error on the"]
            #[doc = "original `zwlr_output_configuration_head_v1`."]
            #[doc = ""]
            #[doc = "Any request conflicting with `set_scale` will also conflict with `set_scale_1000`."]
            #[doc = ""]
            fn set_scale_1000(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                scale_1000: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Using this request a client can tell the compositor that it is not going"]
            #[doc = "to use the configuration object anymore. Already issued requests will"]
            #[doc = "still be attached to the original `zwlr_output_configuration_head_v1`"]
            #[doc = "until it is destroyed."]
            #[doc = ""]
            fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This request requests a new adaptive sync state."]
            #[doc = ""]
            #[doc = "This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_adaptive_sync`."]
            #[doc = "Using `set_adaptive_sync` and `set_adaptive_sync_ext` at once will thus raise an `already_set` error on the"]
            #[doc = "original `zwlr_output_configuration_head_v1`."]
            #[doc = ""]
            #[doc = "Any request conflicting with `set_adaptive_sync` will also conflict with `set_adaptive_sync_ext`."]
            #[doc = ""]
            fn set_adaptive_sync_ext(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state : super :: super :: super :: cosmic :: cosmic_output_management_unstable_v1 :: zcosmic_output_head_v1 :: AdaptiveSyncStateExt,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_overlap_notify_unstable_v1 {
    #[doc = ""]
    #[doc = "The purpose of this protocol is to enable layer-shell client to get"]
    #[doc = "notifications if part of their surfaces are occluded other elements"]
    #[doc = "(currently toplevels and other layer-surfaces)."]
    #[doc = ""]
    #[doc = "You can request a notification object for any of your zwlr_layer_surface_v1"]
    #[doc = "surfaces, which will then emit overlap events."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_overlap_notify_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the zcosmic_overlap_notify_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOverlapNotifyV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_overlap_notify_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let overlap_notification = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let layer_surface = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_overlap_notify_v1#{}.notify_on_overlap({}, {})",
                                sender_id,
                                overlap_notification,
                                layer_surface
                            );
                            self.notify_on_overlap(
                                client,
                                sender_id,
                                overlap_notification,
                                layer_surface,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Requests notifications for toplevels and layer-surfaces entering and leaving the"]
            #[doc = "surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to"]
            #[doc = "implement auto-hide functionality."]
            #[doc = ""]
            #[doc = "To stop receiving notifications, destroy the returned"]
            #[doc = "zcosmic_overlap_notification_v1 object."]
            #[doc = ""]
            fn notify_on_overlap(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                overlap_notification: crate::ObjectId,
                layer_surface: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_overlap_notification_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the zcosmic_overlap_notification_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOverlapNotificationV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_overlap_notification_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "zcosmic_overlap_notification_v1#{}.destroy()",
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
            #[doc = "This request should be called when the client has no interest in overlap"]
            #[doc = "notifications anymore."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "A ext_foreign_toplevel_handle_v1 has entered the surface area."]
            #[doc = ""]
            #[doc = "This event will be emitted once for every ext_foreign_toplevel_handle_v1"]
            #[doc = "representing this toplevel."]
            #[doc = ""]
            #[doc = "Compositors are free to update the overlapping area by sending additional"]
            #[doc = "`toplevel_enter` events for the same toplevel without sending `toplevel_leave`"]
            #[doc = "in between."]
            #[doc = ""]
            fn toplevel_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_overlap_notification_v1#{}.toplevel_enter({}, {}, {}, {}, {})",
                        sender_id,
                        toplevel,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "A ext_foreign_toplevel_handle_v1 has left the surface area."]
            #[doc = ""]
            #[doc = "This event will be emitted once for every ext_foreign_toplevel_handle_v1"]
            #[doc = "representing this toplevel."]
            #[doc = ""]
            fn toplevel_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_overlap_notification_v1#{}.toplevel_leave({})",
                        sender_id,
                        toplevel
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "A zwlr_layer_surface_v1 has entered the surface area."]
            #[doc = ""]
            #[doc = "Compositors are free to update the overlapping area by sending additional"]
            #[doc = "`layer_enter` events for the same surface without sending `layer_leave`"]
            #[doc = "in between."]
            #[doc = ""]
            #[doc = "The overlapping region is given surface-relative to the zwlr_layer_surface_v1"]
            #[doc = "used to create this notification object."]
            #[doc = ""]
            fn layer_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                identifier: String,
                namespace: String,
                exclusive: u32,
                layer : super :: super :: super :: wlr :: wlr_layer_shell_unstable_v1 :: zwlr_layer_shell_v1 :: Layer,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_overlap_notification_v1#{}.layer_enter(\"{}\", \"{}\", {}, {}, {}, {}, {}, {})",
                        sender_id,
                        identifier,
                        namespace,
                        exclusive,
                        layer,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_string(Some(identifier))
                        .put_string(Some(namespace))
                        .put_uint(exclusive)
                        .put_uint(layer as u32)
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "A zwlr_layer_surface_v1 has left the surface area."]
            #[doc = ""]
            fn layer_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                identifier: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_overlap_notification_v1#{}.layer_leave(\"{}\")",
                        sender_id,
                        identifier
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_string(Some(identifier))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 3u16, payload, fds))
                        .await
                }
            }
        }
    }
}
#[doc = ""]
#[doc = "This protocol allows clients to ask the compositor to capture screen"]
#[doc = "contents to user submitted buffers."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod cosmic_screencopy_unstable_v2 {
    #[doc = ""]
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_manager_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid option flag"]
            InvalidOption = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidOption),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Options : u32 { # [doc = "paint cursors onto captured frames"] const PaintCursors = 1u32 ; } }
        impl TryFrom<u32> for Options {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Options {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_screencopy_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopyManagerV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_screencopy_manager_v2";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let session = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let source = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let options = message.uint()?;
                            tracing::debug!(
                                "zcosmic_screencopy_manager_v2#{}.create_session({}, {}, {})",
                                sender_id,
                                session,
                                source,
                                options
                            );
                            self.create_session(
                                client,
                                sender_id,
                                session,
                                source,
                                options.try_into()?,
                            )
                            .await
                        }
                        1u16 => {
                            let session = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let source = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let pointer = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let options = message.uint()?;
                            tracing::debug!(
                                "zcosmic_screencopy_manager_v2#{}.create_pointer_cursor_session({}, {}, {}, {})",
                                sender_id,
                                session,
                                source,
                                pointer,
                                options
                            );
                            self.create_pointer_cursor_session(
                                client, sender_id, session, source, pointer, options,
                            )
                            .await
                        }
                        2u16 => {
                            tracing::debug!(
                                "zcosmic_screencopy_manager_v2#{}.destroy()",
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
            #[doc = "Create a capturing session for an image source."]
            #[doc = ""]
            #[doc = "If the paint_cursors option is set, cursors shall be composited onto"]
            #[doc = "the captured frame. The cursor shall not be composited onto the frame"]
            #[doc = "if this flag is not set."]
            #[doc = ""]
            fn create_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                session: crate::ObjectId,
                source: crate::ObjectId,
                options: Options,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Create a cursor capturing session for the pointer of an image source."]
            #[doc = ""]
            #[doc = "The options argument has no effect and must be set to 0. This is"]
            #[doc = "intended for any future flags that might be added."]
            #[doc = ""]
            fn create_pointer_cursor_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                session: crate::ObjectId,
                source: crate::ObjectId,
                pointer: crate::ObjectId,
                options: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroy the manager object."]
            #[doc = ""]
            #[doc = "Other objects created via this interface are unaffected."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "This object represents an active screencopy session."]
    #[doc = ""]
    #[doc = "After a screencopy session is created, buffer constraint events will be"]
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
    #[doc = "buffer accordingly, attach it to the screencopy session using the"]
    #[doc = "attach_buffer request, set the buffer damage using the damage_buffer"]
    #[doc = "request and then send the capture request."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_session_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the zcosmic_screencopy_session_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopySessionV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_screencopy_session_v2";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let frame = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_screencopy_session_v2#{}.create_frame({})",
                                sender_id,
                                frame
                            );
                            self.create_frame(client, sender_id, frame).await
                        }
                        1u16 => {
                            tracing::debug!(
                                "zcosmic_screencopy_session_v2#{}.destroy()",
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
            #[doc = "Create a capture frame for this session."]
            #[doc = ""]
            fn create_frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                frame: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect zcosmic_screencopy_frame_v2 objects created by"]
            #[doc = "this object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Provides the dimensions of the source image in buffer pixel coordinates."]
            #[doc = ""]
            #[doc = "The client must attach buffers that match this size."]
            #[doc = ""]
            fn buffer_size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                width: u32,
                height: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_session_v2#{}.buffer_size({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Provides the format that must be used for shared-memory buffers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            #[doc = ""]
            fn shm_format(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                format: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_session_v2#{}.shm_format({})",
                        sender_id,
                        format
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_uint(format).build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event advertises the device buffers must be allocated on for"]
            #[doc = "dma-buf buffers."]
            #[doc = ""]
            #[doc = "In general the device is a DRM node. The DRM node type (primary vs."]
            #[doc = "render) is unspecified. Clients must not rely on the compositor sending"]
            #[doc = "a particular node type. Clients cannot check two devices for equality"]
            #[doc = "by comparing the dev_t value."]
            #[doc = ""]
            fn dmabuf_device(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                device: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_session_v2#{}.dmabuf_device(array[{}])",
                        sender_id,
                        device.len()
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_array(device).build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Provides the format that must be used for dma-buf buffers."]
            #[doc = ""]
            #[doc = "The client may choose any of the modifiers advertised in the array of"]
            #[doc = "64-bit unsigned integers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            #[doc = ""]
            fn dmabuf_format(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                format: u32,
                modifiers: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_session_v2#{}.dmabuf_format({}, array[{}])",
                        sender_id,
                        format,
                        modifiers.len()
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(format)
                        .put_array(modifiers)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 3u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is sent once when all buffer constraint events have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "The compositor must always end a batch of buffer constraint events with"]
            #[doc = "this event, regardless of whether it sends the initial constraints or"]
            #[doc = "an update."]
            #[doc = ""]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_screencopy_session_v2#{}.done()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 4u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event indicates that the capture session has stopped and is no"]
            #[doc = "longer available. This can happen in a number of cases, e.g. when the"]
            #[doc = "underlying source is destroyed, if the user decides to end the screen"]
            #[doc = "capture, or if an unrecoverable runtime error has occurred."]
            #[doc = ""]
            #[doc = "The client should destroy the session after receiving this event."]
            #[doc = ""]
            fn stopped(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_screencopy_session_v2#{}.stopped()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 5u16, payload, fds))
                        .await
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "This object represents a screen capture frame."]
    #[doc = ""]
    #[doc = "The client should attach a buffer, damage the buffer, and then send a"]
    #[doc = "capture request."]
    #[doc = ""]
    #[doc = "If the screen capture is successful, the compositor will send the frame"]
    #[doc = "metadata (transform, damage, presentation_time in any order) followed by"]
    #[doc = "the ready event."]
    #[doc = ""]
    #[doc = "If the screen capture fails, the compositor will send the failed event."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_frame_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NoBuffer),
                    2u32 => Ok(Self::InvalidBufferDamage),
                    3u32 => Ok(Self::AlreadyCaptured),
                    _ => Err(crate::DecodeError::MalformedPayload),
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
        pub enum FailureReason {
            Unknown = 0u32,
            BufferConstraints = 1u32,
            Stopped = 2u32,
        }
        impl TryFrom<u32> for FailureReason {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unknown),
                    1u32 => Ok(Self::BufferConstraints),
                    2u32 => Ok(Self::Stopped),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for FailureReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_screencopy_frame_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopyFrameV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_screencopy_frame_v2";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zcosmic_screencopy_frame_v2#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_screencopy_frame_v2#{}.attach_buffer({})",
                                sender_id,
                                buffer
                            );
                            self.attach_buffer(client, sender_id, buffer).await
                        }
                        2u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            tracing::debug!(
                                "zcosmic_screencopy_frame_v2#{}.damage_buffer({}, {}, {}, {})",
                                sender_id,
                                x,
                                y,
                                width,
                                height
                            );
                            self.damage_buffer(client, sender_id, x, y, width, height)
                                .await
                        }
                        3u16 => {
                            tracing::debug!("zcosmic_screencopy_frame_v2#{}.capture()", sender_id,);
                            self.capture(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Attach a buffer to the session."]
            #[doc = ""]
            #[doc = "The wl_buffer.release request is unused."]
            #[doc = ""]
            #[doc = "This request must not be sent after capture, or else the"]
            #[doc = "already_captured protocol error is raised."]
            #[doc = ""]
            fn attach_buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                buffer: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Apply damage to the buffer which is to be captured next. This request"]
            #[doc = "may be sent multiple times to describe a region."]
            #[doc = ""]
            #[doc = "The client indicates the accumulated damage since this wl_buffer was"]
            #[doc = "last captured. During capture, the compositor will update the buffer"]
            #[doc = "with at least the union of the region passed by the client and the"]
            #[doc = "region advertised by zcosmic_screencopy_frame_v2.damage."]
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
            #[doc = ""]
            fn damage_buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Capture a frame."]
            #[doc = ""]
            #[doc = "Unless this is the first successful captured frame performed in this"]
            #[doc = "session, the compositor may wait an indefinite amount of time for the"]
            #[doc = "source content to change before performing the copy."]
            #[doc = ""]
            #[doc = "This request may only be sent once, or else the already_captured"]
            #[doc = "protocol error is raised. A buffer must be attached before this request"]
            #[doc = "is sent, or else the no_buffer protocol error is raised."]
            #[doc = ""]
            fn capture(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is sent before the ready event and holds the transform of"]
            #[doc = "the source buffer."]
            #[doc = ""]
            fn transform(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_frame_v2#{}.transform({})",
                        sender_id,
                        transform
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(transform as u32)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is sent before the ready event. It may be generated multiple"]
            #[doc = "times to describe a region."]
            #[doc = ""]
            #[doc = "The first captured frame in a session will always carry full damage."]
            #[doc = "Subsequent frames' damaged regions describe which parts of the buffer"]
            #[doc = "have changed since the last ready event."]
            #[doc = ""]
            #[doc = "These coordinates originate in the upper left corner of the buffer."]
            #[doc = ""]
            fn damage(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_frame_v2#{}.damage({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event indicates the time at which the frame is presented to the"]
            #[doc = "output in system monotonic time. This event is sent before the ready"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]."]
            #[doc = ""]
            fn presentation_time(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_frame_v2#{}.presentation_time({}, {}, {})",
                        sender_id,
                        tv_sec_hi,
                        tv_sec_lo,
                        tv_nsec
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(tv_sec_hi)
                        .put_uint(tv_sec_lo)
                        .put_uint(tv_nsec)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading."]
            #[doc = ""]
            #[doc = "The buffer may be re-used by the client after this event."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            #[doc = ""]
            fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_screencopy_frame_v2#{}.ready()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 3u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            #[doc = ""]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                reason: FailureReason,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_frame_v2#{}.failed({})",
                        sender_id,
                        reason
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_uint(reason as u32).build();
                    client
                        .send_message(crate::Message::new(sender_id, 4u16, payload, fds))
                        .await
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "This object represents a cursor capture session. It extends the base"]
    #[doc = "capture session with cursor-specific metadata."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_cursor_session_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "get_screencopy_session sent twice"]
            DuplicateSession = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::DuplicateSession),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_screencopy_cursor_session_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopyCursorSessionV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_screencopy_cursor_session_v2";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "zcosmic_screencopy_cursor_session_v2#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let session = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_screencopy_cursor_session_v2#{}.get_screencopy_session({})",
                                sender_id,
                                session
                            );
                            self.get_screencopy_session(client, sender_id, session)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect zcosmic_screencopy_frame_v2 objects created by"]
            #[doc = "this object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Gets the screencopy session for this cursor session."]
            #[doc = ""]
            #[doc = "The session will produce frames of the cursor image. The compositor may"]
            #[doc = "pause the session when the cursor leaves the captured area."]
            #[doc = ""]
            #[doc = "This request must not be sent more than once, or else the"]
            #[doc = "duplicate_session protocol error is raised."]
            #[doc = ""]
            fn get_screencopy_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                session: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sent when a cursor enters the captured area. It shall be generated"]
            #[doc = "before the \"position\" and \"hotspot\" events when and only when a cursor"]
            #[doc = "enters the area."]
            #[doc = ""]
            #[doc = "The cursor enters the captured area when the cursor image intersects"]
            #[doc = "with the captured area. Note, this is different from e.g."]
            #[doc = "wl_pointer.enter."]
            #[doc = ""]
            fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_cursor_session_v2#{}.enter()",
                        sender_id,
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Sent when a cursor leaves the captured area. No \"position\" or \"hotspot\""]
            #[doc = "event is generated for the cursor until the cursor enters the captured"]
            #[doc = "area again."]
            #[doc = ""]
            fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_cursor_session_v2#{}.leave()",
                        sender_id,
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Cursors outside the image source do not get captured and no event will"]
            #[doc = "be generated for them."]
            #[doc = ""]
            #[doc = "The given position is the position of the cursor's hotspot and it is"]
            #[doc = "relative to the main buffer's top left corner in transformed buffer"]
            #[doc = "pixel coordinates."]
            #[doc = ""]
            #[doc = "The position coordinates are relative to the main buffer's upper left"]
            #[doc = "corner. The coordinates may be negative or greater than the main buffer"]
            #[doc = "size."]
            #[doc = ""]
            fn position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_cursor_session_v2#{}.position({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_int(x).put_int(y).build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "The hotspot describes the offset between the cursor image and the"]
            #[doc = "position of the input device."]
            #[doc = ""]
            #[doc = "The given coordinates are the hotspot's offset from the origin in"]
            #[doc = "buffer coordinates."]
            #[doc = ""]
            #[doc = "Clients should not apply the hotspot immediately: the hotspot becomes"]
            #[doc = "effective when the next zcosmic_screencopy_frame_v2.ready event is received."]
            #[doc = ""]
            fn hotspot(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_screencopy_cursor_session_v2#{}.hotspot({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_int(x).put_int(y).build();
                    client
                        .send_message(crate::Message::new(sender_id, 3u16, payload, fds))
                        .await
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_toplevel_info_unstable_v1 {
    #[doc = ""]
    #[doc = "The purpose of this protocol is to enable clients such as taskbars"]
    #[doc = "or docks to access a list of opened applications and basic properties"]
    #[doc = "thereof."]
    #[doc = ""]
    #[doc = "It thus extends ext_foreign_toplevel_v1 to provide more information"]
    #[doc = "and actions on foreign toplevels."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_info_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the zcosmic_toplevel_info_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelInfoV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_toplevel_info_v1";
            const VERSION: u32 = 3u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zcosmic_toplevel_info_v1#{}.stop()", sender_id,);
                            self.stop(client, sender_id).await
                        }
                        1u16 => {
                            let cosmic_toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let foreign_toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_info_v1#{}.get_cosmic_toplevel({}, {})",
                                sender_id,
                                cosmic_toplevel,
                                foreign_toplevel
                            );
                            self.get_cosmic_toplevel(
                                client,
                                sender_id,
                                cosmic_toplevel,
                                foreign_toplevel,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events for new toplevels.  However, the compositor may emit further"]
            #[doc = "toplevel_created events until the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            #[doc = ""]
            #[doc = "Note: This request isn't necessary for clients binding version 2"]
            #[doc = "of this protocol and will be ignored."]
            #[doc = ""]
            fn stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request a zcosmic_toplevel_handle_v1 extension object for an existing"]
            #[doc = "ext_foreign_toplevel_handle_v1."]
            #[doc = ""]
            #[doc = "All initial properties of the toplevel (states, etc.)"]
            #[doc = "will be sent immediately after this event via the corresponding"]
            #[doc = "events in zcosmic_toplevel_handle_v1."]
            #[doc = ""]
            fn get_cosmic_toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                cosmic_toplevel: crate::ObjectId,
                foreign_toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is never emitted for clients binding version 2"]
            #[doc = "of this protocol, they should use `get_cosmic_toplevel` instead."]
            #[doc = ""]
            #[doc = "This event is emitted for clients binding version 1 whenever a"]
            #[doc = "new toplevel window is created. It is emitted for all toplevels,"]
            #[doc = "regardless of the app that has created them."]
            #[doc = ""]
            #[doc = "All initial properties of the toplevel (title, app_id, states, etc.)"]
            #[doc = "will be sent immediately after this event via the corresponding"]
            #[doc = "events in zcosmic_toplevel_handle_v1."]
            #[doc = ""]
            fn toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_info_v1#{}.toplevel({})",
                        sender_id,
                        toplevel
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(toplevel))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event indicates that the compositor is done sending events"]
            #[doc = "to the zcosmic_toplevel_info_v1. The server will destroy the"]
            #[doc = "object immediately after sending this request, so it will become"]
            #[doc = "invalid and the client should free any resources associated with it."]
            #[doc = ""]
            #[doc = "Note: This event is emitted immediately after calling `stop` for"]
            #[doc = "clients binding version 2 of this protocol for backwards compatibility."]
            #[doc = ""]
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_toplevel_info_v1#{}.finished()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is sent after all changes for currently active"]
            #[doc = "zcosmic_toplevel_handle_v1 have been sent."]
            #[doc = ""]
            #[doc = "This allows changes to multiple zcosmic_toplevel_handle_v1 handles"]
            #[doc = "and their properties to be seen as atomic, even if they happen via"]
            #[doc = "multiple events."]
            #[doc = ""]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_toplevel_info_v1#{}.done()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "A zcosmic_toplevel_handle_v1 object represents an open toplevel"]
    #[doc = "window. A single app may have multiple open toplevels."]
    #[doc = ""]
    #[doc = "Each toplevel has a list of outputs it is visible on, exposed to the"]
    #[doc = "client via the output_enter and output_leave events."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_handle_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "The different states that a toplevel may have. These have the same"]
        #[doc = "meaning as the states with the same names defined in xdg-toplevel"]
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
            #[doc = "the toplevel is sticky"]
            Sticky = 4u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Maximized),
                    1u32 => Ok(Self::Minimized),
                    2u32 => Ok(Self::Activated),
                    3u32 => Ok(Self::Fullscreen),
                    4u32 => Ok(Self::Sticky),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelHandleV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_toplevel_handle_v1";
            const VERSION: u32 = 3u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zcosmic_toplevel_handle_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request should be called either when the client will no longer"]
            #[doc = "use the zcosmic_toplevel_handle_v1."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The server will emit no further events on the"]
            #[doc = "zcosmic_toplevel_handle_v1 after this event. Any requests received"]
            #[doc = "aside from the destroy request will be ignored. Upon receiving this"]
            #[doc = "event, the client should make the destroy request to allow freeing"]
            #[doc = "of resources."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.closed` is"]
            #[doc = "equivalent."]
            #[doc = ""]
            fn closed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_toplevel_handle_v1#{}.closed()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is sent after all changes in the toplevel state have"]
            #[doc = "been sent."]
            #[doc = ""]
            #[doc = "This allows changes to the zcosmic_toplevel_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            #[doc = ""]
            #[doc = "Note: this is is not sent after the closed event."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.done` is"]
            #[doc = "equivalent."]
            #[doc = ""]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> zcosmic_toplevel_handle_v1#{}.done()", sender_id,);
                    let (payload, fds) = crate::PayloadBuilder::new().build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.title` is"]
            #[doc = "equivalent."]
            #[doc = ""]
            fn title(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                title: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.title(\"{}\")",
                        sender_id,
                        title
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_string(Some(title)).build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the app_id of the toplevel changes."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.app_id` is"]
            #[doc = "equivalent."]
            #[doc = ""]
            fn app_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                app_id: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.app_id(\"{}\")",
                        sender_id,
                        app_id
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 3u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel becomes visible on the"]
            #[doc = "given output. A toplevel may be visible on multiple outputs."]
            #[doc = ""]
            fn output_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                output: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.output_enter({})",
                        sender_id,
                        output
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 4u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel is no longer visible"]
            #[doc = "on a given output. It is guaranteed that an output_enter event with"]
            #[doc = "the same output has been emitted before this event."]
            #[doc = ""]
            fn output_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                output: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.output_leave({})",
                        sender_id,
                        output
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 5u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel becomes visible on the"]
            #[doc = "given workspace. A toplevel may be visible on multiple workspaces."]
            #[doc = ""]
            fn workspace_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                workspace: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.workspace_enter({})",
                        sender_id,
                        workspace
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(workspace))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 6u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel is no longer visible"]
            #[doc = "on a given workspace. It is guaranteed that an workspace_enter event with"]
            #[doc = "the same workspace has been emitted before this event."]
            #[doc = ""]
            fn workspace_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                workspace: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.workspace_leave({})",
                        sender_id,
                        workspace
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(workspace))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 7u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted once on creation of the"]
            #[doc = "zcosmic_toplevel_handle_v1 and again whenever the state of the"]
            #[doc = "toplevel changes."]
            #[doc = ""]
            fn state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.state(array[{}])",
                        sender_id,
                        state.len()
                    );
                    let (payload, fds) = crate::PayloadBuilder::new().put_array(state).build();
                    client
                        .send_message(crate::Message::new(sender_id, 8u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "Emitted when the geometry of a toplevel (it's position and/or size)"]
            #[doc = "relative to the provided output has changed."]
            #[doc = ""]
            #[doc = "This event is emitted once on creation of the"]
            #[doc = "zcosmic_toplevel_handle_v1 for every entered output and again"]
            #[doc = "whenever the geometry of the toplevel changes relative to any output."]
            #[doc = ""]
            fn geometry(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                output: crate::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.geometry({}, {}, {}, {}, {})",
                        sender_id,
                        output,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(output))
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 9u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel becomes visible on the"]
            #[doc = "given workspace. A toplevel may be visible on multiple workspaces."]
            #[doc = ""]
            fn ext_workspace_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                workspace: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.ext_workspace_enter({})",
                        sender_id,
                        workspace
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(workspace))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 10u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever the toplevel is no longer visible"]
            #[doc = "on a given workspace. It is guaranteed that an workspace_enter event with"]
            #[doc = "the same workspace has been emitted before this event."]
            #[doc = ""]
            fn ext_workspace_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                workspace: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_handle_v1#{}.ext_workspace_leave({})",
                        sender_id,
                        workspace
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_object(Some(workspace))
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 11u16, payload, fds))
                        .await
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_toplevel_management_unstable_v1 {
    #[doc = ""]
    #[doc = "This protocol allows clients such as a taskbar to request the compositor"]
    #[doc = "to preform typical actions on open toplevels. The compositor is in all"]
    #[doc = "cases free to ignore the request."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_manager_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ZcosmicToplelevelManagementCapabilitiesV1 {
            #[doc = "close request is available"]
            Close = 1u32,
            #[doc = "activate request is available"]
            Activate = 2u32,
            #[doc = "set_maximized and unset_maximized requests are available"]
            Maximize = 3u32,
            #[doc = "set_minimized and unset_minimized requests are available"]
            Minimize = 4u32,
            #[doc = "set_fullscreen and unset_fullscreen requests are available"]
            Fullscreen = 5u32,
            #[doc = "move_to_workspace request is available"]
            MoveToWorkspace = 6u32,
            #[doc = "set_sticky and unset_sticky requests are available"]
            Sticky = 7u32,
            #[doc = "move_to_ext_workspace request is available"]
            MoveToExtWorkspace = 8u32,
        }
        impl TryFrom<u32> for ZcosmicToplelevelManagementCapabilitiesV1 {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Close),
                    2u32 => Ok(Self::Activate),
                    3u32 => Ok(Self::Maximize),
                    4u32 => Ok(Self::Minimize),
                    5u32 => Ok(Self::Fullscreen),
                    6u32 => Ok(Self::MoveToWorkspace),
                    7u32 => Ok(Self::Sticky),
                    8u32 => Ok(Self::MoveToExtWorkspace),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ZcosmicToplelevelManagementCapabilitiesV1 {
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
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidRectangle),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelManagerV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_toplevel_manager_v1";
            const VERSION: u32 = 4u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zcosmic_toplevel_manager_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.close({})",
                                sender_id,
                                toplevel
                            );
                            self.close(client, sender_id, toplevel).await
                        }
                        2u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.activate({}, {})",
                                sender_id,
                                toplevel,
                                seat
                            );
                            self.activate(client, sender_id, toplevel, seat).await
                        }
                        3u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.set_maximized({})",
                                sender_id,
                                toplevel
                            );
                            self.set_maximized(client, sender_id, toplevel).await
                        }
                        4u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.unset_maximized({})",
                                sender_id,
                                toplevel
                            );
                            self.unset_maximized(client, sender_id, toplevel).await
                        }
                        5u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.set_minimized({})",
                                sender_id,
                                toplevel
                            );
                            self.set_minimized(client, sender_id, toplevel).await
                        }
                        6u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.unset_minimized({})",
                                sender_id,
                                toplevel
                            );
                            self.unset_minimized(client, sender_id, toplevel).await
                        }
                        7u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let output = message.object()?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.set_fullscreen({}, {})",
                                sender_id,
                                toplevel,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_fullscreen(client, sender_id, toplevel, output)
                                .await
                        }
                        8u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.unset_fullscreen({})",
                                sender_id,
                                toplevel
                            );
                            self.unset_fullscreen(client, sender_id, toplevel).await
                        }
                        9u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.set_rectangle({}, {}, {}, {}, {}, {})",
                                sender_id,
                                toplevel,
                                surface,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_rectangle(
                                client, sender_id, toplevel, surface, x, y, width, height,
                            )
                            .await
                        }
                        10u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let workspace = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.move_to_workspace({}, {}, {})",
                                sender_id,
                                toplevel,
                                workspace,
                                output
                            );
                            self.move_to_workspace(client, sender_id, toplevel, workspace, output)
                                .await
                        }
                        11u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.set_sticky({})",
                                sender_id,
                                toplevel
                            );
                            self.set_sticky(client, sender_id, toplevel).await
                        }
                        12u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.unset_sticky({})",
                                sender_id,
                                toplevel
                            );
                            self.unset_sticky(client, sender_id, toplevel).await
                        }
                        13u16 => {
                            let toplevel = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let workspace = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_toplevel_manager_v1#{}.move_to_ext_workspace({}, {}, {})",
                                sender_id,
                                toplevel,
                                workspace,
                                output
                            );
                            self.move_to_ext_workspace(
                                client, sender_id, toplevel, workspace, output,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request indicates that the client has finished using the"]
            #[doc = "zcosmic_toplevel_manager_v1 object and that it can be safely"]
            #[doc = "destroyed."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.closed event will be sent."]
            #[doc = ""]
            fn close(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn activate(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
                seat: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn set_maximized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn unset_maximized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn set_minimized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn unset_minimized(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state and potentially the"]
            #[doc = "zcosmic_toplevel_handle_v1.output_enter/output_leave events will"]
            #[doc = "be sent."]
            #[doc = ""]
            #[doc = "The output parameter a hint to the compositor and may be ignored. A"]
            #[doc = "value of NULL indicates that the compositor should choose the target"]
            #[doc = "output, if it honors the fullscreen request."]
            #[doc = ""]
            fn set_fullscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
                output: Option<crate::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn unset_fullscreen(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If a client using this protocol displays UI elements corresponding"]
            #[doc = "to toplevels, it may use this request to inform the server about such"]
            #[doc = "a relation. This information may be used by the server, for example as"]
            #[doc = "the target for a minimize animation."]
            #[doc = ""]
            #[doc = "If the client sets more than one rectangle, only the most recently"]
            #[doc = "set rectangle is considered."]
            #[doc = ""]
            #[doc = "The dimensions are given in surface-local coordinates."]
            #[doc = ""]
            #[doc = "Setting width=height=0 removes the current rectangle if one was set."]
            #[doc = ""]
            fn set_rectangle(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
                surface: crate::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Move window to workspace, on given output."]
            #[doc = ""]
            fn move_to_workspace(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
                workspace: crate::ObjectId,
                output: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn set_sticky(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            #[doc = ""]
            fn unset_sticky(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Move window to workspace, on given output."]
            #[doc = ""]
            fn move_to_ext_workspace(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                toplevel: crate::ObjectId,
                workspace: crate::ObjectId,
                output: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event advertises the capabilities supported by the compositor. If"]
            #[doc = "a capability isn't supported, clients should hide or disable the UI"]
            #[doc = "elements that expose this functionality. For instance, if the"]
            #[doc = "compositor doesn't advertise support for closing toplevels, a button"]
            #[doc = "triggering the close request should not be displayed."]
            #[doc = ""]
            #[doc = "The compositor will ignore requests it doesn't support. For instance,"]
            #[doc = "a compositor which doesn't advertise support for closing toplevels will ignore"]
            #[doc = "close requests."]
            #[doc = ""]
            #[doc = "Compositors must send this event once after creation of an"]
            #[doc = "zcosmic_toplevel_manager_v1 . When the capabilities change, compositors"]
            #[doc = "must send this event again."]
            #[doc = ""]
            #[doc = "The capabilities are sent as an array of 32-bit unsigned integers in"]
            #[doc = "native endianness."]
            #[doc = ""]
            fn capabilities(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                capabilities: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_toplevel_manager_v1#{}.capabilities(array[{}])",
                        sender_id,
                        capabilities.len()
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_array(capabilities).build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_workspace_unstable_v2 {
    #[doc = ""]
    #[doc = "This protocol extends `ext-workspace-v1` with addtional requests and events."]
    #[doc = ""]
    #[doc = "The caller should call `get_cosmic_workspace` whenever a new ext workspace is"]
    #[doc = "created."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_workspace_manager_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "zcosmic_workspace_handle_v2 already exists for ext_workspace_handle_v1"]
            WorkspaceExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::WorkspaceExists),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_workspace_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicWorkspaceManagerV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_workspace_manager_v2";
            const VERSION: u32 = 2u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let cosmic_workspace = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let workspace = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_workspace_manager_v2#{}.get_cosmic_workspace({}, {})",
                                sender_id,
                                cosmic_workspace,
                                workspace
                            );
                            self.get_cosmic_workspace(
                                client,
                                sender_id,
                                cosmic_workspace,
                                workspace,
                            )
                            .await
                        }
                        1u16 => {
                            tracing::debug!("zcosmic_workspace_manager_v2#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Request a `zcosmic_workspace_handle_v2` extension object for an existing"]
            #[doc = "`ext_workspace_handle_v1`."]
            #[doc = ""]
            #[doc = "If a `zcosmic_workspace_handle_v2` already exists for the `ext_workspace_handle_v1`, this"]
            #[doc = "will raise a `workspace_exists` protocol error."]
            #[doc = ""]
            fn get_cosmic_workspace(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                cosmic_workspace: crate::ObjectId,
                workspace: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This request should be called either when the client will no longer"]
            #[doc = "use the `zcosmic_workspace_manager_v2`."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "A zcosmic_workspace_handle_v2 object represents a a workspace that handles a"]
    #[doc = "group of surfaces."]
    #[doc = ""]
    #[doc = "Each workspace has a name, conveyed to the client with the name event; a"]
    #[doc = "list of states, conveyed to the client with the state event; and"]
    #[doc = "optionally a set of coordinates, conveyed to the client with the"]
    #[doc = "coordinates event. The client may request that the compositor activate or"]
    #[doc = "deactivate the workspace."]
    #[doc = ""]
    #[doc = "Each workspace can belong to only a single workspace group."]
    #[doc = "Depepending on the compositor policy, there might be workspaces with"]
    #[doc = "the same name in different workspace groups, but these workspaces are still"]
    #[doc = "separate (e.g. one of them might be active while the other is not)."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_workspace_handle_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct WorkspaceCapabilities : u32 { # [doc = "rename request is available"] const Rename = 1u32 ; # [doc = "set_tiling_state request is available"] const SetTilingState = 2u32 ; # [doc = "pin and unpin requests are available"] const Pin = 3u32 ; # [doc = "move_before and move_after requests are available"] const Move = 4u32 ; } }
        impl TryFrom<u32> for WorkspaceCapabilities {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for WorkspaceCapabilities {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TilingState {
            #[doc = "The workspace has no active tiling properties"]
            FloatingOnly = 0u32,
            #[doc = "Tiling behavior is enabled for the workspace"]
            TilingEnabled = 1u32,
        }
        impl TryFrom<u32> for TilingState {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::FloatingOnly),
                    1u32 => Ok(Self::TilingEnabled),
                    _ => Err(crate::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TilingState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct State : u32 { # [doc = "the workspace is pinned"] const Pinned = 1u32 ; } }
        impl TryFrom<u32> for State {
            type Error = crate::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_workspace_handle_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicWorkspaceHandleV2: crate::server::Dispatcher {
            const INTERFACE: &'static str = "zcosmic_workspace_handle_v2";
            const VERSION: u32 = 2u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                message: &mut crate::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("zcosmic_workspace_handle_v2#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let name = message
                                .string()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "zcosmic_workspace_handle_v2#{}.rename(\"{}\")",
                                sender_id,
                                name
                            );
                            self.rename(client, sender_id, name).await
                        }
                        2u16 => {
                            let state = message.uint()?;
                            tracing::debug!(
                                "zcosmic_workspace_handle_v2#{}.set_tiling_state({})",
                                sender_id,
                                state
                            );
                            self.set_tiling_state(client, sender_id, state.try_into()?)
                                .await
                        }
                        3u16 => {
                            let other_workspace = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let axis = message.uint()?;
                            tracing::debug!(
                                "zcosmic_workspace_handle_v2#{}.move_before({}, {})",
                                sender_id,
                                other_workspace,
                                axis
                            );
                            self.move_before(client, sender_id, other_workspace, axis)
                                .await
                        }
                        4u16 => {
                            let other_workspace = message
                                .object()?
                                .ok_or(crate::DecodeError::MalformedPayload)?;
                            let axis = message.uint()?;
                            tracing::debug!(
                                "zcosmic_workspace_handle_v2#{}.move_after({}, {})",
                                sender_id,
                                other_workspace,
                                axis
                            );
                            self.move_after(client, sender_id, other_workspace, axis)
                                .await
                        }
                        5u16 => {
                            tracing::debug!("zcosmic_workspace_handle_v2#{}.pin()", sender_id,);
                            self.pin(client, sender_id).await
                        }
                        6u16 => {
                            tracing::debug!("zcosmic_workspace_handle_v2#{}.unpin()", sender_id,);
                            self.unpin(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request should be called either when the client will no longer"]
            #[doc = "use the `zcosmic_workspace_handle_v1`."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request that this workspace is renamed."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will actually be renamed."]
            #[doc = ""]
            fn rename(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                name: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request that this workspace's tiling state is changed."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will actually change it's tiling state."]
            #[doc = ""]
            fn set_tiling_state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state: TilingState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Move a workspace to be before another workspace along a given axis."]
            #[doc = ""]
            #[doc = "`other_workspace` may be on the same workspace group, or on a different group."]
            #[doc = "If it's a different set, the workspace will also be moved to that group."]
            #[doc = ""]
            #[doc = "`axis` should be a valid index in the coordinates on the workspace group"]
            #[doc = "`other_workspace` is on. The workspace will be positioned on the target group"]
            #[doc = "to have a coordinate with this component less than the value of the component for"]
            #[doc = "`other_workspace`. The exact coordinate values, or how other workspaces are moved"]
            #[doc = "to accommodate the workspace, is unspecified."]
            #[doc = ""]
            #[doc = "The request will be ignored if `axis` is invalid or the compositor is otherwise"]
            #[doc = "unable to move the workspace."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will actually be moved."]
            #[doc = ""]
            fn move_before(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                other_workspace: crate::ObjectId,
                axis: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Move a workspace to be after another workspace along a given axis."]
            #[doc = ""]
            #[doc = "See `move_before`."]
            #[doc = ""]
            fn move_after(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                other_workspace: crate::ObjectId,
                axis: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request that this workspace be pinned."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will be actually pinned."]
            #[doc = ""]
            fn pin(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request that this workspace be unpinned."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will be actually unpinned."]
            #[doc = ""]
            fn unpin(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event advertises the capabilities supported by the compositor. If"]
            #[doc = "a capability isn't supported, clients should hide or disable the UI"]
            #[doc = "elements that expose this functionality. For instance, if the"]
            #[doc = "compositor doesn't advertise support for removing workspaces, a button"]
            #[doc = "triggering the remove request should not be displayed."]
            #[doc = ""]
            #[doc = "The compositor will ignore requests it doesn't support. For instance,"]
            #[doc = "a compositor which doesn't advertise support for remove will ignore"]
            #[doc = "remove requests."]
            #[doc = ""]
            #[doc = "Compositors must send this event once after creation of a"]
            #[doc = "`zcosmic_workspace_handle_v2`. When the capabilities change, compositors"]
            #[doc = "must send this event again."]
            #[doc = ""]
            fn capabilities(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                capabilities: WorkspaceCapabilities,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_workspace_handle_v2#{}.capabilities({})",
                        sender_id,
                        capabilities
                    );
                    let (payload, fds) = crate::PayloadBuilder::new()
                        .put_uint(capabilities.bits())
                        .build();
                    client
                        .send_message(crate::Message::new(sender_id, 0u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted immediately after the zcosmic_workspace_handle_v2 is created"]
            #[doc = "and each time the workspace tiling state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            #[doc = ""]
            fn tiling_state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state: TilingState,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_workspace_handle_v2#{}.tiling_state({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_uint(state as u32).build();
                    client
                        .send_message(crate::Message::new(sender_id, 1u16, payload, fds))
                        .await
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted immediately after the zcosmic_workspace_handle_v2 is"]
            #[doc = "created and each time the workspace state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            #[doc = ""]
            fn state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::ObjectId,
                state: State,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> zcosmic_workspace_handle_v2#{}.state({})",
                        sender_id,
                        state
                    );
                    let (payload, fds) =
                        crate::PayloadBuilder::new().put_uint(state.bits()).build();
                    client
                        .send_message(crate::Message::new(sender_id, 2u16, payload, fds))
                        .await
                }
            }
        }
    }
}
