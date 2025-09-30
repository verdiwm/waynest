#[allow(clippy::module_inception)]
pub mod drm {
    #[allow(clippy::too_many_arguments)]
    pub mod wl_drm {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            AuthenticateFail = 0u32,
            InvalidFormat = 1u32,
            InvalidName = 2u32,
        }
        impl From<Error> for u32 {
            fn from(value: Error) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AuthenticateFail),
                    1u32 => Ok(Self::InvalidFormat),
                    2u32 => Ok(Self::InvalidName),
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
        pub enum Format {
            C8 = 538982467u32,
            Rgb332 = 943867730u32,
            Bgr233 = 944916290u32,
            Xrgb4444 = 842093144u32,
            Xbgr4444 = 842089048u32,
            Rgbx4444 = 842094674u32,
            Bgrx4444 = 842094658u32,
            Argb4444 = 842093121u32,
            Abgr4444 = 842089025u32,
            Rgba4444 = 842088786u32,
            Bgra4444 = 842088770u32,
            Xrgb1555 = 892424792u32,
            Xbgr1555 = 892420696u32,
            Rgbx5551 = 892426322u32,
            Bgrx5551 = 892426306u32,
            Argb1555 = 892424769u32,
            Abgr1555 = 892420673u32,
            Rgba5551 = 892420434u32,
            Bgra5551 = 892420418u32,
            Rgb565 = 909199186u32,
            Bgr565 = 909199170u32,
            Rgb888 = 875710290u32,
            Bgr888 = 875710274u32,
            Xrgb8888 = 875713112u32,
            Xbgr8888 = 875709016u32,
            Rgbx8888 = 875714642u32,
            Bgrx8888 = 875714626u32,
            Argb8888 = 875713089u32,
            Abgr8888 = 875708993u32,
            Rgba8888 = 875708754u32,
            Bgra8888 = 875708738u32,
            Xrgb2101010 = 808669784u32,
            Xbgr2101010 = 808665688u32,
            Rgbx1010102 = 808671314u32,
            Bgrx1010102 = 808671298u32,
            Argb2101010 = 808669761u32,
            Abgr2101010 = 808665665u32,
            Rgba1010102 = 808665426u32,
            Bgra1010102 = 808665410u32,
            Yuyv = 1448695129u32,
            Yvyu = 1431918169u32,
            Uyvy = 1498831189u32,
            Vyuy = 1498765654u32,
            Ayuv = 1448433985u32,
            Xyuv8888 = 1448434008u32,
            Nv12 = 842094158u32,
            Nv21 = 825382478u32,
            Nv16 = 909203022u32,
            Nv61 = 825644622u32,
            Yuv410 = 961959257u32,
            Yvu410 = 961893977u32,
            Yuv411 = 825316697u32,
            Yvu411 = 825316953u32,
            Yuv420 = 842093913u32,
            Yvu420 = 842094169u32,
            Yuv422 = 909202777u32,
            Yvu422 = 909203033u32,
            Yuv444 = 875713881u32,
            Yvu444 = 875714137u32,
            Abgr16f = 1211384385u32,
            Xbgr16f = 1211384408u32,
        }
        impl From<Format> for u32 {
            fn from(value: Format) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Format {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
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
                    875713112u32 => Ok(Self::Xrgb8888),
                    875709016u32 => Ok(Self::Xbgr8888),
                    875714642u32 => Ok(Self::Rgbx8888),
                    875714626u32 => Ok(Self::Bgrx8888),
                    875713089u32 => Ok(Self::Argb8888),
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
                    1448434008u32 => Ok(Self::Xyuv8888),
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
                    1211384385u32 => Ok(Self::Abgr16f),
                    1211384408u32 => Ok(Self::Xbgr16f),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Format {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Bitmask of capabilities."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Capability {
            #[doc = "wl_drm prime available"]
            Prime = 1u32,
        }
        impl From<Capability> for u32 {
            fn from(value: Capability) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Capability {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Prime),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_drm interface. See the module level documentation for more info"]
        pub trait WlDrm
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "wl_drm";
            const VERSION: u32 = 2u32;
            fn authenticate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> wl_drm#{}.authenticate({})", sender_id, id);
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(id).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn create_buffer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                name: u32,
                width: i32,
                height: i32,
                stride: u32,
                format: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> wl_drm#{}.create_buffer({}, {}, {}, {}, {}, {})",
                        sender_id,
                        id,
                        name,
                        width,
                        height,
                        stride,
                        format
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_uint(name)
                        .put_int(width)
                        .put_int(height)
                        .put_uint(stride)
                        .put_uint(format)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn create_planar_buffer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                name: u32,
                width: i32,
                height: i32,
                format: u32,
                offset0: i32,
                stride0: i32,
                offset1: i32,
                stride1: i32,
                offset2: i32,
                stride2: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> wl_drm#{}.create_planar_buffer({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        id,
                        name,
                        width,
                        height,
                        format,
                        offset0,
                        stride0,
                        offset1,
                        stride1,
                        offset2,
                        stride2
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_uint(name)
                        .put_int(width)
                        .put_int(height)
                        .put_uint(format)
                        .put_int(offset0)
                        .put_int(stride0)
                        .put_int(offset1)
                        .put_int(stride1)
                        .put_int(offset2)
                        .put_int(stride2)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn create_prime_buffer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                name: std::os::fd::BorrowedFd,
                width: i32,
                height: i32,
                format: u32,
                offset0: i32,
                stride0: i32,
                offset1: i32,
                stride1: i32,
                offset2: i32,
                stride2: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> wl_drm#{}.create_prime_buffer({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        id,
                        std::os::fd::AsRawFd::as_raw_fd(&name),
                        width,
                        height,
                        format,
                        offset0,
                        stride0,
                        offset1,
                        stride1,
                        offset2,
                        stride2
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_fd(name)
                        .put_int(width)
                        .put_int(height)
                        .put_uint(format)
                        .put_int(offset0)
                        .put_int(stride0)
                        .put_int(offset1)
                        .put_int(stride1)
                        .put_int(offset2)
                        .put_int(stride2)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn device(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn format(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                format: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn authenticated(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn capabilities(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                value: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
        }
    }
}
