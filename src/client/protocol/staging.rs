#![allow(async_fn_in_trait)]
#[allow(clippy::module_inception)]
pub mod alpha_modifier_v1 {
    #[doc = ""]
    #[doc = "This interface allows a client to set a factor for the alpha values on a"]
    #[doc = "surface, which can be used to offload such operations to the compositor,"]
    #[doc = "which can in turn for example offload them to KMS."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_alpha_modifier_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_alpha_modifier_v1 interface. See the module level documentation for more info"]
        pub trait WpAlphaModifierV1 {
            const INTERFACE: &'static str = "wp_alpha_modifier_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the alpha modifier manager. This doesn't destroy objects"]
            #[doc = "created with the manager."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_alpha_modifier_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a new alpha modifier surface object associated with the"]
            #[doc = "given wl_surface. If there is already such an object associated with"]
            #[doc = "the wl_surface, the already_constructed error will be raised."]
            #[doc = ""]
            async fn get_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_alpha_modifier_v1#{}.get_surface()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This interface allows the client to set a factor for the alpha values on"]
    #[doc = "a surface, which can be used to offload such operations to the compositor."]
    #[doc = "The default factor is UINT32_MAX."]
    #[doc = ""]
    #[doc = "This object has to be destroyed before the associated wl_surface. Once the"]
    #[doc = "wl_surface is destroyed, all request on this object will raise the"]
    #[doc = "no_surface error."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_alpha_modifier_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_alpha_modifier_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpAlphaModifierSurfaceV1 {
            const INTERFACE: &'static str = "wp_alpha_modifier_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This destroys the object, and is equivalent to set_multiplier with"]
            #[doc = "a value of UINT32_MAX, with the same double-buffered semantics as"]
            #[doc = "set_multiplier."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_alpha_modifier_surface_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the alpha multiplier for the surface. The alpha multiplier is"]
            #[doc = "double-buffered state, see wl_surface.commit for details."]
            #[doc = ""]
            #[doc = "This factor is applied in the compositor's blending space, as an"]
            #[doc = "additional step after the processing of per-pixel alpha values for the"]
            #[doc = "wl_surface. The exact meaning of the factor is thus undefined, unless"]
            #[doc = "the blending space is specified in a different extension."]
            #[doc = ""]
            #[doc = "This multiplier is applied even if the buffer attached to the"]
            #[doc = "wl_surface doesn't have an alpha channel; in that case an alpha value"]
            #[doc = "of one is used instead."]
            #[doc = ""]
            #[doc = "Zero means completely transparent, UINT32_MAX means completely opaque."]
            #[doc = ""]
            async fn set_multiplier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                factor: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_alpha_modifier_surface_v1#{}.set_multiplier()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(factor).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
#[doc = "The aim of the color management extension is to allow clients to know"]
#[doc = "the color properties of outputs, and to tell the compositor about the color"]
#[doc = "properties of their content on surfaces. Doing this enables a compositor"]
#[doc = "to perform automatic color management of content for different outputs"]
#[doc = "according to how content is intended to look like."]
#[doc = ""]
#[doc = "The color properties are represented as an image description object which"]
#[doc = "is immutable after it has been created. A wl_output always has an"]
#[doc = "associated image description that clients can observe. A wl_surface"]
#[doc = "always has an associated preferred image description as a hint chosen by"]
#[doc = "the compositor that clients can also observe. Clients can set an image"]
#[doc = "description on a wl_surface to denote the color characteristics of the"]
#[doc = "surface contents."]
#[doc = ""]
#[doc = "An image description includes SDR and HDR colorimetry and encoding, HDR"]
#[doc = "metadata, and viewing environment parameters. An image description does"]
#[doc = "not include the properties set through color-representation extension."]
#[doc = "It is expected that the color-representation extension is used in"]
#[doc = "conjunction with the color management extension when necessary,"]
#[doc = "particularly with the YUV family of pixel formats."]
#[doc = ""]
#[doc = "Recommendation ITU-T H.273"]
#[doc = "\"Coding-independent code points for video signal type identification\""]
#[doc = "shall be referred to as simply H.273 here."]
#[doc = ""]
#[doc = "The color-and-hdr repository"]
#[doc = "(https://gitlab.freedesktop.org/pq/color-and-hdr) contains"]
#[doc = "background information on the protocol design and legacy color management."]
#[doc = "It also contains a glossary, learning resources for digital color, tools,"]
#[doc = "samples and more."]
#[doc = ""]
#[doc = "The terminology used in this protocol is based on common color science and"]
#[doc = "color encoding terminology where possible. The glossary in the color-and-hdr"]
#[doc = "repository shall be the authority on the definition of terms in this"]
#[doc = "protocol."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod color_management_v1 {
    #[doc = ""]
    #[doc = "A singleton global interface used for getting color management extensions"]
    #[doc = "for wl_surface and wl_output objects, and for creating client defined"]
    #[doc = "image description objects. The extension interfaces allow"]
    #[doc = "getting the image description of outputs and setting the image"]
    #[doc = "description of surfaces."]
    #[doc = ""]
    #[doc = "Compositors should never remove this global."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_color_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "request not supported"]
            UnsupportedFeature = 0u32,
            #[doc = "color management surface exists already"]
            SurfaceExists = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::UnsupportedFeature),
                    1u32 => Ok(Self::SurfaceExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "See the ICC.1:2022 specification from the International Color Consortium"]
        #[doc = "for more details about rendering intents."]
        #[doc = ""]
        #[doc = "The principles of ICC defined rendering intents apply with all types of"]
        #[doc = "image descriptions, not only those with ICC file profiles."]
        #[doc = ""]
        #[doc = "Compositors must support the perceptual rendering intent. Other"]
        #[doc = "rendering intents are optional."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum RenderIntent {
            #[doc = "perceptual"]
            Perceptual = 0u32,
            #[doc = "media-relative colorimetric"]
            Relative = 1u32,
            #[doc = "saturation"]
            Saturation = 2u32,
            #[doc = "ICC-absolute colorimetric"]
            Absolute = 3u32,
            #[doc = "media-relative colorimetric + black point compensation"]
            RelativeBpc = 4u32,
        }
        impl TryFrom<u32> for RenderIntent {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Perceptual),
                    1u32 => Ok(Self::Relative),
                    2u32 => Ok(Self::Saturation),
                    3u32 => Ok(Self::Absolute),
                    4u32 => Ok(Self::RelativeBpc),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for RenderIntent {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Feature {
            #[doc = "create_icc_creator request"]
            IccV2V4 = 0u32,
            #[doc = "create_parametric_creator request"]
            Parametric = 1u32,
            #[doc = "parametric set_primaries request"]
            SetPrimaries = 2u32,
            #[doc = "parametric set_tf_power request"]
            SetTfPower = 3u32,
            #[doc = "parametric set_luminances request"]
            SetLuminances = 4u32,
            SetMasteringDisplayPrimaries = 5u32,
            ExtendedTargetVolume = 6u32,
            #[doc = "create_windows_scrgb request"]
            WindowsScrgb = 7u32,
        }
        impl TryFrom<u32> for Feature {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::IccV2V4),
                    1u32 => Ok(Self::Parametric),
                    2u32 => Ok(Self::SetPrimaries),
                    3u32 => Ok(Self::SetTfPower),
                    4u32 => Ok(Self::SetLuminances),
                    5u32 => Ok(Self::SetMasteringDisplayPrimaries),
                    6u32 => Ok(Self::ExtendedTargetVolume),
                    7u32 => Ok(Self::WindowsScrgb),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Feature {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "Named color primaries used to encode well-known sets of primaries. H.273"]
        #[doc = "is the authority, when it comes to the exact values of primaries and"]
        #[doc = "authoritative specifications, where an equivalent code point exists."]
        #[doc = ""]
        #[doc = "A value of 0 is invalid and will never be present in the list of enums."]
        #[doc = ""]
        #[doc = "Descriptions do list the specifications for convenience."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Primaries {
            Srgb = 1u32,
            PalM = 2u32,
            Pal = 3u32,
            Ntsc = 4u32,
            GenericFilm = 5u32,
            Bt2020 = 6u32,
            Cie1931Xyz = 7u32,
            DciP3 = 8u32,
            DisplayP3 = 9u32,
            AdobeRgb = 10u32,
        }
        impl TryFrom<u32> for Primaries {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Srgb),
                    2u32 => Ok(Self::PalM),
                    3u32 => Ok(Self::Pal),
                    4u32 => Ok(Self::Ntsc),
                    5u32 => Ok(Self::GenericFilm),
                    6u32 => Ok(Self::Bt2020),
                    7u32 => Ok(Self::Cie1931Xyz),
                    8u32 => Ok(Self::DciP3),
                    9u32 => Ok(Self::DisplayP3),
                    10u32 => Ok(Self::AdobeRgb),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Primaries {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "Named transfer functions used to represent well-known transfer"]
        #[doc = "characteristics. H.273 is the authority, when it comes to the exact"]
        #[doc = "formulas and authoritative specifications, where an equivalent code"]
        #[doc = "point exists."]
        #[doc = ""]
        #[doc = "A value of 0 is invalid and will never be present in the list of enums."]
        #[doc = ""]
        #[doc = "Descriptions do list the specifications for convenience."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TransferFunction {
            Bt1886 = 1u32,
            Gamma22 = 2u32,
            Gamma28 = 3u32,
            St240 = 4u32,
            ExtLinear = 5u32,
            Log100 = 6u32,
            Log316 = 7u32,
            Xvycc = 8u32,
            Srgb = 9u32,
            ExtSrgb = 10u32,
            St2084Pq = 11u32,
            St428 = 12u32,
            Hlg = 13u32,
        }
        impl TryFrom<u32> for TransferFunction {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Bt1886),
                    2u32 => Ok(Self::Gamma22),
                    3u32 => Ok(Self::Gamma28),
                    4u32 => Ok(Self::St240),
                    5u32 => Ok(Self::ExtLinear),
                    6u32 => Ok(Self::Log100),
                    7u32 => Ok(Self::Log316),
                    8u32 => Ok(Self::Xvycc),
                    9u32 => Ok(Self::Srgb),
                    10u32 => Ok(Self::ExtSrgb),
                    11u32 => Ok(Self::St2084Pq),
                    12u32 => Ok(Self::St428),
                    13u32 => Ok(Self::Hlg),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TransferFunction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_color_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpColorManagerV1 {
            const INTERFACE: &'static str = "wp_color_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let render_intent = message.uint()?;
                        tracing::debug!(
                            "wp_color_manager_v1#{}.supported_intent({})",
                            sender_id,
                            render_intent
                        );
                        self.supported_intent(client, sender_id, render_intent.try_into()?)
                            .await
                    }
                    1u16 => {
                        let feature = message.uint()?;
                        tracing::debug!(
                            "wp_color_manager_v1#{}.supported_feature({})",
                            sender_id,
                            feature
                        );
                        self.supported_feature(client, sender_id, feature.try_into()?)
                            .await
                    }
                    2u16 => {
                        let tf = message.uint()?;
                        tracing::debug!(
                            "wp_color_manager_v1#{}.supported_tf_named({})",
                            sender_id,
                            tf
                        );
                        self.supported_tf_named(client, sender_id, tf.try_into()?)
                            .await
                    }
                    3u16 => {
                        let primaries = message.uint()?;
                        tracing::debug!(
                            "wp_color_manager_v1#{}.supported_primaries_named({})",
                            sender_id,
                            primaries
                        );
                        self.supported_primaries_named(client, sender_id, primaries.try_into()?)
                            .await
                    }
                    4u16 => {
                        tracing::debug!("wp_color_manager_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the wp_color_manager_v1 object. This does not affect any other"]
            #[doc = "objects in any way."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_color_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This creates a new wp_color_management_output_v1 object for the"]
            #[doc = "given wl_output."]
            #[doc = ""]
            #[doc = "See the wp_color_management_output_v1 interface for more details."]
            #[doc = ""]
            async fn get_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_color_manager_v1#{}.get_output()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If a wp_color_management_surface_v1 object already exists for the given"]
            #[doc = "wl_surface, the protocol error surface_exists is raised."]
            #[doc = ""]
            #[doc = "This creates a new color wp_color_management_surface_v1 object for the"]
            #[doc = "given wl_surface."]
            #[doc = ""]
            #[doc = "See the wp_color_management_surface_v1 interface for more details."]
            #[doc = ""]
            async fn get_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_color_manager_v1#{}.get_surface()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This creates a new color wp_color_management_surface_feedback_v1 object"]
            #[doc = "for the given wl_surface."]
            #[doc = ""]
            #[doc = "See the wp_color_management_surface_feedback_v1 interface for more"]
            #[doc = "details."]
            #[doc = ""]
            async fn get_surface_feedback(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_manager_v1#{}.get_surface_feedback()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Makes a new ICC-based image description creator object with all"]
            #[doc = "properties initially unset. The client can then use the object's"]
            #[doc = "interface to define all the required properties for an image description"]
            #[doc = "and finally create a wp_image_description_v1 object."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.icc_v2_v4."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            #[doc = ""]
            async fn create_icc_creator(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                obj: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_color_manager_v1#{}.create_icc_creator()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(obj))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Makes a new parametric image description creator object with all"]
            #[doc = "properties initially unset. The client can then use the object's"]
            #[doc = "interface to define all the required properties for an image description"]
            #[doc = "and finally create a wp_image_description_v1 object."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.parametric."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            #[doc = ""]
            async fn create_parametric_creator(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                obj: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_manager_v1#{}.create_parametric_creator()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(obj))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This creates a pre-defined image description for the so-called"]
            #[doc = "Windows-scRGB stimulus encoding. This comes from the Windows 10 handling"]
            #[doc = "of its own definition of an scRGB color space for an HDR screen"]
            #[doc = "driven in BT.2100/PQ signalling mode."]
            #[doc = ""]
            #[doc = "Windows-scRGB uses sRGB (BT.709) color primaries and white point."]
            #[doc = "The transfer characteristic is extended linear."]
            #[doc = ""]
            #[doc = "The nominal color channel value range is extended, meaning it includes"]
            #[doc = "negative and greater than 1.0 values. Negative values are used to"]
            #[doc = "escape the sRGB color gamut boundaries. To make use of the extended"]
            #[doc = "range, the client needs to use a pixel format that can represent those"]
            #[doc = "values, e.g. floating-point 16 bits per channel."]
            #[doc = ""]
            #[doc = "Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system"]
            #[doc = "0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m²."]
            #[doc = "The maximum is R=G=B=125.0 corresponding to 10k cd/m²."]
            #[doc = ""]
            #[doc = "Windows-scRGB is displayed by Windows 10 by converting it to"]
            #[doc = "BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the"]
            #[doc = "luminance as above. No adjustment is made to the signal to account"]
            #[doc = "for the viewing conditions."]
            #[doc = ""]
            #[doc = "The reference white level of Windows-scRGB is unknown. If a"]
            #[doc = "reference white level must be assumed for compositor processing, it"]
            #[doc = "should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R"]
            #[doc = "BT.2408-7."]
            #[doc = ""]
            #[doc = "The target color volume of Windows-scRGB is unknown. The color gamut"]
            #[doc = "may be anything between sRGB and BT.2100."]
            #[doc = ""]
            #[doc = "Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from"]
            #[doc = "Windows-scRGB by using R=G=B=1.0 as the reference white level, while"]
            #[doc = "Windows-scRGB reference white level is unknown or varies. However,"]
            #[doc = "it seems probable that Windows implements both"]
            #[doc = "EGL_EXT_gl_colorspace_scrgb_linear and Vulkan"]
            #[doc = "VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.windows_scrgb."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            #[doc = ""]
            #[doc = "The resulting image description object does not allow get_information"]
            #[doc = "request. The wp_image_description_v1.ready event shall be sent."]
            #[doc = ""]
            async fn create_windows_scrgb(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_manager_v1#{}.create_windows_scrgb()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each rendering intent the compositor supports."]
            #[doc = ""]
            async fn supported_intent(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                render_intent: RenderIntent,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each compositor supported feature listed in the enumeration."]
            #[doc = ""]
            async fn supported_feature(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                feature: Feature,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each named transfer function the compositor supports with the"]
            #[doc = "parametric image description creator."]
            #[doc = ""]
            async fn supported_tf_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tf: TransferFunction,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each named set of primaries the compositor supports with the"]
            #[doc = "parametric image description creator."]
            #[doc = ""]
            async fn supported_primaries_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                primaries: Primaries,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent when all supported rendering intents, features,"]
            #[doc = "transfer functions and named primaries have been sent."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A wp_color_management_output_v1 describes the color properties of an"]
    #[doc = "output."]
    #[doc = ""]
    #[doc = "The wp_color_management_output_v1 is associated with the wl_output global"]
    #[doc = "underlying the wl_output object. Therefore the client destroying the"]
    #[doc = "wl_output object has no impact, but the compositor removing the output"]
    #[doc = "global makes the wp_color_management_output_v1 object inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_color_management_output_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_color_management_output_v1 interface. See the module level documentation for more info"]
        pub trait WpColorManagementOutputV1 {
            const INTERFACE: &'static str = "wp_color_management_output_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        tracing::debug!(
                            "wp_color_management_output_v1#{}.image_description_changed()",
                            sender_id,
                        );
                        self.image_description_changed(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the color wp_color_management_output_v1 object. This does not"]
            #[doc = "affect any remaining protocol objects."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_color_management_output_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This creates a new wp_image_description_v1 object for the current image"]
            #[doc = "description of the output. There always is exactly one image description"]
            #[doc = "active for an output so the client should destroy the image description"]
            #[doc = "created by earlier invocations of this request. This request is usually"]
            #[doc = "sent as a reaction to the image_description_changed event or when"]
            #[doc = "creating a wp_color_management_output_v1 object."]
            #[doc = ""]
            #[doc = "The image description of an output represents the color encoding the"]
            #[doc = "output expects. There might be performance and power advantages, as well"]
            #[doc = "as improved color reproduction, if a content update matches the image"]
            #[doc = "description of the output it is being shown on. If a content update is"]
            #[doc = "shown on any other output than the one it matches the image description"]
            #[doc = "of, then the color reproduction on those outputs might be considerably"]
            #[doc = "worse."]
            #[doc = ""]
            #[doc = "The created wp_image_description_v1 object preserves the image"]
            #[doc = "description of the output from the time the object was created."]
            #[doc = ""]
            #[doc = "The resulting image description object allows get_information request."]
            #[doc = ""]
            #[doc = "If this protocol object is inert, the resulting image description object"]
            #[doc = "shall immediately deliver the wp_image_description_v1.failed event with"]
            #[doc = "the no_output cause."]
            #[doc = ""]
            #[doc = "If the interface version is inadequate for the output's image"]
            #[doc = "description, meaning that the client does not support all the events"]
            #[doc = "needed to deliver the crucial information, the resulting image"]
            #[doc = "description object shall immediately deliver the"]
            #[doc = "wp_image_description_v1.failed event with the low_version cause."]
            #[doc = ""]
            #[doc = "Otherwise the object shall immediately deliver the ready event."]
            #[doc = ""]
            async fn get_image_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_management_output_v1#{}.get_image_description()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event is sent whenever the image description of the output changed,"]
            #[doc = "followed by one wl_output.done event common to output events across all"]
            #[doc = "extensions."]
            #[doc = ""]
            #[doc = "If the client wants to use the updated image description, it needs to do"]
            #[doc = "get_image_description again, because image description objects are"]
            #[doc = "immutable."]
            #[doc = ""]
            async fn image_description_changed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A wp_color_management_surface_v1 allows the client to set the color"]
    #[doc = "space and HDR properties of a surface."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the wp_color_management_surface_v1 is"]
    #[doc = "destroyed, the wp_color_management_surface_v1 object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_color_management_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "unsupported rendering intent"]
            RenderIntent = 0u32,
            #[doc = "invalid image description"]
            ImageDescription = 1u32,
            #[doc = "forbidden request on inert object"]
            Inert = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::RenderIntent),
                    1u32 => Ok(Self::ImageDescription),
                    2u32 => Ok(Self::Inert),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_color_management_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpColorManagementSurfaceV1 {
            const INTERFACE: &'static str = "wp_color_management_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the wp_color_management_surface_v1 object and do the same as"]
            #[doc = "unset_image_description."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_color_management_surface_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "Set the image description of the underlying surface. The image"]
            #[doc = "description and rendering intent are double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            #[doc = "It is the client's responsibility to understand the image description"]
            #[doc = "it sets on a surface, and to provide content that matches that image"]
            #[doc = "description. Compositors might convert images to match their own or any"]
            #[doc = "other image descriptions."]
            #[doc = ""]
            #[doc = "Image descriptions which are not ready (see wp_image_description_v1)"]
            #[doc = "are forbidden in this request, and in such case the protocol error"]
            #[doc = "image_description is raised."]
            #[doc = ""]
            #[doc = "All image descriptions which are ready (see wp_image_description_v1)"]
            #[doc = "are allowed and must always be accepted by the compositor."]
            #[doc = ""]
            #[doc = "A rendering intent provides the client's preference on how content"]
            #[doc = "colors should be mapped to each output. The render_intent value must"]
            #[doc = "be one advertised by the compositor with"]
            #[doc = "wp_color_manager_v1.render_intent event, otherwise the protocol error"]
            #[doc = "render_intent is raised."]
            #[doc = ""]
            #[doc = "When an image description is set on a surface, the Transfer"]
            #[doc = "Characteristics of the image description defines the valid range of"]
            #[doc = "the nominal (real-valued) color channel values. The processing of"]
            #[doc = "out-of-range color channel values is undefined, but compositors are"]
            #[doc = "recommended to clamp the values to the valid range when possible."]
            #[doc = ""]
            #[doc = "By default, a surface does not have an associated image description"]
            #[doc = "nor a rendering intent. The handling of color on such surfaces is"]
            #[doc = "compositor implementation defined. Compositors should handle such"]
            #[doc = "surfaces as sRGB, but may handle them differently if they have specific"]
            #[doc = "requirements."]
            #[doc = ""]
            #[doc = "Setting the image description has copy semantics; after this request,"]
            #[doc = "the image description can be immediately destroyed without affecting"]
            #[doc = "the pending state of the surface."]
            #[doc = ""]
            async fn set_image_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
                render_intent : super :: super :: super :: staging :: color_management_v1 :: wp_color_manager_v1 :: RenderIntent,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_management_surface_v1#{}.set_image_description()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .put_uint(render_intent as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "This request removes any image description from the surface. See"]
            #[doc = "set_image_description for how a compositor handles a surface without"]
            #[doc = "an image description. This is double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            async fn unset_image_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_management_surface_v1#{}.unset_image_description()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "A wp_color_management_surface_feedback_v1 allows the client to get the"]
    #[doc = "preferred image description of a surface."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with this object is destroyed, the"]
    #[doc = "wp_color_management_surface_feedback_v1 object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_color_management_surface_feedback_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "forbidden request on inert object"]
            Inert = 0u32,
            #[doc = "attempted to use an unsupported feature"]
            UnsupportedFeature = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Inert),
                    1u32 => Ok(Self::UnsupportedFeature),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_color_management_surface_feedback_v1 interface. See the module level documentation for more info"]
        pub trait WpColorManagementSurfaceFeedbackV1 {
            const INTERFACE: &'static str = "wp_color_management_surface_feedback_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let identity = message.uint()?;
                        tracing::debug!(
                            "wp_color_management_surface_feedback_v1#{}.preferred_changed({})",
                            sender_id,
                            identity
                        );
                        self.preferred_changed(client, sender_id, identity).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the wp_color_management_surface_feedback_v1 object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_management_surface_feedback_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "The preferred image description represents the compositor's preferred"]
            #[doc = "color encoding for this wl_surface at the current time. There might be"]
            #[doc = "performance and power advantages, as well as improved color"]
            #[doc = "reproduction, if the image description of a content update matches the"]
            #[doc = "preferred image description."]
            #[doc = ""]
            #[doc = "This creates a new wp_image_description_v1 object for the currently"]
            #[doc = "preferred image description for the wl_surface. The client should"]
            #[doc = "stop using and destroy the image descriptions created by earlier"]
            #[doc = "invocations of this request for the associated wl_surface."]
            #[doc = "This request is usually sent as a reaction to the preferred_changed"]
            #[doc = "event or when creating a wp_color_management_surface_feedback_v1 object"]
            #[doc = "if the client is capable of adapting to image descriptions."]
            #[doc = ""]
            #[doc = "The created wp_image_description_v1 object preserves the preferred image"]
            #[doc = "description of the wl_surface from the time the object was created."]
            #[doc = ""]
            #[doc = "The resulting image description object allows get_information request."]
            #[doc = ""]
            #[doc = "If the image description is parametric, the client should set it on its"]
            #[doc = "wl_surface only if the image description is an exact match with the"]
            #[doc = "client content. Particularly if everything else matches, but the target"]
            #[doc = "color volume is greater than what the client needs, the client should"]
            #[doc = "create its own parameric image description with its exact parameters."]
            #[doc = ""]
            #[doc = "If the interface version is inadequate for the preferred image"]
            #[doc = "description, meaning that the client does not support all the"]
            #[doc = "events needed to deliver the crucial information, the resulting image"]
            #[doc = "description object shall immediately deliver the"]
            #[doc = "wp_image_description_v1.failed event with the low_version cause,"]
            #[doc = "otherwise the object shall immediately deliver the ready event."]
            #[doc = ""]
            async fn get_preferred(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_management_surface_feedback_v1#{}.get_preferred()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The same description as for get_preferred applies, except the returned"]
            #[doc = "image description is guaranteed to be parametric. This is meant for"]
            #[doc = "clients that can only deal with parametric image descriptions."]
            #[doc = ""]
            #[doc = "If the compositor doesn't support parametric image descriptions, the"]
            #[doc = "unsupported_feature error is emitted."]
            #[doc = ""]
            async fn get_preferred_parametric(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_management_surface_feedback_v1#{}.get_preferred_parametric()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The preferred image description is the one which likely has the most"]
            #[doc = "performance and/or quality benefits for the compositor if used by the"]
            #[doc = "client for its wl_surface contents. This event is sent whenever the"]
            #[doc = "compositor changes the wl_surface's preferred image description."]
            #[doc = ""]
            #[doc = "This event sends the identity of the new preferred state as the argument,"]
            #[doc = "so clients who are aware of the image description already can reuse it."]
            #[doc = "Otherwise, if the client client wants to know what the preferred image"]
            #[doc = "description is, it shall use the get_preferred request."]
            #[doc = ""]
            #[doc = "The preferred image description is not automatically used for anything."]
            #[doc = "It is only a hint, and clients may set any valid image description with"]
            #[doc = "set_image_description, but there might be performance and color accuracy"]
            #[doc = "improvements by providing the wl_surface contents in the preferred"]
            #[doc = "image description. Therefore clients that can, should render according"]
            #[doc = "to the preferred image description"]
            #[doc = ""]
            async fn preferred_changed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                identity: u32,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "This type of object is used for collecting all the information required"]
    #[doc = "to create a wp_image_description_v1 object from an ICC file. A complete"]
    #[doc = "set of required parameters consists of these properties:"]
    #[doc = "- ICC file"]
    #[doc = ""]
    #[doc = "Each required property must be set exactly once if the client is to create"]
    #[doc = "an image description. The set requests verify that a property was not"]
    #[doc = "already set. The create request verifies that all required properties are"]
    #[doc = "set. There may be several alternative requests for setting each property,"]
    #[doc = "and in that case the client must choose one of them."]
    #[doc = ""]
    #[doc = "Once all properties have been set, the create request must be used to"]
    #[doc = "create the image description object, destroying the creator in the"]
    #[doc = "process."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_image_description_creator_icc_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "incomplete parameter set"]
            IncompleteSet = 0u32,
            #[doc = "property already set"]
            AlreadySet = 1u32,
            #[doc = "fd not seekable and readable"]
            BadFd = 2u32,
            #[doc = "no or too much data"]
            BadSize = 3u32,
            #[doc = "offset + length exceeds file size"]
            OutOfFile = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::IncompleteSet),
                    1u32 => Ok(Self::AlreadySet),
                    2u32 => Ok(Self::BadFd),
                    3u32 => Ok(Self::BadSize),
                    4u32 => Ok(Self::OutOfFile),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_image_description_creator_icc_v1 interface. See the module level documentation for more info"]
        pub trait WpImageDescriptionCreatorIccV1 {
            const INTERFACE: &'static str = "wp_image_description_creator_icc_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Create an image description object based on the ICC information"]
            #[doc = "previously set on this object. A compositor must parse the ICC data in"]
            #[doc = "some undefined but finite amount of time."]
            #[doc = ""]
            #[doc = "The completeness of the parameter set is verified. If the set is not"]
            #[doc = "complete, the protocol error incomplete_set is raised. For the"]
            #[doc = "definition of a complete set, see the description of this interface."]
            #[doc = ""]
            #[doc = "If the particular combination of the information is not supported"]
            #[doc = "by the compositor, the resulting image description object shall"]
            #[doc = "immediately deliver the wp_image_description_v1.failed event with the"]
            #[doc = "'unsupported' cause. If a valid image description was created from the"]
            #[doc = "information, the wp_image_description_v1.ready event will eventually"]
            #[doc = "be sent instead."]
            #[doc = ""]
            #[doc = "This request destroys the wp_image_description_creator_icc_v1 object."]
            #[doc = ""]
            #[doc = "The resulting image description object does not allow get_information"]
            #[doc = "request."]
            #[doc = ""]
            async fn create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_icc_v1#{}.create()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the ICC profile file to be used as the basis of the image"]
            #[doc = "description."]
            #[doc = ""]
            #[doc = "The data shall be found through the given fd at the given offset, having"]
            #[doc = "the given length. The fd must be seekable and readable. Violating these"]
            #[doc = "requirements raises the bad_fd protocol error."]
            #[doc = ""]
            #[doc = "If reading the data fails due to an error independent of the client, the"]
            #[doc = "compositor shall send the wp_image_description_v1.failed event on the"]
            #[doc = "created wp_image_description_v1 with the 'operating_system' cause."]
            #[doc = ""]
            #[doc = "The maximum size of the ICC profile is 32 MB. If length is greater than"]
            #[doc = "that or zero, the protocol error bad_size is raised. If offset + length"]
            #[doc = "exceeds the file size, the protocol error out_of_file is raised."]
            #[doc = ""]
            #[doc = "A compositor may read the file at any time starting from this request"]
            #[doc = "and only until whichever happens first:"]
            #[doc = "- If create request was issued, the wp_image_description_v1 object"]
            #[doc = "delivers either failed or ready event; or"]
            #[doc = "- if create request was not issued, this"]
            #[doc = "wp_image_description_creator_icc_v1 object is destroyed."]
            #[doc = ""]
            #[doc = "A compositor shall not modify the contents of the file, and the fd may"]
            #[doc = "be sealed for writes and size changes. The client must ensure to its"]
            #[doc = "best ability that the data does not change while the compositor is"]
            #[doc = "reading it."]
            #[doc = ""]
            #[doc = "The data must represent a valid ICC profile. The ICC profile version"]
            #[doc = "must be 2 or 4, it must be a 3 channel profile and the class must be"]
            #[doc = "Display or ColorSpace. Violating these requirements will not result in a"]
            #[doc = "protocol error, but will eventually send the"]
            #[doc = "wp_image_description_v1.failed event on the created"]
            #[doc = "wp_image_description_v1 with the 'unsupported' cause."]
            #[doc = ""]
            #[doc = "See the International Color Consortium specification ICC.1:2022 for more"]
            #[doc = "details about ICC profiles."]
            #[doc = ""]
            #[doc = "If ICC file has already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            async fn set_icc_file(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                icc_profile: rustix::fd::OwnedFd,
                offset: u32,
                length: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_icc_v1#{}.set_icc_file()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fd(icc_profile)
                    .put_uint(offset)
                    .put_uint(length)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This type of object is used for collecting all the parameters required"]
    #[doc = "to create a wp_image_description_v1 object. A complete set of required"]
    #[doc = "parameters consists of these properties:"]
    #[doc = "- transfer characteristic function (tf)"]
    #[doc = "- chromaticities of primaries and white point (primary color volume)"]
    #[doc = ""]
    #[doc = "The following properties are optional and have a well-defined default"]
    #[doc = "if not explicitly set:"]
    #[doc = "- primary color volume luminance range"]
    #[doc = "- reference white luminance level"]
    #[doc = "- mastering display primaries and white point (target color volume)"]
    #[doc = "- mastering luminance range"]
    #[doc = ""]
    #[doc = "The following properties are optional and will be ignored"]
    #[doc = "if not explicitly set:"]
    #[doc = "- maximum content light level"]
    #[doc = "- maximum frame-average light level"]
    #[doc = ""]
    #[doc = "Each required property must be set exactly once if the client is to create"]
    #[doc = "an image description. The set requests verify that a property was not"]
    #[doc = "already set. The create request verifies that all required properties are"]
    #[doc = "set. There may be several alternative requests for setting each property,"]
    #[doc = "and in that case the client must choose one of them."]
    #[doc = ""]
    #[doc = "Once all properties have been set, the create request must be used to"]
    #[doc = "create the image description object, destroying the creator in the"]
    #[doc = "process."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_image_description_creator_params_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "incomplete parameter set"]
            IncompleteSet = 0u32,
            #[doc = "property already set"]
            AlreadySet = 1u32,
            #[doc = "request not supported"]
            UnsupportedFeature = 2u32,
            #[doc = "invalid transfer characteristic"]
            InvalidTf = 3u32,
            #[doc = "invalid primaries named"]
            InvalidPrimariesNamed = 4u32,
            #[doc = "invalid luminance value or range"]
            InvalidLuminance = 5u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::IncompleteSet),
                    1u32 => Ok(Self::AlreadySet),
                    2u32 => Ok(Self::UnsupportedFeature),
                    3u32 => Ok(Self::InvalidTf),
                    4u32 => Ok(Self::InvalidPrimariesNamed),
                    5u32 => Ok(Self::InvalidLuminance),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_image_description_creator_params_v1 interface. See the module level documentation for more info"]
        pub trait WpImageDescriptionCreatorParamsV1 {
            const INTERFACE: &'static str = "wp_image_description_creator_params_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Create an image description object based on the parameters previously"]
            #[doc = "set on this object."]
            #[doc = ""]
            #[doc = "The completeness of the parameter set is verified. If the set is not"]
            #[doc = "complete, the protocol error incomplete_set is raised. For the"]
            #[doc = "definition of a complete set, see the description of this interface."]
            #[doc = ""]
            #[doc = "The protocol error invalid_luminance is raised if any of the following"]
            #[doc = "requirements is not met:"]
            #[doc = "- When max_cll is set, it must be greater than min L and less or equal"]
            #[doc = "to max L of the mastering luminance range."]
            #[doc = "- When max_fall is set, it must be greater than min L and less or equal"]
            #[doc = "to max L of the mastering luminance range."]
            #[doc = "- When both max_cll and max_fall are set, max_fall must be less or equal"]
            #[doc = "to max_cll."]
            #[doc = ""]
            #[doc = "If the particular combination of the parameter set is not supported"]
            #[doc = "by the compositor, the resulting image description object shall"]
            #[doc = "immediately deliver the wp_image_description_v1.failed event with the"]
            #[doc = "'unsupported' cause. If a valid image description was created from the"]
            #[doc = "parameter set, the wp_image_description_v1.ready event will eventually"]
            #[doc = "be sent instead."]
            #[doc = ""]
            #[doc = "This request destroys the wp_image_description_creator_params_v1"]
            #[doc = "object."]
            #[doc = ""]
            #[doc = "The resulting image description object does not allow get_information"]
            #[doc = "request."]
            #[doc = ""]
            async fn create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.create()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(image_description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the transfer characteristic using explicitly enumerated named"]
            #[doc = "functions."]
            #[doc = ""]
            #[doc = "When the resulting image description is attached to an image, the"]
            #[doc = "content should be encoded and decoded according to the industry standard"]
            #[doc = "practices for the transfer characteristic."]
            #[doc = ""]
            #[doc = "Only names advertised with wp_color_manager_v1 event supported_tf_named"]
            #[doc = "are allowed. Other values shall raise the protocol error invalid_tf."]
            #[doc = ""]
            #[doc = "If transfer characteristic has already been set on this object, the"]
            #[doc = "protocol error already_set is raised."]
            #[doc = ""]
            async fn set_tf_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tf : super :: super :: super :: staging :: color_management_v1 :: wp_color_manager_v1 :: TransferFunction,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_tf_named()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(tf as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the color component transfer characteristic to a power curve with"]
            #[doc = "the given exponent. Negative values are handled by mirroring the"]
            #[doc = "positive half of the curve through the origin. The valid domain and"]
            #[doc = "range of the curve are all finite real numbers. This curve represents"]
            #[doc = "the conversion from electrical to optical color channel values."]
            #[doc = ""]
            #[doc = "When the resulting image description is attached to an image, the"]
            #[doc = "content should be encoded with the inverse of the power curve."]
            #[doc = ""]
            #[doc = "The curve exponent shall be multiplied by 10000 to get the argument eexp"]
            #[doc = "value to carry the precision of 4 decimals."]
            #[doc = ""]
            #[doc = "The curve exponent must be at least 1.0 and at most 10.0. Otherwise the"]
            #[doc = "protocol error invalid_tf is raised."]
            #[doc = ""]
            #[doc = "If transfer characteristic has already been set on this object, the"]
            #[doc = "protocol error already_set is raised."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.set_tf_power. Otherwise this request raises"]
            #[doc = "the protocol error unsupported_feature."]
            #[doc = ""]
            async fn set_tf_power(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                eexp: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_tf_power()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(eexp).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the color primaries and white point using explicitly named sets."]
            #[doc = "This describes the primary color volume which is the basis for color"]
            #[doc = "value encoding."]
            #[doc = ""]
            #[doc = "Only names advertised with wp_color_manager_v1 event"]
            #[doc = "supported_primaries_named are allowed. Other values shall raise the"]
            #[doc = "protocol error invalid_primaries_named."]
            #[doc = ""]
            #[doc = "If primaries have already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            async fn set_primaries_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                primaries : super :: super :: super :: staging :: color_management_v1 :: wp_color_manager_v1 :: Primaries,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_primaries_named()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(primaries as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the color primaries and white point using CIE 1931 xy chromaticity"]
            #[doc = "coordinates. This describes the primary color volume which is the basis"]
            #[doc = "for color value encoding."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 1 million to get the argument"]
            #[doc = "value to carry precision of 6 decimals."]
            #[doc = ""]
            #[doc = "If primaries have already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            #[doc = "This request can be used if the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.set_primaries. Otherwise this request raises"]
            #[doc = "the protocol error unsupported_feature."]
            #[doc = ""]
            async fn set_primaries(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_primaries()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(r_x)
                    .put_int(r_y)
                    .put_int(g_x)
                    .put_int(g_y)
                    .put_int(b_x)
                    .put_int(b_y)
                    .put_int(w_x)
                    .put_int(w_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the primary color volume luminance range and the reference white"]
            #[doc = "luminance level. These values include the minimum display emission"]
            #[doc = "and ambient flare luminances, assumed to be optically additive and have"]
            #[doc = "the chromaticity of the primary color volume white point."]
            #[doc = ""]
            #[doc = "The default luminances from"]
            #[doc = "https://www.color.org/chardata/rgb/srgb.xalter are"]
            #[doc = "- primary color volume minimum: 0.2 cd/m²"]
            #[doc = "- primary color volume maximum: 80 cd/m²"]
            #[doc = "- reference white: 80 cd/m²"]
            #[doc = ""]
            #[doc = "Setting a named transfer characteristic can imply other default"]
            #[doc = "luminances."]
            #[doc = ""]
            #[doc = "The default luminances get overwritten when this request is used."]
            #[doc = "With transfer_function.st2084_pq the given 'max_lum' value is ignored,"]
            #[doc = "and 'max_lum' is taken as 'min_lum' + 10000 cd/m²."]
            #[doc = ""]
            #[doc = "'min_lum' and 'max_lum' specify the minimum and maximum luminances of"]
            #[doc = "the primary color volume as reproduced by the targeted display."]
            #[doc = ""]
            #[doc = "'reference_lum' specifies the luminance of the reference white as"]
            #[doc = "reproduced by the targeted display, and reflects the targeted viewing"]
            #[doc = "environment."]
            #[doc = ""]
            #[doc = "Compositors should make sure that all content is anchored, meaning that"]
            #[doc = "an input signal level of 'reference_lum' on one image description and"]
            #[doc = "another input signal level of 'reference_lum' on another image"]
            #[doc = "description should produce the same output level, even though the"]
            #[doc = "'reference_lum' on both image representations can be different."]
            #[doc = ""]
            #[doc = "'reference_lum' may be higher than 'max_lum'. In that case reaching"]
            #[doc = "the reference white output level in image content requires the"]
            #[doc = "'extended_target_volume' feature support."]
            #[doc = ""]
            #[doc = "If 'max_lum' or 'reference_lum' are less than or equal to 'min_lum',"]
            #[doc = "the protocol error invalid_luminance is raised."]
            #[doc = ""]
            #[doc = "The minimum luminance is multiplied by 10000 to get the argument"]
            #[doc = "'min_lum' value and carries precision of 4 decimals. The maximum"]
            #[doc = "luminance and reference white luminance values are unscaled."]
            #[doc = ""]
            #[doc = "If the primary color volume luminance range and the reference white"]
            #[doc = "luminance level have already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            #[doc = "This request can be used if the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.set_luminances. Otherwise this request"]
            #[doc = "raises the protocol error unsupported_feature."]
            #[doc = ""]
            async fn set_luminances(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
                reference_lum: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_luminances()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(min_lum)
                    .put_uint(max_lum)
                    .put_uint(reference_lum)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Provides the color primaries and white point of the mastering display"]
            #[doc = "using CIE 1931 xy chromaticity coordinates. This is compatible with the"]
            #[doc = "SMPTE ST 2086 definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "The mastering display primaries and mastering display luminances define"]
            #[doc = "the target color volume."]
            #[doc = ""]
            #[doc = "If mastering display primaries are not explicitly set, the target color"]
            #[doc = "volume is assumed to have the same primaries as the primary color volume."]
            #[doc = ""]
            #[doc = "The target color volume is defined by all tristimulus values between 0.0"]
            #[doc = "and 1.0 (inclusive) of the color space defined by the given mastering"]
            #[doc = "display primaries and white point. The colorimetry is identical between"]
            #[doc = "the container color space and the mastering display color space,"]
            #[doc = "including that no chromatic adaptation is applied even if the white"]
            #[doc = "points differ."]
            #[doc = ""]
            #[doc = "The target color volume can exceed the primary color volume to allow for"]
            #[doc = "a greater color volume with an existing color space definition (for"]
            #[doc = "example scRGB). It can be smaller than the primary color volume to"]
            #[doc = "minimize gamut and tone mapping distances for big color spaces (HDR"]
            #[doc = "metadata)."]
            #[doc = ""]
            #[doc = "To make use of the entire target color volume a suitable pixel format"]
            #[doc = "has to be chosen (e.g. floating point to exceed the primary color"]
            #[doc = "volume, or abusing limited quantization range as with xvYCC)."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 1 million to get the argument"]
            #[doc = "value to carry precision of 6 decimals."]
            #[doc = ""]
            #[doc = "If mastering display primaries have already been set on this object, the"]
            #[doc = "protocol error already_set is raised."]
            #[doc = ""]
            #[doc = "This request can be used if the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise"]
            #[doc = "this request raises the protocol error unsupported_feature. The"]
            #[doc = "advertisement implies support only for target color volumes fully"]
            #[doc = "contained within the primary color volume."]
            #[doc = ""]
            #[doc = "If a compositor additionally supports target color volume exceeding the"]
            #[doc = "primary color volume, it must advertise"]
            #[doc = "wp_color_manager_v1.feature.extended_target_volume. If a client uses"]
            #[doc = "target color volume exceeding the primary color volume and the"]
            #[doc = "compositor does not support it, the result is implementation defined."]
            #[doc = "Compositors are recommended to detect this case and fail the image"]
            #[doc = "description gracefully, but it may as well result in color artifacts."]
            #[doc = ""]
            async fn set_mastering_display_primaries(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_mastering_display_primaries()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(r_x)
                    .put_int(r_y)
                    .put_int(g_x)
                    .put_int(g_y)
                    .put_int(b_x)
                    .put_int(b_y)
                    .put_int(w_x)
                    .put_int(w_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the luminance range that was used during the content mastering"]
            #[doc = "process as the minimum and maximum absolute luminance L. These values"]
            #[doc = "include the minimum display emission and ambient flare luminances,"]
            #[doc = "assumed to be optically additive and have the chromaticity of the"]
            #[doc = "primary color volume white point. This should be"]
            #[doc = "compatible with the SMPTE ST 2086 definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "The mastering display primaries and mastering display luminances define"]
            #[doc = "the target color volume."]
            #[doc = ""]
            #[doc = "If mastering luminances are not explicitly set, the target color volume"]
            #[doc = "is assumed to have the same min and max luminances as the primary color"]
            #[doc = "volume."]
            #[doc = ""]
            #[doc = "If max L is less than or equal to min L, the protocol error"]
            #[doc = "invalid_luminance is raised."]
            #[doc = ""]
            #[doc = "Min L value is multiplied by 10000 to get the argument min_lum value"]
            #[doc = "and carry precision of 4 decimals. Max L value is unscaled for max_lum."]
            #[doc = ""]
            #[doc = "This request can be used if the compositor advertises"]
            #[doc = "wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise"]
            #[doc = "this request raises the protocol error unsupported_feature. The"]
            #[doc = "advertisement implies support only for target color volumes fully"]
            #[doc = "contained within the primary color volume."]
            #[doc = ""]
            #[doc = "If a compositor additionally supports target color volume exceeding the"]
            #[doc = "primary color volume, it must advertise"]
            #[doc = "wp_color_manager_v1.feature.extended_target_volume. If a client uses"]
            #[doc = "target color volume exceeding the primary color volume and the"]
            #[doc = "compositor does not support it, the result is implementation defined."]
            #[doc = "Compositors are recommended to detect this case and fail the image"]
            #[doc = "description gracefully, but it may as well result in color artifacts."]
            #[doc = ""]
            async fn set_mastering_luminance(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_mastering_luminance()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(min_lum)
                    .put_uint(max_lum)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the maximum content light level (max_cll) as defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "max_cll is undefined by default."]
            #[doc = ""]
            async fn set_max_cll(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_cll: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_max_cll()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(max_cll).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 8u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the maximum frame-average light level (max_fall) as defined by"]
            #[doc = "CTA-861-H."]
            #[doc = ""]
            #[doc = "max_fall is undefined by default."]
            #[doc = ""]
            async fn set_max_fall(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_fall: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_image_description_creator_params_v1#{}.set_max_fall()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(max_fall)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 9u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "An image description carries information about the color encoding used on"]
    #[doc = "a surface when attached to a wl_surface via"]
    #[doc = "wp_color_management_surface_v1.set_image_description. A compositor can use"]
    #[doc = "this information to decode pixel values into colorimetrically meaningful"]
    #[doc = "quantities."]
    #[doc = ""]
    #[doc = "Note, that the wp_image_description_v1 object is not ready to be used"]
    #[doc = "immediately after creation. The object eventually delivers either the"]
    #[doc = "'ready' or the 'failed' event, specified in all requests creating it. The"]
    #[doc = "object is deemed \"ready\" after receiving the 'ready' event."]
    #[doc = ""]
    #[doc = "An object which is not ready is illegal to use, it can only be destroyed."]
    #[doc = "Any other request in this interface shall result in the 'not_ready'"]
    #[doc = "protocol error. Attempts to use an object which is not ready through other"]
    #[doc = "interfaces shall raise protocol errors defined there."]
    #[doc = ""]
    #[doc = "Once created and regardless of how it was created, a"]
    #[doc = "wp_image_description_v1 object always refers to one fixed image"]
    #[doc = "description. It cannot change after creation."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_image_description_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "attempted to use an object which is not ready"]
            NotReady = 0u32,
            #[doc = "get_information not allowed"]
            NoInformation = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NotReady),
                    1u32 => Ok(Self::NoInformation),
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
        pub enum Cause {
            #[doc = "interface version too low"]
            LowVersion = 0u32,
            #[doc = "unsupported image description data"]
            Unsupported = 1u32,
            #[doc = "error independent of the client"]
            OperatingSystem = 2u32,
            #[doc = "the relevant output no longer exists"]
            NoOutput = 3u32,
        }
        impl TryFrom<u32> for Cause {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::LowVersion),
                    1u32 => Ok(Self::Unsupported),
                    2u32 => Ok(Self::OperatingSystem),
                    3u32 => Ok(Self::NoOutput),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Cause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_image_description_v1 interface. See the module level documentation for more info"]
        pub trait WpImageDescriptionV1 {
            const INTERFACE: &'static str = "wp_image_description_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let cause = message.uint()?;
                        let msg = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wp_image_description_v1#{}.failed({}, \"{}\")",
                            sender_id,
                            cause,
                            msg
                        );
                        self.failed(client, sender_id, cause.try_into()?, msg).await
                    }
                    1u16 => {
                        let identity = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_v1#{}.ready({})",
                            sender_id,
                            identity
                        );
                        self.ready(client, sender_id, identity).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this object. It is safe to destroy an object which is not ready."]
            #[doc = ""]
            #[doc = "Destroying a wp_image_description_v1 object has no side-effects, not"]
            #[doc = "even if a wp_color_management_surface_v1.set_image_description has not"]
            #[doc = "yet been followed by a wl_surface.commit."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_image_description_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Creates a wp_image_description_info_v1 object which delivers the"]
            #[doc = "information that makes up the image description."]
            #[doc = ""]
            #[doc = "Not all image description protocol objects allow get_information"]
            #[doc = "request. Whether it is allowed or not is defined by the request that"]
            #[doc = "created the object. If get_information is not allowed, the protocol"]
            #[doc = "error no_information is raised."]
            #[doc = ""]
            async fn get_information(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                information: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_image_description_v1#{}.get_information()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(information))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If creating a wp_image_description_v1 object fails for a reason that is"]
            #[doc = "not defined as a protocol error, this event is sent."]
            #[doc = ""]
            #[doc = "The requests that create image description objects define whether and"]
            #[doc = "when this can occur. Only such creation requests can trigger this event."]
            #[doc = "This event cannot be triggered after the image description was"]
            #[doc = "successfully formed."]
            #[doc = ""]
            #[doc = "Once this event has been sent, the wp_image_description_v1 object will"]
            #[doc = "never become ready and it can only be destroyed."]
            #[doc = ""]
            async fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                cause: Cause,
                msg: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Once this event has been sent, the wp_image_description_v1 object is"]
            #[doc = "deemed \"ready\". Ready objects can be used to send requests and can be"]
            #[doc = "used through other interfaces."]
            #[doc = ""]
            #[doc = "Every ready wp_image_description_v1 protocol object refers to an"]
            #[doc = "underlying image description record in the compositor. Multiple protocol"]
            #[doc = "objects may end up referring to the same record. Clients may identify"]
            #[doc = "these \"copies\" by comparing their id numbers: if the numbers from two"]
            #[doc = "protocol objects are identical, the protocol objects refer to the same"]
            #[doc = "image description record. Two different image description records"]
            #[doc = "cannot have the same id number simultaneously. The id number does not"]
            #[doc = "change during the lifetime of the image description record."]
            #[doc = ""]
            #[doc = "The id number is valid only as long as the protocol object is alive. If"]
            #[doc = "all protocol objects referring to the same image description record are"]
            #[doc = "destroyed, the id number may be recycled for a different image"]
            #[doc = "description record."]
            #[doc = ""]
            #[doc = "Image description id number is not a protocol object id. Zero is"]
            #[doc = "reserved as an invalid id number. It shall not be possible for a client"]
            #[doc = "to refer to an image description by its id number in protocol. The id"]
            #[doc = "numbers might not be portable between Wayland connections. A compositor"]
            #[doc = "shall not send an invalid id number."]
            #[doc = ""]
            #[doc = "This identity allows clients to de-duplicate image description records"]
            #[doc = "and avoid get_information request if they already have the image"]
            #[doc = "description information."]
            #[doc = ""]
            async fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                identity: u32,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "Sends all matching events describing an image description object exactly"]
    #[doc = "once and finally sends the 'done' event."]
    #[doc = ""]
    #[doc = "This means"]
    #[doc = "- if the image description is parametric, it must send"]
    #[doc = "- primaries"]
    #[doc = "- named_primaries, if applicable"]
    #[doc = "- at least one of tf_power and tf_named, as applicable"]
    #[doc = "- luminances"]
    #[doc = "- target_primaries"]
    #[doc = "- target_luminance"]
    #[doc = "- if the image description is parametric, it may send, if applicable,"]
    #[doc = "- target_max_cll"]
    #[doc = "- target_max_fall"]
    #[doc = "- if the image description contains an ICC profile, it must send the"]
    #[doc = "icc_file event"]
    #[doc = ""]
    #[doc = "Once a wp_image_description_info_v1 object has delivered a 'done' event it"]
    #[doc = "is automatically destroyed."]
    #[doc = ""]
    #[doc = "Every wp_image_description_info_v1 created from the same"]
    #[doc = "wp_image_description_v1 shall always return the exact same data."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_image_description_info_v1 {
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_image_description_info_v1 interface. See the module level documentation for more info"]
        pub trait WpImageDescriptionInfoV1 {
            const INTERFACE: &'static str = "wp_image_description_info_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        tracing::debug!("wp_image_description_info_v1#{}.done()", sender_id,);
                        let result = self.done(client, sender_id).await;
                        client.remove(sender_id);
                        result
                    }
                    1u16 => {
                        let icc = message.fd()?;
                        let icc_size = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.icc_file({}, {})",
                            sender_id,
                            icc.as_raw_fd(),
                            icc_size
                        );
                        self.icc_file(client, sender_id, icc, icc_size).await
                    }
                    2u16 => {
                        let r_x = message.int()?;
                        let r_y = message.int()?;
                        let g_x = message.int()?;
                        let g_y = message.int()?;
                        let b_x = message.int()?;
                        let b_y = message.int()?;
                        let w_x = message.int()?;
                        let w_y = message.int()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.primaries({}, {}, {}, {}, {}, {}, {}, {})",
                            sender_id,
                            r_x,
                            r_y,
                            g_x,
                            g_y,
                            b_x,
                            b_y,
                            w_x,
                            w_y
                        );
                        self.primaries(client, sender_id, r_x, r_y, g_x, g_y, b_x, b_y, w_x, w_y)
                            .await
                    }
                    3u16 => {
                        let primaries = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.primaries_named({})",
                            sender_id,
                            primaries
                        );
                        self.primaries_named(client, sender_id, primaries.try_into()?)
                            .await
                    }
                    4u16 => {
                        let eexp = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.tf_power({})",
                            sender_id,
                            eexp
                        );
                        self.tf_power(client, sender_id, eexp).await
                    }
                    5u16 => {
                        let tf = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.tf_named({})",
                            sender_id,
                            tf
                        );
                        self.tf_named(client, sender_id, tf.try_into()?).await
                    }
                    6u16 => {
                        let min_lum = message.uint()?;
                        let max_lum = message.uint()?;
                        let reference_lum = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.luminances({}, {}, {})",
                            sender_id,
                            min_lum,
                            max_lum,
                            reference_lum
                        );
                        self.luminances(client, sender_id, min_lum, max_lum, reference_lum)
                            .await
                    }
                    7u16 => {
                        let r_x = message.int()?;
                        let r_y = message.int()?;
                        let g_x = message.int()?;
                        let g_y = message.int()?;
                        let b_x = message.int()?;
                        let b_y = message.int()?;
                        let w_x = message.int()?;
                        let w_y = message.int()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.target_primaries({}, {}, {}, {}, {}, {}, {}, {})",
                            sender_id,
                            r_x,
                            r_y,
                            g_x,
                            g_y,
                            b_x,
                            b_y,
                            w_x,
                            w_y
                        );
                        self.target_primaries(
                            client, sender_id, r_x, r_y, g_x, g_y, b_x, b_y, w_x, w_y,
                        )
                        .await
                    }
                    8u16 => {
                        let min_lum = message.uint()?;
                        let max_lum = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.target_luminance({}, {})",
                            sender_id,
                            min_lum,
                            max_lum
                        );
                        self.target_luminance(client, sender_id, min_lum, max_lum)
                            .await
                    }
                    9u16 => {
                        let max_cll = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.target_max_cll({})",
                            sender_id,
                            max_cll
                        );
                        self.target_max_cll(client, sender_id, max_cll).await
                    }
                    10u16 => {
                        let max_fall = message.uint()?;
                        tracing::debug!(
                            "wp_image_description_info_v1#{}.target_max_fall({})",
                            sender_id,
                            max_fall
                        );
                        self.target_max_fall(client, sender_id, max_fall).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Signals the end of information events and destroys the object."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The icc argument provides a file descriptor to the client which may be"]
            #[doc = "memory-mapped to provide the ICC profile matching the image description."]
            #[doc = "The fd is read-only, and if mapped then it must be mapped with"]
            #[doc = "MAP_PRIVATE by the client."]
            #[doc = ""]
            #[doc = "The ICC profile version and other details are determined by the"]
            #[doc = "compositor. There is no provision for a client to ask for a specific"]
            #[doc = "kind of a profile."]
            #[doc = ""]
            async fn icc_file(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                icc: rustix::fd::OwnedFd,
                icc_size: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Delivers the primary color volume primaries and white point using CIE"]
            #[doc = "1931 xy chromaticity coordinates."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 1 million to get the argument"]
            #[doc = "value to carry precision of 6 decimals."]
            #[doc = ""]
            async fn primaries(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Delivers the primary color volume primaries and white point using an"]
            #[doc = "explicitly enumerated named set."]
            #[doc = ""]
            async fn primaries_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                primaries : super :: super :: super :: staging :: color_management_v1 :: wp_color_manager_v1 :: Primaries,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The color component transfer characteristic of this image description is"]
            #[doc = "a pure power curve. This event provides the exponent of the power"]
            #[doc = "function. This curve represents the conversion from electrical to"]
            #[doc = "optical pixel or color values."]
            #[doc = ""]
            #[doc = "The curve exponent has been multiplied by 10000 to get the argument eexp"]
            #[doc = "value to carry the precision of 4 decimals."]
            #[doc = ""]
            async fn tf_power(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                eexp: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Delivers the transfer characteristic using an explicitly enumerated"]
            #[doc = "named function."]
            #[doc = ""]
            async fn tf_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tf : super :: super :: super :: staging :: color_management_v1 :: wp_color_manager_v1 :: TransferFunction,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Delivers the primary color volume luminance range and the reference"]
            #[doc = "white luminance level. These values include the minimum display emission"]
            #[doc = "and ambient flare luminances, assumed to be optically additive and have"]
            #[doc = "the chromaticity of the primary color volume white point."]
            #[doc = ""]
            #[doc = "The minimum luminance is multiplied by 10000 to get the argument"]
            #[doc = "'min_lum' value and carries precision of 4 decimals. The maximum"]
            #[doc = "luminance and reference white luminance values are unscaled."]
            #[doc = ""]
            async fn luminances(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
                reference_lum: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Provides the color primaries and white point of the target color volume"]
            #[doc = "using CIE 1931 xy chromaticity coordinates. This is compatible with the"]
            #[doc = "SMPTE ST 2086 definition of HDR static metadata for mastering displays."]
            #[doc = ""]
            #[doc = "While primary color volume is about how color is encoded, the target"]
            #[doc = "color volume is the actually displayable color volume. If target color"]
            #[doc = "volume is equal to the primary color volume, then this event is not"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 1 million to get the argument"]
            #[doc = "value to carry precision of 6 decimals."]
            #[doc = ""]
            async fn target_primaries(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Provides the luminance range that the image description is targeting as"]
            #[doc = "the minimum and maximum absolute luminance L. These values include the"]
            #[doc = "minimum display emission and ambient flare luminances, assumed to be"]
            #[doc = "optically additive and have the chromaticity of the primary color"]
            #[doc = "volume white point. This should be compatible with the SMPTE ST 2086"]
            #[doc = "definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "This luminance range is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            #[doc = "Min L value is multiplied by 10000 to get the argument min_lum value and"]
            #[doc = "carry precision of 4 decimals. Max L value is unscaled for max_lum."]
            #[doc = ""]
            async fn target_luminance(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Provides the targeted max_cll of the image description. max_cll is"]
            #[doc = "defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This luminance is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            async fn target_max_cll(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_cll: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Provides the targeted max_fall of the image description. max_fall is"]
            #[doc = "defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This luminance is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            async fn target_max_fall(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_fall: u32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = ""]
#[doc = "This protocol extension delivers the metadata required to define alpha mode,"]
#[doc = "the color model, sub-sampling and quantization range used when interpreting"]
#[doc = "buffer contents. The main use case is defining how the YCbCr family of pixel"]
#[doc = "formats convert to RGB."]
#[doc = ""]
#[doc = "Note that this protocol does not define the colorimetry of the resulting RGB"]
#[doc = "channels / tristimulus values. Without the help of other extensions the"]
#[doc = "resulting colorimetry is therefore implementation defined."]
#[doc = ""]
#[doc = "If this extension is not used, the color representation used is compositor"]
#[doc = "implementation defined."]
#[doc = ""]
#[doc = "Recommendation ITU-T H.273"]
#[doc = "\"Coding-independent code points for video signal type identification\""]
#[doc = "shall be referred to as simply H.273 here."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod color_representation_v1 {
    #[doc = ""]
    #[doc = "A singleton global interface used for getting color representation"]
    #[doc = "extensions for wl_surface. The extension interfaces allow setting the"]
    #[doc = "color representation of surfaces."]
    #[doc = ""]
    #[doc = "Compositors should never remove this global."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_color_representation_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "color representation surface exists already"]
            SurfaceExists = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::SurfaceExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_color_representation_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpColorRepresentationManagerV1 {
            const INTERFACE: &'static str = "wp_color_representation_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let alpha_mode = message.uint()?;
                        tracing::debug!(
                            "wp_color_representation_manager_v1#{}.supported_alpha_mode({})",
                            sender_id,
                            alpha_mode
                        );
                        self.supported_alpha_mode(client, sender_id, alpha_mode.try_into()?)
                            .await
                    }
                    1u16 => {
                        let coefficients = message.uint()?;
                        let range = message.uint()?;
                        tracing::debug!(
                            "wp_color_representation_manager_v1#{}.supported_coefficients_and_ranges({}, {})",
                            sender_id,
                            coefficients,
                            range
                        );
                        self.supported_coefficients_and_ranges(
                            client,
                            sender_id,
                            coefficients.try_into()?,
                            range.try_into()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("wp_color_representation_manager_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the wp_color_representation_manager_v1 object. This does not"]
            #[doc = "affect any other objects in any way."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_representation_manager_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If a wp_color_representation_surface_v1 object already exists for the"]
            #[doc = "given wl_surface, the protocol error surface_exists is raised."]
            #[doc = ""]
            #[doc = "This creates a new color wp_color_representation_surface_v1 object for"]
            #[doc = "the given wl_surface."]
            #[doc = ""]
            #[doc = "See the wp_color_representation_surface_v1 interface for more details."]
            #[doc = ""]
            async fn get_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_representation_manager_v1#{}.get_surface()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each alpha mode the compositor supports."]
            #[doc = ""]
            #[doc = "For the definition of the supported values, see the"]
            #[doc = "wp_color_representation_surface_v1::alpha_mode enum."]
            #[doc = ""]
            async fn supported_alpha_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                alpha_mode : super :: super :: super :: staging :: color_representation_v1 :: wp_color_representation_surface_v1 :: AlphaMode,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each matrix coefficient and color range combination the compositor"]
            #[doc = "supports."]
            #[doc = ""]
            #[doc = "For the definition of the supported values, see the"]
            #[doc = "wp_color_representation_surface_v1::coefficients and"]
            #[doc = "wp_color_representation_surface_v1::range enums."]
            #[doc = ""]
            async fn supported_coefficients_and_ranges(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                coefficients : super :: super :: super :: staging :: color_representation_v1 :: wp_color_representation_surface_v1 :: Coefficients,
                range : super :: super :: super :: staging :: color_representation_v1 :: wp_color_representation_surface_v1 :: Range,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent when all supported features have been sent."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A wp_color_representation_surface_v1 allows the client to set the color"]
    #[doc = "representation metadata of a surface."]
    #[doc = ""]
    #[doc = "By default, a surface does not have any color representation metadata set."]
    #[doc = "The reconstruction of R, G, B signals on such surfaces is compositor"]
    #[doc = "implementation defined. The alpha mode is assumed to be"]
    #[doc = "premultiplied_electrical when the alpha mode is unset."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the wp_color_representation_surface_v1"]
    #[doc = "is destroyed, the wp_color_representation_surface_v1 object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_color_representation_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "unsupported alpha mode"]
            AlphaMode = 1u32,
            #[doc = "unsupported coefficients"]
            Coefficients = 2u32,
            #[doc = "the pixel format and a set value are incompatible"]
            PixelFormat = 3u32,
            #[doc = "forbidden request on inert object"]
            Inert = 4u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlphaMode),
                    2u32 => Ok(Self::Coefficients),
                    3u32 => Ok(Self::PixelFormat),
                    4u32 => Ok(Self::Inert),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "Specifies how the alpha channel affects the color channels."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AlphaMode {
            PremultipliedElectrical = 0u32,
            PremultipliedOptical = 1u32,
            Straight = 2u32,
        }
        impl TryFrom<u32> for AlphaMode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PremultipliedElectrical),
                    1u32 => Ok(Self::PremultipliedOptical),
                    2u32 => Ok(Self::Straight),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AlphaMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "Named matrix coefficients used to encode well-known sets of"]
        #[doc = "coefficients. H.273 is the authority, when it comes to the exact values"]
        #[doc = "of coefficients and authoritative specifications, where an equivalent"]
        #[doc = "code point exists."]
        #[doc = ""]
        #[doc = "A value of 0 is invalid and will never be present in the list of enums."]
        #[doc = ""]
        #[doc = "Descriptions do list the specifications for convenience."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Coefficients {
            Identity = 1u32,
            Bt709 = 2u32,
            Fcc = 3u32,
            Bt601 = 4u32,
            Smpte240 = 5u32,
            Bt2020 = 6u32,
            Bt2020Cl = 7u32,
            Ictcp = 8u32,
        }
        impl TryFrom<u32> for Coefficients {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Identity),
                    2u32 => Ok(Self::Bt709),
                    3u32 => Ok(Self::Fcc),
                    4u32 => Ok(Self::Bt601),
                    5u32 => Ok(Self::Smpte240),
                    6u32 => Ok(Self::Bt2020),
                    7u32 => Ok(Self::Bt2020Cl),
                    8u32 => Ok(Self::Ictcp),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Coefficients {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "Possible color range values."]
        #[doc = ""]
        #[doc = "A value of 0 is invalid and will never be present in the list of enums."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Range {
            #[doc = "Full color range"]
            Full = 1u32,
            #[doc = "Limited color range"]
            Limited = 2u32,
        }
        impl TryFrom<u32> for Range {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Limited),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Range {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = ""]
        #[doc = "Chroma sample location as defined by H.273 Chroma420SampleLocType."]
        #[doc = ""]
        #[doc = "A value of 0 is invalid and will never be present in the list of enums."]
        #[doc = ""]
        #[doc = "The descriptions list the matching Vulkan VkChromaLocation combinations"]
        #[doc = "for convenience."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ChromaLocation {
            Type0 = 1u32,
            Type1 = 2u32,
            Type2 = 3u32,
            Type3 = 4u32,
            Type4 = 5u32,
            Type5 = 6u32,
        }
        impl TryFrom<u32> for ChromaLocation {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Type0),
                    2u32 => Ok(Self::Type1),
                    3u32 => Ok(Self::Type2),
                    4u32 => Ok(Self::Type3),
                    5u32 => Ok(Self::Type4),
                    6u32 => Ok(Self::Type5),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ChromaLocation {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_color_representation_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpColorRepresentationSurfaceV1 {
            const INTERFACE: &'static str = "wp_color_representation_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the wp_color_representation_surface_v1 object."]
            #[doc = ""]
            #[doc = "Destroying this object unsets all the color representation metadata from"]
            #[doc = "the surface. See the wp_color_representation_surface_v1 interface"]
            #[doc = "description for how a compositor handles a surface without color"]
            #[doc = "representation metadata. Unsetting is double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_representation_surface_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "Assuming an alpha channel exists, it is always linear. The alpha mode"]
            #[doc = "determines whether and how the color channels include pre-multiplied"]
            #[doc = "alpha. Using straight alpha might have performance benefits."]
            #[doc = ""]
            #[doc = "Only alpha modes advertised by the compositor are allowed to be used as"]
            #[doc = "argument for this request. The \"alpha_mode\" protocol error is raised"]
            #[doc = "otherwise."]
            #[doc = ""]
            #[doc = "Alpha mode is double buffered, see wl_surface.commit."]
            #[doc = ""]
            async fn set_alpha_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                alpha_mode: AlphaMode,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_representation_surface_v1#{}.set_alpha_mode()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(alpha_mode as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "Set the matrix coefficients and video range which defines the formula"]
            #[doc = "and the related constants used to derive red, green and blue signals."]
            #[doc = "Usually coefficients correspond to MatrixCoefficients code points in"]
            #[doc = "H.273."]
            #[doc = ""]
            #[doc = "Only combinations advertised by the compositor are allowed to be used as"]
            #[doc = "argument for this request. The \"coefficients\" protocol error is raised"]
            #[doc = "otherwise."]
            #[doc = ""]
            #[doc = "A call to wl_surface.commit verifies that the pixel format and the"]
            #[doc = "coefficients-range combination in the committed surface contents are"]
            #[doc = "compatible, if contents exist. The \"pixel_format\" protocol error is"]
            #[doc = "raised otherwise."]
            #[doc = ""]
            #[doc = "A pixel format is compatible with the coefficients-range combination if"]
            #[doc = "the related equations and conventions as defined in H.273 can produce"]
            #[doc = "the color channels (RGB or YCbCr) of the pixel format."]
            #[doc = ""]
            #[doc = "For the definition of the supported combination, see the"]
            #[doc = "wp_color_representation_surface_v1::coefficients and"]
            #[doc = "wp_color_representation_surface_v1::range enums."]
            #[doc = ""]
            #[doc = "The coefficients-range combination is double-buffered, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            async fn set_coefficients_and_range(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                coefficients: Coefficients,
                range: Range,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_representation_surface_v1#{}.set_coefficients_and_range()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(coefficients as u32)
                    .put_uint(range as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "Set the chroma location type which defines the position of downsampled"]
            #[doc = "chroma samples, corresponding to Chroma420SampleLocType code points in"]
            #[doc = "H.273."]
            #[doc = ""]
            #[doc = "A call to wl_surface.commit verifies that the pixel format and chroma"]
            #[doc = "location type in the committed surface contents are compatible, if"]
            #[doc = "contents exist. The \"pixel_format\" protocol error is raised otherwise."]
            #[doc = ""]
            #[doc = "For the definition of the supported chroma location types, see the"]
            #[doc = "wp_color_representation_surface_v1::chroma_location enum."]
            #[doc = ""]
            #[doc = "The chroma location type is double-buffered, see wl_surface.commit."]
            #[doc = ""]
            async fn set_chroma_location(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                chroma_location: ChromaLocation,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_color_representation_surface_v1#{}.set_chroma_location()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(chroma_location as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod commit_timing_v1 {
    #[doc = ""]
    #[doc = "When a compositor latches on to new content updates it will check for"]
    #[doc = "any number of requirements of the available content updates (such as"]
    #[doc = "fences of all buffers being signalled) to consider the update ready."]
    #[doc = ""]
    #[doc = "This protocol provides a method for adding a time constraint to surface"]
    #[doc = "content. This constraint indicates to the compositor that a content"]
    #[doc = "update should be presented as closely as possible to, but not before,"]
    #[doc = "a specified time."]
    #[doc = ""]
    #[doc = "This protocol does not change the Wayland property that content"]
    #[doc = "updates are applied in the order they are received, even when some"]
    #[doc = "content updates contain timestamps and others do not."]
    #[doc = ""]
    #[doc = "To provide timestamps, this global factory interface must be used to"]
    #[doc = "acquire a wp_commit_timing_v1 object for a surface, which may then be"]
    #[doc = "used to provide timestamp information for commits."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_commit_timing_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "commit timer already exists for surface"]
            CommitTimerExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::CommitTimerExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_commit_timing_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpCommitTimingManagerV1 {
            const INTERFACE: &'static str = "wp_commit_timing_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object. Existing objects created by this object"]
            #[doc = "are not affected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_commit_timing_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Establish a timing controller for a surface."]
            #[doc = ""]
            #[doc = "Only one commit timer can be created for a surface, or a"]
            #[doc = "commit_timer_exists protocol error will be generated."]
            #[doc = ""]
            async fn get_timer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_commit_timing_manager_v1#{}.get_timer()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "An object to set a time constraint for a content update on a surface."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_commit_timer_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "timestamp contains an invalid value"]
            InvalidTimestamp = 0u32,
            #[doc = "timestamp exists"]
            TimestampExists = 1u32,
            #[doc = "the associated surface no longer exists"]
            SurfaceDestroyed = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidTimestamp),
                    1u32 => Ok(Self::TimestampExists),
                    2u32 => Ok(Self::SurfaceDestroyed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_commit_timer_v1 interface. See the module level documentation for more info"]
        pub trait WpCommitTimerV1 {
            const INTERFACE: &'static str = "wp_commit_timer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Provide a timing constraint for a surface content update."]
            #[doc = ""]
            #[doc = "A set_timestamp request may be made before a wl_surface.commit to"]
            #[doc = "tell the compositor that the content is intended to be presented"]
            #[doc = "as closely as possible to, but not before, the specified time."]
            #[doc = "The time is in the domain of the compositor's presentation clock."]
            #[doc = ""]
            #[doc = "An invalid_timestamp error will be generated for invalid tv_nsec."]
            #[doc = ""]
            #[doc = "If a timestamp already exists on the surface, a timestamp_exists"]
            #[doc = "error is generated."]
            #[doc = ""]
            #[doc = "Requesting set_timestamp after the commit_timer object's surface is"]
            #[doc = "destroyed will generate a \"surface_destroyed\" error."]
            #[doc = ""]
            async fn set_timestamp(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_commit_timer_v1#{}.set_timestamp()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(tv_sec_hi)
                    .put_uint(tv_sec_lo)
                    .put_uint(tv_nsec)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object."]
            #[doc = ""]
            #[doc = "Existing timing constraints are not affected by the destruction."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_commit_timer_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod content_type_v1 {
    #[doc = ""]
    #[doc = "This interface allows a client to describe the kind of content a surface"]
    #[doc = "will display, to allow the compositor to optimize its behavior for it."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_content_type_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_content_type_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpContentTypeManagerV1 {
            const INTERFACE: &'static str = "wp_content_type_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the content type manager. This doesn't destroy objects created"]
            #[doc = "with the manager."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_content_type_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a new content type object associated with the given surface."]
            #[doc = ""]
            #[doc = "Creating a wp_content_type_v1 from a wl_surface which already has one"]
            #[doc = "attached is a client error: already_constructed."]
            #[doc = ""]
            async fn get_surface_content_type(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_content_type_manager_v1#{}.get_surface_content_type()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "The content type object allows the compositor to optimize for the kind"]
    #[doc = "of content shown on the surface. A compositor may for example use it to"]
    #[doc = "set relevant drm properties like \"content type\"."]
    #[doc = ""]
    #[doc = "The client may request to switch to another content type at any time."]
    #[doc = "When the associated surface gets destroyed, this object becomes inert and"]
    #[doc = "the client should destroy it."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_content_type_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "These values describe the available content types for a surface."]
        #[doc = ""]
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
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_content_type_v1 interface. See the module level documentation for more info"]
        pub trait WpContentTypeV1 {
            const INTERFACE: &'static str = "wp_content_type_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Switch back to not specifying the content type of this surface. This is"]
            #[doc = "equivalent to setting the content type to none, including double"]
            #[doc = "buffering semantics. See set_content_type for details."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_content_type_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Set the surface content type. This informs the compositor that the"]
            #[doc = "client believes it is displaying buffers matching this content type."]
            #[doc = ""]
            #[doc = "This is purely a hint for the compositor, which can be used to adjust"]
            #[doc = "its behavior or hardware settings to fit the presented content best."]
            #[doc = ""]
            #[doc = "The content type is double-buffered state, see wl_surface.commit for"]
            #[doc = "details."]
            #[doc = ""]
            async fn set_content_type(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                content_type: Type,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_content_type_v1#{}.set_content_type()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(content_type as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod cursor_shape_v1 {
    #[doc = ""]
    #[doc = "This global offers an alternative, optional way to set cursor images. This"]
    #[doc = "new way uses enumerated cursors instead of a wl_surface like"]
    #[doc = "wl_pointer.set_cursor does."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_cursor_shape_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_cursor_shape_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpCursorShapeManagerV1 {
            const INTERFACE: &'static str = "wp_cursor_shape_manager_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the cursor shape manager."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_cursor_shape_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Obtain a wp_cursor_shape_device_v1 for a wl_pointer object."]
            #[doc = ""]
            #[doc = "When the pointer capability is removed from the wl_seat, the"]
            #[doc = "wp_cursor_shape_device_v1 object becomes inert."]
            #[doc = ""]
            async fn get_pointer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                cursor_shape_device: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_cursor_shape_manager_v1#{}.get_pointer()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(cursor_shape_device))
                    .put_object(Some(pointer))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object."]
            #[doc = ""]
            #[doc = "When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1"]
            #[doc = "object becomes inert."]
            #[doc = ""]
            async fn get_tablet_tool_v2(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                cursor_shape_device: crate::wire::ObjectId,
                tablet_tool: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_cursor_shape_manager_v1#{}.get_tablet_tool_v2()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(cursor_shape_device))
                    .put_object(Some(tablet_tool))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This interface allows clients to set the cursor shape."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_cursor_shape_device_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "This enum describes cursor shapes."]
        #[doc = ""]
        #[doc = "The names are taken from the CSS W3C specification:"]
        #[doc = "https://w3c.github.io/csswg-drafts/css-ui/#cursor"]
        #[doc = "with a few additions."]
        #[doc = ""]
        #[doc = "Note that there are some groups of cursor shapes that are related:"]
        #[doc = "The first group is drag-and-drop cursors which are used to indicate"]
        #[doc = "the selected action during dnd operations. The second group is resize"]
        #[doc = "cursors which are used to indicate resizing and moving possibilities"]
        #[doc = "on window borders. It is recommended that the shapes in these groups"]
        #[doc = "should use visually compatible images and metaphors."]
        #[doc = ""]
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
            #[doc = "drag-and-drop: the user will select which action will be carried out (non-css value)"]
            DndAsk = 35u32,
            #[doc = "resizing: something can be moved or resized in any direction (non-css value)"]
            AllResize = 36u32,
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
                    35u32 => Ok(Self::DndAsk),
                    36u32 => Ok(Self::AllResize),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Shape {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_cursor_shape_device_v1 interface. See the module level documentation for more info"]
        pub trait WpCursorShapeDeviceV1 {
            const INTERFACE: &'static str = "wp_cursor_shape_device_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the cursor shape device."]
            #[doc = ""]
            #[doc = "The device cursor shape remains unchanged."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_cursor_shape_device_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sets the device cursor to the specified shape. The compositor will"]
            #[doc = "change the cursor image based on the specified shape."]
            #[doc = ""]
            #[doc = "The cursor actually changes only if the input device focus is one of"]
            #[doc = "the requesting client's surfaces. If any, the previous cursor image"]
            #[doc = "(surface or shape) is replaced."]
            #[doc = ""]
            #[doc = "The \"shape\" argument must be a valid enum entry, otherwise the"]
            #[doc = "invalid_shape protocol error is raised."]
            #[doc = ""]
            #[doc = "This is similar to the wl_pointer.set_cursor and"]
            #[doc = "zwp_tablet_tool_v2.set_cursor requests, but this request accepts a"]
            #[doc = "shape instead of contents in the form of a surface. Clients can mix"]
            #[doc = "set_cursor and set_shape requests."]
            #[doc = ""]
            #[doc = "The serial parameter must match the latest wl_pointer.enter or"]
            #[doc = "zwp_tablet_tool_v2.proximity_in serial number sent to the client."]
            #[doc = "Otherwise the request will be ignored."]
            #[doc = ""]
            async fn set_shape(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                shape: Shape,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_cursor_shape_device_v1#{}.set_shape()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(shape as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod drm_lease_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_drm_lease_device_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_drm_lease_device_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseDeviceV1 {
            const INTERFACE: &'static str = "wp_drm_lease_device_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let fd = message.fd()?;
                        tracing::debug!(
                            "wp_drm_lease_device_v1#{}.drm_fd({})",
                            sender_id,
                            fd.as_raw_fd()
                        );
                        self.drm_fd(client, sender_id, fd).await
                    }
                    1u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("wp_drm_lease_device_v1#{}.connector({})", sender_id, id);
                        self.connector(client, sender_id, id).await
                    }
                    2u16 => {
                        tracing::debug!("wp_drm_lease_device_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    3u16 => {
                        tracing::debug!("wp_drm_lease_device_v1#{}.released()", sender_id,);
                        let result = self.released(client, sender_id).await;
                        client.remove(sender_id);
                        result
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Creates a lease request object."]
            #[doc = ""]
            #[doc = "See the documentation for wp_drm_lease_request_v1 for details."]
            #[doc = ""]
            async fn create_lease_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_drm_lease_device_v1#{}.create_lease_request()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Indicates the client no longer wishes to use this object. In response"]
            #[doc = "the compositor will immediately send the released event and destroy"]
            #[doc = "this object. It can however not guarantee that the client won't receive"]
            #[doc = "connector events before the released event. The client must not send any"]
            #[doc = "requests after this one, doing so will raise a wl_display error."]
            #[doc = "Existing connectors, lease request and leases will not be affected."]
            #[doc = ""]
            async fn release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_drm_lease_device_v1#{}.release()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The compositor will send this event when the wp_drm_lease_device_v1"]
            #[doc = "global is bound, although there are no guarantees as to how long this"]
            #[doc = "takes - the compositor might need to wait until regaining DRM master."]
            #[doc = "The included fd is a non-master DRM file descriptor opened for this"]
            #[doc = "device and the compositor must not authenticate it."]
            #[doc = "The purpose of this event is to give the client the ability to"]
            #[doc = "query DRM and discover information which may help them pick the"]
            #[doc = "appropriate DRM device or select the appropriate connectors therein."]
            #[doc = ""]
            async fn drm_fd(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The compositor will use this event to advertise connectors available for"]
            #[doc = "lease by clients. This object may be passed into a lease request to"]
            #[doc = "indicate the client would like to lease that connector, see"]
            #[doc = "wp_drm_lease_request_v1.request_connector for details. While the"]
            #[doc = "compositor will make a best effort to not send disconnected connectors,"]
            #[doc = "no guarantees can be made."]
            #[doc = ""]
            #[doc = "The compositor must send the drm_fd event before sending connectors."]
            #[doc = "After the drm_fd event it will send all available connectors but may"]
            #[doc = "send additional connectors at any time."]
            #[doc = ""]
            async fn connector(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The compositor will send this event to indicate that it has sent all"]
            #[doc = "currently available connectors after the client binds to the global or"]
            #[doc = "when it updates the connector list, for example on hotplug, drm master"]
            #[doc = "change or when a leased connector becomes available again. It will"]
            #[doc = "similarly send this event to group wp_drm_lease_connector_v1.withdrawn"]
            #[doc = "events of connectors of this device."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent in response to the release request and indicates"]
            #[doc = "that the compositor is done sending connector events."]
            #[doc = "The compositor will destroy this object immediately after sending the"]
            #[doc = "event and it will become invalid. The client should release any"]
            #[doc = "resources associated with this device after receiving this event."]
            #[doc = ""]
            async fn released(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "Represents a DRM connector which is available for lease. These objects are"]
    #[doc = "created via wp_drm_lease_device_v1.connector events, and should be passed"]
    #[doc = "to lease requests via wp_drm_lease_request_v1.request_connector."]
    #[doc = "Immediately after the wp_drm_lease_connector_v1 object is created the"]
    #[doc = "compositor will send a name, a description, a connector_id and a done"]
    #[doc = "event. When the description is updated the compositor will send a"]
    #[doc = "description event followed by a done event."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_drm_lease_connector_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_drm_lease_connector_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseConnectorV1 {
            const INTERFACE: &'static str = "wp_drm_lease_connector_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let name = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wp_drm_lease_connector_v1#{}.name(\"{}\")",
                            sender_id,
                            name
                        );
                        self.name(client, sender_id, name).await
                    }
                    1u16 => {
                        let description = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "wp_drm_lease_connector_v1#{}.description(\"{}\")",
                            sender_id,
                            description
                        );
                        self.description(client, sender_id, description).await
                    }
                    2u16 => {
                        let connector_id = message.uint()?;
                        tracing::debug!(
                            "wp_drm_lease_connector_v1#{}.connector_id({})",
                            sender_id,
                            connector_id
                        );
                        self.connector_id(client, sender_id, connector_id).await
                    }
                    3u16 => {
                        tracing::debug!("wp_drm_lease_connector_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    4u16 => {
                        tracing::debug!("wp_drm_lease_connector_v1#{}.withdrawn()", sender_id,);
                        self.withdrawn(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "The client may send this request to indicate that it will not use this"]
            #[doc = "connector. Clients are encouraged to send this after receiving the"]
            #[doc = "\"withdrawn\" event so that the server can release the resources"]
            #[doc = "associated with this connector offer. Neither existing lease requests"]
            #[doc = "nor leases will be affected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_drm_lease_connector_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The compositor sends this event once the connector is created to"]
            #[doc = "indicate the name of this connector. This will not change for the"]
            #[doc = "duration of the Wayland session, but is not guaranteed to be consistent"]
            #[doc = "between sessions."]
            #[doc = ""]
            #[doc = "If the compositor supports wl_output version 4 and this connector"]
            #[doc = "corresponds to a wl_output, the compositor should use the same name as"]
            #[doc = "for the wl_output."]
            #[doc = ""]
            async fn name(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The compositor sends this event once the connector is created to provide"]
            #[doc = "a human-readable description for this connector, which may be presented"]
            #[doc = "to the user. The compositor may send this event multiple times over the"]
            #[doc = "lifetime of this object to reflect changes in the description."]
            #[doc = ""]
            async fn description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                description: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The compositor sends this event once the connector is created to"]
            #[doc = "indicate the DRM object ID which represents the underlying connector"]
            #[doc = "that is being offered. Note that the final lease may include additional"]
            #[doc = "object IDs, such as CRTCs and planes."]
            #[doc = ""]
            async fn connector_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                connector_id: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent after all properties of a connector have been sent."]
            #[doc = "This allows changes to the properties to be seen as atomic even if they"]
            #[doc = "happen via multiple events."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Sent to indicate that the compositor will no longer honor requests for"]
            #[doc = "DRM leases which include this connector. The client may still issue a"]
            #[doc = "lease request including this connector, but the compositor will send"]
            #[doc = "wp_drm_lease_v1.finished without issuing a lease fd. Compositors are"]
            #[doc = "encouraged to send this event when they lose access to connector, for"]
            #[doc = "example when the connector is hot-unplugged, when the connector gets"]
            #[doc = "leased to a client or when the compositor loses DRM master."]
            #[doc = ""]
            #[doc = "If a client holds a lease for the connector, the status of the lease"]
            #[doc = "remains the same. The client should destroy the object after receiving"]
            #[doc = "this event."]
            #[doc = ""]
            async fn withdrawn(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A client that wishes to lease DRM resources will attach the list of"]
    #[doc = "connectors advertised with wp_drm_lease_device_v1.connector that they"]
    #[doc = "wish to lease, then use wp_drm_lease_request_v1.submit to submit the"]
    #[doc = "request."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_drm_lease_request_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_drm_lease_request_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseRequestV1 {
            const INTERFACE: &'static str = "wp_drm_lease_request_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Indicates that the client would like to lease the given connector."]
            #[doc = "This is only used as a suggestion, the compositor may choose to"]
            #[doc = "include any resources in the lease it issues, or change the set of"]
            #[doc = "leased resources at any time. Compositors are however encouraged to"]
            #[doc = "include the requested connector and other resources necessary"]
            #[doc = "to drive the connected output in the lease."]
            #[doc = ""]
            #[doc = "Requesting a connector that was created from a different lease device"]
            #[doc = "than this lease request raises the wrong_device error. Requesting a"]
            #[doc = "connector twice will raise the duplicate_connector error."]
            #[doc = ""]
            async fn request_connector(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                connector: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_drm_lease_request_v1#{}.request_connector()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(connector))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Submits the lease request and creates a new wp_drm_lease_v1 object."]
            #[doc = "After calling submit the compositor will immediately destroy this"]
            #[doc = "object, issuing any more requests will cause a wl_display error."]
            #[doc = "The compositor doesn't make any guarantees about the events of the"]
            #[doc = "lease object, clients cannot expect an immediate response."]
            #[doc = "Not requesting any connectors before submitting the lease request"]
            #[doc = "will raise the empty_lease error."]
            #[doc = ""]
            async fn submit(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_drm_lease_request_v1#{}.submit()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "A DRM lease object is used to transfer the DRM file descriptor to the"]
    #[doc = "client and manage the lifetime of the lease."]
    #[doc = ""]
    #[doc = "Some time after the wp_drm_lease_v1 object is created, the compositor"]
    #[doc = "will reply with the lease request's result. If the lease request is"]
    #[doc = "granted, the compositor will send a lease_fd event. If the lease request"]
    #[doc = "is denied, the compositor will send a finished event without a lease_fd"]
    #[doc = "event."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_drm_lease_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_drm_lease_v1 interface. See the module level documentation for more info"]
        pub trait WpDrmLeaseV1 {
            const INTERFACE: &'static str = "wp_drm_lease_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let leased_fd = message.fd()?;
                        tracing::debug!(
                            "wp_drm_lease_v1#{}.lease_fd({})",
                            sender_id,
                            leased_fd.as_raw_fd()
                        );
                        self.lease_fd(client, sender_id, leased_fd).await
                    }
                    1u16 => {
                        tracing::debug!("wp_drm_lease_v1#{}.finished()", sender_id,);
                        self.finished(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "The client should send this to indicate that it no longer wishes to use"]
            #[doc = "this lease. The compositor should use drmModeRevokeLease on the"]
            #[doc = "appropriate file descriptor, if necessary."]
            #[doc = ""]
            #[doc = "Upon destruction, the compositor should advertise the connector for"]
            #[doc = "leasing again by sending the connector event through the"]
            #[doc = "wp_drm_lease_device_v1 interface."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_drm_lease_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event returns a file descriptor suitable for use with DRM-related"]
            #[doc = "ioctls. The client should use drmModeGetLease to enumerate the DRM"]
            #[doc = "objects which have been leased to them. The compositor guarantees it"]
            #[doc = "will not use the leased DRM objects itself until it sends the finished"]
            #[doc = "event. If the compositor cannot or will not grant a lease for the"]
            #[doc = "requested connectors, it will not send this event, instead sending the"]
            #[doc = "finished event."]
            #[doc = ""]
            #[doc = "The compositor will send this event at most once during this objects"]
            #[doc = "lifetime."]
            #[doc = ""]
            async fn lease_fd(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                leased_fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The compositor uses this event to either reject a lease request, or if"]
            #[doc = "it previously sent a lease_fd, to notify the client that the lease has"]
            #[doc = "been revoked. If the client requires a new lease, they should destroy"]
            #[doc = "this object and submit a new lease request. The compositor will send"]
            #[doc = "no further events for this object after sending the finish event."]
            #[doc = "Compositors should revoke the lease when any of the leased resources"]
            #[doc = "become unavailable, namely when a hot-unplug occurs or when the"]
            #[doc = "compositor loses DRM master. Compositors may advertise the connector"]
            #[doc = "for leasing again, if the resource is available, by sending the"]
            #[doc = "connector event through the wp_drm_lease_device_v1 interface."]
            #[doc = ""]
            async fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ext_background_effect_v1 {
    #[doc = ""]
    #[doc = "This protocol provides a way to improve visuals of translucent surfaces"]
    #[doc = "by applying effects like blur to the background behind them."]
    #[doc = ""]
    #[doc = "The capabilities are send when the global is bound, and every time they"]
    #[doc = "change. Note that when the capability goes away, the corresponding effect"]
    #[doc = "is no longer applied by the compositor, even if it was set before."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_background_effect_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a background effect object"]
            BackgroundEffectExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BackgroundEffectExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "the compositor supports applying blur"] const Blur = 0u32 ; } }
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
        #[doc = "Trait to implement the ext_background_effect_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtBackgroundEffectManagerV1 {
            const INTERFACE: &'static str = "ext_background_effect_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let flags = message.uint()?;
                        tracing::debug!(
                            "ext_background_effect_manager_v1#{}.capabilities({})",
                            sender_id,
                            flags
                        );
                        self.capabilities(client, sender_id, flags.try_into()?)
                            .await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will no longer be using this"]
            #[doc = "protocol object. Existing objects created by this object are not"]
            #[doc = "affected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_background_effect_manager_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Instantiate an interface extension for the given wl_surface to add"]
            #[doc = "effects like blur for the background behind it."]
            #[doc = ""]
            #[doc = "If the given wl_surface already has a ext_background_effect_surface_v1"]
            #[doc = "object associated, the background_effect_exists protocol error will be"]
            #[doc = "raised."]
            #[doc = ""]
            async fn get_background_effect(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_background_effect_manager_v1#{}.get_background_effect()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            async fn capabilities(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                flags: Capability,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "The background effect object provides a way to specify a region behind"]
    #[doc = "a surface that should have background effects like blur applied."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the ext_background_effect_surface_v1"]
    #[doc = "object has been destroyed, this object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_background_effect_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the associated surface has been destroyed"]
            SurfaceDestroyed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SurfaceDestroyed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_background_effect_surface_v1 interface. See the module level documentation for more info"]
        pub trait ExtBackgroundEffectSurfaceV1 {
            const INTERFACE: &'static str = "ext_background_effect_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will no longer be using this protocol"]
            #[doc = "object. The effect regions will be removed on the next commit."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_background_effect_surface_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request sets the region of the surface that will have its"]
            #[doc = "background blurred."]
            #[doc = ""]
            #[doc = "The blur region is specified in the surface-local coordinates, and"]
            #[doc = "clipped by the compositor to the surface size."]
            #[doc = ""]
            #[doc = "The initial value for the blur region is empty. Setting the pending"]
            #[doc = "blur region has copy semantics, and the wl_region object can be"]
            #[doc = "destroyed immediately. A NULL wl_region removes the effect."]
            #[doc = ""]
            #[doc = "The blur region is double-buffered state, and will be applied on"]
            #[doc = "the next wl_surface.commit."]
            #[doc = ""]
            #[doc = "The blur algorithm is subject to compositor policies."]
            #[doc = ""]
            #[doc = "If the associated surface has been destroyed, the surface_destroyed"]
            #[doc = "error will be raised."]
            #[doc = ""]
            async fn set_blur_region(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                region: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_background_effect_surface_v1#{}.set_blur_region()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(region)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
#[doc = "This protocol allows a privileged client to control data devices. In"]
#[doc = "particular, the client will be able to manage the current selection and take"]
#[doc = "the role of a clipboard manager."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod ext_data_control_v1 {
    #[doc = ""]
    #[doc = "This interface is a manager that allows creating per-seat data device"]
    #[doc = "controls."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_data_control_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_data_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtDataControlManagerV1 {
            const INTERFACE: &'static str = "ext_data_control_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Create a new data source."]
            #[doc = ""]
            async fn create_data_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_data_control_manager_v1#{}.create_data_source()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a data device that can be used to manage a seat's selection."]
            #[doc = ""]
            async fn get_data_device(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_data_control_manager_v1#{}.get_data_device()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(seat))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "All objects created by the manager will still remain valid, until their"]
            #[doc = "appropriate destroy request has been called."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_data_control_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This interface allows a client to manage a seat's selection."]
    #[doc = ""]
    #[doc = "When the seat is destroyed, this object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_data_control_device_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_data_control_device_v1 interface. See the module level documentation for more info"]
        pub trait ExtDataControlDeviceV1 {
            const INTERFACE: &'static str = "ext_data_control_device_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let id = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_data_control_device_v1#{}.data_offer({})",
                            sender_id,
                            id
                        );
                        self.data_offer(client, sender_id, id).await
                    }
                    1u16 => {
                        let id = message.object()?;
                        tracing::debug!(
                            "ext_data_control_device_v1#{}.selection({})",
                            sender_id,
                            id.as_ref().map_or("null".to_string(), |v| v.to_string())
                        );
                        self.selection(client, sender_id, id).await
                    }
                    2u16 => {
                        tracing::debug!("ext_data_control_device_v1#{}.finished()", sender_id,);
                        self.finished(client, sender_id).await
                    }
                    3u16 => {
                        let id = message.object()?;
                        tracing::debug!(
                            "ext_data_control_device_v1#{}.primary_selection({})",
                            sender_id,
                            id.as_ref().map_or("null".to_string(), |v| v.to_string())
                        );
                        self.primary_selection(client, sender_id, id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This request asks the compositor to set the selection to the data from"]
            #[doc = "the source on behalf of the client."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "set_primary_selection requests. Attempting to use a previously used"]
            #[doc = "source triggers the used_source protocol error."]
            #[doc = ""]
            #[doc = "To unset the selection, set the source to NULL."]
            #[doc = ""]
            async fn set_selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_data_control_device_v1#{}.set_selection()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the data device object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_data_control_device_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request asks the compositor to set the primary selection to the"]
            #[doc = "data from the source on behalf of the client."]
            #[doc = ""]
            #[doc = "The given source may not be used in any further set_selection or"]
            #[doc = "set_primary_selection requests. Attempting to use a previously used"]
            #[doc = "source triggers the used_source protocol error."]
            #[doc = ""]
            #[doc = "To unset the primary selection, set the source to NULL."]
            #[doc = ""]
            #[doc = "The compositor will ignore this request if it does not support primary"]
            #[doc = "selection."]
            #[doc = ""]
            async fn set_primary_selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_data_control_device_v1#{}.set_primary_selection()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(source)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The data_offer event introduces a new ext_data_control_offer object,"]
            #[doc = "which will subsequently be used in either the"]
            #[doc = "ext_data_control_device.selection event (for the regular clipboard"]
            #[doc = "selections) or the ext_data_control_device.primary_selection event (for"]
            #[doc = "the primary clipboard selections). Immediately following the"]
            #[doc = "ext_data_control_device.data_offer event, the new data_offer object"]
            #[doc = "will send out ext_data_control_offer.offer events to describe the MIME"]
            #[doc = "types it offers."]
            #[doc = ""]
            async fn data_offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The selection event is sent out to notify the client of a new"]
            #[doc = "ext_data_control_offer for the selection for this device. The"]
            #[doc = "ext_data_control_device.data_offer and the ext_data_control_offer.offer"]
            #[doc = "events are sent out immediately before this event to introduce the data"]
            #[doc = "offer object. The selection event is sent to a client when a new"]
            #[doc = "selection is set. The ext_data_control_offer is valid until a new"]
            #[doc = "ext_data_control_offer or NULL is received. The client must destroy the"]
            #[doc = "previous selection ext_data_control_offer, if any, upon receiving this"]
            #[doc = "event. Regardless, the previous selection will be ignored once a new"]
            #[doc = "selection ext_data_control_offer is received."]
            #[doc = ""]
            #[doc = "The first selection event is sent upon binding the"]
            #[doc = "ext_data_control_device object."]
            #[doc = ""]
            async fn selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This data control object is no longer valid and should be destroyed by"]
            #[doc = "the client."]
            #[doc = ""]
            async fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The primary_selection event is sent out to notify the client of a new"]
            #[doc = "ext_data_control_offer for the primary selection for this device. The"]
            #[doc = "ext_data_control_device.data_offer and the ext_data_control_offer.offer"]
            #[doc = "events are sent out immediately before this event to introduce the data"]
            #[doc = "offer object. The primary_selection event is sent to a client when a"]
            #[doc = "new primary selection is set. The ext_data_control_offer is valid until"]
            #[doc = "a new ext_data_control_offer or NULL is received. The client must"]
            #[doc = "destroy the previous primary selection ext_data_control_offer, if any,"]
            #[doc = "upon receiving this event. Regardless, the previous primary selection"]
            #[doc = "will be ignored once a new primary selection ext_data_control_offer is"]
            #[doc = "received."]
            #[doc = ""]
            #[doc = "If the compositor supports primary selection, the first"]
            #[doc = "primary_selection event is sent upon binding the"]
            #[doc = "ext_data_control_device object."]
            #[doc = ""]
            async fn primary_selection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "The ext_data_control_source object is the source side of a"]
    #[doc = "ext_data_control_offer. It is created by the source client in a data"]
    #[doc = "transfer and provides a way to describe the offered data and a way to"]
    #[doc = "respond to requests to transfer the data."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_data_control_source_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "offer sent after ext_data_control_device.set_selection"]
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_data_control_source_v1 interface. See the module level documentation for more info"]
        pub trait ExtDataControlSourceV1 {
            const INTERFACE: &'static str = "ext_data_control_source_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let mime_type = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        let fd = message.fd()?;
                        tracing::debug!(
                            "ext_data_control_source_v1#{}.send(\"{}\", {})",
                            sender_id,
                            mime_type,
                            fd.as_raw_fd()
                        );
                        self.send(client, sender_id, mime_type, fd).await
                    }
                    1u16 => {
                        tracing::debug!("ext_data_control_source_v1#{}.cancelled()", sender_id,);
                        self.cancelled(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This request adds a MIME type to the set of MIME types advertised to"]
            #[doc = "targets. Can be called several times to offer multiple types."]
            #[doc = ""]
            #[doc = "Calling this after ext_data_control_device.set_selection is a protocol"]
            #[doc = "error."]
            #[doc = ""]
            async fn offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_data_control_source_v1#{}.offer()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the data source object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_data_control_source_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Request for data from the client. Send the data as the specified MIME"]
            #[doc = "type over the passed file descriptor, then close it."]
            #[doc = ""]
            async fn send(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This data source is no longer valid. The data source has been replaced"]
            #[doc = "by another data source."]
            #[doc = ""]
            #[doc = "The client should clean up and destroy this data source."]
            #[doc = ""]
            async fn cancelled(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A ext_data_control_offer represents a piece of data offered for transfer"]
    #[doc = "by another client (the source client). The offer describes the different"]
    #[doc = "MIME types that the data can be converted to and provides the mechanism"]
    #[doc = "for transferring the data directly from the source client."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_data_control_offer_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_data_control_offer_v1 interface. See the module level documentation for more info"]
        pub trait ExtDataControlOfferV1 {
            const INTERFACE: &'static str = "ext_data_control_offer_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let mime_type = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_data_control_offer_v1#{}.offer(\"{}\")",
                            sender_id,
                            mime_type
                        );
                        self.offer(client, sender_id, mime_type).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "To transfer the offered data, the client issues this request and"]
            #[doc = "indicates the MIME type it wants to receive. The transfer happens"]
            #[doc = "through the passed file descriptor (typically created with the pipe"]
            #[doc = "system call). The source client writes the data in the MIME type"]
            #[doc = "representation requested and then closes the file descriptor."]
            #[doc = ""]
            #[doc = "The receiving client reads from the read end of the pipe until EOF and"]
            #[doc = "then closes its end, at which point the transfer is complete."]
            #[doc = ""]
            #[doc = "This request may happen multiple times for different MIME types."]
            #[doc = ""]
            async fn receive(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_data_control_offer_v1#{}.receive()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .put_fd(fd)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the data offer object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_data_control_offer_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sent immediately after creating the ext_data_control_offer object."]
            #[doc = "One event per offered MIME type."]
            #[doc = ""]
            async fn offer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                mime_type: String,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod ext_foreign_toplevel_list_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_foreign_toplevel_list_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_foreign_toplevel_list_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelListV1 {
            const INTERFACE: &'static str = "ext_foreign_toplevel_list_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let toplevel = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_foreign_toplevel_list_v1#{}.toplevel({})",
                            sender_id,
                            toplevel
                        );
                        self.toplevel(client, sender_id, toplevel).await
                    }
                    1u16 => {
                        tracing::debug!("ext_foreign_toplevel_list_v1#{}.finished()", sender_id,);
                        self.finished(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events for new toplevels."]
            #[doc = ""]
            #[doc = "The Wayland protocol is asynchronous, meaning the compositor may send"]
            #[doc = "further toplevel events until the stop request is processed."]
            #[doc = "The client should wait for a ext_foreign_toplevel_list_v1.finished"]
            #[doc = "event before destroying this object."]
            #[doc = ""]
            async fn stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_list_v1#{}.stop()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request should be called either when the client will no longer"]
            #[doc = "use the ext_foreign_toplevel_list_v1 or after the finished event"]
            #[doc = "has been received to allow destruction of the object."]
            #[doc = ""]
            #[doc = "If a client wishes to destroy this object it should send a"]
            #[doc = "ext_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished"]
            #[doc = "event, then destroy the handles and then this object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_list_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever a new toplevel window is created. It is"]
            #[doc = "emitted for all toplevels, regardless of the app that has created them."]
            #[doc = ""]
            #[doc = "All initial properties of the toplevel (identifier, title, app_id) will be sent"]
            #[doc = "immediately after this event using the corresponding events for"]
            #[doc = "ext_foreign_toplevel_handle_v1. The compositor will use the"]
            #[doc = "ext_foreign_toplevel_handle_v1.done event to indicate when all data has"]
            #[doc = "been sent."]
            #[doc = ""]
            async fn toplevel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event indicates that the compositor is done sending events"]
            #[doc = "to this object. The client should destroy the object."]
            #[doc = "See ext_foreign_toplevel_list_v1.destroy for more information."]
            #[doc = ""]
            #[doc = "The compositor must not send any more toplevel events after this event."]
            #[doc = ""]
            async fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A ext_foreign_toplevel_handle_v1 object represents a mapped toplevel"]
    #[doc = "window. A single app may have multiple mapped toplevels."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_foreign_toplevel_handle_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_foreign_toplevel_handle_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelHandleV1 {
            const INTERFACE: &'static str = "ext_foreign_toplevel_handle_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        tracing::debug!("ext_foreign_toplevel_handle_v1#{}.closed()", sender_id,);
                        self.closed(client, sender_id).await
                    }
                    1u16 => {
                        tracing::debug!("ext_foreign_toplevel_handle_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    2u16 => {
                        let title = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_foreign_toplevel_handle_v1#{}.title(\"{}\")",
                            sender_id,
                            title
                        );
                        self.title(client, sender_id, title).await
                    }
                    3u16 => {
                        let app_id = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_foreign_toplevel_handle_v1#{}.app_id(\"{}\")",
                            sender_id,
                            app_id
                        );
                        self.app_id(client, sender_id, app_id).await
                    }
                    4u16 => {
                        let identifier = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_foreign_toplevel_handle_v1#{}.identifier(\"{}\")",
                            sender_id,
                            identifier
                        );
                        self.identifier(client, sender_id, identifier).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This request should be used when the client will no longer use the handle"]
            #[doc = "or after the closed event has been received to allow destruction of the"]
            #[doc = "object."]
            #[doc = ""]
            #[doc = "When a handle is destroyed, a new handle may not be created by the server"]
            #[doc = "until the toplevel is unmapped and then remapped. Destroying a toplevel handle"]
            #[doc = "is not recommended unless the client is cleaning up child objects"]
            #[doc = "before destroying the ext_foreign_toplevel_list_v1 object, the toplevel"]
            #[doc = "was closed or the toplevel handle will not be used in the future."]
            #[doc = ""]
            #[doc = "Other protocols which extend the ext_foreign_toplevel_handle_v1"]
            #[doc = "interface should require destructors for extension interfaces be"]
            #[doc = "called before allowing the toplevel handle to be destroyed."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_foreign_toplevel_handle_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The server will emit no further events on the ext_foreign_toplevel_handle_v1"]
            #[doc = "after this event. Any requests received aside from the destroy request must"]
            #[doc = "be ignored. Upon receiving this event, the client should destroy the handle."]
            #[doc = ""]
            #[doc = "Other protocols which extend the ext_foreign_toplevel_handle_v1"]
            #[doc = "interface must also ignore requests other than destructors."]
            #[doc = ""]
            async fn closed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent after all changes in the toplevel state have"]
            #[doc = "been sent."]
            #[doc = ""]
            #[doc = "This allows changes to the ext_foreign_toplevel_handle_v1 properties"]
            #[doc = "to be atomically applied. Other protocols which extend the"]
            #[doc = "ext_foreign_toplevel_handle_v1 interface may use this event to also"]
            #[doc = "atomically apply any pending state."]
            #[doc = ""]
            #[doc = "This event must not be sent after the ext_foreign_toplevel_handle_v1.closed"]
            #[doc = "event."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The title of the toplevel has changed."]
            #[doc = ""]
            #[doc = "The configured state must not be applied immediately. See"]
            #[doc = "ext_foreign_toplevel_handle_v1.done for details."]
            #[doc = ""]
            async fn title(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                title: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The app id of the toplevel has changed."]
            #[doc = ""]
            #[doc = "The configured state must not be applied immediately. See"]
            #[doc = "ext_foreign_toplevel_handle_v1.done for details."]
            #[doc = ""]
            async fn app_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                app_id: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This identifier is used to check if two or more toplevel handles belong"]
            #[doc = "to the same toplevel."]
            #[doc = ""]
            #[doc = "The identifier is useful for command line tools or privileged clients"]
            #[doc = "which may need to reference an exact toplevel across processes or"]
            #[doc = "instances of the ext_foreign_toplevel_list_v1 global."]
            #[doc = ""]
            #[doc = "The compositor must only send this event when the handle is created."]
            #[doc = ""]
            #[doc = "The identifier must be unique per toplevel and it's handles. Two different"]
            #[doc = "toplevels must not have the same identifier. The identifier is only valid"]
            #[doc = "as long as the toplevel is mapped. If the toplevel is unmapped the identifier"]
            #[doc = "must not be reused. An identifier must not be reused by the compositor to"]
            #[doc = "ensure there are no races when sharing identifiers between processes."]
            #[doc = ""]
            #[doc = "An identifier is a string that contains up to 32 printable ASCII bytes."]
            #[doc = "An identifier must not be an empty string. It is recommended that a"]
            #[doc = "compositor includes an opaque generation value in identifiers. How the"]
            #[doc = "generation value is used when generating the identifier is implementation"]
            #[doc = "dependent."]
            #[doc = ""]
            async fn identifier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                identifier: String,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ext_idle_notify_v1 {
    #[doc = ""]
    #[doc = "This interface allows clients to monitor user idle status."]
    #[doc = ""]
    #[doc = "After binding to this global, clients can create ext_idle_notification_v1"]
    #[doc = "objects to get notified when the user is idle for a given amount of time."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_idle_notifier_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_idle_notifier_v1 interface. See the module level documentation for more info"]
        pub trait ExtIdleNotifierV1 {
            const INTERFACE: &'static str = "ext_idle_notifier_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the manager object. All objects created via this interface"]
            #[doc = "remain valid."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_idle_notifier_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a new idle notification object."]
            #[doc = ""]
            #[doc = "The notification object has a minimum timeout duration and is tied to a"]
            #[doc = "seat. The client will be notified if the seat is inactive for at least"]
            #[doc = "the provided timeout. See ext_idle_notification_v1 for more details."]
            #[doc = ""]
            #[doc = "A zero timeout is valid and means the client wants to be notified as"]
            #[doc = "soon as possible when the seat is inactive."]
            #[doc = ""]
            async fn get_idle_notification(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                timeout: u32,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_idle_notifier_v1#{}.get_idle_notification()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_uint(timeout)
                    .put_object(Some(seat))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a new idle notification object to track input from the"]
            #[doc = "user, such as keyboard and mouse movement. Because this object is"]
            #[doc = "meant to track user input alone, it ignores idle inhibitors."]
            #[doc = ""]
            #[doc = "The notification object has a minimum timeout duration and is tied to a"]
            #[doc = "seat. The client will be notified if the seat is inactive for at least"]
            #[doc = "the provided timeout. See ext_idle_notification_v1 for more details."]
            #[doc = ""]
            #[doc = "A zero timeout is valid and means the client wants to be notified as"]
            #[doc = "soon as possible when the seat is inactive."]
            #[doc = ""]
            async fn get_input_idle_notification(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                timeout: u32,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_idle_notifier_v1#{}.get_input_idle_notification()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_uint(timeout)
                    .put_object(Some(seat))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This interface is used by the compositor to send idle notification events"]
    #[doc = "to clients."]
    #[doc = ""]
    #[doc = "Initially the notification object is not idle. The notification object"]
    #[doc = "becomes idle when no user activity has happened for at least the timeout"]
    #[doc = "duration, starting from the creation of the notification object. User"]
    #[doc = "activity may include input events or a presence sensor, but is"]
    #[doc = "compositor-specific."]
    #[doc = ""]
    #[doc = "How this notification responds to idle inhibitors depends on how"]
    #[doc = "it was constructed. If constructed from the"]
    #[doc = "get_idle_notification request, then if an idle inhibitor is"]
    #[doc = "active (e.g. another client has created a zwp_idle_inhibitor_v1"]
    #[doc = "on a visible surface), the compositor must not make the"]
    #[doc = "notification object idle. However, if constructed from the"]
    #[doc = "get_input_idle_notification request, then idle inhibitors are"]
    #[doc = "ignored, and only input from the user, e.g. from a keyboard or"]
    #[doc = "mouse, counts as activity."]
    #[doc = ""]
    #[doc = "When the notification object becomes idle, an idled event is sent. When"]
    #[doc = "user activity starts again, the notification object stops being idle,"]
    #[doc = "a resumed event is sent and the timeout is restarted."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_idle_notification_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_idle_notification_v1 interface. See the module level documentation for more info"]
        pub trait ExtIdleNotificationV1 {
            const INTERFACE: &'static str = "ext_idle_notification_v1";
            const VERSION: u32 = 2u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        tracing::debug!("ext_idle_notification_v1#{}.idled()", sender_id,);
                        self.idled(client, sender_id).await
                    }
                    1u16 => {
                        tracing::debug!("ext_idle_notification_v1#{}.resumed()", sender_id,);
                        self.resumed(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the notification object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_idle_notification_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event is sent when the notification object becomes idle."]
            #[doc = ""]
            #[doc = "It's a compositor protocol error to send this event twice without a"]
            #[doc = "resumed event in-between."]
            #[doc = ""]
            async fn idled(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent when the notification object stops being idle."]
            #[doc = ""]
            #[doc = "It's a compositor protocol error to send this event twice without an"]
            #[doc = "idled event in-between. It's a compositor protocol error to send this"]
            #[doc = "event prior to any idled event."]
            #[doc = ""]
            async fn resumed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = ""]
#[doc = "This protocol serves as an intermediary between capturing protocols and"]
#[doc = "potential image capture sources such as outputs and toplevels."]
#[doc = ""]
#[doc = "This protocol may be extended to support more image capture sources in the"]
#[doc = "future, thereby adding those image capture sources to other protocols that"]
#[doc = "use the image capture source object without having to modify those"]
#[doc = "protocols."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod ext_image_capture_source_v1 {
    #[doc = ""]
    #[doc = "The image capture source object is an opaque descriptor for a capturable"]
    #[doc = "resource.  This resource may be any sort of entity from which an image"]
    #[doc = "may be derived."]
    #[doc = ""]
    #[doc = "Note, because ext_image_capture_source_v1 objects are created from multiple"]
    #[doc = "independent factory interfaces, the ext_image_capture_source_v1 interface is"]
    #[doc = "frozen at version 1."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_image_capture_source_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_image_capture_source_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCaptureSourceV1 {
            const INTERFACE: &'static str = "ext_image_capture_source_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the image capture source. This request may be sent at any time"]
            #[doc = "by the client."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_image_capture_source_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "A manager for creating image capture source objects for wl_output objects."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_output_image_capture_source_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_output_image_capture_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtOutputImageCaptureSourceManagerV1 {
            const INTERFACE: &'static str = "ext_output_image_capture_source_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Creates a source object for an output. Images captured from this source"]
            #[doc = "will show the same content as the output. Some elements may be omitted,"]
            #[doc = "such as cursors and overlays that have been marked as transparent to"]
            #[doc = "capturing."]
            #[doc = ""]
            async fn create_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_output_image_capture_source_manager_v1#{}.create_source()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(source))
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_output_image_capture_source_manager_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "A manager for creating image capture source objects for"]
    #[doc = "ext_foreign_toplevel_handle_v1 objects."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_foreign_toplevel_image_capture_source_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_foreign_toplevel_image_capture_source_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtForeignToplevelImageCaptureSourceManagerV1 {
            const INTERFACE: &'static str = "ext_foreign_toplevel_image_capture_source_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Creates a source object for a foreign toplevel handle. Images captured"]
            #[doc = "from this source will show the same content as the toplevel."]
            #[doc = ""]
            async fn create_source(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                toplevel_handle: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_foreign_toplevel_image_capture_source_manager_v1#{}.create_source()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(source))
                    .put_object(Some(toplevel_handle))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the manager. This request may be sent at any time by the client"]
            #[doc = "and objects created by the manager will remain valid after its"]
            #[doc = "destruction."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_foreign_toplevel_image_capture_source_manager_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
#[doc = "This protocol allows clients to ask the compositor to capture image sources"]
#[doc = "such as outputs and toplevels into user submitted buffers."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod ext_image_copy_capture_v1 {
    #[doc = ""]
    #[doc = "This object is a manager which offers requests to start capturing from a"]
    #[doc = "source."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_image_copy_capture_manager_v1 {
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
        #[doc = "Trait to implement the ext_image_copy_capture_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureManagerV1 {
            const INTERFACE: &'static str = "ext_image_copy_capture_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Create a capturing session for an image capture source."]
            #[doc = ""]
            #[doc = "If the paint_cursors option is set, cursors shall be composited onto"]
            #[doc = "the captured frame. The cursor must not be composited onto the frame"]
            #[doc = "if this flag is not set."]
            #[doc = ""]
            #[doc = "If the options bitfield is invalid, the invalid_option protocol error"]
            #[doc = "is sent."]
            #[doc = ""]
            async fn create_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                session: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                options: Options,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_manager_v1#{}.create_session()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(session))
                    .put_object(Some(source))
                    .put_uint(options.bits())
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a cursor capturing session for the pointer of an image capture"]
            #[doc = "source."]
            #[doc = ""]
            async fn create_pointer_cursor_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                session: crate::wire::ObjectId,
                source: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_manager_v1#{}.create_pointer_cursor_session()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(session))
                    .put_object(Some(source))
                    .put_object(Some(pointer))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroy the manager object."]
            #[doc = ""]
            #[doc = "Other objects created via this interface are unaffected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_manager_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This object represents an active image copy capture session."]
    #[doc = ""]
    #[doc = "After a capture session is created, buffer constraint events will be"]
    #[doc = "emitted from the compositor to tell the client which buffer types and"]
    #[doc = "formats are supported for reading from the session. The compositor may"]
    #[doc = "re-send buffer constraint events whenever they change."]
    #[doc = ""]
    #[doc = "To advertise buffer constraints, the compositor must send in no"]
    #[doc = "particular order: zero or more shm_format and dmabuf_format events, zero"]
    #[doc = "or one dmabuf_device event, and exactly one buffer_size event. Then the"]
    #[doc = "compositor must send a done event."]
    #[doc = ""]
    #[doc = "When the client has received all the buffer constraints, it can create a"]
    #[doc = "buffer accordingly, attach it to the capture session using the"]
    #[doc = "attach_buffer request, set the buffer damage using the damage_buffer"]
    #[doc = "request and then send the capture request."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_image_copy_capture_session_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "create_frame sent before destroying previous frame"]
            DuplicateFrame = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::DuplicateFrame),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_image_copy_capture_session_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureSessionV1 {
            const INTERFACE: &'static str = "ext_image_copy_capture_session_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let width = message.uint()?;
                        let height = message.uint()?;
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.buffer_size({}, {})",
                            sender_id,
                            width,
                            height
                        );
                        self.buffer_size(client, sender_id, width, height).await
                    }
                    1u16 => {
                        let format = message.uint()?;
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.shm_format({})",
                            sender_id,
                            format
                        );
                        self.shm_format(client, sender_id, format.try_into()?).await
                    }
                    2u16 => {
                        let device = message.array()?;
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.dmabuf_device(array[{}])",
                            sender_id,
                            device.len()
                        );
                        self.dmabuf_device(client, sender_id, device).await
                    }
                    3u16 => {
                        let format = message.uint()?;
                        let modifiers = message.array()?;
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.dmabuf_format({}, array[{}])",
                            sender_id,
                            format,
                            modifiers.len()
                        );
                        self.dmabuf_format(client, sender_id, format, modifiers)
                            .await
                    }
                    4u16 => {
                        tracing::debug!("ext_image_copy_capture_session_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    5u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_session_v1#{}.stopped()",
                            sender_id,
                        );
                        self.stopped(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Create a capture frame for this session."]
            #[doc = ""]
            #[doc = "At most one frame object can exist for a given session at any time. If"]
            #[doc = "a client sends a create_frame request before a previous frame object"]
            #[doc = "has been destroyed, the duplicate_frame protocol error is raised."]
            #[doc = ""]
            async fn create_frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                frame: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.create_frame()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(frame))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect ext_image_copy_capture_frame_v1 objects created by"]
            #[doc = "this object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_session_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Provides the dimensions of the source image in buffer pixel coordinates."]
            #[doc = ""]
            #[doc = "The client must attach buffers that match this size."]
            #[doc = ""]
            async fn buffer_size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Provides the format that must be used for shared-memory buffers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            #[doc = ""]
            async fn shm_format(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: super::super::super::core::wayland::wl_shm::Format,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event advertises the device buffers must be allocated on for"]
            #[doc = "dma-buf buffers."]
            #[doc = ""]
            #[doc = "In general the device is a DRM node. The DRM node type (primary vs."]
            #[doc = "render) is unspecified. Clients must not rely on the compositor sending"]
            #[doc = "a particular node type. Clients cannot check two devices for equality"]
            #[doc = "by comparing the dev_t value."]
            #[doc = ""]
            async fn dmabuf_device(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                device: Vec<u8>,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Provides the format that must be used for dma-buf buffers."]
            #[doc = ""]
            #[doc = "The client may choose any of the modifiers advertised in the array of"]
            #[doc = "64-bit unsigned integers."]
            #[doc = ""]
            #[doc = "This event may be emitted multiple times, in which case the client may"]
            #[doc = "choose any given format."]
            #[doc = ""]
            async fn dmabuf_format(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                format: u32,
                modifiers: Vec<u8>,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent once when all buffer constraint events have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "The compositor must always end a batch of buffer constraint events with"]
            #[doc = "this event, regardless of whether it sends the initial constraints or"]
            #[doc = "an update."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event indicates that the capture session has stopped and is no"]
            #[doc = "longer available. This can happen in a number of cases, e.g. when the"]
            #[doc = "underlying source is destroyed, if the user decides to end the image"]
            #[doc = "capture, or if an unrecoverable runtime error has occurred."]
            #[doc = ""]
            #[doc = "The client should destroy the session after receiving this event."]
            #[doc = ""]
            async fn stopped(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "This object represents an image capture frame."]
    #[doc = ""]
    #[doc = "The client should attach a buffer, damage the buffer, and then send a"]
    #[doc = "capture request."]
    #[doc = ""]
    #[doc = "If the capture is successful, the compositor must send the frame metadata"]
    #[doc = "(transform, damage, presentation_time in any order) followed by the ready"]
    #[doc = "event."]
    #[doc = ""]
    #[doc = "If the capture fails, the compositor must send the failed event."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_image_copy_capture_frame_v1 {
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
        #[doc = "Trait to implement the ext_image_copy_capture_frame_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureFrameV1 {
            const INTERFACE: &'static str = "ext_image_copy_capture_frame_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let transform = message.uint()?;
                        tracing::debug!(
                            "ext_image_copy_capture_frame_v1#{}.transform({})",
                            sender_id,
                            transform
                        );
                        self.transform(client, sender_id, transform.try_into()?)
                            .await
                    }
                    1u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        let width = message.int()?;
                        let height = message.int()?;
                        tracing::debug!(
                            "ext_image_copy_capture_frame_v1#{}.damage({}, {}, {}, {})",
                            sender_id,
                            x,
                            y,
                            width,
                            height
                        );
                        self.damage(client, sender_id, x, y, width, height).await
                    }
                    2u16 => {
                        let tv_sec_hi = message.uint()?;
                        let tv_sec_lo = message.uint()?;
                        let tv_nsec = message.uint()?;
                        tracing::debug!(
                            "ext_image_copy_capture_frame_v1#{}.presentation_time({}, {}, {})",
                            sender_id,
                            tv_sec_hi,
                            tv_sec_lo,
                            tv_nsec
                        );
                        self.presentation_time(client, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec)
                            .await
                    }
                    3u16 => {
                        tracing::debug!("ext_image_copy_capture_frame_v1#{}.ready()", sender_id,);
                        self.ready(client, sender_id).await
                    }
                    4u16 => {
                        let reason = message.uint()?;
                        tracing::debug!(
                            "ext_image_copy_capture_frame_v1#{}.failed({})",
                            sender_id,
                            reason
                        );
                        self.failed(client, sender_id, reason.try_into()?).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the frame. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_image_copy_capture_frame_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Attach a buffer to the session."]
            #[doc = ""]
            #[doc = "The wl_buffer.release request is unused."]
            #[doc = ""]
            #[doc = "The new buffer replaces any previously attached buffer."]
            #[doc = ""]
            #[doc = "This request must not be sent after capture, or else the"]
            #[doc = "already_captured protocol error is raised."]
            #[doc = ""]
            async fn attach_buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_frame_v1#{}.attach_buffer()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Apply damage to the buffer which is to be captured next. This request"]
            #[doc = "may be sent multiple times to describe a region."]
            #[doc = ""]
            #[doc = "The client indicates the accumulated damage since this wl_buffer was"]
            #[doc = "last captured. During capture, the compositor will update the buffer"]
            #[doc = "with at least the union of the region passed by the client and the"]
            #[doc = "region advertised by ext_image_copy_capture_frame_v1.damage."]
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
            async fn damage_buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_frame_v1#{}.damage_buffer()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
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
            async fn capture(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_image_copy_capture_frame_v1#{}.capture()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event is sent before the ready event and holds the transform that"]
            #[doc = "the compositor has applied to the buffer contents."]
            #[doc = ""]
            async fn transform(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                transform: super::super::super::core::wayland::wl_output::Transform,
            ) -> crate::client::Result<()>;
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
            async fn damage(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> crate::client::Result<()>;
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
            async fn presentation_time(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Called as soon as the frame is copied, indicating it is available"]
            #[doc = "for reading."]
            #[doc = ""]
            #[doc = "The buffer may be re-used by the client after this event."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            #[doc = ""]
            async fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event indicates that the attempted frame copy has failed."]
            #[doc = ""]
            #[doc = "After receiving this event, the client must destroy the object."]
            #[doc = ""]
            async fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                reason: FailureReason,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "This object represents a cursor capture session. It extends the base"]
    #[doc = "capture session with cursor-specific metadata."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_image_copy_capture_cursor_session_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "get_capture_session sent twice"]
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
        #[doc = "Trait to implement the ext_image_copy_capture_cursor_session_v1 interface. See the module level documentation for more info"]
        pub trait ExtImageCopyCaptureCursorSessionV1 {
            const INTERFACE: &'static str = "ext_image_copy_capture_cursor_session_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_cursor_session_v1#{}.enter()",
                            sender_id,
                        );
                        self.enter(client, sender_id).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "ext_image_copy_capture_cursor_session_v1#{}.leave()",
                            sender_id,
                        );
                        self.leave(client, sender_id).await
                    }
                    2u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        tracing::debug!(
                            "ext_image_copy_capture_cursor_session_v1#{}.position({}, {})",
                            sender_id,
                            x,
                            y
                        );
                        self.position(client, sender_id, x, y).await
                    }
                    3u16 => {
                        let x = message.int()?;
                        let y = message.int()?;
                        tracing::debug!(
                            "ext_image_copy_capture_cursor_session_v1#{}.hotspot({}, {})",
                            sender_id,
                            x,
                            y
                        );
                        self.hotspot(client, sender_id, x, y).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the session. This request can be sent at any time by the"]
            #[doc = "client."]
            #[doc = ""]
            #[doc = "This request doesn't affect ext_image_copy_capture_frame_v1 objects created by"]
            #[doc = "this object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_cursor_session_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Gets the image copy capture session for this cursor session."]
            #[doc = ""]
            #[doc = "The session will produce frames of the cursor image. The compositor may"]
            #[doc = "pause the session when the cursor leaves the captured area."]
            #[doc = ""]
            #[doc = "This request must not be sent more than once, or else the"]
            #[doc = "duplicate_session protocol error is raised."]
            #[doc = ""]
            async fn get_capture_session(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                session: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_image_copy_capture_cursor_session_v1#{}.get_capture_session()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(session))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Sent when a cursor enters the captured area. It shall be generated"]
            #[doc = "before the \"position\" and \"hotspot\" events when and only when a cursor"]
            #[doc = "enters the area."]
            #[doc = ""]
            #[doc = "The cursor enters the captured area when the cursor image intersects"]
            #[doc = "with the captured area. Note, this is different from e.g."]
            #[doc = "wl_pointer.enter."]
            #[doc = ""]
            async fn enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Sent when a cursor leaves the captured area. No \"position\" or \"hotspot\""]
            #[doc = "event is generated for the cursor until the cursor enters the captured"]
            #[doc = "area again."]
            #[doc = ""]
            async fn leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "Cursors outside the image capture source do not get captured and no"]
            #[doc = "event will be generated for them."]
            #[doc = ""]
            #[doc = "The given position is the position of the cursor's hotspot and it is"]
            #[doc = "relative to the main buffer's top left corner in transformed buffer"]
            #[doc = "pixel coordinates. The coordinates may be negative or greater than the"]
            #[doc = "main buffer size."]
            #[doc = ""]
            async fn position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The hotspot describes the offset between the cursor image and the"]
            #[doc = "position of the input device."]
            #[doc = ""]
            #[doc = "The given coordinates are the hotspot's offset from the origin in"]
            #[doc = "buffer coordinates."]
            #[doc = ""]
            #[doc = "Clients should not apply the hotspot immediately: the hotspot becomes"]
            #[doc = "effective when the next ext_image_copy_capture_frame_v1.ready event is received."]
            #[doc = ""]
            #[doc = "Compositors may delay this event until the client captures a new frame."]
            #[doc = ""]
            async fn hotspot(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod ext_session_lock_v1 {
    #[doc = ""]
    #[doc = "This interface is used to request that the session be locked."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_session_lock_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_session_lock_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockManagerV1 {
            const INTERFACE: &'static str = "ext_session_lock_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This informs the compositor that the session lock manager object will"]
            #[doc = "no longer be used. Existing objects created through this interface"]
            #[doc = "remain valid."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_session_lock_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request creates a session lock and asks the compositor to lock the"]
            #[doc = "session. The compositor will send either the ext_session_lock_v1.locked"]
            #[doc = "or ext_session_lock_v1.finished event on the created object in"]
            #[doc = "response to this request."]
            #[doc = ""]
            async fn lock(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_session_lock_manager_v1#{}.lock()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_session_lock_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_session_lock_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockV1 {
            const INTERFACE: &'static str = "ext_session_lock_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        tracing::debug!("ext_session_lock_v1#{}.locked()", sender_id,);
                        self.locked(client, sender_id).await
                    }
                    1u16 => {
                        tracing::debug!("ext_session_lock_v1#{}.finished()", sender_id,);
                        self.finished(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This informs the compositor that the lock object will no longer be"]
            #[doc = "used. Existing objects created through this interface remain valid."]
            #[doc = ""]
            #[doc = "After this request is made, lock surfaces created through this object"]
            #[doc = "should be destroyed by the client as they will no longer be used by"]
            #[doc = "the compositor."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request if the locked event was"]
            #[doc = "sent, the unlock_and_destroy request must be used instead."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_session_lock_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The client is expected to create lock surfaces for all outputs"]
            #[doc = "currently present and any new outputs as they are advertised. These"]
            #[doc = "won't be displayed by the compositor unless the lock is successful"]
            #[doc = "and the locked event is sent."]
            #[doc = ""]
            #[doc = "Providing a wl_surface which already has a role or already has a buffer"]
            #[doc = "attached or committed is a protocol error, as is attaching/committing"]
            #[doc = "a buffer before the first ext_session_lock_surface_v1.configure event."]
            #[doc = ""]
            #[doc = "Attempting to create more than one lock surface for a given output"]
            #[doc = "is a duplicate_output protocol error."]
            #[doc = ""]
            async fn get_lock_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_session_lock_v1#{}.get_lock_surface()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request indicates that the session should be unlocked, for"]
            #[doc = "example because the user has entered their password and it has been"]
            #[doc = "verified by the client."]
            #[doc = ""]
            #[doc = "This request also informs the compositor that the lock object will"]
            #[doc = "no longer be used and should be destroyed. Existing objects created"]
            #[doc = "through this interface remain valid."]
            #[doc = ""]
            #[doc = "After this request is made, lock surfaces created through this object"]
            #[doc = "should be destroyed by the client as they will no longer be used by"]
            #[doc = "the compositor."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request if the locked event has"]
            #[doc = "not been sent. In that case, the lock object must be destroyed using"]
            #[doc = "the destroy request."]
            #[doc = ""]
            #[doc = "Note that a correct client that wishes to exit directly after unlocking"]
            #[doc = "the session must use the wl_display.sync request to ensure the server"]
            #[doc = "receives and processes the unlock_and_destroy request. Otherwise"]
            #[doc = "there is no guarantee that the server has unlocked the session due"]
            #[doc = "to the asynchronous nature of the Wayland protocol. For example,"]
            #[doc = "the server might terminate the client with a protocol error before"]
            #[doc = "it processes the unlock_and_destroy request."]
            #[doc = ""]
            async fn unlock_and_destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_session_lock_v1#{}.unlock_and_destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This client is now responsible for displaying graphics while the"]
            #[doc = "session is locked and deciding when to unlock the session."]
            #[doc = ""]
            #[doc = "The locked event must not be sent until a new \"locked\" frame has been"]
            #[doc = "presented on all outputs and no security sensitive normal/unlocked"]
            #[doc = "content is possibly visible."]
            #[doc = ""]
            #[doc = "If this event is sent, making the destroy request is a protocol error,"]
            #[doc = "the lock object must be destroyed using the unlock_and_destroy request."]
            #[doc = ""]
            async fn locked(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The compositor has decided that the session lock should be destroyed"]
            #[doc = "as it will no longer be used by the compositor. Exactly when this"]
            #[doc = "event is sent is compositor policy, but it must never be sent more"]
            #[doc = "than once for a given session lock object."]
            #[doc = ""]
            #[doc = "This might be sent because there is already another ext_session_lock_v1"]
            #[doc = "object held by a client, or the compositor has decided to deny the"]
            #[doc = "request to lock the session for some other reason. This might also"]
            #[doc = "be sent because the compositor implements some alternative, secure"]
            #[doc = "way to authenticate and unlock the session."]
            #[doc = ""]
            #[doc = "The finished event should be sent immediately on creation of this"]
            #[doc = "object if the compositor decides that the locked event will not"]
            #[doc = "be sent."]
            #[doc = ""]
            #[doc = "If the locked event is sent on creation of this object the finished"]
            #[doc = "event may still be sent at some later time in this object's"]
            #[doc = "lifetime. This is compositor policy."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should make either the destroy"]
            #[doc = "request or the unlock_and_destroy request, depending on whether or"]
            #[doc = "not the locked event was received on this object."]
            #[doc = ""]
            async fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_session_lock_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_session_lock_surface_v1 interface. See the module level documentation for more info"]
        pub trait ExtSessionLockSurfaceV1 {
            const INTERFACE: &'static str = "ext_session_lock_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let serial = message.uint()?;
                        let width = message.uint()?;
                        let height = message.uint()?;
                        tracing::debug!(
                            "ext_session_lock_surface_v1#{}.configure({}, {}, {})",
                            sender_id,
                            serial,
                            width,
                            height
                        );
                        self.configure(client, sender_id, serial, width, height)
                            .await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "This informs the compositor that the lock surface object will no"]
            #[doc = "longer be used."]
            #[doc = ""]
            #[doc = "It is recommended for a lock client to destroy lock surfaces if"]
            #[doc = "their corresponding wl_output global is removed."]
            #[doc = ""]
            #[doc = "If a lock surface on an active output is destroyed before the"]
            #[doc = "ext_session_lock_v1.unlock_and_destroy event is sent, the compositor"]
            #[doc = "must fall back to rendering a solid color."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_session_lock_surface_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "When a configure event is received, if a client commits the surface"]
            #[doc = "in response to the configure event, then the client must make an"]
            #[doc = "ack_configure request sometime before the commit request, passing"]
            #[doc = "along the serial of the configure event."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure events before it can"]
            #[doc = "respond to one, it only has to ack the last configure event."]
            #[doc = ""]
            #[doc = "A client is not required to commit immediately after sending an"]
            #[doc = "ack_configure request - it may even ack_configure several times"]
            #[doc = "before its next surface commit."]
            #[doc = ""]
            #[doc = "A client may send multiple ack_configure requests before committing,"]
            #[doc = "but only the last request sent before a commit indicates which"]
            #[doc = "configure event the client really is responding to."]
            #[doc = ""]
            #[doc = "Sending an ack_configure request consumes the configure event"]
            #[doc = "referenced by the given serial, as well as all older configure events"]
            #[doc = "sent on this object."]
            #[doc = ""]
            #[doc = "It is a protocol error to issue multiple ack_configure requests"]
            #[doc = "referencing the same configure event or to issue an ack_configure"]
            #[doc = "request referencing a configure event older than the last configure"]
            #[doc = "event acked for a given lock surface."]
            #[doc = ""]
            async fn ack_configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_session_lock_surface_v1#{}.ack_configure()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event is sent once on binding the interface and may be sent again"]
            #[doc = "at the compositor's discretion, for example if output geometry changes."]
            #[doc = ""]
            #[doc = "The width and height are in surface-local coordinates and are exact"]
            #[doc = "requirements. Failing to match these surface dimensions in the next"]
            #[doc = "commit after acking a configure is a protocol error."]
            #[doc = ""]
            async fn configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                width: u32,
                height: u32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod ext_transient_seat_v1 {
    #[doc = ""]
    #[doc = "The transient seat manager creates short-lived seats."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_transient_seat_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_transient_seat_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtTransientSeatManagerV1 {
            const INTERFACE: &'static str = "ext_transient_seat_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Create a new seat that is removed when the client side transient seat"]
            #[doc = "object is destroyed."]
            #[doc = ""]
            #[doc = "The actual seat may be removed sooner, in which case the transient seat"]
            #[doc = "object shall become inert."]
            #[doc = ""]
            async fn create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_transient_seat_manager_v1#{}.create()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(seat))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroy the manager."]
            #[doc = ""]
            #[doc = "All objects created by the manager will remain valid until they are"]
            #[doc = "destroyed themselves."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_transient_seat_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "When the transient seat handle is destroyed, the seat itself will also be"]
    #[doc = "destroyed."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_transient_seat_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_transient_seat_v1 interface. See the module level documentation for more info"]
        pub trait ExtTransientSeatV1 {
            const INTERFACE: &'static str = "ext_transient_seat_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let global_name = message.uint()?;
                        tracing::debug!(
                            "ext_transient_seat_v1#{}.ready({})",
                            sender_id,
                            global_name
                        );
                        self.ready(client, sender_id, global_name).await
                    }
                    1u16 => {
                        tracing::debug!("ext_transient_seat_v1#{}.denied()", sender_id,);
                        self.denied(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "When the transient seat object is destroyed by the client, the"]
            #[doc = "associated seat created by the compositor is also destroyed."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_transient_seat_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event advertises the global name for the wl_seat to be used with"]
            #[doc = "wl_registry_bind."]
            #[doc = ""]
            #[doc = "It is sent exactly once, immediately after the transient seat is created"]
            #[doc = "and the new \"wl_seat\" global is advertised, if and only if the creation"]
            #[doc = "of the transient seat was allowed."]
            #[doc = ""]
            async fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                global_name: u32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "The event informs the client that the compositor denied its request to"]
            #[doc = "create a transient seat."]
            #[doc = ""]
            #[doc = "It is sent exactly once, immediately after the transient seat object is"]
            #[doc = "created, if and only if the creation of the transient seat was denied."]
            #[doc = ""]
            #[doc = "After receiving this event, the client should destroy the object."]
            #[doc = ""]
            async fn denied(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ext_workspace_v1 {
    #[doc = ""]
    #[doc = "Workspaces, also called virtual desktops, are groups of surfaces. A"]
    #[doc = "compositor with a concept of workspaces may only show some such groups of"]
    #[doc = "surfaces (those of 'active' workspaces) at a time.\u{a0}'Activating' a"]
    #[doc = "workspace is a request for the compositor to display that workspace's"]
    #[doc = "surfaces as normal, whereas the compositor may hide or otherwise"]
    #[doc = "de-emphasise surfaces that are associated only with 'inactive' workspaces."]
    #[doc = "Workspaces are grouped by which sets of outputs they correspond to, and"]
    #[doc = "may contain surfaces only from those outputs. In this way, it is possible"]
    #[doc = "for each output to have its own set of workspaces, or for all outputs (or"]
    #[doc = "any other arbitrary grouping) to share workspaces. Compositors may"]
    #[doc = "optionally conceptually arrange each group of workspaces in an"]
    #[doc = "N-dimensional grid."]
    #[doc = ""]
    #[doc = "The purpose of this protocol is to enable the creation of taskbars and"]
    #[doc = "docks by providing them with a list of workspaces and their properties,"]
    #[doc = "and allowing them to activate and deactivate workspaces."]
    #[doc = ""]
    #[doc = "After a client binds the ext_workspace_manager_v1, each workspace will be"]
    #[doc = "sent via the workspace event."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_workspace_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ext_workspace_manager_v1 interface. See the module level documentation for more info"]
        pub trait ExtWorkspaceManagerV1 {
            const INTERFACE: &'static str = "ext_workspace_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let workspace_group = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_workspace_manager_v1#{}.workspace_group({})",
                            sender_id,
                            workspace_group
                        );
                        self.workspace_group(client, sender_id, workspace_group)
                            .await
                    }
                    1u16 => {
                        let workspace = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_workspace_manager_v1#{}.workspace({})",
                            sender_id,
                            workspace
                        );
                        self.workspace(client, sender_id, workspace).await
                    }
                    2u16 => {
                        tracing::debug!("ext_workspace_manager_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    3u16 => {
                        tracing::debug!("ext_workspace_manager_v1#{}.finished()", sender_id,);
                        let result = self.finished(client, sender_id).await;
                        client.remove(sender_id);
                        result
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "The client must send this request after it has finished sending other"]
            #[doc = "requests. The compositor must process a series of requests preceding a"]
            #[doc = "commit request atomically."]
            #[doc = ""]
            #[doc = "This allows changes to the workspace properties to be seen as atomic,"]
            #[doc = "even if they happen via multiple events, and even if they involve"]
            #[doc = "multiple ext_workspace_handle_v1 objects, for example, deactivating one"]
            #[doc = "workspace and activating another."]
            #[doc = ""]
            async fn commit(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_manager_v1#{}.commit()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Indicates the client no longer wishes to receive events for new"]
            #[doc = "workspace groups. However the compositor may emit further workspace"]
            #[doc = "events, until the finished event is emitted. The compositor is expected"]
            #[doc = "to send the finished event eventually once the stop request has been processed."]
            #[doc = ""]
            #[doc = "The client must not send any requests after this one, doing so will raise a wl_display"]
            #[doc = "invalid_object error."]
            #[doc = ""]
            async fn stop(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_manager_v1#{}.stop()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event is emitted whenever a new workspace group has been created."]
            #[doc = ""]
            #[doc = "All initial details of the workspace group (outputs) will be"]
            #[doc = "sent immediately after this event via the corresponding events in"]
            #[doc = "ext_workspace_group_handle_v1 and ext_workspace_handle_v1."]
            #[doc = ""]
            async fn workspace_group(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                workspace_group: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted whenever a new workspace has been created."]
            #[doc = ""]
            #[doc = "All initial details of the workspace (name, coordinates, state) will"]
            #[doc = "be sent immediately after this event via the corresponding events in"]
            #[doc = "ext_workspace_handle_v1."]
            #[doc = ""]
            #[doc = "Workspaces start off unassigned to any workspace group."]
            #[doc = ""]
            async fn workspace(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent after all changes in all workspaces and workspace groups have been"]
            #[doc = "sent."]
            #[doc = ""]
            #[doc = "This allows changes to one or more ext_workspace_group_handle_v1"]
            #[doc = "properties and ext_workspace_handle_v1 properties"]
            #[doc = "to be seen as atomic, even if they happen via multiple events."]
            #[doc = "In particular, an output moving from one workspace group to"]
            #[doc = "another sends an output_enter event and an output_leave event to the two"]
            #[doc = "ext_workspace_group_handle_v1 objects in question. The compositor sends"]
            #[doc = "the done event only after updating the output information in both"]
            #[doc = "workspace groups."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event indicates that the compositor is done sending events to the"]
            #[doc = "ext_workspace_manager_v1. The server will destroy the object"]
            #[doc = "immediately after sending this request."]
            #[doc = ""]
            async fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A ext_workspace_group_handle_v1 object represents a workspace group"]
    #[doc = "that is assigned a set of outputs and contains a number of workspaces."]
    #[doc = ""]
    #[doc = "The set of outputs assigned to the workspace group is conveyed to the client via"]
    #[doc = "output_enter and output_leave events, and its workspaces are conveyed with"]
    #[doc = "workspace events."]
    #[doc = ""]
    #[doc = "For example, a compositor which has a set of workspaces for each output may"]
    #[doc = "advertise a workspace group (and its workspaces) per output, whereas a compositor"]
    #[doc = "where a workspace spans all outputs may advertise a single workspace group for all"]
    #[doc = "outputs."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_workspace_group_handle_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct GroupCapabilities : u32 { # [doc = "create_workspace request is available"] const CreateWorkspace = 1u32 ; } }
        impl TryFrom<u32> for GroupCapabilities {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for GroupCapabilities {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_workspace_group_handle_v1 interface. See the module level documentation for more info"]
        pub trait ExtWorkspaceGroupHandleV1 {
            const INTERFACE: &'static str = "ext_workspace_group_handle_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let capabilities = message.uint()?;
                        tracing::debug!(
                            "ext_workspace_group_handle_v1#{}.capabilities({})",
                            sender_id,
                            capabilities
                        );
                        self.capabilities(client, sender_id, capabilities.try_into()?)
                            .await
                    }
                    1u16 => {
                        let output = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_workspace_group_handle_v1#{}.output_enter({})",
                            sender_id,
                            output
                        );
                        self.output_enter(client, sender_id, output).await
                    }
                    2u16 => {
                        let output = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_workspace_group_handle_v1#{}.output_leave({})",
                            sender_id,
                            output
                        );
                        self.output_leave(client, sender_id, output).await
                    }
                    3u16 => {
                        let workspace = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_workspace_group_handle_v1#{}.workspace_enter({})",
                            sender_id,
                            workspace
                        );
                        self.workspace_enter(client, sender_id, workspace).await
                    }
                    4u16 => {
                        let workspace = message
                            .object()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "ext_workspace_group_handle_v1#{}.workspace_leave({})",
                            sender_id,
                            workspace
                        );
                        self.workspace_leave(client, sender_id, workspace).await
                    }
                    5u16 => {
                        tracing::debug!("ext_workspace_group_handle_v1#{}.removed()", sender_id,);
                        self.removed(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Request that the compositor create a new workspace with the given name"]
            #[doc = "and assign it to this group."]
            #[doc = ""]
            #[doc = "There is no guarantee that the compositor will create a new workspace,"]
            #[doc = "or that the created workspace will have the provided name."]
            #[doc = ""]
            async fn create_workspace(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                workspace: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> ext_workspace_group_handle_v1#{}.create_workspace()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(workspace))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroys the ext_workspace_group_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be send either when the client does not want to"]
            #[doc = "use the workspace group object any more or after the removed event to finalize"]
            #[doc = "the destruction of the object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_group_handle_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event advertises the capabilities supported by the compositor. If"]
            #[doc = "a capability isn't supported, clients should hide or disable the UI"]
            #[doc = "elements that expose this functionality. For instance, if the"]
            #[doc = "compositor doesn't advertise support for creating workspaces, a button"]
            #[doc = "triggering the create_workspace request should not be displayed."]
            #[doc = ""]
            #[doc = "The compositor will ignore requests it doesn't support. For instance,"]
            #[doc = "a compositor which doesn't advertise support for creating workspaces will ignore"]
            #[doc = "create_workspace requests."]
            #[doc = ""]
            #[doc = "Compositors must send this event once after creation of an"]
            #[doc = "ext_workspace_group_handle_v1. When the capabilities change, compositors"]
            #[doc = "must send this event again."]
            #[doc = ""]
            async fn capabilities(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                capabilities: GroupCapabilities,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted whenever an output is assigned to the workspace"]
            #[doc = "group or a new `wl_output` object is bound by the client, which was already"]
            #[doc = "assigned to this workspace_group."]
            #[doc = ""]
            async fn output_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted whenever an output is removed from the workspace"]
            #[doc = "group."]
            #[doc = ""]
            async fn output_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted whenever a workspace is assigned to this group."]
            #[doc = "A workspace may only ever be assigned to a single group at a single point"]
            #[doc = "in time, but can be re-assigned during it's lifetime."]
            #[doc = ""]
            async fn workspace_enter(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted whenever a workspace is removed from this group."]
            #[doc = ""]
            async fn workspace_leave(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                workspace: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is send when the group associated with the ext_workspace_group_handle_v1"]
            #[doc = "has been removed. After sending this request the compositor will immediately consider"]
            #[doc = "the object inert. Any requests will be ignored except the destroy request."]
            #[doc = "It is guaranteed there won't be any more events referencing this"]
            #[doc = "ext_workspace_group_handle_v1."]
            #[doc = ""]
            #[doc = "The compositor must remove all workspaces belonging to a workspace group"]
            #[doc = "via a workspace_leave event before removing the workspace group."]
            #[doc = ""]
            async fn removed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "A ext_workspace_handle_v1 object represents a workspace that handles a"]
    #[doc = "group of surfaces."]
    #[doc = ""]
    #[doc = "Each workspace has:"]
    #[doc = "- a name, conveyed to the client with the name event"]
    #[doc = "- potentially an id conveyed with the id event"]
    #[doc = "- a list of states, conveyed to the client with the state event"]
    #[doc = "- and optionally a set of coordinates, conveyed to the client with the"]
    #[doc = "coordinates event"]
    #[doc = ""]
    #[doc = "The client may request that the compositor activate or deactivate the workspace."]
    #[doc = ""]
    #[doc = "Each workspace can belong to only a single workspace group."]
    #[doc = "Depending on the compositor policy, there might be workspaces with"]
    #[doc = "the same name in different workspace groups, but these workspaces are still"]
    #[doc = "separate (e.g. one of them might be active while the other is not)."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ext_workspace_handle_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        bitflags::bitflags! { # [doc = ""] # [doc = "The different states that a workspace can have."] # [doc = ""] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct State : u32 { # [doc = "the workspace is active"] const Active = 1u32 ; # [doc = "the workspace requests attention"] const Urgent = 2u32 ; const Hidden = 4u32 ; } }
        impl TryFrom<u32> for State {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct WorkspaceCapabilities : u32 { # [doc = "activate request is available"] const Activate = 1u32 ; # [doc = "deactivate request is available"] const Deactivate = 2u32 ; # [doc = "remove request is available"] const Remove = 4u32 ; # [doc = "assign request is available"] const Assign = 8u32 ; } }
        impl TryFrom<u32> for WorkspaceCapabilities {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
            }
        }
        impl std::fmt::Display for WorkspaceCapabilities {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the ext_workspace_handle_v1 interface. See the module level documentation for more info"]
        pub trait ExtWorkspaceHandleV1 {
            const INTERFACE: &'static str = "ext_workspace_handle_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let id = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("ext_workspace_handle_v1#{}.id(\"{}\")", sender_id, id);
                        self.id(client, sender_id, id).await
                    }
                    1u16 => {
                        let name = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!("ext_workspace_handle_v1#{}.name(\"{}\")", sender_id, name);
                        self.name(client, sender_id, name).await
                    }
                    2u16 => {
                        let coordinates = message.array()?;
                        tracing::debug!(
                            "ext_workspace_handle_v1#{}.coordinates(array[{}])",
                            sender_id,
                            coordinates.len()
                        );
                        self.coordinates(client, sender_id, coordinates).await
                    }
                    3u16 => {
                        let state = message.uint()?;
                        tracing::debug!("ext_workspace_handle_v1#{}.state({})", sender_id, state);
                        self.state(client, sender_id, state.try_into()?).await
                    }
                    4u16 => {
                        let capabilities = message.uint()?;
                        tracing::debug!(
                            "ext_workspace_handle_v1#{}.capabilities({})",
                            sender_id,
                            capabilities
                        );
                        self.capabilities(client, sender_id, capabilities.try_into()?)
                            .await
                    }
                    5u16 => {
                        tracing::debug!("ext_workspace_handle_v1#{}.removed()", sender_id,);
                        self.removed(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the ext_workspace_handle_v1 object."]
            #[doc = ""]
            #[doc = "This request should be made either when the client does not want to"]
            #[doc = "use the workspace object any more or after the remove event to finalize"]
            #[doc = "the destruction of the object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_handle_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Request that this workspace be activated."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will be actually activated, and"]
            #[doc = "behaviour may be compositor-dependent. For example, activating a"]
            #[doc = "workspace may or may not deactivate all other workspaces in the same"]
            #[doc = "group."]
            #[doc = ""]
            async fn activate(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_handle_v1#{}.activate()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Request that this workspace be deactivated."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will be actually deactivated."]
            #[doc = ""]
            async fn deactivate(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_handle_v1#{}.deactivate()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Requests that this workspace is assigned to the given workspace group."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will be assigned."]
            #[doc = ""]
            async fn assign(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                workspace_group: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_handle_v1#{}.assign()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(workspace_group))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Request that this workspace be removed."]
            #[doc = ""]
            #[doc = "There is no guarantee the workspace will be actually removed."]
            #[doc = ""]
            async fn remove(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> ext_workspace_handle_v1#{}.remove()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "If this event is emitted, it will be send immediately after the"]
            #[doc = "ext_workspace_handle_v1 is created or when an id is assigned to"]
            #[doc = "a workspace (at most once during it's lifetime)."]
            #[doc = ""]
            #[doc = "An id will never change during the lifetime of the `ext_workspace_handle_v1`"]
            #[doc = "and is guaranteed to be unique during it's lifetime."]
            #[doc = ""]
            #[doc = "Ids are not human-readable and shouldn't be displayed, use `name` for that purpose."]
            #[doc = ""]
            #[doc = "Compositors are expected to only send ids for workspaces likely stable across multiple"]
            #[doc = "sessions and can be used by clients to store preferences for workspaces. Workspaces without"]
            #[doc = "ids should be considered temporary and any data associated with them should be deleted once"]
            #[doc = "the respective object is lost."]
            #[doc = ""]
            async fn id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted immediately after the ext_workspace_handle_v1 is"]
            #[doc = "created and whenever the name of the workspace changes."]
            #[doc = ""]
            #[doc = "A name is meant to be human-readable and can be displayed to a user."]
            #[doc = "Unlike the id it is neither stable nor unique."]
            #[doc = ""]
            async fn name(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is used to organize workspaces into an N-dimensional grid"]
            #[doc = "within a workspace group, and if supported, is emitted immediately after"]
            #[doc = "the ext_workspace_handle_v1 is created and whenever the coordinates of"]
            #[doc = "the workspace change. Compositors may not send this event if they do not"]
            #[doc = "conceptually arrange workspaces in this way. If compositors simply"]
            #[doc = "number workspaces, without any geometric interpretation, they may send"]
            #[doc = "1D coordinates, which clients should not interpret as implying any"]
            #[doc = "geometry. Sending an empty array means that the compositor no longer"]
            #[doc = "orders the workspace geometrically."]
            #[doc = ""]
            #[doc = "Coordinates have an arbitrary number of dimensions N with an uint32"]
            #[doc = "position along each dimension. By convention if N > 1, the first"]
            #[doc = "dimension is X, the second Y, the third Z, and so on. The compositor may"]
            #[doc = "chose to utilize these events for a more novel workspace layout"]
            #[doc = "convention, however. No guarantee is made about the grid being filled or"]
            #[doc = "bounded; there may be a workspace at coordinate 1 and another at"]
            #[doc = "coordinate 1000 and none in between. Within a workspace group, however,"]
            #[doc = "workspaces must have unique coordinates of equal dimensionality."]
            #[doc = ""]
            async fn coordinates(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                coordinates: Vec<u8>,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is emitted immediately after the ext_workspace_handle_v1 is"]
            #[doc = "created and each time the workspace state changes, either because of a"]
            #[doc = "compositor action or because of a request in this protocol."]
            #[doc = ""]
            #[doc = "Missing states convey the opposite meaning, e.g. an unset active bit"]
            #[doc = "means the workspace is currently inactive."]
            #[doc = ""]
            async fn state(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                state: State,
            ) -> crate::client::Result<()>;
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
            #[doc = "Compositors must send this event once after creation of an"]
            #[doc = "ext_workspace_handle_v1 . When the capabilities change, compositors"]
            #[doc = "must send this event again."]
            #[doc = ""]
            async fn capabilities(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                capabilities: WorkspaceCapabilities,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is send when the workspace associated with the ext_workspace_handle_v1"]
            #[doc = "has been removed. After sending this request, the compositor will immediately consider"]
            #[doc = "the object inert. Any requests will be ignored except the destroy request."]
            #[doc = ""]
            #[doc = "It is guaranteed there won't be any more events referencing this"]
            #[doc = "ext_workspace_handle_v1."]
            #[doc = ""]
            #[doc = "The compositor must only remove a workspaces not currently belonging to any"]
            #[doc = "workspace_group."]
            #[doc = ""]
            async fn removed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod fifo_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_fifo_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "These fatal protocol errors may be emitted in response to"]
        #[doc = "illegal requests."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "fifo manager already exists for surface"]
            AlreadyExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::AlreadyExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_fifo_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpFifoManagerV1 {
            const INTERFACE: &'static str = "wp_fifo_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object. Existing objects created by this object"]
            #[doc = "are not affected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fifo_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Establish a fifo object for a surface that may be used to add"]
            #[doc = "display refresh constraints to content updates."]
            #[doc = ""]
            #[doc = "Only one such object may exist for a surface and attempting"]
            #[doc = "to create more than one will result in an already_exists"]
            #[doc = "protocol error. If a surface is acted on by multiple software"]
            #[doc = "components, general best practice is that only the component"]
            #[doc = "performing wl_surface.attach operations should use this protocol."]
            #[doc = ""]
            async fn get_fifo(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fifo_manager_v1#{}.get_fifo()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "A fifo object for a surface that may be used to add"]
    #[doc = "display refresh constraints to content updates."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_fifo_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "These fatal protocol errors may be emitted in response to"]
        #[doc = "illegal requests."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the associated surface no longer exists"]
            SurfaceDestroyed = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SurfaceDestroyed),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_fifo_v1 interface. See the module level documentation for more info"]
        pub trait WpFifoV1 {
            const INTERFACE: &'static str = "wp_fifo_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "When the content update containing the \"set_barrier\" is applied,"]
            #[doc = "it sets a \"fifo_barrier\" condition on the surface associated with"]
            #[doc = "the fifo object. The condition is cleared immediately after the"]
            #[doc = "following latching deadline for non-tearing presentation."]
            #[doc = ""]
            #[doc = "The compositor may clear the condition early if it must do so to"]
            #[doc = "ensure client forward progress assumptions."]
            #[doc = ""]
            #[doc = "To wait for this condition to clear, use the \"wait_barrier\" request."]
            #[doc = ""]
            #[doc = "\"set_barrier\" is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "Requesting set_barrier after the fifo object's surface is"]
            #[doc = "destroyed will generate a \"surface_destroyed\" error."]
            #[doc = ""]
            async fn set_barrier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fifo_v1#{}.set_barrier()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Indicate that this content update is not ready while a"]
            #[doc = "\"fifo_barrier\" condition is present on the surface."]
            #[doc = ""]
            #[doc = "This means that when the content update containing \"set_barrier\""]
            #[doc = "was made active at a latching deadline, it will be active for"]
            #[doc = "at least one refresh cycle. A content update which is allowed to"]
            #[doc = "tear might become active after a latching deadline if no content"]
            #[doc = "update became active at the deadline."]
            #[doc = ""]
            #[doc = "The constraint must be ignored if the surface is a subsurface in"]
            #[doc = "synchronized mode. If the surface is not being updated by the"]
            #[doc = "compositor (off-screen, occluded) the compositor may ignore the"]
            #[doc = "constraint. Clients must use an additional mechanism such as"]
            #[doc = "frame callbacks or timestamps to ensure throttling occurs under"]
            #[doc = "all conditions."]
            #[doc = ""]
            #[doc = "\"wait_barrier\" is double-buffered state, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "Requesting \"wait_barrier\" after the fifo object's surface is"]
            #[doc = "destroyed will generate a \"surface_destroyed\" error."]
            #[doc = ""]
            async fn wait_barrier(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fifo_v1#{}.wait_barrier()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will no longer be using"]
            #[doc = "this protocol object."]
            #[doc = ""]
            #[doc = "Surface state changes previously made by this protocol are"]
            #[doc = "unaffected by this object's destruction."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fifo_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod fractional_scale_v1 {
    #[doc = ""]
    #[doc = "A global interface for requesting surfaces to use fractional scales."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_fractional_scale_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_fractional_scale_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpFractionalScaleManagerV1 {
            const INTERFACE: &'static str = "wp_fractional_scale_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will not be using this protocol"]
            #[doc = "object anymore. This does not affect any other objects,"]
            #[doc = "wp_fractional_scale_v1 objects included."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fractional_scale_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create an add-on object for the the wl_surface to let the compositor"]
            #[doc = "request fractional scales. If the given wl_surface already has a"]
            #[doc = "wp_fractional_scale_v1 object associated, the fractional_scale_exists"]
            #[doc = "protocol error is raised."]
            #[doc = ""]
            async fn get_fractional_scale(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_fractional_scale_manager_v1#{}.get_fractional_scale()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "An additional interface to a wl_surface object which allows the compositor"]
    #[doc = "to inform the client of the preferred scale."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_fractional_scale_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_fractional_scale_v1 interface. See the module level documentation for more info"]
        pub trait WpFractionalScaleV1 {
            const INTERFACE: &'static str = "wp_fractional_scale_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let scale = message.uint()?;
                        tracing::debug!(
                            "wp_fractional_scale_v1#{}.preferred_scale({})",
                            sender_id,
                            scale
                        );
                        self.preferred_scale(client, sender_id, scale).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the fractional scale object. When this object is destroyed,"]
            #[doc = "preferred_scale events will no longer be sent."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_fractional_scale_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Notification of a new preferred scale for this surface that the"]
            #[doc = "compositor suggests that the client should use."]
            #[doc = ""]
            #[doc = "The sent scale is the numerator of a fraction with a denominator of 120."]
            #[doc = ""]
            async fn preferred_scale(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                scale: u32,
            ) -> crate::client::Result<()>;
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod linux_drm_syncobj_v1 {
    #[doc = ""]
    #[doc = "This global is a factory interface, allowing clients to request"]
    #[doc = "explicit synchronization for buffers on a per-surface basis."]
    #[doc = ""]
    #[doc = "See wp_linux_drm_syncobj_surface_v1 for more information."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_linux_drm_syncobj_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_linux_drm_syncobj_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjManagerV1 {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this explicit synchronization factory object. Other objects"]
            #[doc = "shall not be affected by this request."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_linux_drm_syncobj_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Instantiate an interface extension for the given wl_surface to provide"]
            #[doc = "explicit synchronization."]
            #[doc = ""]
            #[doc = "If the given wl_surface already has an explicit synchronization object"]
            #[doc = "associated, the surface_exists protocol error is raised."]
            #[doc = ""]
            #[doc = "Graphics APIs, like EGL or Vulkan, that manage the buffer queue and"]
            #[doc = "commits of a wl_surface themselves, are likely to be using this"]
            #[doc = "extension internally. If a client is using such an API for a"]
            #[doc = "wl_surface, it should not directly use this extension on that surface,"]
            #[doc = "to avoid raising a surface_exists protocol error."]
            #[doc = ""]
            async fn get_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_linux_drm_syncobj_manager_v1#{}.get_surface()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Import a DRM synchronization object timeline."]
            #[doc = ""]
            #[doc = "If the FD cannot be imported, the invalid_timeline error is raised."]
            #[doc = ""]
            async fn import_timeline(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_linux_drm_syncobj_manager_v1#{}.import_timeline()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_fd(fd)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "This object represents an explicit synchronization object timeline"]
    #[doc = "imported by the client to the compositor."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_linux_drm_syncobj_timeline_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_linux_drm_syncobj_timeline_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjTimelineV1 {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_timeline_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the synchronization object timeline. Other objects are not"]
            #[doc = "affected by this request, in particular timeline points set by"]
            #[doc = "set_acquire_point and set_release_point are not unset."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_linux_drm_syncobj_timeline_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_linux_drm_syncobj_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_linux_drm_syncobj_surface_v1 interface. See the module level documentation for more info"]
        pub trait WpLinuxDrmSyncobjSurfaceV1 {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this surface synchronization object."]
            #[doc = ""]
            #[doc = "Any timeline point set by this object with set_acquire_point or"]
            #[doc = "set_release_point since the last commit may be discarded by the"]
            #[doc = "compositor. Any timeline point set by this object before the last"]
            #[doc = "commit will not be affected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_linux_drm_syncobj_surface_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Set the timeline point that must be signalled before the compositor may"]
            #[doc = "sample from the buffer attached with wl_surface.attach."]
            #[doc = ""]
            #[doc = "The 64-bit unsigned value combined from point_hi and point_lo is the"]
            #[doc = "point value."]
            #[doc = ""]
            #[doc = "The acquire point is double-buffered state, and will be applied on the"]
            #[doc = "next wl_surface.commit request for the associated surface. Thus, it"]
            #[doc = "applies only to the buffer that is attached to the surface at commit"]
            #[doc = "time."]
            #[doc = ""]
            #[doc = "If an acquire point has already been attached during the same commit"]
            #[doc = "cycle, the new point replaces the old one."]
            #[doc = ""]
            #[doc = "If the associated wl_surface was destroyed, a no_surface error is"]
            #[doc = "raised."]
            #[doc = ""]
            #[doc = "If at surface commit time there is a pending acquire timeline point set"]
            #[doc = "but no pending buffer attached, a no_buffer error is raised. If at"]
            #[doc = "surface commit time there is a pending buffer attached but no pending"]
            #[doc = "acquire timeline point set, the no_acquire_point protocol error is"]
            #[doc = "raised."]
            #[doc = ""]
            async fn set_acquire_point(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                timeline: crate::wire::ObjectId,
                point_hi: u32,
                point_lo: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_linux_drm_syncobj_surface_v1#{}.set_acquire_point()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(timeline))
                    .put_uint(point_hi)
                    .put_uint(point_lo)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Set the timeline point that must be signalled by the compositor when it"]
            #[doc = "has finished its usage of the buffer attached with wl_surface.attach"]
            #[doc = "for the relevant commit."]
            #[doc = ""]
            #[doc = "Once the timeline point is signaled, and assuming the associated buffer"]
            #[doc = "is not pending release from other wl_surface.commit requests, no"]
            #[doc = "additional explicit or implicit synchronization with the compositor is"]
            #[doc = "required to safely re-use the buffer."]
            #[doc = ""]
            #[doc = "Note that clients cannot rely on the release point being always"]
            #[doc = "signaled after the acquire point: compositors may release buffers"]
            #[doc = "without ever reading from them. In addition, the compositor may use"]
            #[doc = "different presentation paths for different commits, which may have"]
            #[doc = "different release behavior. As a result, the compositor may signal the"]
            #[doc = "release points in a different order than the client committed them."]
            #[doc = ""]
            #[doc = "Because signaling a timeline point also signals every previous point,"]
            #[doc = "it is generally not safe to use the same timeline object for the"]
            #[doc = "release points of multiple buffers. The out-of-order signaling"]
            #[doc = "described above may lead to a release point being signaled before the"]
            #[doc = "compositor has finished reading. To avoid this, it is strongly"]
            #[doc = "recommended that each buffer should use a separate timeline for its"]
            #[doc = "release points."]
            #[doc = ""]
            #[doc = "The 64-bit unsigned value combined from point_hi and point_lo is the"]
            #[doc = "point value."]
            #[doc = ""]
            #[doc = "The release point is double-buffered state, and will be applied on the"]
            #[doc = "next wl_surface.commit request for the associated surface. Thus, it"]
            #[doc = "applies only to the buffer that is attached to the surface at commit"]
            #[doc = "time."]
            #[doc = ""]
            #[doc = "If a release point has already been attached during the same commit"]
            #[doc = "cycle, the new point replaces the old one."]
            #[doc = ""]
            #[doc = "If the associated wl_surface was destroyed, a no_surface error is"]
            #[doc = "raised."]
            #[doc = ""]
            #[doc = "If at surface commit time there is a pending release timeline point set"]
            #[doc = "but no pending buffer attached, a no_buffer error is raised. If at"]
            #[doc = "surface commit time there is a pending buffer attached but no pending"]
            #[doc = "release timeline point set, the no_release_point protocol error is"]
            #[doc = "raised."]
            #[doc = ""]
            async fn set_release_point(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                timeline: crate::wire::ObjectId,
                point_hi: u32,
                point_lo: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_linux_drm_syncobj_surface_v1#{}.set_release_point()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(timeline))
                    .put_uint(point_hi)
                    .put_uint(point_lo)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod pointer_warp_v1 {
    #[doc = ""]
    #[doc = "This global interface allows applications to request the pointer to be"]
    #[doc = "moved to a position relative to a wl_surface."]
    #[doc = ""]
    #[doc = "Note that if the desired behavior is to constrain the pointer to an area"]
    #[doc = "or lock it to a position, this protocol does not provide a reliable way"]
    #[doc = "to do that. The pointer constraint and pointer lock protocols should be"]
    #[doc = "used for those use cases instead."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_pointer_warp_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_pointer_warp_v1 interface. See the module level documentation for more info"]
        pub trait WpPointerWarpV1 {
            const INTERFACE: &'static str = "wp_pointer_warp_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the pointer warp manager."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_pointer_warp_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Request the compositor to move the pointer to a surface-local position."]
            #[doc = "Whether or not the compositor honors the request is implementation defined,"]
            #[doc = "but it should"]
            #[doc = "- honor it if the surface has pointer focus, including"]
            #[doc = "when it has an implicit pointer grab"]
            #[doc = "- reject it if the enter serial is incorrect"]
            #[doc = "- reject it if the requested position is outside of the surface"]
            #[doc = ""]
            #[doc = "Note that the enter serial is valid for any surface of the client,"]
            #[doc = "and does not have to be from the surface the pointer is warped to."]
            #[doc = ""]
            #[doc = ""]
            async fn warp_pointer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                pointer: crate::wire::ObjectId,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
                serial: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_pointer_warp_v1#{}.warp_pointer()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .put_object(Some(pointer))
                    .put_fixed(x)
                    .put_fixed(y)
                    .put_uint(serial)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod security_context_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_security_context_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_security_context_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpSecurityContextManagerV1 {
            const INTERFACE: &'static str = "wp_security_context_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the manager. This doesn't destroy objects created with the"]
            #[doc = "manager."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_security_context_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Creates a new security context with a socket listening FD."]
            #[doc = ""]
            #[doc = "The compositor will accept new client connections on listen_fd."]
            #[doc = "listen_fd must be ready to accept new connections when this request is"]
            #[doc = "sent by the client. In other words, the client must call bind(2) and"]
            #[doc = "listen(2) before sending the FD."]
            #[doc = ""]
            #[doc = "close_fd is a FD that will signal hangup when the compositor should stop"]
            #[doc = "accepting new connections on listen_fd."]
            #[doc = ""]
            #[doc = "The compositor must continue to accept connections on listen_fd when"]
            #[doc = "the Wayland client which created the security context disconnects."]
            #[doc = ""]
            #[doc = "After sending this request, closing listen_fd and close_fd remains the"]
            #[doc = "only valid operation on them."]
            #[doc = ""]
            async fn create_listener(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                listen_fd: rustix::fd::OwnedFd,
                close_fd: rustix::fd::OwnedFd,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_security_context_manager_v1#{}.create_listener()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_fd(listen_fd)
                    .put_fd(close_fd)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_security_context_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_security_context_v1 interface. See the module level documentation for more info"]
        pub trait WpSecurityContextV1 {
            const INTERFACE: &'static str = "wp_security_context_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the security context object."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_security_context_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Attach a unique sandbox engine name to the security context. The name"]
            #[doc = "should follow the reverse-DNS style (e.g. \"org.flatpak\")."]
            #[doc = ""]
            #[doc = "A list of well-known engines is maintained at:"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md"]
            #[doc = ""]
            #[doc = "It is a protocol error to call this request twice. The already_set"]
            #[doc = "error is sent in this case."]
            #[doc = ""]
            async fn set_sandbox_engine(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_security_context_v1#{}.set_sandbox_engine()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Attach an application ID to the security context."]
            #[doc = ""]
            #[doc = "The application ID is an opaque, sandbox-specific identifier for an"]
            #[doc = "application. See the well-known engines document for more details:"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md"]
            #[doc = ""]
            #[doc = "The compositor may use the application ID to group clients belonging to"]
            #[doc = "the same security context application."]
            #[doc = ""]
            #[doc = "Whether this request is optional or not depends on the sandbox engine used."]
            #[doc = ""]
            #[doc = "It is a protocol error to call this request twice. The already_set"]
            #[doc = "error is sent in this case."]
            #[doc = ""]
            async fn set_app_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                app_id: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_security_context_v1#{}.set_app_id()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Attach an instance ID to the security context."]
            #[doc = ""]
            #[doc = "The instance ID is an opaque, sandbox-specific identifier for a running"]
            #[doc = "instance of an application. See the well-known engines document for"]
            #[doc = "more details:"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md"]
            #[doc = ""]
            #[doc = "Whether this request is optional or not depends on the sandbox engine used."]
            #[doc = ""]
            #[doc = "It is a protocol error to call this request twice. The already_set"]
            #[doc = "error is sent in this case."]
            #[doc = ""]
            async fn set_instance_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                instance_id: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_security_context_v1#{}.set_instance_id()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(instance_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Atomically register the new client and attach the security context"]
            #[doc = "metadata."]
            #[doc = ""]
            #[doc = "If the provided metadata is inconsistent or does not match with out of"]
            #[doc = "band metadata (see"]
            #[doc = "https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md),"]
            #[doc = "the invalid_metadata error may be sent eventually."]
            #[doc = ""]
            #[doc = "It's a protocol error to send any request other than \"destroy\" after"]
            #[doc = "this request. In this case, the already_used error is sent."]
            #[doc = ""]
            async fn commit(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_security_context_v1#{}.commit()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod single_pixel_buffer_v1 {
    #[doc = ""]
    #[doc = "The wp_single_pixel_buffer_manager_v1 interface is a factory for"]
    #[doc = "single-pixel buffers."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_single_pixel_buffer_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the wp_single_pixel_buffer_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpSinglePixelBufferManagerV1 {
            const INTERFACE: &'static str = "wp_single_pixel_buffer_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the wp_single_pixel_buffer_manager_v1 object."]
            #[doc = ""]
            #[doc = "The child objects created via this interface are unaffected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_single_pixel_buffer_manager_v1#{}.destroy()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create a single-pixel buffer from four 32-bit RGBA values."]
            #[doc = ""]
            #[doc = "Unless specified in another protocol extension, the RGBA values use"]
            #[doc = "pre-multiplied alpha."]
            #[doc = ""]
            #[doc = "The width and height of the buffer are 1."]
            #[doc = ""]
            #[doc = "The r, g, b and a arguments valid range is from UINT32_MIN (0)"]
            #[doc = "to UINT32_MAX (0xffffffff)."]
            #[doc = ""]
            #[doc = "These arguments should be interpreted as a percentage, i.e."]
            #[doc = "- UINT32_MIN = 0% of the given color component"]
            #[doc = "- UINT32_MAX = 100% of the given color component"]
            #[doc = ""]
            async fn create_u32_rgba_buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                r: u32,
                g: u32,
                b: u32,
                a: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_single_pixel_buffer_manager_v1#{}.create_u32_rgba_buffer()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_uint(r)
                    .put_uint(g)
                    .put_uint(b)
                    .put_uint(a)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod tearing_control_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_tearing_control_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_tearing_control_manager_v1 interface. See the module level documentation for more info"]
        pub trait WpTearingControlManagerV1 {
            const INTERFACE: &'static str = "wp_tearing_control_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this tearing control factory object. Other objects, including"]
            #[doc = "wp_tearing_control_v1 objects created by this factory, are not affected"]
            #[doc = "by this request."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_tearing_control_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Instantiate an interface extension for the given wl_surface to request"]
            #[doc = "asynchronous page flips for presentation."]
            #[doc = ""]
            #[doc = "If the given wl_surface already has a wp_tearing_control_v1 object"]
            #[doc = "associated, the tearing_control_exists protocol error is raised."]
            #[doc = ""]
            async fn get_tearing_control(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_tearing_control_manager_v1#{}.get_tearing_control()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "An additional interface to a wl_surface object, which allows the client"]
    #[doc = "to hint to the compositor if the content on the surface is suitable for"]
    #[doc = "presentation with tearing."]
    #[doc = "The default presentation hint is vsync. See presentation_hint for more"]
    #[doc = "details."]
    #[doc = ""]
    #[doc = "If the associated wl_surface is destroyed, this object becomes inert and"]
    #[doc = "should be destroyed."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod wp_tearing_control_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = ""]
        #[doc = "This enum provides information for if submitted frames from the client"]
        #[doc = "may be presented with tearing."]
        #[doc = ""]
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
        impl std::fmt::Display for PresentationHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wp_tearing_control_v1 interface. See the module level documentation for more info"]
        pub trait WpTearingControlV1 {
            const INTERFACE: &'static str = "wp_tearing_control_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Set the presentation hint for the associated wl_surface. This state is"]
            #[doc = "double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The compositor is free to dynamically respect or ignore this hint based"]
            #[doc = "on various conditions like hardware capabilities, surface state and"]
            #[doc = "user preferences."]
            #[doc = ""]
            async fn set_presentation_hint(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                hint: PresentationHint,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> wp_tearing_control_v1#{}.set_presentation_hint()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(hint as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroy this surface tearing object and revert the presentation hint to"]
            #[doc = "vsync. The change will be applied on the next wl_surface.commit."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> wp_tearing_control_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod xdg_activation_v1 {
    #[doc = ""]
    #[doc = "A global interface used for informing the compositor about applications"]
    #[doc = "being activated or started, or for applications to request to be"]
    #[doc = "activated."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_activation_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xdg_activation_v1 interface. See the module level documentation for more info"]
        pub trait XdgActivationV1 {
            const INTERFACE: &'static str = "xdg_activation_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Notify the compositor that the xdg_activation object will no longer be"]
            #[doc = "used."]
            #[doc = ""]
            #[doc = "The child objects created via this interface are unaffected and should"]
            #[doc = "be destroyed separately."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Creates an xdg_activation_token_v1 object that will provide"]
            #[doc = "the initiating client with a unique token for this activation. This"]
            #[doc = "token should be offered to the clients to be activated."]
            #[doc = ""]
            async fn get_activation_token(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_v1#{}.get_activation_token()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Requests surface activation. It's up to the compositor to display"]
            #[doc = "this information as desired, for example by placing the surface above"]
            #[doc = "the rest."]
            #[doc = ""]
            #[doc = "The compositor may know who requested this by checking the activation"]
            #[doc = "token and might decide not to follow through with the activation if it's"]
            #[doc = "considered unwanted."]
            #[doc = ""]
            #[doc = "Compositors can ignore unknown activation tokens when an invalid"]
            #[doc = "token is passed."]
            #[doc = ""]
            async fn activate(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                token: String,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_v1#{}.activate()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(token))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "An object for setting up a token and receiving a token handle that can"]
    #[doc = "be passed as an activation token to another client."]
    #[doc = ""]
    #[doc = "The object is created using the xdg_activation_v1.get_activation_token"]
    #[doc = "request. This object should then be populated with the app_id, surface"]
    #[doc = "and serial information and committed. The compositor shall then issue a"]
    #[doc = "done event with the token. In case the request's parameters are invalid,"]
    #[doc = "the compositor will provide an invalid token."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_activation_token_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_activation_token_v1 interface. See the module level documentation for more info"]
        pub trait XdgActivationTokenV1 {
            const INTERFACE: &'static str = "xdg_activation_token_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let token = message
                            .string()?
                            .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                        tracing::debug!(
                            "xdg_activation_token_v1#{}.done(\"{}\")",
                            sender_id,
                            token
                        );
                        self.done(client, sender_id, token).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Provides information about the seat and serial event that requested the"]
            #[doc = "token."]
            #[doc = ""]
            #[doc = "The serial can come from an input or focus event. For instance, if a"]
            #[doc = "click triggers the launch of a third-party client, the launcher client"]
            #[doc = "should send a set_serial request with the serial and seat from the"]
            #[doc = "wl_pointer.button event."]
            #[doc = ""]
            #[doc = "Some compositors might refuse to activate toplevels when the token"]
            #[doc = "doesn't have a valid and recent enough event serial."]
            #[doc = ""]
            #[doc = "Must be sent before commit. This information is optional."]
            #[doc = ""]
            async fn set_serial(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial: u32,
                seat: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_token_v1#{}.set_serial()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(seat))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The requesting client can specify an app_id to associate the token"]
            #[doc = "being created with it."]
            #[doc = ""]
            #[doc = "Must be sent before commit. This information is optional."]
            #[doc = ""]
            async fn set_app_id(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                app_id: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_token_v1#{}.set_app_id()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(app_id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request sets the surface requesting the activation. Note, this is"]
            #[doc = "different from the surface that will be activated."]
            #[doc = ""]
            #[doc = "Some compositors might refuse to activate toplevels when the token"]
            #[doc = "doesn't have a requesting surface."]
            #[doc = ""]
            #[doc = "Must be sent before commit. This information is optional."]
            #[doc = ""]
            async fn set_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_token_v1#{}.set_surface()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Requests an activation token based on the different parameters that"]
            #[doc = "have been offered through set_serial, set_surface and set_app_id."]
            #[doc = ""]
            async fn commit(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_token_v1#{}.commit()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Notify the compositor that the xdg_activation_token_v1 object will no"]
            #[doc = "longer be used. The received token stays valid."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_activation_token_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "The 'done' event contains the unique token of this activation request"]
            #[doc = "and notifies that the provider is done."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                token: String,
            ) -> crate::client::Result<()>;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_dialog_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_wm_dialog_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_wm_dialog_v1 interface. See the module level documentation for more info"]
        pub trait XdgWmDialogV1 {
            const INTERFACE: &'static str = "xdg_wm_dialog_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the xdg_wm_dialog_v1 object. This does not affect"]
            #[doc = "the xdg_dialog_v1 objects generated through it."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_wm_dialog_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Creates a xdg_dialog_v1 object for the given toplevel. See the interface"]
            #[doc = "description for more details."]
            #[doc = ""]
            #[doc = "Compositors must raise an already_used error if clients attempt to"]
            #[doc = "create multiple xdg_dialog_v1 objects for the same xdg_toplevel."]
            #[doc = ""]
            async fn get_xdg_dialog(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_wm_dialog_v1#{}.get_xdg_dialog()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(toplevel))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "A xdg_dialog_v1 object is an ancillary object tied to a xdg_toplevel. Its"]
    #[doc = "purpose is hinting the compositor that the toplevel is a \"dialog\" (e.g. a"]
    #[doc = "temporary window) relative to another toplevel (see"]
    #[doc = "xdg_toplevel.set_parent). If the xdg_toplevel is destroyed, the xdg_dialog_v1"]
    #[doc = "becomes inert."]
    #[doc = ""]
    #[doc = "Through this object, the client may provide additional hints about"]
    #[doc = "the purpose of the secondary toplevel. This interface has no effect"]
    #[doc = "on toplevels that are not attached to a parent toplevel."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_dialog_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xdg_dialog_v1 interface. See the module level documentation for more info"]
        pub trait XdgDialogV1 {
            const INTERFACE: &'static str = "xdg_dialog_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the xdg_dialog_v1 object. If this object is destroyed"]
            #[doc = "before the related xdg_toplevel, the compositor should unapply its"]
            #[doc = "effects."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_dialog_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Hints that the dialog has \"modal\" behavior. Modal dialogs typically"]
            #[doc = "require to be fully addressed by the user (i.e. closed) before resuming"]
            #[doc = "interaction with the parent toplevel, and may require a distinct"]
            #[doc = "presentation."]
            #[doc = ""]
            #[doc = "Clients must implement the logic to filter events in the parent"]
            #[doc = "toplevel on their own."]
            #[doc = ""]
            #[doc = "Compositors may choose any policy in event delivery to the parent"]
            #[doc = "toplevel, from delivering all events unfiltered to using them for"]
            #[doc = "internal consumption."]
            #[doc = ""]
            async fn set_modal(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_dialog_v1#{}.set_modal()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Drops the hint that this dialog has \"modal\" behavior. See"]
            #[doc = "xdg_dialog_v1.set_modal for more details."]
            #[doc = ""]
            async fn unset_modal(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_dialog_v1#{}.unset_modal()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_system_bell_v1 {
    #[doc = ""]
    #[doc = "This global interface enables clients to ring the system bell."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_system_bell_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xdg_system_bell_v1 interface. See the module level documentation for more info"]
        pub trait XdgSystemBellV1 {
            const INTERFACE: &'static str = "xdg_system_bell_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Notify that the object will no longer be used."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_system_bell_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This requests rings the system bell on behalf of a client. How ringing"]
            #[doc = "the bell is implemented is up to the compositor. It may be an audible"]
            #[doc = "sound, a visual feedback of some kind, or any other thing including"]
            #[doc = "nothing."]
            #[doc = ""]
            #[doc = "The passed surface should correspond to a toplevel like surface role,"]
            #[doc = "or be null, meaning the client doesn't have a particular toplevel it"]
            #[doc = "wants to associate the bell ringing with. See the xdg-shell protocol"]
            #[doc = "extension for a toplevel like surface role."]
            #[doc = ""]
            async fn ring(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_system_bell_v1#{}.ring()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(surface)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_toplevel_drag_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_toplevel_drag_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_drag_manager_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelDragManagerV1 {
            const INTERFACE: &'static str = "xdg_toplevel_drag_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this xdg_toplevel_drag_manager_v1 object. Other objects,"]
            #[doc = "including xdg_toplevel_drag_v1 objects created by this factory, are not"]
            #[doc = "affected by this request."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_drag_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create an xdg_toplevel_drag for a drag and drop operation that is going"]
            #[doc = "to be started with data_source."]
            #[doc = ""]
            #[doc = "This request can only be made on sources used in drag-and-drop, so it"]
            #[doc = "must be performed before wl_data_device.start_drag. Attempting to use"]
            #[doc = "the source other than for drag-and-drop such as in"]
            #[doc = "wl_data_device.set_selection will raise an invalid_source error."]
            #[doc = ""]
            #[doc = "Destroying data_source while a toplevel is attached to the"]
            #[doc = "xdg_toplevel_drag is undefined."]
            #[doc = ""]
            async fn get_xdg_toplevel_drag(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                data_source: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> xdg_toplevel_drag_manager_v1#{}.get_xdg_toplevel_drag()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(data_source))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_toplevel_drag_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_drag_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelDragV1 {
            const INTERFACE: &'static str = "xdg_toplevel_drag_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this xdg_toplevel_drag_v1 object. This request must only be"]
            #[doc = "called after the underlying wl_data_source drag has ended, as indicated"]
            #[doc = "by the dnd_drop_performed or cancelled events. In any other case an"]
            #[doc = "ongoing_drag error is raised."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_drag_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Request that the window will be moved with the cursor during the drag"]
            #[doc = "operation. The offset is a hint to the compositor how the toplevel"]
            #[doc = "should be positioned relative to the cursor hotspot in surface local"]
            #[doc = "coordinates and relative to the geometry of the toplevel being attached."]
            #[doc = "See xdg_surface.set_window_geometry. For example it might only"]
            #[doc = "be used when an unmapped window is attached. The attached window"]
            #[doc = "does not participate in the selection of the drag target."]
            #[doc = ""]
            #[doc = "If the toplevel is unmapped while it is attached, it is automatically"]
            #[doc = "detached from the drag. In this case this request has to be called again"]
            #[doc = "if the window should be attached after it is remapped."]
            #[doc = ""]
            #[doc = "This request can be called multiple times but issuing it while a"]
            #[doc = "toplevel with an active role is attached raises a toplevel_attached"]
            #[doc = "error."]
            #[doc = ""]
            async fn attach(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                x_offset: i32,
                y_offset: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_drag_v1#{}.attach()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_int(x_offset)
                    .put_int(y_offset)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
#[doc = "This protocol allows clients to set icons for their toplevel surfaces"]
#[doc = "either via the XDG icon stock (using an icon name), or from pixel data."]
#[doc = ""]
#[doc = "A toplevel icon represents the individual toplevel (unlike the application"]
#[doc = "or launcher icon, which represents the application as a whole), and may be"]
#[doc = "shown in window switchers, window overviews and taskbars that list"]
#[doc = "individual windows."]
#[doc = ""]
#[doc = "This document adheres to RFC 2119 when using words like \"must\","]
#[doc = "\"should\", \"may\", etc."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the testing"]
#[doc = "phase. Backward compatible changes may be added together with the"]
#[doc = "corresponding interface version bump. Backward incompatible changes can"]
#[doc = "only be done by creating a new major version of the extension."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod xdg_toplevel_icon_v1 {
    #[doc = ""]
    #[doc = "This interface allows clients to create toplevel window icons and set"]
    #[doc = "them on toplevel windows to be displayed to the user."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_toplevel_icon_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xdg_toplevel_icon_manager_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelIconManagerV1 {
            const INTERFACE: &'static str = "xdg_toplevel_icon_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    0u16 => {
                        let size = message.int()?;
                        tracing::debug!(
                            "xdg_toplevel_icon_manager_v1#{}.icon_size({})",
                            sender_id,
                            size
                        );
                        self.icon_size(client, sender_id, size).await
                    }
                    1u16 => {
                        tracing::debug!("xdg_toplevel_icon_manager_v1#{}.done()", sender_id,);
                        self.done(client, sender_id).await
                    }
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the toplevel icon manager."]
            #[doc = "This does not destroy objects created with the manager."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Creates a new icon object. This icon can then be attached to a"]
            #[doc = "xdg_toplevel via the 'set_icon' request."]
            #[doc = ""]
            async fn create_icon(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> xdg_toplevel_icon_manager_v1#{}.create_icon()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request assigns the icon 'icon' to 'toplevel', or clears the"]
            #[doc = "toplevel icon if 'icon' was null."]
            #[doc = "This state is double-buffered and is applied on the next"]
            #[doc = "wl_surface.commit of the toplevel."]
            #[doc = ""]
            #[doc = "After making this call, the xdg_toplevel_icon_v1 provided as 'icon'"]
            #[doc = "can be destroyed by the client without 'toplevel' losing its icon."]
            #[doc = "The xdg_toplevel_icon_v1 is immutable from this point, and any"]
            #[doc = "future attempts to change it must raise the"]
            #[doc = "'xdg_toplevel_icon_v1.immutable' protocol error."]
            #[doc = ""]
            #[doc = "The compositor must set the toplevel icon from either the pixel data"]
            #[doc = "the icon provides, or by loading a stock icon using the icon name."]
            #[doc = "See the description of 'xdg_toplevel_icon_v1' for details."]
            #[doc = ""]
            #[doc = "If 'icon' is set to null, the icon of the respective toplevel is reset"]
            #[doc = "to its default icon (usually the icon of the application, derived from"]
            #[doc = "its desktop-entry file, or a placeholder icon)."]
            #[doc = "If this request is passed an icon with no pixel buffers or icon name"]
            #[doc = "assigned, the icon must be reset just like if 'icon' was null."]
            #[doc = ""]
            async fn set_icon(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                icon: Option<crate::wire::ObjectId>,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_manager_v1#{}.set_icon()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_object(icon)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This event indicates an icon size the compositor prefers to be"]
            #[doc = "available if the client has scalable icons and can render to any size."]
            #[doc = ""]
            #[doc = "When the 'xdg_toplevel_icon_manager_v1' object is created, the"]
            #[doc = "compositor may send one or more 'icon_size' events to describe the list"]
            #[doc = "of preferred icon sizes. If the compositor has no size preference, it"]
            #[doc = "may not send any 'icon_size' event, and it is up to the client to"]
            #[doc = "decide a suitable icon size."]
            #[doc = ""]
            #[doc = "A sequence of 'icon_size' events must be finished with a 'done' event."]
            #[doc = "If the compositor has no size preferences, it must still send the"]
            #[doc = "'done' event, without any preceding 'icon_size' events."]
            #[doc = ""]
            async fn icon_size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                size: i32,
            ) -> crate::client::Result<()>;
            #[doc = ""]
            #[doc = "This event is sent after all 'icon_size' events have been sent."]
            #[doc = ""]
            async fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()>;
        }
    }
    #[doc = ""]
    #[doc = "This interface defines a toplevel icon."]
    #[doc = "An icon can have a name, and multiple buffers."]
    #[doc = "In order to be applied, the icon must have either a name, or at least"]
    #[doc = "one buffer assigned. Applying an empty icon (with no buffer or name) to"]
    #[doc = "a toplevel should reset its icon to the default icon."]
    #[doc = ""]
    #[doc = "It is up to compositor policy whether to prefer using a buffer or loading"]
    #[doc = "an icon via its name. See 'set_name' and 'add_buffer' for details."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_toplevel_icon_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the provided buffer does not satisfy requirements"]
            InvalidBuffer = 1u32,
            #[doc = "the icon has already been assigned to a toplevel and must not be changed"]
            Immutable = 2u32,
            #[doc = "the provided buffer has been destroyed before the toplevel icon"]
            NoBuffer = 3u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::InvalidBuffer),
                    2u32 => Ok(Self::Immutable),
                    3u32 => Ok(Self::NoBuffer),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xdg_toplevel_icon_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelIconV1 {
            const INTERFACE: &'static str = "xdg_toplevel_icon_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroys the 'xdg_toplevel_icon_v1' object."]
            #[doc = "The icon must still remain set on every toplevel it was assigned to,"]
            #[doc = "until the toplevel icon is reset explicitly."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request assigns an icon name to this icon."]
            #[doc = "Any previously set name is overridden."]
            #[doc = ""]
            #[doc = "The compositor must resolve 'icon_name' according to the lookup rules"]
            #[doc = "described in the XDG icon theme specification[1] using the"]
            #[doc = "environment's current icon theme."]
            #[doc = ""]
            #[doc = "If the compositor does not support icon names or cannot resolve"]
            #[doc = "'icon_name' according to the XDG icon theme specification it must"]
            #[doc = "fall back to using pixel buffer data instead."]
            #[doc = ""]
            #[doc = "If this request is made after the icon has been assigned to a toplevel"]
            #[doc = "via 'set_icon', a 'immutable' error must be raised."]
            #[doc = ""]
            #[doc = "[1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html"]
            #[doc = ""]
            async fn set_name(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                icon_name: String,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_v1#{}.set_name()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(icon_name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "This request adds pixel data supplied as wl_buffer to the icon."]
            #[doc = ""]
            #[doc = "The client should add pixel data for all icon sizes and scales that"]
            #[doc = "it can provide, or which are explicitly requested by the compositor"]
            #[doc = "via 'icon_size' events on xdg_toplevel_icon_manager_v1."]
            #[doc = ""]
            #[doc = "The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm"]
            #[doc = "and must be a square (width and height being equal)."]
            #[doc = "If any of these buffer requirements are not fulfilled, a 'invalid_buffer'"]
            #[doc = "error must be raised."]
            #[doc = ""]
            #[doc = "If this icon instance already has a buffer of the same size and scale"]
            #[doc = "from a previous 'add_buffer' request, data from the last request"]
            #[doc = "overrides the preexisting pixel data."]
            #[doc = ""]
            #[doc = "The wl_buffer must be kept alive for as long as the xdg_toplevel_icon"]
            #[doc = "it is associated with is not destroyed, otherwise a 'no_buffer' error"]
            #[doc = "is raised. The buffer contents must not be modified after it was"]
            #[doc = "assigned to the icon. As a result, the region of the wl_shm_pool's"]
            #[doc = "backing storage used for the wl_buffer must not be modified after this"]
            #[doc = "request is sent. The wl_buffer.release event is unused."]
            #[doc = ""]
            #[doc = "If this request is made after the icon has been assigned to a toplevel"]
            #[doc = "via 'set_icon', a 'immutable' error must be raised."]
            #[doc = ""]
            async fn add_buffer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
                scale: i32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_icon_v1#{}.add_buffer()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .put_int(scale)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod xdg_toplevel_tag_v1 {
    #[doc = ""]
    #[doc = "In order to make some window properties like position, size,"]
    #[doc = "\"always on top\" or user defined rules for window behavior persistent, the"]
    #[doc = "compositor needs some way to identify windows even after the application"]
    #[doc = "has been restarted."]
    #[doc = "This protocol allows clients to make this possible by setting a tag for"]
    #[doc = "toplevels."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is currently in the testing"]
    #[doc = "phase. Backward compatible changes may be added together with the"]
    #[doc = "corresponding interface version bump. Backward incompatible changes can"]
    #[doc = "only be done by creating a new major version of the extension."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xdg_toplevel_tag_manager_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xdg_toplevel_tag_manager_v1 interface. See the module level documentation for more info"]
        pub trait XdgToplevelTagManagerV1 {
            const INTERFACE: &'static str = "xdg_toplevel_tag_manager_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy this toplevel tag manager object. This request has no other"]
            #[doc = "effects."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xdg_toplevel_tag_manager_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Set a tag for a toplevel. The tag may be shown to the user in UI, so"]
            #[doc = "it's preferable for it to be human readable, but it must be suitable"]
            #[doc = "for configuration files and should not be translated."]
            #[doc = "Suitable tags would for example be \"main window\", \"settings\","]
            #[doc = "\"e-mail composer\" or similar."]
            #[doc = ""]
            #[doc = "The tag does not need to be unique across applications, and the client"]
            #[doc = "may set the same tag for multiple windows, for example if the user has"]
            #[doc = "opened the same UI twice. How the potentially resulting conflicts are"]
            #[doc = "handled is compositor policy."]
            #[doc = ""]
            #[doc = "The client should set the tag as part of the initial commit on the"]
            #[doc = "associated toplevel, but it may set it at any time afterwards as well,"]
            #[doc = "for example if the purpose of the toplevel changes."]
            #[doc = ""]
            async fn set_toplevel_tag(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                tag: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> xdg_toplevel_tag_manager_v1#{}.set_toplevel_tag()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_string(Some(tag))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Set a description for a toplevel. This description may be shown to the"]
            #[doc = "user in UI or read by a screen reader for accessibility purposes, and"]
            #[doc = "should be translated."]
            #[doc = "It is recommended to make the description the translation of the tag."]
            #[doc = ""]
            #[doc = "The client should set the description as part of the initial commit on"]
            #[doc = "the associated toplevel, but it may set it at any time afterwards as"]
            #[doc = "well, for example if the purpose of the toplevel changes."]
            #[doc = ""]
            async fn set_toplevel_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                toplevel: crate::wire::ObjectId,
                description: String,
            ) -> crate::client::Result<()> {
                tracing::debug!(
                    "-> xdg_toplevel_tag_manager_v1#{}.set_toplevel_description()",
                    sender_id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(toplevel))
                    .put_string(Some(description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
#[doc = ""]
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
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod xwayland_shell_v1 {
    #[doc = ""]
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
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xwayland_shell_v1 {
        use futures_util::SinkExt;
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
        #[doc = "Trait to implement the xwayland_shell_v1 interface. See the module level documentation for more info"]
        pub trait XwaylandShellV1 {
            const INTERFACE: &'static str = "xwayland_shell_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Destroy the xwayland_shell_v1 object."]
            #[doc = ""]
            #[doc = "The child objects created via this interface are unaffected."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xwayland_shell_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Create an xwayland_surface_v1 interface for a given wl_surface"]
            #[doc = "object and gives it the xwayland_surface role."]
            #[doc = ""]
            #[doc = "It is illegal to create an xwayland_surface_v1 for a wl_surface"]
            #[doc = "which already has an assigned role and this will result in the"]
            #[doc = "`role` protocol error."]
            #[doc = ""]
            #[doc = "See the documentation of xwayland_surface_v1 for more details"]
            #[doc = "about what an xwayland_surface_v1 is and how it is used."]
            #[doc = ""]
            async fn get_xwayland_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xwayland_shell_v1#{}.get_xwayland_surface()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
    #[doc = ""]
    #[doc = "An Xwayland surface is a surface managed by an Xwayland server."]
    #[doc = "It is used for associating surfaces to Xwayland windows."]
    #[doc = ""]
    #[doc = "The Xwayland server associated with actions in this interface is"]
    #[doc = "determined by the Wayland client making the request."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for the xwayland_surface_v1 state to take effect."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xwayland_surface_v1 {
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
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
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xwayland_surface_v1 interface. See the module level documentation for more info"]
        pub trait XwaylandSurfaceV1 {
            const INTERFACE: &'static str = "xwayland_surface_v1";
            const VERSION: u32 = 1u32;
            async fn handle_event(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> crate::client::Result<()> {
                #[allow(clippy::match_single_binding)]
                match message.opcode() {
                    _ => Err(crate::client::Error::UnknownOpcode),
                }
            }
            #[doc = ""]
            #[doc = "Associates an Xwayland window to a wl_surface."]
            #[doc = "The association state is double-buffered, see wl_surface.commit."]
            #[doc = ""]
            #[doc = "The `serial_lo` and `serial_hi` parameters specify a non-zero"]
            #[doc = "monotonic serial number which is entirely unique and provided by the"]
            #[doc = "Xwayland server equal to the serial value provided by a client message"]
            #[doc = "with a message type of the `WL_SURFACE_SERIAL` atom on the X11 window"]
            #[doc = "for this surface to be associated to."]
            #[doc = ""]
            #[doc = "The serial value in the `WL_SURFACE_SERIAL` client message is specified"]
            #[doc = "as having the lo-bits specified in `l[0]` and the hi-bits specified"]
            #[doc = "in `l[1]`."]
            #[doc = ""]
            #[doc = "If the serial value provided by `serial_lo` and `serial_hi` is not"]
            #[doc = "valid, the `invalid_serial` protocol error will be raised."]
            #[doc = ""]
            #[doc = "An X11 window may be associated with multiple surfaces throughout its"]
            #[doc = "lifespan. (eg. unmapping and remapping a window)."]
            #[doc = ""]
            #[doc = "For each wl_surface, this state must not be committed more than once,"]
            #[doc = "otherwise the `already_associated` protocol error will be raised."]
            #[doc = ""]
            async fn set_serial(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                serial_lo: u32,
                serial_hi: u32,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xwayland_surface_v1#{}.set_serial()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(serial_lo)
                    .put_uint(serial_hi)
                    .build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
            #[doc = ""]
            #[doc = "Destroy the xwayland_surface_v1 object."]
            #[doc = ""]
            #[doc = "Any already existing associations are unaffected by this action."]
            #[doc = ""]
            async fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> crate::client::Result<()> {
                tracing::debug!("-> xwayland_surface_v1#{}.destroy()", sender_id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        }
    }
}
