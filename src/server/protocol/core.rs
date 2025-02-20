#![allow(async_fn_in_trait)]
#[allow(clippy::module_inception)]
pub mod wayland {
    #[doc = "The core global object.  This is a special singleton object.  It"]
    #[doc = "is used for internal Wayland protocol features."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_display {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_display interface. See the module level documentation for more info"]
        pub trait WlDisplay: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_display";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let callback = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_display#{}.sync({})", sender_id, callback);
                        self.sync(client, sender_id, callback).await
                    }
                    1u16 => {
                        let registry = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_display#{}.get_registry({})", sender_id, registry);
                        self.get_registry(client, sender_id, registry).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                callback: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                registry: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "The error event is sent out when a fatal (non-recoverable)"]
            #[doc = "error has occurred.  The object_id argument is the object"]
            #[doc = "where the error occurred, most often in response to a request"]
            #[doc = "to that object.  The code identifies the error and is defined"]
            #[doc = "by the object interface.  As such, each interface defines its"]
            #[doc = "own set of error codes.  The message is a brief description"]
            #[doc = "of the error, for (debugging) convenience."]
            async fn error(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                object_id: crate::wire::ObjectId,
                code: u32,
                message: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_display#{}.error({}, {}, \"{}\")",
                    sender_id,
                    object_id,
                    code,
                    message
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(object_id))
                    .put_uint(code)
                    .put_string(Some(message))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is used internally by the object ID management"]
            #[doc = "logic. When a client deletes an object that it had created,"]
            #[doc = "the server will send this event to acknowledge that it has"]
            #[doc = "seen the delete request. When the client receives this event,"]
            #[doc = "it will know that it can safely reuse the object ID."]
            async fn delete_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_display#{}.delete_id({})", sender_id, id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(id).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_registry interface. See the module level documentation for more info"]
        pub trait WlRegistry: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_registry";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let name = message.uint()?;
                        let id = message.new_id()?;
                        tracing::debug!("wl_registry#{}.bind({}, {})", sender_id, name, id);
                        self.bind(client, sender_id, name, id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Binds a new, client-created object to the server using the"]
            #[doc = "specified name as the identifier."]
            async fn bind(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: u32,
                id: crate::wire::NewId,
            ) -> crate::server::Result<()>;
            #[doc = "Notify the client of global objects."]
            #[doc = ""]
            #[doc = "The event notifies the client that a global object with"]
            #[doc = "the given name is now available, and it implements the"]
            #[doc = "given version of the given interface."]
            async fn global(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: u32,
                interface: String,
                version: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_registry#{}.global({}, \"{}\", {})",
                    sender_id,
                    name,
                    interface,
                    version
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(name)
                    .put_string(Some(interface))
                    .put_uint(version)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notify the client of removed global objects."]
            #[doc = ""]
            #[doc = "This event notifies the client that the global identified"]
            #[doc = "by name is no longer available.  If the client bound to"]
            #[doc = "the global using the bind request, the client should now"]
            #[doc = "destroy that object."]
            #[doc = ""]
            #[doc = "The object remains valid and requests to the object will be"]
            #[doc = "ignored until the client destroys it, to avoid races between"]
            #[doc = "the global going away and a client sending a request to it."]
            async fn global_remove(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_registry#{}.global_remove({})", sender_id, name);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(name).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_callback interface. See the module level documentation for more info"]
        pub trait WlCallback: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_callback";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Notify the client when the related request is done."]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                callback_data: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_callback#{}.done({})", sender_id, callback_data);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(callback_data)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A compositor.  This object is a singleton global.  The"]
    #[doc = "compositor is in charge of combining the contents of multiple"]
    #[doc = "surfaces into one displayable output."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_compositor {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_compositor interface. See the module level documentation for more info"]
        pub trait WlCompositor: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_compositor";
            const VERSION: u32 = 6u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_compositor#{}.create_surface({})", sender_id, id);
                        self.create_surface(client, sender_id, id).await
                    }
                    1u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_compositor#{}.create_region({})", sender_id, id);
                        self.create_region(client, sender_id, id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Ask the compositor to create a new surface."]
            async fn create_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Ask the compositor to create a new region."]
            async fn create_region(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_shm_pool interface. See the module level documentation for more info"]
        pub trait WlShmPool: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_shm_pool";
            const VERSION: u32 = 2u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let offset = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        let stride = message.int()?;
                        let format = message.uint()?;
                        tracing::debug!(
                            "wl_shm_pool#{}.create_buffer({}, {}, {}, {}, {}, {})",
                            sender_id,
                            id,
                            offset,
                            width,
                            height,
                            stride,
                            format
                        );
                        self.create_buffer(
                            client,
                            sender_id,
                            id,
                            offset,
                            width,
                            height,
                            stride,
                            format.try_into()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("wl_shm_pool#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    2u16 => {
                        let size = message.int()?;
                        tracing::debug!("wl_shm_pool#{}.resize({})", sender_id, size);
                        self.resize(client, sender_id, size).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                offset: i32,
                width: i32,
                height: i32,
                stride: i32,
                format: super::super::super::core::wayland::wl_shm::Format,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the shared memory pool."]
            #[doc = ""]
            #[doc = "The mmapped memory will be released when all"]
            #[doc = "buffers that have been created from this pool"]
            #[doc = "are gone."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                size: i32,
            ) -> crate::server::Result<()>;
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for Format {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_shm interface. See the module level documentation for more info"]
        pub trait WlShm: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_shm";
            const VERSION: u32 = 2u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let fd = message.fd()?;
                        let size = message.int()?;
                        tracing::debug!(
                            "wl_shm#{}.create_pool({}, {}, {})",
                            sender_id,
                            id,
                            fd.as_raw_fd(),
                            size
                        );
                        self.create_pool(client, sender_id, id, fd, size).await
                    }
                    1u16 => {
                        tracing::debug!("wl_shm#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a new wl_shm_pool object."]
            #[doc = ""]
            #[doc = "The pool can be used to create shared memory based buffer"]
            #[doc = "objects.  The server will mmap size bytes of the passed file"]
            #[doc = "descriptor, to use as backing memory for the pool."]
            async fn create_pool(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
                size: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the shm object anymore."]
            #[doc = ""]
            #[doc = "Objects created via this interface remain unaffected."]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Informs the client about a valid pixel format that"]
            #[doc = "can be used for buffers. Known formats include"]
            #[doc = "argb8888 and xrgb8888."]
            async fn format(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: Format,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_shm#{}.format({})", sender_id, format);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(format as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_buffer interface. See the module level documentation for more info"]
        pub trait WlBuffer: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_buffer";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_buffer#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy a buffer. If and how you need to release the backing"]
            #[doc = "storage is defined by the buffer factory interface."]
            #[doc = ""]
            #[doc = "For possible side-effects to a surface, see wl_surface.attach."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Sent when this wl_buffer is no longer used by the compositor."]
            #[doc = "The client is now free to reuse or destroy this buffer and its"]
            #[doc = "backing storage."]
            #[doc = ""]
            #[doc = "If a client receives a release event before the frame callback"]
            #[doc = "requested in the same wl_surface.commit that attaches this"]
            #[doc = "wl_buffer to a surface, then the client is immediately free to"]
            #[doc = "reuse the buffer and its backing storage, and does not need a"]
            #[doc = "second buffer for the next surface content update. Typically"]
            #[doc = "this is possible, when the compositor maintains a copy of the"]
            #[doc = "wl_surface contents, e.g. as a GL texture. This is an important"]
            #[doc = "optimization for GL(ES) compositors with wl_shm clients."]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_buffer#{}.release()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_data_offer interface. See the module level documentation for more info"]
        pub trait WlDataOffer: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_data_offer";
            const VERSION: u32 = 3u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let serial = message.uint()?;
                        let mime_type = message.string()?;
                        tracing::debug!(
                            "wl_data_offer#{}.accept({}, \"{}\")",
                            sender_id,
                            serial,
                            mime_type
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string())
                        );
                        self.accept(client, sender_id, serial, mime_type).await
                    }
                    1u16 => {
                        let mime_type = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let fd = message.fd()?;
                        tracing::debug!(
                            "wl_data_offer#{}.receive(\"{}\", {})",
                            sender_id,
                            mime_type,
                            fd.as_raw_fd()
                        );
                        self.receive(client, sender_id, mime_type, fd).await
                    }
                    2u16 => {
                        tracing::debug!("wl_data_offer#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    3u16 => {
                        tracing::debug!("wl_data_offer#{}.finish()", sender_id,);
                        self.finish(client, sender_id).await
                    }
                    4u16 => {
                        let dnd_actions = message.uint()?;
                        let preferred_action = message.uint()?;
                        tracing::debug!(
                            "wl_data_offer#{}.set_actions({}, {})",
                            sender_id,
                            dnd_actions,
                            preferred_action
                        );
                        self.set_actions(
                            client,
                            sender_id,
                            dnd_actions.try_into()?,
                            preferred_action.try_into()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                mime_type: Option<String>,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the data offer."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                dnd_actions: super::super::super::core::wayland::wl_data_device_manager::DndAction,
                preferred_action : super :: super :: super :: core :: wayland :: wl_data_device_manager :: DndAction,
            ) -> crate::server::Result<()>;
            #[doc = "Sent immediately after creating the wl_data_offer object.  One"]
            #[doc = "event per offered mime type."]
            async fn offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.offer(\"{}\")", sender_id, mime_type);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates the actions offered by the data source. It"]
            #[doc = "will be sent immediately after creating the wl_data_offer object,"]
            #[doc = "or anytime the source side changes its offered actions through"]
            #[doc = "wl_data_source.set_actions."]
            async fn source_actions(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source_actions : super :: super :: super :: core :: wayland :: wl_data_device_manager :: DndAction,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_data_offer#{}.source_actions({})",
                    sender_id,
                    source_actions
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(source_actions.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates the action selected by the compositor after"]
            #[doc = "matching the source/destination side actions. Only one action (or"]
            #[doc = "none) will be offered here."]
            #[doc = ""]
            #[doc = "This event can be emitted multiple times during the drag-and-drop"]
            #[doc = "operation in response to destination side action changes through"]
            #[doc = "wl_data_offer.set_actions."]
            #[doc = ""]
            #[doc = "This event will no longer be emitted after wl_data_device.drop"]
            #[doc = "happened on the drag-and-drop destination, the client must"]
            #[doc = "honor the last action received, or the last preferred one set"]
            #[doc = "through wl_data_offer.set_actions when handling an \"ask\" action."]
            #[doc = ""]
            #[doc = "Compositors may also change the selected action on the fly, mainly"]
            #[doc = "in response to keyboard modifier changes during the drag-and-drop"]
            #[doc = "operation."]
            #[doc = ""]
            #[doc = "The most recent action received is always the valid one. Prior to"]
            #[doc = "receiving wl_data_device.drop, the chosen action may change (e.g."]
            #[doc = "due to keyboard modifiers being pressed). At the time of receiving"]
            #[doc = "wl_data_device.drop the drag-and-drop destination must honor the"]
            #[doc = "last action received."]
            #[doc = ""]
            #[doc = "Action changes may still happen after wl_data_device.drop,"]
            #[doc = "especially on \"ask\" actions, where the drag-and-drop destination"]
            #[doc = "may choose another action afterwards. Action changes happening"]
            #[doc = "at this stage are always the result of inter-client negotiation, the"]
            #[doc = "compositor shall no longer be able to induce a different action."]
            #[doc = ""]
            #[doc = "Upon \"ask\" actions, it is expected that the drag-and-drop destination"]
            #[doc = "may potentially choose a different action and/or mime type,"]
            #[doc = "based on wl_data_offer.source_actions and finally chosen by the"]
            #[doc = "user (e.g. popping up a menu with the available options). The"]
            #[doc = "final wl_data_offer.set_actions and wl_data_offer.accept requests"]
            #[doc = "must happen before the call to wl_data_offer.finish."]
            async fn action(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                dnd_action: super::super::super::core::wayland::wl_data_device_manager::DndAction,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_offer#{}.action({})", sender_id, dnd_action);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(dnd_action.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "The wl_data_source object is the source side of a wl_data_offer."]
    #[doc = "It is created by the source client in a data transfer and"]
    #[doc = "provides a way to describe the offered data and a way to respond"]
    #[doc = "to requests to transfer the data."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_data_source {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_data_source interface. See the module level documentation for more info"]
        pub trait WlDataSource: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_data_source";
            const VERSION: u32 = 3u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let mime_type = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_data_source#{}.offer(\"{}\")", sender_id, mime_type);
                        self.offer(client, sender_id, mime_type).await
                    }
                    1u16 => {
                        tracing::debug!("wl_data_source#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    2u16 => {
                        let dnd_actions = message.uint()?;
                        tracing::debug!(
                            "wl_data_source#{}.set_actions({})",
                            sender_id,
                            dnd_actions
                        );
                        self.set_actions(client, sender_id, dnd_actions.try_into()?)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This request adds a mime type to the set of mime types"]
            #[doc = "advertised to targets.  Can be called several times to offer"]
            #[doc = "multiple types."]
            async fn offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::server::Result<()>;
            #[doc = "Destroy the data source."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                dnd_actions: super::super::super::core::wayland::wl_data_device_manager::DndAction,
            ) -> crate::server::Result<()>;
            #[doc = "Sent when a target accepts pointer_focus or motion events.  If"]
            #[doc = "a target does not accept any of the offered types, type is NULL."]
            #[doc = ""]
            #[doc = "Used for feedback during drag-and-drop."]
            async fn target(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: Option<String>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_data_source#{}.target(\"{}\")",
                    sender_id,
                    mime_type
                        .as_ref()
                        .map_or("null".to_string(), |v| v.to_string())
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(mime_type)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Request for data from the client.  Send the data as the"]
            #[doc = "specified mime type over the passed file descriptor, then"]
            #[doc = "close it."]
            async fn send(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_data_source#{}.send(\"{}\", {})",
                    sender_id,
                    mime_type,
                    fd.as_raw_fd()
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .put_fd(fd)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This data source is no longer valid. There are several reasons why"]
            #[doc = "this could happen:"]
            #[doc = ""]
            #[doc = "- The data source has been replaced by another data source."]
            #[doc = "- The drag-and-drop operation was performed, but the drop destination"]
            #[doc = "did not accept any of the mime types offered through"]
            #[doc = "wl_data_source.target."]
            #[doc = "- The drag-and-drop operation was performed, but the drop destination"]
            #[doc = "did not select any of the actions present in the mask offered through"]
            #[doc = "wl_data_source.action."]
            #[doc = "- The drag-and-drop operation was performed but didn't happen over a"]
            #[doc = "surface."]
            #[doc = "- The compositor cancelled the drag-and-drop operation (e.g. compositor"]
            #[doc = "dependent timeouts to avoid stale drag-and-drop transfers)."]
            #[doc = ""]
            #[doc = "The client should clean up and destroy this data source."]
            #[doc = ""]
            #[doc = "For objects of version 2 or older, wl_data_source.cancelled will"]
            #[doc = "only be emitted if the data source was replaced by another data"]
            #[doc = "source."]
            async fn cancelled(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_source#{}.cancelled()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The user performed the drop action. This event does not indicate"]
            #[doc = "acceptance, wl_data_source.cancelled may still be emitted afterwards"]
            #[doc = "if the drop destination does not accept any mime type."]
            #[doc = ""]
            #[doc = "However, this event might however not be received if the compositor"]
            #[doc = "cancelled the drag-and-drop operation before this event could happen."]
            #[doc = ""]
            #[doc = "Note that the data_source may still be used in the future and should"]
            #[doc = "not be destroyed here."]
            async fn dnd_drop_performed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_source#{}.dnd_drop_performed()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The drop destination finished interoperating with this data"]
            #[doc = "source, so the client is now free to destroy this data source and"]
            #[doc = "free all associated data."]
            #[doc = ""]
            #[doc = "If the action used to perform the operation was \"move\", the"]
            #[doc = "source can now delete the transferred data."]
            async fn dnd_finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_source#{}.dnd_finished()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates the action selected by the compositor after"]
            #[doc = "matching the source/destination side actions. Only one action (or"]
            #[doc = "none) will be offered here."]
            #[doc = ""]
            #[doc = "This event can be emitted multiple times during the drag-and-drop"]
            #[doc = "operation, mainly in response to destination side changes through"]
            #[doc = "wl_data_offer.set_actions, and as the data device enters/leaves"]
            #[doc = "surfaces."]
            #[doc = ""]
            #[doc = "It is only possible to receive this event after"]
            #[doc = "wl_data_source.dnd_drop_performed if the drag-and-drop operation"]
            #[doc = "ended in an \"ask\" action, in which case the final wl_data_source.action"]
            #[doc = "event will happen immediately before wl_data_source.dnd_finished."]
            #[doc = ""]
            #[doc = "Compositors may also change the selected action on the fly, mainly"]
            #[doc = "in response to keyboard modifier changes during the drag-and-drop"]
            #[doc = "operation."]
            #[doc = ""]
            #[doc = "The most recent action received is always the valid one. The chosen"]
            #[doc = "action may change alongside negotiation (e.g. an \"ask\" action can turn"]
            #[doc = "into a \"move\" operation), so the effects of the final action must"]
            #[doc = "always be applied in wl_data_offer.dnd_finished."]
            #[doc = ""]
            #[doc = "Clients can trigger cursor surface changes from this point, so"]
            #[doc = "they reflect the current action."]
            async fn action(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                dnd_action: super::super::super::core::wayland::wl_data_device_manager::DndAction,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_source#{}.action({})", sender_id, dnd_action);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(dnd_action.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_data_device interface. See the module level documentation for more info"]
        pub trait WlDataDevice: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_data_device";
            const VERSION: u32 = 3u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let source = message.object()?;
                        let origin = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let icon = message.object()?;
                        let serial = message.uint()?;
                        tracing::debug!(
                            "wl_data_device#{}.start_drag({}, {}, {}, {})",
                            sender_id,
                            source
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string()),
                            origin,
                            icon.as_ref().map_or("null".to_string(), |v| v.to_string()),
                            serial
                        );
                        self.start_drag(client, sender_id, source, origin, icon, serial)
                            .await
                    }
                    1u16 => {
                        let source = message.object()?;
                        let serial = message.uint()?;
                        tracing::debug!(
                            "wl_data_device#{}.set_selection({}, {})",
                            sender_id,
                            source
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string()),
                            serial
                        );
                        self.set_selection(client, sender_id, source, serial).await
                    }
                    2u16 => {
                        tracing::debug!("wl_data_device#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
                origin: crate::wire::ObjectId,
                icon: Option<crate::wire::ObjectId>,
                serial: u32,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
                serial: u32,
            ) -> crate::server::Result<()>;
            #[doc = "This request destroys the data device."]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "The data_offer event introduces a new wl_data_offer object,"]
            #[doc = "which will subsequently be used in either the"]
            #[doc = "data_device.enter event (for drag-and-drop) or the"]
            #[doc = "data_device.selection event (for selections).  Immediately"]
            #[doc = "following the data_device.data_offer event, the new data_offer"]
            #[doc = "object will send out data_offer.offer events to describe the"]
            #[doc = "mime types it offers."]
            async fn data_offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_device#{}.data_offer({})", sender_id, id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent when an active drag-and-drop pointer enters"]
            #[doc = "a surface owned by the client.  The position of the pointer at"]
            #[doc = "enter time is provided by the x and y arguments, in surface-local"]
            #[doc = "coordinates."]
            async fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                surface: crate::wire::ObjectId,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_data_device#{}.enter({}, {}, {}, {}, {})",
                    sender_id,
                    serial,
                    surface,
                    x,
                    y,
                    id.as_ref().map_or("null".to_string(), |v| v.to_string())
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .put_fixed(x)
                    .put_fixed(y)
                    .put_object(id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent when the drag-and-drop pointer leaves the"]
            #[doc = "surface and the session ends.  The client must destroy the"]
            #[doc = "wl_data_offer introduced at enter time at this point."]
            async fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_device#{}.leave()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent when the drag-and-drop pointer moves within"]
            #[doc = "the currently focused surface. The new position of the pointer"]
            #[doc = "is provided by the x and y arguments, in surface-local"]
            #[doc = "coordinates."]
            async fn motion(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_data_device#{}.motion({}, {}, {})",
                    sender_id,
                    time,
                    x,
                    y
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The event is sent when a drag-and-drop operation is ended"]
            #[doc = "because the implicit grab is removed."]
            #[doc = ""]
            #[doc = "The drag-and-drop destination is expected to honor the last action"]
            #[doc = "received through wl_data_offer.action, if the resulting action is"]
            #[doc = "\"copy\" or \"move\", the destination can still perform"]
            #[doc = "wl_data_offer.receive requests, and is expected to end all"]
            #[doc = "transfers with a wl_data_offer.finish request."]
            #[doc = ""]
            #[doc = "If the resulting action is \"ask\", the action will not be considered"]
            #[doc = "final. The drag-and-drop destination is expected to perform one last"]
            #[doc = "wl_data_offer.set_actions request, or wl_data_offer.destroy in order"]
            #[doc = "to cancel the operation."]
            async fn drop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_data_device#{}.drop()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The selection event is sent out to notify the client of a new"]
            #[doc = "wl_data_offer for the selection for this device.  The"]
            #[doc = "data_device.data_offer and the data_offer.offer events are"]
            #[doc = "sent out immediately before this event to introduce the data"]
            #[doc = "offer object.  The selection event is sent to a client"]
            #[doc = "immediately before receiving keyboard focus and when a new"]
            #[doc = "selection is set while the client has keyboard focus.  The"]
            #[doc = "data_offer is valid until a new data_offer or NULL is received"]
            #[doc = "or until the client loses keyboard focus.  Switching surface with"]
            #[doc = "keyboard focus within the same client doesn't mean a new selection"]
            #[doc = "will be sent.  The client must destroy the previous selection"]
            #[doc = "data_offer, if any, upon receiving this event."]
            async fn selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_data_device#{}.selection({})",
                    sender_id,
                    id.as_ref().map_or("null".to_string(), |v| v.to_string())
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_object(id).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [doc = "This is a bitmask of the available/preferred actions in a"] # [doc = "drag-and-drop operation."] # [doc = ""] # [doc = "In the compositor, the selected action is a result of matching the"] # [doc = "actions offered by the source and destination sides.  \"action\" events"] # [doc = "with a \"none\" action will be sent to both source and destination if"] # [doc = "there is no match. All further checks will effectively happen on"] # [doc = "(source actions ∩ destination actions)."] # [doc = ""] # [doc = "In addition, compositors may also pick different actions in"] # [doc = "reaction to key modifiers being pressed. One common design that"] # [doc = "is used in major toolkits (and the behavior recommended for"] # [doc = "compositors) is:"] # [doc = ""] # [doc = "- If no modifiers are pressed, the first match (in bit order)"] # [doc = "will be used."] # [doc = "- Pressing Shift selects \"move\", if enabled in the mask."] # [doc = "- Pressing Control selects \"copy\", if enabled in the mask."] # [doc = ""] # [doc = "Behavior beyond that is considered implementation-dependent."] # [doc = "Compositors may for example bind other modifiers (like Alt/Meta)"] # [doc = "or drags initiated with other buttons than BTN_LEFT to specific"] # [doc = "actions (e.g. \"ask\")."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct DndAction : u32 { # [doc = "no action"] const None = 0u32 ; # [doc = "copy action"] const Copy = 1u32 ; # [doc = "move action"] const Move = 2u32 ; # [doc = "ask action"] const Ask = 4u32 ; } }
        impl TryFrom<u32> for DndAction {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for DndAction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_data_device_manager interface. See the module level documentation for more info"]
        pub trait WlDataDeviceManager: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_data_device_manager";
            const VERSION: u32 = 3u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wl_data_device_manager#{}.create_data_source({})",
                            sender_id,
                            id
                        );
                        self.create_data_source(client, sender_id, id).await
                    }
                    1u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let seat = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wl_data_device_manager#{}.get_data_device({}, {})",
                            sender_id,
                            id,
                            seat
                        );
                        self.get_data_device(client, sender_id, id, seat).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a new data source."]
            async fn create_data_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Create a new data device for a given seat."]
            async fn get_data_device(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        #[doc = "Trait to implement the wl_shell interface. See the module level documentation for more info"]
        pub trait WlShell: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_shell";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let surface = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wl_shell#{}.get_shell_surface({}, {})",
                            sender_id,
                            id,
                            surface
                        );
                        self.get_shell_surface(client, sender_id, id, surface).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Create a shell surface for an existing surface. This gives"]
            #[doc = "the wl_surface the role of a shell surface. If the wl_surface"]
            #[doc = "already has another role, it raises a protocol error."]
            #[doc = ""]
            #[doc = "Only one shell surface can be associated with a given surface."]
            async fn get_shell_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [doc = "These values are used to indicate which edge of a surface"] # [doc = "is being dragged in a resize operation. The server may"] # [doc = "use this information to adapt its behavior, e.g. choose"] # [doc = "an appropriate cursor image."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Resize : u32 { # [doc = "no edge"] const None = 0u32 ; # [doc = "top edge"] const Top = 1u32 ; # [doc = "bottom edge"] const Bottom = 2u32 ; # [doc = "left edge"] const Left = 4u32 ; # [doc = "top and left edges"] const TopLeft = 5u32 ; # [doc = "bottom and left edges"] const BottomLeft = 6u32 ; # [doc = "right edge"] const Right = 8u32 ; # [doc = "top and right edges"] const TopRight = 9u32 ; # [doc = "bottom and right edges"] const BottomRight = 10u32 ; } }
        impl TryFrom<u32> for Resize {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Resize {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "These flags specify details of the expected behaviour"] # [doc = "of transient surfaces. Used in the set_transient request."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Transient : u32 { # [doc = "do not set keyboard focus"] const Inactive = 1u32 ; } }
        impl TryFrom<u32> for Transient {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Transient {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
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
        impl std::fmt::Display for FullscreenMethod {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_shell_surface interface. See the module level documentation for more info"]
        pub trait WlShellSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_shell_surface";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let serial = message.uint()?;
                        tracing::debug!("wl_shell_surface#{}.pong({})", sender_id, serial);
                        self.pong(client, sender_id, serial).await
                    }
                    1u16 => {
                        let seat = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let serial = message.uint()?;
                        tracing::debug!(
                            "wl_shell_surface#{}.move({}, {})",
                            sender_id,
                            seat,
                            serial
                        );
                        self.r#move(client, sender_id, seat, serial).await
                    }
                    2u16 => {
                        let seat = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let serial = message.uint()?;
                        let edges = message.uint()?;
                        tracing::debug!(
                            "wl_shell_surface#{}.resize({}, {}, {})",
                            sender_id,
                            seat,
                            serial,
                            edges
                        );
                        self.resize(client, sender_id, seat, serial, edges.try_into()?)
                            .await
                    }
                    3u16 => {
                        tracing::debug!("wl_shell_surface#{}.set_toplevel()", sender_id,);
                        self.set_toplevel(client, sender_id).await
                    }
                    4u16 => {
                        let parent = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let x = message.int()?;
                        let y = message.int()?;
                        let flags = message.uint()?;
                        tracing::debug!(
                            "wl_shell_surface#{}.set_transient({}, {}, {}, {})",
                            sender_id,
                            parent,
                            x,
                            y,
                            flags
                        );
                        self.set_transient(client, sender_id, parent, x, y, flags.try_into()?)
                            .await
                    }
                    5u16 => {
                        let method = message.uint()?;
                        let framerate = message.uint()?;
                        let output = message.object()?;
                        tracing::debug!(
                            "wl_shell_surface#{}.set_fullscreen({}, {}, {})",
                            sender_id,
                            method,
                            framerate,
                            output
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string())
                        );
                        self.set_fullscreen(
                            client,
                            sender_id,
                            method.try_into()?,
                            framerate,
                            output,
                        )
                        .await
                    }
                    6u16 => {
                        let seat = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let serial = message.uint()?;
                        let parent = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let x = message.int()?;
                        let y = message.int()?;
                        let flags = message.uint()?;
                        tracing::debug!(
                            "wl_shell_surface#{}.set_popup({}, {}, {}, {}, {}, {})",
                            sender_id,
                            seat,
                            serial,
                            parent,
                            x,
                            y,
                            flags
                        );
                        self.set_popup(
                            client,
                            sender_id,
                            seat,
                            serial,
                            parent,
                            x,
                            y,
                            flags.try_into()?,
                        )
                        .await
                    }
                    7u16 => {
                        let output = message.object()?;
                        tracing::debug!(
                            "wl_shell_surface#{}.set_maximized({})",
                            sender_id,
                            output
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string())
                        );
                        self.set_maximized(client, sender_id, output).await
                    }
                    8u16 => {
                        let title = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_shell_surface#{}.set_title(\"{}\")", sender_id, title);
                        self.set_title(client, sender_id, title).await
                    }
                    9u16 => {
                        let class_ = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_shell_surface#{}.set_class(\"{}\")", sender_id, class_);
                        self.set_class(client, sender_id, class_).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "A client must respond to a ping event with a pong request or"]
            #[doc = "the client may be deemed unresponsive."]
            async fn pong(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Start a pointer-driven move of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to a button press event."]
            #[doc = "The server may ignore move requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized)."]
            async fn r#move(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Start a pointer-driven resizing of the surface."]
            #[doc = ""]
            #[doc = "This request must be used in response to a button press event."]
            #[doc = "The server may ignore resize requests depending on the state of"]
            #[doc = "the surface (e.g. fullscreen or maximized)."]
            async fn resize(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                edges: Resize,
            ) -> crate::server::Result<()>;
            #[doc = "Map the surface as a toplevel surface."]
            #[doc = ""]
            #[doc = "A toplevel surface is not fullscreen, maximized or transient."]
            async fn set_toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Map the surface relative to an existing surface."]
            #[doc = ""]
            #[doc = "The x and y arguments specify the location of the upper left"]
            #[doc = "corner of the surface relative to the upper left corner of the"]
            #[doc = "parent surface, in surface-local coordinates."]
            #[doc = ""]
            #[doc = "The flags argument controls details of the transient behaviour."]
            async fn set_transient(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                parent: crate::wire::ObjectId,
                x: i32,
                y: i32,
                flags: Transient,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                method: FullscreenMethod,
                framerate: u32,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
                parent: crate::wire::ObjectId,
                x: i32,
                y: i32,
                flags: Transient,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
            #[doc = "Set a short title for the surface."]
            #[doc = ""]
            #[doc = "This string may be used to identify the surface in a task bar,"]
            #[doc = "window list, or other user interface elements provided by the"]
            #[doc = "compositor."]
            #[doc = ""]
            #[doc = "The string must be encoded in UTF-8."]
            async fn set_title(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                title: String,
            ) -> crate::server::Result<()>;
            #[doc = "Set a class for the surface."]
            #[doc = ""]
            #[doc = "The surface class identifies the general class of applications"]
            #[doc = "to which the surface belongs. A common convention is to use the"]
            #[doc = "file name (or the full path if it is a non-standard location) of"]
            #[doc = "the application's .desktop file as the class."]
            async fn set_class(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                class: String,
            ) -> crate::server::Result<()>;
            #[doc = "Ping a client to check if it is receiving events and sending"]
            #[doc = "requests. A client is expected to reply with a pong request."]
            async fn ping(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.ping({})", sender_id, serial);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The configure event asks the client to resize its surface."]
            #[doc = ""]
            #[doc = "The size is a hint, in the sense that the client is free to"]
            #[doc = "ignore it if it doesn't resize, pick a smaller size (to"]
            #[doc = "satisfy aspect ratio or resize in steps of NxM pixels)."]
            #[doc = ""]
            #[doc = "The edges parameter provides a hint about how the surface"]
            #[doc = "was resized. The client may use this information to decide"]
            #[doc = "how to adjust its content to the new size (e.g. a scrolling"]
            #[doc = "area might adjust its content position to leave the viewable"]
            #[doc = "content unmoved)."]
            #[doc = ""]
            #[doc = "The client is free to dismiss all but the last configure"]
            #[doc = "event it received."]
            #[doc = ""]
            #[doc = "The width and height arguments specify the size of the window"]
            #[doc = "in surface-local coordinates."]
            async fn configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                edges: Resize,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_shell_surface#{}.configure({}, {}, {})",
                    sender_id,
                    edges,
                    width,
                    height
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(edges.bits())
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The popup_done event is sent out when a popup grab is broken,"]
            #[doc = "that is, when the user clicks a surface that doesn't belong"]
            #[doc = "to the client owning the popup surface."]
            async fn popup_done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_shell_surface#{}.popup_done()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_surface interface. See the module level documentation for more info"]
        pub trait WlSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_surface";
            const VERSION: u32 = 6u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_surface#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    1u16 => {
                        let buffer = message.object()?;
                        let x = message.int()?;
                        let y = message.int()?;
                        tracing::debug!(
                            "wl_surface#{}.attach({}, {}, {})",
                            sender_id,
                            buffer
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string()),
                            x,
                            y
                        );
                        self.attach(client, sender_id, buffer, x, y).await
                    }
                    2u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "wl_surface#{}.damage({}, {}, {}, {})",
                            sender_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.damage(client, sender_id, x, y, width, height).await
                    }
                    3u16 => {
                        let callback = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_surface#{}.frame({})", sender_id, callback);
                        self.frame(client, sender_id, callback).await
                    }
                    4u16 => {
                        let region = message.object()?;
                        tracing::debug!(
                            "wl_surface#{}.set_opaque_region({})",
                            sender_id,
                            region
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string())
                        );
                        self.set_opaque_region(client, sender_id, region).await
                    }
                    5u16 => {
                        let region = message.object()?;
                        tracing::debug!(
                            "wl_surface#{}.set_input_region({})",
                            sender_id,
                            region
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string())
                        );
                        self.set_input_region(client, sender_id, region).await
                    }
                    6u16 => {
                        tracing::debug!("wl_surface#{}.commit()", sender_id,);
                        self.commit(client, sender_id).await
                    }
                    7u16 => {
                        let transform = message.uint()?;
                        tracing::debug!(
                            "wl_surface#{}.set_buffer_transform({})",
                            sender_id,
                            transform
                        );
                        self.set_buffer_transform(client, sender_id, transform.try_into()?)
                            .await
                    }
                    8u16 => {
                        let scale = message.int()?;
                        tracing::debug!("wl_surface#{}.set_buffer_scale({})", sender_id, scale);
                        self.set_buffer_scale(client, sender_id, scale).await
                    }
                    9u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "wl_surface#{}.damage_buffer({}, {}, {}, {})",
                            sender_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.damage_buffer(client, sender_id, x, y, width, height)
                            .await
                    }
                    10u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        tracing::debug!("wl_surface#{}.offset({}, {})", sender_id, x, y);
                        self.offset(client, sender_id, x, y).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Deletes the surface and invalidates its object ID."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: Option<crate::wire::ObjectId>,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                callback: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                scale: i32,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
            #[doc = "This is emitted whenever a surface's creation, movement, or resizing"]
            #[doc = "results in some part of it being within the scanout region of an"]
            #[doc = "output."]
            #[doc = ""]
            #[doc = "Note that a surface may be overlapping with zero or more outputs."]
            async fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_surface#{}.enter({})", sender_id, output);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This is emitted whenever a surface's creation, movement, or resizing"]
            #[doc = "results in it no longer having any part of it within the scanout region"]
            #[doc = "of an output."]
            #[doc = ""]
            #[doc = "Clients should not use the number of outputs the surface is on for frame"]
            #[doc = "throttling purposes. The surface might be hidden even if no leave event"]
            #[doc = "has been sent, and the compositor might expect new surface content"]
            #[doc = "updates even if no enter event has been sent. The frame event should be"]
            #[doc = "used instead."]
            async fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_surface#{}.leave({})", sender_id, output);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates the preferred buffer scale for this surface. It is"]
            #[doc = "sent whenever the compositor's preference changes."]
            #[doc = ""]
            #[doc = "Before receiving this event the preferred buffer scale for this surface"]
            #[doc = "is 1."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients use this event to scale their"]
            #[doc = "content and use wl_surface.set_buffer_scale to indicate the scale they"]
            #[doc = "have rendered with. This allows clients to supply a higher detail"]
            #[doc = "buffer."]
            #[doc = ""]
            #[doc = "The compositor shall emit a scale value greater than 0."]
            async fn preferred_buffer_scale(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                factor: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_surface#{}.preferred_buffer_scale({})",
                    sender_id,
                    factor
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(factor).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event indicates the preferred buffer transform for this surface."]
            #[doc = "It is sent whenever the compositor's preference changes."]
            #[doc = ""]
            #[doc = "Before receiving this event the preferred buffer transform for this"]
            #[doc = "surface is normal."]
            #[doc = ""]
            #[doc = "Applying this transformation to the surface buffer contents and using"]
            #[doc = "wl_surface.set_buffer_transform might allow the compositor to use the"]
            #[doc = "surface buffer more efficiently."]
            async fn preferred_buffer_transform(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_surface#{}.preferred_buffer_transform({})",
                    sender_id,
                    transform
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(transform as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A seat is a group of keyboards, pointer and touch devices. This"]
    #[doc = "object is published as a global during start up, or when such a"]
    #[doc = "device is hot plugged.  A seat typically has a pointer and"]
    #[doc = "maintains a keyboard focus and a pointer focus."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_seat {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [doc = "This is a bitmask of capabilities this seat has; if a member is"] # [doc = "set, then it is present on the seat."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "the seat has pointer devices"] const Pointer = 1u32 ; # [doc = "the seat has one or more keyboards"] const Keyboard = 2u32 ; # [doc = "the seat has touch devices"] const Touch = 4u32 ; } }
        impl TryFrom<u32> for Capability {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_seat interface. See the module level documentation for more info"]
        pub trait WlSeat: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_seat";
            const VERSION: u32 = 9u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_seat#{}.get_pointer({})", sender_id, id);
                        self.get_pointer(client, sender_id, id).await
                    }
                    1u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_seat#{}.get_keyboard({})", sender_id, id);
                        self.get_keyboard(client, sender_id, id).await
                    }
                    2u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_seat#{}.get_touch({})", sender_id, id);
                        self.get_touch(client, sender_id, id).await
                    }
                    3u16 => {
                        tracing::debug!("wl_seat#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the seat object anymore."]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This is emitted whenever a seat gains or loses the pointer,"]
            #[doc = "keyboard or touch capabilities.  The argument is a capability"]
            #[doc = "enum containing the complete set of capabilities this seat has."]
            #[doc = ""]
            #[doc = "When the pointer capability is added, a client may create a"]
            #[doc = "wl_pointer object using the wl_seat.get_pointer request. This object"]
            #[doc = "will receive pointer events until the capability is removed in the"]
            #[doc = "future."]
            #[doc = ""]
            #[doc = "When the pointer capability is removed, a client should destroy the"]
            #[doc = "wl_pointer objects associated with the seat where the capability was"]
            #[doc = "removed, using the wl_pointer.release request. No further pointer"]
            #[doc = "events will be received on these objects."]
            #[doc = ""]
            #[doc = "In some compositors, if a seat regains the pointer capability and a"]
            #[doc = "client has a previously obtained wl_pointer object of version 4 or"]
            #[doc = "less, that object may start sending pointer events again. This"]
            #[doc = "behavior is considered a misinterpretation of the intended behavior"]
            #[doc = "and must not be relied upon by the client. wl_pointer objects of"]
            #[doc = "version 5 or later must not send events if created before the most"]
            #[doc = "recent event notifying the client of an added pointer capability."]
            #[doc = ""]
            #[doc = "The above behavior also applies to wl_keyboard and wl_touch with the"]
            #[doc = "keyboard and touch capabilities, respectively."]
            async fn capabilities(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                capabilities: Capability,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_seat#{}.capabilities({})", sender_id, capabilities);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(capabilities.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "In a multi-seat configuration the seat name can be used by clients to"]
            #[doc = "help identify which physical devices the seat represents."]
            #[doc = ""]
            #[doc = "The seat name is a UTF-8 string with no convention defined for its"]
            #[doc = "contents. Each name is unique among all wl_seat globals. The name is"]
            #[doc = "only guaranteed to be unique for the current compositor instance."]
            #[doc = ""]
            #[doc = "The same seat names are used for all clients. Thus, the name can be"]
            #[doc = "shared across processes to refer to a specific wl_seat global."]
            #[doc = ""]
            #[doc = "The name event is sent after binding to the seat global. This event is"]
            #[doc = "only sent once per seat object, and the name does not change over the"]
            #[doc = "lifetime of the wl_seat global."]
            #[doc = ""]
            #[doc = "Compositors may re-use the same seat name if the wl_seat global is"]
            #[doc = "destroyed and re-created later."]
            async fn name(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_seat#{}.name(\"{}\")", sender_id, name);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for ButtonState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for Axis {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for AxisSource {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for AxisRelativeDirection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_pointer interface. See the module level documentation for more info"]
        pub trait WlPointer: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_pointer";
            const VERSION: u32 = 9u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        let serial = message.uint()?;
                        let surface = message.object()?;
                        let hotspot_x = message.int()?;
                        let hotspot_y = message.int()?;
                        tracing::debug!(
                            "wl_pointer#{}.set_cursor({}, {}, {}, {})",
                            sender_id,
                            serial,
                            surface
                                .as_ref()
                                .map_or("null".to_string(), |v| v.to_string()),
                            hotspot_x,
                            hotspot_y
                        );
                        self.set_cursor(client, sender_id, serial, surface, hotspot_x, hotspot_y)
                            .await
                    }
                    1u16 => {
                        tracing::debug!("wl_pointer#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                surface: Option<crate::wire::ObjectId>,
                hotspot_x: i32,
                hotspot_y: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the pointer object anymore."]
            #[doc = ""]
            #[doc = "This request destroys the pointer proxy object, so clients must not call"]
            #[doc = "wl_pointer_destroy() after using this request."]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Notification that this seat's pointer is focused on a certain"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "When a seat's focus enters a surface, the pointer image"]
            #[doc = "is undefined and a client should respond to this event by setting"]
            #[doc = "an appropriate pointer image with the set_cursor request."]
            async fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                surface: crate::wire::ObjectId,
                surface_x: crate::wire::Fixed,
                surface_y: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.enter({}, {}, {}, {})",
                    sender_id,
                    serial,
                    surface,
                    surface_x,
                    surface_y
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .put_fixed(surface_x)
                    .put_fixed(surface_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notification that this seat's pointer is no longer focused on"]
            #[doc = "a certain surface."]
            #[doc = ""]
            #[doc = "The leave notification is sent before the enter notification"]
            #[doc = "for the new focus."]
            async fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_pointer#{}.leave({}, {})", sender_id, serial, surface);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notification of pointer location change. The arguments"]
            #[doc = "surface_x and surface_y are the location relative to the"]
            #[doc = "focused surface."]
            async fn motion(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                surface_x: crate::wire::Fixed,
                surface_y: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.motion({}, {}, {})",
                    sender_id,
                    time,
                    surface_x,
                    surface_y
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_fixed(surface_x)
                    .put_fixed(surface_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Mouse button click and release notifications."]
            #[doc = ""]
            #[doc = "The location of the click is given by the last motion or"]
            #[doc = "enter event."]
            #[doc = "The time argument is a timestamp with millisecond"]
            #[doc = "granularity, with an undefined base."]
            #[doc = ""]
            #[doc = "The button is a button code as defined in the Linux kernel's"]
            #[doc = "linux/input-event-codes.h header file, e.g. BTN_LEFT."]
            #[doc = ""]
            #[doc = "Any 16-bit button code value is reserved for future additions to the"]
            #[doc = "kernel's event code list. All other button codes above 0xFFFF are"]
            #[doc = "currently undefined but may be used in future versions of this"]
            #[doc = "protocol."]
            async fn button(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                time: u32,
                button: u32,
                state: ButtonState,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.button({}, {}, {}, {})",
                    sender_id,
                    serial,
                    time,
                    button,
                    state
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(button)
                    .put_uint(state as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Scroll and other axis notifications."]
            #[doc = ""]
            #[doc = "For scroll events (vertical and horizontal scroll axes), the"]
            #[doc = "value parameter is the length of a vector along the specified"]
            #[doc = "axis in a coordinate space identical to those of motion events,"]
            #[doc = "representing a relative movement along the specified axis."]
            #[doc = ""]
            #[doc = "For devices that support movements non-parallel to axes multiple"]
            #[doc = "axis events will be emitted."]
            #[doc = ""]
            #[doc = "When applicable, for example for touch pads, the server can"]
            #[doc = "choose to emit scroll events where the motion vector is"]
            #[doc = "equivalent to a motion event vector."]
            #[doc = ""]
            #[doc = "When applicable, a client can transform its content relative to the"]
            #[doc = "scroll distance."]
            async fn axis(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                axis: Axis,
                value: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.axis({}, {}, {})",
                    sender_id,
                    time,
                    axis,
                    value
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis as u32)
                    .put_fixed(value)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Indicates the end of a set of events that logically belong together."]
            #[doc = "A client is expected to accumulate the data in all events within the"]
            #[doc = "frame before proceeding."]
            #[doc = ""]
            #[doc = "All wl_pointer events before a wl_pointer.frame event belong"]
            #[doc = "logically together. For example, in a diagonal scroll motion the"]
            #[doc = "compositor will send an optional wl_pointer.axis_source event, two"]
            #[doc = "wl_pointer.axis events (horizontal and vertical) and finally a"]
            #[doc = "wl_pointer.frame event. The client may use this information to"]
            #[doc = "calculate a diagonal vector for scrolling."]
            #[doc = ""]
            #[doc = "When multiple wl_pointer.axis events occur within the same frame,"]
            #[doc = "the motion vector is the combined motion of all events."]
            #[doc = "When a wl_pointer.axis and a wl_pointer.axis_stop event occur within"]
            #[doc = "the same frame, this indicates that axis movement in one axis has"]
            #[doc = "stopped but continues in the other axis."]
            #[doc = "When multiple wl_pointer.axis_stop events occur within the same"]
            #[doc = "frame, this indicates that these axes stopped in the same instance."]
            #[doc = ""]
            #[doc = "A wl_pointer.frame event is sent for every logical event group,"]
            #[doc = "even if the group only contains a single wl_pointer event."]
            #[doc = "Specifically, a client may get a sequence: motion, frame, button,"]
            #[doc = "frame, axis, frame, axis_stop, frame."]
            #[doc = ""]
            #[doc = "The wl_pointer.enter and wl_pointer.leave events are logical events"]
            #[doc = "generated by the compositor and not the hardware. These events are"]
            #[doc = "also grouped by a wl_pointer.frame. When a pointer moves from one"]
            #[doc = "surface to another, a compositor should group the"]
            #[doc = "wl_pointer.leave event within the same wl_pointer.frame."]
            #[doc = "However, a client must not rely on wl_pointer.leave and"]
            #[doc = "wl_pointer.enter being in the same wl_pointer.frame."]
            #[doc = "Compositor-specific policies may require the wl_pointer.leave and"]
            #[doc = "wl_pointer.enter event being split across multiple wl_pointer.frame"]
            #[doc = "groups."]
            async fn frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_pointer#{}.frame()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Source information for scroll and other axes."]
            #[doc = ""]
            #[doc = "This event does not occur on its own. It is sent before a"]
            #[doc = "wl_pointer.frame event and carries the source information for"]
            #[doc = "all events within that frame."]
            #[doc = ""]
            #[doc = "The source specifies how this event was generated. If the source is"]
            #[doc = "wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be"]
            #[doc = "sent when the user lifts the finger off the device."]
            #[doc = ""]
            #[doc = "If the source is wl_pointer.axis_source.wheel,"]
            #[doc = "wl_pointer.axis_source.wheel_tilt or"]
            #[doc = "wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may"]
            #[doc = "or may not be sent. Whether a compositor sends an axis_stop event"]
            #[doc = "for these sources is hardware-specific and implementation-dependent;"]
            #[doc = "clients must not rely on receiving an axis_stop event for these"]
            #[doc = "scroll sources and should treat scroll sequences from these scroll"]
            #[doc = "sources as unterminated by default."]
            #[doc = ""]
            #[doc = "This event is optional. If the source is unknown for a particular"]
            #[doc = "axis event sequence, no event is sent."]
            #[doc = "Only one wl_pointer.axis_source event is permitted per frame."]
            #[doc = ""]
            #[doc = "The order of wl_pointer.axis_discrete and wl_pointer.axis_source is"]
            #[doc = "not guaranteed."]
            async fn axis_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                axis_source: AxisSource,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_pointer#{}.axis_source({})", sender_id, axis_source);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(axis_source as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Stop notification for scroll and other axes."]
            #[doc = ""]
            #[doc = "For some wl_pointer.axis_source types, a wl_pointer.axis_stop event"]
            #[doc = "is sent to notify a client that the axis sequence has terminated."]
            #[doc = "This enables the client to implement kinetic scrolling."]
            #[doc = "See the wl_pointer.axis_source documentation for information on when"]
            #[doc = "this event may be generated."]
            #[doc = ""]
            #[doc = "Any wl_pointer.axis events with the same axis_source after this"]
            #[doc = "event should be considered as the start of a new axis motion."]
            #[doc = ""]
            #[doc = "The timestamp is to be interpreted identical to the timestamp in the"]
            #[doc = "wl_pointer.axis event. The timestamp value may be the same as a"]
            #[doc = "preceding wl_pointer.axis event."]
            async fn axis_stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                axis: Axis,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_pointer#{}.axis_stop({}, {})", sender_id, time, axis);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Discrete step information for scroll and other axes."]
            #[doc = ""]
            #[doc = "This event carries the axis value of the wl_pointer.axis event in"]
            #[doc = "discrete steps (e.g. mouse wheel clicks)."]
            #[doc = ""]
            #[doc = "This event is deprecated with wl_pointer version 8 - this event is not"]
            #[doc = "sent to clients supporting version 8 or later."]
            #[doc = ""]
            #[doc = "This event does not occur on its own, it is coupled with a"]
            #[doc = "wl_pointer.axis event that represents this axis value on a"]
            #[doc = "continuous scale. The protocol guarantees that each axis_discrete"]
            #[doc = "event is always followed by exactly one axis event with the same"]
            #[doc = "axis number within the same wl_pointer.frame. Note that the protocol"]
            #[doc = "allows for other events to occur between the axis_discrete and"]
            #[doc = "its coupled axis event, including other axis_discrete or axis"]
            #[doc = "events. A wl_pointer.frame must not contain more than one axis_discrete"]
            #[doc = "event per axis type."]
            #[doc = ""]
            #[doc = "This event is optional; continuous scrolling devices"]
            #[doc = "like two-finger scrolling on touchpads do not have discrete"]
            #[doc = "steps and do not generate this event."]
            #[doc = ""]
            #[doc = "The discrete value carries the directional information. e.g. a value"]
            #[doc = "of -2 is two steps towards the negative direction of this axis."]
            #[doc = ""]
            #[doc = "The axis number is identical to the axis number in the associated"]
            #[doc = "axis event."]
            #[doc = ""]
            #[doc = "The order of wl_pointer.axis_discrete and wl_pointer.axis_source is"]
            #[doc = "not guaranteed."]
            async fn axis_discrete(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                axis: Axis,
                discrete: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.axis_discrete({}, {})",
                    sender_id,
                    axis,
                    discrete
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(axis as u32)
                    .put_int(discrete)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Discrete high-resolution scroll information."]
            #[doc = ""]
            #[doc = "This event carries high-resolution wheel scroll information,"]
            #[doc = "with each multiple of 120 representing one logical scroll step"]
            #[doc = "(a wheel detent). For example, an axis_value120 of 30 is one quarter of"]
            #[doc = "a logical scroll step in the positive direction, a value120 of"]
            #[doc = "-240 are two logical scroll steps in the negative direction within the"]
            #[doc = "same hardware event."]
            #[doc = "Clients that rely on discrete scrolling should accumulate the"]
            #[doc = "value120 to multiples of 120 before processing the event."]
            #[doc = ""]
            #[doc = "The value120 must not be zero."]
            #[doc = ""]
            #[doc = "This event replaces the wl_pointer.axis_discrete event in clients"]
            #[doc = "supporting wl_pointer version 8 or later."]
            #[doc = ""]
            #[doc = "Where a wl_pointer.axis_source event occurs in the same"]
            #[doc = "wl_pointer.frame, the axis source applies to this event."]
            #[doc = ""]
            #[doc = "The order of wl_pointer.axis_value120 and wl_pointer.axis_source is"]
            #[doc = "not guaranteed."]
            async fn axis_value120(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                axis: Axis,
                value120: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.axis_value120({}, {})",
                    sender_id,
                    axis,
                    value120
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(axis as u32)
                    .put_int(value120)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Relative directional information of the entity causing the axis"]
            #[doc = "motion."]
            #[doc = ""]
            #[doc = "For a wl_pointer.axis event, the wl_pointer.axis_relative_direction"]
            #[doc = "event specifies the movement direction of the entity causing the"]
            #[doc = "wl_pointer.axis event. For example:"]
            #[doc = "- if a user's fingers on a touchpad move down and this"]
            #[doc = "causes a wl_pointer.axis vertical_scroll down event, the physical"]
            #[doc = "direction is 'identical'"]
            #[doc = "- if a user's fingers on a touchpad move down and this causes a"]
            #[doc = "wl_pointer.axis vertical_scroll up scroll up event ('natural"]
            #[doc = "scrolling'), the physical direction is 'inverted'."]
            #[doc = ""]
            #[doc = "A client may use this information to adjust scroll motion of"]
            #[doc = "components. Specifically, enabling natural scrolling causes the"]
            #[doc = "content to change direction compared to traditional scrolling."]
            #[doc = "Some widgets like volume control sliders should usually match the"]
            #[doc = "physical direction regardless of whether natural scrolling is"]
            #[doc = "active. This event enables clients to match the scroll direction of"]
            #[doc = "a widget to the physical direction."]
            #[doc = ""]
            #[doc = "This event does not occur on its own, it is coupled with a"]
            #[doc = "wl_pointer.axis event that represents this axis value."]
            #[doc = "The protocol guarantees that each axis_relative_direction event is"]
            #[doc = "always followed by exactly one axis event with the same"]
            #[doc = "axis number within the same wl_pointer.frame. Note that the protocol"]
            #[doc = "allows for other events to occur between the axis_relative_direction"]
            #[doc = "and its coupled axis event."]
            #[doc = ""]
            #[doc = "The axis number is identical to the axis number in the associated"]
            #[doc = "axis event."]
            #[doc = ""]
            #[doc = "The order of wl_pointer.axis_relative_direction,"]
            #[doc = "wl_pointer.axis_discrete and wl_pointer.axis_source is not"]
            #[doc = "guaranteed."]
            async fn axis_relative_direction(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                axis: Axis,
                direction: AxisRelativeDirection,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_pointer#{}.axis_relative_direction({}, {})",
                    sender_id,
                    axis,
                    direction
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(axis as u32)
                    .put_uint(direction as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for KeymapFormat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for KeyState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_keyboard interface. See the module level documentation for more info"]
        pub trait WlKeyboard: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_keyboard";
            const VERSION: u32 = 9u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_keyboard#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event provides a file descriptor to the client which can be"]
            #[doc = "memory-mapped in read-only mode to provide a keyboard mapping"]
            #[doc = "description."]
            #[doc = ""]
            #[doc = "From version 7 onwards, the fd must be mapped with MAP_PRIVATE by"]
            #[doc = "the recipient, as MAP_SHARED may fail."]
            async fn keymap(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: KeymapFormat,
                fd: rustix::fd::OwnedFd,
                size: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_keyboard#{}.keymap({}, {}, {})",
                    sender_id,
                    format,
                    fd.as_raw_fd(),
                    size
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(format as u32)
                    .put_fd(fd)
                    .put_uint(size)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notification that this seat's keyboard focus is on a certain"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "The compositor must send the wl_keyboard.modifiers event after this"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "In the wl_keyboard logical state, this event sets the active surface to"]
            #[doc = "the surface argument and the keys currently logically down to the keys"]
            #[doc = "in the keys argument. The compositor must not send this event if the"]
            #[doc = "wl_keyboard already had an active surface immediately before this event."]
            async fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                surface: crate::wire::ObjectId,
                keys: Vec<u8>,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_keyboard#{}.enter({}, {}, array[{}])",
                    sender_id,
                    serial,
                    surface,
                    keys.len()
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .put_array(keys)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notification that this seat's keyboard focus is no longer on"]
            #[doc = "a certain surface."]
            #[doc = ""]
            #[doc = "The leave notification is sent before the enter notification"]
            #[doc = "for the new focus."]
            #[doc = ""]
            #[doc = "In the wl_keyboard logical state, this event resets all values to their"]
            #[doc = "defaults. The compositor must not send this event if the active surface"]
            #[doc = "of the wl_keyboard was not equal to the surface argument immediately"]
            #[doc = "before this event."]
            async fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_keyboard#{}.leave({}, {})",
                    sender_id,
                    serial,
                    surface
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A key was pressed or released."]
            #[doc = "The time argument is a timestamp with millisecond"]
            #[doc = "granularity, with an undefined base."]
            #[doc = ""]
            #[doc = "The key is a platform-specific key code that can be interpreted"]
            #[doc = "by feeding it to the keyboard mapping (see the keymap event)."]
            #[doc = ""]
            #[doc = "If this event produces a change in modifiers, then the resulting"]
            #[doc = "wl_keyboard.modifiers event must be sent after this event."]
            #[doc = ""]
            #[doc = "In the wl_keyboard logical state, this event adds the key to the keys"]
            #[doc = "currently logically down (if the state argument is pressed) or removes"]
            #[doc = "the key from the keys currently logically down (if the state argument is"]
            #[doc = "released). The compositor must not send this event if the wl_keyboard"]
            #[doc = "did not have an active surface immediately before this event. The"]
            #[doc = "compositor must not send this event if state is pressed (resp. released)"]
            #[doc = "and the key was already logically down (resp. was not logically down)"]
            #[doc = "immediately before this event."]
            async fn key(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                time: u32,
                key: u32,
                state: KeyState,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_keyboard#{}.key({}, {}, {}, {})",
                    sender_id,
                    serial,
                    time,
                    key,
                    state
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(key)
                    .put_uint(state as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Notifies clients that the modifier and/or group state has"]
            #[doc = "changed, and it should update its local state."]
            #[doc = ""]
            #[doc = "The compositor may send this event without a surface of the client"]
            #[doc = "having keyboard focus, for example to tie modifier information to"]
            #[doc = "pointer focus instead. If a modifier event with pressed modifiers is sent"]
            #[doc = "without a prior enter event, the client can assume the modifier state is"]
            #[doc = "valid until it receives the next wl_keyboard.modifiers event. In order to"]
            #[doc = "reset the modifier state again, the compositor can send a"]
            #[doc = "wl_keyboard.modifiers event with no pressed modifiers."]
            #[doc = ""]
            #[doc = "In the wl_keyboard logical state, this event updates the modifiers and"]
            #[doc = "group."]
            async fn modifiers(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                mods_depressed: u32,
                mods_latched: u32,
                mods_locked: u32,
                group: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_keyboard#{}.modifiers({}, {}, {}, {}, {})",
                    sender_id,
                    serial,
                    mods_depressed,
                    mods_latched,
                    mods_locked,
                    group
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(mods_depressed)
                    .put_uint(mods_latched)
                    .put_uint(mods_locked)
                    .put_uint(group)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Informs the client about the keyboard's repeat rate and delay."]
            #[doc = ""]
            #[doc = "This event is sent as soon as the wl_keyboard object has been created,"]
            #[doc = "and is guaranteed to be received by the client before any key press"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "Negative values for either rate or delay are illegal. A rate of zero"]
            #[doc = "will disable any repeating (regardless of the value of delay)."]
            #[doc = ""]
            #[doc = "This event can be sent later on as well with a new value if necessary,"]
            #[doc = "so clients should continue listening for the event past the creation"]
            #[doc = "of wl_keyboard."]
            async fn repeat_info(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                rate: i32,
                delay: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_keyboard#{}.repeat_info({}, {})",
                    sender_id,
                    rate,
                    delay
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(rate)
                    .put_int(delay)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_touch interface. See the module level documentation for more info"]
        pub trait WlTouch: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_touch";
            const VERSION: u32 = 9u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_touch#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "A new touch point has appeared on the surface. This touch point is"]
            #[doc = "assigned a unique ID. Future events from this touch point reference"]
            #[doc = "this ID. The ID ceases to be valid after a touch up event and may be"]
            #[doc = "reused in the future."]
            async fn down(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                time: u32,
                surface: crate::wire::ObjectId,
                id: i32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_touch#{}.down({}, {}, {}, {}, {}, {})",
                    sender_id,
                    serial,
                    time,
                    surface,
                    id,
                    x,
                    y
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_object(Some(surface))
                    .put_int(id)
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The touch point has disappeared. No further events will be sent for"]
            #[doc = "this touch point and the touch point's ID is released and may be"]
            #[doc = "reused in a future touch down event."]
            async fn up(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                time: u32,
                id: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_touch#{}.up({}, {}, {})", sender_id, serial, time, id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_int(id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A touch point has changed coordinates."]
            async fn motion(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                id: i32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_touch#{}.motion({}, {}, {}, {})",
                    sender_id,
                    time,
                    id,
                    x,
                    y
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_int(id)
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Indicates the end of a set of events that logically belong together."]
            #[doc = "A client is expected to accumulate the data in all events within the"]
            #[doc = "frame before proceeding."]
            #[doc = ""]
            #[doc = "A wl_touch.frame terminates at least one event but otherwise no"]
            #[doc = "guarantee is provided about the set of events within a frame. A client"]
            #[doc = "must assume that any state not updated in a frame is unchanged from the"]
            #[doc = "previously known state."]
            async fn frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_touch#{}.frame()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent if the compositor decides the touch stream is a global"]
            #[doc = "gesture. No further events are sent to the clients from that"]
            #[doc = "particular gesture. Touch cancellation applies to all touch points"]
            #[doc = "currently active on this client's surface. The client is"]
            #[doc = "responsible for finalizing the touch points, future touch points on"]
            #[doc = "this surface may reuse the touch point ID."]
            #[doc = ""]
            #[doc = "No frame event is required after the cancel event."]
            async fn cancel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_touch#{}.cancel()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent when a touchpoint has changed its shape."]
            #[doc = ""]
            #[doc = "This event does not occur on its own. It is sent before a"]
            #[doc = "wl_touch.frame event and carries the new shape information for"]
            #[doc = "any previously reported, or new touch points of that frame."]
            #[doc = ""]
            #[doc = "Other events describing the touch point such as wl_touch.down,"]
            #[doc = "wl_touch.motion or wl_touch.orientation may be sent within the"]
            #[doc = "same wl_touch.frame. A client should treat these events as a single"]
            #[doc = "logical touch point update. The order of wl_touch.shape,"]
            #[doc = "wl_touch.orientation and wl_touch.motion is not guaranteed."]
            #[doc = "A wl_touch.down event is guaranteed to occur before the first"]
            #[doc = "wl_touch.shape event for this touch ID but both events may occur within"]
            #[doc = "the same wl_touch.frame."]
            #[doc = ""]
            #[doc = "A touchpoint shape is approximated by an ellipse through the major and"]
            #[doc = "minor axis length. The major axis length describes the longer diameter"]
            #[doc = "of the ellipse, while the minor axis length describes the shorter"]
            #[doc = "diameter. Major and minor are orthogonal and both are specified in"]
            #[doc = "surface-local coordinates. The center of the ellipse is always at the"]
            #[doc = "touchpoint location as reported by wl_touch.down or wl_touch.move."]
            #[doc = ""]
            #[doc = "This event is only sent by the compositor if the touch device supports"]
            #[doc = "shape reports. The client has to make reasonable assumptions about the"]
            #[doc = "shape if it did not receive this event."]
            async fn shape(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: i32,
                major: crate::wire::Fixed,
                minor: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_touch#{}.shape({}, {}, {})",
                    sender_id,
                    id,
                    major,
                    minor
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(id)
                    .put_fixed(major)
                    .put_fixed(minor)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent when a touchpoint has changed its orientation."]
            #[doc = ""]
            #[doc = "This event does not occur on its own. It is sent before a"]
            #[doc = "wl_touch.frame event and carries the new shape information for"]
            #[doc = "any previously reported, or new touch points of that frame."]
            #[doc = ""]
            #[doc = "Other events describing the touch point such as wl_touch.down,"]
            #[doc = "wl_touch.motion or wl_touch.shape may be sent within the"]
            #[doc = "same wl_touch.frame. A client should treat these events as a single"]
            #[doc = "logical touch point update. The order of wl_touch.shape,"]
            #[doc = "wl_touch.orientation and wl_touch.motion is not guaranteed."]
            #[doc = "A wl_touch.down event is guaranteed to occur before the first"]
            #[doc = "wl_touch.orientation event for this touch ID but both events may occur"]
            #[doc = "within the same wl_touch.frame."]
            #[doc = ""]
            #[doc = "The orientation describes the clockwise angle of a touchpoint's major"]
            #[doc = "axis to the positive surface y-axis and is normalized to the -180 to"]
            #[doc = "+180 degree range. The granularity of orientation depends on the touch"]
            #[doc = "device, some devices only support binary rotation values between 0 and"]
            #[doc = "90 degrees."]
            #[doc = ""]
            #[doc = "This event is only sent by the compositor if the touch device supports"]
            #[doc = "orientation reports."]
            async fn orientation(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: i32,
                orientation: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_touch#{}.orientation({}, {})",
                    sender_id,
                    id,
                    orientation
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(id)
                    .put_fixed(orientation)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Subpixel {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for Transform {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "These flags describe properties of an output mode."] # [doc = "They are used in the flags bitfield of the mode event."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Mode : u32 { # [doc = "indicates this is the current mode"] const Current = 1u32 ; # [doc = "indicates this is the preferred mode"] const Preferred = 2u32 ; } }
        impl TryFrom<u32> for Mode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_output interface. See the module level documentation for more info"]
        pub trait WlOutput: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_output";
            const VERSION: u32 = 4u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_output#{}.release()", sender_id,);
                        self.release(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Using this request a client can tell the server that it is not going to"]
            #[doc = "use the output object anymore."]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "The geometry event describes geometric properties of the output."]
            #[doc = "The event is sent when binding to the output object and whenever"]
            #[doc = "any of the properties change."]
            #[doc = ""]
            #[doc = "The physical size can be set to zero if it doesn't make sense for this"]
            #[doc = "output (e.g. for projectors or virtual outputs)."]
            #[doc = ""]
            #[doc = "The geometry event will be followed by a done event (starting from"]
            #[doc = "version 2)."]
            #[doc = ""]
            #[doc = "Clients should use wl_surface.preferred_buffer_transform instead of the"]
            #[doc = "transform advertised by this event to find the preferred buffer"]
            #[doc = "transform to use for a surface."]
            #[doc = ""]
            #[doc = "Note: wl_output only advertises partial information about the output"]
            #[doc = "position and identification. Some compositors, for instance those not"]
            #[doc = "implementing a desktop-style output layout or those exposing virtual"]
            #[doc = "outputs, might fake this information. Instead of using x and y, clients"]
            #[doc = "should use xdg_output.logical_position. Instead of using make and model,"]
            #[doc = "clients should use name and description."]
            async fn geometry(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                physical_width: i32,
                physical_height: i32,
                subpixel: Subpixel,
                make: String,
                model: String,
                transform: Transform,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_output#{}.geometry({}, {}, {}, {}, {}, \"{}\", \"{}\", {})",
                    sender_id,
                    x,
                    y,
                    physical_width,
                    physical_height,
                    subpixel,
                    make,
                    model,
                    transform
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(physical_width)
                    .put_int(physical_height)
                    .put_uint(subpixel as u32)
                    .put_string(Some(make))
                    .put_string(Some(model))
                    .put_uint(transform as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The mode event describes an available mode for the output."]
            #[doc = ""]
            #[doc = "The event is sent when binding to the output object and there"]
            #[doc = "will always be one mode, the current mode.  The event is sent"]
            #[doc = "again if an output changes mode, for the mode that is now"]
            #[doc = "current.  In other words, the current mode is always the last"]
            #[doc = "mode that was received with the current flag set."]
            #[doc = ""]
            #[doc = "Non-current modes are deprecated. A compositor can decide to only"]
            #[doc = "advertise the current mode and never send other modes. Clients"]
            #[doc = "should not rely on non-current modes."]
            #[doc = ""]
            #[doc = "The size of a mode is given in physical hardware units of"]
            #[doc = "the output device. This is not necessarily the same as"]
            #[doc = "the output size in the global compositor space. For instance,"]
            #[doc = "the output may be scaled, as described in wl_output.scale,"]
            #[doc = "or transformed, as described in wl_output.transform. Clients"]
            #[doc = "willing to retrieve the output size in the global compositor"]
            #[doc = "space should use xdg_output.logical_size instead."]
            #[doc = ""]
            #[doc = "The vertical refresh rate can be set to zero if it doesn't make"]
            #[doc = "sense for this output (e.g. for virtual outputs)."]
            #[doc = ""]
            #[doc = "The mode event will be followed by a done event (starting from"]
            #[doc = "version 2)."]
            #[doc = ""]
            #[doc = "Clients should not use the refresh rate to schedule frames. Instead,"]
            #[doc = "they should use the wl_surface.frame event or the presentation-time"]
            #[doc = "protocol."]
            #[doc = ""]
            #[doc = "Note: this information is not always meaningful for all outputs. Some"]
            #[doc = "compositors, such as those exposing virtual outputs, might fake the"]
            #[doc = "refresh rate or the size."]
            async fn mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                flags: Mode,
                width: i32,
                height: i32,
                refresh: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_output#{}.mode({}, {}, {}, {})",
                    sender_id,
                    flags,
                    width,
                    height,
                    refresh
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(flags.bits())
                    .put_int(width)
                    .put_int(height)
                    .put_int(refresh)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is sent after all other properties have been"]
            #[doc = "sent after binding to the output object and after any"]
            #[doc = "other property changes done after that. This allows"]
            #[doc = "changes to the output properties to be seen as"]
            #[doc = "atomic, even if they happen via multiple events."]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_output#{}.done()", sender_id,);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. The compositor will emit a non-zero, positive"]
            #[doc = "value for scale. If it is not sent, the client should"]
            #[doc = "assume a scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for very high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "Clients should use wl_surface.preferred_buffer_scale"]
            #[doc = "instead of this event to find the preferred buffer"]
            #[doc = "scale to use for a surface."]
            #[doc = ""]
            #[doc = "The scale event will be followed by a done event."]
            async fn scale(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                factor: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_output#{}.scale({})", sender_id, factor);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_int(factor).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Many compositors will assign user-friendly names to their outputs, show"]
            #[doc = "them to the user, allow the user to refer to an output, etc. The client"]
            #[doc = "may wish to know this name as well to offer the user similar behaviors."]
            #[doc = ""]
            #[doc = "The name is a UTF-8 string with no convention defined for its contents."]
            #[doc = "Each name is unique among all wl_output globals. The name is only"]
            #[doc = "guaranteed to be unique for the compositor instance."]
            #[doc = ""]
            #[doc = "The same output name is used for all clients for a given wl_output"]
            #[doc = "global. Thus, the name can be shared across processes to refer to a"]
            #[doc = "specific wl_output global."]
            #[doc = ""]
            #[doc = "The name is not guaranteed to be persistent across sessions, thus cannot"]
            #[doc = "be used to reliably identify an output in e.g. configuration files."]
            #[doc = ""]
            #[doc = "Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do"]
            #[doc = "not assume that the name is a reflection of an underlying DRM connector,"]
            #[doc = "X11 connection, etc."]
            #[doc = ""]
            #[doc = "The name event is sent after binding the output object. This event is"]
            #[doc = "only sent once per output object, and the name does not change over the"]
            #[doc = "lifetime of the wl_output global."]
            #[doc = ""]
            #[doc = "Compositors may re-use the same output name if the wl_output global is"]
            #[doc = "destroyed and re-created later. Compositors should avoid re-using the"]
            #[doc = "same name if possible."]
            #[doc = ""]
            #[doc = "The name event will be followed by a done event."]
            async fn name(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> wl_output#{}.name(\"{}\")", sender_id, name);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Many compositors can produce human-readable descriptions of their"]
            #[doc = "outputs. The client may wish to know this description as well, e.g. for"]
            #[doc = "output selection purposes."]
            #[doc = ""]
            #[doc = "The description is a UTF-8 string with no convention defined for its"]
            #[doc = "contents. The description is not guaranteed to be unique among all"]
            #[doc = "wl_output globals. Examples might include 'Foocorp 11\" Display' or"]
            #[doc = "'Virtual X11 output via :1'."]
            #[doc = ""]
            #[doc = "The description event is sent after binding the output object and"]
            #[doc = "whenever the description changes. The description is optional, and may"]
            #[doc = "not be sent at all."]
            #[doc = ""]
            #[doc = "The description event will be followed by a done event."]
            async fn description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                description: String,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> wl_output#{}.description(\"{}\")",
                    sender_id,
                    description
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A region object describes an area."]
    #[doc = ""]
    #[doc = "Region objects are used to describe the opaque and input"]
    #[doc = "regions of a surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_region {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wl_region interface. See the module level documentation for more info"]
        pub trait WlRegion: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_region";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_region#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    1u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "wl_region#{}.add({}, {}, {}, {})",
                            sender_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.add(client, sender_id, x, y, width, height).await
                    }
                    2u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "wl_region#{}.subtract({}, {}, {}, {})",
                            sender_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.subtract(client, sender_id, x, y, width, height).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the region.  This will invalidate the object ID."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Add the specified rectangle to the region."]
            async fn add(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
            #[doc = "Subtract the specified rectangle from the region."]
            async fn subtract(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()>;
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_subcompositor interface. See the module level documentation for more info"]
        pub trait WlSubcompositor: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_subcompositor";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_subcompositor#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    1u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let surface = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let parent = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wl_subcompositor#{}.get_subsurface({}, {}, {})",
                            sender_id,
                            id,
                            surface,
                            parent
                        );
                        self.get_subsurface(client, sender_id, id, surface, parent)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other"]
            #[doc = "objects, wl_subsurface objects included."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                parent: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_subsurface interface. See the module level documentation for more info"]
        pub trait WlSubsurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "wl_subsurface";
            const VERSION: u32 = 1u32;
            async fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode {
                    0u16 => {
                        tracing::debug!("wl_subsurface#{}.destroy()", sender_id,);
                        self.destroy(client, sender_id).await
                    }
                    1u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        tracing::debug!("wl_subsurface#{}.set_position({}, {})", sender_id, x, y);
                        self.set_position(client, sender_id, x, y).await
                    }
                    2u16 => {
                        let sibling = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_subsurface#{}.place_above({})", sender_id, sibling);
                        self.place_above(client, sender_id, sibling).await
                    }
                    3u16 => {
                        let sibling = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wl_subsurface#{}.place_below({})", sender_id, sibling);
                        self.place_below(client, sender_id, sibling).await
                    }
                    4u16 => {
                        tracing::debug!("wl_subsurface#{}.set_sync()", sender_id,);
                        self.set_sync(client, sender_id).await
                    }
                    5u16 => {
                        tracing::debug!("wl_subsurface#{}.set_desync()", sender_id,);
                        self.set_desync(client, sender_id).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "The sub-surface interface is removed from the wl_surface object"]
            #[doc = "that was turned into a sub-surface with a"]
            #[doc = "wl_subcompositor.get_subsurface request. The wl_surface's association"]
            #[doc = "to the parent is deleted. The wl_surface is unmapped immediately."]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                sibling: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "The sub-surface is placed just below the reference surface."]
            #[doc = "See wl_subsurface.place_above."]
            async fn place_below(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                sibling: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
}
