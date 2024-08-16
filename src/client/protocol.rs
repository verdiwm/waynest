#![allow(async_fn_in_trait)]
pub mod wayland {
    #[doc = "The core global object.  This is a special singleton object.  It"]
    #[doc = "is used for internal Wayland protocol features."]
    pub mod wl_display {
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
        pub trait WlDisplay: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_display";
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
    pub mod wl_registry {
        #[doc = "Trait to implement the wl_registry interface. See the module level documentation for more info"]
        pub trait WlRegistry: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_registry";
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
    #[doc = "Clients can handle the 'done' event to get notified when"]
    #[doc = "the related request is done."]
    #[doc = ""]
    #[doc = "Note, because wl_callback objects are created from multiple independent"]
    #[doc = "factory interfaces, the wl_callback interface is frozen at version 1."]
    pub mod wl_callback {
        #[doc = "Trait to implement the wl_callback interface. See the module level documentation for more info"]
        pub trait WlCallback: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_callback";
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
    #[doc = "A compositor.  This object is a singleton global.  The"]
    #[doc = "compositor is in charge of combining the contents of multiple"]
    #[doc = "surfaces into one displayable output."]
    pub mod wl_compositor {
        #[doc = "Trait to implement the wl_compositor interface. See the module level documentation for more info"]
        pub trait WlCompositor: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_compositor";
            const VERSION: u32 = 6u32;
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
    #[doc = "The wl_shm_pool object encapsulates a piece of memory shared"]
    #[doc = "between the compositor and client.  Through the wl_shm_pool"]
    #[doc = "object, the client can allocate shared memory wl_buffer objects."]
    #[doc = "All objects created through the same pool share the same"]
    #[doc = "underlying mapped memory. Reusing the mapped memory avoids the"]
    #[doc = "setup/teardown overhead and is useful when interactively resizing"]
    #[doc = "a surface or for many small buffers."]
    pub mod wl_shm_pool {
        #[doc = "Trait to implement the wl_shm_pool interface. See the module level documentation for more info"]
        pub trait WlShmPool: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_shm_pool";
            const VERSION: u32 = 2u32;
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
    #[doc = "A singleton global object that provides support for shared"]
    #[doc = "memory."]
    #[doc = ""]
    #[doc = "Clients can create wl_shm_pool objects using the create_pool"]
    #[doc = "request."]
    #[doc = ""]
    #[doc = "On binding the wl_shm object one or more format events"]
    #[doc = "are emitted to inform clients about the valid pixel formats"]
    #[doc = "that can be used for buffers."]
    pub mod wl_shm {
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
        pub trait WlShm: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_shm";
            const VERSION: u32 = 2u32;
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
    pub mod wl_buffer {
        #[doc = "Trait to implement the wl_buffer interface. See the module level documentation for more info"]
        pub trait WlBuffer: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_buffer";
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
    #[doc = "A wl_data_offer represents a piece of data offered for transfer"]
    #[doc = "by another client (the source client).  It is used by the"]
    #[doc = "copy-and-paste and drag-and-drop mechanisms.  The offer"]
    #[doc = "describes the different mime types that the data can be"]
    #[doc = "converted to and provides the mechanism for transferring the"]
    #[doc = "data directly from the source client."]
    pub mod wl_data_offer {
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
        pub trait WlDataOffer: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_data_offer";
            const VERSION: u32 = 3u32;
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
    #[doc = "The wl_data_source object is the source side of a wl_data_offer."]
    #[doc = "It is created by the source client in a data transfer and"]
    #[doc = "provides a way to describe the offered data and a way to respond"]
    #[doc = "to requests to transfer the data."]
    pub mod wl_data_source {
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
        pub trait WlDataSource: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_data_source";
            const VERSION: u32 = 3u32;
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
    #[doc = "There is one wl_data_device per seat which can be obtained"]
    #[doc = "from the global wl_data_device_manager singleton."]
    #[doc = ""]
    #[doc = "A wl_data_device provides access to inter-client data transfer"]
    #[doc = "mechanisms such as copy-and-paste and drag-and-drop."]
    pub mod wl_data_device {
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
        pub trait WlDataDevice: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_data_device";
            const VERSION: u32 = 3u32;
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
    pub mod wl_data_device_manager {
        bitflags::bitflags! { # [doc = "This is a bitmask of the available/preferred actions in a"] # [doc = "drag-and-drop operation."] # [doc = ""] # [doc = "In the compositor, the selected action is a result of matching the"] # [doc = "actions offered by the source and destination sides.  \"action\" events"] # [doc = "with a \"none\" action will be sent to both source and destination if"] # [doc = "there is no match. All further checks will effectively happen on"] # [doc = "(source actions ∩ destination actions)."] # [doc = ""] # [doc = "In addition, compositors may also pick different actions in"] # [doc = "reaction to key modifiers being pressed. One common design that"] # [doc = "is used in major toolkits (and the behavior recommended for"] # [doc = "compositors) is:"] # [doc = ""] # [doc = "- If no modifiers are pressed, the first match (in bit order)"] # [doc = "will be used."] # [doc = "- Pressing Shift selects \"move\", if enabled in the mask."] # [doc = "- Pressing Control selects \"copy\", if enabled in the mask."] # [doc = ""] # [doc = "Behavior beyond that is considered implementation-dependent."] # [doc = "Compositors may for example bind other modifiers (like Alt/Meta)"] # [doc = "or drags initiated with other buttons than BTN_LEFT to specific"] # [doc = "actions (e.g. \"ask\")."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct DndAction : u32 { # [doc = "no action"] const None = 0u32 ; # [doc = "copy action"] const Copy = 1u32 ; # [doc = "move action"] const Move = 2u32 ; # [doc = "ask action"] const Ask = 4u32 ; } }
        impl TryFrom<u32> for DndAction {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the wl_data_device_manager interface. See the module level documentation for more info"]
        pub trait WlDataDeviceManager: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_data_device_manager";
            const VERSION: u32 = 3u32;
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
    #[doc = "This interface is implemented by servers that provide"]
    #[doc = "desktop-style user interfaces."]
    #[doc = ""]
    #[doc = "It allows clients to associate a wl_shell_surface with"]
    #[doc = "a basic surface."]
    #[doc = ""]
    #[doc = "Note! This protocol is deprecated and not intended for production use."]
    #[doc = "For desktop-style user interfaces, use xdg_shell. Compositors and clients"]
    #[doc = "should not implement this interface."]
    pub mod wl_shell {
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
        pub trait WlShell: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_shell";
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
    pub mod wl_shell_surface {
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
        pub trait WlShellSurface: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_shell_surface";
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
    pub mod wl_surface {
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
        pub trait WlSurface: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_surface";
            const VERSION: u32 = 6u32;
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
    #[doc = "A seat is a group of keyboards, pointer and touch devices. This"]
    #[doc = "object is published as a global during start up, or when such a"]
    #[doc = "device is hot plugged.  A seat typically has a pointer and"]
    #[doc = "maintains a keyboard focus and a pointer focus."]
    pub mod wl_seat {
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
        pub trait WlSeat: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_seat";
            const VERSION: u32 = 9u32;
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
    #[doc = "The wl_pointer interface represents one or more input devices,"]
    #[doc = "such as mice, which control the pointer location and pointer_focus"]
    #[doc = "of a seat."]
    #[doc = ""]
    #[doc = "The wl_pointer interface generates motion, enter and leave"]
    #[doc = "events for the surfaces that the pointer is located over,"]
    #[doc = "and button and axis events for button presses, button releases"]
    #[doc = "and scrolling."]
    pub mod wl_pointer {
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
        pub trait WlPointer: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_pointer";
            const VERSION: u32 = 9u32;
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
    pub mod wl_keyboard {
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
        pub trait WlKeyboard: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_keyboard";
            const VERSION: u32 = 9u32;
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
    #[doc = "The wl_touch interface represents a touchscreen"]
    #[doc = "associated with a seat."]
    #[doc = ""]
    #[doc = "Touch interactions can consist of one or more contacts."]
    #[doc = "For each contact, a series of events is generated, starting"]
    #[doc = "with a down event, followed by zero or more motion events,"]
    #[doc = "and ending with an up event. Events relating to the same"]
    #[doc = "contact point can be identified by the ID of the sequence."]
    pub mod wl_touch {
        #[doc = "Trait to implement the wl_touch interface. See the module level documentation for more info"]
        pub trait WlTouch: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_touch";
            const VERSION: u32 = 9u32;
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
    #[doc = "An output describes part of the compositor geometry.  The"]
    #[doc = "compositor works in the 'compositor coordinate system' and an"]
    #[doc = "output corresponds to a rectangular area in that space that is"]
    #[doc = "actually visible.  This typically corresponds to a monitor that"]
    #[doc = "displays part of the compositor space.  This object is published"]
    #[doc = "as global during start up, or when a monitor is hotplugged."]
    pub mod wl_output {
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
        pub trait WlOutput: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_output";
            const VERSION: u32 = 4u32;
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
    #[doc = "A region object describes an area."]
    #[doc = ""]
    #[doc = "Region objects are used to describe the opaque and input"]
    #[doc = "regions of a surface."]
    pub mod wl_region {
        #[doc = "Trait to implement the wl_region interface. See the module level documentation for more info"]
        pub trait WlRegion: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_region";
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
    pub mod wl_subcompositor {
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
        pub trait WlSubcompositor: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_subcompositor";
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
    pub mod wl_subsurface {
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
        pub trait WlSubsurface: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wl_subsurface";
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
        #[doc = "Trait to implement the zwp_linux_dmabuf_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufV1: crate::client::Dispatcher {
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
        pub trait ZwpLinuxBufferParamsV1: crate::client::Dispatcher {
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
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct TrancheFlags : u32 { # [doc = "direct scan-out tranche"] const Scanout = 1u32 ; } }
        impl TryFrom<u32> for TrancheFlags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the zwp_linux_dmabuf_feedback_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufFeedbackV1: crate::client::Dispatcher {
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
        pub trait WpPresentation: crate::client::Dispatcher {
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
        bitflags::bitflags! { # [doc = "These flags provide information about how the presentation of"] # [doc = "the related content update was done. The intent is to help"] # [doc = "clients assess the reliability of the feedback and the visual"] # [doc = "quality with respect to possible tearing and timings."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Kind : u32 { const Vsync = 1u32 ; const HwClock = 2u32 ; const HwCompletion = 4u32 ; const ZeroCopy = 8u32 ; } }
        impl TryFrom<u32> for Kind {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the wp_presentation_feedback interface. See the module level documentation for more info"]
        pub trait WpPresentationFeedback: crate::client::Dispatcher {
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
        #[doc = "Trait to implement the zwp_tablet_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV2: crate::client::Dispatcher {
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
        }
    }
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "seat. After binding to this interface, the compositor sends a set of"]
    #[doc = "wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."]
    pub mod zwp_tablet_seat_v2 {
        #[doc = "Trait to implement the zwp_tablet_seat_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV2: crate::client::Dispatcher {
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
        pub trait ZwpTabletToolV2: crate::client::Dispatcher {
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
        #[doc = "Trait to implement the zwp_tablet_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV2: crate::client::Dispatcher {
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
        }
    }
    #[doc = "A circular interaction area, such as the touch ring on the Wacom Intuos"]
    #[doc = "Pro series tablets."]
    #[doc = ""]
    #[doc = "Events on a ring are logically grouped by the wl_tablet_pad_ring.frame"]
    #[doc = "event."]
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_ring_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadRingV2: crate::client::Dispatcher {
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
        }
    }
    #[doc = "A linear interaction area, such as the strips found in Wacom Cintiq"]
    #[doc = "models."]
    #[doc = ""]
    #[doc = "Events on a strip are logically grouped by the wl_tablet_pad_strip.frame"]
    #[doc = "event."]
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_strip_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadStripV2: crate::client::Dispatcher {
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
        #[doc = "Trait to implement the zwp_tablet_pad_group_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadGroupV2: crate::client::Dispatcher {
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
        pub trait ZwpTabletPadV2: crate::client::Dispatcher {
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
        pub trait WpViewporter: crate::client::Dispatcher {
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
    #[doc = "This state is double-buffered, and is applied on the next"]
    #[doc = "wl_surface.commit."]
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
        pub trait WpViewport: crate::client::Dispatcher {
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
        pub trait XdgWmBase: crate::client::Dispatcher {
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
        pub trait XdgPositioner: crate::client::Dispatcher {
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
        pub trait XdgSurface: crate::client::Dispatcher {
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
    #[doc = "can re-map the toplevel by perfoming a commit without any buffer"]
    #[doc = "attached, waiting for a configure event and handling it as usual (see"]
    #[doc = "xdg_surface description)."]
    #[doc = ""]
    #[doc = "Attaching a null buffer to a toplevel unmaps the surface."]
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
        #[doc = "States set in this way are double-buffered. They will get applied on"]
        #[doc = "the next commit."]
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
        pub trait XdgToplevel: crate::client::Dispatcher {
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
        pub trait XdgPopup: crate::client::Dispatcher {
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
        }
    }
}
pub mod alpha_modifier_v1 {
    #[doc = "This interface allows a client to set a factor for the alpha values on a"]
    #[doc = "surface, which can be used to offload such operations to the compositor,"]
    #[doc = "which can in turn for example offload them to KMS."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_alpha_modifier_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface already has a alpha modifier object"]
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
        #[doc = "Trait to implement the wp_alpha_modifier_v1 interface. See the module level documentation for more info"]
        pub trait WpAlphaModifierV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_alpha_modifier_v1";
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
    #[doc = "This interface allows the client to set a factor for the alpha values on"]
    #[doc = "a surface, which can be used to offload such operations to the compositor."]
    #[doc = "The default factor is UINT32_MAX."]
    #[doc = ""]
    #[doc = "This object has to be destroyed before the associated wl_surface. Once the"]
    #[doc = "wl_surface is destroyed, all request on this object will raise the"]
    #[doc = "no_surface error."]
    pub mod wp_alpha_modifier_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface was destroyed"]
            NoSurface = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoSurface),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_alpha_modifier_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpAlphaModifierSurfaceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_alpha_modifier_surface_v1";
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
pub mod content_type_v1 {
    #[doc = "This interface allows a client to describe the kind of content a surface"]
    #[doc = "will display, to allow the compositor to optimize its behavior for it."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_content_type_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface already has a content type object"]
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
        #[doc = "Trait to implement the wp_content_type_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpContentTypeManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_content_type_manager_v1";
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
    #[doc = "The content type object allows the compositor to optimize for the kind"]
    #[doc = "of content shown on the surface. A compositor may for example use it to"]
    #[doc = "set relevant drm properties like \"content type\"."]
    #[doc = ""]
    #[doc = "The client may request to switch to another content type at any time."]
    #[doc = "When the associated surface gets destroyed, this object becomes inert and"]
    #[doc = "the client should destroy it."]
    pub mod wp_content_type_v1 {
        #[doc = "These values describe the available content types for a surface."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Type {
            None = 0u32,
            Photo = 1u32,
            Video = 2u32,
            Game = 3u32,
        }
        impl TryFrom<u32> for Type {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Photo),
                    2u32 => Ok(Self::Video),
                    3u32 => Ok(Self::Game),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_content_type_v1 interface. See the module level documentation for more info"]
        pub trait WpContentTypeV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_content_type_v1";
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
pub mod cursor_shape_v1 {
    #[doc = "This global offers an alternative, optional way to set cursor images. This"]
    #[doc = "new way uses enumerated cursors instead of a wl_surface like"]
    #[doc = "wl_pointer.set_cursor does."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_cursor_shape_manager_v1 {
        #[doc = "Trait to implement the wp_cursor_shape_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpCursorShapeManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_cursor_shape_manager_v1";
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
    #[doc = "This interface allows clients to set the cursor shape."]
    pub mod wp_cursor_shape_device_v1 {
        #[doc = "This enum describes cursor shapes."]
        #[doc = ""]
        #[doc = "The names are taken from the CSS W3C specification:"]
        #[doc = "https://w3c.github.io/csswg-drafts/css-ui/#cursor"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Shape {
            #[doc = "default cursor"]
            Default = 1u32,
            #[doc = "a context menu is available for the object under the cursor"]
            ContextMenu = 2u32,
            #[doc = "help is available for the object under the cursor"]
            Help = 3u32,
            #[doc = "pointer that indicates a link or another interactive element"]
            Pointer = 4u32,
            #[doc = "progress indicator"]
            Progress = 5u32,
            #[doc = "program is busy, user should wait"]
            Wait = 6u32,
            #[doc = "a cell or set of cells may be selected"]
            Cell = 7u32,
            #[doc = "simple crosshair"]
            Crosshair = 8u32,
            #[doc = "text may be selected"]
            Text = 9u32,
            #[doc = "vertical text may be selected"]
            VerticalText = 10u32,
            #[doc = "drag-and-drop: alias of/shortcut to something is to be created"]
            Alias = 11u32,
            #[doc = "drag-and-drop: something is to be copied"]
            Copy = 12u32,
            #[doc = "drag-and-drop: something is to be moved"]
            Move = 13u32,
            #[doc = "drag-and-drop: the dragged item cannot be dropped at the current cursor location"]
            NoDrop = 14u32,
            #[doc = "drag-and-drop: the requested action will not be carried out"]
            NotAllowed = 15u32,
            #[doc = "drag-and-drop: something can be grabbed"]
            Grab = 16u32,
            #[doc = "drag-and-drop: something is being grabbed"]
            Grabbing = 17u32,
            #[doc = "resizing: the east border is to be moved"]
            EResize = 18u32,
            #[doc = "resizing: the north border is to be moved"]
            NResize = 19u32,
            #[doc = "resizing: the north-east corner is to be moved"]
            NeResize = 20u32,
            #[doc = "resizing: the north-west corner is to be moved"]
            NwResize = 21u32,
            #[doc = "resizing: the south border is to be moved"]
            SResize = 22u32,
            #[doc = "resizing: the south-east corner is to be moved"]
            SeResize = 23u32,
            #[doc = "resizing: the south-west corner is to be moved"]
            SwResize = 24u32,
            #[doc = "resizing: the west border is to be moved"]
            WResize = 25u32,
            #[doc = "resizing: the east and west borders are to be moved"]
            EwResize = 26u32,
            #[doc = "resizing: the north and south borders are to be moved"]
            NsResize = 27u32,
            #[doc = "resizing: the north-east and south-west corners are to be moved"]
            NeswResize = 28u32,
            #[doc = "resizing: the north-west and south-east corners are to be moved"]
            NwseResize = 29u32,
            #[doc = "resizing: that the item/column can be resized horizontally"]
            ColResize = 30u32,
            #[doc = "resizing: that the item/row can be resized vertically"]
            RowResize = 31u32,
            #[doc = "something can be scrolled in any direction"]
            AllScroll = 32u32,
            #[doc = "something can be zoomed in"]
            ZoomIn = 33u32,
            #[doc = "something can be zoomed out"]
            ZoomOut = 34u32,
        }
        impl TryFrom<u32> for Shape {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Default),
                    2u32 => Ok(Self::ContextMenu),
                    3u32 => Ok(Self::Help),
                    4u32 => Ok(Self::Pointer),
                    5u32 => Ok(Self::Progress),
                    6u32 => Ok(Self::Wait),
                    7u32 => Ok(Self::Cell),
                    8u32 => Ok(Self::Crosshair),
                    9u32 => Ok(Self::Text),
                    10u32 => Ok(Self::VerticalText),
                    11u32 => Ok(Self::Alias),
                    12u32 => Ok(Self::Copy),
                    13u32 => Ok(Self::Move),
                    14u32 => Ok(Self::NoDrop),
                    15u32 => Ok(Self::NotAllowed),
                    16u32 => Ok(Self::Grab),
                    17u32 => Ok(Self::Grabbing),
                    18u32 => Ok(Self::EResize),
                    19u32 => Ok(Self::NResize),
                    20u32 => Ok(Self::NeResize),
                    21u32 => Ok(Self::NwResize),
                    22u32 => Ok(Self::SResize),
                    23u32 => Ok(Self::SeResize),
                    24u32 => Ok(Self::SwResize),
                    25u32 => Ok(Self::WResize),
                    26u32 => Ok(Self::EwResize),
                    27u32 => Ok(Self::NsResize),
                    28u32 => Ok(Self::NeswResize),
                    29u32 => Ok(Self::NwseResize),
                    30u32 => Ok(Self::ColResize),
                    31u32 => Ok(Self::RowResize),
                    32u32 => Ok(Self::AllScroll),
                    33u32 => Ok(Self::ZoomIn),
                    34u32 => Ok(Self::ZoomOut),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the specified shape value is invalid"]
            InvalidShape = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidShape),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_cursor_shape_device_v1 interface. See the module level documentation for more info"]
        pub trait WpCursorShapeDeviceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_cursor_shape_device_v1";
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
pub mod drm_lease_v1 {
    #[doc = "This protocol is used by Wayland compositors which act as Direct"]
    #[doc = "Rendering Manager (DRM) masters to lease DRM resources to Wayland"]
    #[doc = "clients."]
    #[doc = ""]
    #[doc = "The compositor will advertise one wp_drm_lease_device_v1 global for each"]
    #[doc = "DRM node. Some time after a client binds to the wp_drm_lease_device_v1"]
    #[doc = "global, the compositor will send a drm_fd event followed by zero, one or"]
    #[doc = "more connector events. After all currently available connectors have been"]
    #[doc = "sent, the compositor will send a wp_drm_lease_device_v1.done event."]
    #[doc = ""]
    #[doc = "When the list of connectors available for lease changes the compositor"]
    #[doc = "will send wp_drm_lease_device_v1.connector events for added connectors and"]
    #[doc = "wp_drm_lease_connector_v1.withdrawn events for removed connectors,"]
    #[doc = "followed by a wp_drm_lease_device_v1.done event."]
    #[doc = ""]
    #[doc = "The compositor will indicate when a device is gone by removing the global"]
    #[doc = "via a wl_registry.global_remove event. Upon receiving this event, the"]
    #[doc = "client should destroy any matching wp_drm_lease_device_v1 object."]
    #[doc = ""]
    #[doc = "To destroy a wp_drm_lease_device_v1 object, the client must first issue"]
    #[doc = "a release request. Upon receiving this request, the compositor will"]
    #[doc = "immediately send a released event and destroy the object. The client must"]
    #[doc = "continue to process and discard drm_fd and connector events until it"]
    #[doc = "receives the released event. Upon receiving the released event, the"]
    #[doc = "client can safely cleanup any client-side resources."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_drm_lease_device_v1 {
        #[doc = "Trait to implement the wp_drm_lease_device_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseDeviceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_device_v1";
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
    #[doc = "Represents a DRM connector which is available for lease. These objects are"]
    #[doc = "created via wp_drm_lease_device_v1.connector events, and should be passed"]
    #[doc = "to lease requests via wp_drm_lease_request_v1.request_connector."]
    #[doc = "Immediately after the wp_drm_lease_connector_v1 object is created the"]
    #[doc = "compositor will send a name, a description, a connector_id and a done"]
    #[doc = "event. When the description is updated the compositor will send a"]
    #[doc = "description event followed by a done event."]
    pub mod wp_drm_lease_connector_v1 {
        #[doc = "Trait to implement the wp_drm_lease_connector_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseConnectorV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_connector_v1";
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
    #[doc = "A client that wishes to lease DRM resources will attach the list of"]
    #[doc = "connectors advertised with wp_drm_lease_device_v1.connector that they"]
    #[doc = "wish to lease, then use wp_drm_lease_request_v1.submit to submit the"]
    #[doc = "request."]
    pub mod wp_drm_lease_request_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "requested a connector from a different lease device"]
            WrongDevice = 0u32,
            #[doc = "requested a connector twice"]
            DuplicateConnector = 1u32,
            #[doc = "requested a lease without requesting a connector"]
            EmptyLease = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::WrongDevice),
                    1u32 => Ok(Self::DuplicateConnector),
                    2u32 => Ok(Self::EmptyLease),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_drm_lease_request_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseRequestV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_request_v1";
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
    #[doc = "A DRM lease object is used to transfer the DRM file descriptor to the"]
    #[doc = "client and manage the lifetime of the lease."]
    #[doc = ""]
    #[doc = "Some time after the wp_drm_lease_v1 object is created, the compositor"]
    #[doc = "will reply with the lease request's result. If the lease request is"]
    #[doc = "granted, the compositor will send a lease_fd event. If the lease request"]
    #[doc = "is denied, the compositor will send a finished event without a lease_fd"]
    #[doc = "event."]
    pub mod wp_drm_lease_v1 {
        #[doc = "Trait to implement the wp_drm_lease_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_drm_lease_v1";
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
#[doc = "The purpose of this protocol is to provide protocol object handles for"]
#[doc = "toplevels, possibly originating from another client."]
#[doc = ""]
#[doc = "This protocol is intentionally minimalistic and expects additional"]
#[doc = "functionality (e.g. creating a screencopy source from a toplevel handle,"]
#[doc = "getting information about the state of the toplevel) to be implemented"]
#[doc = "in extension protocols."]
#[doc = ""]
#[doc = "The compositor may choose to restrict this protocol to a special client"]
#[doc = "launched by the compositor itself or expose it to all clients,"]
#[doc = "this is compositor policy."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\",  \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod ext_foreign_toplevel_list_v1 {
    #[doc = "A toplevel is defined as a surface with a role similar to xdg_toplevel."]
    #[doc = "XWayland surfaces may be treated like toplevels in this protocol."]
    #[doc = ""]
    #[doc = "After a client binds the ext_foreign_toplevel_list_v1, each mapped"]
    #[doc = "toplevel window will be sent using the ext_foreign_toplevel_list_v1.toplevel"]
    #[doc = "event."]
    #[doc = ""]
    #[doc = "Clients which only care about the current state can perform a roundtrip after"]
    #[doc = "binding this global."]
    #[doc = ""]
    #[doc = "For each instance of ext_foreign_toplevel_list_v1, the compositor must"]
    #[doc = "create a new ext_foreign_toplevel_handle_v1 object for each mapped toplevel."]
    #[doc = ""]
    #[doc = "If a compositor implementation sends the ext_foreign_toplevel_list_v1.finished"]
    #[doc = "event after the global is bound, the compositor must not send any"]
    #[doc = "ext_foreign_toplevel_list_v1.toplevel events."]
    pub mod ext_foreign_toplevel_list_v1 {
        #[doc = "Trait to implement the ext_foreign_toplevel_list_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelListV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_foreign_toplevel_list_v1";
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
    #[doc = "A ext_foreign_toplevel_handle_v1 object represents a mapped toplevel"]
    #[doc = "window. A single app may have multiple mapped toplevels."]
    pub mod ext_foreign_toplevel_handle_v1 {
        #[doc = "Trait to implement the ext_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelHandleV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_foreign_toplevel_handle_v1";
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
pub mod ext_idle_notify_v1 {
    #[doc = "This interface allows clients to monitor user idle status."]
    #[doc = ""]
    #[doc = "After binding to this global, clients can create ext_idle_notification_v1"]
    #[doc = "objects to get notified when the user is idle for a given amount of time."]
    pub mod ext_idle_notifier_v1 {
        #[doc = "Trait to implement the ext_idle_notifier_v1 interface. See the module level documentation for more info"]
        pub trait ExtIdleNotifierV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_idle_notifier_v1";
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
    #[doc = "This interface is used by the compositor to send idle notification events"]
    #[doc = "to clients."]
    #[doc = ""]
    #[doc = "Initially the notification object is not idle. The notification object"]
    #[doc = "becomes idle when no user activity has happened for at least the timeout"]
    #[doc = "duration, starting from the creation of the notification object. User"]
    #[doc = "activity may include input events or a presence sensor, but is"]
    #[doc = "compositor-specific. If an idle inhibitor is active (e.g. another client"]
    #[doc = "has created a zwp_idle_inhibitor_v1 on a visible surface), the compositor"]
    #[doc = "must not make the notification object idle."]
    #[doc = ""]
    #[doc = "When the notification object becomes idle, an idled event is sent. When"]
    #[doc = "user activity starts again, the notification object stops being idle,"]
    #[doc = "a resumed event is sent and the timeout is restarted."]
    pub mod ext_idle_notification_v1 {
        #[doc = "Trait to implement the ext_idle_notification_v1 interface. See the module level documentation for more info"]
        pub trait ExtIdleNotificationV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_idle_notification_v1";
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
#[doc = "This protocol allows for a privileged Wayland client to lock the session"]
#[doc = "and display arbitrary graphics while the session is locked."]
#[doc = ""]
#[doc = "The compositor may choose to restrict this protocol to a special client"]
#[doc = "launched by the compositor itself or expose it to all privileged clients,"]
#[doc = "this is compositor policy."]
#[doc = ""]
#[doc = "The client is responsible for performing authentication and informing the"]
#[doc = "compositor when the session should be unlocked. If the client dies while"]
#[doc = "the session is locked the session remains locked, possibly permanently"]
#[doc = "depending on compositor policy."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\",  \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the"]
#[doc = "testing phase. Backward compatible changes may be added together with"]
#[doc = "the corresponding interface version bump. Backward incompatible changes"]
#[doc = "can only be done by creating a new major version of the extension."]
pub mod ext_session_lock_v1 {
    #[doc = "This interface is used to request that the session be locked."]
    pub mod ext_session_lock_manager_v1 {
        #[doc = "Trait to implement the ext_session_lock_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_session_lock_manager_v1";
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
    #[doc = "In response to the creation of this object the compositor must send"]
    #[doc = "either the locked or finished event."]
    #[doc = ""]
    #[doc = "The locked event indicates that the session is locked. This means"]
    #[doc = "that the compositor must stop rendering and providing input to normal"]
    #[doc = "clients. Instead the compositor must blank all outputs with an opaque"]
    #[doc = "color such that their normal content is fully hidden."]
    #[doc = ""]
    #[doc = "The only surfaces that should be rendered while the session is locked"]
    #[doc = "are the lock surfaces created through this interface and optionally,"]
    #[doc = "at the compositor's discretion, special privileged surfaces such as"]
    #[doc = "input methods or portions of desktop shell UIs."]
    #[doc = ""]
    #[doc = "The locked event must not be sent until a new \"locked\" frame (either"]
    #[doc = "from a session lock surface or the compositor blanking the output) has"]
    #[doc = "been presented on all outputs and no security sensitive normal/unlocked"]
    #[doc = "content is possibly visible."]
    #[doc = ""]
    #[doc = "The finished event should be sent immediately on creation of this"]
    #[doc = "object if the compositor decides that the locked event will not be sent."]
    #[doc = ""]
    #[doc = "The compositor may wait for the client to create and render session lock"]
    #[doc = "surfaces before sending the locked event to avoid displaying intermediate"]
    #[doc = "blank frames. However, it must impose a reasonable time limit if"]
    #[doc = "waiting and send the locked event as soon as the hard requirements"]
    #[doc = "described above can be met if the time limit expires. Clients should"]
    #[doc = "immediately create lock surfaces for all outputs on creation of this"]
    #[doc = "object to make this possible."]
    #[doc = ""]
    #[doc = "This behavior of the locked event is required in order to prevent"]
    #[doc = "possible race conditions with clients that wish to suspend the system"]
    #[doc = "or similar after locking the session. Without these semantics, clients"]
    #[doc = "triggering a suspend after receiving the locked event would race with"]
    #[doc = "the first \"locked\" frame being presented and normal/unlocked frames"]
    #[doc = "might be briefly visible as the system is resumed if the suspend"]
    #[doc = "operation wins the race."]
    #[doc = ""]
    #[doc = "If the client dies while the session is locked, the compositor must not"]
    #[doc = "unlock the session in response. It is acceptable for the session to be"]
    #[doc = "permanently locked if this happens. The compositor may choose to continue"]
    #[doc = "to display the lock surfaces the client had mapped before it died or"]
    #[doc = "alternatively fall back to a solid color, this is compositor policy."]
    #[doc = ""]
    #[doc = "Compositors may also allow a secure way to recover the session, the"]
    #[doc = "details of this are compositor policy. Compositors may allow a new"]
    #[doc = "client to create a ext_session_lock_v1 object and take responsibility"]
    #[doc = "for unlocking the session, they may even start a new lock client"]
    #[doc = "instance automatically."]
    pub mod ext_session_lock_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "attempted to destroy session lock while locked"]
            InvalidDestroy = 0u32,
            #[doc = "unlock requested but locked event was never sent"]
            InvalidUnlock = 1u32,
            #[doc = "given wl_surface already has a role"]
            Role = 2u32,
            #[doc = "given output already has a lock surface"]
            DuplicateOutput = 3u32,
            #[doc = "given wl_surface has a buffer attached or committed"]
            AlreadyConstructed = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidDestroy),
                    1u32 => Ok(Self::InvalidUnlock),
                    2u32 => Ok(Self::Role),
                    3u32 => Ok(Self::DuplicateOutput),
                    4u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_session_lock_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_session_lock_v1";
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
    #[doc = "The client may use lock surfaces to display a screensaver, render a"]
    #[doc = "dialog to enter a password and unlock the session, or however else it"]
    #[doc = "sees fit."]
    #[doc = ""]
    #[doc = "On binding this interface the compositor will immediately send the"]
    #[doc = "first configure event. After making the ack_configure request in"]
    #[doc = "response to this event the client should attach and commit the first"]
    #[doc = "buffer. Committing the surface before acking the first configure is a"]
    #[doc = "protocol error. Committing the surface with a null buffer at any time"]
    #[doc = "is a protocol error."]
    #[doc = ""]
    #[doc = "The compositor is free to handle keyboard/pointer focus for lock"]
    #[doc = "surfaces however it chooses. A reasonable way to do this would be to"]
    #[doc = "give the first lock surface created keyboard focus and change keyboard"]
    #[doc = "focus if the user clicks on other surfaces."]
    pub mod ext_session_lock_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface committed before first ack_configure request"]
            CommitBeforeFirstAck = 0u32,
            #[doc = "surface committed with a null buffer"]
            NullBuffer = 1u32,
            #[doc = "failed to match ack'd width/height"]
            DimensionsMismatch = 2u32,
            #[doc = "serial provided in ack_configure is invalid"]
            InvalidSerial = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::CommitBeforeFirstAck),
                    1u32 => Ok(Self::NullBuffer),
                    2u32 => Ok(Self::DimensionsMismatch),
                    3u32 => Ok(Self::InvalidSerial),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the ext_session_lock_surface_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockSurfaceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_session_lock_surface_v1";
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
#[doc = "The transient seat protocol can be used by privileged clients to create"]
#[doc = "independent seats that will be removed from the compositor when the client"]
#[doc = "destroys its transient seat."]
#[doc = ""]
#[doc = "This protocol is intended for use with virtual input protocols such as"]
#[doc = "\"virtual_keyboard_unstable_v1\" or \"wlr_virtual_pointer_unstable_v1\", both"]
#[doc = "of which allow the user to select a seat."]
#[doc = ""]
#[doc = "The \"wl_seat\" global created by this protocol does not generate input events"]
#[doc = "on its own, or have any capabilities except those assigned to it by other"]
#[doc = "protocol extensions, such as the ones mentioned above."]
#[doc = ""]
#[doc = "For example, a remote desktop server can create a seat with virtual inputs"]
#[doc = "for each remote user by following these steps for each new connection:"]
#[doc = "* Create a transient seat"]
#[doc = "* Wait for the transient seat to be created"]
#[doc = "* Locate a \"wl_seat\" global with a matching name"]
#[doc = "* Create virtual inputs using the resulting \"wl_seat\" global"]
pub mod ext_transient_seat_v1 {
    #[doc = "The transient seat manager creates short-lived seats."]
    pub mod ext_transient_seat_manager_v1 {
        #[doc = "Trait to implement the ext_transient_seat_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtTransientSeatManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_transient_seat_manager_v1";
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
    #[doc = "When the transient seat handle is destroyed, the seat itself will also be"]
    #[doc = "destroyed."]
    pub mod ext_transient_seat_v1 {
        #[doc = "Trait to implement the ext_transient_seat_v1 interface. See the module level documentation for more info"]
        pub trait ExtTransientSeatV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "ext_transient_seat_v1";
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
#[doc = "This protocol allows a compositor to suggest for surfaces to render at"]
#[doc = "fractional scales."]
#[doc = ""]
#[doc = "A client can submit scaled content by utilizing wp_viewport. This is done by"]
#[doc = "creating a wp_viewport object for the surface and setting the destination"]
#[doc = "rectangle to the surface size before the scale factor is applied."]
#[doc = ""]
#[doc = "The buffer size is calculated by multiplying the surface size by the"]
#[doc = "intended scale."]
#[doc = ""]
#[doc = "The wl_surface buffer scale should remain set to 1."]
#[doc = ""]
#[doc = "If a surface has a surface-local size of 100 px by 50 px and wishes to"]
#[doc = "submit buffers with a scale of 1.5, then a buffer of 150px by 75 px should"]
#[doc = "be used and the wp_viewport destination rectangle should be 100 px by 50 px."]
#[doc = ""]
#[doc = "For toplevel surfaces, the size is rounded halfway away from zero. The"]
#[doc = "rounding algorithm for subsurface position and size is not defined."]
pub mod fractional_scale_v1 {
    #[doc = "A global interface for requesting surfaces to use fractional scales."]
    pub mod wp_fractional_scale_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a fractional_scale object associated"]
            FractionalScaleExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::FractionalScaleExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_fractional_scale_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpFractionalScaleManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_fractional_scale_manager_v1";
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
    #[doc = "An additional interface to a wl_surface object which allows the compositor"]
    #[doc = "to inform the client of the preferred scale."]
    pub mod wp_fractional_scale_v1 {
        #[doc = "Trait to implement the wp_fractional_scale_v1 interface. See the module level documentation for more info"]
        pub trait WpFractionalScaleV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_fractional_scale_v1";
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
#[doc = "This protocol allows clients to request explicit synchronization for"]
#[doc = "buffers. It is tied to the Linux DRM synchronization object framework."]
#[doc = ""]
#[doc = "Synchronization refers to co-ordination of pipelined operations performed"]
#[doc = "on buffers. Most GPU clients will schedule an asynchronous operation to"]
#[doc = "render to the buffer, then immediately send the buffer to the compositor"]
#[doc = "to be attached to a surface."]
#[doc = ""]
#[doc = "With implicit synchronization, ensuring that the rendering operation is"]
#[doc = "complete before the compositor displays the buffer is an implementation"]
#[doc = "detail handled by either the kernel or userspace graphics driver."]
#[doc = ""]
#[doc = "By contrast, with explicit synchronization, DRM synchronization object"]
#[doc = "timeline points mark when the asynchronous operations are complete. When"]
#[doc = "submitting a buffer, the client provides a timeline point which will be"]
#[doc = "waited on before the compositor accesses the buffer, and another timeline"]
#[doc = "point that the compositor will signal when it no longer needs to access the"]
#[doc = "buffer contents for the purposes of the surface commit."]
#[doc = ""]
#[doc = "Linux DRM synchronization objects are documented at:"]
#[doc = "https://dri.freedesktop.org/docs/drm/gpu/drm-mm.html#drm-sync-objects"]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod linux_drm_syncobj_v1 {
    #[doc = "This global is a factory interface, allowing clients to request"]
    #[doc = "explicit synchronization for buffers on a per-surface basis."]
    #[doc = ""]
    #[doc = "See wp_linux_drm_syncobj_surface_v1 for more information."]
    pub mod wp_linux_drm_syncobj_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a synchronization object associated"]
            SurfaceExists = 0u32,
            #[doc = "the timeline object could not be imported"]
            InvalidTimeline = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SurfaceExists),
                    1u32 => Ok(Self::InvalidTimeline),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_linux_drm_syncobj_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_manager_v1";
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
    #[doc = "This object represents an explicit synchronization object timeline"]
    #[doc = "imported by the client to the compositor."]
    pub mod wp_linux_drm_syncobj_timeline_v1 {
        #[doc = "Trait to implement the wp_linux_drm_syncobj_timeline_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjTimelineV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_timeline_v1";
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
    #[doc = "This object is an add-on interface for wl_surface to enable explicit"]
    #[doc = "synchronization."]
    #[doc = ""]
    #[doc = "Each surface can be associated with only one object of this interface at"]
    #[doc = "any time."]
    #[doc = ""]
    #[doc = "Explicit synchronization is guaranteed to be supported for buffers"]
    #[doc = "created with any version of the linux-dmabuf protocol. Compositors are"]
    #[doc = "free to support explicit synchronization for additional buffer types."]
    #[doc = "If at surface commit time the attached buffer does not support explicit"]
    #[doc = "synchronization, an unsupported_buffer error is raised."]
    #[doc = ""]
    #[doc = "As long as the wp_linux_drm_syncobj_surface_v1 object is alive, the"]
    #[doc = "compositor may ignore implicit synchronization for buffers attached and"]
    #[doc = "committed to the wl_surface. The delivery of wl_buffer.release events"]
    #[doc = "for buffers attached to the surface becomes undefined."]
    #[doc = ""]
    #[doc = "Clients must set both acquire and release points if and only if a"]
    #[doc = "non-null buffer is attached in the same surface commit. See the"]
    #[doc = "no_buffer, no_acquire_point and no_release_point protocol errors."]
    #[doc = ""]
    #[doc = "If at surface commit time the acquire and release DRM syncobj timelines"]
    #[doc = "are identical, the acquire point value must be strictly less than the"]
    #[doc = "release point value, or else the conflicting_points protocol error is"]
    #[doc = "raised."]
    pub mod wp_linux_drm_syncobj_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the associated wl_surface was destroyed"]
            NoSurface = 1u32,
            #[doc = "the buffer does not support explicit synchronization"]
            UnsupportedBuffer = 2u32,
            #[doc = "no buffer was attached"]
            NoBuffer = 3u32,
            #[doc = "no acquire timeline point was set"]
            NoAcquirePoint = 4u32,
            #[doc = "no release timeline point was set"]
            NoReleasePoint = 5u32,
            #[doc = "acquire and release timeline points are in conflict"]
            ConflictingPoints = 6u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::NoSurface),
                    2u32 => Ok(Self::UnsupportedBuffer),
                    3u32 => Ok(Self::NoBuffer),
                    4u32 => Ok(Self::NoAcquirePoint),
                    5u32 => Ok(Self::NoReleasePoint),
                    6u32 => Ok(Self::ConflictingPoints),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_linux_drm_syncobj_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjSurfaceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_surface_v1";
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
pub mod security_context_v1 {
    #[doc = "This interface allows a client to register a new Wayland connection to"]
    #[doc = "the compositor and attach a security context to it."]
    #[doc = ""]
    #[doc = "This is intended to be used by sandboxes. Sandbox engines attach a"]
    #[doc = "security context to all connections coming from inside the sandbox. The"]
    #[doc = "compositor can then restrict the features that the sandboxed connections"]
    #[doc = "can use."]
    #[doc = ""]
    #[doc = "Compositors should forbid nesting multiple security contexts by not"]
    #[doc = "exposing wp_security_context_manager_v1 global to clients with a security"]
    #[doc = "context attached, or by sending the nested protocol error. Nested"]
    #[doc = "security contexts are dangerous because they can potentially allow"]
    #[doc = "privilege escalation of a sandboxed client."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_security_context_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "listening socket FD is invalid"]
            InvalidListenFd = 1u32,
            #[doc = "nested security contexts are forbidden"]
            Nested = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidListenFd),
                    2u32 => Ok(Self::Nested),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_security_context_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpSecurityContextManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_security_context_manager_v1";
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
    #[doc = "The security context allows a client to register a new client and attach"]
    #[doc = "security context metadata to the connections."]
    #[doc = ""]
    #[doc = "When both are set, the combination of the application ID and the sandbox"]
    #[doc = "engine must uniquely identify an application. The same application ID"]
    #[doc = "will be used across instances (e.g. if the application is restarted, or"]
    #[doc = "if the application is started multiple times)."]
    #[doc = ""]
    #[doc = "When both are set, the combination of the instance ID and the sandbox"]
    #[doc = "engine must uniquely identify a running instance of an application."]
    pub mod wp_security_context_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "security context has already been committed"]
            AlreadyUsed = 1u32,
            #[doc = "metadata has already been set"]
            AlreadySet = 2u32,
            #[doc = "metadata is invalid"]
            InvalidMetadata = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyUsed),
                    2u32 => Ok(Self::AlreadySet),
                    3u32 => Ok(Self::InvalidMetadata),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_security_context_v1 interface. See the module level documentation for more info"]
        pub trait WpSecurityContextV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_security_context_v1";
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
#[doc = "This protocol extension allows clients to create single-pixel buffers."]
#[doc = ""]
#[doc = "Compositors supporting this protocol extension should also support the"]
#[doc = "viewporter protocol extension. Clients may use viewporter to scale a"]
#[doc = "single-pixel buffer to a desired size."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod single_pixel_buffer_v1 {
    #[doc = "The wp_single_pixel_buffer_manager_v1 interface is a factory for"]
    #[doc = "single-pixel buffers."]
    pub mod wp_single_pixel_buffer_manager_v1 {
        #[doc = "Trait to implement the wp_single_pixel_buffer_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpSinglePixelBufferManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_single_pixel_buffer_manager_v1";
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
pub mod tearing_control_v1 {
    #[doc = "For some use cases like games or drawing tablets it can make sense to"]
    #[doc = "reduce latency by accepting tearing with the use of asynchronous page"]
    #[doc = "flips. This global is a factory interface, allowing clients to inform"]
    #[doc = "which type of presentation the content of their surfaces is suitable for."]
    #[doc = ""]
    #[doc = "Graphics APIs like EGL or Vulkan, that manage the buffer queue and commits"]
    #[doc = "of a wl_surface themselves, are likely to be using this extension"]
    #[doc = "internally. If a client is using such an API for a wl_surface, it should"]
    #[doc = "not directly use this extension on that surface, to avoid raising a"]
    #[doc = "tearing_control_exists protocol error."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod wp_tearing_control_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a tearing object associated"]
            TearingControlExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::TearingControlExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_tearing_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpTearingControlManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_tearing_control_manager_v1";
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
    #[doc = "An additional interface to a wl_surface object, which allows the client"]
    #[doc = "to hint to the compositor if the content on the surface is suitable for"]
    #[doc = "presentation with tearing."]
    #[doc = "The default presentation hint is vsync. See presentation_hint for more"]
    #[doc = "details."]
    pub mod wp_tearing_control_v1 {
        #[doc = "This enum provides information for if submitted frames from the client"]
        #[doc = "may be presented with tearing."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PresentationHint {
            Vsync = 0u32,
            Async = 1u32,
        }
        impl TryFrom<u32> for PresentationHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Vsync),
                    1u32 => Ok(Self::Async),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the wp_tearing_control_v1 interface. See the module level documentation for more info"]
        pub trait WpTearingControlV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "wp_tearing_control_v1";
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
#[doc = "The way for a client to pass focus to another toplevel is as follows."]
#[doc = ""]
#[doc = "The client that intends to activate another toplevel uses the"]
#[doc = "xdg_activation_v1.get_activation_token request to get an activation token."]
#[doc = "This token is then forwarded to the client, which is supposed to activate"]
#[doc = "one of its surfaces, through a separate band of communication."]
#[doc = ""]
#[doc = "One established way of doing this is through the XDG_ACTIVATION_TOKEN"]
#[doc = "environment variable of a newly launched child process. The child process"]
#[doc = "should unset the environment variable again right after reading it out in"]
#[doc = "order to avoid propagating it to other child processes."]
#[doc = ""]
#[doc = "Another established way exists for Applications implementing the D-Bus"]
#[doc = "interface org.freedesktop.Application, which should get their token under"]
#[doc = "activation-token on their platform_data."]
#[doc = ""]
#[doc = "In general activation tokens may be transferred across clients through"]
#[doc = "means not described in this protocol."]
#[doc = ""]
#[doc = "The client to be activated will then pass the token"]
#[doc = "it received to the xdg_activation_v1.activate request. The compositor can"]
#[doc = "then use this token to decide how to react to the activation request."]
#[doc = ""]
#[doc = "The token the activating client gets may be ineffective either already at"]
#[doc = "the time it receives it, for example if it was not focused, for focus"]
#[doc = "stealing prevention. The activating client will have no way to discover"]
#[doc = "the validity of the token, and may still forward it to the to be activated"]
#[doc = "client."]
#[doc = ""]
#[doc = "The created activation token may optionally get information attached to it"]
#[doc = "that can be used by the compositor to identify the application that we"]
#[doc = "intend to activate. This can for example be used to display a visual hint"]
#[doc = "about what application is being started."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod xdg_activation_v1 {
    #[doc = "A global interface used for informing the compositor about applications"]
    #[doc = "being activated or started, or for applications to request to be"]
    #[doc = "activated."]
    pub mod xdg_activation_v1 {
        #[doc = "Trait to implement the xdg_activation_v1 interface. See the module level documentation for more info"]
        pub trait XdgActivationV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_activation_v1";
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
    #[doc = "An object for setting up a token and receiving a token handle that can"]
    #[doc = "be passed as an activation token to another client."]
    #[doc = ""]
    #[doc = "The object is created using the xdg_activation_v1.get_activation_token"]
    #[doc = "request. This object should then be populated with the app_id, surface"]
    #[doc = "and serial information and committed. The compositor shall then issue a"]
    #[doc = "done event with the token. In case the request's parameters are invalid,"]
    #[doc = "the compositor will provide an invalid token."]
    pub mod xdg_activation_token_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "The token has already been used previously"]
            AlreadyUsed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_activation_token_v1 interface. See the module level documentation for more info"]
        pub trait XdgActivationTokenV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_activation_token_v1";
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
pub mod xdg_dialog_v1 {
    #[doc = "The xdg_wm_dialog_v1 interface is exposed as a global object allowing"]
    #[doc = "to register surfaces with a xdg_toplevel role as \"dialogs\" relative to"]
    #[doc = "another toplevel."]
    #[doc = ""]
    #[doc = "The compositor may let this relation influence how the surface is"]
    #[doc = "placed, displayed or interacted with."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod xdg_wm_dialog_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the xdg_toplevel object has already been used to create a xdg_dialog_v1"]
            AlreadyUsed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyUsed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_wm_dialog_v1 interface. See the module level documentation for more info"]
        pub trait XdgWmDialogV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_wm_dialog_v1";
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
    #[doc = "A xdg_dialog_v1 object is an ancillary object tied to a xdg_toplevel. Its"]
    #[doc = "purpose is hinting the compositor that the toplevel is a \"dialog\" (e.g. a"]
    #[doc = "temporary window) relative to another toplevel (see"]
    #[doc = "xdg_toplevel.set_parent). If the xdg_toplevel is destroyed, the xdg_dialog_v1"]
    #[doc = "becomes inert."]
    #[doc = ""]
    #[doc = "Through this object, the client may provide additional hints about"]
    #[doc = "the purpose of the secondary toplevel. This interface has no effect"]
    #[doc = "on toplevels that are not attached to a parent toplevel."]
    pub mod xdg_dialog_v1 {
        #[doc = "Trait to implement the xdg_dialog_v1 interface. See the module level documentation for more info"]
        pub trait XdgDialogV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_dialog_v1";
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
pub mod xdg_toplevel_drag_v1 {
    #[doc = "This protocol enhances normal drag and drop with the ability to move a"]
    #[doc = "window at the same time. This allows having detachable parts of a window"]
    #[doc = "that when dragged out of it become a new window and can be dragged over"]
    #[doc = "an existing window to be reattached."]
    #[doc = ""]
    #[doc = "A typical workflow would be when the user starts dragging on top of a"]
    #[doc = "detachable part of a window, the client would create a wl_data_source and"]
    #[doc = "a xdg_toplevel_drag_v1 object and start the drag as normal via"]
    #[doc = "wl_data_device.start_drag. Once the client determines that the detachable"]
    #[doc = "window contents should be detached from the originating window, it creates"]
    #[doc = "a new xdg_toplevel with these contents and issues a"]
    #[doc = "xdg_toplevel_drag_v1.attach request before mapping it. From now on the new"]
    #[doc = "window is moved by the compositor during the drag as if the client called"]
    #[doc = "xdg_toplevel.move."]
    #[doc = ""]
    #[doc = "Dragging an existing window is similar. The client creates a"]
    #[doc = "xdg_toplevel_drag_v1 object and attaches the existing toplevel before"]
    #[doc = "starting the drag."]
    #[doc = ""]
    #[doc = "Clients use the existing drag and drop mechanism to detect when a window"]
    #[doc = "can be docked or undocked. If the client wants to snap a window into a"]
    #[doc = "parent window it should delete or unmap the dragged top-level. If the"]
    #[doc = "contents should be detached again it attaches a new toplevel as described"]
    #[doc = "above. If a drag operation is cancelled without being dropped, clients"]
    #[doc = "should revert to the previous state, deleting any newly created windows"]
    #[doc = "as appropriate. When a drag operation ends as indicated by"]
    #[doc = "wl_data_source.dnd_drop_performed the dragged toplevel window's final"]
    #[doc = "position is determined as if a xdg_toplevel_move operation ended."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    pub mod xdg_toplevel_drag_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "data_source already used for toplevel drag"]
            InvalidSource = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_drag_manager_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelDragManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_toplevel_drag_manager_v1";
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
    pub mod xdg_toplevel_drag_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "valid toplevel already attached"]
            ToplevelAttached = 0u32,
            #[doc = "drag has not ended"]
            OngoingDrag = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::ToplevelAttached),
                    1u32 => Ok(Self::OngoingDrag),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_drag_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelDragV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_toplevel_drag_v1";
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
#[doc = "This protocol adds a xwayland_surface role which allows an Xwayland"]
#[doc = "server to associate an X11 window to a wl_surface."]
#[doc = ""]
#[doc = "Before this protocol, this would be done via the Xwayland server"]
#[doc = "providing the wl_surface's resource id via the a client message with"]
#[doc = "the WL_SURFACE_ID atom on the X window."]
#[doc = "This was problematic as a race could occur if the wl_surface"]
#[doc = "associated with a WL_SURFACE_ID for a window was destroyed before the"]
#[doc = "client message was processed by the compositor and another surface"]
#[doc = "(or other object) had taken its id due to recycling."]
#[doc = ""]
#[doc = "This protocol solves the problem by moving the X11 window to wl_surface"]
#[doc = "association step to the Wayland side, which means that the association"]
#[doc = "cannot happen out-of-sync with the resource lifetime of the wl_surface."]
#[doc = ""]
#[doc = "This protocol avoids duplicating the race on the other side by adding a"]
#[doc = "non-zero monotonic serial number which is entirely unique that is set on"]
#[doc = "both the wl_surface (via. xwayland_surface_v1's set_serial method) and"]
#[doc = "the X11 window (via. the `WL_SURFACE_SERIAL` client message) that can be"]
#[doc = "used to associate them, and synchronize the two timelines."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\",  \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
pub mod xwayland_shell_v1 {
    #[doc = "xwayland_shell_v1 is a singleton global object that"]
    #[doc = "provides the ability to create a xwayland_surface_v1 object"]
    #[doc = "for a given wl_surface."]
    #[doc = ""]
    #[doc = "This interface is intended to be bound by the Xwayland server."]
    #[doc = ""]
    #[doc = "A compositor must not allow clients other than Xwayland to"]
    #[doc = "bind to this interface. A compositor should hide this global"]
    #[doc = "from other clients' wl_registry."]
    #[doc = "A client the compositor does not consider to be an Xwayland"]
    #[doc = "server attempting to bind this interface will result in"]
    #[doc = "an implementation-defined error."]
    #[doc = ""]
    #[doc = "An Xwayland server that has bound this interface must not"]
    #[doc = "set the `WL_SURFACE_ID` atom on a window."]
    pub mod xwayland_shell_v1 {
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
        #[doc = "Trait to implement the xwayland_shell_v1 interface. See the module level documentation for more info"]
        pub trait XwaylandShellV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xwayland_shell_v1";
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
    #[doc = "An Xwayland surface is a surface managed by an Xwayland server."]
    #[doc = "It is used for associating surfaces to Xwayland windows."]
    #[doc = ""]
    #[doc = "The Xwayland server associated with actions in this interface is"]
    #[doc = "determined by the Wayland client making the request."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xwayland_surface_v1 state to take effect."]
    pub mod xwayland_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface is already associated with an X11 window"]
            AlreadyAssociated = 0u32,
            #[doc = "serial was not valid"]
            InvalidSerial = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyAssociated),
                    1u32 => Ok(Self::InvalidSerial),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the xwayland_surface_v1 interface. See the module level documentation for more info"]
        pub trait XwaylandSurfaceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xwayland_surface_v1";
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
    pub mod zwp_fullscreen_shell_v1 {
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
        #[doc = "Trait to implement the zwp_fullscreen_shell_v1 interface. See the module level documentation for more info"]
        pub trait ZwpFullscreenShellV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_fullscreen_shell_v1";
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
    pub mod zwp_fullscreen_shell_mode_feedback_v1 {
        #[doc = "Trait to implement the zwp_fullscreen_shell_mode_feedback_v1 interface. See the module level documentation for more info"]
        pub trait ZwpFullscreenShellModeFeedbackV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_fullscreen_shell_mode_feedback_v1";
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
    pub mod zwp_idle_inhibit_manager_v1 {
        #[doc = "Trait to implement the zwp_idle_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpIdleInhibitManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_idle_inhibit_manager_v1";
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
    pub mod zwp_idle_inhibitor_v1 {
        #[doc = "Trait to implement the zwp_idle_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ZwpIdleInhibitorV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_idle_inhibitor_v1";
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
    pub mod zwp_input_method_context_v1 {
        #[doc = "Trait to implement the zwp_input_method_context_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputMethodContextV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_input_method_context_v1";
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
    #[doc = "An input method object is responsible for composing text in response to"]
    #[doc = "input from hardware or virtual keyboards. There is one input method"]
    #[doc = "object per seat. On activate there is a new input method context object"]
    #[doc = "created which allows the input method to communicate with the text input."]
    pub mod zwp_input_method_v1 {
        #[doc = "Trait to implement the zwp_input_method_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputMethodV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_input_method_v1";
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
    #[doc = "Only one client can bind this interface at a time."]
    pub mod zwp_input_panel_v1 {
        #[doc = "Trait to implement the zwp_input_panel_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputPanelV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_input_panel_v1";
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
    pub mod zwp_input_panel_surface_v1 {
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
        #[doc = "Trait to implement the zwp_input_panel_surface_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputPanelSurfaceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_input_panel_surface_v1";
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
pub mod input_timestamps_unstable_v1 {
    #[doc = "A global interface used for requesting high-resolution timestamps"]
    #[doc = "for input events."]
    pub mod zwp_input_timestamps_manager_v1 {
        #[doc = "Trait to implement the zwp_input_timestamps_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputTimestampsManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_input_timestamps_manager_v1";
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
    #[doc = "Provides high-resolution timestamp events for a set of subscribed input"]
    #[doc = "events. The set of subscribed input events is determined by the"]
    #[doc = "zwp_input_timestamps_manager_v1 request used to create this object."]
    pub mod zwp_input_timestamps_v1 {
        #[doc = "Trait to implement the zwp_input_timestamps_v1 interface. See the module level documentation for more info"]
        pub trait ZwpInputTimestampsV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_input_timestamps_v1";
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
pub mod keyboard_shortcuts_inhibit_unstable_v1 {
    #[doc = "A global interface used for inhibiting the compositor keyboard shortcuts."]
    pub mod zwp_keyboard_shortcuts_inhibit_manager_v1 {
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
        #[doc = "Trait to implement the zwp_keyboard_shortcuts_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpKeyboardShortcutsInhibitManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_keyboard_shortcuts_inhibit_manager_v1";
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
    pub mod zwp_keyboard_shortcuts_inhibitor_v1 {
        #[doc = "Trait to implement the zwp_keyboard_shortcuts_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ZwpKeyboardShortcutsInhibitorV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_keyboard_shortcuts_inhibitor_v1";
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
    pub mod zwp_linux_dmabuf_v1 {
        #[doc = "Trait to implement the zwp_linux_dmabuf_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufV1: crate::client::Dispatcher {
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
        pub trait ZwpLinuxBufferParamsV1: crate::client::Dispatcher {
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
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct TrancheFlags : u32 { # [doc = "direct scan-out tranche"] const Scanout = 1u32 ; } }
        impl TryFrom<u32> for TrancheFlags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the zwp_linux_dmabuf_feedback_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxDmabufFeedbackV1: crate::client::Dispatcher {
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
        }
    }
}
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
    pub mod zwp_linux_explicit_synchronization_v1 {
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
        #[doc = "Trait to implement the zwp_linux_explicit_synchronization_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxExplicitSynchronizationV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_linux_explicit_synchronization_v1";
            const VERSION: u32 = 2u32;
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
    pub mod zwp_linux_surface_synchronization_v1 {
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
        #[doc = "Trait to implement the zwp_linux_surface_synchronization_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxSurfaceSynchronizationV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_linux_surface_synchronization_v1";
            const VERSION: u32 = 2u32;
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
    pub mod zwp_linux_buffer_release_v1 {
        #[doc = "Trait to implement the zwp_linux_buffer_release_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLinuxBufferReleaseV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_linux_buffer_release_v1";
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
    pub mod zwp_pointer_constraints_v1 {
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
        #[doc = "Trait to implement the zwp_pointer_constraints_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerConstraintsV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_pointer_constraints_v1";
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
    pub mod zwp_locked_pointer_v1 {
        #[doc = "Trait to implement the zwp_locked_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpLockedPointerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_locked_pointer_v1";
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
    pub mod zwp_confined_pointer_v1 {
        #[doc = "Trait to implement the zwp_confined_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpConfinedPointerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_confined_pointer_v1";
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
pub mod pointer_gestures_unstable_v1 {
    #[doc = "A global interface to provide semantic touchpad gestures for a given"]
    #[doc = "pointer."]
    #[doc = ""]
    #[doc = "Three gestures are currently supported: swipe, pinch, and hold."]
    #[doc = "Pinch and swipe gestures follow a three-stage cycle: begin, update,"]
    #[doc = "end, hold gestures follow a two-stage cycle: begin and end. All"]
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
    pub mod zwp_pointer_gestures_v1 {
        #[doc = "Trait to implement the zwp_pointer_gestures_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGesturesV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_pointer_gestures_v1";
            const VERSION: u32 = 3u32;
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
    pub mod zwp_pointer_gesture_swipe_v1 {
        #[doc = "Trait to implement the zwp_pointer_gesture_swipe_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGestureSwipeV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_pointer_gesture_swipe_v1";
            const VERSION: u32 = 2u32;
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
    pub mod zwp_pointer_gesture_pinch_v1 {
        #[doc = "Trait to implement the zwp_pointer_gesture_pinch_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGesturePinchV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_pointer_gesture_pinch_v1";
            const VERSION: u32 = 2u32;
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
    pub mod zwp_pointer_gesture_hold_v1 {
        #[doc = "Trait to implement the zwp_pointer_gesture_hold_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPointerGestureHoldV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_pointer_gesture_hold_v1";
            const VERSION: u32 = 3u32;
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
pub mod wp_primary_selection_unstable_v1 {
    #[doc = "The primary selection device manager is a singleton global object that"]
    #[doc = "provides access to the primary selection. It allows to create"]
    #[doc = "wp_primary_selection_source objects, as well as retrieving the per-seat"]
    #[doc = "wp_primary_selection_device objects."]
    pub mod zwp_primary_selection_device_manager_v1 {
        #[doc = "Trait to implement the zwp_primary_selection_device_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionDeviceManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_primary_selection_device_manager_v1";
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
    pub mod zwp_primary_selection_device_v1 {
        #[doc = "Trait to implement the zwp_primary_selection_device_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionDeviceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_primary_selection_device_v1";
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
    #[doc = "A wp_primary_selection_offer represents an offer to transfer the contents"]
    #[doc = "of the primary selection clipboard to the client. Similar to"]
    #[doc = "wl_data_offer, the offer also describes the mime types that the data can"]
    #[doc = "be converted to and provides the mechanisms for transferring the data"]
    #[doc = "directly to the client."]
    pub mod zwp_primary_selection_offer_v1 {
        #[doc = "Trait to implement the zwp_primary_selection_offer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionOfferV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_primary_selection_offer_v1";
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
    #[doc = "The source side of a wp_primary_selection_offer, it provides a way to"]
    #[doc = "describe the offered data and respond to requests to transfer the"]
    #[doc = "requested contents of the primary selection clipboard."]
    pub mod zwp_primary_selection_source_v1 {
        #[doc = "Trait to implement the zwp_primary_selection_source_v1 interface. See the module level documentation for more info"]
        pub trait ZwpPrimarySelectionSourceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_primary_selection_source_v1";
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
pub mod relative_pointer_unstable_v1 {
    #[doc = "A global interface used for getting the relative pointer object for a"]
    #[doc = "given pointer."]
    pub mod zwp_relative_pointer_manager_v1 {
        #[doc = "Trait to implement the zwp_relative_pointer_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpRelativePointerManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_relative_pointer_manager_v1";
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
    #[doc = "A wp_relative_pointer object is an extension to the wl_pointer interface"]
    #[doc = "used for emitting relative pointer events. It shares the same focus as"]
    #[doc = "wl_pointer objects of the same seat and will only emit events when it has"]
    #[doc = "focus."]
    pub mod zwp_relative_pointer_v1 {
        #[doc = "Trait to implement the zwp_relative_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwpRelativePointerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_relative_pointer_v1";
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
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod tablet_unstable_v1 {
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "system. All tablets are associated with a seat, to get access to the"]
    #[doc = "actual tablets, use wp_tablet_manager.get_tablet_seat."]
    pub mod zwp_tablet_manager_v1 {
        #[doc = "Trait to implement the zwp_tablet_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_tablet_manager_v1";
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
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "seat. After binding to this interface, the compositor sends a set of"]
    #[doc = "wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."]
    pub mod zwp_tablet_seat_v1 {
        #[doc = "Trait to implement the zwp_tablet_seat_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_tablet_seat_v1";
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
    pub mod zwp_tablet_tool_v1 {
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
        #[doc = "Trait to implement the zwp_tablet_tool_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletToolV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_tablet_tool_v1";
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
    #[doc = "The wp_tablet interface represents one graphics tablet device. The"]
    #[doc = "tablet interface itself does not generate events; all events are"]
    #[doc = "generated by wp_tablet_tool objects when in proximity above a tablet."]
    #[doc = ""]
    #[doc = "A tablet has a number of static characteristics, e.g. device name and"]
    #[doc = "pid/vid. These capabilities are sent in an event sequence after the"]
    #[doc = "wp_tablet_seat.tablet_added event. This initial event sequence is"]
    #[doc = "terminated by a wp_tablet.done event."]
    pub mod zwp_tablet_v1 {
        #[doc = "Trait to implement the zwp_tablet_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_tablet_v1";
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
#[doc = ""]
#[doc = "Disclaimer: This protocol extension has been marked stable. This copy is"]
#[doc = "no longer used and only retained for backwards compatibility. The"]
#[doc = "canonical version can be found in the stable/ directory."]
pub mod tablet_unstable_v2 {
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "system. All tablets are associated with a seat, to get access to the"]
    #[doc = "actual tablets, use wp_tablet_manager.get_tablet_seat."]
    pub mod zwp_tablet_manager_v2 {
        #[doc = "Trait to implement the zwp_tablet_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletManagerV2: crate::client::Dispatcher {
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
        }
    }
    #[doc = "An object that provides access to the graphics tablets available on this"]
    #[doc = "seat. After binding to this interface, the compositor sends a set of"]
    #[doc = "wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."]
    pub mod zwp_tablet_seat_v2 {
        #[doc = "Trait to implement the zwp_tablet_seat_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletSeatV2: crate::client::Dispatcher {
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
        pub trait ZwpTabletToolV2: crate::client::Dispatcher {
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
        #[doc = "Trait to implement the zwp_tablet_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletV2: crate::client::Dispatcher {
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
        }
    }
    #[doc = "A circular interaction area, such as the touch ring on the Wacom Intuos"]
    #[doc = "Pro series tablets."]
    #[doc = ""]
    #[doc = "Events on a ring are logically grouped by the wl_tablet_pad_ring.frame"]
    #[doc = "event."]
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_ring_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadRingV2: crate::client::Dispatcher {
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
        }
    }
    #[doc = "A linear interaction area, such as the strips found in Wacom Cintiq"]
    #[doc = "models."]
    #[doc = ""]
    #[doc = "Events on a strip are logically grouped by the wl_tablet_pad_strip.frame"]
    #[doc = "event."]
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
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Finger),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwp_tablet_pad_strip_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadStripV2: crate::client::Dispatcher {
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
        #[doc = "Trait to implement the zwp_tablet_pad_group_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTabletPadGroupV2: crate::client::Dispatcher {
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
        pub trait ZwpTabletPadV2: crate::client::Dispatcher {
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
        }
    }
}
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
    pub mod zwp_text_input_v1 {
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behaviour"] const None = 0u32 ; # [doc = "auto completion, correction and capitalization"] const Default = 7u32 ; # [doc = "hidden and sensitive text"] const Password = 192u32 ; # [doc = "suggest word completions"] const AutoCompletion = 1u32 ; # [doc = "suggest word corrections"] const AutoCorrection = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
        impl TryFrom<u32> for ContentHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
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
        #[doc = "Trait to implement the zwp_text_input_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_text_input_v1";
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
    #[doc = "A factory for text_input objects. This object is a global singleton."]
    pub mod zwp_text_input_manager_v1 {
        #[doc = "Trait to implement the zwp_text_input_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_text_input_manager_v1";
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
    pub mod zwp_text_input_v3 {
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
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behavior"] const None = 0u32 ; # [doc = "suggest word completions"] const Completion = 1u32 ; # [doc = "suggest word corrections"] const Spellcheck = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just Latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
        impl TryFrom<u32> for ContentHint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
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
        #[doc = "Trait to implement the zwp_text_input_v3 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV3: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_text_input_v3";
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
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    pub mod zwp_text_input_manager_v3 {
        #[doc = "Trait to implement the zwp_text_input_manager_v3 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV3: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwp_text_input_manager_v3";
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
    pub mod zxdg_decoration_manager_v1 {
        #[doc = "Trait to implement the zxdg_decoration_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgDecorationManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_decoration_manager_v1";
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
    #[doc = "The decoration object allows the compositor to toggle server-side window"]
    #[doc = "decorations for a toplevel surface. The client can request to switch to"]
    #[doc = "another mode."]
    #[doc = ""]
    #[doc = "The xdg_toplevel_decoration object must be destroyed before its"]
    #[doc = "xdg_toplevel."]
    pub mod zxdg_toplevel_decoration_v1 {
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
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::UnconfiguredBuffer),
                    1u32 => Ok(Self::AlreadyConstructed),
                    2u32 => Ok(Self::Orphaned),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
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
        #[doc = "Trait to implement the zxdg_toplevel_decoration_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgToplevelDecorationV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_toplevel_decoration_v1";
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
pub mod xdg_foreign_unstable_v1 {
    #[doc = "A global interface used for exporting surfaces that can later be imported"]
    #[doc = "using xdg_importer."]
    pub mod zxdg_exporter_v1 {
        #[doc = "Trait to implement the zxdg_exporter_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgExporterV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_exporter_v1";
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
    #[doc = "A global interface used for importing surfaces exported by xdg_exporter."]
    #[doc = "With this interface, a client can create a reference to a surface of"]
    #[doc = "another client."]
    pub mod zxdg_importer_v1 {
        #[doc = "Trait to implement the zxdg_importer_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgImporterV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_importer_v1";
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
    #[doc = "An xdg_exported object represents an exported reference to a surface. The"]
    #[doc = "exported surface may be referenced as long as the xdg_exported object not"]
    #[doc = "destroyed. Destroying the xdg_exported invalidates any relationship the"]
    #[doc = "importer may have established using xdg_imported."]
    pub mod zxdg_exported_v1 {
        #[doc = "Trait to implement the zxdg_exported_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgExportedV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_exported_v1";
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
    #[doc = "An xdg_imported object represents an imported reference to surface exported"]
    #[doc = "by some client. A client can use this interface to manipulate"]
    #[doc = "relationships between its own surfaces and the imported surface."]
    pub mod zxdg_imported_v1 {
        #[doc = "Trait to implement the zxdg_imported_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgImportedV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_imported_v1";
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
pub mod xdg_foreign_unstable_v2 {
    #[doc = "A global interface used for exporting surfaces that can later be imported"]
    #[doc = "using xdg_importer."]
    pub mod zxdg_exporter_v2 {
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
        #[doc = "Trait to implement the zxdg_exporter_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgExporterV2: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_exporter_v2";
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
    #[doc = "A global interface used for importing surfaces exported by xdg_exporter."]
    #[doc = "With this interface, a client can create a reference to a surface of"]
    #[doc = "another client."]
    pub mod zxdg_importer_v2 {
        #[doc = "Trait to implement the zxdg_importer_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgImporterV2: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_importer_v2";
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
    #[doc = "An xdg_exported object represents an exported reference to a surface. The"]
    #[doc = "exported surface may be referenced as long as the xdg_exported object not"]
    #[doc = "destroyed. Destroying the xdg_exported invalidates any relationship the"]
    #[doc = "importer may have established using xdg_imported."]
    pub mod zxdg_exported_v2 {
        #[doc = "Trait to implement the zxdg_exported_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgExportedV2: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_exported_v2";
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
    #[doc = "An xdg_imported object represents an imported reference to surface exported"]
    #[doc = "by some client. A client can use this interface to manipulate"]
    #[doc = "relationships between its own surfaces and the imported surface."]
    pub mod zxdg_imported_v2 {
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
        #[doc = "Trait to implement the zxdg_imported_v2 interface. See the module level documentation for more info"]
        pub trait ZxdgImportedV2: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_imported_v2";
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
pub mod xdg_output_unstable_v1 {
    #[doc = "A global factory interface for xdg_output objects."]
    pub mod zxdg_output_manager_v1 {
        #[doc = "Trait to implement the zxdg_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgOutputManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_output_manager_v1";
            const VERSION: u32 = 3u32;
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
    #[doc = "An xdg_output describes part of the compositor geometry."]
    #[doc = ""]
    #[doc = "This typically corresponds to a monitor that displays part of the"]
    #[doc = "compositor space."]
    #[doc = ""]
    #[doc = "For objects version 3 onwards, after all xdg_output properties have been"]
    #[doc = "sent (when the object is created and when properties are updated), a"]
    #[doc = "wl_output.done event is sent. This allows changes to the output"]
    #[doc = "properties to be seen as atomic, even if they happen via multiple events."]
    pub mod zxdg_output_v1 {
        #[doc = "Trait to implement the zxdg_output_v1 interface. See the module level documentation for more info"]
        pub trait ZxdgOutputV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_output_v1";
            const VERSION: u32 = 3u32;
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
pub mod xdg_shell_unstable_v5 {
    #[doc = "xdg_shell allows clients to turn a wl_surface into a \"real window\""]
    #[doc = "which can be dragged, resized, stacked, and moved around by the"]
    #[doc = "user. Everything about this interface is suited towards traditional"]
    #[doc = "desktop environments."]
    pub mod xdg_shell {
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
        #[doc = "Trait to implement the xdg_shell interface. See the module level documentation for more info"]
        pub trait XdgShell: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_shell";
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
    pub mod xdg_surface {
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
        #[doc = "States set in this way are double-buffered. They will get applied on"]
        #[doc = "the next commit."]
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
        #[doc = "Trait to implement the xdg_surface interface. See the module level documentation for more info"]
        pub trait XdgSurface: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_surface";
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
    pub mod xdg_popup {
        #[doc = "Trait to implement the xdg_popup interface. See the module level documentation for more info"]
        pub trait XdgPopup: crate::client::Dispatcher {
            const INTERFACE: &'static str = "xdg_popup";
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
pub mod xdg_shell_unstable_v6 {
    #[doc = "xdg_shell allows clients to turn a wl_surface into a \"real window\""]
    #[doc = "which can be dragged, resized, stacked, and moved around by the"]
    #[doc = "user. Everything about this interface is suited towards traditional"]
    #[doc = "desktop environments."]
    pub mod zxdg_shell_v6 {
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
        #[doc = "Trait to implement the zxdg_shell_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgShellV6: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_shell_v6";
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
    pub mod zxdg_positioner_v6 {
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
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Anchor : u32 { # [doc = "the center of the anchor rectangle"] const None = 0u32 ; # [doc = "the top edge of the anchor rectangle"] const Top = 1u32 ; # [doc = "the bottom edge of the anchor rectangle"] const Bottom = 2u32 ; # [doc = "the left edge of the anchor rectangle"] const Left = 4u32 ; # [doc = "the right edge of the anchor rectangle"] const Right = 8u32 ; } }
        impl TryFrom<u32> for Anchor {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Gravity : u32 { # [doc = "center over the anchor edge"] const None = 0u32 ; # [doc = "position above the anchor edge"] const Top = 1u32 ; # [doc = "position below the anchor edge"] const Bottom = 2u32 ; # [doc = "position to the left of the anchor edge"] const Left = 4u32 ; # [doc = "position to the right of the anchor edge"] const Right = 8u32 ; } }
        impl TryFrom<u32> for Gravity {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        bitflags::bitflags! { # [doc = "The constraint adjustment value define ways the compositor will adjust"] # [doc = "the position of the surface, if the unadjusted position would result"] # [doc = "in the surface being partly constrained."] # [doc = ""] # [doc = "Whether a surface is considered 'constrained' is left to the compositor"] # [doc = "to determine. For example, the surface may be partly outside the"] # [doc = "compositor's defined 'work area', thus necessitating the child surface's"] # [doc = "position be adjusted until it is entirely inside the work area."] # [doc = ""] # [doc = "The adjustments can be combined, according to a defined precedence: 1)"] # [doc = "Flip, 2) Slide, 3) Resize."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ConstraintAdjustment : u32 { const None = 0u32 ; const SlideX = 1u32 ; const SlideY = 2u32 ; const FlipX = 4u32 ; const FlipY = 8u32 ; const ResizeX = 16u32 ; const ResizeY = 32u32 ; } }
        impl TryFrom<u32> for ConstraintAdjustment {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the zxdg_positioner_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgPositionerV6: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_positioner_v6";
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
    pub mod zxdg_surface_v6 {
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
        #[doc = "Trait to implement the zxdg_surface_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgSurfaceV6: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_surface_v6";
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
    #[doc = "This interface defines an xdg_surface role which allows a surface to,"]
    #[doc = "among other things, set window-like properties such as maximize,"]
    #[doc = "fullscreen, and minimize, set application-specific metadata like title and"]
    #[doc = "id, and well as trigger user interactive operations such as interactive"]
    #[doc = "resize and move."]
    pub mod zxdg_toplevel_v6 {
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
        #[doc = "States set in this way are double-buffered. They will get applied on"]
        #[doc = "the next commit."]
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
        #[doc = "Trait to implement the zxdg_toplevel_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgToplevelV6: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_toplevel_v6";
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
    pub mod zxdg_popup_v6 {
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
        #[doc = "Trait to implement the zxdg_popup_v6 interface. See the module level documentation for more info"]
        pub trait ZxdgPopupV6: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zxdg_popup_v6";
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
#[doc = "This protocol allows a privileged client to control data devices. In"]
#[doc = "particular, the client will be able to manage the current selection and take"]
#[doc = "the role of a clipboard manager."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod wlr_data_control_unstable_v1 {
    #[doc = "This interface is a manager that allows creating per-seat data device"]
    #[doc = "controls."]
    pub mod zwlr_data_control_manager_v1 {
        #[doc = "Trait to implement the zwlr_data_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_data_control_manager_v1";
            const VERSION: u32 = 2u32;
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
    #[doc = "This interface allows a client to manage a seat's selection."]
    #[doc = ""]
    #[doc = "When the seat is destroyed, this object becomes inert."]
    pub mod zwlr_data_control_device_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "source given to set_selection or set_primary_selection was already used before"]
            UsedSource = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::UsedSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_data_control_device_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlDeviceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_data_control_device_v1";
            const VERSION: u32 = 2u32;
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
    #[doc = "The wlr_data_control_source object is the source side of a"]
    #[doc = "wlr_data_control_offer. It is created by the source client in a data"]
    #[doc = "transfer and provides a way to describe the offered data and a way to"]
    #[doc = "respond to requests to transfer the data."]
    pub mod zwlr_data_control_source_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "offer sent after wlr_data_control_device.set_selection"]
            InvalidOffer = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidOffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_data_control_source_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlSourceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_data_control_source_v1";
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
    #[doc = "A wlr_data_control_offer represents a piece of data offered for transfer"]
    #[doc = "by another client (the source client). The offer describes the different"]
    #[doc = "MIME types that the data can be converted to and provides the mechanism"]
    #[doc = "for transferring the data directly from the source client."]
    pub mod zwlr_data_control_offer_v1 {
        #[doc = "Trait to implement the zwlr_data_control_offer_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrDataControlOfferV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_data_control_offer_v1";
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
#[doc = "An interface to capture surfaces in an efficient way by exporting DMA-BUFs."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod wlr_export_dmabuf_unstable_v1 {
    #[doc = "This object is a manager with which to start capturing from sources."]
    pub mod zwlr_export_dmabuf_manager_v1 {
        #[doc = "Trait to implement the zwlr_export_dmabuf_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrExportDmabufManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_export_dmabuf_manager_v1";
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
    #[doc = "This object represents a single DMA-BUF frame."]
    #[doc = ""]
    #[doc = "If the capture is successful, the compositor will first send a \"frame\""]
    #[doc = "event, followed by one or several \"object\". When the frame is available"]
    #[doc = "for readout, the \"ready\" event is sent."]
    #[doc = ""]
    #[doc = "If the capture failed, the \"cancel\" event is sent. This can happen anytime"]
    #[doc = "before the \"ready\" event."]
    #[doc = ""]
    #[doc = "Once either a \"ready\" or a \"cancel\" event is received, the client should"]
    #[doc = "destroy the frame. Once an \"object\" event is received, the client is"]
    #[doc = "responsible for closing the associated file descriptor."]
    #[doc = ""]
    #[doc = "All frames are read-only and may not be written into or altered."]
    pub mod zwlr_export_dmabuf_frame_v1 {
        #[doc = "Special flags that should be respected by the client."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Flags {
            #[doc = "clients should copy frame before processing"]
            Transient = 1u32,
        }
        impl TryFrom<u32> for Flags {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Transient),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Indicates reason for cancelling the frame."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum CancelReason {
            #[doc = "temporary error, source will produce more frames"]
            Temporary = 0u32,
            #[doc = "fatal error, source will not produce frames"]
            Permanent = 1u32,
            #[doc = "temporary error, source will produce more frames"]
            Resizing = 2u32,
        }
        impl TryFrom<u32> for CancelReason {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Temporary),
                    1u32 => Ok(Self::Permanent),
                    2u32 => Ok(Self::Resizing),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_export_dmabuf_frame_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrExportDmabufFrameV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_export_dmabuf_frame_v1";
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
pub mod wlr_foreign_toplevel_management_unstable_v1 {
    #[doc = "The purpose of this protocol is to enable the creation of taskbars"]
    #[doc = "and docks by providing them with a list of opened applications and"]
    #[doc = "letting them request certain actions on them, like maximizing, etc."]
    #[doc = ""]
    #[doc = "After a client binds the zwlr_foreign_toplevel_manager_v1, each opened"]
    #[doc = "toplevel window will be sent via the toplevel event"]
    pub mod zwlr_foreign_toplevel_manager_v1 {
        #[doc = "Trait to implement the zwlr_foreign_toplevel_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrForeignToplevelManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_foreign_toplevel_manager_v1";
            const VERSION: u32 = 3u32;
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
    #[doc = "A zwlr_foreign_toplevel_handle_v1 object represents an opened toplevel"]
    #[doc = "window. Each app may have multiple opened toplevels."]
    #[doc = ""]
    #[doc = "Each toplevel has a list of outputs it is visible on, conveyed to the"]
    #[doc = "client with the output_enter and output_leave events."]
    pub mod zwlr_foreign_toplevel_handle_v1 {
        #[doc = "The different states that a toplevel can have. These have the same meaning"]
        #[doc = "as the states with the same names defined in xdg-toplevel"]
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
        #[doc = "Trait to implement the zwlr_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrForeignToplevelHandleV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_foreign_toplevel_handle_v1";
            const VERSION: u32 = 3u32;
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
#[doc = "This protocol allows a privileged client to set the gamma tables for"]
#[doc = "outputs."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod wlr_gamma_control_unstable_v1 {
    #[doc = "This interface is a manager that allows creating per-output gamma"]
    #[doc = "controls."]
    pub mod zwlr_gamma_control_manager_v1 {
        #[doc = "Trait to implement the zwlr_gamma_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrGammaControlManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_gamma_control_manager_v1";
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
    #[doc = "This interface allows a client to adjust gamma tables for a particular"]
    #[doc = "output."]
    #[doc = ""]
    #[doc = "The client will receive the gamma size, and will then be able to set gamma"]
    #[doc = "tables. At any time the compositor can send a failed event indicating that"]
    #[doc = "this object is no longer valid."]
    #[doc = ""]
    #[doc = "There can only be at most one gamma control object per output, which"]
    #[doc = "has exclusive access to this particular output. When the gamma control"]
    #[doc = "object is destroyed, the gamma table is restored to its original value."]
    pub mod zwlr_gamma_control_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid gamma tables"]
            InvalidGamma = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidGamma),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_gamma_control_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrGammaControlV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_gamma_control_v1";
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
pub mod wlr_input_inhibit_unstable_v1 {
    #[doc = "Clients can use this interface to prevent input events from being sent to"]
    #[doc = "any surfaces but its own, which is useful for example in lock screen"]
    #[doc = "software. It is assumed that access to this interface will be locked down"]
    #[doc = "to whitelisted clients by the compositor."]
    #[doc = ""]
    #[doc = "Note! This protocol is deprecated and not intended for production use."]
    #[doc = "For screen lockers, use the ext-session-lock-v1 protocol."]
    pub mod zwlr_input_inhibit_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "an input inhibitor is already in use on the compositor"]
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
        #[doc = "Trait to implement the zwlr_input_inhibit_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrInputInhibitManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_input_inhibit_manager_v1";
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
    #[doc = "While this resource exists, input to clients other than the owner of the"]
    #[doc = "inhibitor resource will not receive input events. Any client which"]
    #[doc = "previously had focus will receive a leave event and will not be given"]
    #[doc = "focus again. The client that owns this resource will receive all input"]
    #[doc = "events normally. The compositor will also disable all of its own input"]
    #[doc = "processing (such as keyboard shortcuts) while the inhibitor is active."]
    #[doc = ""]
    #[doc = "The compositor may continue to send input events to selected clients,"]
    #[doc = "such as an on-screen keyboard (via the input-method protocol)."]
    pub mod zwlr_input_inhibitor_v1 {
        #[doc = "Trait to implement the zwlr_input_inhibitor_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrInputInhibitorV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_input_inhibitor_v1";
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
pub mod wlr_layer_shell_unstable_v1 {
    #[doc = "Clients can use this interface to assign the surface_layer role to"]
    #[doc = "wl_surfaces. Such surfaces are assigned to a \"layer\" of the output and"]
    #[doc = "rendered with a defined z-depth respective to each other. They may also be"]
    #[doc = "anchored to the edges and corners of a screen and specify input handling"]
    #[doc = "semantics. This interface should be suitable for the implementation of"]
    #[doc = "many desktop shell components, and a broad number of other applications"]
    #[doc = "that interact with the desktop."]
    pub mod zwlr_layer_shell_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "wl_surface has another role"]
            Role = 0u32,
            #[doc = "layer value is invalid"]
            InvalidLayer = 1u32,
            #[doc = "wl_surface has a buffer attached or committed"]
            AlreadyConstructed = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::InvalidLayer),
                    2u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "These values indicate which layers a surface can be rendered in. They"]
        #[doc = "are ordered by z depth, bottom-most first. Traditional shell surfaces"]
        #[doc = "will typically be rendered between the bottom and top layers."]
        #[doc = "Fullscreen shell surfaces are typically rendered at the top layer."]
        #[doc = "Multiple surfaces can share a single layer, and ordering within a"]
        #[doc = "single layer is undefined."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Layer {
            Background = 0u32,
            Bottom = 1u32,
            Top = 2u32,
            Overlay = 3u32,
        }
        impl TryFrom<u32> for Layer {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Background),
                    1u32 => Ok(Self::Bottom),
                    2u32 => Ok(Self::Top),
                    3u32 => Ok(Self::Overlay),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_layer_shell_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrLayerShellV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_layer_shell_v1";
            const VERSION: u32 = 5u32;
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
    #[doc = "An interface that may be implemented by a wl_surface, for surfaces that"]
    #[doc = "are designed to be rendered as a layer of a stacked desktop-like"]
    #[doc = "environment."]
    #[doc = ""]
    #[doc = "Layer surface state (layer, size, anchor, exclusive zone,"]
    #[doc = "margin, interactivity) is double-buffered, and will be applied at the"]
    #[doc = "time wl_surface.commit of the corresponding wl_surface is called."]
    #[doc = ""]
    #[doc = "Attaching a null buffer to a layer surface unmaps it."]
    #[doc = ""]
    #[doc = "Unmapping a layer_surface means that the surface cannot be shown by the"]
    #[doc = "compositor until it is explicitly mapped again. The layer_surface"]
    #[doc = "returns to the state it had right after layer_shell.get_layer_surface."]
    #[doc = "The client can re-map the surface by performing a commit without any"]
    #[doc = "buffer attached, waiting for a configure event and handling it as usual."]
    pub mod zwlr_layer_surface_v1 {
        #[doc = "Types of keyboard interaction possible for layer shell surfaces. The"]
        #[doc = "rationale for this is twofold: (1) some applications are not interested"]
        #[doc = "in keyboard events and not allowing them to be focused can improve the"]
        #[doc = "desktop experience; (2) some applications will want to take exclusive"]
        #[doc = "keyboard focus."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum KeyboardInteractivity {
            None = 0u32,
            Exclusive = 1u32,
            OnDemand = 2u32,
        }
        impl TryFrom<u32> for KeyboardInteractivity {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Exclusive),
                    2u32 => Ok(Self::OnDemand),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "provided surface state is invalid"]
            InvalidSurfaceState = 0u32,
            #[doc = "size is invalid"]
            InvalidSize = 1u32,
            #[doc = "anchor bitfield is invalid"]
            InvalidAnchor = 2u32,
            #[doc = "keyboard interactivity is invalid"]
            InvalidKeyboardInteractivity = 3u32,
            #[doc = "exclusive edge is invalid given the surface anchors"]
            InvalidExclusiveEdge = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSurfaceState),
                    1u32 => Ok(Self::InvalidSize),
                    2u32 => Ok(Self::InvalidAnchor),
                    3u32 => Ok(Self::InvalidKeyboardInteractivity),
                    4u32 => Ok(Self::InvalidExclusiveEdge),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Anchor : u32 { # [doc = "the top edge of the anchor rectangle"] const Top = 1u32 ; # [doc = "the bottom edge of the anchor rectangle"] const Bottom = 2u32 ; # [doc = "the left edge of the anchor rectangle"] const Left = 4u32 ; # [doc = "the right edge of the anchor rectangle"] const Right = 8u32 ; } }
        impl TryFrom<u32> for Anchor {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        #[doc = "Trait to implement the zwlr_layer_surface_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrLayerSurfaceV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_layer_surface_v1";
            const VERSION: u32 = 5u32;
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
#[doc = "This protocol exposes interfaces to obtain and modify output device"]
#[doc = "configuration."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod wlr_output_management_unstable_v1 {
    #[doc = "This interface is a manager that allows reading and writing the current"]
    #[doc = "output device configuration."]
    #[doc = ""]
    #[doc = "Output devices that display pixels (e.g. a physical monitor or a virtual"]
    #[doc = "output in a window) are represented as heads. Heads cannot be created nor"]
    #[doc = "destroyed by the client, but they can be enabled or disabled and their"]
    #[doc = "properties can be changed. Each head may have one or more available modes."]
    #[doc = ""]
    #[doc = "Whenever a head appears (e.g. a monitor is plugged in), it will be"]
    #[doc = "advertised via the head event. Immediately after the output manager is"]
    #[doc = "bound, all current heads are advertised."]
    #[doc = ""]
    #[doc = "Whenever a head's properties change, the relevant wlr_output_head events"]
    #[doc = "will be sent. Not all head properties will be sent: only properties that"]
    #[doc = "have changed need to."]
    #[doc = ""]
    #[doc = "Whenever a head disappears (e.g. a monitor is unplugged), a"]
    #[doc = "wlr_output_head.finished event will be sent."]
    #[doc = ""]
    #[doc = "After one or more heads appear, change or disappear, the done event will"]
    #[doc = "be sent. It carries a serial which can be used in a create_configuration"]
    #[doc = "request to update heads properties."]
    #[doc = ""]
    #[doc = "The information obtained from this protocol should only be used for output"]
    #[doc = "configuration purposes. This protocol is not designed to be a generic"]
    #[doc = "output property advertisement protocol for regular clients. Instead,"]
    #[doc = "protocols such as xdg-output should be used."]
    pub mod zwlr_output_manager_v1 {
        #[doc = "Trait to implement the zwlr_output_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_manager_v1";
            const VERSION: u32 = 4u32;
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
    #[doc = "A head is an output device. The difference between a wl_output object and"]
    #[doc = "a head is that heads are advertised even if they are turned off. A head"]
    #[doc = "object only advertises properties and cannot be used directly to change"]
    #[doc = "them."]
    #[doc = ""]
    #[doc = "A head has some read-only properties: modes, name, description and"]
    #[doc = "physical_size. These cannot be changed by clients."]
    #[doc = ""]
    #[doc = "Other properties can be updated via a wlr_output_configuration object."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the"]
    #[doc = "wlr_output_manager.done event. No guarantees are made regarding the order"]
    #[doc = "in which properties are sent."]
    pub mod zwlr_output_head_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AdaptiveSyncState {
            #[doc = "adaptive sync is disabled"]
            Disabled = 0u32,
            #[doc = "adaptive sync is enabled"]
            Enabled = 1u32,
        }
        impl TryFrom<u32> for AdaptiveSyncState {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_output_head_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputHeadV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_head_v1";
            const VERSION: u32 = 4u32;
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
    #[doc = "This object describes an output mode."]
    #[doc = ""]
    #[doc = "Some heads don't support output modes, in which case modes won't be"]
    #[doc = "advertised."]
    #[doc = ""]
    #[doc = "Properties sent via this interface are applied atomically via the"]
    #[doc = "wlr_output_manager.done event. No guarantees are made regarding the order"]
    #[doc = "in which properties are sent."]
    pub mod zwlr_output_mode_v1 {
        #[doc = "Trait to implement the zwlr_output_mode_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputModeV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_mode_v1";
            const VERSION: u32 = 3u32;
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
    #[doc = "This object is used by the client to describe a full output configuration."]
    #[doc = ""]
    #[doc = "First, the client needs to setup the output configuration. Each head can"]
    #[doc = "be either enabled (and configured) or disabled. It is a protocol error to"]
    #[doc = "send two enable_head or disable_head requests with the same head. It is a"]
    #[doc = "protocol error to omit a head in a configuration."]
    #[doc = ""]
    #[doc = "Then, the client can apply or test the configuration. The compositor will"]
    #[doc = "then reply with a succeeded, failed or cancelled event. Finally the client"]
    #[doc = "should destroy the configuration object."]
    pub mod zwlr_output_configuration_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "head has been configured twice"]
            AlreadyConfiguredHead = 1u32,
            #[doc = "head has not been configured"]
            UnconfiguredHead = 2u32,
            #[doc = "request sent after configuration has been applied or tested"]
            AlreadyUsed = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadyConfiguredHead),
                    2u32 => Ok(Self::UnconfiguredHead),
                    3u32 => Ok(Self::AlreadyUsed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_output_configuration_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputConfigurationV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_configuration_v1";
            const VERSION: u32 = 4u32;
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
    #[doc = "This object is used by the client to update a single head's configuration."]
    #[doc = ""]
    #[doc = "It is a protocol error to set the same property twice."]
    pub mod zwlr_output_configuration_head_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "property has already been set"]
            AlreadySet = 1u32,
            #[doc = "mode doesn't belong to head"]
            InvalidMode = 2u32,
            #[doc = "mode is invalid"]
            InvalidCustomMode = 3u32,
            #[doc = "transform value outside enum"]
            InvalidTransform = 4u32,
            #[doc = "scale negative or zero"]
            InvalidScale = 5u32,
            #[doc = "invalid enum value used in the set_adaptive_sync request"]
            InvalidAdaptiveSyncState = 6u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlreadySet),
                    2u32 => Ok(Self::InvalidMode),
                    3u32 => Ok(Self::InvalidCustomMode),
                    4u32 => Ok(Self::InvalidTransform),
                    5u32 => Ok(Self::InvalidScale),
                    6u32 => Ok(Self::InvalidAdaptiveSyncState),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_output_configuration_head_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputConfigurationHeadV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_configuration_head_v1";
            const VERSION: u32 = 4u32;
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
#[doc = "This protocol allows clients to control power management modes"]
#[doc = "of outputs that are currently part of the compositor space. The"]
#[doc = "intent is to allow special clients like desktop shells to power"]
#[doc = "down outputs when the system is idle."]
#[doc = ""]
#[doc = "To modify outputs not currently part of the compositor space see"]
#[doc = "wlr-output-management."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod wlr_output_power_management_unstable_v1 {
    #[doc = "This interface is a manager that allows creating per-output power"]
    #[doc = "management mode controls."]
    pub mod zwlr_output_power_manager_v1 {
        #[doc = "Trait to implement the zwlr_output_power_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputPowerManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_power_manager_v1";
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
    #[doc = "This object offers requests to set the power management mode of"]
    #[doc = "an output."]
    pub mod zwlr_output_power_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "Output is turned off."]
            Off = 0u32,
            #[doc = "Output is turned on, no power saving"]
            On = 1u32,
        }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Off),
                    1u32 => Ok(Self::On),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "nonexistent power save mode"]
            InvalidMode = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidMode),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_output_power_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrOutputPowerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_output_power_v1";
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
#[doc = "This protocol allows clients to ask the compositor to copy part of the"]
#[doc = "screen content to a client buffer."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is experimental and"]
#[doc = "backward incompatible changes may be made. Backward compatible changes"]
#[doc = "may be added together with the corresponding interface version bump."]
#[doc = "Backward incompatible changes are done by bumping the version number in"]
#[doc = "the protocol and interface names and resetting the interface version."]
#[doc = "Once the protocol is to be declared stable, the 'z' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
pub mod wlr_screencopy_unstable_v1 {
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    pub mod zwlr_screencopy_manager_v1 {
        #[doc = "Trait to implement the zwlr_screencopy_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrScreencopyManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_screencopy_manager_v1";
            const VERSION: u32 = 3u32;
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
    #[doc = "This object represents a single frame."]
    #[doc = ""]
    #[doc = "When created, a series of buffer events will be sent, each representing a"]
    #[doc = "supported buffer type. The \"buffer_done\" event is sent afterwards to"]
    #[doc = "indicate that all supported buffer types have been enumerated. The client"]
    #[doc = "will then be able to send a \"copy\" request. If the capture is successful,"]
    #[doc = "the compositor will send a \"flags\" followed by a \"ready\" event."]
    #[doc = ""]
    #[doc = "For objects version 2 or lower, wl_shm buffers are always supported, ie."]
    #[doc = "the \"buffer\" event is guaranteed to be sent."]
    #[doc = ""]
    #[doc = "If the capture failed, the \"failed\" event is sent. This can happen anytime"]
    #[doc = "before the \"ready\" event."]
    #[doc = ""]
    #[doc = "Once either a \"ready\" or a \"failed\" event is received, the client should"]
    #[doc = "destroy the frame."]
    pub mod zwlr_screencopy_frame_v1 {
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
        #[doc = "Trait to implement the zwlr_screencopy_frame_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrScreencopyFrameV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_screencopy_frame_v1";
            const VERSION: u32 = 3u32;
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
pub mod wlr_virtual_pointer_unstable_v1 {
    #[doc = "This protocol allows clients to emulate a physical pointer device. The"]
    #[doc = "requests are mostly mirror opposites of those specified in wl_pointer."]
    pub mod zwlr_virtual_pointer_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "client sent invalid axis enumeration value"]
            InvalidAxis = 0u32,
            #[doc = "client sent invalid axis source enumeration value"]
            InvalidAxisSource = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidAxis),
                    1u32 => Ok(Self::InvalidAxisSource),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        #[doc = "Trait to implement the zwlr_virtual_pointer_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrVirtualPointerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_virtual_pointer_v1";
            const VERSION: u32 = 2u32;
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
    #[doc = "This object allows clients to create individual virtual pointer objects."]
    pub mod zwlr_virtual_pointer_manager_v1 {
        #[doc = "Trait to implement the zwlr_virtual_pointer_manager_v1 interface. See the module level documentation for more info"]
        pub trait ZwlrVirtualPointerManagerV1: crate::client::Dispatcher {
            const INTERFACE: &'static str = "zwlr_virtual_pointer_manager_v1";
            const VERSION: u32 = 2u32;
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
