#[doc = "The aim of this color management extension is to get HDR games working quickly,"]
#[doc = "and have an easy way to test implementations in the wild before the upstream"]
#[doc = "protocol is ready to be merged."]
#[doc = "For that purpose it's intentionally limited and cut down and does not serve"]
#[doc = "all uses cases."]
#[allow(clippy::module_inception)]
pub mod frog_color_management_v1 {
    #[doc = "The color management factory singleton creates color managed surface objects."]
    #[allow(clippy::too_many_arguments)]
    #[allow(unused)]
    pub mod frog_color_management_factory_v1 {
        #[doc = "Trait to implement the frog_color_management_factory_v1 interface. See the module level documentation for more info"]
        pub trait FrogColorManagementFactoryV1<C: waynest::Connection>
        where
            Self: std::marker::Sync,
        {
            const INTERFACE: &'static str = "frog_color_management_factory_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            fn get_color_managed_surface(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                callback: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_color_management_factory_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let callback = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_color_management_factory_v1#{}.get_color_managed_surface({}, {})",
                                sender_id,
                                surface,
                                callback
                            );
                            self.get_color_managed_surface(connection, sender_id, surface, callback)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Interface for changing surface color management and HDR state."]
    #[doc = ""]
    #[doc = "An implementation must: support every part of the version"]
    #[doc = "of the frog_color_managed_surface interface it exposes."]
    #[doc = "Including all known enums associated with a given version."]
    #[allow(clippy::too_many_arguments)]
    #[allow(unused)]
    pub mod frog_color_managed_surface {
        #[doc = "Extended information on the transfer functions described"]
        #[doc = "here can be found in the Khronos Data Format specification:"]
        #[doc = ""]
        #[doc = "https://registry.khronos.org/DataFormat/specs/1.3/dataformat.1.3.html"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TransferFunction {
            #[doc = "specifies undefined, implementation-specific handling of the surface's transfer function."]
            Undefined = 0u32,
            #[doc = "specifies the sRGB non-linear EOTF. An implementation may: display this as Gamma 2.2 for the purposes of being consistent with content rendering across displays, rendering_intent and user expectations."]
            Srgb = 1u32,
            #[doc = "specifies gamma 2.2 power curve as the EOTF"]
            Gamma22 = 2u32,
            #[doc = "specifies the SMPTE ST2084 Perceptual Quantizer (PQ) EOTF"]
            St2084Pq = 3u32,
            #[doc = "specifies the scRGB (extended sRGB) linear EOTF. Note: Primaries outside the gamut triangle specified can be expressed with negative values for this transfer function."]
            ScrgbLinear = 4u32,
        }
        impl TryFrom<u32> for TransferFunction {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Undefined),
                    1u32 => Ok(Self::Srgb),
                    2u32 => Ok(Self::Gamma22),
                    3u32 => Ok(Self::St2084Pq),
                    4u32 => Ok(Self::ScrgbLinear),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TransferFunction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Primaries {
            #[doc = "specifies undefined, implementation-specific handling"]
            Undefined = 0u32,
            #[doc = "specifies Rec.709/sRGB primaries with D65 white point"]
            Rec709 = 1u32,
            #[doc = "specifies Rec.2020/HDR10 primaries with D65 white point"]
            Rec2020 = 2u32,
        }
        impl TryFrom<u32> for Primaries {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Undefined),
                    1u32 => Ok(Self::Rec709),
                    2u32 => Ok(Self::Rec2020),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Primaries {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Extended information on render intents described"]
        #[doc = "here can be found in ICC.1:2022:"]
        #[doc = ""]
        #[doc = "https://www.color.org/specification/ICC.1-2022-05.pdf"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum RenderIntent {
            #[doc = "perceptual"]
            Perceptual = 0u32,
        }
        impl TryFrom<u32> for RenderIntent {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Perceptual),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for RenderIntent {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the frog_color_managed_surface interface. See the module level documentation for more info"]
        pub trait FrogColorManagedSurface<C: waynest::Connection>
        where
            Self: std::marker::Sync,
        {
            const INTERFACE: &'static str = "frog_color_managed_surface";
            const VERSION: u32 = 1u32;
            #[doc = "Destroying the color managed surface resets all known color"]
            #[doc = "state for the surface back to 'undefined' implementation-specific"]
            #[doc = "values."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            fn set_known_transfer_function(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                transfer_function: TransferFunction,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            fn set_known_container_color_volume(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                primaries: Primaries,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            #[doc = "NOTE: On a surface with \"perceptual\" (default) render intent, handling of the container's color volume"]
            #[doc = "is implementation-specific, and may differ between different transfer functions it is paired with:"]
            #[doc = "ie. sRGB + 709 rendering may have it's primaries widened to more of the available display's gamut"]
            #[doc = "to be be more pleasing for the viewer."]
            #[doc = "Compared to scRGB Linear + 709 being treated faithfully as 709"]
            #[doc = "(including utilizing negatives out of the 709 gamut triangle)"]
            fn set_render_intent(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                render_intent: RenderIntent,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            #[doc = "Forwards HDR metadata from the client to the compositor."]
            #[doc = ""]
            #[doc = "HDR Metadata Infoframe as per CTA 861.G spec."]
            #[doc = ""]
            #[doc = "Usage of this HDR metadata is implementation specific and"]
            #[doc = "outside of the scope of this protocol."]
            fn set_hdr_metadata(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                mastering_display_primary_red_x: u32,
                mastering_display_primary_red_y: u32,
                mastering_display_primary_green_x: u32,
                mastering_display_primary_green_y: u32,
                mastering_display_primary_blue_x: u32,
                mastering_display_primary_blue_y: u32,
                mastering_white_point_x: u32,
                mastering_white_point_y: u32,
                max_display_mastering_luminance: u32,
                min_display_mastering_luminance: u32,
                max_cll: u32,
                max_fall: u32,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            #[doc = "Current preferred metadata for a surface."]
            #[doc = "The application should use this information to tone-map its buffers"]
            #[doc = "to this target before committing."]
            #[doc = ""]
            #[doc = "This metadata does not necessarily correspond to any physical output, but"]
            #[doc = "rather what the compositor thinks would be best for a given surface."]
            fn preferred_metadata(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                transfer_function: TransferFunction,
                output_display_primary_red_x: u32,
                output_display_primary_red_y: u32,
                output_display_primary_green_x: u32,
                output_display_primary_green_y: u32,
                output_display_primary_blue_x: u32,
                output_display_primary_blue_y: u32,
                output_white_point_x: u32,
                output_white_point_y: u32,
                max_luminance: u32,
                min_luminance: u32,
                max_full_frame_luminance: u32,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> frog_color_managed_surface#{}.preferred_metadata({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        transfer_function,
                        output_display_primary_red_x,
                        output_display_primary_red_y,
                        output_display_primary_green_x,
                        output_display_primary_green_y,
                        output_display_primary_blue_x,
                        output_display_primary_blue_y,
                        output_white_point_x,
                        output_white_point_y,
                        max_luminance,
                        min_luminance,
                        max_full_frame_luminance
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(transfer_function as u32)
                        .put_uint(output_display_primary_red_x)
                        .put_uint(output_display_primary_red_y)
                        .put_uint(output_display_primary_green_x)
                        .put_uint(output_display_primary_green_y)
                        .put_uint(output_display_primary_blue_x)
                        .put_uint(output_display_primary_blue_y)
                        .put_uint(output_white_point_x)
                        .put_uint(output_white_point_y)
                        .put_uint(max_luminance)
                        .put_uint(min_luminance)
                        .put_uint(max_full_frame_luminance)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await?;
                    Ok(())
                }
            }
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("frog_color_managed_surface#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let transfer_function = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_color_managed_surface#{}.set_known_transfer_function({})",
                                sender_id,
                                transfer_function
                            );
                            self.set_known_transfer_function(
                                connection,
                                sender_id,
                                transfer_function.try_into()?,
                            )
                            .await
                        }
                        2u16 => {
                            let primaries = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_color_managed_surface#{}.set_known_container_color_volume({})",
                                sender_id,
                                primaries
                            );
                            self.set_known_container_color_volume(
                                connection,
                                sender_id,
                                primaries.try_into()?,
                            )
                            .await
                        }
                        3u16 => {
                            let render_intent = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_color_managed_surface#{}.set_render_intent({})",
                                sender_id,
                                render_intent
                            );
                            self.set_render_intent(connection, sender_id, render_intent.try_into()?)
                                .await
                        }
                        4u16 => {
                            let mastering_display_primary_red_x = message.uint()?;
                            let mastering_display_primary_red_y = message.uint()?;
                            let mastering_display_primary_green_x = message.uint()?;
                            let mastering_display_primary_green_y = message.uint()?;
                            let mastering_display_primary_blue_x = message.uint()?;
                            let mastering_display_primary_blue_y = message.uint()?;
                            let mastering_white_point_x = message.uint()?;
                            let mastering_white_point_y = message.uint()?;
                            let max_display_mastering_luminance = message.uint()?;
                            let min_display_mastering_luminance = message.uint()?;
                            let max_cll = message.uint()?;
                            let max_fall = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_color_managed_surface#{}.set_hdr_metadata({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                mastering_display_primary_red_x,
                                mastering_display_primary_red_y,
                                mastering_display_primary_green_x,
                                mastering_display_primary_green_y,
                                mastering_display_primary_blue_x,
                                mastering_display_primary_blue_y,
                                mastering_white_point_x,
                                mastering_white_point_y,
                                max_display_mastering_luminance,
                                min_display_mastering_luminance,
                                max_cll,
                                max_fall
                            );
                            self.set_hdr_metadata(
                                connection,
                                sender_id,
                                mastering_display_primary_red_x,
                                mastering_display_primary_red_y,
                                mastering_display_primary_green_x,
                                mastering_display_primary_green_y,
                                mastering_display_primary_blue_x,
                                mastering_display_primary_blue_y,
                                mastering_white_point_x,
                                mastering_white_point_y,
                                max_display_mastering_luminance,
                                min_display_mastering_luminance,
                                max_cll,
                                max_fall,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod frog_fifo_v1 {
    #[doc = "When a Wayland compositor considers applying a content update,"]
    #[doc = "it must ensure all the update's readiness constraints (fences, etc)"]
    #[doc = "are met."]
    #[doc = ""]
    #[doc = "This protocol provides a way to use the completion of a display refresh"]
    #[doc = "cycle as an additional readiness constraint."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    #[allow(unused)]
    pub mod frog_fifo_manager_v1 {
        #[doc = "These fatal protocol errors may be emitted in response to"]
        #[doc = "illegal requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "fifo extension already exists for surface"]
            FifoSurfaceAlreadyExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::FifoSurfaceAlreadyExists),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the frog_fifo_manager_v1 interface. See the module level documentation for more info"]
        pub trait FrogFifoManagerV1<C: waynest::Connection>
        where
            Self: std::marker::Sync,
        {
            const INTERFACE: &'static str = "frog_fifo_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object. Existing objects created by this object"]
            #[doc = "are not affected."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            #[doc = "Establish a fifo object for a surface that may be used to add"]
            #[doc = "display refresh constraints to content updates."]
            #[doc = ""]
            #[doc = "Only one such object may exist for a surface and attempting"]
            #[doc = "to create more than one will result in a fifo_manager_already_exists"]
            #[doc = "protocol error. If a surface is acted on by multiple software"]
            #[doc = "components, general best practice is that only the component"]
            #[doc = "performing wl_surface.attach operations should use this protocol."]
            fn get_fifo(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("frog_fifo_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "frog_fifo_manager_v1#{}.get_fifo({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_fifo(connection, sender_id, id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A fifo object for a surface that may be used to add"]
    #[doc = "display refresh constraints to content updates."]
    #[allow(clippy::too_many_arguments)]
    #[allow(unused)]
    pub mod frog_fifo_surface_v1 {
        #[doc = "These fatal protocol errors may be emitted in response to"]
        #[doc = "illegal requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the associated surface no longer exists"]
            SurfaceDestroyed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SurfaceDestroyed),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the frog_fifo_surface_v1 interface. See the module level documentation for more info"]
        pub trait FrogFifoSurfaceV1<C: waynest::Connection>
        where
            Self: std::marker::Sync,
        {
            const INTERFACE: &'static str = "frog_fifo_surface_v1";
            const VERSION: u32 = 1u32;
            #[doc = "When the content update containing the \"set_barrier\" is applied,"]
            #[doc = "it sets a \"fifo_barrier\" condition on the surface associated with"]
            #[doc = "the fifo object. The condition is cleared immediately after the"]
            #[doc = "following latching deadline for non-tearing presentation."]
            #[doc = "The condition needs to be cleared soon enough so that forward"]
            #[doc = "progress for the application is guaranteed. The exact rate at which"]
            #[doc = "that happens is implementation defined."]
            #[doc = ""]
            #[doc = "To wait for this condition to clear, use the \"wait_barrier\" request."]
            #[doc = ""]
            #[doc = "\"set_barrier\" is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "Requesting set_barrier after the fifo object's surface is"]
            #[doc = "destroyed will generate a \"surface_destroyed\" error."]
            fn set_barrier(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            #[doc = "Indicate that this content update is not ready while a"]
            #[doc = "\"fifo_barrier\" condition is present on the surface."]
            #[doc = ""]
            #[doc = "This means that when the content update containing \"set_barrier\""]
            #[doc = "was made active at a latching deadline, it will be active for"]
            #[doc = "at least one refresh cycle. A content update which is allowed to"]
            #[doc = "tear might become active after a latching deadline if no content"]
            #[doc = "update became active at the deadline."]
            #[doc = ""]
            #[doc = "\"wait_barrier\" is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "Requesting \"wait_barrier\" after the fifo object's surface is"]
            #[doc = "destroyed will generate a \"surface_destroyed\" error."]
            fn wait_barrier(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object."]
            #[doc = ""]
            #[doc = "Surface state changes previously made by this protocol are"]
            #[doc = "unaffected by this object's destruction."]
            fn destroy(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut C,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("frog_fifo_surface_v1#{}.set_barrier()", sender_id,);
                            self.set_barrier(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("frog_fifo_surface_v1#{}.wait_barrier()", sender_id,);
                            self.wait_barrier(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("frog_fifo_surface_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
