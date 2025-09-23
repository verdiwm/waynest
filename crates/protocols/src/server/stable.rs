#[allow(clippy::module_inception)]
pub mod linux_dmabuf_v1 {
    #[doc = "This interface offers ways to create generic dmabuf-based wl_buffers."]
    #[doc = ""]
    #[doc = "For more information about dmabuf, see:"]
    #[doc = "https://www.kernel.org/doc/html/next/userspace-api/dma-buf-alloc-exchange.html"]
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
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_dmabuf_v1 {
        #[doc = "Trait to implement the zwp_linux_dmabuf_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufV1<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_v1";
            const VERSION: u32 = 5u32;
            #[doc = "Objects created through this interface, especially wl_buffers, will"]
            #[doc = "remain valid."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This temporary object is used to collect multiple dmabuf handles into"]
            #[doc = "a single batch to create a wl_buffer. It can only be used once and"]
            #[doc = "should be destroyed after a 'created' or 'failed' event has been"]
            #[doc = "received."]
            fn create_params(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                params_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This request creates a new wp_linux_dmabuf_feedback object not bound"]
            #[doc = "to a particular surface. This object will deliver feedback about dmabuf"]
            #[doc = "parameters to use if the client doesn't support per-surface feedback"]
            #[doc = "(see get_surface_feedback)."]
            fn get_default_feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This request creates a new wp_linux_dmabuf_feedback object for the"]
            #[doc = "specified wl_surface. This object will deliver feedback about dmabuf"]
            #[doc = "parameters to use for buffers attached to this surface."]
            #[doc = ""]
            #[doc = "If the surface is destroyed before the wp_linux_dmabuf_feedback object,"]
            #[doc = "the feedback object becomes inert."]
            fn get_surface_feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn format(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                format: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn modifier(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                format: u32,
                modifier_hi: u32,
                modifier_lo: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[doc = "be given in any order. Each plane index can only be set once; subsequent"]
    #[doc = "calls with a plane index which has already been set will result in a"]
    #[doc = "plane_set error being generated."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_buffer_params_v1 {
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
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_linux_buffer_params_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxBufferParamsV1<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_linux_buffer_params_v1";
            const VERSION: u32 = 5u32;
            #[doc = "Cleans up the temporary data sent to the server for dmabuf-based"]
            #[doc = "wl_buffer creation."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn add(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                fd: std::os::fd::OwnedFd,
                plane_idx: u32,
                offset: u32,
                stride: u32,
                modifier_hi: u32,
                modifier_lo: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn create(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
                format: u32,
                flags: Flags,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn create_immed(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                buffer_id: waynest::ObjectId,
                width: i32,
                height: i32,
                format: u32,
                flags: Flags,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This event indicates that the attempted buffer creation was"]
            #[doc = "successful. It provides the new wl_buffer referencing the dmabuf(s)."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "zwp_linux_buffer_params_v1 object."]
            fn created(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event indicates that the attempted buffer creation has"]
            #[doc = "failed. It usually means that one of the dmabuf constraints"]
            #[doc = "has not been fulfilled."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "zwp_linux_buffer_params_v1 object."]
            fn failed(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_linux_dmabuf_feedback_v1 {
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct TrancheFlags : u32 { # [doc = "direct scan-out tranche"] const Scanout = 1u32 ; } }
        impl TryFrom<u32> for TrancheFlags {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for TrancheFlags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_linux_dmabuf_feedback_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufFeedbackV1<C: waynest::Connection, E: From<waynest::ProtocolError>>
        {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_feedback_v1";
            const VERSION: u32 = 5u32;
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the wp_linux_dmabuf_feedback object anymore."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This event is sent after all parameters of a wp_linux_dmabuf_feedback"]
            #[doc = "object have been sent."]
            #[doc = ""]
            #[doc = "This allows changes to the wp_linux_dmabuf_feedback parameters to be"]
            #[doc = "seen as atomic, even if they happen via multiple events."]
            fn done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn format_table(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                fd: std::os::fd::OwnedFd,
                size: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn main_device(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                device: Vec<u8>,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event splits tranche_target_device and tranche_formats events in"]
            #[doc = "preference tranches. It is sent after a set of tranche_target_device"]
            #[doc = "and tranche_formats events; it represents the end of a tranche. The"]
            #[doc = "next tranche will have a lower preference."]
            fn tranche_done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn tranche_target_device(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                device: Vec<u8>,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn tranche_formats(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                indices: Vec<u8>,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event sets tranche-specific flags."]
            #[doc = ""]
            #[doc = "The scanout flag is a hint that direct scan-out may be attempted by the"]
            #[doc = "compositor on the target device if the client appropriately allocates a"]
            #[doc = "buffer. How to allocate a buffer that can be scanned out on the target"]
            #[doc = "device is implementation-defined."]
            #[doc = ""]
            #[doc = "This event is tied to a preference tranche, see the tranche_done event."]
            fn tranche_flags(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                flags: TrancheFlags,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod presentation_time {
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
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
    #[allow(clippy::too_many_arguments)]
    pub mod wp_presentation {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidTimestamp),
                    1u32 => Ok(Self::InvalidFlag),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_presentation interface. See the module level documentation for more info"]
        pub trait WpPresentation<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "wp_presentation";
            const VERSION: u32 = 2u32;
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object. Existing objects created by this object"]
            #[doc = "are not affected."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Request presentation feedback for the current content submission"]
            #[doc = "on the given surface. This creates a new presentation_feedback"]
            #[doc = "object, which will deliver the feedback information once. If"]
            #[doc = "multiple presentation_feedback objects are created for the same"]
            #[doc = "submission, they will all deliver the same information."]
            #[doc = ""]
            #[doc = "For details on what information is returned, see the"]
            #[doc = "presentation_feedback interface."]
            fn feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                callback: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This event tells the client in which clock domain the"]
            #[doc = "compositor interprets the timestamps used by the presentation"]
            #[doc = "extension. This clock is called the presentation clock."]
            #[doc = ""]
            #[doc = "The compositor sends this event when the client binds to the"]
            #[doc = "presentation interface. The presentation clock does not change"]
            #[doc = "during the lifetime of the client connection."]
            #[doc = ""]
            #[doc = "The clock identifier is platform dependent. On POSIX platforms, the"]
            #[doc = "identifier value is one of the clockid_t values accepted by"]
            #[doc = "clock_gettime(). clock_gettime() is defined by POSIX.1-2001."]
            #[doc = ""]
            #[doc = "Timestamps in this clock domain are expressed as tv_sec_hi,"]
            #[doc = "tv_sec_lo, tv_nsec triples, each component being an unsigned"]
            #[doc = "32-bit value. Whole seconds are in tv_sec which is a 64-bit"]
            #[doc = "value combined from tv_sec_hi and tv_sec_lo, and the"]
            #[doc = "additional fractional part in tv_nsec as nanoseconds. Hence,"]
            #[doc = "for valid timestamps tv_nsec must be in [0, 999999999]."]
            #[doc = ""]
            #[doc = "Note that clock_id applies only to the presentation clock,"]
            #[doc = "and implies nothing about e.g. the timestamps used in the"]
            #[doc = "Wayland core protocol input events."]
            #[doc = ""]
            #[doc = "Compositors should prefer a clock which does not jump and is"]
            #[doc = "not slewed e.g. by NTP. The absolute value of the clock is"]
            #[doc = "irrelevant. Precision of one millisecond or better is"]
            #[doc = "recommended. Clients must be able to query the current clock"]
            #[doc = "value directly, not by asking the compositor."]
            fn clock_id(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                clk_id: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod wp_presentation_feedback {
        bitflags::bitflags! { # [doc = "These flags provide information about how the presentation of"] # [doc = "the related content update was done. The intent is to help"] # [doc = "clients assess the reliability of the feedback and the visual"] # [doc = "quality with respect to possible tearing and timings."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Kind : u32 { const Vsync = 1u32 ; const HwClock = 2u32 ; const HwCompletion = 4u32 ; const ZeroCopy = 8u32 ; } }
        impl TryFrom<u32> for Kind {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_presentation_feedback interface. See the module level documentation for more info"]
        pub trait WpPresentationFeedback<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "wp_presentation_feedback";
            const VERSION: u32 = 2u32;
            #[doc = "As presentation can be synchronized to only one output at a"]
            #[doc = "time, this event tells which output it was. This event is only"]
            #[doc = "sent prior to the presented event."]
            #[doc = ""]
            #[doc = "As clients may bind to the same global wl_output multiple"]
            #[doc = "times, this event is sent for each bound instance that matches"]
            #[doc = "the synchronized output. If a client has not bound to the"]
            #[doc = "right wl_output global at all, this event is not sent."]
            fn sync_output(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The associated content update was displayed to the user at the"]
            #[doc = "indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of"]
            #[doc = "the timestamp, see presentation.clock_id event."]
            #[doc = ""]
            #[doc = "The timestamp corresponds to the time when the content update"]
            #[doc = "turned into light the first time on the surface's main output."]
            #[doc = "Compositors may approximate this from the framebuffer flip"]
            #[doc = "completion events from the system, and the latency of the"]
            #[doc = "physical display path if known."]
            #[doc = ""]
            #[doc = "This event is preceded by all related sync_output events"]
            #[doc = "telling which output's refresh cycle the feedback corresponds"]
            #[doc = "to, i.e. the main output for the surface. Compositors are"]
            #[doc = "recommended to choose the output containing the largest part"]
            #[doc = "of the wl_surface, or keeping the output they previously"]
            #[doc = "chose. Having a stable presentation output association helps"]
            #[doc = "clients predict future output refreshes (vblank)."]
            #[doc = ""]
            #[doc = "The 'refresh' argument gives the compositor's prediction of how"]
            #[doc = "many nanoseconds after tv_sec, tv_nsec the very next output"]
            #[doc = "refresh may occur. This is to further aid clients in"]
            #[doc = "predicting future refreshes, i.e., estimating the timestamps"]
            #[doc = "targeting the next few vblanks. If such prediction cannot"]
            #[doc = "usefully be done, the argument is zero."]
            #[doc = ""]
            #[doc = "For version 2 and later, if the output does not have a constant"]
            #[doc = "refresh rate, explicit video mode switches excluded, then the"]
            #[doc = "refresh argument must be either an appropriate rate picked by the"]
            #[doc = "compositor (e.g. fastest rate), or 0 if no such rate exists."]
            #[doc = "For version 1, if the output does not have a constant refresh rate,"]
            #[doc = "the refresh argument must be zero."]
            #[doc = ""]
            #[doc = "The 64-bit value combined from seq_hi and seq_lo is the value"]
            #[doc = "of the output's vertical retrace counter when the content"]
            #[doc = "update was first scanned out to the display. This value must"]
            #[doc = "be compatible with the definition of MSC in"]
            #[doc = "GLX_OML_sync_control specification. Note, that if the display"]
            #[doc = "path has a non-zero latency, the time instant specified by"]
            #[doc = "this counter may differ from the timestamp's."]
            #[doc = ""]
            #[doc = "If the output does not have a concept of vertical retrace or a"]
            #[doc = "refresh cycle, or the output device is self-refreshing without"]
            #[doc = "a way to query the refresh count, then the arguments seq_hi"]
            #[doc = "and seq_lo must be zero."]
            fn presented(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                refresh: u32,
                seq_hi: u32,
                seq_lo: u32,
                flags: Kind,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The content update was never displayed to the user."]
            fn discarded(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
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
#[allow(clippy::module_inception)]
pub mod tablet_v2 {
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "system. All tablets are associated with a seat, to get access to the"]
    #[doc = "actual tablets, use wp_tablet_manager.get_tablet_seat."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_manager_v2 {
        #[doc = "Trait to implement the zwp_tablet_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_manager_v2";
            const VERSION: u32 = 2u32;
            #[doc = "Get the wp_tablet_seat object for the given seat. This object"]
            #[doc = "provides access to all graphics tablets in this seat."]
            fn get_tablet_seat(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                tablet_seat: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Destroy the wp_tablet_manager object. Objects created from this"]
            #[doc = "object are unaffected and should be destroyed separately."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "seat. After binding to this interface, the compositor sends a set of"]
    #[doc = "wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_seat_v2 {
        #[doc = "Trait to implement the zwp_tablet_seat_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_seat_v2";
            const VERSION: u32 = 2u32;
            #[doc = "Destroy the wp_tablet_seat object. Objects created from this"]
            #[doc = "object are unaffected and should be destroyed separately."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This event is sent whenever a new tablet becomes available on this"]
            #[doc = "seat. This event only provides the object id of the tablet, any"]
            #[doc = "static information about the tablet (device name, vid/pid, etc.) is"]
            #[doc = "sent through the wp_tablet interface."]
            fn tablet_added(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event is sent whenever a tool that has not previously been used"]
            #[doc = "with a tablet comes into use. This event only provides the object id"]
            #[doc = "of the tool; any static information about the tool (capabilities,"]
            #[doc = "type, etc.) is sent through the wp_tablet_tool interface."]
            fn tool_added(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn pad_added(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_tool_v2 {
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
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Tilt),
                    2u32 => Ok(Self::Pressure),
                    3u32 => Ok(Self::Distance),
                    4u32 => Ok(Self::Rotation),
                    5u32 => Ok(Self::Slider),
                    6u32 => Ok(Self::Wheel),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Released),
                    1u32 => Ok(Self::Pressed),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_tool_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletToolV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_tool_v2";
            const VERSION: u32 = 2u32;
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
            fn set_cursor(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
                surface: Option<waynest::ObjectId>,
                hotspot_x: i32,
                hotspot_y: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This destroys the client's resource for this tool object."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "The tool type is the high-level type of the tool and usually decides"]
            #[doc = "the interaction expected from this tool."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            fn r#type(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                tool_type: Type,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn hardware_serial(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                hardware_serial_hi: u32,
                hardware_serial_lo: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn hardware_id_wacom(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                hardware_id_hi: u32,
                hardware_id_lo: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event notifies the client of any capabilities of this tool,"]
            #[doc = "beyond the main set of x/y axes and tip up/down detection."]
            #[doc = ""]
            #[doc = "One event is sent for each extra capability available on this tool."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_tool.done event."]
            fn capability(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                capability: Capability,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event signals the end of the initial burst of descriptive"]
            #[doc = "events. A client may consider the static description of the tool to"]
            #[doc = "be complete and finalize initialization of the tool."]
            fn done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn removed(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Notification that this tool is focused on a certain surface."]
            #[doc = ""]
            #[doc = "This event can be received when the tool has moved from one surface to"]
            #[doc = "another, or when the tool has come back into proximity above the"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "If any button is logically down when the tool comes into proximity,"]
            #[doc = "the respective button event is sent after the proximity_in event but"]
            #[doc = "within the same frame as the proximity_in event."]
            fn proximity_in(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
                tablet: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn proximity_out(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn down(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn up(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever a tablet tool moves."]
            fn motion(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                x: waynest::Fixed,
                y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the pressure axis on a tool changes. The value of this"]
            #[doc = "event is normalized to a value between 0 and 65535."]
            #[doc = ""]
            #[doc = "Note that pressure may be nonzero even when a tool is not in logical"]
            #[doc = "contact. See the down and up events for more details."]
            fn pressure(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                pressure: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the distance axis on a tool changes. The value of this"]
            #[doc = "event is normalized to a value between 0 and 65535."]
            #[doc = ""]
            #[doc = "Note that distance may be nonzero even when a tool is not in logical"]
            #[doc = "contact. See the down and up events for more details."]
            fn distance(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                distance: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever one or both of the tilt axes on a tool change. Each tilt"]
            #[doc = "value is in degrees, relative to the z-axis of the tablet."]
            #[doc = "The angle is positive when the top of a tool tilts along the"]
            #[doc = "positive x or y axis."]
            fn tilt(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                tilt_x: waynest::Fixed,
                tilt_y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the z-rotation axis on the tool changes. The"]
            #[doc = "rotation value is in degrees clockwise from the tool's"]
            #[doc = "logical neutral position."]
            fn rotation(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                degrees: waynest::Fixed,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the slider position on the tool changes. The"]
            #[doc = "value is normalized between -65535 and 65535, with 0 as the logical"]
            #[doc = "neutral position of the slider."]
            #[doc = ""]
            #[doc = "The slider is available on e.g. the Wacom Airbrush tool."]
            fn slider(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                position: i32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn wheel(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                degrees: waynest::Fixed,
                clicks: i32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever a button on the tool is pressed or released."]
            #[doc = ""]
            #[doc = "If a button is held down when the tool moves in or out of proximity,"]
            #[doc = "button events are generated by the compositor. See"]
            #[doc = "wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for"]
            #[doc = "details."]
            fn button(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
                button: u32,
                state: ButtonState,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Marks the end of a series of axis and/or button updates from the"]
            #[doc = "tablet. The Wayland protocol requires axis updates to be sent"]
            #[doc = "sequentially, however all events within a frame should be considered"]
            #[doc = "one hardware event."]
            fn frame(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                time: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_v2 {
        #[doc = "Describes the bus types this tablet is connected to."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Bustype {
            #[doc = "USB"]
            Usb = 3u32,
            #[doc = "Bluetooth"]
            Bluetooth = 5u32,
            #[doc = "Virtual"]
            Virtual = 6u32,
            #[doc = "Serial"]
            Serial = 17u32,
            #[doc = "I2C"]
            I2c = 24u32,
        }
        impl TryFrom<u32> for Bustype {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    3u32 => Ok(Self::Usb),
                    5u32 => Ok(Self::Bluetooth),
                    6u32 => Ok(Self::Virtual),
                    17u32 => Ok(Self::Serial),
                    24u32 => Ok(Self::I2c),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Bustype {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_v2";
            const VERSION: u32 = 2u32;
            #[doc = "This destroys the client's resource for this tablet object."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "A descriptive name for the tablet device."]
            #[doc = ""]
            #[doc = "If the device has no descriptive name, this event is not sent."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            fn name(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The vendor and product IDs for the tablet device."]
            #[doc = ""]
            #[doc = "The interpretation of the id depends on the wp_tablet.bustype."]
            #[doc = "Prior to version v2 of this protocol, the id was implied to be a USB"]
            #[doc = "vendor and product ID. If no wp_tablet.bustype is sent, the ID"]
            #[doc = "is to be interpreted as USB vendor and product ID."]
            #[doc = ""]
            #[doc = "If the device has no vendor/product ID, this event is not sent."]
            #[doc = "This can happen for virtual devices or non-USB devices, for instance."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            fn id(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                vid: u32,
                pid: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn path(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                path: String,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event is sent immediately to signal the end of the initial"]
            #[doc = "burst of descriptive events. A client may consider the static"]
            #[doc = "description of the tablet to be complete and finalize initialization"]
            #[doc = "of the tablet."]
            fn done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent when the tablet has been removed from the system. When a tablet"]
            #[doc = "is removed, some tools may be removed."]
            #[doc = ""]
            #[doc = "When this event is received, the client must wp_tablet.destroy"]
            #[doc = "the object."]
            fn removed(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The bustype argument is one of the BUS_ defines in the Linux kernel's"]
            #[doc = "linux/input.h"]
            #[doc = ""]
            #[doc = "If the device has no known bustype or the bustype cannot be"]
            #[doc = "queried, this event is not sent."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet.done event."]
            fn bustype(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                bustype: Bustype,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
    #[doc = "A circular interaction area, such as the touch ring on the Wacom Intuos"]
    #[doc = "Pro series tablets."]
    #[doc = ""]
    #[doc = "Events on a ring are logically grouped by the wl_tablet_pad_ring.frame"]
    #[doc = "event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_ring_v2 {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_ring_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadRingV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
            const VERSION: u32 = 2u32;
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
            fn set_feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                description: String,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This destroys the client's resource for this ring object."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn source(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                source: Source,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the angle on a ring changes."]
            #[doc = ""]
            #[doc = "The angle is provided in degrees clockwise from the logical"]
            #[doc = "north of the ring in the pad's current rotation."]
            fn angle(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                degrees: waynest::Fixed,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn stop(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn frame(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                time: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
    #[doc = "A linear interaction area, such as the strips found in Wacom Cintiq"]
    #[doc = "models."]
    #[doc = ""]
    #[doc = "Events on a strip are logically grouped by the wl_tablet_pad_strip.frame"]
    #[doc = "event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_strip_v2 {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_strip_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadStripV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
            const VERSION: u32 = 2u32;
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
            fn set_feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                description: String,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This destroys the client's resource for this strip object."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn source(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                source: Source,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the position on a strip changes."]
            #[doc = ""]
            #[doc = "The position is normalized to a range of [0, 65535], the 0-value"]
            #[doc = "represents the top-most and/or left-most position of the strip in"]
            #[doc = "the pad's current rotation."]
            fn position(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                position: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn stop(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn frame(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                time: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_group_v2 {
        #[doc = "Trait to implement the zwp_tablet_pad_group_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadGroupV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
            const VERSION: u32 = 2u32;
            #[doc = "Destroy the wp_tablet_pad_group object. Objects created from this object"]
            #[doc = "are unaffected and should be destroyed separately."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn buttons(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                buttons: Vec<u8>,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent on wp_tablet_pad_group initialization to announce available rings."]
            #[doc = "One event is sent for each ring available on this pad group."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event."]
            fn ring(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                ring: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent on wp_tablet_pad initialization to announce available strips."]
            #[doc = "One event is sent for each strip available on this pad group."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event."]
            fn strip(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                strip: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn modes(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                modes: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event is sent immediately to signal the end of the initial"]
            #[doc = "burst of descriptive events. A client may consider the static"]
            #[doc = "description of the tablet to be complete and finalize initialization"]
            #[doc = "of the tablet group."]
            fn done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Notification that the mode was switched."]
            #[doc = ""]
            #[doc = "A mode applies to all buttons, rings, strips and dials in a group"]
            #[doc = "simultaneously, but a client is not required to assign different actions"]
            #[doc = "for each mode. For example, a client may have mode-specific button"]
            #[doc = "mappings but map the ring to vertical scrolling in all modes. Mode"]
            #[doc = "indices start at 0."]
            #[doc = ""]
            #[doc = "Switching modes is compositor-dependent. The compositor may provide"]
            #[doc = "visual cues to the user about the mode, e.g. by toggling LEDs on"]
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
            #[doc = "If a ring, strip or dial action in the new mode differs from the action"]
            #[doc = "in the previous mode, the client should immediately issue a"]
            #[doc = "wp_tablet_ring.set_feedback, wp_tablet_strip.set_feedback or"]
            #[doc = "wp_tablet_dial.set_feedback request for each changed ring, strip or dial."]
            fn mode_switch(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                time: u32,
                serial: u32,
                mode: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent on wp_tablet_pad initialization to announce available dials."]
            #[doc = "One event is sent for each dial available on this pad group."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad_group.done event."]
            fn dial(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                dial: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
    #[doc = "A pad device is a set of buttons, rings, strips and dials"]
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
    #[doc = "All pad features (buttons, rings, strips and dials) are logically divided into"]
    #[doc = "groups and all pads have at least one group. The available groups are"]
    #[doc = "notified through the wp_tablet_pad.group event; the compositor will"]
    #[doc = "emit one event per group before emitting wp_tablet_pad.done."]
    #[doc = ""]
    #[doc = "Groups may have multiple modes. Modes allow clients to map multiple"]
    #[doc = "actions to a single pad feature. Only one mode can be active per group,"]
    #[doc = "although different groups may have different active modes."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_v2 {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Released),
                    1u32 => Ok(Self::Pressed),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ButtonState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_pad_v2";
            const VERSION: u32 = 2u32;
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
            fn set_feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                button: u32,
                description: String,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Destroy the wp_tablet_pad object. Objects created from this object"]
            #[doc = "are unaffected and should be destroyed separately."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Sent on wp_tablet_pad initialization to announce available groups."]
            #[doc = "One event is sent for each pad group available."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad.done event. At least one group will be announced."]
            fn group(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                pad_group: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
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
            fn path(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                path: String,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent on wp_tablet_pad initialization to announce the available"]
            #[doc = "buttons."]
            #[doc = ""]
            #[doc = "This event is sent in the initial burst of events before the"]
            #[doc = "wp_tablet_pad.done event. This event is only sent when at least one"]
            #[doc = "button is available."]
            fn buttons(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                buttons: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event signals the end of the initial burst of descriptive"]
            #[doc = "events. A client may consider the static description of the pad to"]
            #[doc = "be complete and finalize initialization of the pad."]
            fn done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent whenever the physical state of a button changes."]
            fn button(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                time: u32,
                button: u32,
                state: ButtonState,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Notification that this pad is focused on the specified surface."]
            fn enter(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
                tablet: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Notification that this pad is no longer focused on the specified"]
            #[doc = "surface."]
            fn leave(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Sent when the pad has been removed from the system. When a tablet"]
            #[doc = "is removed its pad(s) will be removed too."]
            #[doc = ""]
            #[doc = "When this event is received, the client must destroy all rings, strips"]
            #[doc = "and groups that were offered by this pad, and issue wp_tablet_pad.destroy"]
            #[doc = "the pad itself."]
            fn removed(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
    #[doc = "A rotary control, e.g. a dial or a wheel."]
    #[doc = ""]
    #[doc = "Events on a dial are logically grouped by the wl_tablet_pad_dial.frame"]
    #[doc = "event."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_tablet_pad_dial_v2 {
        #[doc = "Trait to implement the zwp_tablet_pad_dial_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadDialV2<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "zwp_tablet_pad_dial_v2";
            const VERSION: u32 = 2u32;
            #[doc = "Requests the compositor to use the provided feedback string"]
            #[doc = "associated with this dial. This request should be issued immediately"]
            #[doc = "after a wp_tablet_pad_group.mode_switch event from the corresponding"]
            #[doc = "group is received, or whenever the dial is mapped to a different"]
            #[doc = "action. See wp_tablet_pad_group.mode_switch for more details."]
            #[doc = ""]
            #[doc = "Clients are encouraged to provide context-aware descriptions for"]
            #[doc = "the actions associated with the dial, and compositors may use this"]
            #[doc = "information to offer visual feedback about the button layout"]
            #[doc = "(eg. on-screen displays)."]
            #[doc = ""]
            #[doc = "The provided string 'description' is a UTF-8 encoded string to be"]
            #[doc = "associated with this ring, and is considered user-visible; general"]
            #[doc = "internationalization rules apply."]
            #[doc = ""]
            #[doc = "The serial argument will be that of the last"]
            #[doc = "wp_tablet_pad_group.mode_switch event received for the group of this"]
            #[doc = "dial. Requests providing other serials than the most recent one will be"]
            #[doc = "ignored."]
            fn set_feedback(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                description: String,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This destroys the client's resource for this dial object."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Sent whenever the position on a dial changes."]
            #[doc = ""]
            #[doc = "This event carries the wheel delta as multiples or fractions"]
            #[doc = "of 120 with each multiple of 120 representing one logical wheel detent."]
            #[doc = "For example, an axis_value120 of 30 is one quarter of"]
            #[doc = "a logical wheel step in the positive direction, a value120 of"]
            #[doc = "-240 are two logical wheel steps in the negative direction within the"]
            #[doc = "same hardware event. See the wl_pointer.axis_value120 for more details."]
            #[doc = ""]
            #[doc = "The value120 must not be zero."]
            fn delta(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                value120: i32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "Indicates the end of a set of events that represent one logical"]
            #[doc = "hardware dial event. A client is expected to accumulate the data"]
            #[doc = "in all events within the frame before proceeding."]
            #[doc = ""]
            #[doc = "All wp_tablet_pad_dial events before a wp_tablet_pad_dial.frame event belong"]
            #[doc = "logically together."]
            #[doc = ""]
            #[doc = "A wp_tablet_pad_dial.frame event is sent for every logical event"]
            #[doc = "group, even if the group only contains a single wp_tablet_pad_dial"]
            #[doc = "event. Specifically, a client may get a sequence: delta, frame,"]
            #[doc = "delta, frame, etc."]
            fn frame(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                time: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod viewporter {
    #[doc = "The global interface exposing surface cropping and scaling"]
    #[doc = "capabilities is used to instantiate an interface extension for a"]
    #[doc = "wl_surface object. This extended interface will then allow"]
    #[doc = "cropping and scaling the surface contents, effectively"]
    #[doc = "disconnecting the direct relationship between the buffer and the"]
    #[doc = "surface size."]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_viewporter {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a viewport object associated"]
            ViewportExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::ViewportExists),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_viewporter interface. See the module level documentation for more info"]
        pub trait WpViewporter<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "wp_viewporter";
            const VERSION: u32 = 1u32;
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other objects,"]
            #[doc = "wp_viewport objects included."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Instantiate an interface extension for the given wl_surface to"]
            #[doc = "crop and scale its content. If the given wl_surface already has"]
            #[doc = "a wp_viewport object associated, the viewport_exists"]
            #[doc = "protocol error is raised."]
            fn get_viewport(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod wp_viewport {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BadValue),
                    1u32 => Ok(Self::BadSize),
                    2u32 => Ok(Self::OutOfBuffer),
                    3u32 => Ok(Self::NoSurface),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_viewport interface. See the module level documentation for more info"]
        pub trait WpViewport<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "wp_viewport";
            const VERSION: u32 = 1u32;
            #[doc = "The associated wl_surface's crop and scale state is removed."]
            #[doc = "The change is applied on the next wl_surface.commit."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_source(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                x: waynest::Fixed,
                y: waynest::Fixed,
                width: waynest::Fixed,
                height: waynest::Fixed,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_destination(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_shell {
    #[doc = "The xdg_wm_base interface is exposed as a global object enabling clients"]
    #[doc = "to turn their wl_surfaces into windows in a desktop environment. It"]
    #[doc = "defines the basic functionality needed for clients and the compositor to"]
    #[doc = "create windows that can be dragged, resized, maximized, etc, as well as"]
    #[doc = "creating transient windows such as popup menus."]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_wm_base {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::DefunctSurfaces),
                    2u32 => Ok(Self::NotTheTopmostPopup),
                    3u32 => Ok(Self::InvalidPopupParent),
                    4u32 => Ok(Self::InvalidSurfaceState),
                    5u32 => Ok(Self::InvalidPositioner),
                    6u32 => Ok(Self::Unresponsive),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_wm_base interface. See the module level documentation for more info"]
        pub trait XdgWmBase<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "xdg_wm_base";
            const VERSION: u32 = 7u32;
            #[doc = "Destroy this xdg_wm_base object."]
            #[doc = ""]
            #[doc = "Destroying a bound xdg_wm_base object while there are surfaces"]
            #[doc = "still alive created by this xdg_wm_base object instance is illegal"]
            #[doc = "and will result in a defunct_surfaces error."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Create a positioner object. A positioner object is used to position"]
            #[doc = "surfaces relative to some parent surface. See the interface description"]
            #[doc = "and xdg_surface.get_popup for details."]
            fn create_positioner(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn get_xdg_surface(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "A client must respond to a ping event with a pong request or"]
            #[doc = "the client may be deemed unresponsive. See xdg_wm_base.ping"]
            #[doc = "and xdg_wm_base.error.unresponsive."]
            fn pong(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "The ping event asks the client if it's still alive. Pass the"]
            #[doc = "serial specified in the event back to the compositor by sending"]
            #[doc = "a \"pong\" request back with the specified serial. See xdg_wm_base.pong."]
            #[doc = ""]
            #[doc = "Compositors can use this to determine if the client is still"]
            #[doc = "alive. It's unspecified what will happen if the client doesn't"]
            #[doc = "respond to the ping request, or in what timeframe. Clients should"]
            #[doc = "try to respond in a reasonable amount of time. The “unresponsive”"]
            #[doc = "error is provided for compositors that wish to disconnect unresponsive"]
            #[doc = "clients."]
            #[doc = ""]
            #[doc = "A compositor is free to ping in any way it wants, but a client must"]
            #[doc = "always respond to any xdg_wm_base object it created."]
            fn ping(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_positioner {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid input provided"]
            InvalidInput = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidInput),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Anchor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Gravity {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "The constraint adjustment value define ways the compositor will adjust"] # [doc = "the position of the surface, if the unadjusted position would result"] # [doc = "in the surface being partly constrained."] # [doc = ""] # [doc = "Whether a surface is considered 'constrained' is left to the compositor"] # [doc = "to determine. For example, the surface may be partly outside the"] # [doc = "compositor's defined 'work area', thus necessitating the child surface's"] # [doc = "position be adjusted until it is entirely inside the work area."] # [doc = ""] # [doc = "The adjustments can be combined, according to a defined precedence: 1)"] # [doc = "Flip, 2) Slide, 3) Resize."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ConstraintAdjustment : u32 { const None = 0u32 ; const SlideX = 1u32 ; const SlideY = 2u32 ; const FlipX = 4u32 ; const FlipY = 8u32 ; const ResizeX = 16u32 ; const ResizeY = 32u32 ; } }
        impl TryFrom<u32> for ConstraintAdjustment {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ConstraintAdjustment {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_positioner interface. See the module level documentation for more info"]
        pub trait XdgPositioner<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "xdg_positioner";
            const VERSION: u32 = 7u32;
            #[doc = "Notify the compositor that the xdg_positioner will no longer be used."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Set the size of the surface that is to be positioned with the positioner"]
            #[doc = "object. The size is in surface-local coordinates and corresponds to the"]
            #[doc = "window geometry. See xdg_surface.set_window_geometry."]
            #[doc = ""]
            #[doc = "If a zero or negative size is set the invalid_input error is raised."]
            fn set_size(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_anchor_rect(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Defines the anchor point for the anchor rectangle. The specified anchor"]
            #[doc = "is used derive an anchor point that the child surface will be"]
            #[doc = "positioned relative to. If a corner anchor is set (e.g. 'top_left' or"]
            #[doc = "'bottom_right'), the anchor point will be at the specified corner;"]
            #[doc = "otherwise, the derived anchor point will be centered on the specified"]
            #[doc = "edge, or in the center of the anchor rectangle if no edge is specified."]
            fn set_anchor(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                anchor: Anchor,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Defines in what direction a surface should be positioned, relative to"]
            #[doc = "the anchor point of the parent surface. If a corner gravity is"]
            #[doc = "specified (e.g. 'bottom_right' or 'top_left'), then the child surface"]
            #[doc = "will be placed towards the specified gravity; otherwise, the child"]
            #[doc = "surface will be centered over the anchor point on any axis that had no"]
            #[doc = "gravity specified. If the gravity is not in the ‘gravity’ enum, an"]
            #[doc = "invalid_input error is raised."]
            fn set_gravity(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                gravity: Gravity,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_constraint_adjustment(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                constraint_adjustment: ConstraintAdjustment,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_offset(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "When set reactive, the surface is reconstrained if the conditions used"]
            #[doc = "for constraining changed, e.g. the parent window moved."]
            #[doc = ""]
            #[doc = "If the conditions changed and the popup was reconstrained, an"]
            #[doc = "xdg_popup.configure event is sent with updated geometry, followed by an"]
            #[doc = "xdg_surface.configure event."]
            fn set_reactive(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Set the parent window geometry the compositor should use when"]
            #[doc = "positioning the popup. The compositor may use this information to"]
            #[doc = "determine the future state the popup should be constrained using. If"]
            #[doc = "this doesn't match the dimension of the parent the popup is eventually"]
            #[doc = "positioned against, the behavior is undefined."]
            #[doc = ""]
            #[doc = "The arguments are given in the surface-local coordinate space."]
            fn set_parent_size(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                parent_width: i32,
                parent_height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Set the serial of an xdg_surface.configure event this positioner will be"]
            #[doc = "used in response to. The compositor may use this information together"]
            #[doc = "with set_parent_size to determine what future state the popup should be"]
            #[doc = "constrained using."]
            fn set_parent_configure(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[doc = "After creating a role-specific object and setting it up (e.g. by sending"]
    #[doc = "the title, app ID, size constraints, parent, etc), the client must"]
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
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_surface {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NotConstructed),
                    2u32 => Ok(Self::AlreadyConstructed),
                    3u32 => Ok(Self::UnconfiguredBuffer),
                    4u32 => Ok(Self::InvalidSerial),
                    5u32 => Ok(Self::InvalidSize),
                    6u32 => Ok(Self::DefunctRoleObject),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_surface interface. See the module level documentation for more info"]
        pub trait XdgSurface<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "xdg_surface";
            const VERSION: u32 = 7u32;
            #[doc = "Destroy the xdg_surface object. An xdg_surface must only be destroyed"]
            #[doc = "after its role object has been destroyed, otherwise"]
            #[doc = "a defunct_role_object error is raised."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This creates an xdg_toplevel object for the given xdg_surface and gives"]
            #[doc = "the associated wl_surface the xdg_toplevel role."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_toplevel for more details about what an"]
            #[doc = "xdg_toplevel is and how it is used."]
            fn get_toplevel(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This creates an xdg_popup object for the given xdg_surface and gives"]
            #[doc = "the associated wl_surface the xdg_popup role."]
            #[doc = ""]
            #[doc = "If null is passed as a parent, a parent surface must be specified using"]
            #[doc = "some other protocol, before committing the initial state."]
            #[doc = ""]
            #[doc = "See the documentation of xdg_popup for more details about what an"]
            #[doc = "xdg_popup is and how it is used."]
            fn get_popup(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                parent: Option<waynest::ObjectId>,
                positioner: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_window_geometry(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn ack_configure(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn configure(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_toplevel {
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidResizeEdge),
                    1u32 => Ok(Self::InvalidParent),
                    2u32 => Ok(Self::InvalidSize),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            TiledLeft = 5u32,
            TiledRight = 6u32,
            TiledTop = 7u32,
            TiledBottom = 8u32,
            Suspended = 9u32,
            ConstrainedLeft = 10u32,
            ConstrainedRight = 11u32,
            ConstrainedTop = 12u32,
            ConstrainedBottom = 13u32,
        }
        impl TryFrom<u32> for State {
            type Error = waynest::ProtocolError;
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
                    10u32 => Ok(Self::ConstrainedLeft),
                    11u32 => Ok(Self::ConstrainedRight),
                    12u32 => Ok(Self::ConstrainedTop),
                    13u32 => Ok(Self::ConstrainedBottom),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
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
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::WindowMenu),
                    2u32 => Ok(Self::Maximize),
                    3u32 => Ok(Self::Fullscreen),
                    4u32 => Ok(Self::Minimize),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for WmCapabilities {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_toplevel interface. See the module level documentation for more info"]
        pub trait XdgToplevel<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "xdg_toplevel";
            const VERSION: u32 = 7u32;
            #[doc = "This request destroys the role surface and unmaps the surface;"]
            #[doc = "see \"Unmapping\" behavior in interface section for details."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_parent(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                parent: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Set a short title for the surface."]
            #[doc = ""]
            #[doc = "This string may be used to identify the surface in a task bar,"]
            #[doc = "window list, or other user interface elements provided by the"]
            #[doc = "compositor."]
            #[doc = ""]
            #[doc = "The string must be encoded in UTF-8."]
            fn set_title(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                title: String,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_app_id(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                app_id: String,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn show_window_menu(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                serial: u32,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn r#move(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn resize(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                serial: u32,
                edges: ResizeEdge,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_max_size(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_min_size(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_maximized(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn unset_maximized(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn set_fullscreen(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                output: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn unset_fullscreen(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "Request that the compositor minimize your surface. There is no"]
            #[doc = "way to know if the surface is currently minimized, nor is there"]
            #[doc = "any way to unset minimization on this surface."]
            #[doc = ""]
            #[doc = "If you are looking to throttle redrawing when minimized, please"]
            #[doc = "instead use the wl_surface.frame event for this, as this will"]
            #[doc = "also work with live previews on windows in Alt-Tab, Expose or"]
            #[doc = "similar compositor features."]
            fn set_minimized(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn configure(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
                states: Vec<u8>,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The close event is sent by the compositor when the user"]
            #[doc = "wants the surface to be closed. This should be equivalent to"]
            #[doc = "the user clicking the close button in client-side decorations,"]
            #[doc = "if your application has any."]
            #[doc = ""]
            #[doc = "This is only a request that the user intends to close the"]
            #[doc = "window. The client may choose to ignore this request, or show"]
            #[doc = "a dialog to ask the user to save their data, etc."]
            fn close(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The configure_bounds event may be sent prior to a xdg_toplevel.configure"]
            #[doc = "event to communicate the bounds a window geometry size is recommended"]
            #[doc = "to constrain to."]
            #[doc = ""]
            #[doc = "The passed width and height are in surface coordinate space. If width"]
            #[doc = "and height are 0, it means bounds is unknown and equivalent to as if no"]
            #[doc = "configure_bounds event was ever sent for this surface."]
            #[doc = ""]
            #[doc = "The bounds can for example correspond to the size of a monitor excluding"]
            #[doc = "any panels or other shell components, so that a surface isn't created in"]
            #[doc = "a way that it cannot fit."]
            #[doc = ""]
            #[doc = "The bounds may change at any point, and in such a case, a new"]
            #[doc = "xdg_toplevel.configure_bounds will be sent, followed by"]
            #[doc = "xdg_toplevel.configure and xdg_surface.configure."]
            fn configure_bounds(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "This event advertises the capabilities supported by the compositor. If"]
            #[doc = "a capability isn't supported, clients should hide or disable the UI"]
            #[doc = "elements that expose this functionality. For instance, if the"]
            #[doc = "compositor doesn't advertise support for minimized toplevels, a button"]
            #[doc = "triggering the set_minimized request should not be displayed."]
            #[doc = ""]
            #[doc = "The compositor will ignore requests it doesn't support. For instance,"]
            #[doc = "a compositor which doesn't advertise support for minimized will ignore"]
            #[doc = "set_minimized requests."]
            #[doc = ""]
            #[doc = "Compositors must send this event once before the first"]
            #[doc = "xdg_surface.configure event. When the capabilities change, compositors"]
            #[doc = "must send this event again and then send an xdg_surface.configure"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The configured state should not be applied immediately. See"]
            #[doc = "xdg_surface.configure for details."]
            #[doc = ""]
            #[doc = "The capabilities are sent as an array of 32-bit unsigned integers in"]
            #[doc = "native endianness."]
            fn wm_capabilities(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                capabilities: Vec<u8>,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
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
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_popup {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "tried to grab after being mapped"]
            InvalidGrab = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidGrab),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_popup interface. See the module level documentation for more info"]
        pub trait XdgPopup<C: waynest::Connection, E: From<waynest::ProtocolError>> {
            const INTERFACE: &'static str = "xdg_popup";
            const VERSION: u32 = 7u32;
            #[doc = "This destroys the popup. Explicitly destroying the xdg_popup"]
            #[doc = "object will also dismiss the popup, and unmap the surface."]
            #[doc = ""]
            #[doc = "If this xdg_popup is not the \"topmost\" popup, the"]
            #[doc = "xdg_wm_base.not_the_topmost_popup protocol error will be sent."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn grab(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
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
            fn reposition(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                positioner: waynest::ObjectId,
                token: u32,
            ) -> impl Future<Output = Result<(), E>> + Send;
            #[doc = "This event asks the popup surface to configure itself given the"]
            #[doc = "configuration. The configured state should not be applied immediately."]
            #[doc = "See xdg_surface.configure for details."]
            #[doc = ""]
            #[doc = "The x and y arguments represent the position the popup was placed at"]
            #[doc = "given the xdg_positioner rule, relative to the upper left corner of the"]
            #[doc = "window geometry of the parent surface."]
            #[doc = ""]
            #[doc = "For version 2 or older, the configure event for an xdg_popup is only"]
            #[doc = "ever sent once for the initial configuration. Starting with version 3,"]
            #[doc = "it may be sent again if the popup is setup with an xdg_positioner with"]
            #[doc = "set_reactive requested, or in response to xdg_popup.reposition requests."]
            fn configure(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The popup_done event is sent out when a popup is dismissed by the"]
            #[doc = "compositor. The client should destroy the xdg_popup object at this"]
            #[doc = "point."]
            fn popup_done(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            #[doc = "The repositioned event is sent as part of a popup configuration"]
            #[doc = "sequence, together with xdg_popup.configure and lastly"]
            #[doc = "xdg_surface.configure to notify the completion of a reposition request."]
            #[doc = ""]
            #[doc = "The repositioned event is to notify about the completion of a"]
            #[doc = "xdg_popup.reposition request. The token argument is the token passed"]
            #[doc = "in the xdg_popup.reposition request."]
            #[doc = ""]
            #[doc = "Immediately after this event is emitted, xdg_popup.configure and"]
            #[doc = "xdg_surface.configure will be sent with the updated size and position,"]
            #[doc = "as well as a new configure serial."]
            #[doc = ""]
            #[doc = "The client should optionally update the content of the popup, but must"]
            #[doc = "acknowledge the new popup configuration for the new position to take"]
            #[doc = "effect. See xdg_surface.ack_configure for details."]
            fn repositioned(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                token: u32,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move { Ok(()) }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), E>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(E::from(waynest::ProtocolError::UnknownOpcode(opcode))),
                    }
                }
            }
        }
    }
}
