#![allow(async_fn_in_trait)]
#[doc = "This protocol provides a relatively straightforward mapping of AtpsiDevice"]
#[doc = "in the at-spi2-core library, so it's possible to add a Wayland backend for it."]
#[doc = ""]
#[doc = "This provides a way for screen reader key bindings to work."]
#[doc = ""]
#[doc = "This is a temporary solution until a better protocol is available for this purpose."]
#[allow(clippy::module_inception)]
pub mod cosmic_atspi_v1 {
    #[doc = "Manager for adding grabs and monitoring key input."]
    #[allow(clippy::too_many_arguments)]
    pub mod cosmic_atspi_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the cosmic_atspi_manager_v1 interface. See the module level documentation for more info"]
        pub trait CosmicAtspiManagerV1 {
            const INTERFACE: &'static str = "cosmic_atspi_manager_v1";
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
            #[doc = "Any grabs that are still active will be disabled."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> cosmic_atspi_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Grab the given key combination, so it will not be sent to clients."]
            async fn add_key_grab(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mods: u32,
                virtual_mods: Vec<u8>,
                key: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> cosmic_atspi_manager_v1#{}.add_key_grab()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(mods)
                    .put_array(virtual_mods)
                    .put_uint(key)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Disables a grab added with add_key_grab."]
            async fn remove_key_grab(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mods: u32,
                virtual_mods: Vec<u8>,
                key: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> cosmic_atspi_manager_v1#{}.remove_key_grab()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(mods)
                    .put_array(virtual_mods)
                    .put_uint(key)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Grab keyboard, so key input will not be sent to clients."]
            async fn grab_keyboard(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> cosmic_atspi_manager_v1#{}.grab_keyboard()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Disables a grab added with grab_keyboard."]
            async fn ungrab_keyboard(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> cosmic_atspi_manager_v1#{}.ungrab_keyboard()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Produces an fd that can be used with libei to monitor keyboard input."]
            async fn key_events_eis(&self, fd: rustix::fd::OwnedFd) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol serves as an intermediary between screen capturing protocols"]
#[doc = "and potential image sources such as outputs and toplevels."]
#[doc = ""]
#[doc = "This protocol may be extended to support more image sources in the future,"]
#[doc = "thereby adding those image sources to other protocols that use the image"]
#[doc = "source object without having to modify those protocols."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[allow(clippy::module_inception)]
pub mod cosmic_image_source_unstable_v1 {
    #[doc = "The image source object is an opaque descriptor for a capturable resource."]
    #[doc = "This resource may be any sort of entity from which an image may be"]
    #[doc = "derived."]
    #[doc = ""]
    #[doc = "Note, because zcosmic_image_source_v1 objects are created from multiple"]
    #[doc = "independent factory interfaces, the zcosmic_image_source_v1 interface is"]
    #[doc = "frozen at version 1."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_image_source_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_image_source_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicImageSourceV1 {
            const INTERFACE: &'static str = "zcosmic_image_source_v1";
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
            #[doc = "Destroys the image source. This request may be sent at any time by the"]
            #[doc = "client."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_image_source_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A manager for creating image source objects for wl_output objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_image_source_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_output_image_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputImageSourceManagerV1 {
            const INTERFACE: &'static str = "zcosmic_output_image_source_manager_v1";
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
            #[doc = "Creates a source object for an output. Images captured from this source"]
            #[doc = "will show the same content as the output. Some elements may be omitted,"]
            #[doc = "such as cursors and overlays that have been marked as transparent to"]
            #[doc = "capturing."]
            async fn create_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_image_source_manager_v1#{}.create_source()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(source))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_image_source_manager_v1#{}.destroy()",
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
    #[doc = "A manager for creating image source objects for wl_output objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_workspace_image_source_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_workspace_image_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicWorkspaceImageSourceManagerV1 {
            const INTERFACE: &'static str = "zcosmic_workspace_image_source_manager_v1";
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
            #[doc = "Creates a source object for a workspaces. Images captured from this source"]
            #[doc = "will show the same content as the workspace. Some elements may be omitted,"]
            #[doc = "such as cursors and overlays that have been marked as transparent to"]
            #[doc = "capturing."]
            async fn create_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_workspace_image_source_manager_v1#{}.create_source()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(source))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_workspace_image_source_manager_v1#{}.destroy()",
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
    #[doc = "A manager for creating image source objects for wl_output objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_ext_workspace_image_source_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_ext_workspace_image_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicExtWorkspaceImageSourceManagerV1 {
            const INTERFACE: &'static str = "zcosmic_ext_workspace_image_source_manager_v1";
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
            #[doc = "Creates a source object for a workspaces. Images captured from this source"]
            #[doc = "will show the same content as the workspace. Some elements may be omitted,"]
            #[doc = "such as cursors and overlays that have been marked as transparent to"]
            #[doc = "capturing."]
            async fn create_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_ext_workspace_image_source_manager_v1#{}.create_source()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(source))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_ext_workspace_image_source_manager_v1#{}.destroy()",
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
    #[doc = "A manager for creating image source objects for"]
    #[doc = "zcosmic_toplevel_handle_v1 objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_image_source_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_toplevel_image_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelImageSourceManagerV1 {
            const INTERFACE: &'static str = "zcosmic_toplevel_image_source_manager_v1";
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
            #[doc = "Creates a source object for a toplevel handle. Images captured"]
            #[doc = "from this source will show the same content as the toplevel."]
            async fn create_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                toplevel_handle: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_image_source_manager_v1#{}.create_source()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(source))
                    .put_object(Some(toplevel_handle))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_image_source_manager_v1#{}.destroy()",
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
#[doc = "This protocol serves as an extension to wlr-output-management."]
#[doc = ""]
#[doc = "It primarily adds explicit output mirroring,"]
#[doc = "while upstream is figuring out how to best support that."]
#[doc = ""]
#[doc = "It was designed against version 4 of wlr-output-management, but tries"]
#[doc = "it's best to be forward compatible."]
#[allow(clippy::module_inception)]
pub mod cosmic_output_management_unstable_v1 {
    #[doc = "This interface provides extension points for wlr-output-management types."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_manager_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "object already created"]
            AlreadyExtended = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyExtended),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputManagerV1 {
            const INTERFACE: &'static str = "zcosmic_output_manager_v1";
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
            async fn get_head(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                extended: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_output_manager_v1#{}.get_head()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(extended))
                    .put_object(Some(head))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Gets an extension object for zwlr_output_configuration_v1."]
            #[doc = ""]
            #[doc = "Trying to create more than one zcosmic_output_configuration_v1 per zwlr_output_configuration_v1"]
            #[doc = "will raise an \"already_extended\" error."]
            async fn get_configuration(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                extended: crate::wire::ObjectId,
                config: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_manager_v1#{}.get_configuration()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(extended))
                    .put_object(Some(config))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Gets an extension object for zwlr_output_configuration_head_v1."]
            #[doc = ""]
            #[doc = "Trying to create more than one zcosmic_output_configuration_head_v1 per"]
            #[doc = "zwlr_output_configuration_head_v1 will raise an \"already_extended\" error."]
            async fn get_configuration_head(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                extended: crate::wire::ObjectId,
                config_head: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_manager_v1#{}.get_configuration_head()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(extended))
                    .put_object(Some(config_head))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys this global. All previously created objects remain valid."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_output_manager_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "Extension to zwlr_output_head_v1."]
    #[doc = ""]
    #[doc = "Adds additional read-only properties."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the wlr_output_manager.done event."]
    #[doc = "No guarantees are made regarding the order in which properties are sent."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_head_v1 {
        use futures_util::SinkExt;
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unsupported),
                    1u32 => Ok(Self::RequiresModeset),
                    2u32 => Ok(Self::Supported),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Automatic),
                    2u32 => Ok(Self::Always),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AdaptiveSyncStateExt {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_output_head_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputHeadV1 {
            const INTERFACE: &'static str = "zcosmic_output_head_v1";
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
            #[doc = "Using this request a client can tell the compositor that it is not interested"]
            #[doc = "in the head object anymore."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_output_head_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This events describes the scale of the head in the global compositor"]
            #[doc = "space multiplied by 1000 for additional precision."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            async fn scale_1000(&self, scale_1000: i32) -> crate::client::Result<()>;
            #[doc = "This events describes that the head is mirroring another."]
            #[doc = "In these cases `name` contains the unique name of the matching `zwlr_output_head_v1`."]
            #[doc = "If the name is null, no head is being mirrored onto this one."]
            #[doc = ""]
            #[doc = "For mirrored heads the `position`-event is meaningless."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            async fn mirroring(&self, name: Option<String>) -> crate::client::Result<()>;
            #[doc = "This events describes if adaptive_sync is available for this head."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            async fn adaptive_sync_available(
                &self,
                available: AdaptiveSyncAvailability,
            ) -> crate::client::Result<()>;
            #[doc = "This events describes the adaptive_sync state of this head."]
            #[doc = ""]
            #[doc = "It is only sent if the output is enabled."]
            async fn adaptive_sync_ext(
                &self,
                state: AdaptiveSyncStateExt,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "Extension to zwlr_output_configuration_v1."]
    #[doc = ""]
    #[doc = "Adds additional parameters to be tested/applyed via the original zwlr_output_configuration_v1."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_configuration_v1 {
        use futures_util::SinkExt;
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyFinished),
                    2u32 => Ok(Self::MirroredHeadBusy),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_output_configuration_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputConfigurationV1 {
            const INTERFACE: &'static str = "zcosmic_output_configuration_v1";
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
            async fn mirror_head(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                head: crate::wire::ObjectId,
                mirroring: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_configuration_v1#{}.mirror_head()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(head))
                    .put_object(Some(mirroring))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Using this request a client can tell the compositor that it is not going"]
            #[doc = "to use the configuration object anymore. Any changes to the outputs"]
            #[doc = "will still be attached to the original `zwlr_output_configuration_head_v1`"]
            #[doc = "if it isn't destroyed."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_output_configuration_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event indicates that the configuration is no longer available."]
            #[doc = ""]
            #[doc = "This usually happens when the original configuration was `cancelled`, `suceeded` or `failed`."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy this object."]
            #[doc = ""]
            #[doc = "The configration object becomes inert and any requests other than `destroy` will be ignored."]
            async fn finished(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "Extension to zwlr_output_configuration_head_v1."]
    #[doc = ""]
    #[doc = "Adds additional/alternative parameters to the original zwlr_output_configuration_head_v1."]
    #[doc = ""]
    #[doc = "Once the original `zwlr_output_configuration_head_v1` is destroyed this object will"]
    #[doc = "become inert and all requests except `release` will be ignored."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_output_configuration_head_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_output_configuration_head_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOutputConfigurationHeadV1 {
            const INTERFACE: &'static str = "zcosmic_output_configuration_head_v1";
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
            #[doc = "This request sets the head's scale multiplied by 1000 for additional precision."]
            #[doc = ""]
            #[doc = "This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_scale`."]
            #[doc = "Using `set_scale` and `set_scale_1000` at once will thus raise an `already_set` error on the"]
            #[doc = "original `zwlr_output_configuration_head_v1`."]
            #[doc = ""]
            #[doc = "Any request conflicting with `set_scale` will also conflict with `set_scale_1000`."]
            async fn set_scale_1000(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                scale_1000: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_configuration_head_v1#{}.set_scale_1000()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(scale_1000)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Using this request a client can tell the compositor that it is not going"]
            #[doc = "to use the configuration object anymore. Already issued requests will"]
            #[doc = "still be attached to the original `zwlr_output_configuration_head_v1`"]
            #[doc = "until it is destroyed."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_configuration_head_v1#{}.release()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request requests a new adaptive sync state."]
            #[doc = ""]
            #[doc = "This request is meant to be used in place of `zwlr_output_configuration_head_v1::set_adaptive_sync`."]
            #[doc = "Using `set_adaptive_sync` and `set_adaptive_sync_ext` at once will thus raise an `already_set` error on the"]
            #[doc = "original `zwlr_output_configuration_head_v1`."]
            #[doc = ""]
            #[doc = "Any request conflicting with `set_adaptive_sync` will also conflict with `set_adaptive_sync_ext`."]
            async fn set_adaptive_sync_ext(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                state : super :: super :: super :: cosmic :: cosmic_output_management_unstable_v1 :: zcosmic_output_head_v1 :: AdaptiveSyncStateExt,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_output_configuration_head_v1#{}.set_adaptive_sync_ext()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(state as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_overlap_notify_unstable_v1 {
    #[doc = "The purpose of this protocol is to enable layer-shell client to get"]
    #[doc = "notifications if part of their surfaces are occluded other elements"]
    #[doc = "(currently toplevels and other layer-surfaces)."]
    #[doc = ""]
    #[doc = "You can request a notification object for any of your zwlr_layer_surface_v1"]
    #[doc = "surfaces, which will then emit overlap events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_overlap_notify_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_overlap_notify_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOverlapNotifyV1 {
            const INTERFACE: &'static str = "zcosmic_overlap_notify_v1";
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
            #[doc = "Requests notifications for toplevels and layer-surfaces entering and leaving the"]
            #[doc = "surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to"]
            #[doc = "implement auto-hide functionality."]
            #[doc = ""]
            #[doc = "To stop receiving notifications, destroy the returned"]
            #[doc = "zcosmic_overlap_notification_v1 object."]
            async fn notify_on_overlap(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                overlap_notification: crate::wire::ObjectId,
                layer_surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_overlap_notify_v1#{}.notify_on_overlap()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(overlap_notification))
                    .put_object(Some(layer_surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_overlap_notification_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_overlap_notification_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicOverlapNotificationV1 {
            const INTERFACE: &'static str = "zcosmic_overlap_notification_v1";
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
            #[doc = "This request should be called when the client has no interest in overlap"]
            #[doc = "notifications anymore."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_overlap_notification_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "A ext_foreign_toplevel_handle_v1 has entered the surface area."]
            #[doc = ""]
            #[doc = "This event will be emitted once for every ext_foreign_toplevel_handle_v1"]
            #[doc = "representing this toplevel."]
            #[doc = ""]
            #[doc = "Compositors are free to update the overlapping area by sending additional"]
            #[doc = "`toplevel_enter` events for the same toplevel without sending `toplevel_leave`"]
            #[doc = "in between."]
            async fn toplevel_enter(
                &self,
                toplevel: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()>;
            #[doc = "A ext_foreign_toplevel_handle_v1 has left the surface area."]
            #[doc = ""]
            #[doc = "This event will be emitted once for every ext_foreign_toplevel_handle_v1"]
            #[doc = "representing this toplevel."]
            async fn toplevel_leave(
                &self,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "A zwlr_layer_surface_v1 has entered the surface area."]
            #[doc = ""]
            #[doc = "Compositors are free to update the overlapping area by sending additional"]
            #[doc = "`layer_enter` events for the same surface without sending `layer_leave`"]
            #[doc = "in between."]
            #[doc = ""]
            #[doc = "The overlapping region is given surface-relative to the zwlr_layer_surface_v1"]
            #[doc = "used to create this notification object."]
            async fn layer_enter(
                &self,
                identifier: String,
                namespace: String,
                exclusive: u32,
                layer : super :: super :: super :: wlr :: wlr_layer_shell_unstable_v1 :: zwlr_layer_shell_v1 :: Layer,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()>;
            #[doc = "A zwlr_layer_surface_v1 has left the surface area."]
            async fn layer_leave(&self, identifier: String) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol allows clients to ask the compositor to capture screen"]
#[doc = "contents to user submitted buffers."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[allow(clippy::module_inception)]
pub mod cosmic_screencopy_unstable_v2 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_manager_v2 {
        use futures_util::SinkExt;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Options : u32 { # [doc = "paint cursors onto captured frames"] const PaintCursors = 1u32 ; } }
        impl TryFrom<u32> for Options {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Options {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_screencopy_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopyManagerV2 {
            const INTERFACE: &'static str = "zcosmic_screencopy_manager_v2";
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
            #[doc = "Create a capturing session for an image source."]
            #[doc = ""]
            #[doc = "If the paint_cursors option is set, cursors shall be composited onto"]
            #[doc = "the captured frame. The cursor shall not be composited onto the frame"]
            #[doc = "if this flag is not set."]
            async fn create_session(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                session: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                options: Options,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_manager_v2#{}.create_session()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(session))
                    .put_object(Some(source))
                    .put_uint(options.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a cursor capturing session for the pointer of an image source."]
            #[doc = ""]
            #[doc = "The options argument has no effect and must be set to 0. This is"]
            #[doc = "intended for any future flags that might be added."]
            async fn create_pointer_cursor_session(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                session: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
                options: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_manager_v2#{}.create_pointer_cursor_session()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(session))
                    .put_object(Some(source))
                    .put_object(Some(pointer))
                    .put_uint(options)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the manager object."]
            #[doc = ""]
            #[doc = "Other objects created via this interface are unaffected."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_screencopy_manager_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
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
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_session_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_screencopy_session_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopySessionV2 {
            const INTERFACE: &'static str = "zcosmic_screencopy_session_v2";
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
            #[doc = "Create a capture frame for this session."]
            async fn create_frame(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_session_v2#{}.create_frame()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect zcosmic_screencopy_frame_v2 objects created by"]
            #[doc = "this object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_screencopy_session_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Provides the dimensions of the source image in buffer pixel coordinates."]
            #[doc = ""]
            #[doc = "The client must attach buffers that match this size."]
            async fn buffer_size(&self, width: u32, height: u32) -> crate::client::Result<()>;
            #[doc = "Provides the format that must be used for shared-memory buffers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            async fn shm_format(&self, format: u32) -> crate::client::Result<()>;
            #[doc = "This event advertises the device buffers must be allocated on for"]
            #[doc = "dma-buf buffers."]
            #[doc = ""]
            #[doc = "In general the device is a DRM node. The DRM node type (primary vs."]
            #[doc = "render) is unspecified. Clients must not rely on the compositor sending"]
            #[doc = "a particular node type. Clients cannot check two devices for equality"]
            #[doc = "by comparing the dev_t value."]
            async fn dmabuf_device(&self, device: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "Provides the format that must be used for dma-buf buffers."]
            #[doc = ""]
            #[doc = "The client may choose any of the modifiers advertised in the array of"]
            #[doc = "64-bit unsigned integers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            async fn dmabuf_format(
                &self,
                format: u32,
                modifiers: Vec<u8>,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent once when all buffer constraint events have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "The compositor must always end a batch of buffer constraint events with"]
            #[doc = "this event, regardless of whether it sends the initial constraints or"]
            #[doc = "an update."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "This event indicates that the capture session has stopped and is no"]
            #[doc = "longer available. This can happen in a number of cases, e.g. when the"]
            #[doc = "underlying source is destroyed, if the user decides to end the screen"]
            #[doc = "capture, or if an unrecoverable runtime error has occurred."]
            #[doc = ""]
            #[doc = "The client should destroy the session after receiving this event."]
            async fn stopped(&self) -> crate::client::Result<()>;
        }
    }
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
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_frame_v2 {
        use futures_util::SinkExt;
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
        impl std::fmt::Display for FailureReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_screencopy_frame_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopyFrameV2 {
            const INTERFACE: &'static str = "zcosmic_screencopy_frame_v2";
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
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_screencopy_frame_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Attach a buffer to the session."]
            #[doc = ""]
            #[doc = "The wl_buffer.release request is unused."]
            #[doc = ""]
            #[doc = "This request must not be sent after capture, or else the"]
            #[doc = "already_captured protocol error is raised."]
            async fn attach_buffer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_frame_v2#{}.attach_buffer()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn damage_buffer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_frame_v2#{}.damage_buffer()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_screencopy_frame_v2#{}.capture()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent before the ready event and holds the transform of"]
            #[doc = "the source buffer."]
            async fn transform(
                &self,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::client::Result<()>;
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
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()>;
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
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading."]
            #[doc = ""]
            #[doc = "The buffer may be re-used by the client after this event."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            async fn ready(&self) -> crate::client::Result<()>;
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            async fn failed(&self, reason: FailureReason) -> crate::client::Result<()>;
        }
    }
    #[doc = "This object represents a cursor capture session. It extends the base"]
    #[doc = "capture session with cursor-specific metadata."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_screencopy_cursor_session_v2 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "get_screencopy_session sent twice"]
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_screencopy_cursor_session_v2 interface. See the module level documentation for more info"]
        pub trait ZcosmicScreencopyCursorSessionV2 {
            const INTERFACE: &'static str = "zcosmic_screencopy_cursor_session_v2";
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
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect zcosmic_screencopy_frame_v2 objects created by"]
            #[doc = "this object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_cursor_session_v2#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Gets the screencopy session for this cursor session."]
            #[doc = ""]
            #[doc = "The session will produce frames of the cursor image. The compositor may"]
            #[doc = "pause the session when the cursor leaves the captured area."]
            #[doc = ""]
            #[doc = "This request must not be sent more than once, or else the"]
            #[doc = "duplicate_session protocol error is raised."]
            async fn get_screencopy_session(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                session: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_screencopy_cursor_session_v2#{}.get_screencopy_session()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(session))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sent when a cursor enters the captured area. It shall be generated"]
            #[doc = "before the \"position\" and \"hotspot\" events when and only when a cursor"]
            #[doc = "enters the area."]
            #[doc = ""]
            #[doc = "The cursor enters the captured area when the cursor image intersects"]
            #[doc = "with the captured area. Note, this is different from e.g."]
            #[doc = "wl_pointer.enter."]
            async fn enter(&self) -> crate::client::Result<()>;
            #[doc = "Sent when a cursor leaves the captured area. No \"position\" or \"hotspot\""]
            #[doc = "event is generated for the cursor until the cursor enters the captured"]
            #[doc = "area again."]
            async fn leave(&self) -> crate::client::Result<()>;
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
            async fn position(&self, x: i32, y: i32) -> crate::client::Result<()>;
            #[doc = "The hotspot describes the offset between the cursor image and the"]
            #[doc = "position of the input device."]
            #[doc = ""]
            #[doc = "The given coordinates are the hotspot's offset from the origin in"]
            #[doc = "buffer coordinates."]
            #[doc = ""]
            #[doc = "Clients should not apply the hotspot immediately: the hotspot becomes"]
            #[doc = "effective when the next zcosmic_screencopy_frame_v2.ready event is received."]
            async fn hotspot(&self, x: i32, y: i32) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_toplevel_info_unstable_v1 {
    #[doc = "The purpose of this protocol is to enable clients such as taskbars"]
    #[doc = "or docks to access a list of opened applications and basic properties"]
    #[doc = "thereof."]
    #[doc = ""]
    #[doc = "It thus extends ext_foreign_toplevel_v1 to provide more information"]
    #[doc = "and actions on foreign toplevels."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_info_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zcosmic_toplevel_info_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelInfoV1 {
            const INTERFACE: &'static str = "zcosmic_toplevel_info_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events for new toplevels.  However, the compositor may emit further"]
            #[doc = "toplevel_created events until the finished event is emitted."]
            #[doc = ""]
            #[doc = "The client must not send any more requests after this one."]
            #[doc = ""]
            #[doc = "Note: This request isn't necessary for clients binding version 2"]
            #[doc = "of this protocol and will be ignored."]
            async fn stop(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_toplevel_info_v1#{}.stop()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request a zcosmic_toplevel_handle_v1 extension object for an existing"]
            #[doc = "ext_foreign_toplevel_handle_v1."]
            #[doc = ""]
            #[doc = "All initial properties of the toplevel (states, etc.)"]
            #[doc = "will be sent immediately after this event via the corresponding"]
            #[doc = "events in zcosmic_toplevel_handle_v1."]
            async fn get_cosmic_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                cosmic_toplevel: crate::wire::ObjectId,
                foreign_toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_info_v1#{}.get_cosmic_toplevel()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(cosmic_toplevel))
                    .put_object(Some(foreign_toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn toplevel(&self, toplevel: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event indicates that the compositor is done sending events"]
            #[doc = "to the zcosmic_toplevel_info_v1. The server will destroy the"]
            #[doc = "object immediately after sending this request, so it will become"]
            #[doc = "invalid and the client should free any resources associated with it."]
            #[doc = ""]
            #[doc = "Note: This event is emitted immediately after calling `stop` for"]
            #[doc = "clients binding version 2 of this protocol for backwards compatibility."]
            async fn finished(&self) -> crate::client::Result<()>;
            #[doc = "This event is sent after all changes for currently active"]
            #[doc = "zcosmic_toplevel_handle_v1 have been sent."]
            #[doc = ""]
            #[doc = "This allows changes to multiple zcosmic_toplevel_handle_v1 handles"]
            #[doc = "and their properties to be seen as atomic, even if they happen via"]
            #[doc = "multiple events."]
            async fn done(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "A zcosmic_toplevel_handle_v1 object represents an open toplevel"]
    #[doc = "window. A single app may have multiple open toplevels."]
    #[doc = ""]
    #[doc = "Each toplevel has a list of outputs it is visible on, exposed to the"]
    #[doc = "client via the output_enter and output_leave events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_handle_v1 {
        use futures_util::SinkExt;
        #[doc = "The different states that a toplevel may have. These have the same"]
        #[doc = "meaning as the states with the same names defined in xdg-toplevel"]
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Maximized),
                    1u32 => Ok(Self::Minimized),
                    2u32 => Ok(Self::Activated),
                    3u32 => Ok(Self::Fullscreen),
                    4u32 => Ok(Self::Sticky),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zcosmic_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelHandleV1 {
            const INTERFACE: &'static str = "zcosmic_toplevel_handle_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request should be called either when the client will no longer"]
            #[doc = "use the zcosmic_toplevel_handle_v1."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_toplevel_handle_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The server will emit no further events on the"]
            #[doc = "zcosmic_toplevel_handle_v1 after this event. Any requests received"]
            #[doc = "aside from the destroy request will be ignored. Upon receiving this"]
            #[doc = "event, the client should make the destroy request to allow freeing"]
            #[doc = "of resources."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.closed` is"]
            #[doc = "equivalent."]
            async fn closed(&self) -> crate::client::Result<()>;
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
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the title of the toplevel changes."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.title` is"]
            #[doc = "equivalent."]
            async fn title(&self, title: String) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the app_id of the toplevel changes."]
            #[doc = ""]
            #[doc = "Note: This event will not be emitted for clients binding version 2"]
            #[doc = "of this protocol, as `ext_foreign_toplevel_handle_v1.app_id` is"]
            #[doc = "equivalent."]
            async fn app_id(&self, app_id: String) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel becomes visible on the"]
            #[doc = "given output. A toplevel may be visible on multiple outputs."]
            async fn output_enter(
                &self,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel is no longer visible"]
            #[doc = "on a given output. It is guaranteed that an output_enter event with"]
            #[doc = "the same output has been emitted before this event."]
            async fn output_leave(
                &self,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel becomes visible on the"]
            #[doc = "given workspace. A toplevel may be visible on multiple workspaces."]
            async fn workspace_enter(
                &self,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel is no longer visible"]
            #[doc = "on a given workspace. It is guaranteed that an workspace_enter event with"]
            #[doc = "the same workspace has been emitted before this event."]
            async fn workspace_leave(
                &self,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted once on creation of the"]
            #[doc = "zcosmic_toplevel_handle_v1 and again whenever the state of the"]
            #[doc = "toplevel changes."]
            async fn state(&self, state: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "Emitted when the geometry of a toplevel (it's position and/or size)"]
            #[doc = "relative to the provided output has changed."]
            #[doc = ""]
            #[doc = "This event is emitted once on creation of the"]
            #[doc = "zcosmic_toplevel_handle_v1 for every entered output and again"]
            #[doc = "whenever the geometry of the toplevel changes relative to any output."]
            async fn geometry(
                &self,
                output: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel becomes visible on the"]
            #[doc = "given workspace. A toplevel may be visible on multiple workspaces."]
            async fn ext_workspace_enter(
                &self,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "This event is emitted whenever the toplevel is no longer visible"]
            #[doc = "on a given workspace. It is guaranteed that an workspace_enter event with"]
            #[doc = "the same workspace has been emitted before this event."]
            async fn ext_workspace_leave(
                &self,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cosmic_toplevel_management_unstable_v1 {
    #[doc = "This protocol allows clients such as a taskbar to request the compositor"]
    #[doc = "to preform typical actions on open toplevels. The compositor is in all"]
    #[doc = "cases free to ignore the request."]
    #[allow(clippy::too_many_arguments)]
    pub mod zcosmic_toplevel_manager_v1 {
        use futures_util::SinkExt;
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
            type Error = crate::wire::DecodeError;
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
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
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
        #[doc = "Trait to implement the zcosmic_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZcosmicToplevelManagerV1 {
            const INTERFACE: &'static str = "zcosmic_toplevel_manager_v1";
            const VERSION: u32 = 4u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request indicates that the client has finished using the"]
            #[doc = "zcosmic_toplevel_manager_v1 object and that it can be safely"]
            #[doc = "destroyed."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_toplevel_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.closed event will be sent."]
            async fn close(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_toplevel_manager_v1#{}.close()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn activate(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_toplevel_manager_v1#{}.activate()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn set_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.set_maximized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn unset_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.unset_maximized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn set_minimized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.set_minimized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn unset_minimized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.unset_minimized()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state and potentially the"]
            #[doc = "zcosmic_toplevel_handle_v1.output_enter/output_leave events will"]
            #[doc = "be sent."]
            #[doc = ""]
            #[doc = "The output parameter a hint to the compositor and may be ignored. A"]
            #[doc = "value of NULL indicates that the compositor should choose the target"]
            #[doc = "output, if it honors the fullscreen request."]
            async fn set_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.set_fullscreen()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn unset_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.unset_fullscreen()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn set_rectangle(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.set_rectangle()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_object(Some(surface))
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Move window to workspace, on given output."]
            async fn move_to_workspace(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                workspace: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.move_to_workspace()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_object(Some(workspace))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn set_sticky(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zcosmic_toplevel_manager_v1#{}.set_sticky()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 11u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "If the compositor honors this request, the"]
            #[doc = "zcosmic_toplevel_handle_v1.state event will be sent."]
            async fn unset_sticky(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.unset_sticky()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 12u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Move window to workspace, on given output."]
            async fn move_to_ext_workspace(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                workspace: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zcosmic_toplevel_manager_v1#{}.move_to_ext_workspace()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_object(Some(workspace))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 13u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn capabilities(&self, capabilities: Vec<u8>) -> crate::client::Result<()>;
        }
    }
}
