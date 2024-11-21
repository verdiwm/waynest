#![allow(async_fn_in_trait)]
#[allow(clippy::module_inception)]
pub mod wayland {
    #[doc = "The core global object.  This is a special singleton object.  It"]
    #[doc = "is used for internal Wayland protocol features."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_display {
        use futures_util::SinkExt;
        #[doc = "These errors are global and can be emitted in response to any"]
        #[doc = "server request."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "server couldn't find object"]
            InvalidObject = 0u32,
            #[doc = "method doesn't exist on the specified interface or malformed request"]
            InvalidMethod = 1u32,
            #[doc = "server is out of memory"]
            NoMemory = 2u32,
            #[doc = "implementation error in compositor"]
            Implementation = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidObject),
                    1u32 => Ok(Self::InvalidMethod),
                    2u32 => Ok(Self::NoMemory),
                    3u32 => Ok(Self::Implementation),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_display interface. See the module level documentation for more info"]
        pub trait WlDisplay {
            const INTERFACE: &'static str = "wl_display";
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
            #[doc = "The sync request asks the server to emit the 'done' event"]
            #[doc = "on the returned wl_callback object.  Since requests are"]
            #[doc = "handled in-order and events are delivered in-order, this can"]
            #[doc = "be used as a barrier to ensure all previous requests and the"]
            #[doc = "resulting events have been handled."]
            #[doc = ""]
            #[doc = "The object returned by this request will be destroyed by the"]
            #[doc = "compositor after the callback is fired and as such the client must not"]
            #[doc = "attempt to use it after that point."]
            #[doc = ""]
            #[doc = "The callback_data passed in the callback is undefined and should be ignored."]
            async fn sync(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                callback: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_display#{}.sync()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(callback))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request creates a registry object that allows the client"]
            #[doc = "to list and bind the global objects available from the"]
            #[doc = "compositor."]
            #[doc = ""]
            #[doc = "It should be noted that the server side resources consumed in"]
            #[doc = "response to a get_registry request can only be released when the"]
            #[doc = "client disconnects, not when the client side proxy is destroyed."]
            #[doc = "Therefore, clients should invoke get_registry as infrequently as"]
            #[doc = "possible to avoid wasting memory."]
            async fn get_registry(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                registry: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_display#{}.get_registry()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(registry))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The singleton global registry object.  The server has a number of"]
    #[doc = "global objects that are available to all clients.  These objects"]
    #[doc = "typically represent an actual object in the server (for example,"]
    #[doc = "an input device) or they are singleton objects that provide"]
    #[doc = "extension functionality."]
    #[doc = ""]
    #[doc = "When a client creates a registry object, the registry object"]
    #[doc = "will emit a global event for each global currently in the"]
    #[doc = "registry.  Globals come and go as a result of device or"]
    #[doc = "monitor hotplugs, reconfiguration or other events, and the"]
    #[doc = "registry will send out global and global_remove events to"]
    #[doc = "keep the client up to date with the changes.  To mark the end"]
    #[doc = "of the initial burst of events, the client can use the"]
    #[doc = "wl_display.sync request immediately after calling"]
    #[doc = "wl_display.get_registry."]
    #[doc = ""]
    #[doc = "A client can bind to a global object by using the bind"]
    #[doc = "request.  This creates a client-side handle that lets the object"]
    #[doc = "emit events to the client and lets the client invoke requests on"]
    #[doc = "the object."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_registry {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the wl_registry interface. See the module level documentation for more info"]
        pub trait WlRegistry {
            const INTERFACE: &'static str = "wl_registry";
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
            #[doc = "Binds a new, client-created object to the server using the"]
            #[doc = "specified name as the identifier."]
            async fn bind(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                name: u32,
                id: crate::wire::NewId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_registry#{}.bind()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(name)
                    .put_new_id(id)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "Clients can handle the 'done' event to get notified when"]
    #[doc = "the related request is done."]
    #[doc = ""]
    #[doc = "Note, because wl_callback objects are created from multiple independent"]
    #[doc = "factory interfaces, the wl_callback interface is frozen at version 1."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_callback {
        #[doc = "Trait to implement the wl_callback interface. See the module level documentation for more info"]
        pub trait WlCallback {
            const INTERFACE: &'static str = "wl_callback";
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
        }
    }
    #[doc = "A compositor.  This object is a singleton global.  The"]
    #[doc = "compositor is in charge of combining the contents of multiple"]
    #[doc = "surfaces into one displayable output."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_compositor {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the wl_compositor interface. See the module level documentation for more info"]
        pub trait WlCompositor {
            const INTERFACE: &'static str = "wl_compositor";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Ask the compositor to create a new surface."]
            async fn create_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_compositor#{}.create_surface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Ask the compositor to create a new region."]
            async fn create_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_compositor#{}.create_region()", object_id);
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
    #[doc = "The wl_shm_pool object encapsulates a piece of memory shared"]
    #[doc = "between the compositor and client.  Through the wl_shm_pool"]
    #[doc = "object, the client can allocate shared memory wl_buffer objects."]
    #[doc = "All objects created through the same pool share the same"]
    #[doc = "underlying mapped memory. Reusing the mapped memory avoids the"]
    #[doc = "setup/teardown overhead and is useful when interactively resizing"]
    #[doc = "a surface or for many small buffers."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_shm_pool {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the wl_shm_pool interface. See the module level documentation for more info"]
        pub trait WlShmPool {
            const INTERFACE: &'static str = "wl_shm_pool";
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
            #[doc = "Create a wl_buffer object from the pool."]
            #[doc = ""]
            #[doc = "The buffer is created offset bytes into the pool and has"]
            #[doc = "width and height as specified.  The stride argument specifies"]
            #[doc = "the number of bytes from the beginning of one row to the beginning"]
            #[doc = "of the next.  The format is the pixel format of the buffer and"]
            #[doc = "must be one of those advertised through the wl_shm.format event."]
            #[doc = ""]
            #[doc = "A buffer will keep a reference to the pool it was created from"]
            #[doc = "so it is valid to destroy the pool immediately after creating"]
            #[doc = "a buffer from it."]
            async fn create_buffer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                offset: i32,
                width: i32,
                height: i32,
                stride: i32,
                format: super::super::super::core::wayland::wl_shm::Format,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shm_pool#{}.create_buffer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_int(offset)
                    .put_int(width)
                    .put_int(height)
                    .put_int(stride)
                    .put_uint(format as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the shared memory pool."]
            #[doc = ""]
            #[doc = "The mmapped memory will be released when all"]
            #[doc = "buffers that have been created from this pool"]
            #[doc = "are gone."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shm_pool#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request will cause the server to remap the backing memory"]
            #[doc = "for the pool from the file descriptor passed when the pool was"]
            #[doc = "created, but using the new size.  This request can only be"]
            #[doc = "used to make the pool bigger."]
            #[doc = ""]
            #[doc = "This request only changes the amount of bytes that are mmapped"]
            #[doc = "by the server and does not touch the file corresponding to the"]
            #[doc = "file descriptor passed at creation time. It is the client's"]
            #[doc = "responsibility to ensure that the file is at least as big as"]
            #[doc = "the new pool size."]
            async fn resize(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                size: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shm_pool#{}.resize()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(size).build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A singleton global object that provides support for shared"]
    #[doc = "memory."]
    #[doc = ""]
    #[doc = "Clients can create wl_shm_pool objects using the create_pool"]
    #[doc = "request."]
    #[doc = ""]
    #[doc = "On binding the wl_shm object one or more format events"]
    #[doc = "are emitted to inform clients about the valid pixel formats"]
    #[doc = "that can be used for buffers."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_shm {
        use futures_util::SinkExt;
        #[doc = "These errors can be emitted in response to wl_shm requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "buffer format is not known"]
            InvalidFormat = 0u32,
            #[doc = "invalid size or stride during pool or buffer creation"]
            InvalidStride = 1u32,
            #[doc = "mmapping the file descriptor failed"]
            InvalidFd = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidFormat),
                    1u32 => Ok(Self::InvalidStride),
                    2u32 => Ok(Self::InvalidFd),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "This describes the memory layout of an individual pixel."]
        #[doc = ""]
        #[doc = "All renderers should support argb8888 and xrgb8888 but any other"]
        #[doc = "formats are optional and may not be supported by the particular"]
        #[doc = "renderer in use."]
        #[doc = ""]
        #[doc = "The drm format codes match the macros defined in drm_fourcc.h, except"]
        #[doc = "argb8888 and xrgb8888. The formats actually supported by the compositor"]
        #[doc = "will be reported by the format event."]
        #[doc = ""]
        #[doc = "For all wl_shm formats and unless specified in another protocol"]
        #[doc = "extension, pre-multiplied alpha is used for pixel values."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Format {
            #[doc = "32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian"]
            Argb8888 = 0u32,
            #[doc = "32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian"]
            Xrgb8888 = 1u32,
            #[doc = "8-bit color index format, [7:0] C"]
            C8 = 538982467u32,
            #[doc = "8-bit RGB format, [7:0] R:G:B 3:3:2"]
            Rgb332 = 943867730u32,
            #[doc = "8-bit BGR format, [7:0] B:G:R 2:3:3"]
            Bgr233 = 944916290u32,
            #[doc = "16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian"]
            Xrgb4444 = 842093144u32,
            #[doc = "16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian"]
            Xbgr4444 = 842089048u32,
            #[doc = "16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian"]
            Rgbx4444 = 842094674u32,
            #[doc = "16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian"]
            Bgrx4444 = 842094658u32,
            #[doc = "16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian"]
            Argb4444 = 842093121u32,
            #[doc = "16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian"]
            Abgr4444 = 842089025u32,
            #[doc = "16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian"]
            Rgba4444 = 842088786u32,
            #[doc = "16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian"]
            Bgra4444 = 842088770u32,
            #[doc = "16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian"]
            Xrgb1555 = 892424792u32,
            #[doc = "16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian"]
            Xbgr1555 = 892420696u32,
            #[doc = "16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian"]
            Rgbx5551 = 892426322u32,
            #[doc = "16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian"]
            Bgrx5551 = 892426306u32,
            #[doc = "16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian"]
            Argb1555 = 892424769u32,
            #[doc = "16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian"]
            Abgr1555 = 892420673u32,
            #[doc = "16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian"]
            Rgba5551 = 892420434u32,
            #[doc = "16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian"]
            Bgra5551 = 892420418u32,
            #[doc = "16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian"]
            Rgb565 = 909199186u32,
            #[doc = "16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian"]
            Bgr565 = 909199170u32,
            #[doc = "24-bit RGB format, [23:0] R:G:B little endian"]
            Rgb888 = 875710290u32,
            #[doc = "24-bit BGR format, [23:0] B:G:R little endian"]
            Bgr888 = 875710274u32,
            #[doc = "32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian"]
            Xbgr8888 = 875709016u32,
            #[doc = "32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian"]
            Rgbx8888 = 875714642u32,
            #[doc = "32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian"]
            Bgrx8888 = 875714626u32,
            #[doc = "32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian"]
            Abgr8888 = 875708993u32,
            #[doc = "32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian"]
            Rgba8888 = 875708754u32,
            #[doc = "32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian"]
            Bgra8888 = 875708738u32,
            #[doc = "32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian"]
            Xrgb2101010 = 808669784u32,
            #[doc = "32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian"]
            Xbgr2101010 = 808665688u32,
            #[doc = "32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian"]
            Rgbx1010102 = 808671314u32,
            #[doc = "32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian"]
            Bgrx1010102 = 808671298u32,
            #[doc = "32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian"]
            Argb2101010 = 808669761u32,
            #[doc = "32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian"]
            Abgr2101010 = 808665665u32,
            #[doc = "32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian"]
            Rgba1010102 = 808665426u32,
            #[doc = "32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian"]
            Bgra1010102 = 808665410u32,
            #[doc = "packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian"]
            Yuyv = 1448695129u32,
            #[doc = "packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian"]
            Yvyu = 1431918169u32,
            #[doc = "packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian"]
            Uyvy = 1498831189u32,
            #[doc = "packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian"]
            Vyuy = 1498765654u32,
            #[doc = "packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian"]
            Ayuv = 1448433985u32,
            #[doc = "2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane"]
            Nv12 = 842094158u32,
            #[doc = "2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane"]
            Nv21 = 825382478u32,
            #[doc = "2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane"]
            Nv16 = 909203022u32,
            #[doc = "2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane"]
            Nv61 = 825644622u32,
            #[doc = "3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes"]
            Yuv410 = 961959257u32,
            #[doc = "3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes"]
            Yvu410 = 961893977u32,
            #[doc = "3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes"]
            Yuv411 = 825316697u32,
            #[doc = "3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes"]
            Yvu411 = 825316953u32,
            #[doc = "3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes"]
            Yuv420 = 842093913u32,
            #[doc = "3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes"]
            Yvu420 = 842094169u32,
            #[doc = "3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes"]
            Yuv422 = 909202777u32,
            #[doc = "3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes"]
            Yvu422 = 909203033u32,
            #[doc = "3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes"]
            Yuv444 = 875713881u32,
            #[doc = "3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes"]
            Yvu444 = 875714137u32,
            #[doc = "[7:0] R"]
            R8 = 538982482u32,
            #[doc = "[15:0] R little endian"]
            R16 = 540422482u32,
            #[doc = "[15:0] R:G 8:8 little endian"]
            Rg88 = 943212370u32,
            #[doc = "[15:0] G:R 8:8 little endian"]
            Gr88 = 943215175u32,
            #[doc = "[31:0] R:G 16:16 little endian"]
            Rg1616 = 842221394u32,
            #[doc = "[31:0] G:R 16:16 little endian"]
            Gr1616 = 842224199u32,
            #[doc = "[63:0] x:R:G:B 16:16:16:16 little endian"]
            Xrgb16161616f = 1211388504u32,
            #[doc = "[63:0] x:B:G:R 16:16:16:16 little endian"]
            Xbgr16161616f = 1211384408u32,
            #[doc = "[63:0] A:R:G:B 16:16:16:16 little endian"]
            Argb16161616f = 1211388481u32,
            #[doc = "[63:0] A:B:G:R 16:16:16:16 little endian"]
            Abgr16161616f = 1211384385u32,
            #[doc = "[31:0] X:Y:Cb:Cr 8:8:8:8 little endian"]
            Xyuv8888 = 1448434008u32,
            #[doc = "[23:0] Cr:Cb:Y 8:8:8 little endian"]
            Vuy888 = 875713878u32,
            #[doc = "Y followed by U then V, 10:10:10. Non-linear modifier only"]
            Vuy101010 = 808670550u32,
            #[doc = "[63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels"]
            Y210 = 808530521u32,
            #[doc = "[63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels"]
            Y212 = 842084953u32,
            #[doc = "[63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels"]
            Y216 = 909193817u32,
            #[doc = "[31:0] A:Cr:Y:Cb 2:10:10:10 little endian"]
            Y410 = 808531033u32,
            #[doc = "[63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian"]
            Y412 = 842085465u32,
            #[doc = "[63:0] A:Cr:Y:Cb 16:16:16:16 little endian"]
            Y416 = 909194329u32,
            #[doc = "[31:0] X:Cr:Y:Cb 2:10:10:10 little endian"]
            Xvyu2101010 = 808670808u32,
            #[doc = "[63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian"]
            Xvyu1216161616 = 909334104u32,
            #[doc = "[63:0] X:Cr:Y:Cb 16:16:16:16 little endian"]
            Xvyu16161616 = 942954072u32,
            #[doc = "[63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian"]
            Y0l0 = 810299481u32,
            #[doc = "[63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian"]
            X0l0 = 810299480u32,
            #[doc = "[63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian"]
            Y0l2 = 843853913u32,
            #[doc = "[63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian"]
            X0l2 = 843853912u32,
            Yuv4208bit = 942691673u32,
            Yuv42010bit = 808539481u32,
            Xrgb8888A8 = 943805016u32,
            Xbgr8888A8 = 943800920u32,
            Rgbx8888A8 = 943806546u32,
            Bgrx8888A8 = 943806530u32,
            Rgb888A8 = 943798354u32,
            Bgr888A8 = 943798338u32,
            Rgb565A8 = 943797586u32,
            Bgr565A8 = 943797570u32,
            #[doc = "non-subsampled Cr:Cb plane"]
            Nv24 = 875714126u32,
            #[doc = "non-subsampled Cb:Cr plane"]
            Nv42 = 842290766u32,
            #[doc = "2x1 subsampled Cr:Cb plane, 10 bit per channel"]
            P210 = 808530512u32,
            #[doc = "2x2 subsampled Cr:Cb plane 10 bits per channel"]
            P010 = 808530000u32,
            #[doc = "2x2 subsampled Cr:Cb plane 12 bits per channel"]
            P012 = 842084432u32,
            #[doc = "2x2 subsampled Cr:Cb plane 16 bits per channel"]
            P016 = 909193296u32,
            #[doc = "[63:0] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian"]
            Axbxgxrx106106106106 = 808534593u32,
            #[doc = "2x2 subsampled Cr:Cb plane"]
            Nv15 = 892425806u32,
            Q410 = 808531025u32,
            Q401 = 825242705u32,
            #[doc = "[63:0] x:R:G:B 16:16:16:16 little endian"]
            Xrgb16161616 = 942953048u32,
            #[doc = "[63:0] x:B:G:R 16:16:16:16 little endian"]
            Xbgr16161616 = 942948952u32,
            #[doc = "[63:0] A:R:G:B 16:16:16:16 little endian"]
            Argb16161616 = 942953025u32,
            #[doc = "[63:0] A:B:G:R 16:16:16:16 little endian"]
            Abgr16161616 = 942948929u32,
            #[doc = "[7:0] C0:C1:C2:C3:C4:C5:C6:C7 1:1:1:1:1:1:1:1 eight pixels/byte"]
            C1 = 538980675u32,
            #[doc = "[7:0] C0:C1:C2:C3 2:2:2:2 four pixels/byte"]
            C2 = 538980931u32,
            #[doc = "[7:0] C0:C1 4:4 two pixels/byte"]
            C4 = 538981443u32,
            #[doc = "[7:0] D0:D1:D2:D3:D4:D5:D6:D7 1:1:1:1:1:1:1:1 eight pixels/byte"]
            D1 = 538980676u32,
            #[doc = "[7:0] D0:D1:D2:D3 2:2:2:2 four pixels/byte"]
            D2 = 538980932u32,
            #[doc = "[7:0] D0:D1 4:4 two pixels/byte"]
            D4 = 538981444u32,
            #[doc = "[7:0] D"]
            D8 = 538982468u32,
            #[doc = "[7:0] R0:R1:R2:R3:R4:R5:R6:R7 1:1:1:1:1:1:1:1 eight pixels/byte"]
            R1 = 538980690u32,
            #[doc = "[7:0] R0:R1:R2:R3 2:2:2:2 four pixels/byte"]
            R2 = 538980946u32,
            #[doc = "[7:0] R0:R1 4:4 two pixels/byte"]
            R4 = 538981458u32,
            #[doc = "[15:0] x:R 6:10 little endian"]
            R10 = 540029266u32,
            #[doc = "[15:0] x:R 4:12 little endian"]
            R12 = 540160338u32,
            #[doc = "[31:0] A:Cr:Cb:Y 8:8:8:8 little endian"]
            Avuy8888 = 1498764865u32,
            #[doc = "[31:0] X:Cr:Cb:Y 8:8:8:8 little endian"]
            Xvuy8888 = 1498764888u32,
            #[doc = "2x2 subsampled Cr:Cb plane 10 bits per channel packed"]
            P030 = 808661072u32,
        }
        impl TryFrom<u32> for Format {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Argb8888),
                    1u32 => Ok(Self::Xrgb8888),
                    538982467u32 => Ok(Self::C8),
                    943867730u32 => Ok(Self::Rgb332),
                    944916290u32 => Ok(Self::Bgr233),
                    842093144u32 => Ok(Self::Xrgb4444),
                    842089048u32 => Ok(Self::Xbgr4444),
                    842094674u32 => Ok(Self::Rgbx4444),
                    842094658u32 => Ok(Self::Bgrx4444),
                    842093121u32 => Ok(Self::Argb4444),
                    842089025u32 => Ok(Self::Abgr4444),
                    842088786u32 => Ok(Self::Rgba4444),
                    842088770u32 => Ok(Self::Bgra4444),
                    892424792u32 => Ok(Self::Xrgb1555),
                    892420696u32 => Ok(Self::Xbgr1555),
                    892426322u32 => Ok(Self::Rgbx5551),
                    892426306u32 => Ok(Self::Bgrx5551),
                    892424769u32 => Ok(Self::Argb1555),
                    892420673u32 => Ok(Self::Abgr1555),
                    892420434u32 => Ok(Self::Rgba5551),
                    892420418u32 => Ok(Self::Bgra5551),
                    909199186u32 => Ok(Self::Rgb565),
                    909199170u32 => Ok(Self::Bgr565),
                    875710290u32 => Ok(Self::Rgb888),
                    875710274u32 => Ok(Self::Bgr888),
                    875709016u32 => Ok(Self::Xbgr8888),
                    875714642u32 => Ok(Self::Rgbx8888),
                    875714626u32 => Ok(Self::Bgrx8888),
                    875708993u32 => Ok(Self::Abgr8888),
                    875708754u32 => Ok(Self::Rgba8888),
                    875708738u32 => Ok(Self::Bgra8888),
                    808669784u32 => Ok(Self::Xrgb2101010),
                    808665688u32 => Ok(Self::Xbgr2101010),
                    808671314u32 => Ok(Self::Rgbx1010102),
                    808671298u32 => Ok(Self::Bgrx1010102),
                    808669761u32 => Ok(Self::Argb2101010),
                    808665665u32 => Ok(Self::Abgr2101010),
                    808665426u32 => Ok(Self::Rgba1010102),
                    808665410u32 => Ok(Self::Bgra1010102),
                    1448695129u32 => Ok(Self::Yuyv),
                    1431918169u32 => Ok(Self::Yvyu),
                    1498831189u32 => Ok(Self::Uyvy),
                    1498765654u32 => Ok(Self::Vyuy),
                    1448433985u32 => Ok(Self::Ayuv),
                    842094158u32 => Ok(Self::Nv12),
                    825382478u32 => Ok(Self::Nv21),
                    909203022u32 => Ok(Self::Nv16),
                    825644622u32 => Ok(Self::Nv61),
                    961959257u32 => Ok(Self::Yuv410),
                    961893977u32 => Ok(Self::Yvu410),
                    825316697u32 => Ok(Self::Yuv411),
                    825316953u32 => Ok(Self::Yvu411),
                    842093913u32 => Ok(Self::Yuv420),
                    842094169u32 => Ok(Self::Yvu420),
                    909202777u32 => Ok(Self::Yuv422),
                    909203033u32 => Ok(Self::Yvu422),
                    875713881u32 => Ok(Self::Yuv444),
                    875714137u32 => Ok(Self::Yvu444),
                    538982482u32 => Ok(Self::R8),
                    540422482u32 => Ok(Self::R16),
                    943212370u32 => Ok(Self::Rg88),
                    943215175u32 => Ok(Self::Gr88),
                    842221394u32 => Ok(Self::Rg1616),
                    842224199u32 => Ok(Self::Gr1616),
                    1211388504u32 => Ok(Self::Xrgb16161616f),
                    1211384408u32 => Ok(Self::Xbgr16161616f),
                    1211388481u32 => Ok(Self::Argb16161616f),
                    1211384385u32 => Ok(Self::Abgr16161616f),
                    1448434008u32 => Ok(Self::Xyuv8888),
                    875713878u32 => Ok(Self::Vuy888),
                    808670550u32 => Ok(Self::Vuy101010),
                    808530521u32 => Ok(Self::Y210),
                    842084953u32 => Ok(Self::Y212),
                    909193817u32 => Ok(Self::Y216),
                    808531033u32 => Ok(Self::Y410),
                    842085465u32 => Ok(Self::Y412),
                    909194329u32 => Ok(Self::Y416),
                    808670808u32 => Ok(Self::Xvyu2101010),
                    909334104u32 => Ok(Self::Xvyu1216161616),
                    942954072u32 => Ok(Self::Xvyu16161616),
                    810299481u32 => Ok(Self::Y0l0),
                    810299480u32 => Ok(Self::X0l0),
                    843853913u32 => Ok(Self::Y0l2),
                    843853912u32 => Ok(Self::X0l2),
                    942691673u32 => Ok(Self::Yuv4208bit),
                    808539481u32 => Ok(Self::Yuv42010bit),
                    943805016u32 => Ok(Self::Xrgb8888A8),
                    943800920u32 => Ok(Self::Xbgr8888A8),
                    943806546u32 => Ok(Self::Rgbx8888A8),
                    943806530u32 => Ok(Self::Bgrx8888A8),
                    943798354u32 => Ok(Self::Rgb888A8),
                    943798338u32 => Ok(Self::Bgr888A8),
                    943797586u32 => Ok(Self::Rgb565A8),
                    943797570u32 => Ok(Self::Bgr565A8),
                    875714126u32 => Ok(Self::Nv24),
                    842290766u32 => Ok(Self::Nv42),
                    808530512u32 => Ok(Self::P210),
                    808530000u32 => Ok(Self::P010),
                    842084432u32 => Ok(Self::P012),
                    909193296u32 => Ok(Self::P016),
                    808534593u32 => Ok(Self::Axbxgxrx106106106106),
                    892425806u32 => Ok(Self::Nv15),
                    808531025u32 => Ok(Self::Q410),
                    825242705u32 => Ok(Self::Q401),
                    942953048u32 => Ok(Self::Xrgb16161616),
                    942948952u32 => Ok(Self::Xbgr16161616),
                    942953025u32 => Ok(Self::Argb16161616),
                    942948929u32 => Ok(Self::Abgr16161616),
                    538980675u32 => Ok(Self::C1),
                    538980931u32 => Ok(Self::C2),
                    538981443u32 => Ok(Self::C4),
                    538980676u32 => Ok(Self::D1),
                    538980932u32 => Ok(Self::D2),
                    538981444u32 => Ok(Self::D4),
                    538982468u32 => Ok(Self::D8),
                    538980690u32 => Ok(Self::R1),
                    538980946u32 => Ok(Self::R2),
                    538981458u32 => Ok(Self::R4),
                    540029266u32 => Ok(Self::R10),
                    540160338u32 => Ok(Self::R12),
                    1498764865u32 => Ok(Self::Avuy8888),
                    1498764888u32 => Ok(Self::Xvuy8888),
                    808661072u32 => Ok(Self::P030),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_shm interface. See the module level documentation for more info"]
        pub trait WlShm {
            const INTERFACE: &'static str = "wl_shm";
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
            #[doc = "Create a new wl_shm_pool object."]
            #[doc = ""]
            #[doc = "The pool can be used to create shared memory based buffer"]
            #[doc = "objects.  The server will mmap size bytes of the passed file"]
            #[doc = "descriptor, to use as backing memory for the pool."]
            async fn create_pool(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
                size: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shm#{}.create_pool()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_fd(fd)
                    .put_int(size)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the shm object anymore."]
            #[doc = ""]
            #[doc = "Objects created via this interface remain unaffected."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shm#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A buffer provides the content for a wl_surface. Buffers are"]
    #[doc = "created through factory interfaces such as wl_shm, wp_linux_buffer_params"]
    #[doc = "(from the linux-dmabuf protocol extension) or similar. It has a width and"]
    #[doc = "a height and can be attached to a wl_surface, but the mechanism by which a"]
    #[doc = "client provides and updates the contents is defined by the buffer factory"]
    #[doc = "interface."]
    #[doc = ""]
    #[doc = "Color channels are assumed to be electrical rather than optical (in other"]
    #[doc = "words, encoded with a transfer function) unless otherwise specified. If"]
    #[doc = "the buffer uses a format that has an alpha channel, the alpha channel is"]
    #[doc = "assumed to be premultiplied into the electrical color channel values"]
    #[doc = "(after transfer function encoding) unless otherwise specified."]
    #[doc = ""]
    #[doc = "Note, because wl_buffer objects are created from multiple independent"]
    #[doc = "factory interfaces, the wl_buffer interface is frozen at version 1."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_buffer {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the wl_buffer interface. See the module level documentation for more info"]
        pub trait WlBuffer {
            const INTERFACE: &'static str = "wl_buffer";
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
            #[doc = "Destroy a buffer. If and how you need to release the backing"]
            #[doc = "storage is defined by the buffer factory interface."]
            #[doc = ""]
            #[doc = "For possible side-effects to a surface, see wl_surface.attach."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_buffer#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A wl_data_offer represents a piece of data offered for transfer"]
    #[doc = "by another client (the source client).  It is used by the"]
    #[doc = "copy-and-paste and drag-and-drop mechanisms.  The offer"]
    #[doc = "describes the different mime types that the data can be"]
    #[doc = "converted to and provides the mechanism for transferring the"]
    #[doc = "data directly from the source client."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_data_offer {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "finish request was called untimely"]
            InvalidFinish = 0u32,
            #[doc = "action mask contains invalid values"]
            InvalidActionMask = 1u32,
            #[doc = "action argument has an invalid value"]
            InvalidAction = 2u32,
            #[doc = "offer doesn't accept this request"]
            InvalidOffer = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidFinish),
                    1u32 => Ok(Self::InvalidActionMask),
                    2u32 => Ok(Self::InvalidAction),
                    3u32 => Ok(Self::InvalidOffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_data_offer interface. See the module level documentation for more info"]
        pub trait WlDataOffer {
            const INTERFACE: &'static str = "wl_data_offer";
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
            #[doc = "Indicate that the client can accept the given mime type, or"]
            #[doc = "NULL for not accepted."]
            #[doc = ""]
            #[doc = "For objects of version 2 or older, this request is used by the"]
            #[doc = "client to give feedback whether the client can receive the given"]
            #[doc = "mime type, or NULL if none is accepted; the feedback does not"]
            #[doc = "determine whether the drag-and-drop operation succeeds or not."]
            #[doc = ""]
            #[doc = "For objects of version 3 or newer, this request determines the"]
            #[doc = "final result of the drag-and-drop operation. If the end result"]
            #[doc = "is that no mime types were accepted, the drag-and-drop operation"]
            #[doc = "will be cancelled and the corresponding drag source will receive"]
            #[doc = "wl_data_source.cancelled. Clients may still use this event in"]
            #[doc = "conjunction with wl_data_source.action for feedback."]
            async fn accept(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                mime_type: Option<String>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.accept()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_string(mime_type)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "To transfer the offered data, the client issues this request"]
            #[doc = "and indicates the mime type it wants to receive.  The transfer"]
            #[doc = "happens through the passed file descriptor (typically created"]
            #[doc = "with the pipe system call).  The source client writes the data"]
            #[doc = "in the mime type representation requested and then closes the"]
            #[doc = "file descriptor."]
            #[doc = ""]
            #[doc = "The receiving client reads from the read end of the pipe until"]
            #[doc = "EOF and then closes its end, at which point the transfer is"]
            #[doc = "complete."]
            #[doc = ""]
            #[doc = "This request may happen multiple times for different mime types,"]
            #[doc = "both before and after wl_data_device.drop. Drag-and-drop destination"]
            #[doc = "clients may preemptively fetch data or examine it more closely to"]
            #[doc = "determine acceptance."]
            async fn receive(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.receive()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .put_fd(fd)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the data offer."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Notifies the compositor that the drag destination successfully"]
            #[doc = "finished the drag-and-drop operation."]
            #[doc = ""]
            #[doc = "Upon receiving this request, the compositor will emit"]
            #[doc = "wl_data_source.dnd_finished on the drag source client."]
            #[doc = ""]
            #[doc = "It is a client error to perform other requests than"]
            #[doc = "wl_data_offer.destroy after this one. It is also an error to perform"]
            #[doc = "this request after a NULL mime type has been set in"]
            #[doc = "wl_data_offer.accept or no action was received through"]
            #[doc = "wl_data_offer.action."]
            #[doc = ""]
            #[doc = "If wl_data_offer.finish request is received for a non drag and drop"]
            #[doc = "operation, the invalid_finish protocol error is raised."]
            async fn finish(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.finish()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets the actions that the destination side client supports for"]
            #[doc = "this operation. This request may trigger the emission of"]
            #[doc = "wl_data_source.action and wl_data_offer.action events if the compositor"]
            #[doc = "needs to change the selected action."]
            #[doc = ""]
            #[doc = "This request can be called multiple times throughout the"]
            #[doc = "drag-and-drop operation, typically in response to wl_data_device.enter"]
            #[doc = "or wl_data_device.motion events."]
            #[doc = ""]
            #[doc = "This request determines the final result of the drag-and-drop"]
            #[doc = "operation. If the end result is that no action is accepted,"]
            #[doc = "the drag source will receive wl_data_source.cancelled."]
            #[doc = ""]
            #[doc = "The dnd_actions argument must contain only values expressed in the"]
            #[doc = "wl_data_device_manager.dnd_actions enum, and the preferred_action"]
            #[doc = "argument must only contain one of those values set, otherwise it"]
            #[doc = "will result in a protocol error."]
            #[doc = ""]
            #[doc = "While managing an \"ask\" action, the destination drag-and-drop client"]
            #[doc = "may perform further wl_data_offer.receive requests, and is expected"]
            #[doc = "to perform one last wl_data_offer.set_actions request with a preferred"]
            #[doc = "action other than \"ask\" (and optionally wl_data_offer.accept) before"]
            #[doc = "requesting wl_data_offer.finish, in order to convey the action selected"]
            #[doc = "by the user. If the preferred action is not in the"]
            #[doc = "wl_data_offer.source_actions mask, an error will be raised."]
            #[doc = ""]
            #[doc = "If the \"ask\" action is dismissed (e.g. user cancellation), the client"]
            #[doc = "is expected to perform wl_data_offer.destroy right away."]
            #[doc = ""]
            #[doc = "This request can only be made on drag-and-drop offers, a protocol error"]
            #[doc = "will be raised otherwise."]
            async fn set_actions(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                dnd_actions: super::super::super::core::wayland::wl_data_device_manager::DndAction,
                preferred_action : super :: super :: super :: core :: wayland :: wl_data_device_manager :: DndAction,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.set_actions()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(dnd_actions.bits())
                    .put_uint(preferred_action.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The wl_data_source object is the source side of a wl_data_offer."]
    #[doc = "It is created by the source client in a data transfer and"]
    #[doc = "provides a way to describe the offered data and a way to respond"]
    #[doc = "to requests to transfer the data."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_data_source {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "action mask contains invalid values"]
            InvalidActionMask = 0u32,
            #[doc = "source doesn't accept this request"]
            InvalidSource = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidActionMask),
                    1u32 => Ok(Self::InvalidSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_data_source interface. See the module level documentation for more info"]
        pub trait WlDataSource {
            const INTERFACE: &'static str = "wl_data_source";
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
            #[doc = "This request adds a mime type to the set of mime types"]
            #[doc = "advertised to targets.  Can be called several times to offer"]
            #[doc = "multiple types."]
            async fn offer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_source#{}.offer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Destroy the data source."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_source#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Sets the actions that the source side client supports for this"]
            #[doc = "operation. This request may trigger wl_data_source.action and"]
            #[doc = "wl_data_offer.action events if the compositor needs to change the"]
            #[doc = "selected action."]
            #[doc = ""]
            #[doc = "The dnd_actions argument must contain only values expressed in the"]
            #[doc = "wl_data_device_manager.dnd_actions enum, otherwise it will result"]
            #[doc = "in a protocol error."]
            #[doc = ""]
            #[doc = "This request must be made once only, and can only be made on sources"]
            #[doc = "used in drag-and-drop, so it must be performed before"]
            #[doc = "wl_data_device.start_drag. Attempting to use the source other than"]
            #[doc = "for drag-and-drop will raise a protocol error."]
            async fn set_actions(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                dnd_actions: super::super::super::core::wayland::wl_data_device_manager::DndAction,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_source#{}.set_actions()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(dnd_actions.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "There is one wl_data_device per seat which can be obtained"]
    #[doc = "from the global wl_data_device_manager singleton."]
    #[doc = ""]
    #[doc = "A wl_data_device provides access to inter-client data transfer"]
    #[doc = "mechanisms such as copy-and-paste and drag-and-drop."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_data_device {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "source has already been used"]
            UsedSource = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::UsedSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_data_device interface. See the module level documentation for more info"]
        pub trait WlDataDevice {
            const INTERFACE: &'static str = "wl_data_device";
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
            #[doc = "This request asks the compositor to start a drag-and-drop"]
            #[doc = "operation on behalf of the client."]
            #[doc = ""]
            #[doc = "The source argument is the data source that provides the data"]
            #[doc = "for the eventual data transfer. If source is NULL, enter, leave"]
            #[doc = "and motion events are sent only to the client that initiated the"]
            #[doc = "drag and the client is expected to handle the data passing"]
            #[doc = "internally. If source is destroyed, the drag-and-drop session will be"]
            #[doc = "cancelled."]
            #[doc = ""]
            #[doc = "The origin surface is the surface where the drag originates and"]
            #[doc = "the client must have an active implicit grab that matches the"]
            #[doc = "serial."]
            #[doc = ""]
            #[doc = "The icon surface is an optional (can be NULL) surface that"]
            #[doc = "provides an icon to be moved around with the cursor.  Initially,"]
            #[doc = "the top-left corner of the icon surface is placed at the cursor"]
            #[doc = "hotspot, but subsequent wl_surface.offset requests can move the"]
            #[doc = "relative position. Attach requests must be confirmed with"]
            #[doc = "wl_surface.commit as usual. The icon surface is given the role of"]
            #[doc = "a drag-and-drop icon. If the icon surface already has another role,"]
            #[doc = "it raises a protocol error."]
            #[doc = ""]
            #[doc = "The input region is ignored for wl_surfaces with the role of a"]
            #[doc = "drag-and-drop icon."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "start_drag requests. Attempting to reuse a previously-used source"]
            #[doc = "may send a used_source error."]
            async fn start_drag(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
                origin: crate::wire::ObjectId,
                icon: Option<crate::wire::ObjectId>,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_device#{}.start_drag()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .put_object(Some(origin))
                    .put_object(icon)
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request asks the compositor to set the selection"]
            #[doc = "to the data from the source on behalf of the client."]
            #[doc = ""]
            #[doc = "To unset the selection, set the source to NULL."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "start_drag requests. Attempting to reuse a previously-used source"]
            #[doc = "may send a used_source error."]
            async fn set_selection(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_device#{}.set_selection()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request destroys the data device."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_device#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The wl_data_device_manager is a singleton global object that"]
    #[doc = "provides access to inter-client data transfer mechanisms such as"]
    #[doc = "copy-and-paste and drag-and-drop.  These mechanisms are tied to"]
    #[doc = "a wl_seat and this interface lets a client get a wl_data_device"]
    #[doc = "corresponding to a wl_seat."]
    #[doc = ""]
    #[doc = "Depending on the version bound, the objects created from the bound"]
    #[doc = "wl_data_device_manager object will have different requirements for"]
    #[doc = "functioning properly. See wl_data_source.set_actions,"]
    #[doc = "wl_data_offer.accept and wl_data_offer.finish for details."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_data_device_manager {
        use futures_util::SinkExt;
        bitflags::bitflags! { # [doc = "This is a bitmask of the available/preferred actions in a"] # [doc = "drag-and-drop operation."] # [doc = ""] # [doc = "In the compositor, the selected action is a result of matching the"] # [doc = "actions offered by the source and destination sides.  \"action\" events"] # [doc = "with a \"none\" action will be sent to both source and destination if"] # [doc = "there is no match. All further checks will effectively happen on"] # [doc = "(source actions ∩ destination actions)."] # [doc = ""] # [doc = "In addition, compositors may also pick different actions in"] # [doc = "reaction to key modifiers being pressed. One common design that"] # [doc = "is used in major toolkits (and the behavior recommended for"] # [doc = "compositors) is:"] # [doc = ""] # [doc = "- If no modifiers are pressed, the first match (in bit order)"] # [doc = "will be used."] # [doc = "- Pressing Shift selects \"move\", if enabled in the mask."] # [doc = "- Pressing Control selects \"copy\", if enabled in the mask."] # [doc = ""] # [doc = "Behavior beyond that is considered implementation-dependent."] # [doc = "Compositors may for example bind other modifiers (like Alt/Meta)"] # [doc = "or drags initiated with other buttons than BTN_LEFT to specific"] # [doc = "actions (e.g. \"ask\")."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct DndAction : u32 { # [doc = "no action"] const None = 0u32 ; # [doc = "copy action"] const Copy = 1u32 ; # [doc = "move action"] const Move = 2u32 ; # [doc = "ask action"] const Ask = 4u32 ; } }
        impl TryFrom<u32> for DndAction {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the wl_data_device_manager interface. See the module level documentation for more info"]
        pub trait WlDataDeviceManager {
            const INTERFACE: &'static str = "wl_data_device_manager";
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
            #[doc = "Create a new data source."]
            async fn create_data_source(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wl_data_device_manager#{}.create_data_source()",
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
            async fn get_data_device(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_data_device_manager#{}.get_data_device()", object_id);
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
    #[doc = "This interface is implemented by servers that provide"]
    #[doc = "desktop-style user interfaces."]
    #[doc = ""]
    #[doc = "It allows clients to associate a wl_shell_surface with"]
    #[doc = "a basic surface."]
    #[doc = ""]
    #[doc = "Note! This protocol is deprecated and not intended for production use."]
    #[doc = "For desktop-style user interfaces, use xdg_shell. Compositors and clients"]
    #[doc = "should not implement this interface."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_shell {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the wl_shell interface. See the module level documentation for more info"]
        pub trait WlShell {
            const INTERFACE: &'static str = "wl_shell";
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
            #[doc = "Create a shell surface for an existing surface. This gives"]
            #[doc = "the wl_surface the role of a shell surface. If the wl_surface"]
            #[doc = "already has another role, it raises a protocol error."]
            #[doc = ""]
            #[doc = "Only one shell surface can be associated with a given surface."]
            async fn get_shell_surface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell#{}.get_shell_surface()", object_id);
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
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide a desktop-style user interface."]
    #[doc = ""]
    #[doc = "It provides requests to treat surfaces like toplevel, fullscreen"]
    #[doc = "or popup windows, move, resize or maximize them, associate"]
    #[doc = "metadata like title and class, etc."]
    #[doc = ""]
    #[doc = "On the server side the object is automatically destroyed when"]
    #[doc = "the related wl_surface is destroyed. On the client side,"]
    #[doc = "wl_shell_surface_destroy() must be called before destroying"]
    #[doc = "the wl_surface object."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_shell_surface {
        use futures_util::SinkExt;
        bitflags::bitflags! { # [doc = "These values are used to indicate which edge of a surface"] # [doc = "is being dragged in a resize operation. The server may"] # [doc = "use this information to adapt its behavior, e.g. choose"] # [doc = "an appropriate cursor image."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Resize : u32 { # [doc = "no edge"] const None = 0u32 ; # [doc = "top edge"] const Top = 1u32 ; # [doc = "bottom edge"] const Bottom = 2u32 ; # [doc = "left edge"] const Left = 4u32 ; # [doc = "top and left edges"] const TopLeft = 5u32 ; # [doc = "bottom and left edges"] const BottomLeft = 6u32 ; # [doc = "right edge"] const Right = 8u32 ; # [doc = "top and right edges"] const TopRight = 9u32 ; # [doc = "bottom and right edges"] const BottomRight = 10u32 ; } }
        impl TryFrom<u32> for Resize {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        bitflags::bitflags! { # [doc = "These flags specify details of the expected behaviour"] # [doc = "of transient surfaces. Used in the set_transient request."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Transient : u32 { # [doc = "do not set keyboard focus"] const Inactive = 1u32 ; } }
        impl TryFrom<u32> for Transient {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Hints to indicate to the compositor how to deal with a conflict"]
        #[doc = "between the dimensions of the surface and the dimensions of the"]
        #[doc = "output. The compositor is free to ignore this parameter."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum FullscreenMethod {
            #[doc = "no preference, apply default policy"]
            Default = 0u32,
            #[doc = "scale, preserve the surface's aspect ratio and center on output"]
            Scale = 1u32,
            #[doc = "switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch"]
            Driver = 2u32,
            #[doc = "no upscaling, center on output and add black borders to compensate size mismatch"]
            Fill = 3u32,
        }
        impl TryFrom<u32> for FullscreenMethod {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Default),
                    1u32 => Ok(Self::Scale),
                    2u32 => Ok(Self::Driver),
                    3u32 => Ok(Self::Fill),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_shell_surface interface. See the module level documentation for more info"]
        pub trait WlShellSurface {
            const INTERFACE: &'static str = "wl_shell_surface";
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
            #[doc = "A client must respond to a ping event with a pong request or"]
            #[doc = "the client may be deemed unresponsive."]
            async fn pong(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.pong()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Start a pointer-driven move of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to a button press event."]
            #[doc = "The server may ignore move requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized)."]
            async fn r#move(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.move()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Start a pointer-driven resizing of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to a button press event."]
            #[doc = "The server may ignore resize requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized)."]
            async fn resize(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                edges: Resize,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.resize()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_uint(edges.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Map the surface as a toplevel surface."]
            #[doc = ""]
            #[doc = "A toplevel surface is not fullscreen, maximized or transient."]
            async fn set_toplevel(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.set_toplevel()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Map the surface relative to an existing surface."]
            #[doc = ""]
            #[doc = "The x and y arguments specify the location of the upper left"]
            #[doc = "corner of the surface relative to the upper left corner of the"]
            #[doc = "parent surface, in surface-local coordinates."]
            #[doc = ""]
            #[doc = "The flags argument controls details of the transient behaviour."]
            async fn set_transient(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                parent: crate::wire::ObjectId,
                x: i32,
                y: i32,
                flags: Transient,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.set_transient()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(parent))
                    .put_int(x)
                    .put_int(y)
                    .put_uint(flags.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Map the surface as a fullscreen surface."]
            #[doc = ""]
            #[doc = "If an output parameter is given then the surface will be made"]
            #[doc = "fullscreen on that output. If the client does not specify the"]
            #[doc = "output then the compositor will apply its policy - usually"]
            #[doc = "choosing the output on which the surface has the biggest surface"]
            #[doc = "area."]
            #[doc = ""]
            #[doc = "The client may specify a method to resolve a size conflict"]
            #[doc = "between the output size and the surface size - this is provided"]
            #[doc = "through the method parameter."]
            #[doc = ""]
            #[doc = "The framerate parameter is used only when the method is set"]
            #[doc = "to \"driver\", to indicate the preferred framerate. A value of 0"]
            #[doc = "indicates that the client does not care about framerate.  The"]
            #[doc = "framerate is specified in mHz, that is framerate of 60000 is 60Hz."]
            #[doc = ""]
            #[doc = "A method of \"scale\" or \"driver\" implies a scaling operation of"]
            #[doc = "the surface, either via a direct scaling operation or a change of"]
            #[doc = "the output mode. This will override any kind of output scaling, so"]
            #[doc = "that mapping a surface with a buffer size equal to the mode can"]
            #[doc = "fill the screen independent of buffer_scale."]
            #[doc = ""]
            #[doc = "A method of \"fill\" means we don't scale up the buffer, however"]
            #[doc = "any output scale is applied. This means that you may run into"]
            #[doc = "an edge case where the application maps a buffer with the same"]
            #[doc = "size of the output mode but buffer_scale 1 (thus making a"]
            #[doc = "surface larger than the output). In this case it is allowed to"]
            #[doc = "downscale the results to fit the screen."]
            #[doc = ""]
            #[doc = "The compositor must reply to this request with a configure event"]
            #[doc = "with the dimensions for the output on which the surface will"]
            #[doc = "be made fullscreen."]
            async fn set_fullscreen(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                method: FullscreenMethod,
                framerate: u32,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.set_fullscreen()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(method as u32)
                    .put_uint(framerate)
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Map the surface as a popup."]
            #[doc = ""]
            #[doc = "A popup surface is a transient surface with an added pointer"]
            #[doc = "grab."]
            #[doc = ""]
            #[doc = "An existing implicit grab will be changed to owner-events mode,"]
            #[doc = "and the popup grab will continue after the implicit grab ends"]
            #[doc = "(i.e. releasing the mouse button does not cause the popup to"]
            #[doc = "be unmapped)."]
            #[doc = ""]
            #[doc = "The popup grab continues until the window is destroyed or a"]
            #[doc = "mouse button is pressed in any other client's window. A click"]
            #[doc = "in any of the client's surfaces is reported as normal, however,"]
            #[doc = "clicks in other clients' surfaces will be discarded and trigger"]
            #[doc = "the callback."]
            #[doc = ""]
            #[doc = "The x and y arguments specify the location of the upper left"]
            #[doc = "corner of the surface relative to the upper left corner of the"]
            #[doc = "parent surface, in surface-local coordinates."]
            async fn set_popup(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                parent: crate::wire::ObjectId,
                x: i32,
                y: i32,
                flags: Transient,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.set_popup()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .put_uint(serial)
                    .put_object(Some(parent))
                    .put_int(x)
                    .put_int(y)
                    .put_uint(flags.bits())
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Map the surface as a maximized surface."]
            #[doc = ""]
            #[doc = "If an output parameter is given then the surface will be"]
            #[doc = "maximized on that output. If the client does not specify the"]
            #[doc = "output then the compositor will apply its policy - usually"]
            #[doc = "choosing the output on which the surface has the biggest surface"]
            #[doc = "area."]
            #[doc = ""]
            #[doc = "The compositor will reply with a configure event telling"]
            #[doc = "the expected new surface size. The operation is completed"]
            #[doc = "on the next buffer attach to this surface."]
            #[doc = ""]
            #[doc = "A maximized surface typically fills the entire output it is"]
            #[doc = "bound to, except for desktop elements such as panels. This is"]
            #[doc = "the main difference between a maximized shell surface and a"]
            #[doc = "fullscreen shell surface."]
            #[doc = ""]
            #[doc = "The details depend on the compositor implementation."]
            async fn set_maximized(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.set_maximized()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(output)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
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
                tracing::debug!("-> wl_shell_surface#{}.set_title()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(title))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a class for the surface."]
            #[doc = ""]
            #[doc = "The surface class identifies the general class of applications"]
            #[doc = "to which the surface belongs. A common convention is to use the"]
            #[doc = "file name (or the full path if it is a non-standard location) of"]
            #[doc = "the application's .desktop file as the class."]
            async fn set_class(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                class: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.set_class()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(class))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A surface is a rectangular area that may be displayed on zero"]
    #[doc = "or more outputs, and shown any number of times at the compositor's"]
    #[doc = "discretion. They can present wl_buffers, receive user input, and"]
    #[doc = "define a local coordinate system."]
    #[doc = ""]
    #[doc = "The size of a surface (and relative positions on it) is described"]
    #[doc = "in surface-local coordinates, which may differ from the buffer"]
    #[doc = "coordinates of the pixel content, in case a buffer_transform"]
    #[doc = "or a buffer_scale is used."]
    #[doc = ""]
    #[doc = "A surface without a \"role\" is fairly useless: a compositor does"]
    #[doc = "not know where, when or how to present it. The role is the"]
    #[doc = "purpose of a wl_surface. Examples of roles are a cursor for a"]
    #[doc = "pointer (as set by wl_pointer.set_cursor), a drag icon"]
    #[doc = "(wl_data_device.start_drag), a sub-surface"]
    #[doc = "(wl_subcompositor.get_subsurface), and a window as defined by a"]
    #[doc = "shell protocol (e.g. wl_shell.get_shell_surface)."]
    #[doc = ""]
    #[doc = "A surface can have only one role at a time. Initially a"]
    #[doc = "wl_surface does not have a role. Once a wl_surface is given a"]
    #[doc = "role, it is set permanently for the whole lifetime of the"]
    #[doc = "wl_surface object. Giving the current role again is allowed,"]
    #[doc = "unless explicitly forbidden by the relevant interface"]
    #[doc = "specification."]
    #[doc = ""]
    #[doc = "Surface roles are given by requests in other interfaces such as"]
    #[doc = "wl_pointer.set_cursor. The request should explicitly mention"]
    #[doc = "that this request gives a role to a wl_surface. Often, this"]
    #[doc = "request also creates a new protocol object that represents the"]
    #[doc = "role and adds additional functionality to wl_surface. When a"]
    #[doc = "client wants to destroy a wl_surface, they must destroy this role"]
    #[doc = "object before the wl_surface, otherwise a defunct_role_object error is"]
    #[doc = "sent."]
    #[doc = ""]
    #[doc = "Destroying the role object does not remove the role from the"]
    #[doc = "wl_surface, but it may stop the wl_surface from \"playing the role\"."]
    #[doc = "For instance, if a wl_subsurface object is destroyed, the wl_surface"]
    #[doc = "it was created for will be unmapped and forget its position and"]
    #[doc = "z-order. It is allowed to create a wl_subsurface for the same"]
    #[doc = "wl_surface again, but it is not allowed to use the wl_surface as"]
    #[doc = "a cursor (cursor is a different role than sub-surface, and role"]
    #[doc = "switching is not allowed)."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_surface {
        use futures_util::SinkExt;
        #[doc = "These errors can be emitted in response to wl_surface requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "buffer scale value is invalid"]
            InvalidScale = 0u32,
            #[doc = "buffer transform value is invalid"]
            InvalidTransform = 1u32,
            #[doc = "buffer size is invalid"]
            InvalidSize = 2u32,
            #[doc = "buffer offset is invalid"]
            InvalidOffset = 3u32,
            #[doc = "surface was destroyed before its role object"]
            DefunctRoleObject = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidScale),
                    1u32 => Ok(Self::InvalidTransform),
                    2u32 => Ok(Self::InvalidSize),
                    3u32 => Ok(Self::InvalidOffset),
                    4u32 => Ok(Self::DefunctRoleObject),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_surface interface. See the module level documentation for more info"]
        pub trait WlSurface {
            const INTERFACE: &'static str = "wl_surface";
            const VERSION: u32 = 6u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Deletes the surface and invalidates its object ID."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Set a buffer as the content of this surface."]
            #[doc = ""]
            #[doc = "The new size of the surface is calculated based on the buffer"]
            #[doc = "size transformed by the inverse buffer_transform and the"]
            #[doc = "inverse buffer_scale. This means that at commit time the supplied"]
            #[doc = "buffer size must be an integer multiple of the buffer_scale. If"]
            #[doc = "that's not the case, an invalid_size error is sent."]
            #[doc = ""]
            #[doc = "The x and y arguments specify the location of the new pending"]
            #[doc = "buffer's upper left corner, relative to the current buffer's upper"]
            #[doc = "left corner, in surface-local coordinates. In other words, the"]
            #[doc = "x and y, combined with the new surface size define in which"]
            #[doc = "directions the surface's size changes. Setting anything other than 0"]
            #[doc = "as x and y arguments is discouraged, and should instead be replaced"]
            #[doc = "with using the separate wl_surface.offset request."]
            #[doc = ""]
            #[doc = "When the bound wl_surface version is 5 or higher, passing any"]
            #[doc = "non-zero x or y is a protocol violation, and will result in an"]
            #[doc = "'invalid_offset' error being raised. The x and y arguments are ignored"]
            #[doc = "and do not change the pending state. To achieve equivalent semantics,"]
            #[doc = "use wl_surface.offset."]
            #[doc = ""]
            #[doc = "Surface contents are double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The initial surface contents are void; there is no content."]
            #[doc = "wl_surface.attach assigns the given wl_buffer as the pending"]
            #[doc = "wl_buffer. wl_surface.commit makes the pending wl_buffer the new"]
            #[doc = "surface contents, and the size of the surface becomes the size"]
            #[doc = "calculated from the wl_buffer, as described above. After commit,"]
            #[doc = "there is no pending buffer until the next attach."]
            #[doc = ""]
            #[doc = "Committing a pending wl_buffer allows the compositor to read the"]
            #[doc = "pixels in the wl_buffer. The compositor may access the pixels at"]
            #[doc = "any time after the wl_surface.commit request. When the compositor"]
            #[doc = "will not access the pixels anymore, it will send the"]
            #[doc = "wl_buffer.release event. Only after receiving wl_buffer.release,"]
            #[doc = "the client may reuse the wl_buffer. A wl_buffer that has been"]
            #[doc = "attached and then replaced by another attach instead of committed"]
            #[doc = "will not receive a release event, and is not used by the"]
            #[doc = "compositor."]
            #[doc = ""]
            #[doc = "If a pending wl_buffer has been committed to more than one wl_surface,"]
            #[doc = "the delivery of wl_buffer.release events becomes undefined. A well"]
            #[doc = "behaved client should not rely on wl_buffer.release events in this"]
            #[doc = "case. Alternatively, a client could create multiple wl_buffer objects"]
            #[doc = "from the same backing storage or use wp_linux_buffer_release."]
            #[doc = ""]
            #[doc = "Destroying the wl_buffer after wl_buffer.release does not change"]
            #[doc = "the surface contents. Destroying the wl_buffer before wl_buffer.release"]
            #[doc = "is allowed as long as the underlying buffer storage isn't re-used (this"]
            #[doc = "can happen e.g. on client process termination). However, if the client"]
            #[doc = "destroys the wl_buffer before receiving the wl_buffer.release event and"]
            #[doc = "mutates the underlying buffer storage, the surface contents become"]
            #[doc = "undefined immediately."]
            #[doc = ""]
            #[doc = "If wl_surface.attach is sent with a NULL wl_buffer, the"]
            #[doc = "following wl_surface.commit will remove the surface content."]
            #[doc = ""]
            #[doc = "If a pending wl_buffer has been destroyed, the result is not specified."]
            #[doc = "Many compositors are known to remove the surface content on the following"]
            #[doc = "wl_surface.commit, but this behaviour is not universal. Clients seeking to"]
            #[doc = "maximise compatibility should not destroy pending buffers and should"]
            #[doc = "ensure that they explicitly remove content from surfaces, even after"]
            #[doc = "destroying buffers."]
            async fn attach(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                buffer: Option<crate::wire::ObjectId>,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.attach()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(buffer)
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request is used to describe the regions where the pending"]
            #[doc = "buffer is different from the current surface contents, and where"]
            #[doc = "the surface therefore needs to be repainted. The compositor"]
            #[doc = "ignores the parts of the damage that fall outside of the surface."]
            #[doc = ""]
            #[doc = "Damage is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The damage rectangle is specified in surface-local coordinates,"]
            #[doc = "where x and y specify the upper left corner of the damage rectangle."]
            #[doc = ""]
            #[doc = "The initial value for pending damage is empty: no damage."]
            #[doc = "wl_surface.damage adds pending damage: the new pending damage"]
            #[doc = "is the union of old pending damage and the given rectangle."]
            #[doc = ""]
            #[doc = "wl_surface.commit assigns pending damage as the current damage,"]
            #[doc = "and clears pending damage. The server will clear the current"]
            #[doc = "damage as it repaints the surface."]
            #[doc = ""]
            #[doc = "Note! New clients should not use this request. Instead damage can be"]
            #[doc = "posted with wl_surface.damage_buffer which uses buffer coordinates"]
            #[doc = "instead of surface coordinates."]
            async fn damage(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.damage()", object_id);
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
            #[doc = "Request a notification when it is a good time to start drawing a new"]
            #[doc = "frame, by creating a frame callback. This is useful for throttling"]
            #[doc = "redrawing operations, and driving animations."]
            #[doc = ""]
            #[doc = "When a client is animating on a wl_surface, it can use the 'frame'"]
            #[doc = "request to get notified when it is a good time to draw and commit the"]
            #[doc = "next frame of animation. If the client commits an update earlier than"]
            #[doc = "that, it is likely that some updates will not make it to the display,"]
            #[doc = "and the client is wasting resources by drawing too often."]
            #[doc = ""]
            #[doc = "The frame request will take effect on the next wl_surface.commit."]
            #[doc = "The notification will only be posted for one frame unless"]
            #[doc = "requested again. For a wl_surface, the notifications are posted in"]
            #[doc = "the order the frame requests were committed."]
            #[doc = ""]
            #[doc = "The server must send the notifications so that a client"]
            #[doc = "will not send excessive updates, while still allowing"]
            #[doc = "the highest possible update rate for clients that wait for the reply"]
            #[doc = "before drawing again. The server should give some time for the client"]
            #[doc = "to draw and commit after sending the frame callback events to let it"]
            #[doc = "hit the next output refresh."]
            #[doc = ""]
            #[doc = "A server should avoid signaling the frame callbacks if the"]
            #[doc = "surface is not visible in any way, e.g. the surface is off-screen,"]
            #[doc = "or completely obscured by other opaque surfaces."]
            #[doc = ""]
            #[doc = "The object returned by this request will be destroyed by the"]
            #[doc = "compositor after the callback is fired and as such the client must not"]
            #[doc = "attempt to use it after that point."]
            #[doc = ""]
            #[doc = "The callback_data passed in the callback is the current time, in"]
            #[doc = "milliseconds, with an undefined base."]
            async fn frame(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                callback: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.frame()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(callback))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the region of the surface that contains"]
            #[doc = "opaque content."]
            #[doc = ""]
            #[doc = "The opaque region is an optimization hint for the compositor"]
            #[doc = "that lets it optimize the redrawing of content behind opaque"]
            #[doc = "regions.  Setting an opaque region is not required for correct"]
            #[doc = "behaviour, but marking transparent content as opaque will result"]
            #[doc = "in repaint artifacts."]
            #[doc = ""]
            #[doc = "The opaque region is specified in surface-local coordinates."]
            #[doc = ""]
            #[doc = "The compositor ignores the parts of the opaque region that fall"]
            #[doc = "outside of the surface."]
            #[doc = ""]
            #[doc = "Opaque region is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "wl_surface.set_opaque_region changes the pending opaque region."]
            #[doc = "wl_surface.commit copies the pending region to the current region."]
            #[doc = "Otherwise, the pending and current regions are never changed."]
            #[doc = ""]
            #[doc = "The initial value for an opaque region is empty. Setting the pending"]
            #[doc = "opaque region has copy semantics, and the wl_region object can be"]
            #[doc = "destroyed immediately. A NULL wl_region causes the pending opaque"]
            #[doc = "region to be set to empty."]
            async fn set_opaque_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.set_opaque_region()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(region)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the region of the surface that can receive"]
            #[doc = "pointer and touch events."]
            #[doc = ""]
            #[doc = "Input events happening outside of this region will try the next"]
            #[doc = "surface in the server surface stack. The compositor ignores the"]
            #[doc = "parts of the input region that fall outside of the surface."]
            #[doc = ""]
            #[doc = "The input region is specified in surface-local coordinates."]
            #[doc = ""]
            #[doc = "Input region is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "wl_surface.set_input_region changes the pending input region."]
            #[doc = "wl_surface.commit copies the pending region to the current region."]
            #[doc = "Otherwise the pending and current regions are never changed,"]
            #[doc = "except cursor and icon surfaces are special cases, see"]
            #[doc = "wl_pointer.set_cursor and wl_data_device.start_drag."]
            #[doc = ""]
            #[doc = "The initial value for an input region is infinite. That means the"]
            #[doc = "whole surface will accept input. Setting the pending input region"]
            #[doc = "has copy semantics, and the wl_region object can be destroyed"]
            #[doc = "immediately. A NULL wl_region causes the input region to be set"]
            #[doc = "to infinite."]
            async fn set_input_region(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.set_input_region()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(region)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Surface state (input, opaque, and damage regions, attached buffers,"]
            #[doc = "etc.) is double-buffered. Protocol requests modify the pending state,"]
            #[doc = "as opposed to the active state in use by the compositor."]
            #[doc = ""]
            #[doc = "A commit request atomically creates a content update from the pending"]
            #[doc = "state, even if the pending state has not been touched. The content"]
            #[doc = "update is placed in a queue until it becomes active. After commit, the"]
            #[doc = "new pending state is as documented for each related request."]
            #[doc = ""]
            #[doc = "When the content update is applied, the wl_buffer is applied before all"]
            #[doc = "other state. This means that all coordinates in double-buffered state"]
            #[doc = "are relative to the newly attached wl_buffers, except for"]
            #[doc = "wl_surface.attach itself. If there is no newly attached wl_buffer, the"]
            #[doc = "coordinates are relative to the previous content update."]
            #[doc = ""]
            #[doc = "All requests that need a commit to become effective are documented"]
            #[doc = "to affect double-buffered state."]
            #[doc = ""]
            #[doc = "Other interfaces may add further double-buffered surface state."]
            async fn commit(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.commit()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets the transformation that the client has already applied"]
            #[doc = "to the content of the buffer. The accepted values for the transform"]
            #[doc = "parameter are the values for wl_output.transform."]
            #[doc = ""]
            #[doc = "The compositor applies the inverse of this transformation whenever it"]
            #[doc = "uses the buffer contents."]
            #[doc = ""]
            #[doc = "Buffer transform is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "A newly created surface has its buffer transformation set to normal."]
            #[doc = ""]
            #[doc = "wl_surface.set_buffer_transform changes the pending buffer"]
            #[doc = "transformation. wl_surface.commit copies the pending buffer"]
            #[doc = "transformation to the current one. Otherwise, the pending and current"]
            #[doc = "values are never changed."]
            #[doc = ""]
            #[doc = "The purpose of this request is to allow clients to render content"]
            #[doc = "according to the output transform, thus permitting the compositor to"]
            #[doc = "use certain optimizations even if the display is rotated. Using"]
            #[doc = "hardware overlays and scanning out a client buffer for fullscreen"]
            #[doc = "surfaces are examples of such optimizations. Those optimizations are"]
            #[doc = "highly dependent on the compositor implementation, so the use of this"]
            #[doc = "request should be considered on a case-by-case basis."]
            #[doc = ""]
            #[doc = "Note that if the transform value includes 90 or 270 degree rotation,"]
            #[doc = "the width of the buffer will become the surface height and the height"]
            #[doc = "of the buffer will become the surface width."]
            #[doc = ""]
            #[doc = "If transform is not one of the values from the"]
            #[doc = "wl_output.transform enum the invalid_transform protocol error"]
            #[doc = "is raised."]
            async fn set_buffer_transform(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.set_buffer_transform()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(transform as u32)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request sets an optional scaling factor on how the compositor"]
            #[doc = "interprets the contents of the buffer attached to the window."]
            #[doc = ""]
            #[doc = "Buffer scale is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "A newly created surface has its buffer scale set to 1."]
            #[doc = ""]
            #[doc = "wl_surface.set_buffer_scale changes the pending buffer scale."]
            #[doc = "wl_surface.commit copies the pending buffer scale to the current one."]
            #[doc = "Otherwise, the pending and current values are never changed."]
            #[doc = ""]
            #[doc = "The purpose of this request is to allow clients to supply higher"]
            #[doc = "resolution buffer data for use on high resolution outputs. It is"]
            #[doc = "intended that you pick the same buffer scale as the scale of the"]
            #[doc = "output that the surface is displayed on. This means the compositor"]
            #[doc = "can avoid scaling when rendering the surface on that output."]
            #[doc = ""]
            #[doc = "Note that if the scale is larger than 1, then you have to attach"]
            #[doc = "a buffer that is larger (by a factor of scale in each dimension)"]
            #[doc = "than the desired surface size."]
            #[doc = ""]
            #[doc = "If scale is not greater than 0 the invalid_scale protocol error is"]
            #[doc = "raised."]
            async fn set_buffer_scale(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                scale: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.set_buffer_scale()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(scale).build();
                socket
                    .send(crate::wire::Message::new(object_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This request is used to describe the regions where the pending"]
            #[doc = "buffer is different from the current surface contents, and where"]
            #[doc = "the surface therefore needs to be repainted. The compositor"]
            #[doc = "ignores the parts of the damage that fall outside of the surface."]
            #[doc = ""]
            #[doc = "Damage is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The damage rectangle is specified in buffer coordinates,"]
            #[doc = "where x and y specify the upper left corner of the damage rectangle."]
            #[doc = ""]
            #[doc = "The initial value for pending damage is empty: no damage."]
            #[doc = "wl_surface.damage_buffer adds pending damage: the new pending"]
            #[doc = "damage is the union of old pending damage and the given rectangle."]
            #[doc = ""]
            #[doc = "wl_surface.commit assigns pending damage as the current damage,"]
            #[doc = "and clears pending damage. The server will clear the current"]
            #[doc = "damage as it repaints the surface."]
            #[doc = ""]
            #[doc = "This request differs from wl_surface.damage in only one way - it"]
            #[doc = "takes damage in buffer coordinates instead of surface-local"]
            #[doc = "coordinates. While this generally is more intuitive than surface"]
            #[doc = "coordinates, it is especially desirable when using wp_viewport"]
            #[doc = "or when a drawing library (like EGL) is unaware of buffer scale"]
            #[doc = "and buffer transform."]
            #[doc = ""]
            #[doc = "Note: Because buffer transformation changes and damage requests may"]
            #[doc = "be interleaved in the protocol stream, it is impossible to determine"]
            #[doc = "the actual mapping between surface and buffer damage until"]
            #[doc = "wl_surface.commit time. Therefore, compositors wishing to take both"]
            #[doc = "kinds of damage into account will have to accumulate damage from the"]
            #[doc = "two requests separately and only transform from one to the other"]
            #[doc = "after receiving the wl_surface.commit."]
            async fn damage_buffer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.damage_buffer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
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
            #[doc = "The x and y arguments specify the location of the new pending"]
            #[doc = "buffer's upper left corner, relative to the current buffer's upper"]
            #[doc = "left corner, in surface-local coordinates. In other words, the"]
            #[doc = "x and y, combined with the new surface size define in which"]
            #[doc = "directions the surface's size changes."]
            #[doc = ""]
            #[doc = "Surface location offset is double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            #[doc = "This request is semantically equivalent to and the replaces the x and y"]
            #[doc = "arguments in the wl_surface.attach request in wl_surface versions prior"]
            #[doc = "to 5. See wl_surface.attach for details."]
            async fn offset(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_surface#{}.offset()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A seat is a group of keyboards, pointer and touch devices. This"]
    #[doc = "object is published as a global during start up, or when such a"]
    #[doc = "device is hot plugged.  A seat typically has a pointer and"]
    #[doc = "maintains a keyboard focus and a pointer focus."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_seat {
        use futures_util::SinkExt;
        bitflags::bitflags! { # [doc = "This is a bitmask of capabilities this seat has; if a member is"] # [doc = "set, then it is present on the seat."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "the seat has pointer devices"] const Pointer = 1u32 ; # [doc = "the seat has one or more keyboards"] const Keyboard = 2u32 ; # [doc = "the seat has touch devices"] const Touch = 4u32 ; } }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "These errors can be emitted in response to wl_seat requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "get_pointer, get_keyboard or get_touch called on seat without the matching capability"]
            MissingCapability = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::MissingCapability),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_seat interface. See the module level documentation for more info"]
        pub trait WlSeat {
            const INTERFACE: &'static str = "wl_seat";
            const VERSION: u32 = 9u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "The ID provided will be initialized to the wl_pointer interface"]
            #[doc = "for this seat."]
            #[doc = ""]
            #[doc = "This request only takes effect if the seat has the pointer"]
            #[doc = "capability, or has had the pointer capability in the past."]
            #[doc = "It is a protocol violation to issue this request on a seat that has"]
            #[doc = "never had the pointer capability. The missing_capability error will"]
            #[doc = "be sent in this case."]
            async fn get_pointer(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_seat#{}.get_pointer()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The ID provided will be initialized to the wl_keyboard interface"]
            #[doc = "for this seat."]
            #[doc = ""]
            #[doc = "This request only takes effect if the seat has the keyboard"]
            #[doc = "capability, or has had the keyboard capability in the past."]
            #[doc = "It is a protocol violation to issue this request on a seat that has"]
            #[doc = "never had the keyboard capability. The missing_capability error will"]
            #[doc = "be sent in this case."]
            async fn get_keyboard(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_seat#{}.get_keyboard()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The ID provided will be initialized to the wl_touch interface"]
            #[doc = "for this seat."]
            #[doc = ""]
            #[doc = "This request only takes effect if the seat has the touch"]
            #[doc = "capability, or has had the touch capability in the past."]
            #[doc = "It is a protocol violation to issue this request on a seat that has"]
            #[doc = "never had the touch capability. The missing_capability error will"]
            #[doc = "be sent in this case."]
            async fn get_touch(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_seat#{}.get_touch()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the seat object anymore."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_seat#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The wl_pointer interface represents one or more input devices,"]
    #[doc = "such as mice, which control the pointer location and pointer_focus"]
    #[doc = "of a seat."]
    #[doc = ""]
    #[doc = "The wl_pointer interface generates motion, enter and leave"]
    #[doc = "events for the surfaces that the pointer is located over,"]
    #[doc = "and button and axis events for button presses, button releases"]
    #[doc = "and scrolling."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_pointer {
        use futures_util::SinkExt;
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
        #[doc = "Describes the physical state of a button that produced the button"]
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
        #[doc = "Describes the axis types of scroll events."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Axis {
            #[doc = "vertical axis"]
            VerticalScroll = 0u32,
            #[doc = "horizontal axis"]
            HorizontalScroll = 1u32,
        }
        impl TryFrom<u32> for Axis {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::VerticalScroll),
                    1u32 => Ok(Self::HorizontalScroll),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Describes the source types for axis events. This indicates to the"]
        #[doc = "client how an axis event was physically generated; a client may"]
        #[doc = "adjust the user interface accordingly. For example, scroll events"]
        #[doc = "from a \"finger\" source may be in a smooth coordinate space with"]
        #[doc = "kinetic scrolling whereas a \"wheel\" source may be in discrete steps"]
        #[doc = "of a number of lines."]
        #[doc = ""]
        #[doc = "The \"continuous\" axis source is a device generating events in a"]
        #[doc = "continuous coordinate space, but using something other than a"]
        #[doc = "finger. One example for this source is button-based scrolling where"]
        #[doc = "the vertical motion of a device is converted to scroll events while"]
        #[doc = "a button is held down."]
        #[doc = ""]
        #[doc = "The \"wheel tilt\" axis source indicates that the actual device is a"]
        #[doc = "wheel but the scroll event is not caused by a rotation but a"]
        #[doc = "(usually sideways) tilt of the wheel."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AxisSource {
            #[doc = "a physical wheel rotation"]
            Wheel = 0u32,
            #[doc = "finger on a touch surface"]
            Finger = 1u32,
            #[doc = "continuous coordinate space"]
            Continuous = 2u32,
            #[doc = "a physical wheel tilt"]
            WheelTilt = 3u32,
        }
        impl TryFrom<u32> for AxisSource {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Wheel),
                    1u32 => Ok(Self::Finger),
                    2u32 => Ok(Self::Continuous),
                    3u32 => Ok(Self::WheelTilt),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "This specifies the direction of the physical motion that caused a"]
        #[doc = "wl_pointer.axis event, relative to the wl_pointer.axis direction."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AxisRelativeDirection {
            #[doc = "physical motion matches axis direction"]
            Identical = 0u32,
            #[doc = "physical motion is the inverse of the axis direction"]
            Inverted = 1u32,
        }
        impl TryFrom<u32> for AxisRelativeDirection {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Identical),
                    1u32 => Ok(Self::Inverted),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_pointer interface. See the module level documentation for more info"]
        pub trait WlPointer {
            const INTERFACE: &'static str = "wl_pointer";
            const VERSION: u32 = 9u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = "Set the pointer surface, i.e., the surface that contains the"]
            #[doc = "pointer image (cursor). This request gives the surface the role"]
            #[doc = "of a cursor. If the surface already has another role, it raises"]
            #[doc = "a protocol error."]
            #[doc = ""]
            #[doc = "The cursor actually changes only if the pointer"]
            #[doc = "focus for this device is one of the requesting client's surfaces"]
            #[doc = "or the surface parameter is the current pointer surface. If"]
            #[doc = "there was a previous surface set with this request it is"]
            #[doc = "replaced. If surface is NULL, the pointer image is hidden."]
            #[doc = ""]
            #[doc = "The parameters hotspot_x and hotspot_y define the position of"]
            #[doc = "the pointer surface relative to the pointer location. Its"]
            #[doc = "top-left corner is always at (x, y) - (hotspot_x, hotspot_y),"]
            #[doc = "where (x, y) are the coordinates of the pointer location, in"]
            #[doc = "surface-local coordinates."]
            #[doc = ""]
            #[doc = "On wl_surface.offset requests to the pointer surface, hotspot_x"]
            #[doc = "and hotspot_y are decremented by the x and y parameters"]
            #[doc = "passed to the request. The offset must be applied by"]
            #[doc = "wl_surface.commit as usual."]
            #[doc = ""]
            #[doc = "The hotspot can also be updated by passing the currently set"]
            #[doc = "pointer surface to this request with new values for hotspot_x"]
            #[doc = "and hotspot_y."]
            #[doc = ""]
            #[doc = "The input region is ignored for wl_surfaces with the role of"]
            #[doc = "a cursor. When the use as a cursor ends, the wl_surface is"]
            #[doc = "unmapped."]
            #[doc = ""]
            #[doc = "The serial parameter must match the latest wl_pointer.enter"]
            #[doc = "serial number sent to the client. Otherwise the request will be"]
            #[doc = "ignored."]
            async fn set_cursor(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                serial: u32,
                surface: Option<crate::wire::ObjectId>,
                hotspot_x: i32,
                hotspot_y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_pointer#{}.set_cursor()", object_id);
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
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the pointer object anymore."]
            #[doc = ""]
            #[doc = "This request destroys the pointer proxy object, so clients must not call"]
            #[doc = "wl_pointer_destroy() after using this request."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_pointer#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The wl_keyboard interface represents one or more keyboards"]
    #[doc = "associated with a seat."]
    #[doc = ""]
    #[doc = "Each wl_keyboard has the following logical state:"]
    #[doc = ""]
    #[doc = "- an active surface (possibly null),"]
    #[doc = "- the keys currently logically down,"]
    #[doc = "- the active modifiers,"]
    #[doc = "- the active group."]
    #[doc = ""]
    #[doc = "By default, the active surface is null, the keys currently logically down"]
    #[doc = "are empty, the active modifiers and the active group are 0."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_keyboard {
        use futures_util::SinkExt;
        #[doc = "This specifies the format of the keymap provided to the"]
        #[doc = "client with the wl_keyboard.keymap event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum KeymapFormat {
            #[doc = "no keymap; client must understand how to interpret the raw keycode"]
            NoKeymap = 0u32,
            #[doc = "libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode"]
            XkbV1 = 1u32,
        }
        impl TryFrom<u32> for KeymapFormat {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoKeymap),
                    1u32 => Ok(Self::XkbV1),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Describes the physical state of a key that produced the key event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum KeyState {
            #[doc = "key is not pressed"]
            Released = 0u32,
            #[doc = "key is pressed"]
            Pressed = 1u32,
        }
        impl TryFrom<u32> for KeyState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Released),
                    1u32 => Ok(Self::Pressed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_keyboard interface. See the module level documentation for more info"]
        pub trait WlKeyboard {
            const INTERFACE: &'static str = "wl_keyboard";
            const VERSION: u32 = 9u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_keyboard#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "The wl_touch interface represents a touchscreen"]
    #[doc = "associated with a seat."]
    #[doc = ""]
    #[doc = "Touch interactions can consist of one or more contacts."]
    #[doc = "For each contact, a series of events is generated, starting"]
    #[doc = "with a down event, followed by zero or more motion events,"]
    #[doc = "and ending with an up event. Events relating to the same"]
    #[doc = "contact point can be identified by the ID of the sequence."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_touch {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the wl_touch interface. See the module level documentation for more info"]
        pub trait WlTouch {
            const INTERFACE: &'static str = "wl_touch";
            const VERSION: u32 = 9u32;
            async fn handle_event(
                &self,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_touch#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An output describes part of the compositor geometry.  The"]
    #[doc = "compositor works in the 'compositor coordinate system' and an"]
    #[doc = "output corresponds to a rectangular area in that space that is"]
    #[doc = "actually visible.  This typically corresponds to a monitor that"]
    #[doc = "displays part of the compositor space.  This object is published"]
    #[doc = "as global during start up, or when a monitor is hotplugged."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_output {
        use futures_util::SinkExt;
        #[doc = "This enumeration describes how the physical"]
        #[doc = "pixels on an output are laid out."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Subpixel {
            #[doc = "unknown geometry"]
            Unknown = 0u32,
            #[doc = "no geometry"]
            None = 1u32,
            #[doc = "horizontal RGB"]
            HorizontalRgb = 2u32,
            #[doc = "horizontal BGR"]
            HorizontalBgr = 3u32,
            #[doc = "vertical RGB"]
            VerticalRgb = 4u32,
            #[doc = "vertical BGR"]
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
        #[doc = "This describes transformations that clients and compositors apply to"]
        #[doc = "buffer contents."]
        #[doc = ""]
        #[doc = "The flipped values correspond to an initial flip around a"]
        #[doc = "vertical axis followed by rotation."]
        #[doc = ""]
        #[doc = "The purpose is mainly to allow clients to render accordingly and"]
        #[doc = "tell the compositor, so that for fullscreen surfaces, the"]
        #[doc = "compositor will still be able to scan out directly from client"]
        #[doc = "surfaces."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Transform {
            #[doc = "no transform"]
            Normal = 0u32,
            #[doc = "90 degrees counter-clockwise"]
            _90 = 1u32,
            #[doc = "180 degrees counter-clockwise"]
            _180 = 2u32,
            #[doc = "270 degrees counter-clockwise"]
            _270 = 3u32,
            #[doc = "180 degree flip around a vertical axis"]
            Flipped = 4u32,
            #[doc = "flip and rotate 90 degrees counter-clockwise"]
            Flipped90 = 5u32,
            #[doc = "flip and rotate 180 degrees counter-clockwise"]
            Flipped180 = 6u32,
            #[doc = "flip and rotate 270 degrees counter-clockwise"]
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
        bitflags::bitflags! { # [doc = "These flags describe properties of an output mode."] # [doc = "They are used in the flags bitfield of the mode event."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Mode : u32 { # [doc = "indicates this is the current mode"] const Current = 1u32 ; # [doc = "indicates this is the preferred mode"] const Preferred = 2u32 ; } }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the wl_output interface. See the module level documentation for more info"]
        pub trait WlOutput {
            const INTERFACE: &'static str = "wl_output";
            const VERSION: u32 = 4u32;
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
            #[doc = "use the output object anymore."]
            async fn release(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_output#{}.release()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "A region object describes an area."]
    #[doc = ""]
    #[doc = "Region objects are used to describe the opaque and input"]
    #[doc = "regions of a surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_region {
        use futures_util::SinkExt;
        #[doc = "Trait to implement the wl_region interface. See the module level documentation for more info"]
        pub trait WlRegion {
            const INTERFACE: &'static str = "wl_region";
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
            #[doc = "Destroy the region.  This will invalidate the object ID."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_region#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Add the specified rectangle to the region."]
            async fn add(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_region#{}.add()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Subtract the specified rectangle from the region."]
            async fn subtract(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_region#{}.subtract()", object_id);
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
        }
    }
    #[doc = "The global interface exposing sub-surface compositing capabilities."]
    #[doc = "A wl_surface, that has sub-surfaces associated, is called the"]
    #[doc = "parent surface. Sub-surfaces can be arbitrarily nested and create"]
    #[doc = "a tree of sub-surfaces."]
    #[doc = ""]
    #[doc = "The root surface in a tree of sub-surfaces is the main"]
    #[doc = "surface. The main surface cannot be a sub-surface, because"]
    #[doc = "sub-surfaces must always have a parent."]
    #[doc = ""]
    #[doc = "A main surface with its sub-surfaces forms a (compound) window."]
    #[doc = "For window management purposes, this set of wl_surface objects is"]
    #[doc = "to be considered as a single window, and it should also behave as"]
    #[doc = "such."]
    #[doc = ""]
    #[doc = "The aim of sub-surfaces is to offload some of the compositing work"]
    #[doc = "within a window from clients to the compositor. A prime example is"]
    #[doc = "a video player with decorations and video in separate wl_surface"]
    #[doc = "objects. This should allow the compositor to pass YUV video buffer"]
    #[doc = "processing to dedicated overlay hardware when possible."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_subcompositor {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the to-be sub-surface is invalid"]
            BadSurface = 0u32,
            #[doc = "the to-be sub-surface parent is invalid"]
            BadParent = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BadSurface),
                    1u32 => Ok(Self::BadParent),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_subcompositor interface. See the module level documentation for more info"]
        pub trait WlSubcompositor {
            const INTERFACE: &'static str = "wl_subcompositor";
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
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other"]
            #[doc = "objects, wl_subsurface objects included."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subcompositor#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Create a sub-surface interface for the given surface, and"]
            #[doc = "associate it with the given parent surface. This turns a"]
            #[doc = "plain wl_surface into a sub-surface."]
            #[doc = ""]
            #[doc = "The to-be sub-surface must not already have another role, and it"]
            #[doc = "must not have an existing wl_subsurface object. Otherwise the"]
            #[doc = "bad_surface protocol error is raised."]
            #[doc = ""]
            #[doc = "Adding sub-surfaces to a parent is a double-buffered operation on the"]
            #[doc = "parent (see wl_surface.commit). The effect of adding a sub-surface"]
            #[doc = "becomes visible on the next time the state of the parent surface is"]
            #[doc = "applied."]
            #[doc = ""]
            #[doc = "The parent surface must not be one of the child surface's descendants,"]
            #[doc = "and the parent must be different from the child surface, otherwise the"]
            #[doc = "bad_parent protocol error is raised."]
            #[doc = ""]
            #[doc = "This request modifies the behaviour of wl_surface.commit request on"]
            #[doc = "the sub-surface, see the documentation on wl_subsurface interface."]
            async fn get_subsurface(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                parent: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subcompositor#{}.get_subsurface()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(parent))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = "An additional interface to a wl_surface object, which has been"]
    #[doc = "made a sub-surface. A sub-surface has one parent surface. A"]
    #[doc = "sub-surface's size and position are not limited to that of the parent."]
    #[doc = "Particularly, a sub-surface is not automatically clipped to its"]
    #[doc = "parent's area."]
    #[doc = ""]
    #[doc = "A sub-surface becomes mapped, when a non-NULL wl_buffer is applied"]
    #[doc = "and the parent surface is mapped. The order of which one happens"]
    #[doc = "first is irrelevant. A sub-surface is hidden if the parent becomes"]
    #[doc = "hidden, or if a NULL wl_buffer is applied. These rules apply"]
    #[doc = "recursively through the tree of surfaces."]
    #[doc = ""]
    #[doc = "The behaviour of a wl_surface.commit request on a sub-surface"]
    #[doc = "depends on the sub-surface's mode. The possible modes are"]
    #[doc = "synchronized and desynchronized, see methods"]
    #[doc = "wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized"]
    #[doc = "mode caches the wl_surface state to be applied when the parent's"]
    #[doc = "state gets applied, and desynchronized mode applies the pending"]
    #[doc = "wl_surface state directly. A sub-surface is initially in the"]
    #[doc = "synchronized mode."]
    #[doc = ""]
    #[doc = "Sub-surfaces also have another kind of state, which is managed by"]
    #[doc = "wl_subsurface requests, as opposed to wl_surface requests. This"]
    #[doc = "state includes the sub-surface position relative to the parent"]
    #[doc = "surface (wl_subsurface.set_position), and the stacking order of"]
    #[doc = "the parent and its sub-surfaces (wl_subsurface.place_above and"]
    #[doc = ".place_below). This state is applied when the parent surface's"]
    #[doc = "wl_surface state is applied, regardless of the sub-surface's mode."]
    #[doc = "As the exception, set_sync and set_desync are effective immediately."]
    #[doc = ""]
    #[doc = "The main surface can be thought to be always in desynchronized mode,"]
    #[doc = "since it does not have a parent in the sub-surfaces sense."]
    #[doc = ""]
    #[doc = "Even if a sub-surface is in desynchronized mode, it will behave as"]
    #[doc = "in synchronized mode, if its parent surface behaves as in"]
    #[doc = "synchronized mode. This rule is applied recursively throughout the"]
    #[doc = "tree of surfaces. This means, that one can set a sub-surface into"]
    #[doc = "synchronized mode, and then assume that all its child and grand-child"]
    #[doc = "sub-surfaces are synchronized, too, without explicitly setting them."]
    #[doc = ""]
    #[doc = "Destroying a sub-surface takes effect immediately. If you need to"]
    #[doc = "synchronize the removal of a sub-surface to the parent surface update,"]
    #[doc = "unmap the sub-surface first by attaching a NULL wl_buffer, update parent,"]
    #[doc = "and then destroy the sub-surface."]
    #[doc = ""]
    #[doc = "If the parent wl_surface object is destroyed, the sub-surface is"]
    #[doc = "unmapped."]
    #[doc = ""]
    #[doc = "A sub-surface never has the keyboard focus of any seat."]
    #[doc = ""]
    #[doc = "The wl_surface.offset request is ignored: clients must use set_position"]
    #[doc = "instead to move the sub-surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_subsurface {
        use futures_util::SinkExt;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface is not a sibling or the parent"]
            BadSurface = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BadSurface),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wl_subsurface interface. See the module level documentation for more info"]
        pub trait WlSubsurface {
            const INTERFACE: &'static str = "wl_subsurface";
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
            #[doc = "The sub-surface interface is removed from the wl_surface object"]
            #[doc = "that was turned into a sub-surface with a"]
            #[doc = "wl_subcompositor.get_subsurface request. The wl_surface's association"]
            #[doc = "to the parent is deleted. The wl_surface is unmapped immediately."]
            async fn destroy(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subsurface#{}.destroy()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This schedules a sub-surface position change."]
            #[doc = "The sub-surface will be moved so that its origin (top left"]
            #[doc = "corner pixel) will be at the location x, y of the parent surface"]
            #[doc = "coordinate system. The coordinates are not restricted to the parent"]
            #[doc = "surface area. Negative values are allowed."]
            #[doc = ""]
            #[doc = "The scheduled coordinates will take effect whenever the state of the"]
            #[doc = "parent surface is applied."]
            #[doc = ""]
            #[doc = "If more than one set_position request is invoked by the client before"]
            #[doc = "the commit of the parent surface, the position of a new request always"]
            #[doc = "replaces the scheduled position from any previous request."]
            #[doc = ""]
            #[doc = "The initial position is 0, 0."]
            async fn set_position(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subsurface#{}.set_position()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "This sub-surface is taken from the stack, and put back just"]
            #[doc = "above the reference surface, changing the z-order of the sub-surfaces."]
            #[doc = "The reference surface must be one of the sibling surfaces, or the"]
            #[doc = "parent surface. Using any other surface, including this sub-surface,"]
            #[doc = "will cause a protocol error."]
            #[doc = ""]
            #[doc = "The z-order is double-buffered. Requests are handled in order and"]
            #[doc = "applied immediately to a pending state. The final pending state is"]
            #[doc = "copied to the active state the next time the state of the parent"]
            #[doc = "surface is applied."]
            #[doc = ""]
            #[doc = "A new sub-surface is initially added as the top-most in the stack"]
            #[doc = "of its siblings and parent."]
            async fn place_above(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                sibling: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subsurface#{}.place_above()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(sibling))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "The sub-surface is placed just below the reference surface."]
            #[doc = "See wl_subsurface.place_above."]
            async fn place_below(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
                sibling: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subsurface#{}.place_below()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(sibling))
                    .build();
                socket
                    .send(crate::wire::Message::new(object_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Change the commit behaviour of the sub-surface to synchronized"]
            #[doc = "mode, also described as the parent dependent mode."]
            #[doc = ""]
            #[doc = "In synchronized mode, wl_surface.commit on a sub-surface will"]
            #[doc = "accumulate the committed state in a cache, but the state will"]
            #[doc = "not be applied and hence will not change the compositor output."]
            #[doc = "The cached state is applied to the sub-surface immediately after"]
            #[doc = "the parent surface's state is applied. This ensures atomic"]
            #[doc = "updates of the parent and all its synchronized sub-surfaces."]
            #[doc = "Applying the cached state will invalidate the cache, so further"]
            #[doc = "parent surface commits do not (re-)apply old state."]
            #[doc = ""]
            #[doc = "See wl_subsurface for the recursive effect of this mode."]
            async fn set_sync(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subsurface#{}.set_sync()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = "Change the commit behaviour of the sub-surface to desynchronized"]
            #[doc = "mode, also described as independent or freely running mode."]
            #[doc = ""]
            #[doc = "In desynchronized mode, wl_surface.commit on a sub-surface will"]
            #[doc = "apply the pending state directly, without caching, as happens"]
            #[doc = "normally with a wl_surface. Calling wl_surface.commit on the"]
            #[doc = "parent surface has no effect on the sub-surface's wl_surface"]
            #[doc = "state. This mode allows a sub-surface to be updated on its own."]
            #[doc = ""]
            #[doc = "If cached state exists when wl_surface.commit is called in"]
            #[doc = "desynchronized mode, the pending state is added to the cached"]
            #[doc = "state, and applied as a whole. This invalidates the cache."]
            #[doc = ""]
            #[doc = "Note: even if a sub-surface is set to desynchronized, a parent"]
            #[doc = "sub-surface may override it to behave as synchronized. For details,"]
            #[doc = "see wl_subsurface."]
            #[doc = ""]
            #[doc = "If a surface's parent surface behaves as desynchronized, then"]
            #[doc = "the cached state is applied on set_desync."]
            async fn set_desync(
                &self,
                socket: &mut crate::wire::Socket,
                object_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wl_subsurface#{}.set_desync()", object_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                socket
                    .send(crate::wire::Message::new(object_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
