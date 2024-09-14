#![allow(async_fn_in_trait)]
pub mod linux_dmabuf_v1 {
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
    pub mod zwp_linux_dmabuf_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_linux_dmabuf_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufV1 {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_v1";
            const VERSION: u32 = 5u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
    pub mod zwp_linux_buffer_params_v1 {
        #[allow(unused)]
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
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInvert = 1u32 ; # [doc = "content is interlaced"] const Interlaced = 2u32 ; # [doc = "bottom field first"] const BottomFirst = 4u32 ; } }
        impl TryFrom<u32> for Flags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
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
    pub mod zwp_linux_dmabuf_feedback_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct TrancheFlags : u32 { # [doc = "direct scan-out tranche"] const Scanout = 1u32 ; } }
        impl TryFrom<u32> for TrancheFlags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
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
        }
    }
}
pub mod presentation_time {
    #[doc = "The main feature of this interface is accurate presentation"]
    #[doc = "timing feedback to ensure smooth video playback while maintaining"]
    #[doc = "audio/video synchronization. Some features use the concept of a"]
    #[doc = "presentation clock, which is defined in the"]
    #[doc = "presentation.clock_id event."]
    #[doc = ""]
    #[doc = "A content update for a wl_surface is submitted by a"]
    #[doc = "wl_surface.commit request. Request 'feedback' associates with"]
    #[doc = "the wl_surface.commit and provides feedback on the content"]
    #[doc = "update, particularly the final realized presentation time."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "When the final realized presentation time is available, e.g."]
    #[doc = "after a framebuffer flip completes, the requested"]
    #[doc = "presentation_feedback.presented events are sent. The final"]
    #[doc = "presentation time can differ from the compositor's predicted"]
    #[doc = "display update time and the update's target time, especially"]
    #[doc = "when the compositor misses its target vertical blanking period."]
    pub mod wp_presentation {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[doc = "These fatal protocol errors may be emitted in response to"]
        #[doc = "illegal presentation requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid value in tv_nsec"]
            InvalidTimestamp = 0u32,
            #[doc = "invalid flag"]
            InvalidFlag = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidTimestamp),
                    1u32 => Ok(Self::InvalidFlag),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_presentation interface. See the module level documentation for more info"]
        pub trait WpPresentation {
            const INTERFACE: &'static str = "wp_presentation";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object. Existing objects created by this object"]
            #[doc = "are not affected."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_presentation#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Request presentation feedback for the current content submission"]
            #[doc = "on the given surface. This creates a new presentation_feedback"]
            #[doc = "object, which will deliver the feedback information once. If"]
            #[doc = "multiple presentation_feedback objects are created for the same"]
            #[doc = "submission, they will all deliver the same information."]
            #[doc = ""]
            #[doc = "For details on what information is returned, see the"]
            #[doc = "presentation_feedback interface."]
            async fn feedback(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                callback: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_presentation#{}.feedback()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .put_object(Some(callback))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A presentation_feedback object returns an indication that a"]
    #[doc = "wl_surface content update has become visible to the user."]
    #[doc = "One object corresponds to one content update submission"]
    #[doc = "(wl_surface.commit). There are two possible outcomes: the"]
    #[doc = "content update is presented to the user, and a presentation"]
    #[doc = "timestamp delivered; or, the user did not see the content"]
    #[doc = "update because it was superseded or its surface destroyed,"]
    #[doc = "and the content update is discarded."]
    #[doc = ""]
    #[doc = "Once a presentation_feedback object has delivered a 'presented'"]
    #[doc = "or 'discarded' event it is automatically destroyed."]
    pub mod wp_presentation_feedback {
        #[allow(unused)]
        use futures_util::SinkExt;
        bitflags::bitflags! { # [doc = "These flags provide information about how the presentation of"] # [doc = "the related content update was done. The intent is to help"] # [doc = "clients assess the reliability of the feedback and the visual"] # [doc = "quality with respect to possible tearing and timings."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Kind : u32 { const Vsync = 1u32 ; const HwClock = 2u32 ; const HwCompletion = 4u32 ; const ZeroCopy = 8u32 ; } }
        impl TryFrom<u32> for Kind {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the wp_presentation_feedback interface. See the module level documentation for more info"]
        pub trait WpPresentationFeedback {
            const INTERFACE: &'static str = "wp_presentation_feedback";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
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
pub mod tablet_v2 {
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "system. All tablets are associated with a seat, to get access to the"]
    #[doc = "actual tablets, use wp_tablet_manager.get_tablet_seat."]
    pub mod zwp_tablet_manager_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV2 {
            const INTERFACE: &'static str = "zwp_tablet_manager_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
    pub mod zwp_tablet_seat_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_seat_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV2 {
            const INTERFACE: &'static str = "zwp_tablet_seat_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
    pub mod zwp_tablet_tool_v2 {
        #[allow(unused)]
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
        #[doc = "Trait to implement the zwp_tablet_tool_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletToolV2 {
            const INTERFACE: &'static str = "zwp_tablet_tool_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
    pub mod zwp_tablet_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV2 {
            const INTERFACE: &'static str = "zwp_tablet_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
        }
    }
    #[doc = "A circular interaction area, such as the touch ring on the Wacom Intuos"]
    #[doc = "Pro series tablets."]
    #[doc = ""]
    #[doc = "Events on a ring are logically grouped by the wl_tablet_pad_ring.frame"]
    #[doc = "event."]
    pub mod zwp_tablet_pad_ring_v2 {
        #[allow(unused)]
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
        #[doc = "Trait to implement the zwp_tablet_pad_ring_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadRingV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
        }
    }
    #[doc = "A linear interaction area, such as the strips found in Wacom Cintiq"]
    #[doc = "models."]
    #[doc = ""]
    #[doc = "Events on a strip are logically grouped by the wl_tablet_pad_strip.frame"]
    #[doc = "event."]
    pub mod zwp_tablet_pad_strip_v2 {
        #[allow(unused)]
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
        #[doc = "Trait to implement the zwp_tablet_pad_strip_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadStripV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
    pub mod zwp_tablet_pad_group_v2 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[doc = "Trait to implement the zwp_tablet_pad_group_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadGroupV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
    pub mod zwp_tablet_pad_v2 {
        #[allow(unused)]
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
        #[doc = "Trait to implement the zwp_tablet_pad_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_v2";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
        }
    }
}
pub mod viewporter {
    #[doc = "The global interface exposing surface cropping and scaling"]
    #[doc = "capabilities is used to instantiate an interface extension for a"]
    #[doc = "wl_surface object. This extended interface will then allow"]
    #[doc = "cropping and scaling the surface contents, effectively"]
    #[doc = "disconnecting the direct relationship between the buffer and the"]
    #[doc = "surface size."]
    pub mod wp_viewporter {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a viewport object associated"]
            ViewportExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::ViewportExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_viewporter interface. See the module level documentation for more info"]
        pub trait WpViewporter {
            const INTERFACE: &'static str = "wp_viewporter";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other objects,"]
            #[doc = "wp_viewport objects included."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_viewporter#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Instantiate an interface extension for the given wl_surface to"]
            #[doc = "crop and scale its content. If the given wl_surface already has"]
            #[doc = "a wp_viewport object associated, the viewport_exists"]
            #[doc = "protocol error is raised."]
            async fn get_viewport(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_viewporter#{}.get_viewport()", object_id);
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
    #[doc = "An additional interface to a wl_surface object, which allows the"]
    #[doc = "client to specify the cropping and scaling of the surface"]
    #[doc = "contents."]
    #[doc = ""]
    #[doc = "This interface works with two concepts: the source rectangle (src_x,"]
    #[doc = "src_y, src_width, src_height), and the destination size (dst_width,"]
    #[doc = "dst_height). The contents of the source rectangle are scaled to the"]
    #[doc = "destination size, and content outside the source rectangle is ignored."]
    #[doc = "This state is double-buffered, see wl_surface.commit."]
    #[doc = ""]
    #[doc = "The two parts of crop and scale state are independent: the source"]
    #[doc = "rectangle, and the destination size. Initially both are unset, that"]
    #[doc = "is, no scaling is applied. The whole of the current wl_buffer is"]
    #[doc = "used as the source, and the surface size is as defined in"]
    #[doc = "wl_surface.attach."]
    #[doc = ""]
    #[doc = "If the destination size is set, it causes the surface size to become"]
    #[doc = "dst_width, dst_height. The source (rectangle) is scaled to exactly"]
    #[doc = "this size. This overrides whatever the attached wl_buffer size is,"]
    #[doc = "unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface"]
    #[doc = "has no content and therefore no size. Otherwise, the size is always"]
    #[doc = "at least 1x1 in surface local coordinates."]
    #[doc = ""]
    #[doc = "If the source rectangle is set, it defines what area of the wl_buffer is"]
    #[doc = "taken as the source. If the source rectangle is set and the destination"]
    #[doc = "size is not set, then src_width and src_height must be integers, and the"]
    #[doc = "surface size becomes the source rectangle size. This results in cropping"]
    #[doc = "without scaling. If src_width or src_height are not integers and"]
    #[doc = "destination size is not set, the bad_size protocol error is raised when"]
    #[doc = "the surface state is applied."]
    #[doc = ""]
    #[doc = "The coordinate transformations from buffer pixel coordinates up to"]
    #[doc = "the surface-local coordinates happen in the following order:"]
    #[doc = "1. buffer_transform (wl_surface.set_buffer_transform)"]
    #[doc = "2. buffer_scale (wl_surface.set_buffer_scale)"]
    #[doc = "3. crop and scale (wp_viewport.set*)"]
    #[doc = "This means, that the source rectangle coordinates of crop and scale"]
    #[doc = "are given in the coordinates after the buffer transform and scale,"]
    #[doc = "i.e. in the coordinates that would be the surface-local coordinates"]
    #[doc = "if the crop and scale was not applied."]
    #[doc = ""]
    #[doc = "If src_x or src_y are negative, the bad_value protocol error is raised."]
    #[doc = "Otherwise, if the source rectangle is partially or completely outside of"]
    #[doc = "the non-NULL wl_buffer, then the out_of_buffer protocol error is raised"]
    #[doc = "when the surface state is applied. A NULL wl_buffer does not raise the"]
    #[doc = "out_of_buffer error."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the wp_viewport is destroyed,"]
    #[doc = "all wp_viewport requests except 'destroy' raise the protocol error"]
    #[doc = "no_surface."]
    #[doc = ""]
    #[doc = "If the wp_viewport object is destroyed, the crop and scale"]
    #[doc = "state is removed from the wl_surface. The change will be applied"]
    #[doc = "on the next wl_surface.commit."]
    pub mod wp_viewport {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "negative or zero values in width or height"]
            BadValue = 0u32,
            #[doc = "destination size is not integer"]
            BadSize = 1u32,
            #[doc = "source rectangle extends outside of the content area"]
            OutOfBuffer = 2u32,
            #[doc = "the wl_surface was destroyed"]
            NoSurface = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BadValue),
                    1u32 => Ok(Self::BadSize),
                    2u32 => Ok(Self::OutOfBuffer),
                    3u32 => Ok(Self::NoSurface),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_viewport interface. See the module level documentation for more info"]
        pub trait WpViewport {
            const INTERFACE: &'static str = "wp_viewport";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "The associated wl_surface's crop and scale state is removed."]
            #[doc = "The change is applied on the next wl_surface.commit."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_viewport#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the source rectangle of the associated wl_surface. See"]
            #[doc = "wp_viewport for the description, and relation to the wl_buffer"]
            #[doc = "size."]
            #[doc = ""]
            #[doc = "If all of x, y, width and height are -1.0, the source rectangle is"]
            #[doc = "unset instead. Any other set of values where width or height are zero"]
            #[doc = "or negative, or x or y are negative, raise the bad_value protocol"]
            #[doc = "error."]
            #[doc = ""]
            #[doc = "The crop and scale state is double-buffered, see wl_surface.commit."]
            async fn set_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
                width: crate::wire::Fixed,
                height: crate::wire::Fixed,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_viewport#{}.set_source()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fixed(x)
                    .put_fixed(y)
                    .put_fixed(width)
                    .put_fixed(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the destination size of the associated wl_surface. See"]
            #[doc = "wp_viewport for the description, and relation to the wl_buffer"]
            #[doc = "size."]
            #[doc = ""]
            #[doc = "If width is -1 and height is -1, the destination size is unset"]
            #[doc = "instead. Any other pair of values for width and height that"]
            #[doc = "contains zero or negative values raises the bad_value protocol"]
            #[doc = "error."]
            #[doc = ""]
            #[doc = "The crop and scale state is double-buffered, see wl_surface.commit."]
            async fn set_destination(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_viewport#{}.set_destination()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
pub mod xdg_shell {
    #[doc = "The xdg_wm_base interface is exposed as a global object enabling clients"]
    #[doc = "to turn their wl_surfaces into windows in a desktop environment. It"]
    #[doc = "defines the basic functionality needed for clients and the compositor to"]
    #[doc = "create windows that can be dragged, resized, maximized, etc, as well as"]
    #[doc = "creating transient windows such as popup menus."]
    pub mod xdg_wm_base {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "xdg_wm_base was destroyed before children"]
            DefunctSurfaces = 1u32,
            #[doc = "the client tried to map or destroy a non-topmost popup"]
            NotTheTopmostPopup = 2u32,
            #[doc = "the client specified an invalid popup parent surface"]
            InvalidPopupParent = 3u32,
            #[doc = "the client provided an invalid surface state"]
            InvalidSurfaceState = 4u32,
            #[doc = "the client provided an invalid positioner"]
            InvalidPositioner = 5u32,
            #[doc = "the client didn’t respond to a ping event in time"]
            Unresponsive = 6u32,
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
                    6u32 => Ok(Self::Unresponsive),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_wm_base interface. See the module level documentation for more info"]
        pub trait XdgWmBase {
            const INTERFACE: &'static str = "xdg_wm_base";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this xdg_wm_base object."]
            #[doc = ""]
            #[doc = "Destroying a bound xdg_wm_base object while there are surfaces"]
            #[doc = "still alive created by this xdg_wm_base object instance is illegal"]
            #[doc = "and will result in a defunct_surfaces error."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_wm_base#{}.destroy()", object_id);
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
                tracing::debug!("-> xdg_wm_base#{}.create_positioner()", object_id);
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
            #[doc = "a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is"]
            #[doc = "illegal to create an xdg_surface for a wl_surface which already has an"]
            #[doc = "assigned role and this will result in a role error."]
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
                tracing::debug!("-> xdg_wm_base#{}.get_xdg_surface()", object_id);
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
            #[doc = "the client may be deemed unresponsive. See xdg_wm_base.ping"]
            #[doc = "and xdg_wm_base.error.unresponsive."]
            async fn pong(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_wm_base#{}.pong()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
    #[doc = "positioning a surface raises an invalid_positioner error."]
    pub mod xdg_positioner {
        #[allow(unused)]
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
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Anchor {
            None = 0u32,
            Top = 1u32,
            Bottom = 2u32,
            Left = 3u32,
            Right = 4u32,
            TopLeft = 5u32,
            BottomLeft = 6u32,
            TopRight = 7u32,
            BottomRight = 8u32,
        }
        impl TryFrom<u32> for Anchor {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Bottom),
                    3u32 => Ok(Self::Left),
                    4u32 => Ok(Self::Right),
                    5u32 => Ok(Self::TopLeft),
                    6u32 => Ok(Self::BottomLeft),
                    7u32 => Ok(Self::TopRight),
                    8u32 => Ok(Self::BottomRight),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Gravity {
            None = 0u32,
            Top = 1u32,
            Bottom = 2u32,
            Left = 3u32,
            Right = 4u32,
            TopLeft = 5u32,
            BottomLeft = 6u32,
            TopRight = 7u32,
            BottomRight = 8u32,
        }
        impl TryFrom<u32> for Gravity {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Bottom),
                    3u32 => Ok(Self::Left),
                    4u32 => Ok(Self::Right),
                    5u32 => Ok(Self::TopLeft),
                    6u32 => Ok(Self::BottomLeft),
                    7u32 => Ok(Self::TopRight),
                    8u32 => Ok(Self::BottomRight),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        bitflags::bitflags! { # [doc = "The constraint adjustment value define ways the compositor will adjust"] # [doc = "the position of the surface, if the unadjusted position would result"] # [doc = "in the surface being partly constrained."] # [doc = ""] # [doc = "Whether a surface is considered 'constrained' is left to the compositor"] # [doc = "to determine. For example, the surface may be partly outside the"] # [doc = "compositor's defined 'work area', thus necessitating the child surface's"] # [doc = "position be adjusted until it is entirely inside the work area."] # [doc = ""] # [doc = "The adjustments can be combined, according to a defined precedence: 1)"] # [doc = "Flip, 2) Slide, 3) Resize."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ConstraintAdjustment : u32 { const None = 0u32 ; const SlideX = 1u32 ; const SlideY = 2u32 ; const FlipX = 4u32 ; const FlipY = 8u32 ; const ResizeX = 16u32 ; const ResizeY = 32u32 ; } }
        impl TryFrom<u32> for ConstraintAdjustment {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the xdg_positioner interface. See the module level documentation for more info"]
        pub trait XdgPositioner {
            const INTERFACE: &'static str = "xdg_positioner";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
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
                tracing::debug!("-> xdg_positioner#{}.destroy()", object_id);
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
                tracing::debug!("-> xdg_positioner#{}.set_size()", object_id);
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
            #[doc = "parent surface."]
            #[doc = ""]
            #[doc = "When the xdg_positioner object is used to position a child surface, the"]
            #[doc = "anchor rectangle may not extend outside the window geometry of the"]
            #[doc = "positioned child's parent surface."]
            #[doc = ""]
            #[doc = "If a negative size is set the invalid_input error is raised."]
            async fn set_anchor_rect(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_positioner#{}.set_anchor_rect()", object_id);
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
            #[doc = "Defines the anchor point for the anchor rectangle. The specified anchor"]
            #[doc = "is used derive an anchor point that the child surface will be"]
            #[doc = "positioned relative to. If a corner anchor is set (e.g. 'top_left' or"]
            #[doc = "'bottom_right'), the anchor point will be at the specified corner;"]
            #[doc = "otherwise, the derived anchor point will be centered on the specified"]
            #[doc = "edge, or in the center of the anchor rectangle if no edge is specified."]
            async fn set_anchor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                anchor: Anchor,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_positioner#{}.set_anchor()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(anchor as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Defines in what direction a surface should be positioned, relative to"]
            #[doc = "the anchor point of the parent surface. If a corner gravity is"]
            #[doc = "specified (e.g. 'bottom_right' or 'top_left'), then the child surface"]
            #[doc = "will be placed towards the specified gravity; otherwise, the child"]
            #[doc = "surface will be centered over the anchor point on any axis that had no"]
            #[doc = "gravity specified. If the gravity is not in the ‘gravity’ enum, an"]
            #[doc = "invalid_input error is raised."]
            async fn set_gravity(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                gravity: Gravity,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_positioner#{}.set_gravity()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(gravity as u32)
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
                constraint_adjustment: ConstraintAdjustment,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> xdg_positioner#{}.set_constraint_adjustment()",
                    object_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(constraint_adjustment.bits())
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
                tracing::debug!("-> xdg_positioner#{}.set_offset()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "When set reactive, the surface is reconstrained if the conditions used"]
            #[doc = "for constraining changed, e.g. the parent window moved."]
            #[doc = ""]
            #[doc = "If the conditions changed and the popup was reconstrained, an"]
            #[doc = "xdg_popup.configure event is sent with updated geometry, followed by an"]
            #[doc = "xdg_surface.configure event."]
            async fn set_reactive(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_positioner#{}.set_reactive()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the parent window geometry the compositor should use when"]
            #[doc = "positioning the popup. The compositor may use this information to"]
            #[doc = "determine the future state the popup should be constrained using. If"]
            #[doc = "this doesn't match the dimension of the parent the popup is eventually"]
            #[doc = "positioned against, the behavior is undefined."]
            #[doc = ""]
            #[doc = "The arguments are given in the surface-local coordinate space."]
            async fn set_parent_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                parent_width: i32,
                parent_height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_positioner#{}.set_parent_size()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(parent_width)
                    .put_int(parent_height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the serial of an xdg_surface.configure event this positioner will be"]
            #[doc = "used in response to. The compositor may use this information together"]
            #[doc = "with set_parent_size to determine what future state the popup should be"]
            #[doc = "constrained using."]
            async fn set_parent_configure(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_positioner#{}.set_parent_configure()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
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
    #[doc = "After creating a role-specific object and setting it up, the client must"]
    #[doc = "perform an initial commit without any buffer attached. The compositor"]
    #[doc = "will reply with initial wl_surface state such as"]
    #[doc = "wl_surface.preferred_buffer_scale followed by an xdg_surface.configure"]
    #[doc = "event. The client must acknowledge it and is then allowed to attach a"]
    #[doc = "buffer to map the surface."]
    #[doc = ""]
    #[doc = "Mapping an xdg_surface-based role surface is defined as making it"]
    #[doc = "possible for the surface to be shown by the compositor. Note that"]
    #[doc = "a mapped surface is not guaranteed to be visible once it is mapped."]
    #[doc = ""]
    #[doc = "For an xdg_surface to be mapped by the compositor, the following"]
    #[doc = "conditions must be met:"]
    #[doc = "(1) the client has assigned an xdg_surface-based role to the surface"]
    #[doc = "(2) the client has set and committed the xdg_surface state and the"]
    #[doc = "role-dependent state to the surface"]
    #[doc = "(3) the client has committed a buffer to the surface"]
    #[doc = ""]
    #[doc = "A newly-unmapped surface is considered to have met condition (1) out"]
    #[doc = "of the 3 required conditions for mapping a surface if its role surface"]
    #[doc = "has not been destroyed, i.e. the client must perform the initial commit"]
    #[doc = "again before attaching a buffer."]
    pub mod xdg_surface {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Surface was not fully constructed"]
            NotConstructed = 1u32,
            #[doc = "Surface was already constructed"]
            AlreadyConstructed = 2u32,
            #[doc = "Attaching a buffer to an unconfigured surface"]
            UnconfiguredBuffer = 3u32,
            #[doc = "Invalid serial number when acking a configure event"]
            InvalidSerial = 4u32,
            #[doc = "Width or height was zero or negative"]
            InvalidSize = 5u32,
            #[doc = "Surface was destroyed before its role object"]
            DefunctRoleObject = 6u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NotConstructed),
                    2u32 => Ok(Self::AlreadyConstructed),
                    3u32 => Ok(Self::UnconfiguredBuffer),
                    4u32 => Ok(Self::InvalidSerial),
                    5u32 => Ok(Self::InvalidSize),
                    6u32 => Ok(Self::DefunctRoleObject),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_surface interface. See the module level documentation for more info"]
        pub trait XdgSurface {
            const INTERFACE: &'static str = "xdg_surface";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the xdg_surface object. An xdg_surface must only be destroyed"]
            #[doc = "after its role object has been destroyed, otherwise"]
            #[doc = "a defunct_role_object error is raised."]
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
            #[doc = "This creates an xdg_toplevel object for the given xdg_surface and gives"]
            #[doc = "the associated wl_surface the xdg_toplevel role."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_toplevel for more details about what an"]
            #[doc = "xdg_toplevel is and how it is used."]
            async fn get_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.get_toplevel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This creates an xdg_popup object for the given xdg_surface and gives"]
            #[doc = "the associated wl_surface the xdg_popup role."]
            #[doc = ""]
            #[doc = "If null is passed as a parent, a parent surface must be specified using"]
            #[doc = "some other protocol, before committing the initial state."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_popup for more details about what an"]
            #[doc = "xdg_popup is and how it is used."]
            async fn get_popup(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                parent: Option<crate::wire::ObjectId>,
                positioner: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.get_popup()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(parent)
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
            #[doc = "When maintaining a position, the compositor should treat the (x, y)"]
            #[doc = "coordinate of the window geometry as the top left corner of the window."]
            #[doc = "A client changing the (x, y) window geometry coordinate should in"]
            #[doc = "general not alter the position of the window."]
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
            #[doc = "the wl_surface associated with this xdg_surface, and may extend outside"]
            #[doc = "of the wl_surface itself to mark parts of the subsurface tree as part of"]
            #[doc = "the window geometry."]
            #[doc = ""]
            #[doc = "When applied, the effective window geometry will be the set window"]
            #[doc = "geometry clamped to the bounding rectangle of the combined"]
            #[doc = "geometry of the surface of the xdg_surface and the associated"]
            #[doc = "subsurfaces."]
            #[doc = ""]
            #[doc = "The effective geometry will not be recalculated unless a new call to"]
            #[doc = "set_window_geometry is done and the new pending surface state is"]
            #[doc = "subsequently applied."]
            #[doc = ""]
            #[doc = "The width and height of the effective window geometry must be"]
            #[doc = "greater than zero. Setting an invalid size will raise an"]
            #[doc = "invalid_size error."]
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
            #[doc = "Acking a configure event that was never sent raises an invalid_serial"]
            #[doc = "error."]
            #[doc = ""]
            #[doc = "A client is not required to commit immediately after sending"]
            #[doc = "an ack_configure request - it may even ack_configure several times"]
            #[doc = "before its next surface commit."]
            #[doc = ""]
            #[doc = "A client may send multiple ack_configure requests before committing, but"]
            #[doc = "only the last request sent before a commit indicates which configure"]
            #[doc = "event the client really is responding to."]
            #[doc = ""]
            #[doc = "Sending an ack_configure request consumes the serial number sent with"]
            #[doc = "the request, as well as serial numbers sent by all configure events"]
            #[doc = "sent on this xdg_surface prior to the configure event referenced by"]
            #[doc = "the committed serial."]
            #[doc = ""]
            #[doc = "It is an error to issue multiple ack_configure requests referencing a"]
            #[doc = "serial from the same configure event, or to issue an ack_configure"]
            #[doc = "request referencing a serial from a configure event issued before the"]
            #[doc = "event identified by the last ack_configure request for the same"]
            #[doc = "xdg_surface. Doing so will raise an invalid_serial error."]
            async fn ack_configure(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_surface#{}.ack_configure()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "This interface defines an xdg_surface role which allows a surface to,"]
    #[doc = "among other things, set window-like properties such as maximize,"]
    #[doc = "fullscreen, and minimize, set application-specific metadata like title and"]
    #[doc = "id, and well as trigger user interactive operations such as interactive"]
    #[doc = "resize and move."]
    #[doc = ""]
    #[doc = "A xdg_toplevel by default is responsible for providing the full intended"]
    #[doc = "visual representation of the toplevel, which depending on the window"]
    #[doc = "state, may mean things like a title bar, window controls and drop shadow."]
    #[doc = ""]
    #[doc = "Unmapping an xdg_toplevel means that the surface cannot be shown"]
    #[doc = "by the compositor until it is explicitly mapped again."]
    #[doc = "All active operations (e.g., move, resize) are canceled and all"]
    #[doc = "attributes (e.g. title, state, stacking, ...) are discarded for"]
    #[doc = "an xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to"]
    #[doc = "the state it had right after xdg_surface.get_toplevel. The client"]
    #[doc = "can re-map the toplevel by performing a commit without any buffer"]
    #[doc = "attached, waiting for a configure event and handling it as usual (see"]
    #[doc = "xdg_surface description)."]
    #[doc = ""]
    #[doc = "Attaching a null buffer to a toplevel unmaps the surface."]
    pub mod xdg_toplevel {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "provided value is"]
            #[doc = "not a valid variant of the resize_edge enum"]
            InvalidResizeEdge = 0u32,
            #[doc = "invalid parent toplevel"]
            InvalidParent = 1u32,
            #[doc = "client provided an invalid min or max size"]
            InvalidSize = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidResizeEdge),
                    1u32 => Ok(Self::InvalidParent),
                    2u32 => Ok(Self::InvalidSize),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
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
            TiledLeft = 5u32,
            TiledRight = 6u32,
            TiledTop = 7u32,
            TiledBottom = 8u32,
            Suspended = 9u32,
        }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Maximized),
                    2u32 => Ok(Self::Fullscreen),
                    3u32 => Ok(Self::Resizing),
                    4u32 => Ok(Self::Activated),
                    5u32 => Ok(Self::TiledLeft),
                    6u32 => Ok(Self::TiledRight),
                    7u32 => Ok(Self::TiledTop),
                    8u32 => Ok(Self::TiledBottom),
                    9u32 => Ok(Self::Suspended),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum WmCapabilities {
            #[doc = "show_window_menu is available"]
            WindowMenu = 1u32,
            #[doc = "set_maximized and unset_maximized are available"]
            Maximize = 2u32,
            #[doc = "set_fullscreen and unset_fullscreen are available"]
            Fullscreen = 3u32,
            #[doc = "set_minimized is available"]
            Minimize = 4u32,
        }
        impl TryFrom<u32> for WmCapabilities {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::WindowMenu),
                    2u32 => Ok(Self::Maximize),
                    3u32 => Ok(Self::Fullscreen),
                    4u32 => Ok(Self::Minimize),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_toplevel interface. See the module level documentation for more info"]
        pub trait XdgToplevel {
            const INTERFACE: &'static str = "xdg_toplevel";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This request destroys the role surface and unmaps the surface;"]
            #[doc = "see \"Unmapping\" behavior in interface section for details."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set the \"parent\" of this surface. This surface should be stacked"]
            #[doc = "above the parent surface and all other ancestor surfaces."]
            #[doc = ""]
            #[doc = "Parent surfaces should be set on dialogs, toolboxes, or other"]
            #[doc = "\"auxiliary\" surfaces, so that the parent is raised when the dialog"]
            #[doc = "is raised."]
            #[doc = ""]
            #[doc = "Setting a null parent for a child surface unsets its parent. Setting"]
            #[doc = "a null parent for a surface which currently has no parent is a no-op."]
            #[doc = ""]
            #[doc = "Only mapped surfaces can have child surfaces. Setting a parent which"]
            #[doc = "is not mapped is equivalent to setting a null parent. If a surface"]
            #[doc = "becomes unmapped, its children's parent is set to the parent of"]
            #[doc = "the now-unmapped surface. If the now-unmapped surface has no parent,"]
            #[doc = "its children's parent is unset. If the now-unmapped surface becomes"]
            #[doc = "mapped again, its parent-child relationship is not restored."]
            #[doc = ""]
            #[doc = "The parent toplevel must not be one of the child toplevel's"]
            #[doc = "descendants, and the parent must be different from the child toplevel,"]
            #[doc = "otherwise the invalid_parent protocol error is raised."]
            async fn set_parent(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                parent: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.set_parent()", object_id);
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
                tracing::debug!("-> xdg_toplevel#{}.set_title()", object_id);
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
            #[doc = "Like other properties, a set_app_id request can be sent after the"]
            #[doc = "xdg_toplevel has been mapped to update the property."]
            #[doc = ""]
            #[doc = "See the desktop-entry specification [0] for more details on"]
            #[doc = "application identifiers and how they relate to well-known D-Bus"]
            #[doc = "names and .desktop files."]
            #[doc = ""]
            #[doc = "[0] https://standards.freedesktop.org/desktop-entry-spec/"]
            async fn set_app_id(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                app_id: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.set_app_id()", object_id);
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
            #[doc = "the window menu contains, or even if a window menu will be drawn"]
            #[doc = "at all."]
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
                tracing::debug!("-> xdg_toplevel#{}.show_window_menu()", object_id);
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
                tracing::debug!("-> xdg_toplevel#{}.move()", object_id);
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
            #[doc = "The edges parameter specifies how the surface should be resized, and"]
            #[doc = "is one of the values of the resize_edge enum. Values not matching"]
            #[doc = "a variant of the enum will cause the invalid_resize_edge protocol error."]
            #[doc = "The compositor may use this information to update the surface position"]
            #[doc = "for example when dragging the top left corner. The compositor may also"]
            #[doc = "use this information to adapt its behavior, e.g. choose an appropriate"]
            #[doc = "cursor image."]
            async fn resize(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                edges: ResizeEdge,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.resize()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_uint(edges as u32)
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
            #[doc = "a surface is illegal and will result in an invalid_size error."]
            #[doc = ""]
            #[doc = "The width and height must be greater than or equal to zero. Using"]
            #[doc = "strictly negative values for width or height will result in a"]
            #[doc = "invalid_size error."]
            async fn set_max_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.set_max_size()", object_id);
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
            #[doc = "a surface is illegal and will result in an invalid_size error."]
            #[doc = ""]
            #[doc = "The width and height must be greater than or equal to zero. Using"]
            #[doc = "strictly negative values for width and height will result in a"]
            #[doc = "invalid_size error."]
            async fn set_min_size(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.set_min_size()", object_id);
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
            #[doc = "will respond by emitting a configure event. Whether this configure"]
            #[doc = "actually sets the window maximized is subject to compositor policies."]
            #[doc = "The client must then update its content, drawing in the configured"]
            #[doc = "state. The client must also acknowledge the configure when committing"]
            #[doc = "the new content (see ack_configure)."]
            #[doc = ""]
            #[doc = "It is up to the compositor to decide how and where to maximize the"]
            #[doc = "surface, for example which output and what region of the screen should"]
            #[doc = "be used."]
            #[doc = ""]
            #[doc = "If the surface was already maximized, the compositor will still emit"]
            #[doc = "a configure event with the \"maximized\" state."]
            #[doc = ""]
            #[doc = "If the surface is in a fullscreen state, this request has no direct"]
            #[doc = "effect. It may alter the state the surface is returned to when"]
            #[doc = "unmaximized unless overridden by the compositor."]
            async fn set_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.set_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Unmaximize the surface."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be unmaximized, the compositor"]
            #[doc = "will respond by emitting a configure event. Whether this actually"]
            #[doc = "un-maximizes the window is subject to compositor policies."]
            #[doc = "If available and applicable, the compositor will include the window"]
            #[doc = "geometry dimensions the window had prior to being maximized in the"]
            #[doc = "configure event. The client must then update its content, drawing it in"]
            #[doc = "the configured state. The client must also acknowledge the configure"]
            #[doc = "when committing the new content (see ack_configure)."]
            #[doc = ""]
            #[doc = "It is up to the compositor to position the surface after it was"]
            #[doc = "unmaximized; usually the position the surface had before maximizing, if"]
            #[doc = "applicable."]
            #[doc = ""]
            #[doc = "If the surface was already not maximized, the compositor will still"]
            #[doc = "emit a configure event without the \"maximized\" state."]
            #[doc = ""]
            #[doc = "If the surface is in a fullscreen state, this request has no direct"]
            #[doc = "effect. It may alter the state the surface is returned to when"]
            #[doc = "unmaximized unless overridden by the compositor."]
            async fn unset_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.unset_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Make the surface fullscreen."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be fullscreened, the"]
            #[doc = "compositor will respond by emitting a configure event. Whether the"]
            #[doc = "client is actually put into a fullscreen state is subject to compositor"]
            #[doc = "policies. The client must also acknowledge the configure when"]
            #[doc = "committing the new content (see ack_configure)."]
            #[doc = ""]
            #[doc = "The output passed by the request indicates the client's preference as"]
            #[doc = "to which display it should be set fullscreen on. If this value is NULL,"]
            #[doc = "it's up to the compositor to choose which display will be used to map"]
            #[doc = "this surface."]
            #[doc = ""]
            #[doc = "If the surface doesn't cover the whole output, the compositor will"]
            #[doc = "position the surface in the center of the output and compensate with"]
            #[doc = "with border fill covering the rest of the output. The content of the"]
            #[doc = "border fill is undefined, but should be assumed to be in some way that"]
            #[doc = "attempts to blend into the surrounding area (e.g. solid black)."]
            #[doc = ""]
            #[doc = "If the fullscreened surface is not opaque, the compositor must make"]
            #[doc = "sure that other screen content not part of the same surface tree (made"]
            #[doc = "up of subsurfaces, popups or similarly coupled surfaces) are not"]
            #[doc = "visible below the fullscreened surface."]
            async fn set_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.set_fullscreen()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 11u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Make the surface no longer fullscreen."]
            #[doc = ""]
            #[doc = "After requesting that the surface should be unfullscreened, the"]
            #[doc = "compositor will respond by emitting a configure event."]
            #[doc = "Whether this actually removes the fullscreen state of the client is"]
            #[doc = "subject to compositor policies."]
            #[doc = ""]
            #[doc = "Making a surface unfullscreen sets states for the surface based on the following:"]
            #[doc = "* the state(s) it may have had before becoming fullscreen"]
            #[doc = "* any state(s) decided by the compositor"]
            #[doc = "* any state(s) requested by the client while the surface was fullscreen"]
            #[doc = ""]
            #[doc = "The compositor may include the previous window geometry dimensions in"]
            #[doc = "the configure event, if applicable."]
            #[doc = ""]
            #[doc = "The client must also acknowledge the configure when committing the new"]
            #[doc = "content (see ack_configure)."]
            async fn unset_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel#{}.unset_fullscreen()", object_id);
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
                tracing::debug!("-> xdg_toplevel#{}.set_minimized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 13u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
    #[doc = "A newly created xdg_popup will be stacked on top of all previously created"]
    #[doc = "xdg_popup surfaces associated with the same xdg_toplevel."]
    #[doc = ""]
    #[doc = "The parent of an xdg_popup must be mapped (see the xdg_surface"]
    #[doc = "description) before the xdg_popup itself."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xdg_popup state to take effect."]
    pub mod xdg_popup {
        #[allow(unused)]
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
        #[doc = "Trait to implement the xdg_popup interface. See the module level documentation for more info"]
        pub trait XdgPopup {
            const INTERFACE: &'static str = "xdg_popup";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "This destroys the popup. Explicitly destroying the xdg_popup"]
            #[doc = "object will also dismiss the popup, and unmap the surface."]
            #[doc = ""]
            #[doc = "If this xdg_popup is not the \"topmost\" popup, the"]
            #[doc = "xdg_wm_base.not_the_topmost_popup protocol error will be sent."]
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
                tracing::debug!("-> xdg_popup#{}.grab()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Reposition an already-mapped popup. The popup will be placed given the"]
            #[doc = "details in the passed xdg_positioner object, and a"]
            #[doc = "xdg_popup.repositioned followed by xdg_popup.configure and"]
            #[doc = "xdg_surface.configure will be emitted in response. Any parameters set"]
            #[doc = "by the previous positioner will be discarded."]
            #[doc = ""]
            #[doc = "The passed token will be sent in the corresponding"]
            #[doc = "xdg_popup.repositioned event. The new popup position will not take"]
            #[doc = "effect until the corresponding configure event is acknowledged by the"]
            #[doc = "client. See xdg_popup.repositioned for details. The token itself is"]
            #[doc = "opaque, and has no other special meaning."]
            #[doc = ""]
            #[doc = "If multiple reposition requests are sent, the compositor may skip all"]
            #[doc = "but the last one."]
            #[doc = ""]
            #[doc = "If the popup is repositioned in response to a configure event for its"]
            #[doc = "parent, the client should send an xdg_positioner.set_parent_configure"]
            #[doc = "and possibly an xdg_positioner.set_parent_size request to allow the"]
            #[doc = "compositor to properly constrain the popup."]
            #[doc = ""]
            #[doc = "If the popup is repositioned together with a parent that is being"]
            #[doc = "resized, but not in response to a configure event, the client should"]
            #[doc = "send an xdg_positioner.set_parent_size request."]
            async fn reposition(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                positioner: crate::wire::ObjectId,
                token: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_popup#{}.reposition()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(positioner))
                    .put_uint(token)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
