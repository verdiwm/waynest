#![allow(async_fn_in_trait)]
#[allow(clippy::module_inception)]
pub mod fullscreen_shell_unstable_v1 {
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
    #[doc = "Requesting a surface to be presented on an output that already has a"]
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
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_fullscreen_shell_v1 {
        use futures_util::SinkExt;
        #[doc = "Various capabilities that can be advertised by the compositor.  They"]
        #[doc = "are advertised one-at-a-time when the wl_fullscreen_shell interface is"]
        #[doc = "bound.  See the wl_fullscreen_shell.capability event for more details."]
        #[doc = ""]
        #[doc = "ARBITRARY_MODES:"]
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
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for PresentMethod {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These errors can be emitted in response to wl_fullscreen_shell requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "present_method is not known"]
            InvalidMethod = 0u32,
            #[doc = "given wl_surface has another role"]
            Role = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidMethod),
                    1u32 => Ok(Self::Role),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_fullscreen_shell_v1 interface. See the module level documentation for more info"]
        pub trait ZwpFullscreenShellV1 {
            const INTERFACE: &'static str = "zwp_fullscreen_shell_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Release the binding from the wl_fullscreen_shell interface."]
            #[doc = ""]
            #[doc = "This destroys the server-side object and frees this binding.  If"]
            #[doc = "the client binds to wl_fullscreen_shell multiple times, it may wish"]
            #[doc = "to free some of those bindings."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_fullscreen_shell_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Present a surface on the given output."]
            #[doc = ""]
            #[doc = "If the output is null, the compositor will present the surface on"]
            #[doc = "whatever display (or displays) it thinks best.  In particular, this"]
            #[doc = "may replace any or all surfaces currently presented so it should"]
            #[doc = "not be used in combination with placing surfaces on specific"]
            #[doc = "outputs."]
            #[doc = ""]
            #[doc = "The method parameter is a hint to the compositor for how the surface"]
            #[doc = "is to be presented.  In particular, it tells the compositor how to"]
            #[doc = "handle a size mismatch between the presented surface and the"]
            #[doc = "output.  The compositor is free to ignore this parameter."]
            #[doc = ""]
            #[doc = "The \"zoom\", \"zoom_crop\", and \"stretch\" methods imply a scaling"]
            #[doc = "operation on the surface.  This will override any kind of output"]
            #[doc = "scaling, so the buffer_scale property of the surface is effectively"]
            #[doc = "ignored."]
            #[doc = ""]
            #[doc = "This request gives the surface the role of a fullscreen shell surface."]
            #[doc = "If the surface already has another role, it raises a role protocol"]
            #[doc = "error."]
            async fn present_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: Option<crate::wire::ObjectId>,
                method: PresentMethod,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_fullscreen_shell_v1#{}.present_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(surface)
                    .put_uint(method as u32)
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            #[doc = ""]
            #[doc = "This request gives the surface the role of a fullscreen shell surface."]
            #[doc = "If the surface already has another role, it raises a role protocol"]
            #[doc = "error."]
            async fn present_surface_for_mode(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                framerate: i32,
                feedback: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_fullscreen_shell_v1#{}.present_surface_for_mode()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .put_object(Some(output))
                    .put_int(framerate)
                    .put_object(Some(feedback))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Advertises a single capability of the compositor."]
            #[doc = ""]
            #[doc = "When the wl_fullscreen_shell interface is bound, this event is emitted"]
            #[doc = "once for each capability advertised.  Valid capabilities are given by"]
            #[doc = "the wl_fullscreen_shell.capability enum.  If clients want to take"]
            #[doc = "advantage of any of these capabilities, they should use a"]
            #[doc = "wl_display.sync request immediately after binding to ensure that they"]
            #[doc = "receive all the capability events."]
            async fn capability(&self, capability: Capability) -> crate::client::Result<()>;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_fullscreen_shell_mode_feedback_v1 {
        #[doc = "Trait to implement the zwp_fullscreen_shell_mode_feedback_v1 interface. See the module level documentation for more info"]
        pub trait ZwpFullscreenShellModeFeedbackV1 {
            const INTERFACE: &'static str = "zwp_fullscreen_shell_mode_feedback_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This event indicates that the attempted mode switch operation was"]
            #[doc = "successful.  A surface of the size requested in the mode switch"]
            #[doc = "will fill the output without scaling."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            async fn mode_successful(&self) -> crate::client::Result<()>;
            #[doc = "This event indicates that the attempted mode switch operation"]
            #[doc = "failed.  This may be because the requested output mode is not"]
            #[doc = "possible or it may mean that the compositor does not want to allow it."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            async fn mode_failed(&self) -> crate::client::Result<()>;
            #[doc = "This event indicates that the attempted mode switch operation was"]
            #[doc = "cancelled.  Most likely this is because the client requested a"]
            #[doc = "second mode switch before the first one completed."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            async fn present_cancelled(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol aims at describing outputs in a way which is more in line"]
#[doc = "with the concept of an output on desktop oriented systems."]
#[doc = ""]
#[doc = "Some information are more specific to the concept of an output for"]
#[doc = "a desktop oriented system and may not make sense in other applications,"]
#[doc = "such as IVI systems for example."]
#[doc = ""]
#[doc = "Typically, the global compositor space on a desktop system is made of"]
#[doc = "a contiguous or overlapping set of rectangular regions."]
#[doc = ""]
#[doc = "The logical_position and logical_size events defined in this protocol"]
#[doc = "might provide information identical to their counterparts already"]
#[doc = "available from wl_output, in which case the information provided by this"]
#[doc = "protocol should be preferred to their equivalent in wl_output. The goal is"]
#[doc = "to move the desktop specific concepts (such as output location within the"]
#[doc = "global compositor space, etc.) out of the core wl_output protocol."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible"]
#[doc = "changes may be added together with the corresponding interface"]
#[doc = "version bump."]
#[doc = "Backward incompatible changes are done by bumping the version"]
#[doc = "number in the protocol and interface names and resetting the"]
#[doc = "interface version. Once the protocol is to be declared stable,"]
#[doc = "the 'z' prefix and the version number in the protocol and"]
#[doc = "interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod xdg_output_unstable_v1 {
    #[doc = "A global factory interface for xdg_output objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_output_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgOutputManagerV1 {
            const INTERFACE: &'static str = "zxdg_output_manager_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Using this request a client can tell the server that it is not"]
            #[doc = "going to use the xdg_output_manager object anymore."]
            #[doc = ""]
            #[doc = "Any objects already created through this instance are not affected."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_output_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates a new xdg_output object for the given wl_output."]
            async fn get_xdg_output(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_output_manager_v1#{}.get_xdg_output()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(output))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An xdg_output describes part of the compositor geometry."]
    #[doc = ""]
    #[doc = "This typically corresponds to a monitor that displays part of the"]
    #[doc = "compositor space."]
    #[doc = ""]
    #[doc = "For objects version 3 onwards, after all xdg_output properties have been"]
    #[doc = "sent (when the object is created and when properties are updated), a"]
    #[doc = "wl_output.done event is sent. This allows changes to the output"]
    #[doc = "properties to be seen as atomic, even if they happen via multiple events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_output_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_output_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgOutputV1 {
            const INTERFACE: &'static str = "zxdg_output_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Using this request a client can tell the server that it is not"]
            #[doc = "going to use the xdg_output object anymore."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_output_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The position event describes the location of the wl_output within"]
            #[doc = "the global compositor space."]
            #[doc = ""]
            #[doc = "The logical_position event is sent after creating an xdg_output"]
            #[doc = "(see xdg_output_manager.get_xdg_output) and whenever the location"]
            #[doc = "of the output changes within the global compositor space."]
            async fn logical_position(&self, x: i32, y: i32) -> crate::client::Result<()>;
            #[doc = "The logical_size event describes the size of the output in the"]
            #[doc = "global compositor space."]
            #[doc = ""]
            #[doc = "Most regular Wayland clients should not pay attention to the"]
            #[doc = "logical size and would rather rely on xdg_shell interfaces."]
            #[doc = ""]
            #[doc = "Some clients such as Xwayland, however, need this to configure"]
            #[doc = "their surfaces in the global compositor space as the compositor"]
            #[doc = "may apply a different scale from what is advertised by the output"]
            #[doc = "scaling property (to achieve fractional scaling, for example)."]
            #[doc = ""]
            #[doc = "For example, for a wl_output mode 3840×2160 and a scale factor 2:"]
            #[doc = ""]
            #[doc = "- A compositor not scaling the monitor viewport in its compositing space"]
            #[doc = "will advertise a logical size of 3840×2160,"]
            #[doc = ""]
            #[doc = "- A compositor scaling the monitor viewport with scale factor 2 will"]
            #[doc = "advertise a logical size of 1920×1080,"]
            #[doc = ""]
            #[doc = "- A compositor scaling the monitor viewport using a fractional scale of"]
            #[doc = "1.5 will advertise a logical size of 2560×1440."]
            #[doc = ""]
            #[doc = "For example, for a wl_output mode 1920×1080 and a 90 degree rotation,"]
            #[doc = "the compositor will advertise a logical size of 1080x1920."]
            #[doc = ""]
            #[doc = "The logical_size event is sent after creating an xdg_output"]
            #[doc = "(see xdg_output_manager.get_xdg_output) and whenever the logical"]
            #[doc = "size of the output changes, either as a result of a change in the"]
            #[doc = "applied scale or because of a change in the corresponding output"]
            #[doc = "mode(see wl_output.mode) or transform (see wl_output.transform)."]
            async fn logical_size(&self, width: i32, height: i32) -> crate::client::Result<()>;
            #[doc = "This event is sent after all other properties of an xdg_output"]
            #[doc = "have been sent."]
            #[doc = ""]
            #[doc = "This allows changes to the xdg_output properties to be seen as"]
            #[doc = "atomic, even if they happen via multiple events."]
            #[doc = ""]
            #[doc = "For objects version 3 onwards, this event is deprecated. Compositors"]
            #[doc = "are not required to send it anymore and must send wl_output.done"]
            #[doc = "instead."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "Many compositors will assign names to their outputs, show them to the"]
            #[doc = "user, allow them to be configured by name, etc. The client may wish to"]
            #[doc = "know this name as well to offer the user similar behaviors."]
            #[doc = ""]
            #[doc = "The naming convention is compositor defined, but limited to"]
            #[doc = "alphanumeric characters and dashes (-). Each name is unique among all"]
            #[doc = "wl_output globals, but if a wl_output global is destroyed the same name"]
            #[doc = "may be reused later. The names will also remain consistent across"]
            #[doc = "sessions with the same hardware and software configuration."]
            #[doc = ""]
            #[doc = "Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do"]
            #[doc = "not assume that the name is a reflection of an underlying DRM"]
            #[doc = "connector, X11 connection, etc."]
            #[doc = ""]
            #[doc = "The name event is sent after creating an xdg_output (see"]
            #[doc = "xdg_output_manager.get_xdg_output). This event is only sent once per"]
            #[doc = "xdg_output, and the name does not change over the lifetime of the"]
            #[doc = "wl_output global."]
            #[doc = ""]
            #[doc = "This event is deprecated, instead clients should use wl_output.name."]
            #[doc = "Compositors must still support this event."]
            async fn name(&self, name: String) -> crate::client::Result<()>;
            #[doc = "Many compositors can produce human-readable descriptions of their"]
            #[doc = "outputs.  The client may wish to know this description as well, to"]
            #[doc = "communicate the user for various purposes."]
            #[doc = ""]
            #[doc = "The description is a UTF-8 string with no convention defined for its"]
            #[doc = "contents. Examples might include 'Foocorp 11\" Display' or 'Virtual X11"]
            #[doc = "output via :1'."]
            #[doc = ""]
            #[doc = "The description event is sent after creating an xdg_output (see"]
            #[doc = "xdg_output_manager.get_xdg_output) and whenever the description"]
            #[doc = "changes. The description is optional, and may not be sent at all."]
            #[doc = ""]
            #[doc = "For objects of version 2 and lower, this event is only sent once per"]
            #[doc = "xdg_output, and the description does not change over the lifetime of"]
            #[doc = "the wl_output global."]
            #[doc = ""]
            #[doc = "This event is deprecated, instead clients should use"]
            #[doc = "wl_output.description. Compositors must still support this event."]
            async fn description(&self, description: String) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_shell_unstable_v5 {
    #[doc = "xdg_shell allows clients to turn a wl_surface into a \"real window\""]
    #[doc = "which can be dragged, resized, stacked, and moved around by the"]
    #[doc = "user. Everything about this interface is suited towards traditional"]
    #[doc = "desktop environments."]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_shell {
        use futures_util::SinkExt;
        #[doc = "The 'current' member of this enum gives the version of the"]
        #[doc = "protocol.  Implementations can compare this to the version"]
        #[doc = "they implement using static_assert to ensure the protocol and"]
        #[doc = "implementation versions match."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Version {
            #[doc = "Always the latest version"]
            Current = 5u32,
        }
        impl TryFrom<u32> for Version {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    5u32 => Ok(Self::Current),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Version {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "xdg_shell was destroyed before children"]
            DefunctSurfaces = 1u32,
            #[doc = "the client tried to map or destroy a non-topmost popup"]
            NotTheTopmostPopup = 2u32,
            #[doc = "the client specified an invalid popup parent surface"]
            InvalidPopupParent = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::DefunctSurfaces),
                    2u32 => Ok(Self::NotTheTopmostPopup),
                    3u32 => Ok(Self::InvalidPopupParent),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_shell interface. See the module level documentation for more info"]
        pub trait XdgShell {
            const INTERFACE: &'static str = "xdg_shell";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this xdg_shell object."]
            #[doc = ""]
            #[doc = "Destroying a bound xdg_shell object while there are surfaces"]
            #[doc = "still alive created by this xdg_shell object instance is illegal"]
            #[doc = "and will result in a protocol error."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_shell#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Negotiate the unstable version of the interface.  This"]
            #[doc = "mechanism is in place to ensure client and server agree on the"]
            #[doc = "unstable versions of the protocol that they speak or exit"]
            #[doc = "cleanly if they don't agree.  This request will go away once"]
            #[doc = "the xdg-shell protocol is stable."]
            async fn use_unstable_version(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                version: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_shell#{}.use_unstable_version()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(version).build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates an xdg_surface for the given surface and gives it the"]
            #[doc = "xdg_surface role. A wl_surface can only be given an xdg_surface role"]
            #[doc = "once. If get_xdg_surface is called with a wl_surface that already has"]
            #[doc = "an active xdg_surface associated with it, or if it had any other role,"]
            #[doc = "an error is raised."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_surface for more details about what an"]
            #[doc = "xdg_surface is and how it is used."]
            async fn get_xdg_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_shell#{}.get_xdg_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates an xdg_popup for the given surface and gives it the"]
            #[doc = "xdg_popup role. A wl_surface can only be given an xdg_popup role"]
            #[doc = "once. If get_xdg_popup is called with a wl_surface that already has"]
            #[doc = "an active xdg_popup associated with it, or if it had any other role,"]
            #[doc = "an error is raised."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_popup for more details about what an"]
            #[doc = "xdg_popup is and how it is used."]
            async fn get_xdg_popup(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                parent: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_shell#{}.get_xdg_popup()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(parent))
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "A client must respond to a ping event with a pong request or"]
            #[doc = "the client may be deemed unresponsive."]
            async fn pong(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_shell#{}.pong()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The ping event asks the client if it's still alive. Pass the"]
            #[doc = "serial specified in the event back to the compositor by sending"]
            #[doc = "a \"pong\" request back with the specified serial."]
            #[doc = ""]
            #[doc = "Compositors can use this to determine if the client is still"]
            #[doc = "alive. It's unspecified what will happen if the client doesn't"]
            #[doc = "respond to the ping request, or in what timeframe. Clients should"]
            #[doc = "try to respond in a reasonable amount of time."]
            #[doc = ""]
            #[doc = "A compositor is free to ping in any way it wants, but a client must"]
            #[doc = "always respond to any xdg_shell object it created."]
            async fn ping(&self, serial: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide a desktop-style user interface."]
    #[doc = ""]
    #[doc = "It provides requests to treat surfaces like windows, allowing to set"]
    #[doc = "properties like maximized, fullscreen, minimized, and to move and resize"]
    #[doc = "them, and associate metadata like title and app id."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xdg_surface state to take effect. Prior to committing the new"]
    #[doc = "state, it can set up initial configuration, such as maximizing or setting"]
    #[doc = "a window geometry."]
    #[doc = ""]
    #[doc = "Even without attaching a buffer the compositor must respond to initial"]
    #[doc = "committed configuration, for instance sending a configure event with"]
    #[doc = "expected window geometry if the client maximized its surface during"]
    #[doc = "initialization."]
    #[doc = ""]
    #[doc = "For a surface to be mapped by the compositor the client must have"]
    #[doc = "committed both an xdg_surface state and a buffer."]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_surface {
        use futures_util::SinkExt;
        #[doc = "These values are used to indicate which edge of a surface"]
        #[doc = "is being dragged in a resize operation."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ResizeEdge {
            None = 0u32,
            Top = 1u32,
            Bottom = 2u32,
            Left = 4u32,
            TopLeft = 5u32,
            BottomLeft = 6u32,
            Right = 8u32,
            TopRight = 9u32,
            BottomRight = 10u32,
        }
        impl TryFrom<u32> for ResizeEdge {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Bottom),
                    4u32 => Ok(Self::Left),
                    5u32 => Ok(Self::TopLeft),
                    6u32 => Ok(Self::BottomLeft),
                    8u32 => Ok(Self::Right),
                    9u32 => Ok(Self::TopRight),
                    10u32 => Ok(Self::BottomRight),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ResizeEdge {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "The different state values used on the surface. This is designed for"]
        #[doc = "state values like maximized, fullscreen. It is paired with the"]
        #[doc = "configure event to ensure that both the client and the compositor"]
        #[doc = "setting the state can be synchronized."]
        #[doc = ""]
        #[doc = "States set in this way are double-buffered, see wl_surface.commit."]
        #[doc = ""]
        #[doc = "Desktop environments may extend this enum by taking up a range of"]
        #[doc = "values and documenting the range they chose in this description."]
        #[doc = "They are not required to document the values for the range that they"]
        #[doc = "chose. Ideally, any good extensions from a desktop environment should"]
        #[doc = "make its way into standardization into this enum."]
        #[doc = ""]
        #[doc = "The current reserved ranges are:"]
        #[doc = ""]
        #[doc = "0x0000 - 0x0FFF: xdg-shell core values, documented below."]
        #[doc = "0x1000 - 0x1FFF: GNOME"]
        #[doc = "0x2000 - 0x2FFF: EFL"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            #[doc = "the surface is maximized"]
            Maximized = 1u32,
            #[doc = "the surface is fullscreen"]
            Fullscreen = 2u32,
            #[doc = "the surface is being resized"]
            Resizing = 3u32,
            #[doc = "the surface is now activated"]
            Activated = 4u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Maximized),
                    2u32 => Ok(Self::Fullscreen),
                    3u32 => Ok(Self::Resizing),
                    4u32 => Ok(Self::Activated),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_surface interface. See the module level documentation for more info"]
        pub trait XdgSurface {
            const INTERFACE: &'static str = "xdg_surface";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Unmap and destroy the window. The window will be effectively"]
            #[doc = "hidden from the user's point of view, and all state like"]
            #[doc = "maximization, fullscreen, and so on, will be lost."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the \"parent\" of this surface. This window should be stacked"]
            #[doc = "above a parent. The parent surface must be mapped as long as this"]
            #[doc = "surface is mapped."]
            #[doc = ""]
            #[doc = "Parent windows should be set on dialogs, toolboxes, or other"]
            #[doc = "\"auxiliary\" surfaces, so that the parent is raised when the dialog"]
            #[doc = "is raised."]
            async fn set_parent(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                parent: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_parent()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(parent)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a short title for the surface."]
            #[doc = ""]
            #[doc = "This string may be used to identify the surface in a task bar,"]
            #[doc = "window list, or other user interface elements provided by the"]
            #[doc = "compositor."]
            #[doc = ""]
            #[doc = "The string must be encoded in UTF-8."]
            async fn set_title(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                title: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_title()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(title))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set an application identifier for the surface."]
            #[doc = ""]
            #[doc = "The app ID identifies the general class of applications to which"]
            #[doc = "the surface belongs. The compositor can use this to group multiple"]
            #[doc = "surfaces together, or to determine how to launch a new application."]
            #[doc = ""]
            #[doc = "For D-Bus activatable applications, the app ID is used as the D-Bus"]
            #[doc = "service name."]
            #[doc = ""]
            #[doc = "The compositor shell will try to group application surfaces together"]
            #[doc = "by their app ID.  As a best practice, it is suggested to select app"]
            #[doc = "ID's that match the basename of the application's .desktop file."]
            #[doc = "For example, \"org.freedesktop.FooViewer\" where the .desktop file is"]
            #[doc = "\"org.freedesktop.FooViewer.desktop\"."]
            #[doc = ""]
            #[doc = "See the desktop-entry specification [0] for more details on"]
            #[doc = "application identifiers and how they relate to well-known D-Bus"]
            #[doc = "names and .desktop files."]
            #[doc = ""]
            #[doc = "[0] http://standards.freedesktop.org/desktop-entry-spec/"]
            async fn set_app_id(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                app_id: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_app_id()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Clients implementing client-side decorations might want to show"]
            #[doc = "a context menu when right-clicking on the decorations, giving the"]
            #[doc = "user a menu that they can use to maximize or minimize the window."]
            #[doc = ""]
            #[doc = "This request asks the compositor to pop up such a window menu at"]
            #[doc = "the given position, relative to the local surface coordinates of"]
            #[doc = "the parent surface. There are no guarantees as to what menu items"]
            #[doc = "the window menu contains."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event."]
            async fn show_window_menu(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.show_window_menu()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Start an interactive, user-driven move of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event. The passed"]
            #[doc = "serial is used to determine the type of interactive move (touch,"]
            #[doc = "pointer, etc)."]
            #[doc = ""]
            #[doc = "The server may ignore move requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized), or if the passed serial"]
            #[doc = "is no longer valid."]
            #[doc = ""]
            #[doc = "If triggered, the surface will lose the focus of the device"]
            #[doc = "(wl_pointer, wl_touch, etc) used for the move. It is up to the"]
            #[doc = "compositor to visually indicate that the move is taking place, such as"]
            #[doc = "updating a pointer cursor, during the move. There is no guarantee"]
            #[doc = "that the device focus will return when the move is completed."]
            async fn r#move(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.move()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Start a user-driven, interactive resize of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event. The passed"]
            #[doc = "serial is used to determine the type of interactive resize (touch,"]
            #[doc = "pointer, etc)."]
            #[doc = ""]
            #[doc = "The server may ignore resize requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized)."]
            #[doc = ""]
            #[doc = "If triggered, the client will receive configure events with the"]
            #[doc = "\"resize\" state enum value and the expected sizes. See the \"resize\""]
            #[doc = "enum value for more details about what is required. The client"]
            #[doc = "must also acknowledge configure events using \"ack_configure\". After"]
            #[doc = "the resize is completed, the client will receive another \"configure\""]
            #[doc = "event without the resize state."]
            #[doc = ""]
            #[doc = "If triggered, the surface also will lose the focus of the device"]
            #[doc = "(wl_pointer, wl_touch, etc) used for the resize. It is up to the"]
            #[doc = "compositor to visually indicate that the resize is taking place,"]
            #[doc = "such as updating a pointer cursor, during the resize. There is no"]
            #[doc = "guarantee that the device focus will return when the resize is"]
            #[doc = "completed."]
            #[doc = ""]
            #[doc = "The edges parameter specifies how the surface should be resized,"]
            #[doc = "and is one of the values of the resize_edge enum. The compositor"]
            #[doc = "may use this information to update the surface position for"]
            #[doc = "example when dragging the top left corner. The compositor may also"]
            #[doc = "use this information to adapt its behavior, e.g. choose an"]
            #[doc = "appropriate cursor image."]
            async fn resize(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                edges: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.resize()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_uint(edges)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "When a configure event is received, if a client commits the"]
            #[doc = "surface in response to the configure event, then the client"]
            #[doc = "must make an ack_configure request sometime before the commit"]
            #[doc = "request, passing along the serial of the configure event."]
            #[doc = ""]
            #[doc = "For instance, the compositor might use this information to move"]
            #[doc = "a surface to the top left only when the client has drawn itself"]
            #[doc = "for the maximized or fullscreen state."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it"]
            #[doc = "can respond to one, it only has to ack the last configure event."]
            #[doc = ""]
            #[doc = "A client is not required to commit immediately after sending"]
            #[doc = "an ack_configure request - it may even ack_configure several times"]
            #[doc = "before its next surface commit."]
            #[doc = ""]
            #[doc = "The compositor expects that the most recently received"]
            #[doc = "ack_configure request at the time of a commit indicates which"]
            #[doc = "configure event the client is responding to."]
            async fn ack_configure(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.ack_configure()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The window geometry of a window is its \"visible bounds\" from the"]
            #[doc = "user's perspective. Client-side decorations often have invisible"]
            #[doc = "portions like drop-shadows which should be ignored for the"]
            #[doc = "purposes of aligning, placing and constraining windows."]
            #[doc = ""]
            #[doc = "The window geometry is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "Once the window geometry of the surface is set once, it is not"]
            #[doc = "possible to unset it, and it will remain the same until"]
            #[doc = "set_window_geometry is called again, even if a new subsurface or"]
            #[doc = "buffer is attached."]
            #[doc = ""]
            #[doc = "If never set, the value is the full bounds of the surface,"]
            #[doc = "including any subsurfaces. This updates dynamically on every"]
            #[doc = "commit. This unset mode is meant for extremely simple clients."]
            #[doc = ""]
            #[doc = "If responding to a configure event, the window geometry in here"]
            #[doc = "must respect the sizing negotiations specified by the states in"]
            #[doc = "the configure event."]
            #[doc = ""]
            #[doc = "The arguments are given in the surface local coordinate space of"]
            #[doc = "the wl_surface associated with this xdg_surface."]
            #[doc = ""]
            #[doc = "The width and height must be greater than zero."]
            async fn set_window_geometry(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_window_geometry()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Maximize the surface."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be maximized, the compositor"]
            #[doc = "will respond by emitting a configure event with the \"maximized\" state"]
            #[doc = "and the required window geometry. The client should then update its"]
            #[doc = "content, drawing it in a maximized state, i.e. without shadow or other"]
            #[doc = "decoration outside of the window geometry. The client must also"]
            #[doc = "acknowledge the configure when committing the new content (see"]
            #[doc = "ack_configure)."]
            #[doc = ""]
            #[doc = "It is up to the compositor to decide how and where to maximize the"]
            #[doc = "surface, for example which output and what region of the screen should"]
            #[doc = "be used."]
            #[doc = ""]
            #[doc = "If the surface was already maximized, the compositor will still emit"]
            #[doc = "a configure event with the \"maximized\" state."]
            #[doc = ""]
            #[doc = "Note that unrelated compositor side state changes may cause"]
            #[doc = "configure events to be emitted at any time, meaning trying to"]
            #[doc = "match this request to a specific future configure event is"]
            #[doc = "futile."]
            async fn set_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Unmaximize the surface."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be unmaximized, the compositor"]
            #[doc = "will respond by emitting a configure event without the \"maximized\""]
            #[doc = "state. If available, the compositor will include the window geometry"]
            #[doc = "dimensions the window had prior to being maximized in the configure"]
            #[doc = "request. The client must then update its content, drawing it in a"]
            #[doc = "regular state, i.e. potentially with shadow, etc. The client must also"]
            #[doc = "acknowledge the configure when committing the new content (see"]
            #[doc = "ack_configure)."]
            #[doc = ""]
            #[doc = "It is up to the compositor to position the surface after it was"]
            #[doc = "unmaximized; usually the position the surface had before maximizing, if"]
            #[doc = "applicable."]
            #[doc = ""]
            #[doc = "If the surface was already not maximized, the compositor will still"]
            #[doc = "emit a configure event without the \"maximized\" state."]
            #[doc = ""]
            #[doc = "Note that unrelated compositor side state changes may cause"]
            #[doc = "configure events to be emitted at any time, meaning trying to"]
            #[doc = "match this request to a specific future configure event is"]
            #[doc = "futile."]
            async fn unset_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.unset_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Make the surface fullscreen."]
            #[doc = ""]
            #[doc = "You can specify an output that you would prefer to be fullscreen."]
            #[doc = "If this value is NULL, it's up to the compositor to choose which"]
            #[doc = "display will be used to map this surface."]
            #[doc = ""]
            #[doc = "If the surface doesn't cover the whole output, the compositor will"]
            #[doc = "position the surface in the center of the output and compensate with"]
            #[doc = "black borders filling the rest of the output."]
            async fn set_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_fullscreen()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 11u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn unset_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.unset_fullscreen()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 12u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request that the compositor minimize your surface. There is no"]
            #[doc = "way to know if the surface is currently minimized, nor is there"]
            #[doc = "any way to unset minimization on this surface."]
            #[doc = ""]
            #[doc = "If you are looking to throttle redrawing when minimized, please"]
            #[doc = "instead use the wl_surface.frame event for this, as this will"]
            #[doc = "also work with live previews on windows in Alt-Tab, Expose or"]
            #[doc = "similar compositor features."]
            async fn set_minimized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.set_minimized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 13u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The configure event asks the client to resize its surface or to"]
            #[doc = "change its state."]
            #[doc = ""]
            #[doc = "The width and height arguments specify a hint to the window"]
            #[doc = "about how its surface should be resized in window geometry"]
            #[doc = "coordinates. See set_window_geometry."]
            #[doc = ""]
            #[doc = "If the width or height arguments are zero, it means the client"]
            #[doc = "should decide its own window dimension. This may happen when the"]
            #[doc = "compositor need to configure the state of the surface but doesn't"]
            #[doc = "have any information about any previous or expected dimension."]
            #[doc = ""]
            #[doc = "The states listed in the event specify how the width/height"]
            #[doc = "arguments should be interpreted, and possibly how it should be"]
            #[doc = "drawn."]
            #[doc = ""]
            #[doc = "Clients should arrange their surface for the new size and"]
            #[doc = "states, and then send a ack_configure request with the serial"]
            #[doc = "sent in this configure event at some point before committing"]
            #[doc = "the new surface."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it"]
            #[doc = "can respond to one, it is free to discard all but the last"]
            #[doc = "event it received."]
            async fn configure(
                &self,
                width: i32,
                height: i32,
                states: Vec<u8>,
                serial: u32,
            ) -> crate::client::Result<()>;
            #[doc = "The close event is sent by the compositor when the user"]
            #[doc = "wants the surface to be closed. This should be equivalent to"]
            #[doc = "the user clicking the close button in client-side decorations,"]
            #[doc = "if your application has any..."]
            #[doc = ""]
            #[doc = "This is only a request that the user intends to close your"]
            #[doc = "window. The client may choose to ignore this request, or show"]
            #[doc = "a dialog to ask the user to save their data..."]
            async fn close(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "A popup surface is a short-lived, temporary surface that can be"]
    #[doc = "used to implement menus. It takes an explicit grab on the surface"]
    #[doc = "that will be dismissed when the user dismisses the popup. This can"]
    #[doc = "be done by the user clicking outside the surface, using the keyboard,"]
    #[doc = "or even locking the screen through closing the lid or a timeout."]
    #[doc = ""]
    #[doc = "When the popup is dismissed, a popup_done event will be sent out,"]
    #[doc = "and at the same time the surface will be unmapped. The xdg_popup"]
    #[doc = "object is now inert and cannot be reactivated, so clients should"]
    #[doc = "destroy it. Explicitly destroying the xdg_popup object will also"]
    #[doc = "dismiss the popup and unmap the surface."]
    #[doc = ""]
    #[doc = "Clients will receive events for all their surfaces during this"]
    #[doc = "grab (which is an \"owner-events\" grab in X11 parlance). This is"]
    #[doc = "done so that users can navigate through submenus and other"]
    #[doc = "\"nested\" popup windows without having to dismiss the topmost"]
    #[doc = "popup."]
    #[doc = ""]
    #[doc = "Clients that want to dismiss the popup when another surface of"]
    #[doc = "their own is clicked should dismiss the popup using the destroy"]
    #[doc = "request."]
    #[doc = ""]
    #[doc = "The parent surface must have either an xdg_surface or xdg_popup"]
    #[doc = "role."]
    #[doc = ""]
    #[doc = "Specifying an xdg_popup for the parent means that the popups are"]
    #[doc = "nested, with this popup now being the topmost popup. Nested"]
    #[doc = "popups must be destroyed in the reverse order they were created"]
    #[doc = "in, e.g. the only popup you are allowed to destroy at all times"]
    #[doc = "is the topmost one."]
    #[doc = ""]
    #[doc = "If there is an existing popup when creating a new popup, the"]
    #[doc = "parent must be the current topmost popup."]
    #[doc = ""]
    #[doc = "A parent surface must be mapped before the new popup is mapped."]
    #[doc = ""]
    #[doc = "When compositors choose to dismiss a popup, they will likely"]
    #[doc = "dismiss every nested popup as well. When a compositor dismisses"]
    #[doc = "popups, it will follow the same dismissing order as required"]
    #[doc = "from the client."]
    #[doc = ""]
    #[doc = "The x and y arguments passed when creating the popup object specify"]
    #[doc = "where the top left of the popup should be placed, relative to the"]
    #[doc = "local surface coordinates of the parent surface. See"]
    #[doc = "xdg_shell.get_xdg_popup."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xdg_popup state to take effect."]
    #[doc = ""]
    #[doc = "For a surface to be mapped by the compositor the client must have"]
    #[doc = "committed both the xdg_popup state and a buffer."]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_popup {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the xdg_popup interface. See the module level documentation for more info"]
        pub trait XdgPopup {
            const INTERFACE: &'static str = "xdg_popup";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This destroys the popup. Explicitly destroying the xdg_popup"]
            #[doc = "object will also dismiss the popup, and unmap the surface."]
            #[doc = ""]
            #[doc = "If this xdg_popup is not the \"topmost\" popup, a protocol error"]
            #[doc = "will be sent."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_popup#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The popup_done event is sent out when a popup is dismissed by the"]
            #[doc = "compositor. The client should destroy the xdg_popup object at this"]
            #[doc = "point."]
            async fn popup_done(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol specifies a way for a client to request the compositor"]
#[doc = "to ignore its own keyboard shortcuts for a given seat, so that all"]
#[doc = "key events from that seat get forwarded to a surface."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible"]
#[doc = "changes may be added together with the corresponding interface"]
#[doc = "version bump."]
#[doc = "Backward incompatible changes are done by bumping the version"]
#[doc = "number in the protocol and interface names and resetting the"]
#[doc = "interface version. Once the protocol is to be declared stable,"]
#[doc = "the 'z' prefix and the version number in the protocol and"]
#[doc = "interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod keyboard_shortcuts_inhibit_unstable_v1 {
    #[doc = "A global interface used for inhibiting the compositor keyboard shortcuts."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_keyboard_shortcuts_inhibit_manager_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the shortcuts are already inhibited for this surface"]
            AlreadyInhibited = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyInhibited),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_keyboard_shortcuts_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpKeyboardShortcutsInhibitManagerV1 {
            const INTERFACE: &'static str = "zwp_keyboard_shortcuts_inhibit_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the keyboard shortcuts inhibitor manager."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_keyboard_shortcuts_inhibit_manager_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a new keyboard shortcuts inhibitor object associated with"]
            #[doc = "the given surface for the given seat."]
            #[doc = ""]
            #[doc = "If shortcuts are already inhibited for the specified seat and surface,"]
            #[doc = "a protocol error \"already_inhibited\" is raised by the compositor."]
            async fn inhibit_shortcuts(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_keyboard_shortcuts_inhibit_manager_v1#{}.inhibit_shortcuts()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A keyboard shortcuts inhibitor instructs the compositor to ignore"]
    #[doc = "its own keyboard shortcuts when the associated surface has keyboard"]
    #[doc = "focus. As a result, when the surface has keyboard focus on the given"]
    #[doc = "seat, it will receive all key events originating from the specified"]
    #[doc = "seat, even those which would normally be caught by the compositor for"]
    #[doc = "its own shortcuts."]
    #[doc = ""]
    #[doc = "The Wayland compositor is however under no obligation to disable"]
    #[doc = "all of its shortcuts, and may keep some special key combo for its own"]
    #[doc = "use, including but not limited to one allowing the user to forcibly"]
    #[doc = "restore normal keyboard events routing in the case of an unwilling"]
    #[doc = "client. The compositor may also use the same key combo to reactivate"]
    #[doc = "an existing shortcut inhibitor that was previously deactivated on"]
    #[doc = "user request."]
    #[doc = ""]
    #[doc = "When the compositor restores its own keyboard shortcuts, an"]
    #[doc = "\"inactive\" event is emitted to notify the client that the keyboard"]
    #[doc = "shortcuts inhibitor is not effectively active for the surface and"]
    #[doc = "seat any more, and the client should not expect to receive all"]
    #[doc = "keyboard events."]
    #[doc = ""]
    #[doc = "When the keyboard shortcuts inhibitor is inactive, the client has"]
    #[doc = "no way to forcibly reactivate the keyboard shortcuts inhibitor."]
    #[doc = ""]
    #[doc = "The user can chose to re-enable a previously deactivated keyboard"]
    #[doc = "shortcuts inhibitor using any mechanism the compositor may offer,"]
    #[doc = "in which case the compositor will send an \"active\" event to notify"]
    #[doc = "the client."]
    #[doc = ""]
    #[doc = "If the surface is destroyed, unmapped, or loses the seat's keyboard"]
    #[doc = "focus, the keyboard shortcuts inhibitor becomes irrelevant and the"]
    #[doc = "compositor will restore its own keyboard shortcuts but no \"inactive\""]
    #[doc = "event is emitted in this case."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_keyboard_shortcuts_inhibitor_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_keyboard_shortcuts_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ZwpKeyboardShortcutsInhibitorV1 {
            const INTERFACE: &'static str = "zwp_keyboard_shortcuts_inhibitor_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Remove the keyboard shortcuts inhibitor from the associated wl_surface."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_keyboard_shortcuts_inhibitor_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event indicates that the shortcut inhibitor is active."]
            #[doc = ""]
            #[doc = "The compositor sends this event every time compositor shortcuts"]
            #[doc = "are inhibited on behalf of the surface. When active, the client"]
            #[doc = "may receive input events normally reserved by the compositor"]
            #[doc = "(see zwp_keyboard_shortcuts_inhibitor_v1)."]
            #[doc = ""]
            #[doc = "This occurs typically when the initial request \"inhibit_shortcuts\""]
            #[doc = "first becomes active or when the user instructs the compositor to"]
            #[doc = "re-enable and existing shortcuts inhibitor using any mechanism"]
            #[doc = "offered by the compositor."]
            async fn active(&self) -> crate::client::Result<()>;
            #[doc = "This event indicates that the shortcuts inhibitor is inactive,"]
            #[doc = "normal shortcuts processing is restored by the compositor."]
            async fn inactive(&self) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod idle_inhibit_unstable_v1 {
    #[doc = "This interface permits inhibiting the idle behavior such as screen"]
    #[doc = "blanking, locking, and screensaving.  The client binds the idle manager"]
    #[doc = "globally, then creates idle-inhibitor objects for each surface."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_idle_inhibit_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_idle_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpIdleInhibitManagerV1 {
            const INTERFACE: &'static str = "zwp_idle_inhibit_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the inhibit manager."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_idle_inhibit_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a new inhibitor object associated with the given surface."]
            async fn create_inhibitor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_idle_inhibit_manager_v1#{}.create_inhibitor()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An idle inhibitor prevents the output that the associated surface is"]
    #[doc = "visible on from being set to a state where it is not visually usable due"]
    #[doc = "to lack of user interaction (e.g. blanked, dimmed, locked, set to power"]
    #[doc = "save, etc.)  Any screensaver processes are also blocked from displaying."]
    #[doc = ""]
    #[doc = "If the surface is destroyed, unmapped, becomes occluded, loses"]
    #[doc = "visibility, or otherwise becomes not visually relevant for the user, the"]
    #[doc = "idle inhibitor will not be honored by the compositor; if the surface"]
    #[doc = "subsequently regains visibility the inhibitor takes effect once again."]
    #[doc = "Likewise, the inhibitor isn't honored if the system was already idled at"]
    #[doc = "the time the inhibitor was established, although if the system later"]
    #[doc = "de-idles and re-idles the inhibitor will take effect."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_idle_inhibitor_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_idle_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ZwpIdleInhibitorV1 {
            const INTERFACE: &'static str = "zwp_idle_inhibitor_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Remove the inhibitor effect from the associated wl_surface."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_idle_inhibitor_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod input_method_unstable_v1 {
    #[doc = "Corresponds to a text input on the input method side. An input method context"]
    #[doc = "is created on text input activation on the input method side. It allows"]
    #[doc = "receiving information about the text input from the application via events."]
    #[doc = "Input method contexts do not keep state after deactivation and should be"]
    #[doc = "destroyed after deactivation is handled."]
    #[doc = ""]
    #[doc = "Text is generally UTF-8 encoded, indices and lengths are in bytes."]
    #[doc = ""]
    #[doc = "Serials are used to synchronize the state between the text input and"]
    #[doc = "an input method. New serials are sent by the text input in the"]
    #[doc = "commit_state request and are used by the input method to indicate"]
    #[doc = "the known text input state in events like preedit_string, commit_string,"]
    #[doc = "and keysym. The text input can then ignore events from the input method"]
    #[doc = "which are based on an outdated state (for example after a reset)."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_input_method_context_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_input_method_context_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputMethodContextV1 {
            const INTERFACE: &'static str = "zwp_input_method_context_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_method_context_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Send the commit string text for insertion to the application."]
            #[doc = ""]
            #[doc = "The text to commit could be either just a single character after a key"]
            #[doc = "press or the result of some composing (pre-edit). It could be also an"]
            #[doc = "empty text when some text should be removed (see"]
            #[doc = "delete_surrounding_text) or when the input cursor should be moved (see"]
            #[doc = "cursor_position)."]
            #[doc = ""]
            #[doc = "Any previously set composing text will be removed."]
            async fn commit_string(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                text: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.commit_string()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(Some(text))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Send the pre-edit string text to the application text input."]
            #[doc = ""]
            #[doc = "The commit text can be used to replace the pre-edit text on reset (for"]
            #[doc = "example on unfocus)."]
            #[doc = ""]
            #[doc = "Previously sent preedit_style and preedit_cursor requests are also"]
            #[doc = "processed by the text_input."]
            async fn preedit_string(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                text: String,
                commit: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.preedit_string()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(Some(text))
                    .put_string(Some(commit))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the styling information on composing text. The style is applied for"]
            #[doc = "length in bytes from index relative to the beginning of"]
            #[doc = "the composing text (as byte offset). Multiple styles can"]
            #[doc = "be applied to a composing text."]
            #[doc = ""]
            #[doc = "This request should be sent before sending a preedit_string request."]
            async fn preedit_styling(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                index: u32,
                length: u32,
                style: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.preedit_styling()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(index)
                    .put_uint(length)
                    .put_uint(style)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the cursor position inside the composing text (as byte offset)"]
            #[doc = "relative to the start of the composing text."]
            #[doc = ""]
            #[doc = "When index is negative no cursor should be displayed."]
            #[doc = ""]
            #[doc = "This request should be sent before sending a preedit_string request."]
            async fn preedit_cursor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                index: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.preedit_cursor()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(index).build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Remove the surrounding text."]
            #[doc = ""]
            #[doc = "This request will be handled on the text_input side directly following"]
            #[doc = "a commit_string request."]
            async fn delete_surrounding_text(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                index: i32,
                length: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.delete_surrounding_text()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(index)
                    .put_uint(length)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the cursor and anchor to a new position. Index is the new cursor"]
            #[doc = "position in bytes (when >= 0 this is relative to the end of the inserted text,"]
            #[doc = "otherwise it is relative to the beginning of the inserted text). Anchor is"]
            #[doc = "the new anchor position in bytes (when >= 0 this is relative to the end of the"]
            #[doc = "inserted text, otherwise it is relative to the beginning of the inserted"]
            #[doc = "text). When there should be no selected text, anchor should be the same"]
            #[doc = "as index."]
            #[doc = ""]
            #[doc = "This request will be handled on the text_input side directly following"]
            #[doc = "a commit_string request."]
            async fn cursor_position(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                index: i32,
                anchor: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.cursor_position()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(index)
                    .put_int(anchor)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn modifiers_map(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                map: Vec<u8>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.modifiers_map()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_array(map).build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Notify when a key event was sent. Key events should not be used for"]
            #[doc = "normal text input operations, which should be done with commit_string,"]
            #[doc = "delete_surrounding_text, etc. The key event follows the wl_keyboard key"]
            #[doc = "event convention. Sym is an XKB keysym, state is a wl_keyboard key_state."]
            async fn keysym(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                time: u32,
                sym: u32,
                state: u32,
                modifiers: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_method_context_v1#{}.keysym()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(sym)
                    .put_uint(state)
                    .put_uint(modifiers)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Allow an input method to receive hardware keyboard input and process"]
            #[doc = "key events to generate text events (with pre-edit) over the wire. This"]
            #[doc = "allows input methods which compose multiple key events for inputting"]
            #[doc = "text like it is done for CJK languages."]
            async fn grab_keyboard(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                keyboard: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.grab_keyboard()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(keyboard))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Forward a wl_keyboard::key event to the client that was not processed"]
            #[doc = "by the input method itself. Should be used when filtering key events"]
            #[doc = "with grab_keyboard.  The arguments should be the ones from the"]
            #[doc = "wl_keyboard::key event."]
            #[doc = ""]
            #[doc = "For generating custom key events use the keysym request instead."]
            async fn key(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                time: u32,
                key: u32,
                state: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_method_context_v1#{}.key()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(key)
                    .put_uint(state)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Forward a wl_keyboard::modifiers event to the client that was not"]
            #[doc = "processed by the input method itself.  Should be used when filtering"]
            #[doc = "key events with grab_keyboard. The arguments should be the ones"]
            #[doc = "from the wl_keyboard::modifiers event."]
            async fn modifiers(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                mods_depressed: u32,
                mods_latched: u32,
                mods_locked: u32,
                group: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_method_context_v1#{}.modifiers()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(mods_depressed)
                    .put_uint(mods_latched)
                    .put_uint(mods_locked)
                    .put_uint(group)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 11u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn language(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                language: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_method_context_v1#{}.language()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(Some(language))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 12u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn text_direction(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                direction: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_method_context_v1#{}.text_direction()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(direction)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 13u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The plain surrounding text around the input position. Cursor is the"]
            #[doc = "position in bytes within the surrounding text relative to the beginning"]
            #[doc = "of the text. Anchor is the position in bytes of the selection anchor"]
            #[doc = "within the surrounding text relative to the beginning of the text. If"]
            #[doc = "there is no selected text then anchor is the same as cursor."]
            async fn surrounding_text(
                &self,
                text: String,
                cursor: u32,
                anchor: u32,
            ) -> crate::client::Result<()>;
            async fn reset(&self) -> crate::client::Result<()>;
            async fn content_type(&self, hint: u32, purpose: u32) -> crate::client::Result<()>;
            async fn invoke_action(&self, button: u32, index: u32) -> crate::client::Result<()>;
            async fn commit_state(&self, serial: u32) -> crate::client::Result<()>;
            async fn preferred_language(&self, language: String) -> crate::client::Result<()>;
        }
    }
    #[doc = "An input method object is responsible for composing text in response to"]
    #[doc = "input from hardware or virtual keyboards. There is one input method"]
    #[doc = "object per seat. On activate there is a new input method context object"]
    #[doc = "created which allows the input method to communicate with the text input."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_input_method_v1 {
        #[doc = "Trait to implement the zwp_input_method_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputMethodV1 {
            const INTERFACE: &'static str = "zwp_input_method_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "A text input was activated. Creates an input method context object"]
            #[doc = "which allows communication with the text input."]
            async fn activate(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "The text input corresponding to the context argument was deactivated."]
            #[doc = "The input method context should be destroyed after deactivation is"]
            #[doc = "handled."]
            async fn deactivate(&self, context: crate::wire::ObjectId)
                -> crate::client::Result<()>;
        }
    }
    #[doc = "Only one client can bind this interface at a time."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_input_panel_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_input_panel_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputPanelV1 {
            const INTERFACE: &'static str = "zwp_input_panel_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn get_input_panel_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_panel_v1#{}.get_input_panel_surface()",
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
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_input_panel_surface_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Position {
            CenterBottom = 0u32,
        }
        impl TryFrom<u32> for Position {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::CenterBottom),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Position {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_input_panel_surface_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputPanelSurfaceV1 {
            const INTERFACE: &'static str = "zwp_input_panel_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Set the input_panel_surface type to keyboard."]
            #[doc = ""]
            #[doc = "A keyboard surface is only shown when a text input is active."]
            async fn set_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                position: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_panel_surface_v1#{}.set_toplevel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .put_uint(position)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the input_panel_surface to be an overlay panel."]
            #[doc = ""]
            #[doc = "This is shown near the input cursor above the application window when"]
            #[doc = "a text input is active."]
            async fn set_overlay_panel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_panel_surface_v1#{}.set_overlay_panel()",
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
#[doc = "This protocol specifies a set of interfaces used for adding constraints to"]
#[doc = "the motion of a pointer. Possible constraints include confining pointer"]
#[doc = "motions to a given region, or locking it to its current position."]
#[doc = ""]
#[doc = "In order to constrain the pointer, a client must first bind the global"]
#[doc = "interface \"wp_pointer_constraints\" which, if a compositor supports pointer"]
#[doc = "constraints, is exposed by the registry. Using the bound global object, the"]
#[doc = "client uses the request that corresponds to the type of constraint it wants"]
#[doc = "to make. See wp_pointer_constraints for more details."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and backward"]
#[doc = "incompatible changes may be made. Backward compatible changes may be added"]
#[doc = "together with the corresponding interface version bump. Backward"]
#[doc = "incompatible changes are done by bumping the version number in the protocol"]
#[doc = "and interface names and resetting the interface version. Once the protocol"]
#[doc = "is to be declared stable, the 'z' prefix and the version number in the"]
#[doc = "protocol and interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod pointer_constraints_unstable_v1 {
    #[doc = "The global interface exposing pointer constraining functionality. It"]
    #[doc = "exposes two requests: lock_pointer for locking the pointer to its"]
    #[doc = "position, and confine_pointer for locking the pointer to a region."]
    #[doc = ""]
    #[doc = "The lock_pointer and confine_pointer requests create the objects"]
    #[doc = "wp_locked_pointer and wp_confined_pointer respectively, and the client can"]
    #[doc = "use these objects to interact with the lock."]
    #[doc = ""]
    #[doc = "For any surface, only one lock or confinement may be active across all"]
    #[doc = "wl_pointer objects of the same seat. If a lock or confinement is requested"]
    #[doc = "when another lock or confinement is active or requested on the same surface"]
    #[doc = "and with any of the wl_pointer objects of the same seat, an"]
    #[doc = "'already_constrained' error will be raised."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_pointer_constraints_v1 {
        use futures_util::SinkExt;
        #[doc = "These errors can be emitted in response to wp_pointer_constraints"]
        #[doc = "requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "pointer constraint already requested on that surface"]
            AlreadyConstrained = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyConstrained),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These values represent different lifetime semantics. They are passed"]
        #[doc = "as arguments to the factory requests to specify how the constraint"]
        #[doc = "lifetimes should be managed."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Lifetime {
            Oneshot = 1u32,
            Persistent = 2u32,
        }
        impl TryFrom<u32> for Lifetime {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Oneshot),
                    2u32 => Ok(Self::Persistent),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Lifetime {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_pointer_constraints_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerConstraintsV1 {
            const INTERFACE: &'static str = "zwp_pointer_constraints_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Used by the client to notify the server that it will no longer use this"]
            #[doc = "pointer constraints object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_pointer_constraints_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The lock_pointer request lets the client request to disable movements of"]
            #[doc = "the virtual pointer (i.e. the cursor), effectively locking the pointer"]
            #[doc = "to a position. This request may not take effect immediately; in the"]
            #[doc = "future, when the compositor deems implementation-specific constraints"]
            #[doc = "are satisfied, the pointer lock will be activated and the compositor"]
            #[doc = "sends a locked event."]
            #[doc = ""]
            #[doc = "The protocol provides no guarantee that the constraints are ever"]
            #[doc = "satisfied, and does not require the compositor to send an error if the"]
            #[doc = "constraints cannot ever be satisfied. It is thus possible to request a"]
            #[doc = "lock that will never activate."]
            #[doc = ""]
            #[doc = "There may not be another pointer constraint of any kind requested or"]
            #[doc = "active on the surface for any of the wl_pointer objects of the seat of"]
            #[doc = "the passed pointer when requesting a lock. If there is, an error will be"]
            #[doc = "raised. See general pointer lock documentation for more details."]
            #[doc = ""]
            #[doc = "The intersection of the region passed with this request and the input"]
            #[doc = "region of the surface is used to determine where the pointer must be"]
            #[doc = "in order for the lock to activate. It is up to the compositor whether to"]
            #[doc = "warp the pointer or require some kind of user interaction for the lock"]
            #[doc = "to activate. If the region is null the surface input region is used."]
            #[doc = ""]
            #[doc = "A surface may receive pointer focus without the lock being activated."]
            #[doc = ""]
            #[doc = "The request creates a new object wp_locked_pointer which is used to"]
            #[doc = "interact with the lock as well as receive updates about its state. See"]
            #[doc = "the the description of wp_locked_pointer for further information."]
            #[doc = ""]
            #[doc = "Note that while a pointer is locked, the wl_pointer objects of the"]
            #[doc = "corresponding seat will not emit any wl_pointer.motion events, but"]
            #[doc = "relative motion events will still be emitted via wp_relative_pointer"]
            #[doc = "objects of the same seat. wl_pointer.axis and wl_pointer.button events"]
            #[doc = "are unaffected."]
            async fn lock_pointer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
                lifetime: Lifetime,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_pointer_constraints_v1#{}.lock_pointer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(pointer))
                    .put_object(region)
                    .put_uint(lifetime as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The confine_pointer request lets the client request to confine the"]
            #[doc = "pointer cursor to a given region. This request may not take effect"]
            #[doc = "immediately; in the future, when the compositor deems implementation-"]
            #[doc = "specific constraints are satisfied, the pointer confinement will be"]
            #[doc = "activated and the compositor sends a confined event."]
            #[doc = ""]
            #[doc = "The intersection of the region passed with this request and the input"]
            #[doc = "region of the surface is used to determine where the pointer must be"]
            #[doc = "in order for the confinement to activate. It is up to the compositor"]
            #[doc = "whether to warp the pointer or require some kind of user interaction for"]
            #[doc = "the confinement to activate. If the region is null the surface input"]
            #[doc = "region is used."]
            #[doc = ""]
            #[doc = "The request will create a new object wp_confined_pointer which is used"]
            #[doc = "to interact with the confinement as well as receive updates about its"]
            #[doc = "state. See the the description of wp_confined_pointer for further"]
            #[doc = "information."]
            async fn confine_pointer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
                lifetime: Lifetime,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_pointer_constraints_v1#{}.confine_pointer()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(pointer))
                    .put_object(region)
                    .put_uint(lifetime as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The wp_locked_pointer interface represents a locked pointer state."]
    #[doc = ""]
    #[doc = "While the lock of this object is active, the wl_pointer objects of the"]
    #[doc = "associated seat will not emit any wl_pointer.motion events."]
    #[doc = ""]
    #[doc = "This object will send the event 'locked' when the lock is activated."]
    #[doc = "Whenever the lock is activated, it is guaranteed that the locked surface"]
    #[doc = "will already have received pointer focus and that the pointer will be"]
    #[doc = "within the region passed to the request creating this object."]
    #[doc = ""]
    #[doc = "To unlock the pointer, send the destroy request. This will also destroy"]
    #[doc = "the wp_locked_pointer object."]
    #[doc = ""]
    #[doc = "If the compositor decides to unlock the pointer the unlocked event is"]
    #[doc = "sent. See wp_locked_pointer.unlock for details."]
    #[doc = ""]
    #[doc = "When unlocking, the compositor may warp the cursor position to the set"]
    #[doc = "cursor position hint. If it does, it will not result in any relative"]
    #[doc = "motion events emitted via wp_relative_pointer."]
    #[doc = ""]
    #[doc = "If the surface the lock was requested on is destroyed and the lock is not"]
    #[doc = "yet activated, the wp_locked_pointer object is now defunct and must be"]
    #[doc = "destroyed."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_locked_pointer_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_locked_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLockedPointerV1 {
            const INTERFACE: &'static str = "zwp_locked_pointer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the locked pointer object. If applicable, the compositor will"]
            #[doc = "unlock the pointer."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_locked_pointer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the cursor position hint relative to the top left corner of the"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "If the client is drawing its own cursor, it should update the position"]
            #[doc = "hint to the position of its own cursor. A compositor may use this"]
            #[doc = "information to warp the pointer upon unlock in order to avoid pointer"]
            #[doc = "jumps."]
            #[doc = ""]
            #[doc = "The cursor position hint is double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            async fn set_cursor_position_hint(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface_x: crate::wire::Fixed,
                surface_y: crate::wire::Fixed,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_locked_pointer_v1#{}.set_cursor_position_hint()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fixed(surface_x)
                    .put_fixed(surface_y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a new region used to lock the pointer."]
            #[doc = ""]
            #[doc = "The new lock region is double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "For details about the lock region, see wp_locked_pointer."]
            async fn set_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_locked_pointer_v1#{}.set_region()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(region)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Notification that the pointer lock of the seat's pointer is activated."]
            async fn locked(&self) -> crate::client::Result<()>;
            #[doc = "Notification that the pointer lock of the seat's pointer is no longer"]
            #[doc = "active. If this is a oneshot pointer lock (see"]
            #[doc = "wp_pointer_constraints.lifetime) this object is now defunct and should"]
            #[doc = "be destroyed. If this is a persistent pointer lock (see"]
            #[doc = "wp_pointer_constraints.lifetime) this pointer lock may again"]
            #[doc = "reactivate in the future."]
            async fn unlocked(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "The wp_confined_pointer interface represents a confined pointer state."]
    #[doc = ""]
    #[doc = "This object will send the event 'confined' when the confinement is"]
    #[doc = "activated. Whenever the confinement is activated, it is guaranteed that"]
    #[doc = "the surface the pointer is confined to will already have received pointer"]
    #[doc = "focus and that the pointer will be within the region passed to the request"]
    #[doc = "creating this object. It is up to the compositor to decide whether this"]
    #[doc = "requires some user interaction and if the pointer will warp to within the"]
    #[doc = "passed region if outside."]
    #[doc = ""]
    #[doc = "To unconfine the pointer, send the destroy request. This will also destroy"]
    #[doc = "the wp_confined_pointer object."]
    #[doc = ""]
    #[doc = "If the compositor decides to unconfine the pointer the unconfined event is"]
    #[doc = "sent. The wp_confined_pointer object is at this point defunct and should"]
    #[doc = "be destroyed."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_confined_pointer_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_confined_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpConfinedPointerV1 {
            const INTERFACE: &'static str = "zwp_confined_pointer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the confined pointer object. If applicable, the compositor will"]
            #[doc = "unconfine the pointer."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_confined_pointer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a new region used to confine the pointer."]
            #[doc = ""]
            #[doc = "The new confine region is double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "If the confinement is active when the new confinement region is applied"]
            #[doc = "and the pointer ends up outside of newly applied region, the pointer may"]
            #[doc = "warped to a position within the new confinement region. If warped, a"]
            #[doc = "wl_pointer.motion event will be emitted, but no"]
            #[doc = "wp_relative_pointer.relative_motion event."]
            #[doc = ""]
            #[doc = "The compositor may also, instead of using the new region, unconfine the"]
            #[doc = "pointer."]
            #[doc = ""]
            #[doc = "For details about the confine region, see wp_confined_pointer."]
            async fn set_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_confined_pointer_v1#{}.set_region()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(region)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Notification that the pointer confinement of the seat's pointer is"]
            #[doc = "activated."]
            async fn confined(&self) -> crate::client::Result<()>;
            #[doc = "Notification that the pointer confinement of the seat's pointer is no"]
            #[doc = "longer active. If this is a oneshot pointer confinement (see"]
            #[doc = "wp_pointer_constraints.lifetime) this object is now defunct and should"]
            #[doc = "be destroyed. If this is a persistent pointer confinement (see"]
            #[doc = "wp_pointer_constraints.lifetime) this pointer confinement may again"]
            #[doc = "reactivate in the future."]
            async fn unconfined(&self) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod linux_dmabuf_unstable_v1 {
    #[doc = "Following the interfaces from:"]
    #[doc = "https://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt"]
    #[doc = "https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt"]
    #[doc = "and the Linux DRM sub-system's AddFb2 ioctl."]
    #[doc = ""]
    #[doc = "This interface offers ways to create generic dmabuf-based wl_buffers."]
    #[doc = ""]
    #[doc = "Clients can use the get_surface_feedback request to get dmabuf feedback"]
    #[doc = "for a particular surface. If the client wants to retrieve feedback not"]
    #[doc = "tied to a surface, they can use the get_default_feedback request."]
    #[doc = ""]
    #[doc = "The following are required from clients:"]
    #[doc = ""]
    #[doc = "- Clients must ensure that either all data in the dma-buf is"]
    #[doc = "coherent for all subsequent read access or that coherency is"]
    #[doc = "correctly handled by the underlying kernel-side dma-buf"]
    #[doc = "implementation."]
    #[doc = ""]
    #[doc = "- Don't make any more attachments after sending the buffer to the"]
    #[doc = "compositor. Making more attachments later increases the risk of"]
    #[doc = "the compositor not being able to use (re-import) an existing"]
    #[doc = "dmabuf-based wl_buffer."]
    #[doc = ""]
    #[doc = "The underlying graphics stack must ensure the following:"]
    #[doc = ""]
    #[doc = "- The dmabuf file descriptors relayed to the server will stay valid"]
    #[doc = "for the whole lifetime of the wl_buffer. This means the server may"]
    #[doc = "at any time use those fds to import the dmabuf into any kernel"]
    #[doc = "sub-system that might accept it."]
    #[doc = ""]
    #[doc = "However, when the underlying graphics stack fails to deliver the"]
    #[doc = "promise, because of e.g. a device hot-unplug which raises internal"]
    #[doc = "errors, after the wl_buffer has been successfully created the"]
    #[doc = "compositor must not raise protocol errors to the client when dmabuf"]
    #[doc = "import later fails."]
    #[doc = ""]
    #[doc = "To create a wl_buffer from one or more dmabufs, a client creates a"]
    #[doc = "zwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params"]
    #[doc = "request. All planes required by the intended format are added with"]
    #[doc = "the 'add' request. Finally, a 'create' or 'create_immed' request is"]
    #[doc = "issued, which has the following outcome depending on the import success."]
    #[doc = ""]
    #[doc = "The 'create' request,"]
    #[doc = "- on success, triggers a 'created' event which provides the final"]
    #[doc = "wl_buffer to the client."]
    #[doc = "- on failure, triggers a 'failed' event to convey that the server"]
    #[doc = "cannot use the dmabufs received from the client."]
    #[doc = ""]
    #[doc = "For the 'create_immed' request,"]
    #[doc = "- on success, the server immediately imports the added dmabufs to"]
    #[doc = "create a wl_buffer. No event is sent from the server in this case."]
    #[doc = "- on failure, the server can choose to either:"]
    #[doc = "- terminate the client by raising a fatal error."]
    #[doc = "- mark the wl_buffer as failed, and send a 'failed' event to the"]
    #[doc = "client. If the client uses a failed wl_buffer as an argument to any"]
    #[doc = "request, the behaviour is compositor implementation-defined."]
    #[doc = ""]
    #[doc = "For all DRM formats and unless specified in another protocol extension,"]
    #[doc = "pre-multiplied alpha is used for pixel values."]
    #[doc = ""]
    #[doc = "Unless specified otherwise in another protocol extension, implicit"]
    #[doc = "synchronization is used. In other words, compositors and clients must"]
    #[doc = "wait and signal fences implicitly passed via the DMA-BUF's reservation"]
    #[doc = "mechanism."]
    #[doc = ""]
    #[doc = "Disclaimer: This protocol extension has been marked stable. This copy is"]
    #[doc = "no longer used and only retained for backwards compatibility. The"]
    #[doc = "canonical version can be found in the stable/ directory."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_dmabuf_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_linux_dmabuf_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufV1 {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_v1";
            const VERSION: u32 = 5u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Objects created through this interface, especially wl_buffers, will"]
            #[doc = "remain valid."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_dmabuf_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This temporary object is used to collect multiple dmabuf handles into"]
            #[doc = "a single batch to create a wl_buffer. It can only be used once and"]
            #[doc = "should be destroyed after a 'created' or 'failed' event has been"]
            #[doc = "received."]
            async fn create_params(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                params_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_dmabuf_v1#{}.create_params()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(params_id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request creates a new wp_linux_dmabuf_feedback object not bound"]
            #[doc = "to a particular surface. This object will deliver feedback about dmabuf"]
            #[doc = "parameters to use if the client doesn't support per-surface feedback"]
            #[doc = "(see get_surface_feedback)."]
            async fn get_default_feedback(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_dmabuf_v1#{}.get_default_feedback()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request creates a new wp_linux_dmabuf_feedback object for the"]
            #[doc = "specified wl_surface. This object will deliver feedback about dmabuf"]
            #[doc = "parameters to use for buffers attached to this surface."]
            #[doc = ""]
            #[doc = "If the surface is destroyed before the wp_linux_dmabuf_feedback object,"]
            #[doc = "the feedback object becomes inert."]
            async fn get_surface_feedback(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_dmabuf_v1#{}.get_surface_feedback()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event advertises one buffer format that the server supports."]
            #[doc = "All the supported formats are advertised once when the client"]
            #[doc = "binds to this interface. A roundtrip after binding guarantees"]
            #[doc = "that the client has received all supported formats."]
            #[doc = ""]
            #[doc = "For the definition of the format codes, see the"]
            #[doc = "zwp_linux_buffer_params_v1::create request."]
            #[doc = ""]
            #[doc = "Starting version 4, the format event is deprecated and must not be"]
            #[doc = "sent by compositors. Instead, use get_default_feedback or"]
            #[doc = "get_surface_feedback."]
            async fn format(&self, format: u32) -> crate::client::Result<()>;
            #[doc = "This event advertises the formats that the server supports, along with"]
            #[doc = "the modifiers supported for each format. All the supported modifiers"]
            #[doc = "for all the supported formats are advertised once when the client"]
            #[doc = "binds to this interface. A roundtrip after binding guarantees that"]
            #[doc = "the client has received all supported format-modifier pairs."]
            #[doc = ""]
            #[doc = "For legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi =="]
            #[doc = "0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event."]
            #[doc = "It indicates that the server can support the format with an implicit"]
            #[doc = "modifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it"]
            #[doc = "is as if no explicit modifier is specified. The effective modifier"]
            #[doc = "will be derived from the dmabuf."]
            #[doc = ""]
            #[doc = "A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for"]
            #[doc = "a given format supports both explicit modifiers and implicit modifiers."]
            #[doc = ""]
            #[doc = "For the definition of the format and modifier codes, see the"]
            #[doc = "zwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add"]
            #[doc = "requests."]
            #[doc = ""]
            #[doc = "Starting version 4, the modifier event is deprecated and must not be"]
            #[doc = "sent by compositors. Instead, use get_default_feedback or"]
            #[doc = "get_surface_feedback."]
            async fn modifier(
                &self,
                format: u32,
                modifier_hi: u32,
                modifier_lo: u32,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "This temporary object is a collection of dmabufs and other"]
    #[doc = "parameters that together form a single logical buffer. The temporary"]
    #[doc = "object may eventually create one wl_buffer unless cancelled by"]
    #[doc = "destroying it before requesting 'create'."]
    #[doc = ""]
    #[doc = "Single-planar formats only require one dmabuf, however"]
    #[doc = "multi-planar formats may require more than one dmabuf. For all"]
    #[doc = "formats, an 'add' request must be called once per plane (even if the"]
    #[doc = "underlying dmabuf fd is identical)."]
    #[doc = ""]
    #[doc = "You must use consecutive plane indices ('plane_idx' argument for 'add')"]
    #[doc = "from zero to the number of planes used by the drm_fourcc format code."]
    #[doc = "All planes required by the format must be given exactly once, but can"]
    #[doc = "be given in any order. Each plane index can be set only once."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_buffer_params_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the dmabuf_batch object has already been used to create a wl_buffer"]
            AlreadyUsed = 0u32,
            #[doc = "plane index out of bounds"]
            PlaneIdx = 1u32,
            #[doc = "the plane index was already set"]
            PlaneSet = 2u32,
            #[doc = "missing or too many planes to create a buffer"]
            Incomplete = 3u32,
            #[doc = "format not supported"]
            InvalidFormat = 4u32,
            #[doc = "invalid width or height"]
            InvalidDimensions = 5u32,
            #[doc = "offset + stride * height goes out of dmabuf bounds"]
            OutOfBounds = 6u32,
            #[doc = "invalid wl_buffer resulted from importing dmabufs via"]
            #[doc = "the create_immed request on given buffer_params"]
            InvalidWlBuffer = 7u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    1u32 => Ok(Self::PlaneIdx),
                    2u32 => Ok(Self::PlaneSet),
                    3u32 => Ok(Self::Incomplete),
                    4u32 => Ok(Self::InvalidFormat),
                    5u32 => Ok(Self::InvalidDimensions),
                    6u32 => Ok(Self::OutOfBounds),
                    7u32 => Ok(Self::InvalidWlBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInvert = 1u32 ; # [doc = "content is interlaced"] const Interlaced = 2u32 ; # [doc = "bottom field first"] const BottomFirst = 4u32 ; } }
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
        #[doc = "Trait to implement the zwp_linux_buffer_params_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxBufferParamsV1 {
            const INTERFACE: &'static str = "zwp_linux_buffer_params_v1";
            const VERSION: u32 = 5u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Cleans up the temporary data sent to the server for dmabuf-based"]
            #[doc = "wl_buffer creation."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_buffer_params_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request adds one dmabuf to the set in this"]
            #[doc = "zwp_linux_buffer_params_v1."]
            #[doc = ""]
            #[doc = "The 64-bit unsigned value combined from modifier_hi and modifier_lo"]
            #[doc = "is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the"]
            #[doc = "fb modifier, which is defined in drm_mode.h of Linux UAPI."]
            #[doc = "This is an opaque token. Drivers use this token to express tiling,"]
            #[doc = "compression, etc. driver-specific modifications to the base format"]
            #[doc = "defined by the DRM fourcc code."]
            #[doc = ""]
            #[doc = "Starting from version 4, the invalid_format protocol error is sent if"]
            #[doc = "the format + modifier pair was not advertised as supported."]
            #[doc = ""]
            #[doc = "Starting from version 5, the invalid_format protocol error is sent if"]
            #[doc = "all planes don't use the same modifier."]
            #[doc = ""]
            #[doc = "This request raises the PLANE_IDX error if plane_idx is too large."]
            #[doc = "The error PLANE_SET is raised if attempting to set a plane that"]
            #[doc = "was already set."]
            async fn add(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
                plane_idx: u32,
                offset: u32,
                stride: u32,
                modifier_hi: u32,
                modifier_lo: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_buffer_params_v1#{}.add()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fd(fd)
                    .put_uint(plane_idx)
                    .put_uint(offset)
                    .put_uint(stride)
                    .put_uint(modifier_hi)
                    .put_uint(modifier_lo)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This asks for creation of a wl_buffer from the added dmabuf"]
            #[doc = "buffers. The wl_buffer is not created immediately but returned via"]
            #[doc = "the 'created' event if the dmabuf sharing succeeds. The sharing"]
            #[doc = "may fail at runtime for reasons a client cannot predict, in"]
            #[doc = "which case the 'failed' event is triggered."]
            #[doc = ""]
            #[doc = "The 'format' argument is a DRM_FORMAT code, as defined by the"]
            #[doc = "libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the"]
            #[doc = "authoritative source on how the format codes should work."]
            #[doc = ""]
            #[doc = "The 'flags' is a bitfield of the flags defined in enum \"flags\"."]
            #[doc = "'y_invert' means the that the image needs to be y-flipped."]
            #[doc = ""]
            #[doc = "Flag 'interlaced' means that the frame in the buffer is not"]
            #[doc = "progressive as usual, but interlaced. An interlaced buffer as"]
            #[doc = "supported here must always contain both top and bottom fields."]
            #[doc = "The top field always begins on the first pixel row. The temporal"]
            #[doc = "ordering between the two fields is top field first, unless"]
            #[doc = "'bottom_first' is specified. It is undefined whether 'bottom_first'"]
            #[doc = "is ignored if 'interlaced' is not set."]
            #[doc = ""]
            #[doc = "This protocol does not convey any information about field rate,"]
            #[doc = "duration, or timing, other than the relative ordering between the"]
            #[doc = "two fields in one buffer. A compositor may have to estimate the"]
            #[doc = "intended field rate from the incoming buffer rate. It is undefined"]
            #[doc = "whether the time of receiving wl_surface.commit with a new buffer"]
            #[doc = "attached, applying the wl_surface state, wl_surface.frame callback"]
            #[doc = "trigger, presentation, or any other point in the compositor cycle"]
            #[doc = "is used to measure the frame or field times. There is no support"]
            #[doc = "for detecting missed or late frames/fields/buffers either, and"]
            #[doc = "there is no support whatsoever for cooperating with interlaced"]
            #[doc = "compositor output."]
            #[doc = ""]
            #[doc = "The composited image quality resulting from the use of interlaced"]
            #[doc = "buffers is explicitly undefined. A compositor may use elaborate"]
            #[doc = "hardware features or software to deinterlace and create progressive"]
            #[doc = "output frames from a sequence of interlaced input buffers, or it"]
            #[doc = "may produce substandard image quality. However, compositors that"]
            #[doc = "cannot guarantee reasonable image quality in all cases are recommended"]
            #[doc = "to just reject all interlaced buffers."]
            #[doc = ""]
            #[doc = "Any argument errors, including non-positive width or height,"]
            #[doc = "mismatch between the number of planes and the format, bad"]
            #[doc = "format, bad offset or stride, may be indicated by fatal protocol"]
            #[doc = "errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,"]
            #[doc = "OUT_OF_BOUNDS."]
            #[doc = ""]
            #[doc = "Dmabuf import errors in the server that are not obvious client"]
            #[doc = "bugs are returned via the 'failed' event as non-fatal. This"]
            #[doc = "allows attempting dmabuf sharing and falling back in the client"]
            #[doc = "if it fails."]
            #[doc = ""]
            #[doc = "This request can be sent only once in the object's lifetime, after"]
            #[doc = "which the only legal request is destroy. This object should be"]
            #[doc = "destroyed after issuing a 'create' request. Attempting to use this"]
            #[doc = "object after issuing 'create' raises ALREADY_USED protocol error."]
            #[doc = ""]
            #[doc = "It is not mandatory to issue 'create'. If a client wants to"]
            #[doc = "cancel the buffer creation, it can just destroy this object."]
            async fn create(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
                format: u32,
                flags: Flags,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_buffer_params_v1#{}.create()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .put_uint(format)
                    .put_uint(flags.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This asks for immediate creation of a wl_buffer by importing the"]
            #[doc = "added dmabufs."]
            #[doc = ""]
            #[doc = "In case of import success, no event is sent from the server, and the"]
            #[doc = "wl_buffer is ready to be used by the client."]
            #[doc = ""]
            #[doc = "Upon import failure, either of the following may happen, as seen fit"]
            #[doc = "by the implementation:"]
            #[doc = "- the client is terminated with one of the following fatal protocol"]
            #[doc = "errors:"]
            #[doc = "- INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,"]
            #[doc = "in case of argument errors such as mismatch between the number"]
            #[doc = "of planes and the format, bad format, non-positive width or"]
            #[doc = "height, or bad offset or stride."]
            #[doc = "- INVALID_WL_BUFFER, in case the cause for failure is unknown or"]
            #[doc = "platform specific."]
            #[doc = "- the server creates an invalid wl_buffer, marks it as failed and"]
            #[doc = "sends a 'failed' event to the client. The result of using this"]
            #[doc = "invalid wl_buffer as an argument in any request by the client is"]
            #[doc = "defined by the compositor implementation."]
            #[doc = ""]
            #[doc = "This takes the same arguments as a 'create' request, and obeys the"]
            #[doc = "same restrictions."]
            async fn create_immed(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                buffer_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
                format: u32,
                flags: Flags,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_buffer_params_v1#{}.create_immed()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer_id))
                    .put_int(width)
                    .put_int(height)
                    .put_uint(format)
                    .put_uint(flags.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event indicates that the attempted buffer creation was"]
            #[doc = "successful. It provides the new wl_buffer referencing the dmabuf(s)."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "zwp_linux_buffer_params_v1 object."]
            async fn created(&self, buffer: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event indicates that the attempted buffer creation has"]
            #[doc = "failed. It usually means that one of the dmabuf constraints"]
            #[doc = "has not been fulfilled."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "zwp_linux_buffer_params_v1 object."]
            async fn failed(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "This object advertises dmabuf parameters feedback. This includes the"]
    #[doc = "preferred devices and the supported formats/modifiers."]
    #[doc = ""]
    #[doc = "The parameters are sent once when this object is created and whenever they"]
    #[doc = "change. The done event is always sent once after all parameters have been"]
    #[doc = "sent. When a single parameter changes, all parameters are re-sent by the"]
    #[doc = "compositor."]
    #[doc = ""]
    #[doc = "Compositors can re-send the parameters when the current client buffer"]
    #[doc = "allocations are sub-optimal. Compositors should not re-send the"]
    #[doc = "parameters if re-allocating the buffers would not result in a more optimal"]
    #[doc = "configuration. In particular, compositors should avoid sending the exact"]
    #[doc = "same parameters multiple times in a row."]
    #[doc = ""]
    #[doc = "The tranche_target_device and tranche_formats events are grouped by"]
    #[doc = "tranches of preference. For each tranche, a tranche_target_device, one"]
    #[doc = "tranche_flags and one or more tranche_formats events are sent, followed"]
    #[doc = "by a tranche_done event finishing the list. The tranches are sent in"]
    #[doc = "descending order of preference. All formats and modifiers in the same"]
    #[doc = "tranche have the same preference."]
    #[doc = ""]
    #[doc = "To send parameters, the compositor sends one main_device event, tranches"]
    #[doc = "(each consisting of one tranche_target_device event, one tranche_flags"]
    #[doc = "event, tranche_formats events and then a tranche_done event), then one"]
    #[doc = "done event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_dmabuf_feedback_v1 {
        use futures_util::SinkExt;
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct TrancheFlags : u32 { # [doc = "direct scan-out tranche"] const Scanout = 1u32 ; } }
        impl TryFrom<u32> for TrancheFlags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for TrancheFlags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_linux_dmabuf_feedback_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufFeedbackV1 {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_feedback_v1";
            const VERSION: u32 = 5u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the wp_linux_dmabuf_feedback object anymore."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_linux_dmabuf_feedback_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent after all parameters of a wp_linux_dmabuf_feedback"]
            #[doc = "object have been sent."]
            #[doc = ""]
            #[doc = "This allows changes to the wp_linux_dmabuf_feedback parameters to be"]
            #[doc = "seen as atomic, even if they happen via multiple events."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "This event provides a file descriptor which can be memory-mapped to"]
            #[doc = "access the format and modifier table."]
            #[doc = ""]
            #[doc = "The table contains a tightly packed array of consecutive format +"]
            #[doc = "modifier pairs. Each pair is 16 bytes wide. It contains a format as a"]
            #[doc = "32-bit unsigned integer, followed by 4 bytes of unused padding, and a"]
            #[doc = "modifier as a 64-bit unsigned integer. The native endianness is used."]
            #[doc = ""]
            #[doc = "The client must map the file descriptor in read-only private mode."]
            #[doc = ""]
            #[doc = "Compositors are not allowed to mutate the table file contents once this"]
            #[doc = "event has been sent. Instead, compositors must create a new, separate"]
            #[doc = "table file and re-send feedback parameters. Compositors are allowed to"]
            #[doc = "store duplicate format + modifier pairs in the table."]
            async fn format_table(
                &self,
                fd: rustix::fd::OwnedFd,
                size: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event advertises the main device that the server prefers to use"]
            #[doc = "when direct scan-out to the target device isn't possible. The"]
            #[doc = "advertised main device may be different for each"]
            #[doc = "wp_linux_dmabuf_feedback object, and may change over time."]
            #[doc = ""]
            #[doc = "There is exactly one main device. The compositor must send at least"]
            #[doc = "one preference tranche with tranche_target_device equal to main_device."]
            #[doc = ""]
            #[doc = "Clients need to create buffers that the main device can import and"]
            #[doc = "read from, otherwise creating the dmabuf wl_buffer will fail (see the"]
            #[doc = "wp_linux_buffer_params.create and create_immed requests for details)."]
            #[doc = "The main device will also likely be kept active by the compositor,"]
            #[doc = "so clients can use it instead of waking up another device for power"]
            #[doc = "savings."]
            #[doc = ""]
            #[doc = "In general the device is a DRM node. The DRM node type (primary vs."]
            #[doc = "render) is unspecified. Clients must not rely on the compositor sending"]
            #[doc = "a particular node type. Clients cannot check two devices for equality"]
            #[doc = "by comparing the dev_t value."]
            #[doc = ""]
            #[doc = "If explicit modifiers are not supported and the client performs buffer"]
            #[doc = "allocations on a different device than the main device, then the client"]
            #[doc = "must force the buffer to have a linear layout."]
            async fn main_device(&self, device: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "This event splits tranche_target_device and tranche_formats events in"]
            #[doc = "preference tranches. It is sent after a set of tranche_target_device"]
            #[doc = "and tranche_formats events; it represents the end of a tranche. The"]
            #[doc = "next tranche will have a lower preference."]
            async fn tranche_done(&self) -> crate::client::Result<()>;
            #[doc = "This event advertises the target device that the server prefers to use"]
            #[doc = "for a buffer created given this tranche. The advertised target device"]
            #[doc = "may be different for each preference tranche, and may change over time."]
            #[doc = ""]
            #[doc = "There is exactly one target device per tranche."]
            #[doc = ""]
            #[doc = "The target device may be a scan-out device, for example if the"]
            #[doc = "compositor prefers to directly scan-out a buffer created given this"]
            #[doc = "tranche. The target device may be a rendering device, for example if"]
            #[doc = "the compositor prefers to texture from said buffer."]
            #[doc = ""]
            #[doc = "The client can use this hint to allocate the buffer in a way that makes"]
            #[doc = "it accessible from the target device, ideally directly. The buffer must"]
            #[doc = "still be accessible from the main device, either through direct import"]
            #[doc = "or through a potentially more expensive fallback path. If the buffer"]
            #[doc = "can't be directly imported from the main device then clients must be"]
            #[doc = "prepared for the compositor changing the tranche priority or making"]
            #[doc = "wl_buffer creation fail (see the wp_linux_buffer_params.create and"]
            #[doc = "create_immed requests for details)."]
            #[doc = ""]
            #[doc = "If the device is a DRM node, the DRM node type (primary vs. render) is"]
            #[doc = "unspecified. Clients must not rely on the compositor sending a"]
            #[doc = "particular node type. Clients cannot check two devices for equality by"]
            #[doc = "comparing the dev_t value."]
            #[doc = ""]
            #[doc = "This event is tied to a preference tranche, see the tranche_done event."]
            async fn tranche_target_device(&self, device: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "This event advertises the format + modifier combinations that the"]
            #[doc = "compositor supports."]
            #[doc = ""]
            #[doc = "It carries an array of indices, each referring to a format + modifier"]
            #[doc = "pair in the last received format table (see the format_table event)."]
            #[doc = "Each index is a 16-bit unsigned integer in native endianness."]
            #[doc = ""]
            #[doc = "For legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier."]
            #[doc = "It indicates that the server can support the format with an implicit"]
            #[doc = "modifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it"]
            #[doc = "is as if no explicit modifier is specified. The effective modifier"]
            #[doc = "will be derived from the dmabuf."]
            #[doc = ""]
            #[doc = "A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for"]
            #[doc = "a given format supports both explicit modifiers and implicit modifiers."]
            #[doc = ""]
            #[doc = "Compositors must not send duplicate format + modifier pairs within the"]
            #[doc = "same tranche or across two different tranches with the same target"]
            #[doc = "device and flags."]
            #[doc = ""]
            #[doc = "This event is tied to a preference tranche, see the tranche_done event."]
            #[doc = ""]
            #[doc = "For the definition of the format and modifier codes, see the"]
            #[doc = "wp_linux_buffer_params.create request."]
            async fn tranche_formats(&self, indices: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "This event sets tranche-specific flags."]
            #[doc = ""]
            #[doc = "The scanout flag is a hint that direct scan-out may be attempted by the"]
            #[doc = "compositor on the target device if the client appropriately allocates a"]
            #[doc = "buffer. How to allocate a buffer that can be scanned out on the target"]
            #[doc = "device is implementation-defined."]
            #[doc = ""]
            #[doc = "This event is tied to a preference tranche, see the tranche_done event."]
            async fn tranche_flags(&self, flags: TrancheFlags) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod zwp_linux_explicit_synchronization_unstable_v1 {
    #[doc = "This global is a factory interface, allowing clients to request"]
    #[doc = "explicit synchronization for buffers on a per-surface basis."]
    #[doc = ""]
    #[doc = "See zwp_linux_surface_synchronization_v1 for more information."]
    #[doc = ""]
    #[doc = "This interface is derived from Chromium's"]
    #[doc = "zcr_linux_explicit_synchronization_v1."]
    #[doc = ""]
    #[doc = "Note: this protocol is superseded by linux-drm-syncobj."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_explicit_synchronization_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a synchronization object associated"]
            SynchronizationExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SynchronizationExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_linux_explicit_synchronization_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxExplicitSynchronizationV1 {
            const INTERFACE: &'static str = "zwp_linux_explicit_synchronization_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this explicit synchronization factory object. Other objects,"]
            #[doc = "including zwp_linux_surface_synchronization_v1 objects created by this"]
            #[doc = "factory, shall not be affected by this request."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_explicit_synchronization_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Instantiate an interface extension for the given wl_surface to provide"]
            #[doc = "explicit synchronization."]
            #[doc = ""]
            #[doc = "If the given wl_surface already has an explicit synchronization object"]
            #[doc = "associated, the synchronization_exists protocol error is raised."]
            #[doc = ""]
            #[doc = "Graphics APIs, like EGL or Vulkan, that manage the buffer queue and"]
            #[doc = "commits of a wl_surface themselves, are likely to be using this"]
            #[doc = "extension internally. If a client is using such an API for a"]
            #[doc = "wl_surface, it should not directly use this extension on that surface,"]
            #[doc = "to avoid raising a synchronization_exists protocol error."]
            async fn get_synchronization(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_explicit_synchronization_v1#{}.get_synchronization()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object implements per-surface explicit synchronization."]
    #[doc = ""]
    #[doc = "Synchronization refers to co-ordination of pipelined operations performed"]
    #[doc = "on buffers. Most GPU clients will schedule an asynchronous operation to"]
    #[doc = "render to the buffer, then immediately send the buffer to the compositor"]
    #[doc = "to be attached to a surface."]
    #[doc = ""]
    #[doc = "In implicit synchronization, ensuring that the rendering operation is"]
    #[doc = "complete before the compositor displays the buffer is an implementation"]
    #[doc = "detail handled by either the kernel or userspace graphics driver."]
    #[doc = ""]
    #[doc = "By contrast, in explicit synchronization, dma_fence objects mark when the"]
    #[doc = "asynchronous operations are complete. When submitting a buffer, the"]
    #[doc = "client provides an acquire fence which will be waited on before the"]
    #[doc = "compositor accesses the buffer. The Wayland server, through a"]
    #[doc = "zwp_linux_buffer_release_v1 object, will inform the client with an event"]
    #[doc = "which may be accompanied by a release fence, when the compositor will no"]
    #[doc = "longer access the buffer contents due to the specific commit that"]
    #[doc = "requested the release event."]
    #[doc = ""]
    #[doc = "Each surface can be associated with only one object of this interface at"]
    #[doc = "any time."]
    #[doc = ""]
    #[doc = "In version 1 of this interface, explicit synchronization is only"]
    #[doc = "guaranteed to be supported for buffers created with any version of the"]
    #[doc = "wp_linux_dmabuf buffer factory. Version 2 additionally guarantees"]
    #[doc = "explicit synchronization support for opaque EGL buffers, which is a type"]
    #[doc = "of platform specific buffers described in the EGL_WL_bind_wayland_display"]
    #[doc = "extension. Compositors are free to support explicit synchronization for"]
    #[doc = "additional buffer types."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_surface_synchronization_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the fence specified by the client could not be imported"]
            InvalidFence = 0u32,
            #[doc = "multiple fences added for a single surface commit"]
            DuplicateFence = 1u32,
            #[doc = "multiple releases added for a single surface commit"]
            DuplicateRelease = 2u32,
            #[doc = "the associated wl_surface was destroyed"]
            NoSurface = 3u32,
            #[doc = "the buffer does not support explicit synchronization"]
            UnsupportedBuffer = 4u32,
            #[doc = "no buffer was attached"]
            NoBuffer = 5u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidFence),
                    1u32 => Ok(Self::DuplicateFence),
                    2u32 => Ok(Self::DuplicateRelease),
                    3u32 => Ok(Self::NoSurface),
                    4u32 => Ok(Self::UnsupportedBuffer),
                    5u32 => Ok(Self::NoBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_linux_surface_synchronization_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxSurfaceSynchronizationV1 {
            const INTERFACE: &'static str = "zwp_linux_surface_synchronization_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this explicit synchronization object."]
            #[doc = ""]
            #[doc = "Any fence set by this object with set_acquire_fence since the last"]
            #[doc = "commit will be discarded by the server. Any fences set by this object"]
            #[doc = "before the last commit are not affected."]
            #[doc = ""]
            #[doc = "zwp_linux_buffer_release_v1 objects created by this object are not"]
            #[doc = "affected by this request."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_surface_synchronization_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the acquire fence that must be signaled before the compositor"]
            #[doc = "may sample from the buffer attached with wl_surface.attach. The fence"]
            #[doc = "is a dma_fence kernel object."]
            #[doc = ""]
            #[doc = "The acquire fence is double-buffered state, and will be applied on the"]
            #[doc = "next wl_surface.commit request for the associated surface. Thus, it"]
            #[doc = "applies only to the buffer that is attached to the surface at commit"]
            #[doc = "time."]
            #[doc = ""]
            #[doc = "If the provided fd is not a valid dma_fence fd, then an INVALID_FENCE"]
            #[doc = "error is raised."]
            #[doc = ""]
            #[doc = "If a fence has already been attached during the same commit cycle, a"]
            #[doc = "DUPLICATE_FENCE error is raised."]
            #[doc = ""]
            #[doc = "If the associated wl_surface was destroyed, a NO_SURFACE error is"]
            #[doc = "raised."]
            #[doc = ""]
            #[doc = "If at surface commit time the attached buffer does not support explicit"]
            #[doc = "synchronization, an UNSUPPORTED_BUFFER error is raised."]
            #[doc = ""]
            #[doc = "If at surface commit time there is no buffer attached, a NO_BUFFER"]
            #[doc = "error is raised."]
            async fn set_acquire_fence(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_surface_synchronization_v1#{}.set_acquire_fence()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_fd(fd).build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a listener for the release of the buffer attached by the"]
            #[doc = "client with wl_surface.attach. See zwp_linux_buffer_release_v1"]
            #[doc = "documentation for more information."]
            #[doc = ""]
            #[doc = "The release object is double-buffered state, and will be associated"]
            #[doc = "with the buffer that is attached to the surface at wl_surface.commit"]
            #[doc = "time."]
            #[doc = ""]
            #[doc = "If a zwp_linux_buffer_release_v1 object has already been requested for"]
            #[doc = "the surface in the same commit cycle, a DUPLICATE_RELEASE error is"]
            #[doc = "raised."]
            #[doc = ""]
            #[doc = "If the associated wl_surface was destroyed, a NO_SURFACE error"]
            #[doc = "is raised."]
            #[doc = ""]
            #[doc = "If at surface commit time there is no buffer attached, a NO_BUFFER"]
            #[doc = "error is raised."]
            async fn get_release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                release: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_linux_surface_synchronization_v1#{}.get_release()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(release))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This object is instantiated in response to a"]
    #[doc = "zwp_linux_surface_synchronization_v1.get_release request."]
    #[doc = ""]
    #[doc = "It provides an alternative to wl_buffer.release events, providing a"]
    #[doc = "unique release from a single wl_surface.commit request. The release event"]
    #[doc = "also supports explicit synchronization, providing a fence FD for the"]
    #[doc = "client to synchronize against."]
    #[doc = ""]
    #[doc = "Exactly one event, either a fenced_release or an immediate_release, will"]
    #[doc = "be emitted for the wl_surface.commit request. The compositor can choose"]
    #[doc = "release by release which event it uses."]
    #[doc = ""]
    #[doc = "This event does not replace wl_buffer.release events; servers are still"]
    #[doc = "required to send those events."]
    #[doc = ""]
    #[doc = "Once a buffer release object has delivered a 'fenced_release' or an"]
    #[doc = "'immediate_release' event it is automatically destroyed."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_buffer_release_v1 {
        #[doc = "Trait to implement the zwp_linux_buffer_release_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxBufferReleaseV1 {
            const INTERFACE: &'static str = "zwp_linux_buffer_release_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Sent when the compositor has finalised its usage of the associated"]
            #[doc = "buffer for the relevant commit, providing a dma_fence which will be"]
            #[doc = "signaled when all operations by the compositor on that buffer for that"]
            #[doc = "commit have finished."]
            #[doc = ""]
            #[doc = "Once the fence has signaled, and assuming the associated buffer is not"]
            #[doc = "pending release from other wl_surface.commit requests, no additional"]
            #[doc = "explicit or implicit synchronization is required to safely reuse or"]
            #[doc = "destroy the buffer."]
            #[doc = ""]
            #[doc = "This event destroys the zwp_linux_buffer_release_v1 object."]
            async fn fenced_release(&self, fence: rustix::fd::OwnedFd)
                -> crate::client::Result<()>;
            #[doc = "Sent when the compositor has finalised its usage of the associated"]
            #[doc = "buffer for the relevant commit, and either performed no operations"]
            #[doc = "using it, or has a guarantee that all its operations on that buffer for"]
            #[doc = "that commit have finished."]
            #[doc = ""]
            #[doc = "Once this event is received, and assuming the associated buffer is not"]
            #[doc = "pending release from other wl_surface.commit requests, no additional"]
            #[doc = "explicit or implicit synchronization is required to safely reuse or"]
            #[doc = "destroy the buffer."]
            #[doc = ""]
            #[doc = "This event destroys the zwp_linux_buffer_release_v1 object."]
            async fn immediate_release(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol specifies a way for making it possible to reference a surface"]
#[doc = "of a different client. With such a reference, a client can, by using the"]
#[doc = "interfaces provided by this protocol, manipulate the relationship between"]
#[doc = "its own surfaces and the surface of some other client. For example, stack"]
#[doc = "some of its own surface above the other clients surface."]
#[doc = ""]
#[doc = "In order for a client A to get a reference of a surface of client B, client"]
#[doc = "B must first export its surface using xdg_exporter.export_toplevel. Upon"]
#[doc = "doing this, client B will receive a handle (a unique string) that it may"]
#[doc = "share with client A in some way (for example D-Bus). After client A has"]
#[doc = "received the handle from client B, it may use xdg_importer.import_toplevel"]
#[doc = "to create a reference to the surface client B just exported. See the"]
#[doc = "corresponding requests for details."]
#[doc = ""]
#[doc = "A possible use case for this is out-of-process dialogs. For example when a"]
#[doc = "sandboxed client without file system access needs the user to select a file"]
#[doc = "on the file system, given sandbox environment support, it can export its"]
#[doc = "surface, passing the exported surface handle to an unsandboxed process that"]
#[doc = "can show a file browser dialog and stack it above the sandboxed client's"]
#[doc = "surface."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and backward"]
#[doc = "incompatible changes may be made. Backward compatible changes may be added"]
#[doc = "together with the corresponding interface version bump. Backward"]
#[doc = "incompatible changes are done by bumping the version number in the protocol"]
#[doc = "and interface names and resetting the interface version. Once the protocol"]
#[doc = "is to be declared stable, the 'z' prefix and the version number in the"]
#[doc = "protocol and interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod xdg_foreign_unstable_v2 {
    #[doc = "A global interface used for exporting surfaces that can later be imported"]
    #[doc = "using xdg_importer."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_exporter_v2 {
        use futures_util::SinkExt;
        #[doc = "These errors can be emitted in response to invalid xdg_exporter"]
        #[doc = "requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface is not an xdg_toplevel"]
            InvalidSurface = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSurface),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_exporter_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgExporterV2 {
            const INTERFACE: &'static str = "zxdg_exporter_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the xdg_exporter object will no longer be"]
            #[doc = "used."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_exporter_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The export_toplevel request exports the passed surface so that it can later be"]
            #[doc = "imported via xdg_importer. When called, a new xdg_exported object will"]
            #[doc = "be created and xdg_exported.handle will be sent immediately. See the"]
            #[doc = "corresponding interface and event for details."]
            #[doc = ""]
            #[doc = "A surface may be exported multiple times, and each exported handle may"]
            #[doc = "be used to create an xdg_imported multiple times. Only xdg_toplevel"]
            #[doc = "equivalent surfaces may be exported, otherwise an invalid_surface"]
            #[doc = "protocol error is sent."]
            async fn export_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_exporter_v2#{}.export_toplevel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A global interface used for importing surfaces exported by xdg_exporter."]
    #[doc = "With this interface, a client can create a reference to a surface of"]
    #[doc = "another client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_importer_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_importer_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgImporterV2 {
            const INTERFACE: &'static str = "zxdg_importer_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the xdg_importer object will no longer be"]
            #[doc = "used."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_importer_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The import_toplevel request imports a surface from any client given a handle"]
            #[doc = "retrieved by exporting said surface using xdg_exporter.export_toplevel."]
            #[doc = "When called, a new xdg_imported object will be created. This new object"]
            #[doc = "represents the imported surface, and the importing client can"]
            #[doc = "manipulate its relationship using it. See xdg_imported for details."]
            async fn import_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                handle: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_importer_v2#{}.import_toplevel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_string(Some(handle))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An xdg_exported object represents an exported reference to a surface. The"]
    #[doc = "exported surface may be referenced as long as the xdg_exported object not"]
    #[doc = "destroyed. Destroying the xdg_exported invalidates any relationship the"]
    #[doc = "importer may have established using xdg_imported."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_exported_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_exported_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgExportedV2 {
            const INTERFACE: &'static str = "zxdg_exported_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Revoke the previously exported surface. This invalidates any"]
            #[doc = "relationship the importer may have set up using the xdg_imported created"]
            #[doc = "given the handle sent via xdg_exported.handle."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_exported_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The handle event contains the unique handle of this exported surface"]
            #[doc = "reference. It may be shared with any client, which then can use it to"]
            #[doc = "import the surface by calling xdg_importer.import_toplevel. A handle"]
            #[doc = "may be used to import the surface multiple times."]
            async fn handle(&self, handle: String) -> crate::client::Result<()>;
        }
    }
    #[doc = "An xdg_imported object represents an imported reference to surface exported"]
    #[doc = "by some client. A client can use this interface to manipulate"]
    #[doc = "relationships between its own surfaces and the imported surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_imported_v2 {
        use futures_util::SinkExt;
        #[doc = "These errors can be emitted in response to invalid xdg_imported"]
        #[doc = "requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface is not an xdg_toplevel"]
            InvalidSurface = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSurface),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_imported_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgImportedV2 {
            const INTERFACE: &'static str = "zxdg_imported_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that it will no longer use the xdg_imported"]
            #[doc = "object. Any relationship that may have been set up will at this point"]
            #[doc = "be invalidated."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_imported_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the imported surface as the parent of some surface of the client."]
            #[doc = "The passed surface must be an xdg_toplevel equivalent, otherwise an"]
            #[doc = "invalid_surface protocol error is sent. Calling this function sets up"]
            #[doc = "a surface to surface relation with the same stacking and positioning"]
            #[doc = "semantics as xdg_toplevel.set_parent."]
            async fn set_parent_of(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_imported_v2#{}.set_parent_of()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The imported surface handle has been destroyed and any relationship set"]
            #[doc = "up has been invalidated. This may happen for various reasons, for"]
            #[doc = "example if the exported surface or the exported surface handle has been"]
            #[doc = "destroyed, if the handle used for importing was invalid."]
            async fn destroyed(&self) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod pointer_gestures_unstable_v1 {
    #[doc = "A global interface to provide semantic touchpad gestures for a given"]
    #[doc = "pointer."]
    #[doc = ""]
    #[doc = "Three gestures are currently supported: swipe, pinch, and hold."]
    #[doc = "Pinch and swipe gestures follow a three-stage cycle: begin, update,"]
    #[doc = "end. Hold gestures follow a two-stage cycle: begin and end. All"]
    #[doc = "gestures are identified by a unique id."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_pointer_gestures_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_pointer_gestures_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGesturesV1 {
            const INTERFACE: &'static str = "zwp_pointer_gestures_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a swipe gesture object. See the"]
            #[doc = "wl_pointer_gesture_swipe interface for details."]
            async fn get_swipe_gesture(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_pointer_gestures_v1#{}.get_swipe_gesture()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(pointer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a pinch gesture object. See the"]
            #[doc = "wl_pointer_gesture_pinch interface for details."]
            async fn get_pinch_gesture(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_pointer_gestures_v1#{}.get_pinch_gesture()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(pointer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the pointer gesture object. Swipe, pinch and hold objects"]
            #[doc = "created via this gesture object remain valid."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_pointer_gestures_v1#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a hold gesture object. See the"]
            #[doc = "wl_pointer_gesture_hold interface for details."]
            async fn get_hold_gesture(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_pointer_gestures_v1#{}.get_hold_gesture()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(pointer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A swipe gesture object notifies a client about a multi-finger swipe"]
    #[doc = "gesture detected on an indirect input device such as a touchpad."]
    #[doc = "The gesture is usually initiated by multiple fingers moving in the"]
    #[doc = "same direction but once initiated the direction may change."]
    #[doc = "The precise conditions of when such a gesture is detected are"]
    #[doc = "implementation-dependent."]
    #[doc = ""]
    #[doc = "A gesture consists of three stages: begin, update (optional) and end."]
    #[doc = "There cannot be multiple simultaneous hold, pinch or swipe gestures on a"]
    #[doc = "same pointer/seat, how compositors prevent these situations is"]
    #[doc = "implementation-dependent."]
    #[doc = ""]
    #[doc = "A gesture may be cancelled by the compositor or the hardware."]
    #[doc = "Clients should not consider performing permanent or irreversible"]
    #[doc = "actions until the end of a gesture has been received."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_pointer_gesture_swipe_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_pointer_gesture_swipe_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGestureSwipeV1 {
            const INTERFACE: &'static str = "zwp_pointer_gesture_swipe_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_pointer_gesture_swipe_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent when a multi-finger swipe gesture is detected"]
            #[doc = "on the device."]
            async fn begin(
                &self,
                serial: u32,
                time: u32,
                surface: crate::wire::ObjectId,
                fingers: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent when a multi-finger swipe gesture changes the"]
            #[doc = "position of the logical center."]
            #[doc = ""]
            #[doc = "The dx and dy coordinates are relative coordinates of the logical"]
            #[doc = "center of the gesture compared to the previous event."]
            async fn update(
                &self,
                time: u32,
                dx: crate::wire::Fixed,
                dy: crate::wire::Fixed,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent when a multi-finger swipe gesture ceases to"]
            #[doc = "be valid. This may happen when one or more fingers are lifted or"]
            #[doc = "the gesture is cancelled."]
            #[doc = ""]
            #[doc = "When a gesture is cancelled, the client should undo state changes"]
            #[doc = "caused by this gesture. What causes a gesture to be cancelled is"]
            #[doc = "implementation-dependent."]
            async fn end(
                &self,
                serial: u32,
                time: u32,
                cancelled: i32,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "A pinch gesture object notifies a client about a multi-finger pinch"]
    #[doc = "gesture detected on an indirect input device such as a touchpad."]
    #[doc = "The gesture is usually initiated by multiple fingers moving towards"]
    #[doc = "each other or away from each other, or by two or more fingers rotating"]
    #[doc = "around a logical center of gravity. The precise conditions of when"]
    #[doc = "such a gesture is detected are implementation-dependent."]
    #[doc = ""]
    #[doc = "A gesture consists of three stages: begin, update (optional) and end."]
    #[doc = "There cannot be multiple simultaneous hold, pinch or swipe gestures on a"]
    #[doc = "same pointer/seat, how compositors prevent these situations is"]
    #[doc = "implementation-dependent."]
    #[doc = ""]
    #[doc = "A gesture may be cancelled by the compositor or the hardware."]
    #[doc = "Clients should not consider performing permanent or irreversible"]
    #[doc = "actions until the end of a gesture has been received."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_pointer_gesture_pinch_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_pointer_gesture_pinch_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGesturePinchV1 {
            const INTERFACE: &'static str = "zwp_pointer_gesture_pinch_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_pointer_gesture_pinch_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent when a multi-finger pinch gesture is detected"]
            #[doc = "on the device."]
            async fn begin(
                &self,
                serial: u32,
                time: u32,
                surface: crate::wire::ObjectId,
                fingers: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent when a multi-finger pinch gesture changes the"]
            #[doc = "position of the logical center, the rotation or the relative scale."]
            #[doc = ""]
            #[doc = "The dx and dy coordinates are relative coordinates in the"]
            #[doc = "surface coordinate space of the logical center of the gesture."]
            #[doc = ""]
            #[doc = "The scale factor is an absolute scale compared to the"]
            #[doc = "pointer_gesture_pinch.begin event, e.g. a scale of 2 means the fingers"]
            #[doc = "are now twice as far apart as on pointer_gesture_pinch.begin."]
            #[doc = ""]
            #[doc = "The rotation is the relative angle in degrees clockwise compared to the previous"]
            #[doc = "pointer_gesture_pinch.begin or pointer_gesture_pinch.update event."]
            async fn update(
                &self,
                time: u32,
                dx: crate::wire::Fixed,
                dy: crate::wire::Fixed,
                scale: crate::wire::Fixed,
                rotation: crate::wire::Fixed,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent when a multi-finger pinch gesture ceases to"]
            #[doc = "be valid. This may happen when one or more fingers are lifted or"]
            #[doc = "the gesture is cancelled."]
            #[doc = ""]
            #[doc = "When a gesture is cancelled, the client should undo state changes"]
            #[doc = "caused by this gesture. What causes a gesture to be cancelled is"]
            #[doc = "implementation-dependent."]
            async fn end(
                &self,
                serial: u32,
                time: u32,
                cancelled: i32,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "A hold gesture object notifies a client about a single- or"]
    #[doc = "multi-finger hold gesture detected on an indirect input device such as"]
    #[doc = "a touchpad. The gesture is usually initiated by one or more fingers"]
    #[doc = "being held down without significant movement. The precise conditions"]
    #[doc = "of when such a gesture is detected are implementation-dependent."]
    #[doc = ""]
    #[doc = "In particular, this gesture may be used to cancel kinetic scrolling."]
    #[doc = ""]
    #[doc = "A hold gesture consists of two stages: begin and end. Unlike pinch and"]
    #[doc = "swipe there is no update stage."]
    #[doc = "There cannot be multiple simultaneous hold, pinch or swipe gestures on a"]
    #[doc = "same pointer/seat, how compositors prevent these situations is"]
    #[doc = "implementation-dependent."]
    #[doc = ""]
    #[doc = "A gesture may be cancelled by the compositor or the hardware."]
    #[doc = "Clients should not consider performing permanent or irreversible"]
    #[doc = "actions until the end of a gesture has been received."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_pointer_gesture_hold_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_pointer_gesture_hold_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGestureHoldV1 {
            const INTERFACE: &'static str = "zwp_pointer_gesture_hold_v1";
            const VERSION: u32 = 3u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_pointer_gesture_hold_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent when a hold gesture is detected on the device."]
            async fn begin(
                &self,
                serial: u32,
                time: u32,
                surface: crate::wire::ObjectId,
                fingers: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event is sent when a hold gesture ceases to"]
            #[doc = "be valid. This may happen when the holding fingers are lifted or"]
            #[doc = "the gesture is cancelled, for example if the fingers move past an"]
            #[doc = "implementation-defined threshold, the finger count changes or the hold"]
            #[doc = "gesture changes into a different type of gesture."]
            #[doc = ""]
            #[doc = "When a gesture is cancelled, the client may need to undo state changes"]
            #[doc = "caused by this gesture. What causes a gesture to be cancelled is"]
            #[doc = "implementation-dependent."]
            async fn end(
                &self,
                serial: u32,
                time: u32,
                cancelled: i32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol specifies a set of interfaces used for making clients able to"]
#[doc = "receive relative pointer events not obstructed by barriers (such as the"]
#[doc = "monitor edge or other pointer barriers)."]
#[doc = ""]
#[doc = "To start receiving relative pointer events, a client must first bind the"]
#[doc = "global interface \"wp_relative_pointer_manager\" which, if a compositor"]
#[doc = "supports relative pointer motion events, is exposed by the registry. After"]
#[doc = "having created the relative pointer manager proxy object, the client uses"]
#[doc = "it to create the actual relative pointer object using the"]
#[doc = "\"get_relative_pointer\" request given a wl_pointer. The relative pointer"]
#[doc = "motion events will then, when applicable, be transmitted via the proxy of"]
#[doc = "the newly created relative pointer object. See the documentation of the"]
#[doc = "relative pointer interface for more details."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and backward"]
#[doc = "incompatible changes may be made. Backward compatible changes may be added"]
#[doc = "together with the corresponding interface version bump. Backward"]
#[doc = "incompatible changes are done by bumping the version number in the protocol"]
#[doc = "and interface names and resetting the interface version. Once the protocol"]
#[doc = "is to be declared stable, the 'z' prefix and the version number in the"]
#[doc = "protocol and interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod relative_pointer_unstable_v1 {
    #[doc = "A global interface used for getting the relative pointer object for a"]
    #[doc = "given pointer."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_relative_pointer_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_relative_pointer_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpRelativePointerManagerV1 {
            const INTERFACE: &'static str = "zwp_relative_pointer_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Used by the client to notify the server that it will no longer use this"]
            #[doc = "relative pointer manager object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_relative_pointer_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a relative pointer interface given a wl_pointer object. See the"]
            #[doc = "wp_relative_pointer interface for more details."]
            async fn get_relative_pointer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_relative_pointer_manager_v1#{}.get_relative_pointer()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(pointer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A wp_relative_pointer object is an extension to the wl_pointer interface"]
    #[doc = "used for emitting relative pointer events. It shares the same focus as"]
    #[doc = "wl_pointer objects of the same seat and will only emit events when it has"]
    #[doc = "focus."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_relative_pointer_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_relative_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpRelativePointerV1 {
            const INTERFACE: &'static str = "zwp_relative_pointer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_relative_pointer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Relative x/y pointer motion from the pointer of the seat associated with"]
            #[doc = "this object."]
            #[doc = ""]
            #[doc = "A relative motion is in the same dimension as regular wl_pointer motion"]
            #[doc = "events, except they do not represent an absolute position. For example,"]
            #[doc = "moving a pointer from (x, y) to (x', y') would have the equivalent"]
            #[doc = "relative motion (x' - x, y' - y). If a pointer motion caused the"]
            #[doc = "absolute pointer position to be clipped by for example the edge of the"]
            #[doc = "monitor, the relative motion is unaffected by the clipping and will"]
            #[doc = "represent the unclipped motion."]
            #[doc = ""]
            #[doc = "This event also contains non-accelerated motion deltas. The"]
            #[doc = "non-accelerated delta is, when applicable, the regular pointer motion"]
            #[doc = "delta as it was before having applied motion acceleration and other"]
            #[doc = "transformations such as normalization."]
            #[doc = ""]
            #[doc = "Note that the non-accelerated delta does not represent 'raw' events as"]
            #[doc = "they were read from some device. Pointer motion acceleration is device-"]
            #[doc = "and configuration-specific and non-accelerated deltas and accelerated"]
            #[doc = "deltas may have the same value on some devices."]
            #[doc = ""]
            #[doc = "Relative motions are not coupled to wl_pointer.motion events, and can be"]
            #[doc = "sent in combination with such events, but also independently. There may"]
            #[doc = "also be scenarios where wl_pointer.motion is sent, but there is no"]
            #[doc = "relative motion. The order of an absolute and relative motion event"]
            #[doc = "originating from the same physical motion is not guaranteed."]
            #[doc = ""]
            #[doc = "If the client needs button events or focus state, it can receive them"]
            #[doc = "from a wl_pointer object of the same seat that the wp_relative_pointer"]
            #[doc = "object is associated with."]
            async fn relative_motion(
                &self,
                utime_hi: u32,
                utime_lo: u32,
                dx: crate::wire::Fixed,
                dy: crate::wire::Fixed,
                dx_unaccel: crate::wire::Fixed,
                dy_unaccel: crate::wire::Fixed,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This description provides a high-level overview of the interplay between"]
#[doc = "the interfaces defined this protocol. For details, see the protocol"]
#[doc = "specification."]
#[doc = ""]
#[doc = "More than one tablet may exist, and device-specifics matter. Tablets are"]
#[doc = "not represented by a single virtual device like wl_pointer. A client"]
#[doc = "binds to the tablet manager object which is just a proxy object. From"]
#[doc = "that, the client requests wp_tablet_manager.get_tablet_seat(wl_seat)"]
#[doc = "and that returns the actual interface that has all the tablets. With"]
#[doc = "this indirection, we can avoid merging wp_tablet into the actual Wayland"]
#[doc = "protocol, a long-term benefit."]
#[doc = ""]
#[doc = "The wp_tablet_seat sends a \"tablet added\" event for each tablet"]
#[doc = "connected. That event is followed by descriptive events about the"]
#[doc = "hardware; currently that includes events for name, vid/pid and"]
#[doc = "a wp_tablet.path event that describes a local path. This path can be"]
#[doc = "used to uniquely identify a tablet or get more information through"]
#[doc = "libwacom. Emulated or nested tablets can skip any of those, e.g. a"]
#[doc = "virtual tablet may not have a vid/pid. The sequence of descriptive"]
#[doc = "events is terminated by a wp_tablet.done event to signal that a client"]
#[doc = "may now finalize any initialization for that tablet."]
#[doc = ""]
#[doc = "Events from tablets require a tool in proximity. Tools are also managed"]
#[doc = "by the tablet seat; a \"tool added\" event is sent whenever a tool is new"]
#[doc = "to the compositor. That event is followed by a number of descriptive"]
#[doc = "events about the hardware; currently that includes capabilities,"]
#[doc = "hardware id and serial number, and tool type. Similar to the tablet"]
#[doc = "interface, a wp_tablet_tool.done event is sent to terminate that initial"]
#[doc = "sequence."]
#[doc = ""]
#[doc = "Any event from a tool happens on the wp_tablet_tool interface. When the"]
#[doc = "tool gets into proximity of the tablet, a proximity_in event is sent on"]
#[doc = "the wp_tablet_tool interface, listing the tablet and the surface. That"]
#[doc = "event is followed by a motion event with the coordinates. After that,"]
#[doc = "it's the usual motion, axis, button, etc. events. The protocol's"]
#[doc = "serialisation means events are grouped by wp_tablet_tool.frame events."]
#[doc = ""]
#[doc = "Two special events (that don't exist in X) are down and up. They signal"]
#[doc = "\"tip touching the surface\". For tablets without real proximity"]
#[doc = "detection, the sequence is: proximity_in, motion, down, frame."]
#[doc = ""]
#[doc = "When the tool leaves proximity, a proximity_out event is sent. If any"]
#[doc = "button is still down, a button release event is sent before this"]
#[doc = "proximity event. These button events are sent in the same frame as the"]
#[doc = "proximity event to signal to the client that the buttons were held when"]
#[doc = "the tool left proximity."]
#[doc = ""]
#[doc = "If the tool moves out of the surface but stays in proximity (i.e."]
#[doc = "between windows), compositor-specific grab policies apply. This usually"]
#[doc = "means that the proximity-out is delayed until all buttons are released."]
#[doc = ""]
#[doc = "Moving a tool physically from one tablet to the other has no real effect"]
#[doc = "on the protocol, since we already have the tool object from the \"tool"]
#[doc = "added\" event. All the information is already there and the proximity"]
#[doc = "events on both tablets are all a client needs to reconstruct what"]
#[doc = "happened."]
#[doc = ""]
#[doc = "Some extra axes are normalized, i.e. the client knows the range as"]
#[doc = "specified in the protocol (e.g. [0, 65535]), the granularity however is"]
#[doc = "unknown. The current normalized axes are pressure, distance, and slider."]
#[doc = ""]
#[doc = "Other extra axes are in physical units as specified in the protocol."]
#[doc = "The current extra axes with physical units are tilt, rotation and"]
#[doc = "wheel rotation."]
#[doc = ""]
#[doc = "Since tablets work independently of the pointer controlled by the mouse,"]
#[doc = "the focus handling is independent too and controlled by proximity."]
#[doc = "The wp_tablet_tool.set_cursor request sets a tool-specific cursor."]
#[doc = "This cursor surface may be the same as the mouse cursor, and it may be"]
#[doc = "the same across tools but it is possible to be more fine-grained. For"]
#[doc = "example, a client may set different cursors for the pen and eraser."]
#[doc = ""]
#[doc = "Tools are generally independent of tablets and it is"]
#[doc = "compositor-specific policy when a tool can be removed. Common approaches"]
#[doc = "will likely include some form of removing a tool when all tablets the"]
#[doc = "tool was used on are removed."]
#[doc = ""]
#[doc = "Disclaimer: This protocol extension has been marked stable. This copy is"]
#[doc = "no longer used and only retained for backwards compatibility. The"]
#[doc = "canonical version can be found in the stable/ directory."]
#[allow(clippy::module_inception)]
pub mod tablet_unstable_v2 {
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "system. All tablets are associated with a seat, to get access to the"]
    #[doc = "actual tablets, use wp_tablet_manager.get_tablet_seat."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_manager_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV2 {
            const INTERFACE: &'static str = "zwp_tablet_manager_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Get the wp_tablet_seat object for the given seat. This object"]
            #[doc = "provides access to all graphics tablets in this seat."]
            async fn get_tablet_seat(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                tablet_seat: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_manager_v2#{}.get_tablet_seat()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(tablet_seat))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the wp_tablet_manager object. Objects created from this"]
            #[doc = "object are unaffected and should be destroyed separately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_manager_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "seat. After binding to this interface, the compositor sends a set of"]
    #[doc = "wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_seat_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_seat_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV2 {
            const INTERFACE: &'static str = "zwp_tablet_seat_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_tablet_seat object. Objects created from this"]
            #[doc = "object are unaffected and should be destroyed separately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_seat_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent whenever a new tablet becomes available on this"]
            #[doc = "seat. This event only provides the object id of the tablet, any"]
            #[doc = "static information about the tablet (device name, vid/pid, etc.) is"]
            #[doc = "sent through the wp_tablet interface."]
            async fn tablet_added(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event is sent whenever a tool that has not previously been used"]
            #[doc = "with a tablet comes into use. This event only provides the object id"]
            #[doc = "of the tool; any static information about the tool (capabilities,"]
            #[doc = "type, etc.) is sent through the wp_tablet_tool interface."]
            async fn tool_added(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event is sent whenever a new pad is known to the system. Typically,"]
            #[doc = "pads are physically attached to tablets and a pad_added event is"]
            #[doc = "sent immediately after the wp_tablet_seat.tablet_added."]
            #[doc = "However, some standalone pad devices logically attach to tablets at"]
            #[doc = "runtime, and the client must wait for wp_tablet_pad.enter to know"]
            #[doc = "the tablet a pad is attached to."]
            #[doc = ""]
            #[doc = "This event only provides the object id of the pad. All further"]
            #[doc = "features (buttons, strips, rings) are sent through the wp_tablet_pad"]
            #[doc = "interface."]
            async fn pad_added(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
        }
    }
    #[doc = "An object that represents a physical tool that has been, or is"]
    #[doc = "currently in use with a tablet in this seat. Each wp_tablet_tool"]
    #[doc = "object stays valid until the client destroys it; the compositor"]
    #[doc = "reuses the wp_tablet_tool object to indicate that the object's"]
    #[doc = "respective physical tool has come into proximity of a tablet again."]
    #[doc = ""]
    #[doc = "A wp_tablet_tool object's relation to a physical tool depends on the"]
    #[doc = "tablet's ability to report serial numbers. If the tablet supports"]
    #[doc = "this capability, then the object represents a specific physical tool"]
    #[doc = "and can be identified even when used on multiple tablets."]
    #[doc = ""]
    #[doc = "A tablet tool has a number of static characteristics, e.g. tool type,"]
    #[doc = "hardware_serial and capabilities. These capabilities are sent in an"]
    #[doc = "event sequence after the wp_tablet_seat.tool_added event before any"]
    #[doc = "actual events from this tool. This initial event sequence is"]
    #[doc = "terminated by a wp_tablet_tool.done event."]
    #[doc = ""]
    #[doc = "Tablet tool events are grouped by wp_tablet_tool.frame events."]
    #[doc = "Any events received before a wp_tablet_tool.frame event should be"]
    #[doc = "considered part of the same hardware state change."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_tool_v2 {
        use futures_util::SinkExt;
        #[doc = "Describes the physical type of a tool. The physical type of a tool"]
        #[doc = "generally defines its base usage."]
        #[doc = ""]
        #[doc = "The mouse tool represents a mouse-shaped tool that is not a relative"]
        #[doc = "device but bound to the tablet's surface, providing absolute"]
        #[doc = "coordinates."]
        #[doc = ""]
        #[doc = "The lens tool is a mouse-shaped tool with an attached lens to"]
        #[doc = "provide precision focus."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Type {
            #[doc = "Pen"]
            Pen = 320u32,
            #[doc = "Eraser"]
            Eraser = 321u32,
            #[doc = "Brush"]
            Brush = 322u32,
            #[doc = "Pencil"]
            Pencil = 323u32,
            #[doc = "Airbrush"]
            Airbrush = 324u32,
            #[doc = "Finger"]
            Finger = 325u32,
            #[doc = "Mouse"]
            Mouse = 326u32,
            #[doc = "Lens"]
            Lens = 327u32,
        }
        impl TryFrom<u32> for Type {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    320u32 => Ok(Self::Pen),
                    321u32 => Ok(Self::Eraser),
                    322u32 => Ok(Self::Brush),
                    323u32 => Ok(Self::Pencil),
                    324u32 => Ok(Self::Airbrush),
                    325u32 => Ok(Self::Finger),
                    326u32 => Ok(Self::Mouse),
                    327u32 => Ok(Self::Lens),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Describes extra capabilities on a tablet."]
        #[doc = ""]
        #[doc = "Any tool must provide x and y values, extra axes are"]
        #[doc = "device-specific."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Capability {
            #[doc = "Tilt axes"]
            Tilt = 1u32,
            #[doc = "Pressure axis"]
            Pressure = 2u32,
            #[doc = "Distance axis"]
            Distance = 3u32,
            #[doc = "Z-rotation axis"]
            Rotation = 4u32,
            #[doc = "Slider axis"]
            Slider = 5u32,
            #[doc = "Wheel axis"]
            Wheel = 6u32,
        }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Tilt),
                    2u32 => Ok(Self::Pressure),
                    3u32 => Ok(Self::Distance),
                    4u32 => Ok(Self::Rotation),
                    5u32 => Ok(Self::Slider),
                    6u32 => Ok(Self::Wheel),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Describes the physical state of a button that produced the button event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ButtonState {
            #[doc = "button is not pressed"]
            Released = 0u32,
            #[doc = "button is pressed"]
            Pressed = 1u32,
        }
        impl TryFrom<u32> for ButtonState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Released),
                    1u32 => Ok(Self::Pressed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ButtonState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_tool_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletToolV2 {
            const INTERFACE: &'static str = "zwp_tablet_tool_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Sets the surface of the cursor used for this tool on the given"]
            #[doc = "tablet. This request only takes effect if the tool is in proximity"]
            #[doc = "of one of the requesting client's surfaces or the surface parameter"]
            #[doc = "is the current pointer surface. If there was a previous surface set"]
            #[doc = "with this request it is replaced. If surface is NULL, the cursor"]
            #[doc = "image is hidden."]
            #[doc = ""]
            #[doc = "The parameters hotspot_x and hotspot_y define the position of the"]
            #[doc = "pointer surface relative to the pointer location. Its top-left corner"]
            #[doc = "is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the"]
            #[doc = "coordinates of the pointer location, in surface-local coordinates."]
            #[doc = ""]
            #[doc = "On surface.attach requests to the pointer surface, hotspot_x and"]
            #[doc = "hotspot_y are decremented by the x and y parameters passed to the"]
            #[doc = "request. Attach must be confirmed by wl_surface.commit as usual."]
            #[doc = ""]
            #[doc = "The hotspot can also be updated by passing the currently set pointer"]
            #[doc = "surface to this request with new values for hotspot_x and hotspot_y."]
            #[doc = ""]
            #[doc = "The current and pending input regions of the wl_surface are cleared,"]
            #[doc = "and wl_surface.set_input_region is ignored until the wl_surface is no"]
            #[doc = "longer used as the cursor. When the use as a cursor ends, the current"]
            #[doc = "and pending input regions become undefined, and the wl_surface is"]
            #[doc = "unmapped."]
            #[doc = ""]
            #[doc = "This request gives the surface the role of a wp_tablet_tool cursor. A"]
            #[doc = "surface may only ever be used as the cursor surface for one"]
            #[doc = "wp_tablet_tool. If the surface already has another role or has"]
            #[doc = "previously been used as cursor surface for a different tool, a"]
            #[doc = "protocol error is raised."]
            async fn set_cursor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                surface: Option<crate::wire::ObjectId>,
                hotspot_x: i32,
                hotspot_y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_tool_v2#{}.set_cursor()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(surface)
                    .put_int(hotspot_x)
                    .put_int(hotspot_y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This destroys the client's resource for this tool object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_tool_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The tool type is the high-level type of the tool and usually decides"]
            #[doc = "the interaction expected from this tool."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn r#type(&self, tool_type: Type) -> crate::client::Result<()>;
            #[doc = "If the physical tool can be identified by a unique 64-bit serial"]
            #[doc = "number, this event notifies the client of this serial number."]
            #[doc = ""]
            #[doc = "If multiple tablets are available in the same seat and the tool is"]
            #[doc = "uniquely identifiable by the serial number, that tool may move"]
            #[doc = "between tablets."]
            #[doc = ""]
            #[doc = "Otherwise, if the tool has no serial number and this event is"]
            #[doc = "missing, the tool is tied to the tablet it first comes into"]
            #[doc = "proximity with. Even if the physical tool is used on multiple"]
            #[doc = "tablets, separate wp_tablet_tool objects will be created, one per"]
            #[doc = "tablet."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn hardware_serial(
                &self,
                hardware_serial_hi: u32,
                hardware_serial_lo: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event notifies the client of a hardware id available on this tool."]
            #[doc = ""]
            #[doc = "The hardware id is a device-specific 64-bit id that provides extra"]
            #[doc = "information about the tool in use, beyond the wl_tool.type"]
            #[doc = "enumeration. The format of the id is specific to tablets made by"]
            #[doc = "Wacom Inc. For example, the hardware id of a Wacom Grip"]
            #[doc = "Pen (a stylus) is 0x802."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn hardware_id_wacom(
                &self,
                hardware_id_hi: u32,
                hardware_id_lo: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event notifies the client of any capabilities of this tool,"]
            #[doc = "beyond the main set of x/y axes and tip up/down detection."]
            #[doc = ""]
            #[doc = "One event is sent for each extra capability available on this tool."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn capability(&self, capability: Capability) -> crate::client::Result<()>;
            #[doc = "This event signals the end of the initial burst of descriptive"]
            #[doc = "events. A client may consider the static description of the tool to"]
            #[doc = "be complete and finalize initialization of the tool."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "This event is sent when the tool is removed from the system and will"]
            #[doc = "send no further events. Should the physical tool come back into"]
            #[doc = "proximity later, a new wp_tablet_tool object will be created."]
            #[doc = ""]
            #[doc = "It is compositor-dependent when a tool is removed. A compositor may"]
            #[doc = "remove a tool on proximity out, tablet removal or any other reason."]
            #[doc = "A compositor may also keep a tool alive until shutdown."]
            #[doc = ""]
            #[doc = "If the tool is currently in proximity, a proximity_out event will be"]
            #[doc = "sent before the removed event. See wp_tablet_tool.proximity_out for"]
            #[doc = "the handling of any buttons logically down."]
            #[doc = ""]
            #[doc = "When this event is received, the client must wp_tablet_tool.destroy"]
            #[doc = "the object."]
            async fn removed(&self) -> crate::client::Result<()>;
            #[doc = "Notification that this tool is focused on a certain surface."]
            #[doc = ""]
            #[doc = "This event can be received when the tool has moved from one surface to"]
            #[doc = "another, or when the tool has come back into proximity above the"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "If any button is logically down when the tool comes into proximity,"]
            #[doc = "the respective button event is sent after the proximity_in event but"]
            #[doc = "within the same frame as the proximity_in event."]
            async fn proximity_in(
                &self,
                serial: u32,
                tablet: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "Notification that this tool has either left proximity, or is no"]
            #[doc = "longer focused on a certain surface."]
            #[doc = ""]
            #[doc = "When the tablet tool leaves proximity of the tablet, button release"]
            #[doc = "events are sent for each button that was held down at the time of"]
            #[doc = "leaving proximity. These events are sent before the proximity_out"]
            #[doc = "event but within the same wp_tablet.frame."]
            #[doc = ""]
            #[doc = "If the tool stays within proximity of the tablet, but the focus"]
            #[doc = "changes from one surface to another, a button release event may not"]
            #[doc = "be sent until the button is actually released or the tool leaves the"]
            #[doc = "proximity of the tablet."]
            async fn proximity_out(&self) -> crate::client::Result<()>;
            #[doc = "Sent whenever the tablet tool comes in contact with the surface of the"]
            #[doc = "tablet."]
            #[doc = ""]
            #[doc = "If the tool is already in contact with the tablet when entering the"]
            #[doc = "input region, the client owning said region will receive a"]
            #[doc = "wp_tablet.proximity_in event, followed by a wp_tablet.down"]
            #[doc = "event and a wp_tablet.frame event."]
            #[doc = ""]
            #[doc = "Note that this event describes logical contact, not physical"]
            #[doc = "contact. On some devices, a compositor may not consider a tool in"]
            #[doc = "logical contact until a minimum physical pressure threshold is"]
            #[doc = "exceeded."]
            async fn down(&self, serial: u32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the tablet tool stops making contact with the surface of"]
            #[doc = "the tablet, or when the tablet tool moves out of the input region"]
            #[doc = "and the compositor grab (if any) is dismissed."]
            #[doc = ""]
            #[doc = "If the tablet tool moves out of the input region while in contact"]
            #[doc = "with the surface of the tablet and the compositor does not have an"]
            #[doc = "ongoing grab on the surface, the client owning said region will"]
            #[doc = "receive a wp_tablet.up event, followed by a wp_tablet.proximity_out"]
            #[doc = "event and a wp_tablet.frame event. If the compositor has an ongoing"]
            #[doc = "grab on this device, this event sequence is sent whenever the grab"]
            #[doc = "is dismissed in the future."]
            #[doc = ""]
            #[doc = "Note that this event describes logical contact, not physical"]
            #[doc = "contact. On some devices, a compositor may not consider a tool out"]
            #[doc = "of logical contact until physical pressure falls below a specific"]
            #[doc = "threshold."]
            async fn up(&self) -> crate::client::Result<()>;
            #[doc = "Sent whenever a tablet tool moves."]
            async fn motion(
                &self,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::client::Result<()>;
            #[doc = "Sent whenever the pressure axis on a tool changes. The value of this"]
            #[doc = "event is normalized to a value between 0 and 65535."]
            #[doc = ""]
            #[doc = "Note that pressure may be nonzero even when a tool is not in logical"]
            #[doc = "contact. See the down and up events for more details."]
            async fn pressure(&self, pressure: u32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the distance axis on a tool changes. The value of this"]
            #[doc = "event is normalized to a value between 0 and 65535."]
            #[doc = ""]
            #[doc = "Note that distance may be nonzero even when a tool is not in logical"]
            #[doc = "contact. See the down and up events for more details."]
            async fn distance(&self, distance: u32) -> crate::client::Result<()>;
            #[doc = "Sent whenever one or both of the tilt axes on a tool change. Each tilt"]
            #[doc = "value is in degrees, relative to the z-axis of the tablet."]
            #[doc = "The angle is positive when the top of a tool tilts along the"]
            #[doc = "positive x or y axis."]
            async fn tilt(
                &self,
                tilt_x: crate::wire::Fixed,
                tilt_y: crate::wire::Fixed,
            ) -> crate::client::Result<()>;
            #[doc = "Sent whenever the z-rotation axis on the tool changes. The"]
            #[doc = "rotation value is in degrees clockwise from the tool's"]
            #[doc = "logical neutral position."]
            async fn rotation(&self, degrees: crate::wire::Fixed) -> crate::client::Result<()>;
            #[doc = "Sent whenever the slider position on the tool changes. The"]
            #[doc = "value is normalized between -65535 and 65535, with 0 as the logical"]
            #[doc = "neutral position of the slider."]
            #[doc = ""]
            #[doc = "The slider is available on e.g. the Wacom Airbrush tool."]
            async fn slider(&self, position: i32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the wheel on the tool emits an event. This event"]
            #[doc = "contains two values for the same axis change. The degrees value is"]
            #[doc = "in the same orientation as the wl_pointer.vertical_scroll axis. The"]
            #[doc = "clicks value is in discrete logical clicks of the mouse wheel. This"]
            #[doc = "value may be zero if the movement of the wheel was less"]
            #[doc = "than one logical click."]
            #[doc = ""]
            #[doc = "Clients should choose either value and avoid mixing degrees and"]
            #[doc = "clicks. The compositor may accumulate values smaller than a logical"]
            #[doc = "click and emulate click events when a certain threshold is met."]
            #[doc = "Thus, wl_tablet_tool.wheel events with non-zero clicks values may"]
            #[doc = "have different degrees values."]
            async fn wheel(
                &self,
                degrees: crate::wire::Fixed,
                clicks: i32,
            ) -> crate::client::Result<()>;
            #[doc = "Sent whenever a button on the tool is pressed or released."]
            #[doc = ""]
            #[doc = "If a button is held down when the tool moves in or out of proximity,"]
            #[doc = "button events are generated by the compositor. See"]
            #[doc = "wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for"]
            #[doc = "details."]
            async fn button(
                &self,
                serial: u32,
                button: u32,
                state: ButtonState,
            ) -> crate::client::Result<()>;
            #[doc = "Marks the end of a series of axis and/or button updates from the"]
            #[doc = "tablet. The Wayland protocol requires axis updates to be sent"]
            #[doc = "sequentially, however all events within a frame should be considered"]
            #[doc = "one hardware event."]
            async fn frame(&self, time: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "The wp_tablet interface represents one graphics tablet device. The"]
    #[doc = "tablet interface itself does not generate events; all events are"]
    #[doc = "generated by wp_tablet_tool objects when in proximity above a tablet."]
    #[doc = ""]
    #[doc = "A tablet has a number of static characteristics, e.g. device name and"]
    #[doc = "pid/vid. These capabilities are sent in an event sequence after the"]
    #[doc = "wp_tablet_seat.tablet_added event. This initial event sequence is"]
    #[doc = "terminated by a wp_tablet.done event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV2 {
            const INTERFACE: &'static str = "zwp_tablet_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This destroys the client's resource for this tablet object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "A descriptive name for the tablet device."]
            #[doc = ""]
            #[doc = "If the device has no descriptive name, this event is not sent."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            async fn name(&self, name: String) -> crate::client::Result<()>;
            #[doc = "The USB vendor and product IDs for the tablet device."]
            #[doc = ""]
            #[doc = "If the device has no USB vendor/product ID, this event is not sent."]
            #[doc = "This can happen for virtual devices or non-USB devices, for instance."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            async fn id(&self, vid: u32, pid: u32) -> crate::client::Result<()>;
            #[doc = "A system-specific device path that indicates which device is behind"]
            #[doc = "this wp_tablet. This information may be used to gather additional"]
            #[doc = "information about the device, e.g. through libwacom."]
            #[doc = ""]
            #[doc = "A device may have more than one device path. If so, multiple"]
            #[doc = "wp_tablet.path events are sent. A device may be emulated and not"]
            #[doc = "have a device path, and in that case this event will not be sent."]
            #[doc = ""]
            #[doc = "The format of the path is unspecified, it may be a device node, a"]
            #[doc = "sysfs path, or some other identifier. It is up to the client to"]
            #[doc = "identify the string provided."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            async fn path(&self, path: String) -> crate::client::Result<()>;
            #[doc = "This event is sent immediately to signal the end of the initial"]
            #[doc = "burst of descriptive events. A client may consider the static"]
            #[doc = "description of the tablet to be complete and finalize initialization"]
            #[doc = "of the tablet."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "Sent when the tablet has been removed from the system. When a tablet"]
            #[doc = "is removed, some tools may be removed."]
            #[doc = ""]
            #[doc = "When this event is received, the client must wp_tablet.destroy"]
            #[doc = "the object."]
            async fn removed(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "A circular interaction area, such as the touch ring on the Wacom Intuos"]
    #[doc = "Pro series tablets."]
    #[doc = ""]
    #[doc = "Events on a ring are logically grouped by the wl_tablet_pad_ring.frame"]
    #[doc = "event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_ring_v2 {
        use futures_util::SinkExt;
        #[doc = "Describes the source types for ring events. This indicates to the"]
        #[doc = "client how a ring event was physically generated; a client may"]
        #[doc = "adjust the user interface accordingly. For example, events"]
        #[doc = "from a \"finger\" source may trigger kinetic scrolling."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Source {
            #[doc = "finger"]
            Finger = 1u32,
        }
        impl TryFrom<u32> for Source {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_ring_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadRingV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Request that the compositor use the provided feedback string"]
            #[doc = "associated with this ring. This request should be issued immediately"]
            #[doc = "after a wp_tablet_pad_group.mode_switch event from the corresponding"]
            #[doc = "group is received, or whenever the ring is mapped to a different"]
            #[doc = "action. See wp_tablet_pad_group.mode_switch for more details."]
            #[doc = ""]
            #[doc = "Clients are encouraged to provide context-aware descriptions for"]
            #[doc = "the actions associated with the ring; compositors may use this"]
            #[doc = "information to offer visual feedback about the button layout"]
            #[doc = "(eg. on-screen displays)."]
            #[doc = ""]
            #[doc = "The provided string 'description' is a UTF-8 encoded string to be"]
            #[doc = "associated with this ring, and is considered user-visible; general"]
            #[doc = "internationalization rules apply."]
            #[doc = ""]
            #[doc = "The serial argument will be that of the last"]
            #[doc = "wp_tablet_pad_group.mode_switch event received for the group of this"]
            #[doc = "ring. Requests providing other serials than the most recent one will be"]
            #[doc = "ignored."]
            async fn set_feedback(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                description: String,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_ring_v2#{}.set_feedback()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(description))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This destroys the client's resource for this ring object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_ring_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Source information for ring events."]
            #[doc = ""]
            #[doc = "This event does not occur on its own. It is sent before a"]
            #[doc = "wp_tablet_pad_ring.frame event and carries the source information"]
            #[doc = "for all events within that frame."]
            #[doc = ""]
            #[doc = "The source specifies how this event was generated. If the source is"]
            #[doc = "wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop event"]
            #[doc = "will be sent when the user lifts the finger off the device."]
            #[doc = ""]
            #[doc = "This event is optional. If the source is unknown for an interaction,"]
            #[doc = "no event is sent."]
            async fn source(&self, source: Source) -> crate::client::Result<()>;
            #[doc = "Sent whenever the angle on a ring changes."]
            #[doc = ""]
            #[doc = "The angle is provided in degrees clockwise from the logical"]
            #[doc = "north of the ring in the pad's current rotation."]
            async fn angle(&self, degrees: crate::wire::Fixed) -> crate::client::Result<()>;
            #[doc = "Stop notification for ring events."]
            #[doc = ""]
            #[doc = "For some wp_tablet_pad_ring.source types, a wp_tablet_pad_ring.stop"]
            #[doc = "event is sent to notify a client that the interaction with the ring"]
            #[doc = "has terminated. This enables the client to implement kinetic scrolling."]
            #[doc = "See the wp_tablet_pad_ring.source documentation for information on"]
            #[doc = "when this event may be generated."]
            #[doc = ""]
            #[doc = "Any wp_tablet_pad_ring.angle events with the same source after this"]
            #[doc = "event should be considered as the start of a new interaction."]
            async fn stop(&self) -> crate::client::Result<()>;
            #[doc = "Indicates the end of a set of ring events that logically belong"]
            #[doc = "together. A client is expected to accumulate the data in all events"]
            #[doc = "within the frame before proceeding."]
            #[doc = ""]
            #[doc = "All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame event belong"]
            #[doc = "logically together. For example, on termination of a finger interaction"]
            #[doc = "on a ring the compositor will send a wp_tablet_pad_ring.source event,"]
            #[doc = "a wp_tablet_pad_ring.stop event and a wp_tablet_pad_ring.frame event."]
            #[doc = ""]
            #[doc = "A wp_tablet_pad_ring.frame event is sent for every logical event"]
            #[doc = "group, even if the group only contains a single wp_tablet_pad_ring"]
            #[doc = "event. Specifically, a client may get a sequence: angle, frame,"]
            #[doc = "angle, frame, etc."]
            async fn frame(&self, time: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "A linear interaction area, such as the strips found in Wacom Cintiq"]
    #[doc = "models."]
    #[doc = ""]
    #[doc = "Events on a strip are logically grouped by the wl_tablet_pad_strip.frame"]
    #[doc = "event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_strip_v2 {
        use futures_util::SinkExt;
        #[doc = "Describes the source types for strip events. This indicates to the"]
        #[doc = "client how a strip event was physically generated; a client may"]
        #[doc = "adjust the user interface accordingly. For example, events"]
        #[doc = "from a \"finger\" source may trigger kinetic scrolling."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Source {
            #[doc = "finger"]
            Finger = 1u32,
        }
        impl TryFrom<u32> for Source {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_strip_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadStripV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Requests the compositor to use the provided feedback string"]
            #[doc = "associated with this strip. This request should be issued immediately"]
            #[doc = "after a wp_tablet_pad_group.mode_switch event from the corresponding"]
            #[doc = "group is received, or whenever the strip is mapped to a different"]
            #[doc = "action. See wp_tablet_pad_group.mode_switch for more details."]
            #[doc = ""]
            #[doc = "Clients are encouraged to provide context-aware descriptions for"]
            #[doc = "the actions associated with the strip, and compositors may use this"]
            #[doc = "information to offer visual feedback about the button layout"]
            #[doc = "(eg. on-screen displays)."]
            #[doc = ""]
            #[doc = "The provided string 'description' is a UTF-8 encoded string to be"]
            #[doc = "associated with this ring, and is considered user-visible; general"]
            #[doc = "internationalization rules apply."]
            #[doc = ""]
            #[doc = "The serial argument will be that of the last"]
            #[doc = "wp_tablet_pad_group.mode_switch event received for the group of this"]
            #[doc = "strip. Requests providing other serials than the most recent one will be"]
            #[doc = "ignored."]
            async fn set_feedback(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                description: String,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_strip_v2#{}.set_feedback()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(description))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This destroys the client's resource for this strip object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_strip_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Source information for strip events."]
            #[doc = ""]
            #[doc = "This event does not occur on its own. It is sent before a"]
            #[doc = "wp_tablet_pad_strip.frame event and carries the source information"]
            #[doc = "for all events within that frame."]
            #[doc = ""]
            #[doc = "The source specifies how this event was generated. If the source is"]
            #[doc = "wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop event"]
            #[doc = "will be sent when the user lifts their finger off the device."]
            #[doc = ""]
            #[doc = "This event is optional. If the source is unknown for an interaction,"]
            #[doc = "no event is sent."]
            async fn source(&self, source: Source) -> crate::client::Result<()>;
            #[doc = "Sent whenever the position on a strip changes."]
            #[doc = ""]
            #[doc = "The position is normalized to a range of [0, 65535], the 0-value"]
            #[doc = "represents the top-most and/or left-most position of the strip in"]
            #[doc = "the pad's current rotation."]
            async fn position(&self, position: u32) -> crate::client::Result<()>;
            #[doc = "Stop notification for strip events."]
            #[doc = ""]
            #[doc = "For some wp_tablet_pad_strip.source types, a wp_tablet_pad_strip.stop"]
            #[doc = "event is sent to notify a client that the interaction with the strip"]
            #[doc = "has terminated. This enables the client to implement kinetic"]
            #[doc = "scrolling. See the wp_tablet_pad_strip.source documentation for"]
            #[doc = "information on when this event may be generated."]
            #[doc = ""]
            #[doc = "Any wp_tablet_pad_strip.position events with the same source after this"]
            #[doc = "event should be considered as the start of a new interaction."]
            async fn stop(&self) -> crate::client::Result<()>;
            #[doc = "Indicates the end of a set of events that represent one logical"]
            #[doc = "hardware strip event. A client is expected to accumulate the data"]
            #[doc = "in all events within the frame before proceeding."]
            #[doc = ""]
            #[doc = "All wp_tablet_pad_strip events before a wp_tablet_pad_strip.frame event belong"]
            #[doc = "logically together. For example, on termination of a finger interaction"]
            #[doc = "on a strip the compositor will send a wp_tablet_pad_strip.source event,"]
            #[doc = "a wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "A wp_tablet_pad_strip.frame event is sent for every logical event"]
            #[doc = "group, even if the group only contains a single wp_tablet_pad_strip"]
            #[doc = "event. Specifically, a client may get a sequence: position, frame,"]
            #[doc = "position, frame, etc."]
            async fn frame(&self, time: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "A pad group describes a distinct (sub)set of buttons, rings and strips"]
    #[doc = "present in the tablet. The criteria of this grouping is usually positional,"]
    #[doc = "eg. if a tablet has buttons on the left and right side, 2 groups will be"]
    #[doc = "presented. The physical arrangement of groups is undisclosed and may"]
    #[doc = "change on the fly."]
    #[doc = ""]
    #[doc = "Pad groups will announce their features during pad initialization. Between"]
    #[doc = "the corresponding wp_tablet_pad.group event and wp_tablet_pad_group.done, the"]
    #[doc = "pad group will announce the buttons, rings and strips contained in it,"]
    #[doc = "plus the number of supported modes."]
    #[doc = ""]
    #[doc = "Modes are a mechanism to allow multiple groups of actions for every element"]
    #[doc = "in the pad group. The number of groups and available modes in each is"]
    #[doc = "persistent across device plugs. The current mode is user-switchable, it"]
    #[doc = "will be announced through the wp_tablet_pad_group.mode_switch event both"]
    #[doc = "whenever it is switched, and after wp_tablet_pad.enter."]
    #[doc = ""]
    #[doc = "The current mode logically applies to all elements in the pad group,"]
    #[doc = "although it is at clients' discretion whether to actually perform different"]
    #[doc = "actions, and/or issue the respective .set_feedback requests to notify the"]
    #[doc = "compositor. See the wp_tablet_pad_group.mode_switch event for more details."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_group_v2 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_pad_group_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadGroupV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_tablet_pad_group object. Objects created from this object"]
            #[doc = "are unaffected and should be destroyed separately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_group_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sent on wp_tablet_pad_group initialization to announce the available"]
            #[doc = "buttons in the group. Button indices start at 0, a button may only be"]
            #[doc = "in one group at a time."]
            #[doc = ""]
            #[doc = "This event is first sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event."]
            #[doc = ""]
            #[doc = "Some buttons are reserved by the compositor. These buttons may not be"]
            #[doc = "assigned to any wp_tablet_pad_group. Compositors may broadcast this"]
            #[doc = "event in the case of changes to the mapping of these reserved buttons."]
            #[doc = "If the compositor happens to reserve all buttons in a group, this event"]
            #[doc = "will be sent with an empty array."]
            async fn buttons(&self, buttons: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "Sent on wp_tablet_pad_group initialization to announce available rings."]
            #[doc = "One event is sent for each ring available on this pad group."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event."]
            async fn ring(&self, ring: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "Sent on wp_tablet_pad initialization to announce available strips."]
            #[doc = "One event is sent for each strip available on this pad group."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event."]
            async fn strip(&self, strip: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "Sent on wp_tablet_pad_group initialization to announce that the pad"]
            #[doc = "group may switch between modes. A client may use a mode to store a"]
            #[doc = "specific configuration for buttons, rings and strips and use the"]
            #[doc = "wl_tablet_pad_group.mode_switch event to toggle between these"]
            #[doc = "configurations. Mode indices start at 0."]
            #[doc = ""]
            #[doc = "Switching modes is compositor-dependent. See the"]
            #[doc = "wp_tablet_pad_group.mode_switch event for more details."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event. This event is only sent when more than"]
            #[doc = "more than one mode is available."]
            async fn modes(&self, modes: u32) -> crate::client::Result<()>;
            #[doc = "This event is sent immediately to signal the end of the initial"]
            #[doc = "burst of descriptive events. A client may consider the static"]
            #[doc = "description of the tablet to be complete and finalize initialization"]
            #[doc = "of the tablet group."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "Notification that the mode was switched."]
            #[doc = ""]
            #[doc = "A mode applies to all buttons, rings and strips in a group"]
            #[doc = "simultaneously, but a client is not required to assign different actions"]
            #[doc = "for each mode. For example, a client may have mode-specific button"]
            #[doc = "mappings but map the ring to vertical scrolling in all modes. Mode"]
            #[doc = "indices start at 0."]
            #[doc = ""]
            #[doc = "Switching modes is compositor-dependent. The compositor may provide"]
            #[doc = "visual cues to the client about the mode, e.g. by toggling LEDs on"]
            #[doc = "the tablet device. Mode-switching may be software-controlled or"]
            #[doc = "controlled by one or more physical buttons. For example, on a Wacom"]
            #[doc = "Intuos Pro, the button inside the ring may be assigned to switch"]
            #[doc = "between modes."]
            #[doc = ""]
            #[doc = "The compositor will also send this event after wp_tablet_pad.enter on"]
            #[doc = "each group in order to notify of the current mode. Groups that only"]
            #[doc = "feature one mode will use mode=0 when emitting this event."]
            #[doc = ""]
            #[doc = "If a button action in the new mode differs from the action in the"]
            #[doc = "previous mode, the client should immediately issue a"]
            #[doc = "wp_tablet_pad.set_feedback request for each changed button."]
            #[doc = ""]
            #[doc = "If a ring or strip action in the new mode differs from the action"]
            #[doc = "in the previous mode, the client should immediately issue a"]
            #[doc = "wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback request"]
            #[doc = "for each changed ring or strip."]
            async fn mode_switch(
                &self,
                time: u32,
                serial: u32,
                mode: u32,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "A pad device is a set of buttons, rings and strips"]
    #[doc = "usually physically present on the tablet device itself. Some"]
    #[doc = "exceptions exist where the pad device is physically detached, e.g. the"]
    #[doc = "Wacom ExpressKey Remote."]
    #[doc = ""]
    #[doc = "Pad devices have no axes that control the cursor and are generally"]
    #[doc = "auxiliary devices to the tool devices used on the tablet surface."]
    #[doc = ""]
    #[doc = "A pad device has a number of static characteristics, e.g. the number"]
    #[doc = "of rings. These capabilities are sent in an event sequence after the"]
    #[doc = "wp_tablet_seat.pad_added event before any actual events from this pad."]
    #[doc = "This initial event sequence is terminated by a wp_tablet_pad.done"]
    #[doc = "event."]
    #[doc = ""]
    #[doc = "All pad features (buttons, rings and strips) are logically divided into"]
    #[doc = "groups and all pads have at least one group. The available groups are"]
    #[doc = "notified through the wp_tablet_pad.group event; the compositor will"]
    #[doc = "emit one event per group before emitting wp_tablet_pad.done."]
    #[doc = ""]
    #[doc = "Groups may have multiple modes. Modes allow clients to map multiple"]
    #[doc = "actions to a single pad feature. Only one mode can be active per group,"]
    #[doc = "although different groups may have different active modes."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_v2 {
        use futures_util::SinkExt;
        #[doc = "Describes the physical state of a button that caused the button"]
        #[doc = "event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ButtonState {
            #[doc = "the button is not pressed"]
            Released = 0u32,
            #[doc = "the button is pressed"]
            Pressed = 1u32,
        }
        impl TryFrom<u32> for ButtonState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Released),
                    1u32 => Ok(Self::Pressed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ButtonState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Requests the compositor to use the provided feedback string"]
            #[doc = "associated with this button. This request should be issued immediately"]
            #[doc = "after a wp_tablet_pad_group.mode_switch event from the corresponding"]
            #[doc = "group is received, or whenever a button is mapped to a different"]
            #[doc = "action. See wp_tablet_pad_group.mode_switch for more details."]
            #[doc = ""]
            #[doc = "Clients are encouraged to provide context-aware descriptions for"]
            #[doc = "the actions associated with each button, and compositors may use"]
            #[doc = "this information to offer visual feedback on the button layout"]
            #[doc = "(e.g. on-screen displays)."]
            #[doc = ""]
            #[doc = "Button indices start at 0. Setting the feedback string on a button"]
            #[doc = "that is reserved by the compositor (i.e. not belonging to any"]
            #[doc = "wp_tablet_pad_group) does not generate an error but the compositor"]
            #[doc = "is free to ignore the request."]
            #[doc = ""]
            #[doc = "The provided string 'description' is a UTF-8 encoded string to be"]
            #[doc = "associated with this ring, and is considered user-visible; general"]
            #[doc = "internationalization rules apply."]
            #[doc = ""]
            #[doc = "The serial argument will be that of the last"]
            #[doc = "wp_tablet_pad_group.mode_switch event received for the group of this"]
            #[doc = "button. Requests providing other serials than the most recent one will"]
            #[doc = "be ignored."]
            async fn set_feedback(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                button: u32,
                description: String,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_v2#{}.set_feedback()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(button)
                    .put_string(Some(description))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the wp_tablet_pad object. Objects created from this object"]
            #[doc = "are unaffected and should be destroyed separately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_pad_v2#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sent on wp_tablet_pad initialization to announce available groups."]
            #[doc = "One event is sent for each pad group available."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad.done event. At least one group will be announced."]
            async fn group(&self, pad_group: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "A system-specific device path that indicates which device is behind"]
            #[doc = "this wp_tablet_pad. This information may be used to gather additional"]
            #[doc = "information about the device, e.g. through libwacom."]
            #[doc = ""]
            #[doc = "The format of the path is unspecified, it may be a device node, a"]
            #[doc = "sysfs path, or some other identifier. It is up to the client to"]
            #[doc = "identify the string provided."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad.done event."]
            async fn path(&self, path: String) -> crate::client::Result<()>;
            #[doc = "Sent on wp_tablet_pad initialization to announce the available"]
            #[doc = "buttons."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad.done event. This event is only sent when at least one"]
            #[doc = "button is available."]
            async fn buttons(&self, buttons: u32) -> crate::client::Result<()>;
            #[doc = "This event signals the end of the initial burst of descriptive"]
            #[doc = "events. A client may consider the static description of the pad to"]
            #[doc = "be complete and finalize initialization of the pad."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "Sent whenever the physical state of a button changes."]
            async fn button(
                &self,
                time: u32,
                button: u32,
                state: ButtonState,
            ) -> crate::client::Result<()>;
            #[doc = "Notification that this pad is focused on the specified surface."]
            async fn enter(
                &self,
                serial: u32,
                tablet: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "Notification that this pad is no longer focused on the specified"]
            #[doc = "surface."]
            async fn leave(
                &self,
                serial: u32,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "Sent when the pad has been removed from the system. When a tablet"]
            #[doc = "is removed its pad(s) will be removed too."]
            #[doc = ""]
            #[doc = "When this event is received, the client must destroy all rings, strips"]
            #[doc = "and groups that were offered by this pad, and issue wp_tablet_pad.destroy"]
            #[doc = "the pad itself."]
            async fn removed(&self) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod text_input_unstable_v1 {
    #[doc = "An object used for text input. Adds support for text input and input"]
    #[doc = "methods to applications. A text_input object is created from a"]
    #[doc = "wl_text_input_manager and corresponds typically to a text entry in an"]
    #[doc = "application."]
    #[doc = ""]
    #[doc = "Requests are used to activate/deactivate the text_input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about entered text is sent to the text_input object via"]
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
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_v1 {
        use futures_util::SinkExt;
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behaviour"] const None = 0u32 ; # [doc = "auto completion, correction and capitalization"] const Default = 7u32 ; # [doc = "hidden and sensitive text"] const Password = 192u32 ; # [doc = "suggest word completions"] const AutoCompletion = 1u32 ; # [doc = "suggest word corrections"] const AutoCorrection = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
        impl TryFrom<u32> for ContentHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ContentHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
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
        impl std::fmt::Display for ContentPurpose {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for PreeditStyle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for TextDirection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_text_input_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV1 {
            const INTERFACE: &'static str = "zwp_text_input_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Requests the text_input object to be activated (typically when the"]
            #[doc = "text entry gets focus)."]
            #[doc = ""]
            #[doc = "The seat argument is a wl_seat which maintains the focus for this"]
            #[doc = "activation. The surface argument is a wl_surface assigned to the"]
            #[doc = "text_input object and tracked for focus lost. The enter event"]
            #[doc = "is emitted on successful activation."]
            async fn activate(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.activate()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests the text_input object to be deactivated (typically when the"]
            #[doc = "text entry lost focus). The seat argument is a wl_seat which was used"]
            #[doc = "for activation."]
            async fn deactivate(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.deactivate()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests input panels (virtual keyboard) to show."]
            async fn show_input_panel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.show_input_panel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests input panels (virtual keyboard) to hide."]
            async fn hide_input_panel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.hide_input_panel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Should be called by an editor widget when the input state should be"]
            #[doc = "reset, for example after the text was changed outside of the normal"]
            #[doc = "input method flow."]
            async fn reset(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.reset()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets the plain surrounding text around the input position. Text is"]
            #[doc = "UTF-8 encoded. Cursor is the byte offset within the"]
            #[doc = "surrounding text. Anchor is the byte offset of the"]
            #[doc = "selection anchor within the surrounding text. If there is no selected"]
            #[doc = "text anchor, then it is the same as cursor."]
            async fn set_surrounding_text(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                text: String,
                cursor: u32,
                anchor: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.set_surrounding_text()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(text))
                    .put_uint(cursor)
                    .put_uint(anchor)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some"]
            #[doc = "of the behavior."]
            #[doc = ""]
            #[doc = "When no content type is explicitly set, a normal content purpose with"]
            #[doc = "default hints (auto completion, auto correction, auto capitalization)"]
            #[doc = "should be assumed."]
            async fn set_content_type(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                hint: ContentHint,
                purpose: ContentPurpose,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.set_content_type()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(hint.bits())
                    .put_uint(purpose as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn set_cursor_rectangle(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.set_cursor_rectangle()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets a specific language. This allows for example a virtual keyboard to"]
            #[doc = "show a language specific layout. The \"language\" argument is an RFC-3066"]
            #[doc = "format language tag."]
            #[doc = ""]
            #[doc = "It could be used for example in a word processor to indicate the"]
            #[doc = "language of the currently edited document or in an instant message"]
            #[doc = "application which tracks languages of contacts."]
            async fn set_preferred_language(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                language: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_text_input_v1#{}.set_preferred_language()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(language))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn commit_state(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.commit_state()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn invoke_action(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                button: u32,
                index: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v1#{}.invoke_action()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(button)
                    .put_uint(index)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Notify the text_input object when it received focus. Typically in"]
            #[doc = "response to an activate request."]
            async fn enter(&self, surface: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "Notify the text_input object when it lost focus. Either in response"]
            #[doc = "to a deactivate request or when the assigned surface lost focus or was"]
            #[doc = "destroyed."]
            async fn leave(&self) -> crate::client::Result<()>;
            #[doc = "Transfer an array of 0-terminated modifier names. The position in"]
            #[doc = "the array is the index of the modifier as used in the modifiers"]
            #[doc = "bitmask in the keysym event."]
            async fn modifiers_map(&self, map: Vec<u8>) -> crate::client::Result<()>;
            #[doc = "Notify when the visibility state of the input panel changed."]
            async fn input_panel_state(&self, state: u32) -> crate::client::Result<()>;
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
                serial: u32,
                text: String,
                commit: String,
            ) -> crate::client::Result<()>;
            #[doc = "Sets styling information on composing text. The style is applied for"]
            #[doc = "length bytes from index relative to the beginning of the composing"]
            #[doc = "text (as byte offset). Multiple styles can"]
            #[doc = "be applied to a composing text by sending multiple preedit_styling"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            async fn preedit_styling(
                &self,
                index: u32,
                length: u32,
                style: PreeditStyle,
            ) -> crate::client::Result<()>;
            #[doc = "Sets the cursor position inside the composing text (as byte"]
            #[doc = "offset) relative to the start of the composing text. When index is a"]
            #[doc = "negative number no cursor is shown."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            async fn preedit_cursor(&self, index: i32) -> crate::client::Result<()>;
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit). It could also be an empty text"]
            #[doc = "when some text should be removed (see delete_surrounding_text) or when"]
            #[doc = "the input cursor should be moved (see cursor_position)."]
            #[doc = ""]
            #[doc = "Any previously set composing text should be removed."]
            async fn commit_string(&self, serial: u32, text: String) -> crate::client::Result<()>;
            #[doc = "Notify when the cursor or anchor position should be modified."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            async fn cursor_position(&self, index: i32, anchor: i32) -> crate::client::Result<()>;
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
                index: i32,
                length: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Notify when a key event was sent. Key events should not be used"]
            #[doc = "for normal text input operations, which should be done with"]
            #[doc = "commit_string, delete_surrounding_text, etc. The key event follows"]
            #[doc = "the wl_keyboard key event convention. Sym is an XKB keysym, state a"]
            #[doc = "wl_keyboard key_state. Modifiers are a mask for effective modifiers"]
            #[doc = "(where the modifier indices are set by the modifiers_map event)"]
            async fn keysym(
                &self,
                serial: u32,
                time: u32,
                sym: u32,
                state: u32,
                modifiers: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Sets the language of the input text. The \"language\" argument is an"]
            #[doc = "RFC-3066 format language tag."]
            async fn language(&self, serial: u32, language: String) -> crate::client::Result<()>;
            #[doc = "Sets the text direction of input text."]
            #[doc = ""]
            #[doc = "It is mainly needed for showing an input cursor on the correct side of"]
            #[doc = "the editor when there is no input done yet and making sure neutral"]
            #[doc = "direction text is laid out properly."]
            async fn text_direction(
                &self,
                serial: u32,
                direction: TextDirection,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "A factory for text_input objects. This object is a global singleton."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_text_input_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV1 {
            const INTERFACE: &'static str = "zwp_text_input_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Creates a new text_input object."]
            async fn create_text_input(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_text_input_manager_v1#{}.create_text_input()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_shell_unstable_v6 {
    #[doc = "xdg_shell allows clients to turn a wl_surface into a \"real window\""]
    #[doc = "which can be dragged, resized, stacked, and moved around by the"]
    #[doc = "user. Everything about this interface is suited towards traditional"]
    #[doc = "desktop environments."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_shell_v6 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "xdg_shell was destroyed before children"]
            DefunctSurfaces = 1u32,
            #[doc = "the client tried to map or destroy a non-topmost popup"]
            NotTheTopmostPopup = 2u32,
            #[doc = "the client specified an invalid popup parent surface"]
            InvalidPopupParent = 3u32,
            #[doc = "the client provided an invalid surface state"]
            InvalidSurfaceState = 4u32,
            #[doc = "the client provided an invalid positioner"]
            InvalidPositioner = 5u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::DefunctSurfaces),
                    2u32 => Ok(Self::NotTheTopmostPopup),
                    3u32 => Ok(Self::InvalidPopupParent),
                    4u32 => Ok(Self::InvalidSurfaceState),
                    5u32 => Ok(Self::InvalidPositioner),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_shell_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgShellV6 {
            const INTERFACE: &'static str = "zxdg_shell_v6";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this xdg_shell object."]
            #[doc = ""]
            #[doc = "Destroying a bound xdg_shell object while there are surfaces"]
            #[doc = "still alive created by this xdg_shell object instance is illegal"]
            #[doc = "and will result in a protocol error."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_shell_v6#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a positioner object. A positioner object is used to position"]
            #[doc = "surfaces relative to some parent surface. See the interface description"]
            #[doc = "and xdg_surface.get_popup for details."]
            async fn create_positioner(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_shell_v6#{}.create_positioner()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates an xdg_surface for the given surface. While xdg_surface"]
            #[doc = "itself is not a role, the corresponding surface may only be assigned"]
            #[doc = "a role extending xdg_surface, such as xdg_toplevel or xdg_popup."]
            #[doc = ""]
            #[doc = "This creates an xdg_surface for the given surface. An xdg_surface is"]
            #[doc = "used as basis to define a role to a given surface, such as xdg_toplevel"]
            #[doc = "or xdg_popup. It also manages functionality shared between xdg_surface"]
            #[doc = "based surface roles."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_surface for more details about what an"]
            #[doc = "xdg_surface is and how it is used."]
            async fn get_xdg_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_shell_v6#{}.get_xdg_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "A client must respond to a ping event with a pong request or"]
            #[doc = "the client may be deemed unresponsive. See xdg_shell.ping."]
            async fn pong(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_shell_v6#{}.pong()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The ping event asks the client if it's still alive. Pass the"]
            #[doc = "serial specified in the event back to the compositor by sending"]
            #[doc = "a \"pong\" request back with the specified serial. See xdg_shell.ping."]
            #[doc = ""]
            #[doc = "Compositors can use this to determine if the client is still"]
            #[doc = "alive. It's unspecified what will happen if the client doesn't"]
            #[doc = "respond to the ping request, or in what timeframe. Clients should"]
            #[doc = "try to respond in a reasonable amount of time."]
            #[doc = ""]
            #[doc = "A compositor is free to ping in any way it wants, but a client must"]
            #[doc = "always respond to any xdg_shell object it created."]
            async fn ping(&self, serial: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "The xdg_positioner provides a collection of rules for the placement of a"]
    #[doc = "child surface relative to a parent surface. Rules can be defined to ensure"]
    #[doc = "the child surface remains within the visible area's borders, and to"]
    #[doc = "specify how the child surface changes its position, such as sliding along"]
    #[doc = "an axis, or flipping around a rectangle. These positioner-created rules are"]
    #[doc = "constrained by the requirement that a child surface must intersect with or"]
    #[doc = "be at least partially adjacent to its parent surface."]
    #[doc = ""]
    #[doc = "See the various requests for details about possible rules."]
    #[doc = ""]
    #[doc = "At the time of the request, the compositor makes a copy of the rules"]
    #[doc = "specified by the xdg_positioner. Thus, after the request is complete the"]
    #[doc = "xdg_positioner object can be destroyed or reused; further changes to the"]
    #[doc = "object will have no effect on previous usages."]
    #[doc = ""]
    #[doc = "For an xdg_positioner object to be considered complete, it must have a"]
    #[doc = "non-zero size set by set_size, and a non-zero anchor rectangle set by"]
    #[doc = "set_anchor_rect. Passing an incomplete xdg_positioner object when"]
    #[doc = "positioning a surface raises an error."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_positioner_v6 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid input provided"]
            InvalidInput = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidInput),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Anchor : u32 { # [doc = "the center of the anchor rectangle"] const None = 0u32 ; # [doc = "the top edge of the anchor rectangle"] const Top = 1u32 ; # [doc = "the bottom edge of the anchor rectangle"] const Bottom = 2u32 ; # [doc = "the left edge of the anchor rectangle"] const Left = 4u32 ; # [doc = "the right edge of the anchor rectangle"] const Right = 8u32 ; } }
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
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Gravity : u32 { # [doc = "center over the anchor edge"] const None = 0u32 ; # [doc = "position above the anchor edge"] const Top = 1u32 ; # [doc = "position below the anchor edge"] const Bottom = 2u32 ; # [doc = "position to the left of the anchor edge"] const Left = 4u32 ; # [doc = "position to the right of the anchor edge"] const Right = 8u32 ; } }
        impl TryFrom<u32> for Gravity {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Gravity {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "The constraint adjustment value define ways the compositor will adjust"] # [doc = "the position of the surface, if the unadjusted position would result"] # [doc = "in the surface being partly constrained."] # [doc = ""] # [doc = "Whether a surface is considered 'constrained' is left to the compositor"] # [doc = "to determine. For example, the surface may be partly outside the"] # [doc = "compositor's defined 'work area', thus necessitating the child surface's"] # [doc = "position be adjusted until it is entirely inside the work area."] # [doc = ""] # [doc = "The adjustments can be combined, according to a defined precedence: 1)"] # [doc = "Flip, 2) Slide, 3) Resize."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ConstraintAdjustment : u32 { const None = 0u32 ; const SlideX = 1u32 ; const SlideY = 2u32 ; const FlipX = 4u32 ; const FlipY = 8u32 ; const ResizeX = 16u32 ; const ResizeY = 32u32 ; } }
        impl TryFrom<u32> for ConstraintAdjustment {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ConstraintAdjustment {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_positioner_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgPositionerV6 {
            const INTERFACE: &'static str = "zxdg_positioner_v6";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the xdg_positioner will no longer be used."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_positioner_v6#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the size of the surface that is to be positioned with the positioner"]
            #[doc = "object. The size is in surface-local coordinates and corresponds to the"]
            #[doc = "window geometry. See xdg_surface.set_window_geometry."]
            #[doc = ""]
            #[doc = "If a zero or negative size is set the invalid_input error is raised."]
            async fn set_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_positioner_v6#{}.set_size()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Specify the anchor rectangle within the parent surface that the child"]
            #[doc = "surface will be placed relative to. The rectangle is relative to the"]
            #[doc = "window geometry as defined by xdg_surface.set_window_geometry of the"]
            #[doc = "parent surface. The rectangle must be at least 1x1 large."]
            #[doc = ""]
            #[doc = "When the xdg_positioner object is used to position a child surface, the"]
            #[doc = "anchor rectangle may not extend outside the window geometry of the"]
            #[doc = "positioned child's parent surface."]
            #[doc = ""]
            #[doc = "If a zero or negative size is set the invalid_input error is raised."]
            async fn set_anchor_rect(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_positioner_v6#{}.set_anchor_rect()", object_id);
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
            #[doc = "Defines a set of edges for the anchor rectangle. These are used to"]
            #[doc = "derive an anchor point that the child surface will be positioned"]
            #[doc = "relative to. If two orthogonal edges are specified (e.g. 'top' and"]
            #[doc = "'left'), then the anchor point will be the intersection of the edges"]
            #[doc = "(e.g. the top left position of the rectangle); otherwise, the derived"]
            #[doc = "anchor point will be centered on the specified edge, or in the center of"]
            #[doc = "the anchor rectangle if no edge is specified."]
            #[doc = ""]
            #[doc = "If two parallel anchor edges are specified (e.g. 'left' and 'right'),"]
            #[doc = "the invalid_input error is raised."]
            async fn set_anchor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                anchor: Anchor,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_positioner_v6#{}.set_anchor()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(anchor.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Defines in what direction a surface should be positioned, relative to"]
            #[doc = "the anchor point of the parent surface. If two orthogonal gravities are"]
            #[doc = "specified (e.g. 'bottom' and 'right'), then the child surface will be"]
            #[doc = "placed in the specified direction; otherwise, the child surface will be"]
            #[doc = "centered over the anchor point on any axis that had no gravity"]
            #[doc = "specified."]
            #[doc = ""]
            #[doc = "If two parallel gravities are specified (e.g. 'left' and 'right'), the"]
            #[doc = "invalid_input error is raised."]
            async fn set_gravity(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                gravity: Gravity,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_positioner_v6#{}.set_gravity()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(gravity.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Specify how the window should be positioned if the originally intended"]
            #[doc = "position caused the surface to be constrained, meaning at least"]
            #[doc = "partially outside positioning boundaries set by the compositor. The"]
            #[doc = "adjustment is set by constructing a bitmask describing the adjustment to"]
            #[doc = "be made when the surface is constrained on that axis."]
            #[doc = ""]
            #[doc = "If no bit for one axis is set, the compositor will assume that the child"]
            #[doc = "surface should not change its position on that axis when constrained."]
            #[doc = ""]
            #[doc = "If more than one bit for one axis is set, the order of how adjustments"]
            #[doc = "are applied is specified in the corresponding adjustment descriptions."]
            #[doc = ""]
            #[doc = "The default adjustment is none."]
            async fn set_constraint_adjustment(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                constraint_adjustment: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zxdg_positioner_v6#{}.set_constraint_adjustment()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(constraint_adjustment)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Specify the surface position offset relative to the position of the"]
            #[doc = "anchor on the anchor rectangle and the anchor on the surface. For"]
            #[doc = "example if the anchor of the anchor rectangle is at (x, y), the surface"]
            #[doc = "has the gravity bottom|right, and the offset is (ox, oy), the calculated"]
            #[doc = "surface position will be (x + ox, y + oy). The offset position of the"]
            #[doc = "surface is the one used for constraint testing. See"]
            #[doc = "set_constraint_adjustment."]
            #[doc = ""]
            #[doc = "An example use case is placing a popup menu on top of a user interface"]
            #[doc = "element, while aligning the user interface element of the parent surface"]
            #[doc = "with some user interface element placed somewhere in the popup surface."]
            async fn set_offset(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_positioner_v6#{}.set_offset()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide a desktop-style user interface."]
    #[doc = ""]
    #[doc = "It provides a base set of functionality required to construct user"]
    #[doc = "interface elements requiring management by the compositor, such as"]
    #[doc = "toplevel windows, menus, etc. The types of functionality are split into"]
    #[doc = "xdg_surface roles."]
    #[doc = ""]
    #[doc = "Creating an xdg_surface does not set the role for a wl_surface. In order"]
    #[doc = "to map an xdg_surface, the client must create a role-specific object"]
    #[doc = "using, e.g., get_toplevel, get_popup. The wl_surface for any given"]
    #[doc = "xdg_surface can have at most one role, and may not be assigned any role"]
    #[doc = "not based on xdg_surface."]
    #[doc = ""]
    #[doc = "A role must be assigned before any other requests are made to the"]
    #[doc = "xdg_surface object."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xdg_surface state to take effect."]
    #[doc = ""]
    #[doc = "Creating an xdg_surface from a wl_surface which has a buffer attached or"]
    #[doc = "committed is a client error, and any attempts by a client to attach or"]
    #[doc = "manipulate a buffer prior to the first xdg_surface.configure call must"]
    #[doc = "also be treated as errors."]
    #[doc = ""]
    #[doc = "For a surface to be mapped by the compositor, the following conditions"]
    #[doc = "must be met: (1) the client has assigned an xdg_surface based role to the"]
    #[doc = "surface, (2) the client has set and committed the xdg_surface state and"]
    #[doc = "the role dependent state to the surface and (3) the client has committed a"]
    #[doc = "buffer to the surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_surface_v6 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            NotConstructed = 1u32,
            AlreadyConstructed = 2u32,
            UnconfiguredBuffer = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NotConstructed),
                    2u32 => Ok(Self::AlreadyConstructed),
                    3u32 => Ok(Self::UnconfiguredBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_surface_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgSurfaceV6 {
            const INTERFACE: &'static str = "zxdg_surface_v6";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the xdg_surface object. An xdg_surface must only be destroyed"]
            #[doc = "after its role object has been destroyed. If the role object still"]
            #[doc = "exists when this request is issued, the zxdg_shell_v6.defunct_surfaces"]
            #[doc = "is raised."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_surface_v6#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates an xdg_toplevel object for the given xdg_surface and gives"]
            #[doc = "the associated wl_surface the xdg_toplevel role. If the surface already"]
            #[doc = "had a role, the zxdg_shell_v6.role error is raised."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_toplevel for more details about what an"]
            #[doc = "xdg_toplevel is and how it is used."]
            async fn get_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_surface_v6#{}.get_toplevel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates an xdg_popup object for the given xdg_surface and gives the"]
            #[doc = "associated wl_surface the xdg_popup role. If the surface already"]
            #[doc = "had a role, the zxdg_shell_v6.role error is raised."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_popup for more details about what an"]
            #[doc = "xdg_popup is and how it is used."]
            async fn get_popup(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                parent: crate::wire::ObjectId,
                positioner: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_surface_v6#{}.get_popup()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(parent))
                    .put_object(Some(positioner))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The window geometry of a surface is its \"visible bounds\" from the"]
            #[doc = "user's perspective. Client-side decorations often have invisible"]
            #[doc = "portions like drop-shadows which should be ignored for the"]
            #[doc = "purposes of aligning, placing and constraining windows."]
            #[doc = ""]
            #[doc = "The window geometry is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "Once the window geometry of the surface is set, it is not possible to"]
            #[doc = "unset it, and it will remain the same until set_window_geometry is"]
            #[doc = "called again, even if a new subsurface or buffer is attached."]
            #[doc = ""]
            #[doc = "If never set, the value is the full bounds of the surface,"]
            #[doc = "including any subsurfaces. This updates dynamically on every"]
            #[doc = "commit. This unset is meant for extremely simple clients."]
            #[doc = ""]
            #[doc = "The arguments are given in the surface-local coordinate space of"]
            #[doc = "the wl_surface associated with this xdg_surface."]
            #[doc = ""]
            #[doc = "The width and height must be greater than zero. Setting an invalid size"]
            #[doc = "will raise an error. When applied, the effective window geometry will be"]
            #[doc = "the set window geometry clamped to the bounding rectangle of the"]
            #[doc = "combined geometry of the surface of the xdg_surface and the associated"]
            #[doc = "subsurfaces."]
            async fn set_window_geometry(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_surface_v6#{}.set_window_geometry()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "When a configure event is received, if a client commits the"]
            #[doc = "surface in response to the configure event, then the client"]
            #[doc = "must make an ack_configure request sometime before the commit"]
            #[doc = "request, passing along the serial of the configure event."]
            #[doc = ""]
            #[doc = "For instance, for toplevel surfaces the compositor might use this"]
            #[doc = "information to move a surface to the top left only when the client has"]
            #[doc = "drawn itself for the maximized or fullscreen state."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it"]
            #[doc = "can respond to one, it only has to ack the last configure event."]
            #[doc = ""]
            #[doc = "A client is not required to commit immediately after sending"]
            #[doc = "an ack_configure request - it may even ack_configure several times"]
            #[doc = "before its next surface commit."]
            #[doc = ""]
            #[doc = "A client may send multiple ack_configure requests before committing, but"]
            #[doc = "only the last request sent before a commit indicates which configure"]
            #[doc = "event the client really is responding to."]
            #[doc = ""]
            #[doc = "If an invalid serial is used, the zxdg_shell_v6.invalid_surface_state"]
            #[doc = "error is raised."]
            async fn ack_configure(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_surface_v6#{}.ack_configure()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The configure event marks the end of a configure sequence. A configure"]
            #[doc = "sequence is a set of one or more events configuring the state of the"]
            #[doc = "xdg_surface, including the final xdg_surface.configure event."]
            #[doc = ""]
            #[doc = "Where applicable, xdg_surface surface roles will during a configure"]
            #[doc = "sequence extend this event as a latched state sent as events before the"]
            #[doc = "xdg_surface.configure event. Such events should be considered to make up"]
            #[doc = "a set of atomically applied configuration states, where the"]
            #[doc = "xdg_surface.configure commits the accumulated state."]
            #[doc = ""]
            #[doc = "Clients should arrange their surface for the new states, and then send"]
            #[doc = "an ack_configure request with the serial sent in this configure event at"]
            #[doc = "some point before committing the new surface."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it can respond"]
            #[doc = "to one, it is free to discard all but the last event it received."]
            async fn configure(&self, serial: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "This interface defines an xdg_surface role which allows a surface to,"]
    #[doc = "among other things, set window-like properties such as maximize,"]
    #[doc = "fullscreen, and minimize, set application-specific metadata like title and"]
    #[doc = "id, and well as trigger user interactive operations such as interactive"]
    #[doc = "resize and move."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_toplevel_v6 {
        use futures_util::SinkExt;
        #[doc = "These values are used to indicate which edge of a surface"]
        #[doc = "is being dragged in a resize operation."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ResizeEdge {
            None = 0u32,
            Top = 1u32,
            Bottom = 2u32,
            Left = 4u32,
            TopLeft = 5u32,
            BottomLeft = 6u32,
            Right = 8u32,
            TopRight = 9u32,
            BottomRight = 10u32,
        }
        impl TryFrom<u32> for ResizeEdge {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Bottom),
                    4u32 => Ok(Self::Left),
                    5u32 => Ok(Self::TopLeft),
                    6u32 => Ok(Self::BottomLeft),
                    8u32 => Ok(Self::Right),
                    9u32 => Ok(Self::TopRight),
                    10u32 => Ok(Self::BottomRight),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ResizeEdge {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "The different state values used on the surface. This is designed for"]
        #[doc = "state values like maximized, fullscreen. It is paired with the"]
        #[doc = "configure event to ensure that both the client and the compositor"]
        #[doc = "setting the state can be synchronized."]
        #[doc = ""]
        #[doc = "States set in this way are double-buffered, see wl_surface.commit."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            #[doc = "the surface is maximized"]
            Maximized = 1u32,
            #[doc = "the surface is fullscreen"]
            Fullscreen = 2u32,
            #[doc = "the surface is being resized"]
            Resizing = 3u32,
            #[doc = "the surface is now activated"]
            Activated = 4u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Maximized),
                    2u32 => Ok(Self::Fullscreen),
                    3u32 => Ok(Self::Resizing),
                    4u32 => Ok(Self::Activated),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_toplevel_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgToplevelV6 {
            const INTERFACE: &'static str = "zxdg_toplevel_v6";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Unmap and destroy the window. The window will be effectively"]
            #[doc = "hidden from the user's point of view, and all state like"]
            #[doc = "maximization, fullscreen, and so on, will be lost."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the \"parent\" of this surface. This window should be stacked"]
            #[doc = "above a parent. The parent surface must be mapped as long as this"]
            #[doc = "surface is mapped."]
            #[doc = ""]
            #[doc = "Parent windows should be set on dialogs, toolboxes, or other"]
            #[doc = "\"auxiliary\" surfaces, so that the parent is raised when the dialog"]
            #[doc = "is raised."]
            async fn set_parent(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                parent: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_parent()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(parent)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a short title for the surface."]
            #[doc = ""]
            #[doc = "This string may be used to identify the surface in a task bar,"]
            #[doc = "window list, or other user interface elements provided by the"]
            #[doc = "compositor."]
            #[doc = ""]
            #[doc = "The string must be encoded in UTF-8."]
            async fn set_title(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                title: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_title()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(title))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set an application identifier for the surface."]
            #[doc = ""]
            #[doc = "The app ID identifies the general class of applications to which"]
            #[doc = "the surface belongs. The compositor can use this to group multiple"]
            #[doc = "surfaces together, or to determine how to launch a new application."]
            #[doc = ""]
            #[doc = "For D-Bus activatable applications, the app ID is used as the D-Bus"]
            #[doc = "service name."]
            #[doc = ""]
            #[doc = "The compositor shell will try to group application surfaces together"]
            #[doc = "by their app ID. As a best practice, it is suggested to select app"]
            #[doc = "ID's that match the basename of the application's .desktop file."]
            #[doc = "For example, \"org.freedesktop.FooViewer\" where the .desktop file is"]
            #[doc = "\"org.freedesktop.FooViewer.desktop\"."]
            #[doc = ""]
            #[doc = "See the desktop-entry specification [0] for more details on"]
            #[doc = "application identifiers and how they relate to well-known D-Bus"]
            #[doc = "names and .desktop files."]
            #[doc = ""]
            #[doc = "[0] http://standards.freedesktop.org/desktop-entry-spec/"]
            async fn set_app_id(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                app_id: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_app_id()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Clients implementing client-side decorations might want to show"]
            #[doc = "a context menu when right-clicking on the decorations, giving the"]
            #[doc = "user a menu that they can use to maximize or minimize the window."]
            #[doc = ""]
            #[doc = "This request asks the compositor to pop up such a window menu at"]
            #[doc = "the given position, relative to the local surface coordinates of"]
            #[doc = "the parent surface. There are no guarantees as to what menu items"]
            #[doc = "the window menu contains."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event."]
            async fn show_window_menu(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.show_window_menu()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Start an interactive, user-driven move of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event. The passed"]
            #[doc = "serial is used to determine the type of interactive move (touch,"]
            #[doc = "pointer, etc)."]
            #[doc = ""]
            #[doc = "The server may ignore move requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized), or if the passed serial"]
            #[doc = "is no longer valid."]
            #[doc = ""]
            #[doc = "If triggered, the surface will lose the focus of the device"]
            #[doc = "(wl_pointer, wl_touch, etc) used for the move. It is up to the"]
            #[doc = "compositor to visually indicate that the move is taking place, such as"]
            #[doc = "updating a pointer cursor, during the move. There is no guarantee"]
            #[doc = "that the device focus will return when the move is completed."]
            async fn r#move(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.move()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Start a user-driven, interactive resize of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action"]
            #[doc = "like a button press, key press, or touch down event. The passed"]
            #[doc = "serial is used to determine the type of interactive resize (touch,"]
            #[doc = "pointer, etc)."]
            #[doc = ""]
            #[doc = "The server may ignore resize requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized)."]
            #[doc = ""]
            #[doc = "If triggered, the client will receive configure events with the"]
            #[doc = "\"resize\" state enum value and the expected sizes. See the \"resize\""]
            #[doc = "enum value for more details about what is required. The client"]
            #[doc = "must also acknowledge configure events using \"ack_configure\". After"]
            #[doc = "the resize is completed, the client will receive another \"configure\""]
            #[doc = "event without the resize state."]
            #[doc = ""]
            #[doc = "If triggered, the surface also will lose the focus of the device"]
            #[doc = "(wl_pointer, wl_touch, etc) used for the resize. It is up to the"]
            #[doc = "compositor to visually indicate that the resize is taking place,"]
            #[doc = "such as updating a pointer cursor, during the resize. There is no"]
            #[doc = "guarantee that the device focus will return when the resize is"]
            #[doc = "completed."]
            #[doc = ""]
            #[doc = "The edges parameter specifies how the surface should be resized,"]
            #[doc = "and is one of the values of the resize_edge enum. The compositor"]
            #[doc = "may use this information to update the surface position for"]
            #[doc = "example when dragging the top left corner. The compositor may also"]
            #[doc = "use this information to adapt its behavior, e.g. choose an"]
            #[doc = "appropriate cursor image."]
            async fn resize(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                edges: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.resize()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_uint(edges)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a maximum size for the window."]
            #[doc = ""]
            #[doc = "The client can specify a maximum size so that the compositor does"]
            #[doc = "not try to configure the window beyond this size."]
            #[doc = ""]
            #[doc = "The width and height arguments are in window geometry coordinates."]
            #[doc = "See xdg_surface.set_window_geometry."]
            #[doc = ""]
            #[doc = "Values set in this way are double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The compositor can use this information to allow or disallow"]
            #[doc = "different states like maximize or fullscreen and draw accurate"]
            #[doc = "animations."]
            #[doc = ""]
            #[doc = "Similarly, a tiling window manager may use this information to"]
            #[doc = "place and resize client windows in a more effective way."]
            #[doc = ""]
            #[doc = "The client should not rely on the compositor to obey the maximum"]
            #[doc = "size. The compositor may decide to ignore the values set by the"]
            #[doc = "client and request a larger size."]
            #[doc = ""]
            #[doc = "If never set, or a value of zero in the request, means that the"]
            #[doc = "client has no expected maximum size in the given dimension."]
            #[doc = "As a result, a client wishing to reset the maximum size"]
            #[doc = "to an unspecified state can use zero for width and height in the"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Requesting a maximum size to be smaller than the minimum size of"]
            #[doc = "a surface is illegal and will result in a protocol error."]
            #[doc = ""]
            #[doc = "The width and height must be greater than or equal to zero. Using"]
            #[doc = "strictly negative values for width and height will result in the"]
            #[doc = "zxdg_shell_v6.invalid_surface_state error being raised."]
            async fn set_max_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_max_size()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a minimum size for the window."]
            #[doc = ""]
            #[doc = "The client can specify a minimum size so that the compositor does"]
            #[doc = "not try to configure the window below this size."]
            #[doc = ""]
            #[doc = "The width and height arguments are in window geometry coordinates."]
            #[doc = "See xdg_surface.set_window_geometry."]
            #[doc = ""]
            #[doc = "Values set in this way are double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The compositor can use this information to allow or disallow"]
            #[doc = "different states like maximize or fullscreen and draw accurate"]
            #[doc = "animations."]
            #[doc = ""]
            #[doc = "Similarly, a tiling window manager may use this information to"]
            #[doc = "place and resize client windows in a more effective way."]
            #[doc = ""]
            #[doc = "The client should not rely on the compositor to obey the minimum"]
            #[doc = "size. The compositor may decide to ignore the values set by the"]
            #[doc = "client and request a smaller size."]
            #[doc = ""]
            #[doc = "If never set, or a value of zero in the request, means that the"]
            #[doc = "client has no expected minimum size in the given dimension."]
            #[doc = "As a result, a client wishing to reset the minimum size"]
            #[doc = "to an unspecified state can use zero for width and height in the"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Requesting a minimum size to be larger than the maximum size of"]
            #[doc = "a surface is illegal and will result in a protocol error."]
            #[doc = ""]
            #[doc = "The width and height must be greater than or equal to zero. Using"]
            #[doc = "strictly negative values for width and height will result in the"]
            #[doc = "zxdg_shell_v6.invalid_surface_state error being raised."]
            async fn set_min_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_min_size()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Maximize the surface."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be maximized, the compositor"]
            #[doc = "will respond by emitting a configure event with the \"maximized\" state"]
            #[doc = "and the required window geometry. The client should then update its"]
            #[doc = "content, drawing it in a maximized state, i.e. without shadow or other"]
            #[doc = "decoration outside of the window geometry. The client must also"]
            #[doc = "acknowledge the configure when committing the new content (see"]
            #[doc = "ack_configure)."]
            #[doc = ""]
            #[doc = "It is up to the compositor to decide how and where to maximize the"]
            #[doc = "surface, for example which output and what region of the screen should"]
            #[doc = "be used."]
            #[doc = ""]
            #[doc = "If the surface was already maximized, the compositor will still emit"]
            #[doc = "a configure event with the \"maximized\" state."]
            #[doc = ""]
            #[doc = "Note that unrelated compositor side state changes may cause"]
            #[doc = "configure events to be emitted at any time, meaning trying to"]
            #[doc = "match this request to a specific future configure event is"]
            #[doc = "futile."]
            async fn set_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Unmaximize the surface."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be unmaximized, the compositor"]
            #[doc = "will respond by emitting a configure event without the \"maximized\""]
            #[doc = "state. If available, the compositor will include the window geometry"]
            #[doc = "dimensions the window had prior to being maximized in the configure"]
            #[doc = "request. The client must then update its content, drawing it in a"]
            #[doc = "regular state, i.e. potentially with shadow, etc. The client must also"]
            #[doc = "acknowledge the configure when committing the new content (see"]
            #[doc = "ack_configure)."]
            #[doc = ""]
            #[doc = "It is up to the compositor to position the surface after it was"]
            #[doc = "unmaximized; usually the position the surface had before maximizing, if"]
            #[doc = "applicable."]
            #[doc = ""]
            #[doc = "If the surface was already not maximized, the compositor will still"]
            #[doc = "emit a configure event without the \"maximized\" state."]
            #[doc = ""]
            #[doc = "Note that unrelated changes in the state of compositor may cause"]
            #[doc = "configure events to be emitted by the compositor between processing"]
            #[doc = "this request and emitting corresponding configure event, so trying"]
            #[doc = "to match the request with the event is futile."]
            async fn unset_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.unset_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Make the surface fullscreen."]
            #[doc = ""]
            #[doc = "You can specify an output that you would prefer to be fullscreen."]
            #[doc = "If this value is NULL, it's up to the compositor to choose which"]
            #[doc = "display will be used to map this surface."]
            #[doc = ""]
            #[doc = "If the surface doesn't cover the whole output, the compositor will"]
            #[doc = "position the surface in the center of the output and compensate with"]
            #[doc = "black borders filling the rest of the output."]
            async fn set_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_fullscreen()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 11u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn unset_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.unset_fullscreen()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 12u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request that the compositor minimize your surface. There is no"]
            #[doc = "way to know if the surface is currently minimized, nor is there"]
            #[doc = "any way to unset minimization on this surface."]
            #[doc = ""]
            #[doc = "If you are looking to throttle redrawing when minimized, please"]
            #[doc = "instead use the wl_surface.frame event for this, as this will"]
            #[doc = "also work with live previews on windows in Alt-Tab, Expose or"]
            #[doc = "similar compositor features."]
            async fn set_minimized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_v6#{}.set_minimized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 13u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This configure event asks the client to resize its toplevel surface or"]
            #[doc = "to change its state. The configured state should not be applied"]
            #[doc = "immediately. See xdg_surface.configure for details."]
            #[doc = ""]
            #[doc = "The width and height arguments specify a hint to the window"]
            #[doc = "about how its surface should be resized in window geometry"]
            #[doc = "coordinates. See set_window_geometry."]
            #[doc = ""]
            #[doc = "If the width or height arguments are zero, it means the client"]
            #[doc = "should decide its own window dimension. This may happen when the"]
            #[doc = "compositor needs to configure the state of the surface but doesn't"]
            #[doc = "have any information about any previous or expected dimension."]
            #[doc = ""]
            #[doc = "The states listed in the event specify how the width/height"]
            #[doc = "arguments should be interpreted, and possibly how it should be"]
            #[doc = "drawn."]
            #[doc = ""]
            #[doc = "Clients must send an ack_configure in response to this event. See"]
            #[doc = "xdg_surface.configure and xdg_surface.ack_configure for details."]
            async fn configure(
                &self,
                width: i32,
                height: i32,
                states: Vec<u8>,
            ) -> crate::client::Result<()>;
            #[doc = "The close event is sent by the compositor when the user"]
            #[doc = "wants the surface to be closed. This should be equivalent to"]
            #[doc = "the user clicking the close button in client-side decorations,"]
            #[doc = "if your application has any."]
            #[doc = ""]
            #[doc = "This is only a request that the user intends to close the"]
            #[doc = "window. The client may choose to ignore this request, or show"]
            #[doc = "a dialog to ask the user to save their data, etc."]
            async fn close(&self) -> crate::client::Result<()>;
        }
    }
    #[doc = "A popup surface is a short-lived, temporary surface. It can be used to"]
    #[doc = "implement for example menus, popovers, tooltips and other similar user"]
    #[doc = "interface concepts."]
    #[doc = ""]
    #[doc = "A popup can be made to take an explicit grab. See xdg_popup.grab for"]
    #[doc = "details."]
    #[doc = ""]
    #[doc = "When the popup is dismissed, a popup_done event will be sent out, and at"]
    #[doc = "the same time the surface will be unmapped. See the xdg_popup.popup_done"]
    #[doc = "event for details."]
    #[doc = ""]
    #[doc = "Explicitly destroying the xdg_popup object will also dismiss the popup and"]
    #[doc = "unmap the surface. Clients that want to dismiss the popup when another"]
    #[doc = "surface of their own is clicked should dismiss the popup using the destroy"]
    #[doc = "request."]
    #[doc = ""]
    #[doc = "The parent surface must have either the xdg_toplevel or xdg_popup surface"]
    #[doc = "role."]
    #[doc = ""]
    #[doc = "A newly created xdg_popup will be stacked on top of all previously created"]
    #[doc = "xdg_popup surfaces associated with the same xdg_toplevel."]
    #[doc = ""]
    #[doc = "The parent of an xdg_popup must be mapped (see the xdg_surface"]
    #[doc = "description) before the xdg_popup itself."]
    #[doc = ""]
    #[doc = "The x and y arguments passed when creating the popup object specify"]
    #[doc = "where the top left of the popup should be placed, relative to the"]
    #[doc = "local surface coordinates of the parent surface. See"]
    #[doc = "xdg_surface.get_popup. An xdg_popup must intersect with or be at least"]
    #[doc = "partially adjacent to its parent surface."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xdg_popup state to take effect."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_popup_v6 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "tried to grab after being mapped"]
            InvalidGrab = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidGrab),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_popup_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgPopupV6 {
            const INTERFACE: &'static str = "zxdg_popup_v6";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This destroys the popup. Explicitly destroying the xdg_popup"]
            #[doc = "object will also dismiss the popup, and unmap the surface."]
            #[doc = ""]
            #[doc = "If this xdg_popup is not the \"topmost\" popup, a protocol error"]
            #[doc = "will be sent."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_popup_v6#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request makes the created popup take an explicit grab. An explicit"]
            #[doc = "grab will be dismissed when the user dismisses the popup, or when the"]
            #[doc = "client destroys the xdg_popup. This can be done by the user clicking"]
            #[doc = "outside the surface, using the keyboard, or even locking the screen"]
            #[doc = "through closing the lid or a timeout."]
            #[doc = ""]
            #[doc = "If the compositor denies the grab, the popup will be immediately"]
            #[doc = "dismissed."]
            #[doc = ""]
            #[doc = "This request must be used in response to some sort of user action like a"]
            #[doc = "button press, key press, or touch down event. The serial number of the"]
            #[doc = "event should be passed as 'serial'."]
            #[doc = ""]
            #[doc = "The parent of a grabbing popup must either be an xdg_toplevel surface or"]
            #[doc = "another xdg_popup with an explicit grab. If the parent is another"]
            #[doc = "xdg_popup it means that the popups are nested, with this popup now being"]
            #[doc = "the topmost popup."]
            #[doc = ""]
            #[doc = "Nested popups must be destroyed in the reverse order they were created"]
            #[doc = "in, e.g. the only popup you are allowed to destroy at all times is the"]
            #[doc = "topmost one."]
            #[doc = ""]
            #[doc = "When compositors choose to dismiss a popup, they may dismiss every"]
            #[doc = "nested grabbing popup as well. When a compositor dismisses popups, it"]
            #[doc = "will follow the same dismissing order as required from the client."]
            #[doc = ""]
            #[doc = "The parent of a grabbing popup must either be another xdg_popup with an"]
            #[doc = "active explicit grab, or an xdg_popup or xdg_toplevel, if there are no"]
            #[doc = "explicit grabs already taken."]
            #[doc = ""]
            #[doc = "If the topmost grabbing popup is destroyed, the grab will be returned to"]
            #[doc = "the parent of the popup, if that parent previously had an explicit grab."]
            #[doc = ""]
            #[doc = "If the parent is a grabbing popup which has already been dismissed, this"]
            #[doc = "popup will be immediately dismissed. If the parent is a popup that did"]
            #[doc = "not take an explicit grab, an error will be raised."]
            #[doc = ""]
            #[doc = "During a popup grab, the client owning the grab will receive pointer"]
            #[doc = "and touch events for all their surfaces as normal (similar to an"]
            #[doc = "\"owner-events\" grab in X11 parlance), while the top most grabbing popup"]
            #[doc = "will always have keyboard focus."]
            async fn grab(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_popup_v6#{}.grab()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event asks the popup surface to configure itself given the"]
            #[doc = "configuration. The configured state should not be applied immediately."]
            #[doc = "See xdg_surface.configure for details."]
            #[doc = ""]
            #[doc = "The x and y arguments represent the position the popup was placed at"]
            #[doc = "given the xdg_positioner rule, relative to the upper left corner of the"]
            #[doc = "window geometry of the parent surface."]
            async fn configure(
                &self,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()>;
            #[doc = "The popup_done event is sent out when a popup is dismissed by the"]
            #[doc = "compositor. The client should destroy the xdg_popup object at this"]
            #[doc = "point."]
            async fn popup_done(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol is application-specific to meet the needs of the X11"]
#[doc = "protocol through Xwayland. It provides a way for Xwayland to request"]
#[doc = "all keyboard events to be forwarded to a surface even when the"]
#[doc = "surface does not have keyboard focus."]
#[doc = ""]
#[doc = "In the X11 protocol, a client may request an \"active grab\" on the"]
#[doc = "keyboard. On success, all key events are reported only to the"]
#[doc = "grabbing X11 client. For details, see XGrabKeyboard(3)."]
#[doc = ""]
#[doc = "The core Wayland protocol does not have a notion of an active"]
#[doc = "keyboard grab. When running in Xwayland, X11 applications may"]
#[doc = "acquire an active grab inside Xwayland but that cannot be translated"]
#[doc = "to the Wayland compositor who may set the input focus to some other"]
#[doc = "surface. In doing so, it breaks the X11 client assumption that all"]
#[doc = "key events are reported to the grabbing client."]
#[doc = ""]
#[doc = "This protocol specifies a way for Xwayland to request all keyboard"]
#[doc = "be directed to the given surface. The protocol does not guarantee"]
#[doc = "that the compositor will honor this request and it does not"]
#[doc = "prescribe user interfaces on how to handle the respond. For example,"]
#[doc = "a compositor may inform the user that all key events are now"]
#[doc = "forwarded to the given client surface, or it may ask the user for"]
#[doc = "permission to do so."]
#[doc = ""]
#[doc = "Compositors are required to restrict access to this application"]
#[doc = "specific protocol to Xwayland alone."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible"]
#[doc = "changes may be added together with the corresponding interface"]
#[doc = "version bump."]
#[doc = "Backward incompatible changes are done by bumping the version"]
#[doc = "number in the protocol and interface names and resetting the"]
#[doc = "interface version. Once the protocol is to be declared stable,"]
#[doc = "the 'z' prefix and the version number in the protocol and"]
#[doc = "interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod xwayland_keyboard_grab_unstable_v1 {
    #[doc = "A global interface used for grabbing the keyboard."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_xwayland_keyboard_grab_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_xwayland_keyboard_grab_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpXwaylandKeyboardGrabManagerV1 {
            const INTERFACE: &'static str = "zwp_xwayland_keyboard_grab_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the keyboard grab manager."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_xwayland_keyboard_grab_manager_v1#{}.destroy()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The grab_keyboard request asks for a grab of the keyboard, forcing"]
            #[doc = "the keyboard focus for the given seat upon the given surface."]
            #[doc = ""]
            #[doc = "The protocol provides no guarantee that the grab is ever satisfied,"]
            #[doc = "and does not require the compositor to send an error if the grab"]
            #[doc = "cannot ever be satisfied. It is thus possible to request a keyboard"]
            #[doc = "grab that will never be effective."]
            #[doc = ""]
            #[doc = "The protocol:"]
            #[doc = ""]
            #[doc = "* does not guarantee that the grab itself is applied for a surface,"]
            #[doc = "the grab request may be silently ignored by the compositor,"]
            #[doc = "* does not guarantee that any events are sent to this client even"]
            #[doc = "if the grab is applied to a surface,"]
            #[doc = "* does not guarantee that events sent to this client are exhaustive,"]
            #[doc = "a compositor may filter some events for its own consumption,"]
            #[doc = "* does not guarantee that events sent to this client are continuous,"]
            #[doc = "a compositor may change and reroute keyboard events while the grab"]
            #[doc = "is nominally active."]
            async fn grab_keyboard(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_xwayland_keyboard_grab_manager_v1#{}.grab_keyboard()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A global interface used for grabbing the keyboard."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_xwayland_keyboard_grab_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_xwayland_keyboard_grab_v1 interface. See the module level documentation for more info"]
        pub trait ZwpXwaylandKeyboardGrabV1 {
            const INTERFACE: &'static str = "zwp_xwayland_keyboard_grab_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the grabbed keyboard object. If applicable, the compositor"]
            #[doc = "will ungrab the keyboard."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_xwayland_keyboard_grab_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol allows compositors to act as input methods and to send text"]
#[doc = "to applications. A text input object is used to manage state of what are"]
#[doc = "typically text entry fields in the application."]
#[doc = ""]
#[doc = "This document adheres to the RFC 2119 when using words like \"must\","]
#[doc = "\"should\", \"may\", etc."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod text_input_unstable_v3 {
    #[doc = "The zwp_text_input_v3 interface represents text input and input methods"]
    #[doc = "associated with a seat. It provides enter/leave events to follow the"]
    #[doc = "text input focus for a seat."]
    #[doc = ""]
    #[doc = "Requests are used to enable/disable the text-input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about the entered text is sent to the text-input object"]
    #[doc = "via the preedit_string and commit_string events."]
    #[doc = ""]
    #[doc = "Text is valid UTF-8 encoded, indices and lengths are in bytes. Indices"]
    #[doc = "must not point to middle bytes inside a code point: they must either"]
    #[doc = "point to the first byte of a code point or to the end of the buffer."]
    #[doc = "Lengths must be measured between two valid indices."]
    #[doc = ""]
    #[doc = "Focus moving throughout surfaces will result in the emission of"]
    #[doc = "zwp_text_input_v3.enter and zwp_text_input_v3.leave events. The focused"]
    #[doc = "surface must commit zwp_text_input_v3.enable and"]
    #[doc = "zwp_text_input_v3.disable requests as the keyboard focus moves across"]
    #[doc = "editable and non-editable elements of the UI. Those two requests are not"]
    #[doc = "expected to be paired with each other, the compositor must be able to"]
    #[doc = "handle consecutive series of the same request."]
    #[doc = ""]
    #[doc = "State is sent by the state requests (set_surrounding_text,"]
    #[doc = "set_content_type and set_cursor_rectangle) and a commit request. After an"]
    #[doc = "enter event or disable request all state information is invalidated and"]
    #[doc = "needs to be resent by the client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_v3 {
        use futures_util::SinkExt;
        #[doc = "Reason for the change of surrounding text or cursor posision."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ChangeCause {
            #[doc = "input method caused the change"]
            InputMethod = 0u32,
            #[doc = "something else than the input method caused the change"]
            Other = 1u32,
        }
        impl TryFrom<u32> for ChangeCause {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InputMethod),
                    1u32 => Ok(Self::Other),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ChangeCause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behavior"] const None = 0u32 ; # [doc = "suggest word completions"] const Completion = 1u32 ; # [doc = "suggest word corrections"] const Spellcheck = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just Latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
        impl TryFrom<u32> for ContentHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ContentHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
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
            #[doc = "input a password (combine with sensitive_data hint)"]
            Password = 8u32,
            #[doc = "input is a numeric password (combine with sensitive_data hint)"]
            Pin = 9u32,
            #[doc = "input a date"]
            Date = 10u32,
            #[doc = "input a time"]
            Time = 11u32,
            #[doc = "input a date and time"]
            Datetime = 12u32,
            #[doc = "input for a terminal"]
            Terminal = 13u32,
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
                    9u32 => Ok(Self::Pin),
                    10u32 => Ok(Self::Date),
                    11u32 => Ok(Self::Time),
                    12u32 => Ok(Self::Datetime),
                    13u32 => Ok(Self::Terminal),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ContentPurpose {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_text_input_v3 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV3 {
            const INTERFACE: &'static str = "zwp_text_input_v3";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_text_input object. Also disables all surfaces enabled"]
            #[doc = "through this wp_text_input object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Requests text input on the surface previously obtained from the enter"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "This request must be issued every time the active text input changes"]
            #[doc = "to a new one, including within the current surface. Use"]
            #[doc = "zwp_text_input_v3.disable when there is no longer any input focus on"]
            #[doc = "the current surface."]
            #[doc = ""]
            #[doc = "Clients must not enable more than one text input on the single seat"]
            #[doc = "and should disable the current text input before enabling the new one."]
            #[doc = "At most one instance of text input may be in enabled state per instance,"]
            #[doc = "Requests to enable the another text input when some text input is active"]
            #[doc = "must be ignored by compositor."]
            #[doc = ""]
            #[doc = "This request resets all state associated with previous enable, disable,"]
            #[doc = "set_surrounding_text, set_text_change_cause, set_content_type, and"]
            #[doc = "set_cursor_rectangle requests, as well as the state associated with"]
            #[doc = "preedit_string, commit_string, and delete_surrounding_text events."]
            #[doc = ""]
            #[doc = "The set_surrounding_text, set_content_type and set_cursor_rectangle"]
            #[doc = "requests must follow if the text input supports the necessary"]
            #[doc = "functionality."]
            #[doc = ""]
            #[doc = "State set with this request is double-buffered. It will get applied on"]
            #[doc = "the next zwp_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The changes must be applied by the compositor after issuing a"]
            #[doc = "zwp_text_input_v3.commit request."]
            async fn enable(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.enable()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Explicitly disable text input on the current surface (typically when"]
            #[doc = "there is no focus on any text entry inside the surface)."]
            #[doc = ""]
            #[doc = "State set with this request is double-buffered. It will get applied on"]
            #[doc = "the next zwp_text_input_v3.commit request."]
            async fn disable(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.disable()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets the surrounding plain text around the input, excluding the preedit"]
            #[doc = "text."]
            #[doc = ""]
            #[doc = "The client should notify the compositor of any changes in any of the"]
            #[doc = "values carried with this request, including changes caused by handling"]
            #[doc = "incoming text-input events as well as changes caused by other"]
            #[doc = "mechanisms like keyboard typing."]
            #[doc = ""]
            #[doc = "If the client is unaware of the text around the cursor, it should not"]
            #[doc = "issue this request, to signify lack of support to the compositor."]
            #[doc = ""]
            #[doc = "Text is UTF-8 encoded, and should include the cursor position, the"]
            #[doc = "complete selection and additional characters before and after them."]
            #[doc = "There is a maximum length of wayland messages, so text can not be"]
            #[doc = "longer than 4000 bytes."]
            #[doc = ""]
            #[doc = "Cursor is the byte offset of the cursor within text buffer."]
            #[doc = ""]
            #[doc = "Anchor is the byte offset of the selection anchor within text buffer."]
            #[doc = "If there is no selected text, anchor is the same as cursor."]
            #[doc = ""]
            #[doc = "If any preedit text is present, it is replaced with a cursor for the"]
            #[doc = "purpose of this event."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial state for affected fields is empty, meaning that the text"]
            #[doc = "input does not support sending surrounding text. If the empty values"]
            #[doc = "get applied, subsequent attempts to change them may have no effect."]
            async fn set_surrounding_text(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                text: String,
                cursor: i32,
                anchor: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.set_surrounding_text()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(text))
                    .put_int(cursor)
                    .put_int(anchor)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Tells the compositor why the text surrounding the cursor changed."]
            #[doc = ""]
            #[doc = "Whenever the client detects an external change in text, cursor, or"]
            #[doc = "anchor posision, it must issue this request to the compositor. This"]
            #[doc = "request is intended to give the input method a chance to update the"]
            #[doc = "preedit text in an appropriate way, e.g. by removing it when the user"]
            #[doc = "starts typing with a keyboard."]
            #[doc = ""]
            #[doc = "cause describes the source of the change."]
            #[doc = ""]
            #[doc = "The value set with this request is double-buffered. It must be applied"]
            #[doc = "and reset to initial at the next zwp_text_input_v3.commit request."]
            #[doc = ""]
            #[doc = "The initial value of cause is input_method."]
            async fn set_text_change_cause(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                cause: ChangeCause,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.set_text_change_cause()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(cause as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some of"]
            #[doc = "the behavior."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request."]
            #[doc = "Subsequent attempts to update them may have no effect. The values"]
            #[doc = "remain valid until the next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial value for hint is none, and the initial value for purpose"]
            #[doc = "is normal."]
            async fn set_content_type(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                hint: ContentHint,
                purpose: ContentPurpose,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.set_content_type()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(hint.bits())
                    .put_uint(purpose as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Marks an area around the cursor as a x, y, width, height rectangle in"]
            #[doc = "surface local coordinates."]
            #[doc = ""]
            #[doc = "Allows the compositor to put a window with word suggestions near the"]
            #[doc = "cursor, without obstructing the text being input."]
            #[doc = ""]
            #[doc = "If the client is unaware of the position of edited text, it should not"]
            #[doc = "issue this request, to signify lack of support to the compositor."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next zwp_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial values describing a cursor rectangle are empty. That means"]
            #[doc = "the text input does not support describing the cursor area. If the"]
            #[doc = "empty values get applied, subsequent attempts to change them may have"]
            #[doc = "no effect."]
            async fn set_cursor_rectangle(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.set_cursor_rectangle()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Atomically applies state changes recently sent to the compositor."]
            #[doc = ""]
            #[doc = "The commit request establishes and updates the state of the client, and"]
            #[doc = "must be issued after any changes to apply them."]
            #[doc = ""]
            #[doc = "Text input state (enabled status, content purpose, content hint,"]
            #[doc = "surrounding text and change cause, cursor rectangle) is conceptually"]
            #[doc = "double-buffered within the context of a text input, i.e. between a"]
            #[doc = "committed enable request and the following committed enable or disable"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Protocol requests modify the pending state, as opposed to the current"]
            #[doc = "state in use by the input method. A commit request atomically applies"]
            #[doc = "all pending state, replacing the current state. After commit, the new"]
            #[doc = "pending state is as documented for each related request."]
            #[doc = ""]
            #[doc = "Requests are applied in the order of arrival."]
            #[doc = ""]
            #[doc = "Neither current nor pending state are modified unless noted otherwise."]
            #[doc = ""]
            #[doc = "The compositor must count the number of commit requests coming from"]
            #[doc = "each zwp_text_input_v3 object and use the count as the serial in done"]
            #[doc = "events."]
            async fn commit(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_v3#{}.commit()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Notification that this seat's text-input focus is on a certain surface."]
            #[doc = ""]
            #[doc = "If client has created multiple text input objects, compositor must send"]
            #[doc = "this event to all of them."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus. This event sets the current surface for the"]
            #[doc = "text-input object."]
            async fn enter(&self, surface: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "Notification that this seat's text-input focus is no longer on a"]
            #[doc = "certain surface. The client should reset any preedit string previously"]
            #[doc = "set."]
            #[doc = ""]
            #[doc = "The leave notification clears the current surface. It is sent before"]
            #[doc = "the enter notification for the new focus. After leave event, compositor"]
            #[doc = "must ignore requests from any text input instances until next enter"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            async fn leave(&self, surface: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "Notify when a new composing text (pre-edit) should be set at the"]
            #[doc = "current cursor position. Any previously set composing text must be"]
            #[doc = "removed. Any previously existing selected text must be removed."]
            #[doc = ""]
            #[doc = "The argument text contains the pre-edit string buffer."]
            #[doc = ""]
            #[doc = "The parameters cursor_begin and cursor_end are counted in bytes"]
            #[doc = "relative to the beginning of the submitted text buffer. Cursor should"]
            #[doc = "be hidden when both are equal to -1."]
            #[doc = ""]
            #[doc = "They could be represented by the client as a line if both values are"]
            #[doc = "the same, or as a text highlight otherwise."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string, and cursor_begin,"]
            #[doc = "cursor_end and cursor_hidden are all 0."]
            async fn preedit_string(
                &self,
                text: Option<String>,
                cursor_begin: i32,
                cursor_end: i32,
            ) -> crate::client::Result<()>;
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit)."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string."]
            async fn commit_string(&self, text: Option<String>) -> crate::client::Result<()>;
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted."]
            #[doc = ""]
            #[doc = "Before_length and after_length are the number of bytes before and after"]
            #[doc = "the current cursor index (excluding the selection) to delete."]
            #[doc = ""]
            #[doc = "If a preedit text is present, in effect before_length is counted from"]
            #[doc = "the beginning of it, and after_length from its end (see done event"]
            #[doc = "sequence)."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next zwp_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial values of both before_length and after_length are 0."]
            async fn delete_surrounding_text(
                &self,
                before_length: u32,
                after_length: u32,
            ) -> crate::client::Result<()>;
            #[doc = "Instruct the application to apply changes to state requested by the"]
            #[doc = "preedit_string, commit_string and delete_surrounding_text events. The"]
            #[doc = "state relating to these events is double-buffered, and each one"]
            #[doc = "modifies the pending state. This event replaces the current state with"]
            #[doc = "the pending state."]
            #[doc = ""]
            #[doc = "The application must proceed by evaluating the changes in the following"]
            #[doc = "order:"]
            #[doc = ""]
            #[doc = "1. Replace existing preedit string with the cursor."]
            #[doc = "2. Delete requested surrounding text."]
            #[doc = "3. Insert commit string with the cursor at its end."]
            #[doc = "4. Calculate surrounding text to send."]
            #[doc = "5. Insert new preedit text in cursor position."]
            #[doc = "6. Place cursor inside preedit text."]
            #[doc = ""]
            #[doc = "The serial number reflects the last state of the zwp_text_input_v3"]
            #[doc = "object known to the compositor. The value of the serial argument must"]
            #[doc = "be equal to the number of commit requests already issued on that object."]
            #[doc = ""]
            #[doc = "When the client receives a done event with a serial different than the"]
            #[doc = "number of past commit requests, it must proceed with evaluating and"]
            #[doc = "applying the changes as normal, except it should not change the current"]
            #[doc = "state of the zwp_text_input_v3 object. All pending state requests"]
            #[doc = "(set_surrounding_text, set_content_type and set_cursor_rectangle) on"]
            #[doc = "the zwp_text_input_v3 object should be sent and committed after"]
            #[doc = "receiving a zwp_text_input_v3.done event with a matching serial."]
            async fn done(&self, serial: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_manager_v3 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_text_input_manager_v3 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV3 {
            const INTERFACE: &'static str = "zwp_text_input_manager_v3";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_text_input_manager object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_text_input_manager_v3#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Creates a new text-input object for a given seat."]
            async fn get_text_input(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_text_input_manager_v3#{}.get_text_input()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = "This protocol specifies a way for making it possible to reference a surface"]
#[doc = "of a different client. With such a reference, a client can, by using the"]
#[doc = "interfaces provided by this protocol, manipulate the relationship between"]
#[doc = "its own surfaces and the surface of some other client. For example, stack"]
#[doc = "some of its own surface above the other clients surface."]
#[doc = ""]
#[doc = "In order for a client A to get a reference of a surface of client B, client"]
#[doc = "B must first export its surface using xdg_exporter.export. Upon doing this,"]
#[doc = "client B will receive a handle (a unique string) that it may share with"]
#[doc = "client A in some way (for example D-Bus). After client A has received the"]
#[doc = "handle from client B, it may use xdg_importer.import to create a reference"]
#[doc = "to the surface client B just exported. See the corresponding requests for"]
#[doc = "details."]
#[doc = ""]
#[doc = "A possible use case for this is out-of-process dialogs. For example when a"]
#[doc = "sandboxed client without file system access needs the user to select a file"]
#[doc = "on the file system, given sandbox environment support, it can export its"]
#[doc = "surface, passing the exported surface handle to an unsandboxed process that"]
#[doc = "can show a file browser dialog and stack it above the sandboxed client's"]
#[doc = "surface."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and backward"]
#[doc = "incompatible changes may be made. Backward compatible changes may be added"]
#[doc = "together with the corresponding interface version bump. Backward"]
#[doc = "incompatible changes are done by bumping the version number in the protocol"]
#[doc = "and interface names and resetting the interface version. Once the protocol"]
#[doc = "is to be declared stable, the 'z' prefix and the version number in the"]
#[doc = "protocol and interface names are removed and the interface version number is"]
#[doc = "reset."]
#[allow(clippy::module_inception)]
pub mod xdg_foreign_unstable_v1 {
    #[doc = "A global interface used for exporting surfaces that can later be imported"]
    #[doc = "using xdg_importer."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_exporter_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_exporter_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgExporterV1 {
            const INTERFACE: &'static str = "zxdg_exporter_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the xdg_exporter object will no longer be"]
            #[doc = "used."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_exporter_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The export request exports the passed surface so that it can later be"]
            #[doc = "imported via xdg_importer. When called, a new xdg_exported object will"]
            #[doc = "be created and xdg_exported.handle will be sent immediately. See the"]
            #[doc = "corresponding interface and event for details."]
            #[doc = ""]
            #[doc = "A surface may be exported multiple times, and each exported handle may"]
            #[doc = "be used to create an xdg_imported multiple times. Only xdg_surface"]
            #[doc = "surfaces may be exported."]
            async fn export(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_exporter_v1#{}.export()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A global interface used for importing surfaces exported by xdg_exporter."]
    #[doc = "With this interface, a client can create a reference to a surface of"]
    #[doc = "another client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_importer_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_importer_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgImporterV1 {
            const INTERFACE: &'static str = "zxdg_importer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that the xdg_importer object will no longer be"]
            #[doc = "used."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_importer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The import request imports a surface from any client given a handle"]
            #[doc = "retrieved by exporting said surface using xdg_exporter.export. When"]
            #[doc = "called, a new xdg_imported object will be created. This new object"]
            #[doc = "represents the imported surface, and the importing client can"]
            #[doc = "manipulate its relationship using it. See xdg_imported for details."]
            async fn import(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                handle: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_importer_v1#{}.import()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_string(Some(handle))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An xdg_exported object represents an exported reference to a surface. The"]
    #[doc = "exported surface may be referenced as long as the xdg_exported object not"]
    #[doc = "destroyed. Destroying the xdg_exported invalidates any relationship the"]
    #[doc = "importer may have established using xdg_imported."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_exported_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_exported_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgExportedV1 {
            const INTERFACE: &'static str = "zxdg_exported_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Revoke the previously exported surface. This invalidates any"]
            #[doc = "relationship the importer may have set up using the xdg_imported created"]
            #[doc = "given the handle sent via xdg_exported.handle."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_exported_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The handle event contains the unique handle of this exported surface"]
            #[doc = "reference. It may be shared with any client, which then can use it to"]
            #[doc = "import the surface by calling xdg_importer.import. A handle may be"]
            #[doc = "used to import the surface multiple times."]
            async fn handle(&self, handle: String) -> crate::client::Result<()>;
        }
    }
    #[doc = "An xdg_imported object represents an imported reference to surface exported"]
    #[doc = "by some client. A client can use this interface to manipulate"]
    #[doc = "relationships between its own surfaces and the imported surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_imported_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_imported_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgImportedV1 {
            const INTERFACE: &'static str = "zxdg_imported_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the compositor that it will no longer use the xdg_imported"]
            #[doc = "object. Any relationship that may have been set up will at this point"]
            #[doc = "be invalidated."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_imported_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the imported surface as the parent of some surface of the client."]
            #[doc = "The passed surface must be a toplevel xdg_surface. Calling this function"]
            #[doc = "sets up a surface to surface relation with the same stacking and positioning"]
            #[doc = "semantics as xdg_surface.set_parent."]
            async fn set_parent_of(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_imported_v1#{}.set_parent_of()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The imported surface handle has been destroyed and any relationship set"]
            #[doc = "up has been invalidated. This may happen for various reasons, for"]
            #[doc = "example if the exported surface or the exported surface handle has been"]
            #[doc = "destroyed, if the handle used for importing was invalid."]
            async fn destroyed(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This description provides a high-level overview of the interplay between"]
#[doc = "the interfaces defined this protocol. For details, see the protocol"]
#[doc = "specification."]
#[doc = ""]
#[doc = "More than one tablet may exist, and device-specifics matter. Tablets are"]
#[doc = "not represented by a single virtual device like wl_pointer. A client"]
#[doc = "binds to the tablet manager object which is just a proxy object. From"]
#[doc = "that, the client requests wp_tablet_manager.get_tablet_seat(wl_seat)"]
#[doc = "and that returns the actual interface that has all the tablets. With"]
#[doc = "this indirection, we can avoid merging wp_tablet into the actual Wayland"]
#[doc = "protocol, a long-term benefit."]
#[doc = ""]
#[doc = "The wp_tablet_seat sends a \"tablet added\" event for each tablet"]
#[doc = "connected. That event is followed by descriptive events about the"]
#[doc = "hardware; currently that includes events for name, vid/pid and"]
#[doc = "a wp_tablet.path event that describes a local path. This path can be"]
#[doc = "used to uniquely identify a tablet or get more information through"]
#[doc = "libwacom. Emulated or nested tablets can skip any of those, e.g. a"]
#[doc = "virtual tablet may not have a vid/pid. The sequence of descriptive"]
#[doc = "events is terminated by a wp_tablet.done event to signal that a client"]
#[doc = "may now finalize any initialization for that tablet."]
#[doc = ""]
#[doc = "Events from tablets require a tool in proximity. Tools are also managed"]
#[doc = "by the tablet seat; a \"tool added\" event is sent whenever a tool is new"]
#[doc = "to the compositor. That event is followed by a number of descriptive"]
#[doc = "events about the hardware; currently that includes capabilities,"]
#[doc = "hardware id and serial number, and tool type. Similar to the tablet"]
#[doc = "interface, a wp_tablet_tool.done event is sent to terminate that initial"]
#[doc = "sequence."]
#[doc = ""]
#[doc = "Any event from a tool happens on the wp_tablet_tool interface. When the"]
#[doc = "tool gets into proximity of the tablet, a proximity_in event is sent on"]
#[doc = "the wp_tablet_tool interface, listing the tablet and the surface. That"]
#[doc = "event is followed by a motion event with the coordinates. After that,"]
#[doc = "it's the usual motion, axis, button, etc. events. The protocol's"]
#[doc = "serialisation means events are grouped by wp_tablet_tool.frame events."]
#[doc = ""]
#[doc = "Two special events (that don't exist in X) are down and up. They signal"]
#[doc = "\"tip touching the surface\". For tablets without real proximity"]
#[doc = "detection, the sequence is: proximity_in, motion, down, frame."]
#[doc = ""]
#[doc = "When the tool leaves proximity, a proximity_out event is sent. If any"]
#[doc = "button is still down, a button release event is sent before this"]
#[doc = "proximity event. These button events are sent in the same frame as the"]
#[doc = "proximity event to signal to the client that the buttons were held when"]
#[doc = "the tool left proximity."]
#[doc = ""]
#[doc = "If the tool moves out of the surface but stays in proximity (i.e."]
#[doc = "between windows), compositor-specific grab policies apply. This usually"]
#[doc = "means that the proximity-out is delayed until all buttons are released."]
#[doc = ""]
#[doc = "Moving a tool physically from one tablet to the other has no real effect"]
#[doc = "on the protocol, since we already have the tool object from the \"tool"]
#[doc = "added\" event. All the information is already there and the proximity"]
#[doc = "events on both tablets are all a client needs to reconstruct what"]
#[doc = "happened."]
#[doc = ""]
#[doc = "Some extra axes are normalized, i.e. the client knows the range as"]
#[doc = "specified in the protocol (e.g. [0, 65535]), the granularity however is"]
#[doc = "unknown. The current normalized axes are pressure, distance, and slider."]
#[doc = ""]
#[doc = "Other extra axes are in physical units as specified in the protocol."]
#[doc = "The current extra axes with physical units are tilt, rotation and"]
#[doc = "wheel rotation."]
#[doc = ""]
#[doc = "Since tablets work independently of the pointer controlled by the mouse,"]
#[doc = "the focus handling is independent too and controlled by proximity."]
#[doc = "The wp_tablet_tool.set_cursor request sets a tool-specific cursor."]
#[doc = "This cursor surface may be the same as the mouse cursor, and it may be"]
#[doc = "the same across tools but it is possible to be more fine-grained. For"]
#[doc = "example, a client may set different cursors for the pen and eraser."]
#[doc = ""]
#[doc = "Tools are generally independent of tablets and it is"]
#[doc = "compositor-specific policy when a tool can be removed. Common approaches"]
#[doc = "will likely include some form of removing a tool when all tablets the"]
#[doc = "tool was used on are removed."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod tablet_unstable_v1 {
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "system. All tablets are associated with a seat, to get access to the"]
    #[doc = "actual tablets, use wp_tablet_manager.get_tablet_seat."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV1 {
            const INTERFACE: &'static str = "zwp_tablet_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Get the wp_tablet_seat object for the given seat. This object"]
            #[doc = "provides access to all graphics tablets in this seat."]
            async fn get_tablet_seat(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                tablet_seat: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_manager_v1#{}.get_tablet_seat()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(tablet_seat))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the wp_tablet_manager object. Objects created from this"]
            #[doc = "object are unaffected and should be destroyed separately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "seat. After binding to this interface, the compositor sends a set of"]
    #[doc = "wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_seat_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_seat_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV1 {
            const INTERFACE: &'static str = "zwp_tablet_seat_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the wp_tablet_seat object. Objects created from this"]
            #[doc = "object are unaffected and should be destroyed separately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_seat_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent whenever a new tablet becomes available on this"]
            #[doc = "seat. This event only provides the object id of the tablet, any"]
            #[doc = "static information about the tablet (device name, vid/pid, etc.) is"]
            #[doc = "sent through the wp_tablet interface."]
            async fn tablet_added(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "This event is sent whenever a tool that has not previously been used"]
            #[doc = "with a tablet comes into use. This event only provides the object id"]
            #[doc = "of the tool; any static information about the tool (capabilities,"]
            #[doc = "type, etc.) is sent through the wp_tablet_tool interface."]
            async fn tool_added(&self, id: crate::wire::ObjectId) -> crate::client::Result<()>;
        }
    }
    #[doc = "An object that represents a physical tool that has been, or is"]
    #[doc = "currently in use with a tablet in this seat. Each wp_tablet_tool"]
    #[doc = "object stays valid until the client destroys it; the compositor"]
    #[doc = "reuses the wp_tablet_tool object to indicate that the object's"]
    #[doc = "respective physical tool has come into proximity of a tablet again."]
    #[doc = ""]
    #[doc = "A wp_tablet_tool object's relation to a physical tool depends on the"]
    #[doc = "tablet's ability to report serial numbers. If the tablet supports"]
    #[doc = "this capability, then the object represents a specific physical tool"]
    #[doc = "and can be identified even when used on multiple tablets."]
    #[doc = ""]
    #[doc = "A tablet tool has a number of static characteristics, e.g. tool type,"]
    #[doc = "hardware_serial and capabilities. These capabilities are sent in an"]
    #[doc = "event sequence after the wp_tablet_seat.tool_added event before any"]
    #[doc = "actual events from this tool. This initial event sequence is"]
    #[doc = "terminated by a wp_tablet_tool.done event."]
    #[doc = ""]
    #[doc = "Tablet tool events are grouped by wp_tablet_tool.frame events."]
    #[doc = "Any events received before a wp_tablet_tool.frame event should be"]
    #[doc = "considered part of the same hardware state change."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_tool_v1 {
        use futures_util::SinkExt;
        #[doc = "Describes the physical type of a tool. The physical type of a tool"]
        #[doc = "generally defines its base usage."]
        #[doc = ""]
        #[doc = "The mouse tool represents a mouse-shaped tool that is not a relative"]
        #[doc = "device but bound to the tablet's surface, providing absolute"]
        #[doc = "coordinates."]
        #[doc = ""]
        #[doc = "The lens tool is a mouse-shaped tool with an attached lens to"]
        #[doc = "provide precision focus."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Type {
            #[doc = "Pen"]
            Pen = 320u32,
            #[doc = "Eraser"]
            Eraser = 321u32,
            #[doc = "Brush"]
            Brush = 322u32,
            #[doc = "Pencil"]
            Pencil = 323u32,
            #[doc = "Airbrush"]
            Airbrush = 324u32,
            #[doc = "Finger"]
            Finger = 325u32,
            #[doc = "Mouse"]
            Mouse = 326u32,
            #[doc = "Lens"]
            Lens = 327u32,
        }
        impl TryFrom<u32> for Type {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    320u32 => Ok(Self::Pen),
                    321u32 => Ok(Self::Eraser),
                    322u32 => Ok(Self::Brush),
                    323u32 => Ok(Self::Pencil),
                    324u32 => Ok(Self::Airbrush),
                    325u32 => Ok(Self::Finger),
                    326u32 => Ok(Self::Mouse),
                    327u32 => Ok(Self::Lens),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Describes extra capabilities on a tablet."]
        #[doc = ""]
        #[doc = "Any tool must provide x and y values, extra axes are"]
        #[doc = "device-specific."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Capability {
            #[doc = "Tilt axes"]
            Tilt = 1u32,
            #[doc = "Pressure axis"]
            Pressure = 2u32,
            #[doc = "Distance axis"]
            Distance = 3u32,
            #[doc = "Z-rotation axis"]
            Rotation = 4u32,
            #[doc = "Slider axis"]
            Slider = 5u32,
            #[doc = "Wheel axis"]
            Wheel = 6u32,
        }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Tilt),
                    2u32 => Ok(Self::Pressure),
                    3u32 => Ok(Self::Distance),
                    4u32 => Ok(Self::Rotation),
                    5u32 => Ok(Self::Slider),
                    6u32 => Ok(Self::Wheel),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Describes the physical state of a button that produced the button event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ButtonState {
            #[doc = "button is not pressed"]
            Released = 0u32,
            #[doc = "button is pressed"]
            Pressed = 1u32,
        }
        impl TryFrom<u32> for ButtonState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Released),
                    1u32 => Ok(Self::Pressed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ButtonState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_tool_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletToolV1 {
            const INTERFACE: &'static str = "zwp_tablet_tool_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Sets the surface of the cursor used for this tool on the given"]
            #[doc = "tablet. This request only takes effect if the tool is in proximity"]
            #[doc = "of one of the requesting client's surfaces or the surface parameter"]
            #[doc = "is the current pointer surface. If there was a previous surface set"]
            #[doc = "with this request it is replaced. If surface is NULL, the cursor"]
            #[doc = "image is hidden."]
            #[doc = ""]
            #[doc = "The parameters hotspot_x and hotspot_y define the position of the"]
            #[doc = "pointer surface relative to the pointer location. Its top-left corner"]
            #[doc = "is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the"]
            #[doc = "coordinates of the pointer location, in surface-local coordinates."]
            #[doc = ""]
            #[doc = "On surface.attach requests to the pointer surface, hotspot_x and"]
            #[doc = "hotspot_y are decremented by the x and y parameters passed to the"]
            #[doc = "request. Attach must be confirmed by wl_surface.commit as usual."]
            #[doc = ""]
            #[doc = "The hotspot can also be updated by passing the currently set pointer"]
            #[doc = "surface to this request with new values for hotspot_x and hotspot_y."]
            #[doc = ""]
            #[doc = "The current and pending input regions of the wl_surface are cleared,"]
            #[doc = "and wl_surface.set_input_region is ignored until the wl_surface is no"]
            #[doc = "longer used as the cursor. When the use as a cursor ends, the current"]
            #[doc = "and pending input regions become undefined, and the wl_surface is"]
            #[doc = "unmapped."]
            #[doc = ""]
            #[doc = "This request gives the surface the role of a cursor. The role"]
            #[doc = "assigned by this request is the same as assigned by"]
            #[doc = "wl_pointer.set_cursor meaning the same surface can be"]
            #[doc = "used both as a wl_pointer cursor and a wp_tablet cursor. If the"]
            #[doc = "surface already has another role, it raises a protocol error."]
            #[doc = "The surface may be used on multiple tablets and across multiple"]
            #[doc = "seats."]
            async fn set_cursor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                surface: Option<crate::wire::ObjectId>,
                hotspot_x: i32,
                hotspot_y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_tool_v1#{}.set_cursor()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(surface)
                    .put_int(hotspot_x)
                    .put_int(hotspot_y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This destroys the client's resource for this tool object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_tool_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The tool type is the high-level type of the tool and usually decides"]
            #[doc = "the interaction expected from this tool."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn r#type(&self, tool_type: Type) -> crate::client::Result<()>;
            #[doc = "If the physical tool can be identified by a unique 64-bit serial"]
            #[doc = "number, this event notifies the client of this serial number."]
            #[doc = ""]
            #[doc = "If multiple tablets are available in the same seat and the tool is"]
            #[doc = "uniquely identifiable by the serial number, that tool may move"]
            #[doc = "between tablets."]
            #[doc = ""]
            #[doc = "Otherwise, if the tool has no serial number and this event is"]
            #[doc = "missing, the tool is tied to the tablet it first comes into"]
            #[doc = "proximity with. Even if the physical tool is used on multiple"]
            #[doc = "tablets, separate wp_tablet_tool objects will be created, one per"]
            #[doc = "tablet."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn hardware_serial(
                &self,
                hardware_serial_hi: u32,
                hardware_serial_lo: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event notifies the client of a hardware id available on this tool."]
            #[doc = ""]
            #[doc = "The hardware id is a device-specific 64-bit id that provides extra"]
            #[doc = "information about the tool in use, beyond the wl_tool.type"]
            #[doc = "enumeration. The format of the id is specific to tablets made by"]
            #[doc = "Wacom Inc. For example, the hardware id of a Wacom Grip"]
            #[doc = "Pen (a stylus) is 0x802."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn hardware_id_wacom(
                &self,
                hardware_id_hi: u32,
                hardware_id_lo: u32,
            ) -> crate::client::Result<()>;
            #[doc = "This event notifies the client of any capabilities of this tool,"]
            #[doc = "beyond the main set of x/y axes and tip up/down detection."]
            #[doc = ""]
            #[doc = "One event is sent for each extra capability available on this tool."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            async fn capability(&self, capability: Capability) -> crate::client::Result<()>;
            #[doc = "This event signals the end of the initial burst of descriptive"]
            #[doc = "events. A client may consider the static description of the tool to"]
            #[doc = "be complete and finalize initialization of the tool."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "This event is sent when the tool is removed from the system and will"]
            #[doc = "send no further events. Should the physical tool come back into"]
            #[doc = "proximity later, a new wp_tablet_tool object will be created."]
            #[doc = ""]
            #[doc = "It is compositor-dependent when a tool is removed. A compositor may"]
            #[doc = "remove a tool on proximity out, tablet removal or any other reason."]
            #[doc = "A compositor may also keep a tool alive until shutdown."]
            #[doc = ""]
            #[doc = "If the tool is currently in proximity, a proximity_out event will be"]
            #[doc = "sent before the removed event. See wp_tablet_tool.proximity_out for"]
            #[doc = "the handling of any buttons logically down."]
            #[doc = ""]
            #[doc = "When this event is received, the client must wp_tablet_tool.destroy"]
            #[doc = "the object."]
            async fn removed(&self) -> crate::client::Result<()>;
            #[doc = "Notification that this tool is focused on a certain surface."]
            #[doc = ""]
            #[doc = "This event can be received when the tool has moved from one surface to"]
            #[doc = "another, or when the tool has come back into proximity above the"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "If any button is logically down when the tool comes into proximity,"]
            #[doc = "the respective button event is sent after the proximity_in event but"]
            #[doc = "within the same frame as the proximity_in event."]
            async fn proximity_in(
                &self,
                serial: u32,
                tablet: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = "Notification that this tool has either left proximity, or is no"]
            #[doc = "longer focused on a certain surface."]
            #[doc = ""]
            #[doc = "When the tablet tool leaves proximity of the tablet, button release"]
            #[doc = "events are sent for each button that was held down at the time of"]
            #[doc = "leaving proximity. These events are sent before the proximity_out"]
            #[doc = "event but within the same wp_tablet.frame."]
            #[doc = ""]
            #[doc = "If the tool stays within proximity of the tablet, but the focus"]
            #[doc = "changes from one surface to another, a button release event may not"]
            #[doc = "be sent until the button is actually released or the tool leaves the"]
            #[doc = "proximity of the tablet."]
            async fn proximity_out(&self) -> crate::client::Result<()>;
            #[doc = "Sent whenever the tablet tool comes in contact with the surface of the"]
            #[doc = "tablet."]
            #[doc = ""]
            #[doc = "If the tool is already in contact with the tablet when entering the"]
            #[doc = "input region, the client owning said region will receive a"]
            #[doc = "wp_tablet.proximity_in event, followed by a wp_tablet.down"]
            #[doc = "event and a wp_tablet.frame event."]
            #[doc = ""]
            #[doc = "Note that this event describes logical contact, not physical"]
            #[doc = "contact. On some devices, a compositor may not consider a tool in"]
            #[doc = "logical contact until a minimum physical pressure threshold is"]
            #[doc = "exceeded."]
            async fn down(&self, serial: u32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the tablet tool stops making contact with the surface of"]
            #[doc = "the tablet, or when the tablet tool moves out of the input region"]
            #[doc = "and the compositor grab (if any) is dismissed."]
            #[doc = ""]
            #[doc = "If the tablet tool moves out of the input region while in contact"]
            #[doc = "with the surface of the tablet and the compositor does not have an"]
            #[doc = "ongoing grab on the surface, the client owning said region will"]
            #[doc = "receive a wp_tablet.up event, followed by a wp_tablet.proximity_out"]
            #[doc = "event and a wp_tablet.frame event. If the compositor has an ongoing"]
            #[doc = "grab on this device, this event sequence is sent whenever the grab"]
            #[doc = "is dismissed in the future."]
            #[doc = ""]
            #[doc = "Note that this event describes logical contact, not physical"]
            #[doc = "contact. On some devices, a compositor may not consider a tool out"]
            #[doc = "of logical contact until physical pressure falls below a specific"]
            #[doc = "threshold."]
            async fn up(&self) -> crate::client::Result<()>;
            #[doc = "Sent whenever a tablet tool moves."]
            async fn motion(
                &self,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::client::Result<()>;
            #[doc = "Sent whenever the pressure axis on a tool changes. The value of this"]
            #[doc = "event is normalized to a value between 0 and 65535."]
            #[doc = ""]
            #[doc = "Note that pressure may be nonzero even when a tool is not in logical"]
            #[doc = "contact. See the down and up events for more details."]
            async fn pressure(&self, pressure: u32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the distance axis on a tool changes. The value of this"]
            #[doc = "event is normalized to a value between 0 and 65535."]
            #[doc = ""]
            #[doc = "Note that distance may be nonzero even when a tool is not in logical"]
            #[doc = "contact. See the down and up events for more details."]
            async fn distance(&self, distance: u32) -> crate::client::Result<()>;
            #[doc = "Sent whenever one or both of the tilt axes on a tool change. Each tilt"]
            #[doc = "value is in 0.01 of a degree, relative to the z-axis of the tablet."]
            #[doc = "The angle is positive when the top of a tool tilts along the"]
            #[doc = "positive x or y axis."]
            async fn tilt(&self, tilt_x: i32, tilt_y: i32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the z-rotation axis on the tool changes. The"]
            #[doc = "rotation value is in 0.01 of a degree clockwise from the tool's"]
            #[doc = "logical neutral position."]
            async fn rotation(&self, degrees: i32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the slider position on the tool changes. The"]
            #[doc = "value is normalized between -65535 and 65535, with 0 as the logical"]
            #[doc = "neutral position of the slider."]
            #[doc = ""]
            #[doc = "The slider is available on e.g. the Wacom Airbrush tool."]
            async fn slider(&self, position: i32) -> crate::client::Result<()>;
            #[doc = "Sent whenever the wheel on the tool emits an event. This event"]
            #[doc = "contains two values for the same axis change. The degrees value is"]
            #[doc = "in 0.01 of a degree in the same orientation as the"]
            #[doc = "wl_pointer.vertical_scroll axis. The clicks value is in discrete"]
            #[doc = "logical clicks of the mouse wheel. This value may be zero if the"]
            #[doc = "movement of the wheel was less than one logical click."]
            #[doc = ""]
            #[doc = "Clients should choose either value and avoid mixing degrees and"]
            #[doc = "clicks. The compositor may accumulate values smaller than a logical"]
            #[doc = "click and emulate click events when a certain threshold is met."]
            #[doc = "Thus, wl_tablet_tool.wheel events with non-zero clicks values may"]
            #[doc = "have different degrees values."]
            async fn wheel(&self, degrees: i32, clicks: i32) -> crate::client::Result<()>;
            #[doc = "Sent whenever a button on the tool is pressed or released."]
            #[doc = ""]
            #[doc = "If a button is held down when the tool moves in or out of proximity,"]
            #[doc = "button events are generated by the compositor. See"]
            #[doc = "wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for"]
            #[doc = "details."]
            async fn button(
                &self,
                serial: u32,
                button: u32,
                state: ButtonState,
            ) -> crate::client::Result<()>;
            #[doc = "Marks the end of a series of axis and/or button updates from the"]
            #[doc = "tablet. The Wayland protocol requires axis updates to be sent"]
            #[doc = "sequentially, however all events within a frame should be considered"]
            #[doc = "one hardware event."]
            async fn frame(&self, time: u32) -> crate::client::Result<()>;
        }
    }
    #[doc = "The wp_tablet interface represents one graphics tablet device. The"]
    #[doc = "tablet interface itself does not generate events; all events are"]
    #[doc = "generated by wp_tablet_tool objects when in proximity above a tablet."]
    #[doc = ""]
    #[doc = "A tablet has a number of static characteristics, e.g. device name and"]
    #[doc = "pid/vid. These capabilities are sent in an event sequence after the"]
    #[doc = "wp_tablet_seat.tablet_added event. This initial event sequence is"]
    #[doc = "terminated by a wp_tablet.done event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV1 {
            const INTERFACE: &'static str = "zwp_tablet_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This destroys the client's resource for this tablet object."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_tablet_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            async fn name(&self, name: String) -> crate::client::Result<()>;
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            async fn id(&self, vid: u32, pid: u32) -> crate::client::Result<()>;
            #[doc = "A system-specific device path that indicates which device is behind"]
            #[doc = "this wp_tablet. This information may be used to gather additional"]
            #[doc = "information about the device, e.g. through libwacom."]
            #[doc = ""]
            #[doc = "A device may have more than one device path. If so, multiple"]
            #[doc = "wp_tablet.path events are sent. A device may be emulated and not"]
            #[doc = "have a device path, and in that case this event will not be sent."]
            #[doc = ""]
            #[doc = "The format of the path is unspecified, it may be a device node, a"]
            #[doc = "sysfs path, or some other identifier. It is up to the client to"]
            #[doc = "identify the string provided."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            async fn path(&self, path: String) -> crate::client::Result<()>;
            #[doc = "This event is sent immediately to signal the end of the initial"]
            #[doc = "burst of descriptive events. A client may consider the static"]
            #[doc = "description of the tablet to be complete and finalize initialization"]
            #[doc = "of the tablet."]
            async fn done(&self) -> crate::client::Result<()>;
            #[doc = "Sent when the tablet has been removed from the system. When a tablet"]
            #[doc = "is removed, some tools may be removed."]
            #[doc = ""]
            #[doc = "When this event is received, the client must wp_tablet.destroy"]
            #[doc = "the object."]
            async fn removed(&self) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol specifies a way for a client to request and receive"]
#[doc = "high-resolution timestamps for input events."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod input_timestamps_unstable_v1 {
    #[doc = "A global interface used for requesting high-resolution timestamps"]
    #[doc = "for input events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_input_timestamps_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_input_timestamps_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputTimestampsManagerV1 {
            const INTERFACE: &'static str = "zwp_input_timestamps_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will no longer be using this"]
            #[doc = "protocol object. Existing objects created by this object are not"]
            #[doc = "affected."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_timestamps_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Creates a new input timestamps object that represents a subscription"]
            #[doc = "to high-resolution timestamp events for all wl_keyboard events that"]
            #[doc = "carry a timestamp."]
            #[doc = ""]
            #[doc = "If the associated wl_keyboard object is invalidated, either through"]
            #[doc = "client action (e.g. release) or server-side changes, the input"]
            #[doc = "timestamps object becomes inert and the client should destroy it"]
            #[doc = "by calling zwp_input_timestamps_v1.destroy."]
            async fn get_keyboard_timestamps(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                keyboard: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_timestamps_manager_v1#{}.get_keyboard_timestamps()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(keyboard))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Creates a new input timestamps object that represents a subscription"]
            #[doc = "to high-resolution timestamp events for all wl_pointer events that"]
            #[doc = "carry a timestamp."]
            #[doc = ""]
            #[doc = "If the associated wl_pointer object is invalidated, either through"]
            #[doc = "client action (e.g. release) or server-side changes, the input"]
            #[doc = "timestamps object becomes inert and the client should destroy it"]
            #[doc = "by calling zwp_input_timestamps_v1.destroy."]
            async fn get_pointer_timestamps(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_timestamps_manager_v1#{}.get_pointer_timestamps()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(pointer))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Creates a new input timestamps object that represents a subscription"]
            #[doc = "to high-resolution timestamp events for all wl_touch events that"]
            #[doc = "carry a timestamp."]
            #[doc = ""]
            #[doc = "If the associated wl_touch object becomes invalid, either through"]
            #[doc = "client action (e.g. release) or server-side changes, the input"]
            #[doc = "timestamps object becomes inert and the client should destroy it"]
            #[doc = "by calling zwp_input_timestamps_v1.destroy."]
            async fn get_touch_timestamps(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                touch: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_input_timestamps_manager_v1#{}.get_touch_timestamps()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(touch))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "Provides high-resolution timestamp events for a set of subscribed input"]
    #[doc = "events. The set of subscribed input events is determined by the"]
    #[doc = "zwp_input_timestamps_manager_v1 request used to create this object."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_input_timestamps_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_input_timestamps_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputTimestampsV1 {
            const INTERFACE: &'static str = "zwp_input_timestamps_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will no longer be using this"]
            #[doc = "protocol object. After the server processes the request, no more"]
            #[doc = "timestamp events will be emitted."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_input_timestamps_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The timestamp event is associated with the first subsequent input event"]
            #[doc = "carrying a timestamp which belongs to the set of input events this"]
            #[doc = "object is subscribed to."]
            #[doc = ""]
            #[doc = "The timestamp provided by this event is a high-resolution version of"]
            #[doc = "the timestamp argument of the associated input event. The provided"]
            #[doc = "timestamp is in the same clock domain and is at least as accurate as"]
            #[doc = "the associated input event timestamp."]
            #[doc = ""]
            #[doc = "The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,"]
            #[doc = "each component being an unsigned 32-bit value. Whole seconds are in"]
            #[doc = "tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,"]
            #[doc = "and the additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]."]
            async fn timestamp(
                &self,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = "This protocol provides the ability to have a primary selection device to"]
#[doc = "match that of the X server. This primary selection is a shortcut to the"]
#[doc = "common clipboard selection, where text just needs to be selected in order"]
#[doc = "to allow copying it elsewhere. The de facto way to perform this action"]
#[doc = "is the middle mouse button, although it is not limited to this one."]
#[doc = ""]
#[doc = "Clients wishing to honor primary selection should create a primary"]
#[doc = "selection source and set it as the selection through"]
#[doc = "wp_primary_selection_device.set_selection whenever the text selection"]
#[doc = "changes. In order to minimize calls in pointer-driven text selection,"]
#[doc = "it should happen only once after the operation finished. Similarly,"]
#[doc = "a NULL source should be set when text is unselected."]
#[doc = ""]
#[doc = "wp_primary_selection_offer objects are first announced through the"]
#[doc = "wp_primary_selection_device.data_offer event. Immediately after this event,"]
#[doc = "the primary data offer will emit wp_primary_selection_offer.offer events"]
#[doc = "to let know of the mime types being offered."]
#[doc = ""]
#[doc = "When the primary selection changes, the client with the keyboard focus"]
#[doc = "will receive wp_primary_selection_device.selection events. Only the client"]
#[doc = "with the keyboard focus will receive such events with a non-NULL"]
#[doc = "wp_primary_selection_offer. Across keyboard focus changes, previously"]
#[doc = "focused clients will receive wp_primary_selection_device.events with a"]
#[doc = "NULL wp_primary_selection_offer."]
#[doc = ""]
#[doc = "In order to request the primary selection data, the client must pass"]
#[doc = "a recent serial pertaining to the press event that is triggering the"]
#[doc = "operation, if the compositor deems the serial valid and recent, the"]
#[doc = "wp_primary_selection_source.send event will happen in the other end"]
#[doc = "to let the transfer begin. The client owning the primary selection"]
#[doc = "should write the requested data, and close the file descriptor"]
#[doc = "immediately."]
#[doc = ""]
#[doc = "If the primary selection owner client disappeared during the transfer,"]
#[doc = "the client reading the data will receive a"]
#[doc = "wp_primary_selection_device.selection event with a NULL"]
#[doc = "wp_primary_selection_offer, the client should take this as a hint"]
#[doc = "to finish the reads related to the no longer existing offer."]
#[doc = ""]
#[doc = "The primary selection owner should be checking for errors during"]
#[doc = "writes, merely cancelling the ongoing transfer if any happened."]
#[allow(clippy::module_inception)]
pub mod wp_primary_selection_unstable_v1 {
    #[doc = "The primary selection device manager is a singleton global object that"]
    #[doc = "provides access to the primary selection. It allows to create"]
    #[doc = "wp_primary_selection_source objects, as well as retrieving the per-seat"]
    #[doc = "wp_primary_selection_device objects."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_primary_selection_device_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_primary_selection_device_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionDeviceManagerV1 {
            const INTERFACE: &'static str = "zwp_primary_selection_device_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a new primary selection source."]
            async fn create_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_primary_selection_device_manager_v1#{}.create_source()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a new data device for a given seat."]
            async fn get_device(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_primary_selection_device_manager_v1#{}.get_device()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(seat))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the primary selection device manager."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_primary_selection_device_manager_v1#{}.destroy()",
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
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_primary_selection_device_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_primary_selection_device_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionDeviceV1 {
            const INTERFACE: &'static str = "zwp_primary_selection_device_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Replaces the current selection. The previous owner of the primary"]
            #[doc = "selection will receive a wp_primary_selection_source.cancelled event."]
            #[doc = ""]
            #[doc = "To unset the selection, set the source to NULL."]
            async fn set_selection(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zwp_primary_selection_device_v1#{}.set_selection()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the primary selection device."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_primary_selection_device_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Introduces a new wp_primary_selection_offer object that may be used"]
            #[doc = "to receive the current primary selection. Immediately following this"]
            #[doc = "event, the new wp_primary_selection_offer object will send"]
            #[doc = "wp_primary_selection_offer.offer events to describe the offered mime"]
            #[doc = "types."]
            async fn data_offer(&self, offer: crate::wire::ObjectId) -> crate::client::Result<()>;
            #[doc = "The wp_primary_selection_device.selection event is sent to notify the"]
            #[doc = "client of a new primary selection. This event is sent after the"]
            #[doc = "wp_primary_selection.data_offer event introducing this object, and after"]
            #[doc = "the offer has announced its mimetypes through"]
            #[doc = "wp_primary_selection_offer.offer."]
            #[doc = ""]
            #[doc = "The data_offer is valid until a new offer or NULL is received"]
            #[doc = "or until the client loses keyboard focus. The client must destroy the"]
            #[doc = "previous selection data_offer, if any, upon receiving this event."]
            async fn selection(
                &self,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = "A wp_primary_selection_offer represents an offer to transfer the contents"]
    #[doc = "of the primary selection clipboard to the client. Similar to"]
    #[doc = "wl_data_offer, the offer also describes the mime types that the data can"]
    #[doc = "be converted to and provides the mechanisms for transferring the data"]
    #[doc = "directly to the client."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_primary_selection_offer_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_primary_selection_offer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionOfferV1 {
            const INTERFACE: &'static str = "zwp_primary_selection_offer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "To transfer the contents of the primary selection clipboard, the client"]
            #[doc = "issues this request and indicates the mime type that it wants to"]
            #[doc = "receive. The transfer happens through the passed file descriptor"]
            #[doc = "(typically created with the pipe system call). The source client writes"]
            #[doc = "the data in the mime type representation requested and then closes the"]
            #[doc = "file descriptor."]
            #[doc = ""]
            #[doc = "The receiving client reads from the read end of the pipe until EOF and"]
            #[doc = "closes its end, at which point the transfer is complete."]
            async fn receive(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_primary_selection_offer_v1#{}.receive()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .put_fd(fd)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the primary selection offer."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_primary_selection_offer_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sent immediately after creating announcing the"]
            #[doc = "wp_primary_selection_offer through"]
            #[doc = "wp_primary_selection_device.data_offer. One event is sent per offered"]
            #[doc = "mime type."]
            async fn offer(&self, mime_type: String) -> crate::client::Result<()>;
        }
    }
    #[doc = "The source side of a wp_primary_selection_offer, it provides a way to"]
    #[doc = "describe the offered data and respond to requests to transfer the"]
    #[doc = "requested contents of the primary selection clipboard."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_primary_selection_source_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_primary_selection_source_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionSourceV1 {
            const INTERFACE: &'static str = "zwp_primary_selection_source_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request adds a mime type to the set of mime types advertised to"]
            #[doc = "targets. Can be called several times to offer multiple types."]
            async fn offer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_primary_selection_source_v1#{}.offer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the primary selection source."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zwp_primary_selection_source_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request for the current primary selection contents from the client."]
            #[doc = "Send the specified mime type over the passed file descriptor, then"]
            #[doc = "close it."]
            async fn send(
                &self,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()>;
            #[doc = "This primary selection source is no longer valid. The client should"]
            #[doc = "clean up and destroy this primary selection source."]
            async fn cancelled(&self) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_decoration_unstable_v1 {
    #[doc = "This interface allows a compositor to announce support for server-side"]
    #[doc = "decorations."]
    #[doc = ""]
    #[doc = "A window decoration is a set of window controls as deemed appropriate by"]
    #[doc = "the party managing them, such as user interface components used to move,"]
    #[doc = "resize and change a window's state."]
    #[doc = ""]
    #[doc = "A client can use this protocol to request being decorated by a supporting"]
    #[doc = "compositor."]
    #[doc = ""]
    #[doc = "If compositor and client do not negotiate the use of a server-side"]
    #[doc = "decoration using this protocol, clients continue to self-decorate as they"]
    #[doc = "see fit."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is experimental and"]
    #[doc = "backward incompatible changes may be made. Backward compatible changes"]
    #[doc = "may be added together with the corresponding interface version bump."]
    #[doc = "Backward incompatible changes are done by bumping the version number in"]
    #[doc = "the protocol and interface names and resetting the interface version."]
    #[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
    #[doc = "version number in the protocol and interface names are removed and the"]
    #[doc = "interface version number is reset."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_decoration_manager_v1 {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zxdg_decoration_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgDecorationManagerV1 {
            const INTERFACE: &'static str = "zxdg_decoration_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the decoration manager. This doesn't destroy objects created"]
            #[doc = "with the manager."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_decoration_manager_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a new decoration object associated with the given toplevel."]
            #[doc = ""]
            #[doc = "Creating an xdg_toplevel_decoration from an xdg_toplevel which has a"]
            #[doc = "buffer attached or committed is a client error, and any attempts by a"]
            #[doc = "client to attach or manipulate a buffer prior to the first"]
            #[doc = "xdg_toplevel_decoration.configure event must also be treated as"]
            #[doc = "errors."]
            async fn get_toplevel_decoration(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> zxdg_decoration_manager_v1#{}.get_toplevel_decoration()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(toplevel))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The decoration object allows the compositor to toggle server-side window"]
    #[doc = "decorations for a toplevel surface. The client can request to switch to"]
    #[doc = "another mode."]
    #[doc = ""]
    #[doc = "The xdg_toplevel_decoration object must be destroyed before its"]
    #[doc = "xdg_toplevel."]
    #[allow(clippy::too_many_arguments)]
    pub mod zxdg_toplevel_decoration_v1 {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "xdg_toplevel has a buffer attached before configure"]
            UnconfiguredBuffer = 0u32,
            #[doc = "xdg_toplevel already has a decoration object"]
            AlreadyConstructed = 1u32,
            #[doc = "xdg_toplevel destroyed before the decoration object"]
            Orphaned = 2u32,
            #[doc = "invalid mode"]
            InvalidMode = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::UnconfiguredBuffer),
                    1u32 => Ok(Self::AlreadyConstructed),
                    2u32 => Ok(Self::Orphaned),
                    3u32 => Ok(Self::InvalidMode),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These values describe window decoration modes."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "no server-side window decoration"]
            ClientSide = 1u32,
            #[doc = "server-side window decoration"]
            ServerSide = 2u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::ClientSide),
                    2u32 => Ok(Self::ServerSide),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zxdg_toplevel_decoration_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgToplevelDecorationV1 {
            const INTERFACE: &'static str = "zxdg_toplevel_decoration_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Switch back to a mode without any server-side decorations at the next"]
            #[doc = "commit."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_decoration_v1#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the toplevel surface decoration mode. This informs the compositor"]
            #[doc = "that the client prefers the provided decoration mode."]
            #[doc = ""]
            #[doc = "After requesting a decoration mode, the compositor will respond by"]
            #[doc = "emitting an xdg_surface.configure event. The client should then update"]
            #[doc = "its content, drawing it without decorations if the received mode is"]
            #[doc = "server-side decorations. The client must also acknowledge the configure"]
            #[doc = "when committing the new content (see xdg_surface.ack_configure)."]
            #[doc = ""]
            #[doc = "The compositor can decide not to use the client's mode and enforce a"]
            #[doc = "different mode instead."]
            #[doc = ""]
            #[doc = "Clients whose decoration mode depend on the xdg_toplevel state may send"]
            #[doc = "a set_mode request in response to an xdg_surface.configure event and wait"]
            #[doc = "for the next xdg_surface.configure event to prevent unwanted state."]
            #[doc = "Such clients are responsible for preventing configure loops and must"]
            #[doc = "make sure not to send multiple successive set_mode requests with the"]
            #[doc = "same decoration mode."]
            #[doc = ""]
            #[doc = "If an invalid mode is supplied by the client, the invalid_mode protocol"]
            #[doc = "error is raised by the compositor."]
            async fn set_mode(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mode: Mode,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_decoration_v1#{}.set_mode()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(mode as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Unset the toplevel surface decoration mode. This informs the compositor"]
            #[doc = "that the client doesn't prefer a particular decoration mode."]
            #[doc = ""]
            #[doc = "This request has the same semantics as set_mode."]
            async fn unset_mode(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> zxdg_toplevel_decoration_v1#{}.unset_mode()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The configure event configures the effective decoration mode. The"]
            #[doc = "configured state should not be applied immediately. Clients must send an"]
            #[doc = "ack_configure in response to this event. See xdg_surface.configure and"]
            #[doc = "xdg_surface.ack_configure for details."]
            #[doc = ""]
            #[doc = "A configure event can be sent at any time. The specified mode must be"]
            #[doc = "obeyed by the client."]
            async fn configure(&self, mode: Mode) -> crate::client::Result<()>;
        }
    }
}
