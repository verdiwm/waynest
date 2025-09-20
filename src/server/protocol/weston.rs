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
#[allow(clippy::module_inception)]
pub mod color_management_v1 {
    #[doc = ""]
    #[doc = "A global interface used for getting color management extensions for"]
    #[doc = "wl_surface and wl_output objects, and for creating client defined image"]
    #[doc = "description objects. The extension interfaces allow"]
    #[doc = "getting the image description of outputs and setting the image"]
    #[doc = "description of surfaces."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_color_manager_v4 {
        #[allow(unused)]
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
            #[doc = "new_icc_creator request"]
            IccV2V4 = 0u32,
            #[doc = "new_parametric_creator request"]
            Parametric = 1u32,
            #[doc = "parametric set_primaries request"]
            SetPrimaries = 2u32,
            #[doc = "parametric set_tf_power request"]
            SetTfPower = 3u32,
            #[doc = "parametric set_luminances request"]
            SetLuminances = 4u32,
            SetMasteringDisplayPrimaries = 5u32,
            ExtendedTargetVolume = 6u32,
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
        #[doc = "Descriptions do list the specifications for convenience."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Primaries {
            Srgb = 0u32,
            PalM = 1u32,
            Pal = 2u32,
            Ntsc = 3u32,
            GenericFilm = 4u32,
            Bt2020 = 5u32,
            Cie1931Xyz = 6u32,
            DciP3 = 7u32,
            DisplayP3 = 8u32,
            AdobeRgb = 9u32,
        }
        impl TryFrom<u32> for Primaries {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Srgb),
                    1u32 => Ok(Self::PalM),
                    2u32 => Ok(Self::Pal),
                    3u32 => Ok(Self::Ntsc),
                    4u32 => Ok(Self::GenericFilm),
                    5u32 => Ok(Self::Bt2020),
                    6u32 => Ok(Self::Cie1931Xyz),
                    7u32 => Ok(Self::DciP3),
                    8u32 => Ok(Self::DisplayP3),
                    9u32 => Ok(Self::AdobeRgb),
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
        #[doc = "Named transfer functions used to encode well-known transfer"]
        #[doc = "characteristics. H.273 is the authority, when it comes to the exact"]
        #[doc = "formulas and authoritative specifications, where an equivalent code"]
        #[doc = "point exists."]
        #[doc = ""]
        #[doc = "Descriptions do list the specifications for convenience."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TransferFunction {
            Bt709 = 0u32,
            Gamma22 = 1u32,
            Gamma28 = 2u32,
            St240 = 3u32,
            Linear = 4u32,
            Log100 = 5u32,
            Log316 = 6u32,
            Xvycc = 7u32,
            Bt1361 = 8u32,
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
                    0u32 => Ok(Self::Bt709),
                    1u32 => Ok(Self::Gamma22),
                    2u32 => Ok(Self::Gamma28),
                    3u32 => Ok(Self::St240),
                    4u32 => Ok(Self::Linear),
                    5u32 => Ok(Self::Log100),
                    6u32 => Ok(Self::Log316),
                    7u32 => Ok(Self::Xvycc),
                    8u32 => Ok(Self::Bt1361),
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
        #[doc = "Trait to implement the xx_color_manager_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagerV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_manager_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("xx_color_manager_v4#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_manager_v4#{}.get_output({}, {})",
                                sender_id,
                                id,
                                output
                            );
                            self.get_output(client, sender_id, id, output).await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_manager_v4#{}.get_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_surface(client, sender_id, id, surface).await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_manager_v4#{}.get_feedback_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_feedback_surface(client, sender_id, id, surface)
                                .await
                        }
                        4u16 => {
                            let obj = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_manager_v4#{}.new_icc_creator({})",
                                sender_id,
                                obj
                            );
                            self.new_icc_creator(client, sender_id, obj).await
                        }
                        5u16 => {
                            let obj = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_manager_v4#{}.new_parametric_creator({})",
                                sender_id,
                                obj
                            );
                            self.new_parametric_creator(client, sender_id, obj).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the xx_color_manager_v4 object. This does not affect any other"]
            #[doc = "objects in any way."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This creates a new xx_color_management_output_v4 object for the"]
            #[doc = "given wl_output."]
            #[doc = ""]
            #[doc = "See the xx_color_management_output_v4 interface for more details."]
            #[doc = ""]
            fn get_output(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If a xx_color_management_surface_v4 object already exists for the given"]
            #[doc = "wl_surface, the protocol error surface_exists is raised."]
            #[doc = ""]
            #[doc = "This creates a new color xx_color_management_surface_v4 object for the"]
            #[doc = "given wl_surface."]
            #[doc = ""]
            #[doc = "See the xx_color_management_surface_v4 interface for more details."]
            #[doc = ""]
            fn get_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This creates a new color xx_color_management_feedback_surface_v4 object"]
            #[doc = "for the given wl_surface."]
            #[doc = ""]
            #[doc = "See the xx_color_management_feedback_surface_v4 interface for more"]
            #[doc = "details."]
            #[doc = ""]
            fn get_feedback_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Makes a new ICC-based image description creator object with all"]
            #[doc = "properties initially unset. The client can then use the object's"]
            #[doc = "interface to define all the required properties for an image description"]
            #[doc = "and finally create a xx_image_description_v4 object."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "xx_color_manager_v4.feature.icc_v2_v4."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            #[doc = ""]
            fn new_icc_creator(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                obj: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Makes a new parametric image description creator object with all"]
            #[doc = "properties initially unset. The client can then use the object's"]
            #[doc = "interface to define all the required properties for an image description"]
            #[doc = "and finally create a xx_image_description_v4 object."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "xx_color_manager_v4.feature.parametric."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            #[doc = ""]
            fn new_parametric_creator(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                obj: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each rendering intent the compositor supports."]
            #[doc = ""]
            fn supported_intent(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                render_intent: RenderIntent,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_color_manager_v4#{}.supported_intent({})",
                        sender_id,
                        render_intent
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(render_intent as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each compositor supported feature listed in the enumeration."]
            #[doc = ""]
            fn supported_feature(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                feature: Feature,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_color_manager_v4#{}.supported_feature({})",
                        sender_id,
                        feature
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(feature as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each named transfer function the compositor supports with the"]
            #[doc = "parametric image description creator."]
            #[doc = ""]
            fn supported_tf_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tf: TransferFunction,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_color_manager_v4#{}.supported_tf_named({})",
                        sender_id,
                        tf
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tf as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each named set of primaries the compositor supports with the"]
            #[doc = "parametric image description creator."]
            #[doc = ""]
            fn supported_primaries_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                primaries: Primaries,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_color_manager_v4#{}.supported_primaries_named({})",
                        sender_id,
                        primaries
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(primaries as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "A xx_color_management_output_v4 describes the color properties of an"]
    #[doc = "output."]
    #[doc = ""]
    #[doc = "The xx_color_management_output_v4 is associated with the wl_output global"]
    #[doc = "underlying the wl_output object. Therefore the client destroying the"]
    #[doc = "wl_output object has no impact, but the compositor removing the output"]
    #[doc = "global makes the xx_color_management_output_v4 object inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_color_management_output_v4 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xx_color_management_output_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagementOutputV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_management_output_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "xx_color_management_output_v4#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let image_description = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_management_output_v4#{}.get_image_description({})",
                                sender_id,
                                image_description
                            );
                            self.get_image_description(client, sender_id, image_description)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the color xx_color_management_output_v4 object. This does not"]
            #[doc = "affect any remaining protocol objects."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This creates a new xx_image_description_v4 object for the current image"]
            #[doc = "description of the output. There always is exactly one image description"]
            #[doc = "active for an output so the client should destroy the image description"]
            #[doc = "created by earlier invocations of this request. This request is usually"]
            #[doc = "sent as a reaction to the image_description_changed event or when"]
            #[doc = "creating a xx_color_management_output_v4 object."]
            #[doc = ""]
            #[doc = "The image description of an output represents the color encoding the"]
            #[doc = "output expects. There might be performance and power advantages, as well"]
            #[doc = "as improved color reproduction, if a content update matches the image"]
            #[doc = "description of the output it is being shown on. If a content update is"]
            #[doc = "shown on any other output than the one it matches the image description"]
            #[doc = "of, then the color reproduction on those outputs might be considerably"]
            #[doc = "worse."]
            #[doc = ""]
            #[doc = "The created xx_image_description_v4 object preserves the image"]
            #[doc = "description of the output from the time the object was created."]
            #[doc = ""]
            #[doc = "The resulting image description object allows get_information request."]
            #[doc = ""]
            #[doc = "If this protocol object is inert, the resulting image description object"]
            #[doc = "shall immediately deliver the xx_image_description_v4.failed event with"]
            #[doc = "the no_output cause."]
            #[doc = ""]
            #[doc = "If the interface version is inadequate for the output's image"]
            #[doc = "description, meaning that the client does not support all the events"]
            #[doc = "needed to deliver the crucial information, the resulting image"]
            #[doc = "description object shall immediately deliver the"]
            #[doc = "xx_image_description_v4.failed event with the low_version cause."]
            #[doc = ""]
            #[doc = "Otherwise the object shall immediately deliver the ready event."]
            #[doc = ""]
            fn get_image_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is sent whenever the image description of the output changed,"]
            #[doc = "followed by one wl_output.done event common to output events across all"]
            #[doc = "extensions."]
            #[doc = ""]
            #[doc = "If the client wants to use the updated image description, it needs to do"]
            #[doc = "get_image_description again, because image description objects are"]
            #[doc = "immutable."]
            #[doc = ""]
            fn image_description_changed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_color_management_output_v4#{}.image_description_changed()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "A xx_color_management_surface_v4 allows the client to set the color"]
    #[doc = "space and HDR properties of a surface."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the xx_color_management_surface_v4 is"]
    #[doc = "destroyed, the xx_color_management_surface_v4 object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_color_management_surface_v4 {
        #[allow(unused)]
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
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::RenderIntent),
                    1u32 => Ok(Self::ImageDescription),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_color_management_surface_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagementSurfaceV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_management_surface_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "xx_color_management_surface_v4#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let image_description = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let render_intent = message.uint()?;
                            tracing::debug!(
                                "xx_color_management_surface_v4#{}.set_image_description({}, {})",
                                sender_id,
                                image_description,
                                render_intent
                            );
                            self.set_image_description(
                                client,
                                sender_id,
                                image_description,
                                render_intent.try_into()?,
                            )
                            .await
                        }
                        2u16 => {
                            tracing::debug!(
                                "xx_color_management_surface_v4#{}.unset_image_description()",
                                sender_id,
                            );
                            self.unset_image_description(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the xx_color_management_surface_v4 object and do the same as"]
            #[doc = "unset_image_description."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
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
            #[doc = "Image description whose creation gracefully failed (received"]
            #[doc = "xx_image_description_v4.failed) are forbidden in this request, and in"]
            #[doc = "such case the protocol error image_description is raised."]
            #[doc = ""]
            #[doc = "All image descriptions whose creation succeeded (received"]
            #[doc = "xx_image_description_v4.ready) are allowed and must always be accepted"]
            #[doc = "by the compositor."]
            #[doc = ""]
            #[doc = "A rendering intent provides the client's preference on how content"]
            #[doc = "colors should be mapped to each output. The render_intent value must"]
            #[doc = "be one advertised by the compositor with"]
            #[doc = "xx_color_manager_v4.render_intent event, otherwise the protocol error"]
            #[doc = "render_intent is raised."]
            #[doc = ""]
            #[doc = "By default, a surface does not have an associated image description"]
            #[doc = "nor a rendering intent. The handling of color on such surfaces is"]
            #[doc = "compositor implementation defined. Compositors should handle such"]
            #[doc = "surfaces as sRGB but may handle them differently if they have specific"]
            #[doc = "requirements."]
            #[doc = ""]
            fn set_image_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
                render_intent : super :: super :: super :: weston :: color_management_v1 :: xx_color_manager_v4 :: RenderIntent,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This request removes any image description from the surface. See"]
            #[doc = "set_image_description for how a compositor handles a surface without"]
            #[doc = "an image description. This is double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            fn unset_image_description(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "A xx_color_management_feedback_surface_v4 allows the client to get the"]
    #[doc = "preferred color description of a surface."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with this object is destroyed, the"]
    #[doc = "xx_color_management_feedback_surface_v4 object becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_color_management_feedback_surface_v4 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "forbidden request on inert object"]
            Inert = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Inert),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_color_management_feedback_surface_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagementFeedbackSurfaceV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_management_feedback_surface_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!(
                                "xx_color_management_feedback_surface_v4#{}.destroy()",
                                sender_id,
                            );
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let image_description = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_color_management_feedback_surface_v4#{}.get_preferred({})",
                                sender_id,
                                image_description
                            );
                            self.get_preferred(client, sender_id, image_description)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the xx_color_management_feedback_surface_v4 object."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If this protocol object is inert, the protocol error inert is raised."]
            #[doc = ""]
            #[doc = "The preferred image description represents the compositor's preferred"]
            #[doc = "color encoding for this wl_surface at the current time. There might be"]
            #[doc = "performance and power advantages, as well as improved color"]
            #[doc = "reproduction, if the image description of a content update matches the"]
            #[doc = "preferred image description."]
            #[doc = ""]
            #[doc = "This creates a new xx_image_description_v4 object for the currently"]
            #[doc = "preferred image description for the wl_surface. The client should"]
            #[doc = "stop using and destroy the image descriptions created by earlier"]
            #[doc = "invocations of this request for the associated wl_surface."]
            #[doc = "This request is usually sent as a reaction to the preferred_changed"]
            #[doc = "event or when creating a xx_color_management_feedback_surface_v4 object"]
            #[doc = "if the client is capable of adapting to image descriptions."]
            #[doc = ""]
            #[doc = "The created xx_image_description_v4 object preserves the preferred image"]
            #[doc = "description of the wl_surface from the time the object was created."]
            #[doc = ""]
            #[doc = "The resulting image description object allows get_information request."]
            #[doc = ""]
            #[doc = "If the interface version is inadequate for the preferred image"]
            #[doc = "description, meaning that the client does not support all the"]
            #[doc = "events needed to deliver the crucial information, the resulting image"]
            #[doc = "description object shall immediately deliver the"]
            #[doc = "xx_image_description_v4.failed event with the low_version cause,"]
            #[doc = "otherwise the object shall immediately deliver the ready event."]
            #[doc = ""]
            fn get_preferred(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The preferred image description is the one which likely has the most"]
            #[doc = "performance and/or quality benefits for the compositor if used by the"]
            #[doc = "client for its wl_surface contents. This event is sent whenever the"]
            #[doc = "compositor changes the wl_surface's preferred image description."]
            #[doc = ""]
            #[doc = "This event is merely a notification. When the client wants to know"]
            #[doc = "what the preferred image description is, it shall use the get_preferred"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "The preferred image description is not automatically used for anything."]
            #[doc = "It is only a hint, and clients may set any valid image description with"]
            #[doc = "set_image_description but there might be performance and color accuracy"]
            #[doc = "improvements by providing the wl_surface contents in the preferred"]
            #[doc = "image description. Therefore clients that can, should render according"]
            #[doc = "to the preferred image description"]
            #[doc = ""]
            fn preferred_changed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_color_management_feedback_surface_v4#{}.preferred_changed()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "This type of object is used for collecting all the information required"]
    #[doc = "to create a xx_image_description_v4 object from an ICC file. A complete"]
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
    pub mod xx_image_description_creator_icc_v4 {
        #[allow(unused)]
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
        #[doc = "Trait to implement the xx_image_description_creator_icc_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionCreatorIccV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_creator_icc_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let image_description = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_image_description_creator_icc_v4#{}.create({})",
                                sender_id,
                                image_description
                            );
                            let result = self.create(client, sender_id, image_description).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let icc_profile = client.fd()?;
                            let offset = message.uint()?;
                            let length = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_icc_v4#{}.set_icc_file({}, {}, {})",
                                sender_id,
                                icc_profile.as_raw_fd(),
                                offset,
                                length
                            );
                            self.set_icc_file(client, sender_id, icc_profile, offset, length)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
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
            #[doc = "immediately deliver the xx_image_description_v4.failed event with the"]
            #[doc = "'unsupported' cause. If a valid image description was created from the"]
            #[doc = "information, the xx_image_description_v4.ready event will eventually"]
            #[doc = "be sent instead."]
            #[doc = ""]
            #[doc = "This request destroys the xx_image_description_creator_icc_v4 object."]
            #[doc = ""]
            #[doc = "The resulting image description object does not allow get_information"]
            #[doc = "request."]
            #[doc = ""]
            fn create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the ICC profile file to be used as the basis of the image"]
            #[doc = "description."]
            #[doc = ""]
            #[doc = "The data shall be found through the given fd at the given offset, having"]
            #[doc = "the given length. The fd must seekable and readable. Violating these"]
            #[doc = "requirements raises the bad_fd protocol error."]
            #[doc = ""]
            #[doc = "If reading the data fails due to an error independent of the client, the"]
            #[doc = "compositor shall send the xx_image_description_v4.failed event on the"]
            #[doc = "created xx_image_description_v4 with the 'operating_system' cause."]
            #[doc = ""]
            #[doc = "The maximum size of the ICC profile is 4 MB. If length is greater than"]
            #[doc = "that or zero, the protocol error bad_size is raised. If offset + length"]
            #[doc = "exceeds the file size, the protocol error out_of_file is raised."]
            #[doc = ""]
            #[doc = "A compositor may read the file at any time starting from this request"]
            #[doc = "and only until whichever happens first:"]
            #[doc = "- If create request was issued, the xx_image_description_v4 object"]
            #[doc = "delivers either failed or ready event; or"]
            #[doc = "- if create request was not issued, this"]
            #[doc = "xx_image_description_creator_icc_v4 object is destroyed."]
            #[doc = ""]
            #[doc = "A compositor shall not modify the contents of the file, and the fd may"]
            #[doc = "be sealed for writes and size changes. The client must ensure to its"]
            #[doc = "best ability that the data does not change while the compositor is"]
            #[doc = "reading it."]
            #[doc = ""]
            #[doc = "The data must represent a valid ICC profile. The ICC profile version"]
            #[doc = "must be 2 or 4, it must be a 3 channel profile and the class must be"]
            #[doc = "Display or ColorSpace. Violating these requirements will not result in a"]
            #[doc = "protocol error but will eventually send the"]
            #[doc = "xx_image_description_v4.failed event on the created"]
            #[doc = "xx_image_description_v4 with the 'unsupported' cause."]
            #[doc = ""]
            #[doc = "See the International Color Consortium specification ICC.1:2022 for more"]
            #[doc = "details about ICC profiles."]
            #[doc = ""]
            #[doc = "If ICC file has already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            fn set_icc_file(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                icc_profile: rustix::fd::OwnedFd,
                offset: u32,
                length: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "This type of object is used for collecting all the parameters required"]
    #[doc = "to create a xx_image_description_v4 object. A complete set of required"]
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
    pub mod xx_image_description_creator_params_v4 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "incomplete parameter set"]
            IncompleteSet = 0u32,
            #[doc = "invalid combination of parameters"]
            InconsistentSet = 1u32,
            #[doc = "property already set"]
            AlreadySet = 2u32,
            #[doc = "request not supported"]
            UnsupportedFeature = 3u32,
            #[doc = "invalid transfer characteristic"]
            InvalidTf = 4u32,
            #[doc = "invalid primaries or white point"]
            InvalidPrimaries = 5u32,
            #[doc = "invalid luminance value or range"]
            InvalidLuminance = 6u32,
            #[doc = "invalid mastering information"]
            InvalidMastering = 7u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::IncompleteSet),
                    1u32 => Ok(Self::InconsistentSet),
                    2u32 => Ok(Self::AlreadySet),
                    3u32 => Ok(Self::UnsupportedFeature),
                    4u32 => Ok(Self::InvalidTf),
                    5u32 => Ok(Self::InvalidPrimaries),
                    6u32 => Ok(Self::InvalidLuminance),
                    7u32 => Ok(Self::InvalidMastering),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_image_description_creator_params_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionCreatorParamsV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_creator_params_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let image_description = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.create({})",
                                sender_id,
                                image_description
                            );
                            let result = self.create(client, sender_id, image_description).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let tf = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_tf_named({})",
                                sender_id,
                                tf
                            );
                            self.set_tf_named(client, sender_id, tf.try_into()?).await
                        }
                        2u16 => {
                            let eexp = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_tf_power({})",
                                sender_id,
                                eexp
                            );
                            self.set_tf_power(client, sender_id, eexp).await
                        }
                        3u16 => {
                            let primaries = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_primaries_named({})",
                                sender_id,
                                primaries
                            );
                            self.set_primaries_named(client, sender_id, primaries.try_into()?)
                                .await
                        }
                        4u16 => {
                            let r_x = message.int()?;
                            let r_y = message.int()?;
                            let g_x = message.int()?;
                            let g_y = message.int()?;
                            let b_x = message.int()?;
                            let b_y = message.int()?;
                            let w_x = message.int()?;
                            let w_y = message.int()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_primaries({}, {}, {}, {}, {}, {}, {}, {})",
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
                            self.set_primaries(
                                client, sender_id, r_x, r_y, g_x, g_y, b_x, b_y, w_x, w_y,
                            )
                            .await
                        }
                        5u16 => {
                            let min_lum = message.uint()?;
                            let max_lum = message.uint()?;
                            let reference_lum = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_luminances({}, {}, {})",
                                sender_id,
                                min_lum,
                                max_lum,
                                reference_lum
                            );
                            self.set_luminances(client, sender_id, min_lum, max_lum, reference_lum)
                                .await
                        }
                        6u16 => {
                            let r_x = message.int()?;
                            let r_y = message.int()?;
                            let g_x = message.int()?;
                            let g_y = message.int()?;
                            let b_x = message.int()?;
                            let b_y = message.int()?;
                            let w_x = message.int()?;
                            let w_y = message.int()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_mastering_display_primaries({}, {}, {}, {}, {}, {}, {}, {})",
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
                            self.set_mastering_display_primaries(
                                client, sender_id, r_x, r_y, g_x, g_y, b_x, b_y, w_x, w_y,
                            )
                            .await
                        }
                        7u16 => {
                            let min_lum = message.uint()?;
                            let max_lum = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_mastering_luminance({}, {})",
                                sender_id,
                                min_lum,
                                max_lum
                            );
                            self.set_mastering_luminance(client, sender_id, min_lum, max_lum)
                                .await
                        }
                        8u16 => {
                            let max_cll = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_max_cll({})",
                                sender_id,
                                max_cll
                            );
                            self.set_max_cll(client, sender_id, max_cll).await
                        }
                        9u16 => {
                            let max_fall = message.uint()?;
                            tracing::debug!(
                                "xx_image_description_creator_params_v4#{}.set_max_fall({})",
                                sender_id,
                                max_fall
                            );
                            self.set_max_fall(client, sender_id, max_fall).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
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
            #[doc = "Also, the combination of the parameter set is verified. If the set is"]
            #[doc = "not consistent, the protocol error inconsistent_set is raised."]
            #[doc = ""]
            #[doc = "If the particular combination of the parameter set is not supported"]
            #[doc = "by the compositor, the resulting image description object shall"]
            #[doc = "immediately deliver the xx_image_description_v4.failed event with the"]
            #[doc = "'unsupported' cause. If a valid image description was created from the"]
            #[doc = "parameter set, the xx_image_description_v4.ready event will eventually"]
            #[doc = "be sent instead."]
            #[doc = ""]
            #[doc = "This request destroys the xx_image_description_creator_params_v4"]
            #[doc = "object."]
            #[doc = ""]
            #[doc = "The resulting image description object does not allow get_information"]
            #[doc = "request."]
            #[doc = ""]
            fn create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                image_description: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the transfer characteristic using explicitly enumerated named"]
            #[doc = "functions."]
            #[doc = ""]
            #[doc = "When the resulting image description is attached to an image, the"]
            #[doc = "content should be encoded and decoded according to the industry standard"]
            #[doc = "practices for the transfer characteristic."]
            #[doc = ""]
            #[doc = "Only names advertised with xx_color_manager_v4 event supported_tf_named"]
            #[doc = "are allowed. Other values shall raise the protocol error invalid_tf."]
            #[doc = ""]
            #[doc = "If transfer characteristic has already been set on this object, the"]
            #[doc = "protocol error already_set is raised."]
            #[doc = ""]
            fn set_tf_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tf : super :: super :: super :: weston :: color_management_v1 :: xx_color_manager_v4 :: TransferFunction,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the color component transfer characteristic to a power curve with"]
            #[doc = "the given exponent. This curve represents the conversion from electrical"]
            #[doc = "to optical pixel or color values."]
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
            #[doc = "xx_color_manager_v4.feature.set_tf_power. Otherwise this request raises"]
            #[doc = "the protocol error unsupported_feature."]
            #[doc = ""]
            fn set_tf_power(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                eexp: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the color primaries and white point using explicitly named sets."]
            #[doc = "This describes the primary color volume which is the basis for color"]
            #[doc = "value encoding."]
            #[doc = ""]
            #[doc = "Only names advertised with xx_color_manager_v4 event"]
            #[doc = "supported_primaries_named are allowed. Other values shall raise the"]
            #[doc = "protocol error invalid_primaries."]
            #[doc = ""]
            #[doc = "If primaries have already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            fn set_primaries_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                primaries : super :: super :: super :: weston :: color_management_v1 :: xx_color_manager_v4 :: Primaries,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the color primaries and white point using CIE 1931 xy chromaticity"]
            #[doc = "coordinates. This describes the primary color volume which is the basis"]
            #[doc = "for color value encoding."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 10000 to get the argument value"]
            #[doc = "to carry precision of 4 decimals."]
            #[doc = ""]
            #[doc = "If primaries have already been set on this object, the protocol error"]
            #[doc = "already_set is raised."]
            #[doc = ""]
            #[doc = "This request can be used if the compositor advertises"]
            #[doc = "xx_color_manager_v4.feature.set_primaries. Otherwise this request raises"]
            #[doc = "the protocol error unsupported_feature."]
            #[doc = ""]
            fn set_primaries(
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
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the primary color volume luminance range and the reference white"]
            #[doc = "luminance level."]
            #[doc = ""]
            #[doc = "The default luminances are"]
            #[doc = "- primary color volume minimum: 0.2 cd/m²"]
            #[doc = "- primary color volume maximum: 80 cd/m²"]
            #[doc = "- reference white: 80 cd/m²"]
            #[doc = ""]
            #[doc = "Setting a named transfer characteristic can imply other default"]
            #[doc = "luminances."]
            #[doc = ""]
            #[doc = "The default luminances get overwritten when this request is used."]
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
            #[doc = "If 'max_lum' is less than the 'reference_lum', or 'reference_lum' is"]
            #[doc = "less than or equal to 'min_lum', the protocol error invalid_luminance is"]
            #[doc = "raised."]
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
            #[doc = "xx_color_manager_v4.feature.set_luminances. Otherwise this request"]
            #[doc = "raises the protocol error unsupported_feature."]
            #[doc = ""]
            fn set_luminances(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
                reference_lum: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Provides the color primaries and white point of the mastering display"]
            #[doc = "using CIE 1931 xy chromaticity coordinates. This is compatible with the"]
            #[doc = "SMPTE ST 2086 definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "The mastering display primaries define the target color volume."]
            #[doc = ""]
            #[doc = "If mastering display primaries are not explicitly set, the target color"]
            #[doc = "volume is assumed to be equal to the primary color volume."]
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
            #[doc = "Each coordinate value is multiplied by 10000 to get the argument value"]
            #[doc = "to carry precision of 4 decimals."]
            #[doc = ""]
            #[doc = "If mastering display primaries have already been set on this object, the"]
            #[doc = "protocol error already_set is raised."]
            #[doc = ""]
            #[doc = "This request can be used if the compositor advertises"]
            #[doc = "xx_color_manager_v4.feature.set_mastering_display_primaries. Otherwise"]
            #[doc = "this request raises the protocol error unsupported_feature. The"]
            #[doc = "advertisement implies support only for target color volumes fully"]
            #[doc = "contained within the primary color volume."]
            #[doc = ""]
            #[doc = "If a compositor additionally supports target color volume exceeding the"]
            #[doc = "primary color volume, it must advertise"]
            #[doc = "xx_color_manager_v4.feature.extended_target_volume. If a client uses"]
            #[doc = "target color volume exceeding the primary color volume and the"]
            #[doc = "compositor does not support it, the result is implementation defined."]
            #[doc = "Compositors are recommended to detect this case and fail the image"]
            #[doc = "description gracefully, but it may as well result in color artifacts."]
            #[doc = ""]
            fn set_mastering_display_primaries(
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
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the luminance range that was used during the content mastering"]
            #[doc = "process as the minimum and maximum absolute luminance L. This is"]
            #[doc = "compatible with the SMPTE ST 2086 definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "The mastering luminance range is undefined by default."]
            #[doc = ""]
            #[doc = "If max L is less than or equal to min L, the protocol error"]
            #[doc = "invalid_luminance is raised."]
            #[doc = ""]
            #[doc = "Min L value is multiplied by 10000 to get the argument min_lum value"]
            #[doc = "and carry precision of 4 decimals. Max L value is unscaled for max_lum."]
            #[doc = ""]
            fn set_mastering_luminance(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the maximum content light level (max_cll) as defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This can only be set when set_tf_cicp is used to set the transfer"]
            #[doc = "characteristic to Rec. ITU-R BT.2100-2 perceptual quantization system."]
            #[doc = "Otherwise, 'create' request shall raise inconsistent_set protocol"]
            #[doc = "error."]
            #[doc = ""]
            #[doc = "max_cll is undefined by default."]
            #[doc = ""]
            fn set_max_cll(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_cll: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Sets the maximum frame-average light level (max_fall) as defined by"]
            #[doc = "CTA-861-H."]
            #[doc = ""]
            #[doc = "This can only be set when set_tf_cicp is used to set the transfer"]
            #[doc = "characteristic to Rec. ITU-R BT.2100-2 perceptual quantization system."]
            #[doc = "Otherwise, 'create' request shall raise inconsistent_set protocol error."]
            #[doc = ""]
            #[doc = "max_fall is undefined by default."]
            #[doc = ""]
            fn set_max_fall(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_fall: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "An image description carries information about the color encoding used on"]
    #[doc = "a surface when attached to a wl_surface via"]
    #[doc = "xx_color_management_surface_v4.set_image_description. A compositor can use"]
    #[doc = "this information to decode pixel values into colorimetrically meaningful"]
    #[doc = "quantities."]
    #[doc = ""]
    #[doc = "Note, that the xx_image_description_v4 object is not ready to be used"]
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
    #[doc = "xx_image_description_v4 object always refers to one fixed image"]
    #[doc = "description. It cannot change after creation."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_image_description_v4 {
        #[allow(unused)]
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
        #[doc = "Trait to implement the xx_image_description_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("xx_image_description_v4#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let information = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "xx_image_description_v4#{}.get_information({})",
                                sender_id,
                                information
                            );
                            self.get_information(client, sender_id, information).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy this object. It is safe to destroy an object which is not ready."]
            #[doc = ""]
            #[doc = "Destroying a xx_image_description_v4 object has no side-effects, not"]
            #[doc = "even if a xx_color_management_surface_v4.set_image_description has not"]
            #[doc = "yet been followed by a wl_surface.commit."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Creates a xx_image_description_info_v4 object which delivers the"]
            #[doc = "information that makes up the image description."]
            #[doc = ""]
            #[doc = "Not all image description protocol objects allow get_information"]
            #[doc = "request. Whether it is allowed or not is defined by the request that"]
            #[doc = "created the object. If get_information is not allowed, the protocol"]
            #[doc = "error no_information is raised."]
            #[doc = ""]
            fn get_information(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                information: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If creating a xx_image_description_v4 object fails for a reason that is"]
            #[doc = "not defined as a protocol error, this event is sent."]
            #[doc = ""]
            #[doc = "The requests that create image description objects define whether and"]
            #[doc = "when this can occur. Only such creation requests can trigger this event."]
            #[doc = "This event cannot be triggered after the image description was"]
            #[doc = "successfully formed."]
            #[doc = ""]
            #[doc = "Once this event has been sent, the xx_image_description_v4 object will"]
            #[doc = "never become ready and it can only be destroyed."]
            #[doc = ""]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                cause: Cause,
                msg: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_v4#{}.failed({}, \"{}\")",
                        sender_id,
                        cause,
                        msg
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(cause as u32)
                        .put_string(Some(msg))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Once this event has been sent, the xx_image_description_v4 object is"]
            #[doc = "deemed \"ready\". Ready objects can be used to send requests and can be"]
            #[doc = "used through other interfaces."]
            #[doc = ""]
            #[doc = "Every ready xx_image_description_v4 protocol object refers to an"]
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
            #[doc = "numbers might not be portable between Wayland connections."]
            #[doc = ""]
            #[doc = "This identity allows clients to de-duplicate image description records"]
            #[doc = "and avoid get_information request if they already have the image"]
            #[doc = "description information."]
            #[doc = ""]
            fn ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                identity: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_v4#{}.ready({})",
                        sender_id,
                        identity
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(identity)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "Sends all matching events describing an image description object exactly"]
    #[doc = "once and finally sends the 'done' event."]
    #[doc = ""]
    #[doc = "Once a xx_image_description_info_v4 object has delivered a 'done' event it"]
    #[doc = "is automatically destroyed."]
    #[doc = ""]
    #[doc = "Every xx_image_description_info_v4 created from the same"]
    #[doc = "xx_image_description_v4 shall always return the exact same data."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_image_description_info_v4 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the xx_image_description_info_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionInfoV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_info_v4";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Signals the end of information events and destroys the object."]
            #[doc = ""]
            fn done(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> xx_image_description_info_v4#{}.done()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
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
            fn icc_file(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                icc: rustix::fd::OwnedFd,
                icc_size: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.icc_file({}, {})",
                        sender_id,
                        icc.as_raw_fd(),
                        icc_size
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_fd(icc)
                        .put_uint(icc_size)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Delivers the primary color volume primaries and white point using CIE"]
            #[doc = "1931 xy chromaticity coordinates."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 10000 to get the argument value"]
            #[doc = "to carry precision of 4 decimals."]
            #[doc = ""]
            fn primaries(
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
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.primaries({}, {}, {}, {}, {}, {}, {}, {})",
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
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Delivers the primary color volume primaries and white point using an"]
            #[doc = "explicitly enumerated named set."]
            #[doc = ""]
            fn primaries_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                primaries : super :: super :: super :: weston :: color_management_v1 :: xx_color_manager_v4 :: Primaries,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.primaries_named({})",
                        sender_id,
                        primaries
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(primaries as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "The color component transfer characteristic of this image description is"]
            #[doc = "a pure power curve. This event provides the exponent of the power"]
            #[doc = "function. This curve represents the conversion from electrical to"]
            #[doc = "optical pixel or color values."]
            #[doc = ""]
            #[doc = "The curve exponent has been multiplied by 10000 to get the argument eexp"]
            #[doc = "value to carry the precision of 4 decimals."]
            #[doc = ""]
            fn tf_power(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                eexp: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.tf_power({})",
                        sender_id,
                        eexp
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(eexp).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Delivers the transfer characteristic using an explicitly enumerated"]
            #[doc = "named function."]
            #[doc = ""]
            fn tf_named(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tf : super :: super :: super :: weston :: color_management_v1 :: xx_color_manager_v4 :: TransferFunction,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.tf_named({})",
                        sender_id,
                        tf
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(tf as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Delivers the primary color volume luminance range and the reference"]
            #[doc = "white luminance level."]
            #[doc = ""]
            #[doc = "The minimum luminance is multiplied by 10000 to get the argument"]
            #[doc = "'min_lum' value and carries precision of 4 decimals. The maximum"]
            #[doc = "luminance and reference white luminance values are unscaled."]
            #[doc = ""]
            fn luminances(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
                reference_lum: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.luminances({}, {}, {})",
                        sender_id,
                        min_lum,
                        max_lum,
                        reference_lum
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(min_lum)
                        .put_uint(max_lum)
                        .put_uint(reference_lum)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
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
            #[doc = "Each coordinate value is multiplied by 10000 to get the argument value"]
            #[doc = "to carry precision of 4 decimals."]
            #[doc = ""]
            fn target_primaries(
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
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.target_primaries({}, {}, {}, {}, {}, {}, {}, {})",
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
                        .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Provides the luminance range that the image description is targeting as"]
            #[doc = "the minimum and maximum absolute luminance L. This is compatible with"]
            #[doc = "the SMPTE ST 2086 definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "This luminance range is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            #[doc = "Min L value is multiplied by 10000 to get the argument min_lum value and"]
            #[doc = "carry precision of 4 decimals. Max L value is unscaled for max_lum."]
            #[doc = ""]
            fn target_luminance(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                min_lum: u32,
                max_lum: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.target_luminance({}, {})",
                        sender_id,
                        min_lum,
                        max_lum
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(min_lum)
                        .put_uint(max_lum)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 8u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Provides the targeted max_cll of the image description. max_cll is"]
            #[doc = "defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This luminance is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            fn target_max_cll(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_cll: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.target_max_cll({})",
                        sender_id,
                        max_cll
                    );
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_uint(max_cll).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 9u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Provides the targeted max_fall of the image description. max_fall is"]
            #[doc = "defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This luminance is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            fn target_max_fall(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                max_fall: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> xx_image_description_info_v4#{}.target_max_fall({})",
                        sender_id,
                        max_fall
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(max_fall)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 10u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ivi_application {
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_surface {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the ivi_surface interface. See the module level documentation for more info"]
        pub trait IviSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_surface";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("ivi_surface#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This removes the link from ivi_id to wl_surface and destroys ivi_surface."]
            #[doc = "The ID, ivi_id, is free and can be used for surface_create again."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The configure event asks the client to resize its surface."]
            #[doc = ""]
            #[doc = "The size is a hint, in the sense that the client is free to"]
            #[doc = "ignore it if it doesn't resize, pick a smaller size (to"]
            #[doc = "satisfy aspect ratio or resize in steps of NxM pixels)."]
            #[doc = ""]
            #[doc = "The client is free to dismiss all but the last configure"]
            #[doc = "event it received."]
            #[doc = ""]
            #[doc = "The width and height arguments specify the size of the window"]
            #[doc = "in surface-local coordinates."]
            #[doc = ""]
            fn configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> ivi_surface#{}.configure({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "This interface is exposed as a global singleton."]
    #[doc = "This interface is implemented by servers that provide IVI-style user interfaces."]
    #[doc = "It allows clients to associate an ivi_surface with wl_surface."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_application {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "given wl_surface has another role"]
            Role = 0u32,
            #[doc = "given ivi_id is assigned to another wl_surface"]
            IviId = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Role),
                    1u32 => Ok(Self::IviId),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_application interface. See the module level documentation for more info"]
        pub trait IviApplication: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_application";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let ivi_id = message.uint()?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "ivi_application#{}.surface_create({}, {}, {})",
                                sender_id,
                                ivi_id,
                                surface,
                                id
                            );
                            self.surface_create(client, sender_id, ivi_id, surface, id)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request gives the wl_surface the role of an IVI Surface. Creating more than"]
            #[doc = "one ivi_surface for a wl_surface is not allowed. Note, that this still allows the"]
            #[doc = "following example:"]
            #[doc = ""]
            #[doc = "1. create a wl_surface"]
            #[doc = "2. create ivi_surface for the wl_surface"]
            #[doc = "3. destroy the ivi_surface"]
            #[doc = "4. create ivi_surface for the wl_surface (with the same or another ivi_id as before)"]
            #[doc = ""]
            #[doc = "surface_create will create an interface:ivi_surface with numeric ID; ivi_id in"]
            #[doc = "ivi compositor. These ivi_ids are defined as unique in the system to identify"]
            #[doc = "it inside of ivi compositor. The ivi compositor implements business logic how to"]
            #[doc = "set properties of the surface with ivi_id according to the status of the system."]
            #[doc = "E.g. a unique ID for Car Navigation application is used for implementing special"]
            #[doc = "logic of the application about where it shall be located."]
            #[doc = "The server regards the following cases as protocol errors and disconnects the client."]
            #[doc = "- wl_surface already has another role."]
            #[doc = "- ivi_id is already assigned to another wl_surface."]
            #[doc = ""]
            #[doc = "If client destroys ivi_surface or wl_surface which is assigned to the ivi_surface,"]
            #[doc = "ivi_id which is assigned to the ivi_surface is free for reuse."]
            #[doc = ""]
            fn surface_create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                ivi_id: u32,
                surface: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod ivi_hmi_controller {
    #[allow(clippy::too_many_arguments)]
    pub mod ivi_hmi_controller {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum LayoutMode {
            Tiling = 0u32,
            SideBySide = 1u32,
            FullScreen = 2u32,
            Random = 3u32,
        }
        impl TryFrom<u32> for LayoutMode {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Tiling),
                    1u32 => Ok(Self::SideBySide),
                    2u32 => Ok(Self::FullScreen),
                    3u32 => Ok(Self::Random),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for LayoutMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Home {
            Off = 0u32,
            On = 1u32,
        }
        impl TryFrom<u32> for Home {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Off),
                    1u32 => Ok(Self::On),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Home {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the ivi_hmi_controller interface. See the module level documentation for more info"]
        pub trait IviHmiController: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_hmi_controller";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("ivi_hmi_controller#{}.ui_ready()", sender_id,);
                            self.ui_ready(client, sender_id).await
                        }
                        1u16 => {
                            let seat = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let serial = message.uint()?;
                            tracing::debug!(
                                "ivi_hmi_controller#{}.workspace_control({}, {})",
                                sender_id,
                                seat,
                                serial
                            );
                            self.workspace_control(client, sender_id, seat, serial)
                                .await
                        }
                        2u16 => {
                            let layout_mode = message.uint()?;
                            tracing::debug!(
                                "ivi_hmi_controller#{}.switch_mode({})",
                                sender_id,
                                layout_mode
                            );
                            self.switch_mode(client, sender_id, layout_mode).await
                        }
                        3u16 => {
                            let home = message.uint()?;
                            tracing::debug!("ivi_hmi_controller#{}.home({})", sender_id, home);
                            self.home(client, sender_id, home).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn ui_ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Reference protocol to control a surface by server."]
            #[doc = "To control a surface by server, it gives seat to the server"]
            #[doc = "to e.g. control Home screen. Home screen has several workspaces"]
            #[doc = "to group launchers of wayland application. These workspaces"]
            #[doc = "are drawn on a horizontally long surface to be controlled"]
            #[doc = "by motion of input device. E.g. A motion from right to left"]
            #[doc = "happens, the viewport of surface is controlled in the ivi-shell"]
            #[doc = "by using ivi-layout. client can recognizes the end of controlling"]
            #[doc = "by event \"workspace_end_control\"."]
            #[doc = ""]
            fn workspace_control(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "hmi-controller loaded to ivi-shall implements 4 types of layout"]
            #[doc = "as a reference; tiling, side by side, full_screen, and random."]
            #[doc = ""]
            fn switch_mode(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                layout_mode: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "home screen is a reference implementation of launcher to launch"]
            #[doc = "wayland applications. The home screen has several workspaces to"]
            #[doc = "group wayland applications. By defining the following keys in"]
            #[doc = "weston.ini, user can add launcher icon to launch a wayland application"]
            #[doc = "to a workspace."]
            #[doc = "[ivi-launcher]"]
            #[doc = "workspace-id=0"]
            #[doc = ": id of workspace to add a launcher"]
            #[doc = "icon-id=4001"]
            #[doc = ": ivi id of ivi_surface to draw an icon"]
            #[doc = "icon=/home/user/review/build-ivi-shell/data/icon_ivi_flower.png"]
            #[doc = ": path to icon image"]
            #[doc = "path=/home/user/review/build-ivi-shell/weston-dnd"]
            #[doc = ": path to wayland application"]
            #[doc = ""]
            fn home(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                home: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn workspace_end_control(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                is_controlled: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> ivi_hmi_controller#{}.workspace_end_control({})",
                        sender_id,
                        is_controlled
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(is_controlled)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod text_cursor_position {
    #[allow(clippy::too_many_arguments)]
    pub mod text_cursor_position {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the text_cursor_position interface. See the module level documentation for more info"]
        pub trait TextCursorPosition: crate::server::Dispatcher {
            const INTERFACE: &'static str = "text_cursor_position";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let x = message.fixed()?;
                            let y = message.fixed()?;
                            tracing::debug!(
                                "text_cursor_position#{}.notify({}, {}, {})",
                                sender_id,
                                surface,
                                x,
                                y
                            );
                            self.notify(client, sender_id, surface, x, y).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn notify(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[doc = ""]
#[doc = "This protocol specifies a set of interfaces used to provide"]
#[doc = "content-protection for e.g. HDCP, and protect surface contents on the"]
#[doc = "secured outputs and prevent from appearing in screenshots or from being"]
#[doc = "visible on non-secure outputs."]
#[doc = ""]
#[doc = "A secure-output is defined as an output that is secured by some"]
#[doc = "content-protection mechanism e.g. HDCP, and meets at least the minimum"]
#[doc = "required content-protection level requested by a client."]
#[doc = ""]
#[doc = "The term content-protection is defined in terms of HDCP type 0 and"]
#[doc = "HDCP type 1, but this may be extended in future."]
#[doc = ""]
#[doc = "This protocol is not intended for implementing Digital Rights Management on"]
#[doc = "general (e.g. Desktop) systems, and would only be useful for closed systems."]
#[doc = "As the server is the one responsible for implementing"]
#[doc = "the content-protection, the client can only trust the content-protection as"]
#[doc = "much they can trust the server."]
#[doc = ""]
#[doc = "In order to protect the content and prevent surface contents from appearing"]
#[doc = "in screenshots or from being visible on non-secure outputs, a client must"]
#[doc = "first bind the global interface \"weston_content_protection\" which, if a"]
#[doc = "compositor supports secure output, is exposed by the registry."]
#[doc = "Using the bound global object, the client uses the \"get_protection\" request"]
#[doc = "to instantiate an interface extension for a wl_surface object."]
#[doc = "This extended interface will then allow surfaces to request for"]
#[doc = "content-protection, and also to censor the visibility of the surface on"]
#[doc = "non-secure outputs. Client applications should not wait for the protection"]
#[doc = "to change, as it might never change in case the content-protection cannot be"]
#[doc = "achieved. Alternatively, clients can use a timeout and start showing the"]
#[doc = "content in lower quality."]
#[doc = ""]
#[doc = "Censored visibility is defined as the compositor censoring the protected"]
#[doc = "content on non-secure outputs. Censoring may include artificially reducing"]
#[doc = "image quality or replacing the protected content completely with"]
#[doc = "placeholder graphics."]
#[doc = ""]
#[doc = "Censored visibility is controlled by protection mode, set by the client."]
#[doc = "In \"relax\" mode, the compositor may show protected content on non-secure"]
#[doc = "outputs. It will be up to the client to adapt to secure and non-secure"]
#[doc = "presentation. In \"enforce\" mode, the compositor will censor the parts of"]
#[doc = "protected content that would otherwise show on non-secure outputs."]
#[doc = ""]
#[allow(clippy::module_inception)]
pub mod weston_content_protection {
    #[doc = ""]
    #[doc = "The global interface weston_content_protection is used for exposing the"]
    #[doc = "content protection capabilities to a client. It provides a way for clients"]
    #[doc = "to request their wl_surface contents to not be displayed on an output"]
    #[doc = "below their required level of content-protection."]
    #[doc = "Using this interface clients can request for a weston_protected_surface"]
    #[doc = "which is an extension to the wl_surface to provide content-protection, and"]
    #[doc = "set the censored-visibility on the non-secured-outputs."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_content_protection {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the surface already has a protected surface associated"]
            SurfaceExists = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SurfaceExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_content_protection interface. See the module level documentation for more info"]
        pub trait WestonContentProtection: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_content_protection";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_content_protection#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_content_protection#{}.get_protection({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_protection(client, sender_id, id, surface).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other objects,"]
            #[doc = "protected_surface objects included."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Instantiate an interface extension for the given wl_surface to"]
            #[doc = "provide surface protection. If the given wl_surface already has"]
            #[doc = "a weston_protected_surface associated, the surface_exists protocol"]
            #[doc = "error is raised."]
            #[doc = ""]
            fn get_protection(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "An additional interface to a wl_surface object, which allows a client to"]
    #[doc = "request the minimum level of content-protection, request to change the"]
    #[doc = "visibility of their contents, and receive notifications about changes in"]
    #[doc = "content-protection."]
    #[doc = ""]
    #[doc = "A protected surface has a 'status' associated with it, that indicates"]
    #[doc = "what type of protection it is currently providing, specified by"]
    #[doc = "content-type. Updates to this status are sent to the client"]
    #[doc = "via the 'status' event. Before the first status event is sent, the client"]
    #[doc = "should assume that the status is 'unprotected'."]
    #[doc = ""]
    #[doc = "A client can request a content protection level to be the minimum for an"]
    #[doc = "output to be considered secure, using the 'set_type' request."]
    #[doc = "It is responsibility of the client to monitor the actual"]
    #[doc = "content-protection level achieved via the 'status' event, and make"]
    #[doc = "decisions as to what content to show based on this."]
    #[doc = ""]
    #[doc = "The server should make its best effort to achieve the desired"]
    #[doc = "content-protection level on all of the outputs the client's contents are"]
    #[doc = "being displayed on. Any changes to the content protection status should be"]
    #[doc = "reported to the client, even if they are below the requested"]
    #[doc = "content-protection level. If the client's contents are being displayed on"]
    #[doc = "multiple outputs, the lowest content protection level achieved should be"]
    #[doc = "reported."]
    #[doc = ""]
    #[doc = "A client can also request that its content only be displayed on outputs"]
    #[doc = "that are considered secure. The 'enforce/relax' requests can achieve this."]
    #[doc = "In enforce mode, the content is censored for non-secure outputs."]
    #[doc = "The implementation of censored-visibility is compositor-defined."]
    #[doc = "In relax mode there are no such limitation. On an attempt to show the"]
    #[doc = "client on unsecured output, compositor would keep on showing the content"]
    #[doc = "and send the 'status' event to the client. Client can take a call to"]
    #[doc = "downgrade the content."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the protected_surface is destroyed,"]
    #[doc = "the protected_surface becomes inert."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_protected_surface {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "provided type was not valid"]
            InvalidType = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidType),
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
        #[doc = "Description of a particular type of content protection."]
        #[doc = ""]
        #[doc = "A server may not necessarily support all of these types."]
        #[doc = ""]
        #[doc = "Note that there is no ordering between enum members unless specified."]
        #[doc = "Over time, different types of content protection may be added, which"]
        #[doc = "may be considered less secure than what is already here."]
        #[doc = ""]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Type {
            #[doc = "no protection required"]
            Unprotected = 0u32,
            #[doc = "HDCP type 0"]
            Hdcp0 = 1u32,
            #[doc = "HDCP type 1. This is a more secure than HDCP type 0."]
            Hdcp1 = 2u32,
        }
        impl TryFrom<u32> for Type {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unprotected),
                    1u32 => Ok(Self::Hdcp0),
                    2u32 => Ok(Self::Hdcp1),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_protected_surface interface. See the module level documentation for more info"]
        pub trait WestonProtectedSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_protected_surface";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_protected_surface#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let r#type = message.uint()?;
                            tracing::debug!(
                                "weston_protected_surface#{}.set_type({})",
                                sender_id,
                                r#type
                            );
                            self.set_type(client, sender_id, r#type.try_into()?).await
                        }
                        2u16 => {
                            tracing::debug!("weston_protected_surface#{}.enforce()", sender_id,);
                            self.enforce(client, sender_id).await
                        }
                        3u16 => {
                            tracing::debug!("weston_protected_surface#{}.relax()", sender_id,);
                            self.relax(client, sender_id).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "If the protected_surface is destroyed, the wl_surface desired protection"]
            #[doc = "level returns to unprotected, as if set_type request was sent with type"]
            #[doc = "as 'unprotected'."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Informs the server about the type of content. The level of"]
            #[doc = "content-protection depends upon the content-type set by the client"]
            #[doc = "through this request. Initially, this is set to 'unprotected'."]
            #[doc = ""]
            #[doc = "If the requested value is not a valid content_type enum value, the"]
            #[doc = "'invalid_type' protocol error is raised. It is not an error to request"]
            #[doc = "a valid protection type the compositor does not implement or cannot"]
            #[doc = "achieve."]
            #[doc = ""]
            #[doc = "The requested content protection is double-buffered, see"]
            #[doc = "wl_surface.commit."]
            #[doc = ""]
            fn set_type(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                r#type: Type,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Censor the visibility of the wl_surface contents on non-secure outputs."]
            #[doc = "See weston_protected_surface for the description."]
            #[doc = ""]
            #[doc = "The force constrain mode is double-buffered, see wl_surface.commit"]
            #[doc = ""]
            fn enforce(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Do not enforce censored-visibility of the wl_surface contents on"]
            #[doc = "non-secure-outputs. See weston_protected_surface for the description."]
            #[doc = ""]
            #[doc = "The relax mode is selected by default, if no explicit request is made"]
            #[doc = "for enforcing the censored-visibility."]
            #[doc = ""]
            #[doc = "The relax mode is double-buffered, see wl_surface.commit"]
            #[doc = ""]
            fn relax(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event is sent to the client to inform about the actual protection"]
            #[doc = "level for its surface in the relax mode."]
            #[doc = ""]
            #[doc = "The 'type' argument indicates what that current level of content"]
            #[doc = "protection that the server has currently established."]
            #[doc = ""]
            #[doc = "The 'status' event is first sent, when a weston_protected_surface is"]
            #[doc = "created."]
            #[doc = ""]
            #[doc = "Until this event is sent for the first time, the client should assume"]
            #[doc = "that its contents are not secure, and the type is 'unprotected'."]
            #[doc = ""]
            #[doc = "Possible reasons the content protection status can change is due to"]
            #[doc = "change in censored-visibility mode from enforced to relaxed, a new"]
            #[doc = "connector being added, movement of window to another output, or,"]
            #[doc = "the client attaching a buffer too large for what the server may secure."]
            #[doc = "However, it is not limited to these reasons."]
            #[doc = ""]
            #[doc = "A client may want to listen to this event and lower the resolution of"]
            #[doc = "their content until it can successfully be shown securely."]
            #[doc = ""]
            #[doc = "In case of \"enforce\" mode, the client will not get any status event."]
            #[doc = "If the mode is then changed to \"relax\", the client will receive the"]
            #[doc = "status event."]
            #[doc = ""]
            fn status(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                r#type: Type,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_protected_surface#{}.status({})",
                        sender_id,
                        r#type
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(r#type as u32)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_debug {
    #[doc = ""]
    #[doc = "This is a generic debugging interface for Weston internals, the global"]
    #[doc = "object advertized through wl_registry."]
    #[doc = ""]
    #[doc = "WARNING: This interface by design allows a denial-of-service attack. It"]
    #[doc = "should not be offered in production, or proper authorization mechanisms"]
    #[doc = "must be enforced."]
    #[doc = ""]
    #[doc = "The idea is for a client to provide a file descriptor that the server"]
    #[doc = "uses for printing debug information. The server uses the file"]
    #[doc = "descriptor in blocking writes mode, which exposes the denial-of-service"]
    #[doc = "risk. The blocking mode is necessary to ensure all debug messages can"]
    #[doc = "be easily printed in place. It also ensures message ordering if a"]
    #[doc = "client subscribes to more than one debug stream."]
    #[doc = ""]
    #[doc = "The available debugging features depend on the server."]
    #[doc = ""]
    #[doc = "A debug stream can be one-shot where the server prints the requested"]
    #[doc = "information and then closes it, or continuous where server keeps on"]
    #[doc = "printing until the client stops it. Or anything in between."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_debug_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the weston_debug_v1 interface. See the module level documentation for more info"]
        pub trait WestonDebugV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_debug_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_debug_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let name = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let streamfd = client.fd()?;
                            let stream = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_debug_v1#{}.subscribe(\"{}\", {}, {})",
                                sender_id,
                                name,
                                streamfd.as_raw_fd(),
                                stream
                            );
                            self.subscribe(client, sender_id, name, streamfd, stream)
                                .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroys the factory object, but does not affect any other objects."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Subscribe to a named debug stream. The server will start printing"]
            #[doc = "to the given file descriptor."]
            #[doc = ""]
            #[doc = "If the named debug stream is a one-shot dump, the server will send"]
            #[doc = "weston_debug_stream_v1.complete event once all requested data has"]
            #[doc = "been printed. Otherwise, the server will continue streaming debug"]
            #[doc = "prints until the subscription object is destroyed."]
            #[doc = ""]
            #[doc = "If the debug stream name is unknown to the server, the server will"]
            #[doc = "immediately respond with weston_debug_stream_v1.failure event."]
            #[doc = ""]
            fn subscribe(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
                streamfd: rustix::fd::OwnedFd,
                stream: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Advertises an available debug scope which the client may be able to"]
            #[doc = "bind to. No information is provided by the server about the content"]
            #[doc = "contained within the debug streams provided by the scope, once a"]
            #[doc = "client has subscribed."]
            #[doc = ""]
            fn available(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                name: String,
                description: Option<String>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_debug_v1#{}.available(\"{}\", \"{}\")",
                        sender_id,
                        name,
                        description
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_string(description)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "Represents one subscribed debug stream, created with"]
    #[doc = "weston_debug_v1.subscribe. When the object is created, it is associated"]
    #[doc = "with a given file descriptor. The server will continue writing to the"]
    #[doc = "file descriptor until the object is destroyed or the server sends an"]
    #[doc = "event through the object."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_debug_stream_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the weston_debug_stream_v1 interface. See the module level documentation for more info"]
        pub trait WestonDebugStreamV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_debug_stream_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_debug_stream_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroys the object, which causes the server to stop writing into"]
            #[doc = "and closes the associated file descriptor if it was not closed"]
            #[doc = "already."]
            #[doc = ""]
            #[doc = "Use a wl_display.sync if the clients needs to guarantee the file"]
            #[doc = "descriptor is closed before continuing."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The server has successfully finished writing to and has closed the"]
            #[doc = "associated file descriptor."]
            #[doc = ""]
            #[doc = "This event is delivered only for one-shot debug streams where the"]
            #[doc = "server dumps some data and stop. This is never delivered for"]
            #[doc = "continuous debbug streams because they by definition never complete."]
            #[doc = ""]
            fn complete(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_debug_stream_v1#{}.complete()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "The server has stopped writing to and has closed the"]
            #[doc = "associated file descriptor. The data already written to the file"]
            #[doc = "descriptor is correct, but it may be truncated."]
            #[doc = ""]
            #[doc = "This event may be delivered at any time and for any kind of debug"]
            #[doc = "stream. It may be due to a failure in or shutdown of the server."]
            #[doc = "The message argument may provide a hint of the reason."]
            #[doc = ""]
            fn failure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: Option<String>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_debug_stream_v1#{}.failure(\"{}\")",
                        sender_id,
                        message
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(message)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_desktop {
    #[doc = ""]
    #[doc = "Traditional user interfaces can rely on this interface to define the"]
    #[doc = "foundations of typical desktops. Currently it's possible to set up"]
    #[doc = "background, panels and locking surfaces."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_desktop_shell {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Cursor {
            None = 0u32,
            ResizeTop = 1u32,
            ResizeBottom = 2u32,
            Arrow = 3u32,
            ResizeLeft = 4u32,
            ResizeTopLeft = 5u32,
            ResizeBottomLeft = 6u32,
            Move = 7u32,
            ResizeRight = 8u32,
            ResizeTopRight = 9u32,
            ResizeBottomRight = 10u32,
            Busy = 11u32,
        }
        impl TryFrom<u32> for Cursor {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::ResizeTop),
                    2u32 => Ok(Self::ResizeBottom),
                    3u32 => Ok(Self::Arrow),
                    4u32 => Ok(Self::ResizeLeft),
                    5u32 => Ok(Self::ResizeTopLeft),
                    6u32 => Ok(Self::ResizeBottomLeft),
                    7u32 => Ok(Self::Move),
                    8u32 => Ok(Self::ResizeRight),
                    9u32 => Ok(Self::ResizeTopRight),
                    10u32 => Ok(Self::ResizeBottomRight),
                    11u32 => Ok(Self::Busy),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Cursor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PanelPosition {
            Top = 0u32,
            Bottom = 1u32,
            Left = 2u32,
            Right = 3u32,
        }
        impl TryFrom<u32> for PanelPosition {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Top),
                    1u32 => Ok(Self::Bottom),
                    2u32 => Ok(Self::Left),
                    3u32 => Ok(Self::Right),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PanelPosition {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "an invalid argument was provided in a request"]
            InvalidArgument = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidArgument),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_desktop_shell interface. See the module level documentation for more info"]
        pub trait WestonDesktopShell: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_desktop_shell";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_desktop_shell#{}.set_background({}, {})",
                                sender_id,
                                output,
                                surface
                            );
                            self.set_background(client, sender_id, output, surface)
                                .await
                        }
                        1u16 => {
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_desktop_shell#{}.set_panel({}, {})",
                                sender_id,
                                output,
                                surface
                            );
                            self.set_panel(client, sender_id, output, surface).await
                        }
                        2u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_desktop_shell#{}.set_lock_surface({})",
                                sender_id,
                                surface
                            );
                            self.set_lock_surface(client, sender_id, surface).await
                        }
                        3u16 => {
                            tracing::debug!("weston_desktop_shell#{}.unlock()", sender_id,);
                            self.unlock(client, sender_id).await
                        }
                        4u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_desktop_shell#{}.set_grab_surface({})",
                                sender_id,
                                surface
                            );
                            self.set_grab_surface(client, sender_id, surface).await
                        }
                        5u16 => {
                            tracing::debug!("weston_desktop_shell#{}.desktop_ready()", sender_id,);
                            self.desktop_ready(client, sender_id).await
                        }
                        6u16 => {
                            let position = message.uint()?;
                            tracing::debug!(
                                "weston_desktop_shell#{}.set_panel_position({})",
                                sender_id,
                                position
                            );
                            self.set_panel_position(client, sender_id, position).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn set_background(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn set_panel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn set_lock_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn unlock(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "The surface set by this request will receive a fake"]
            #[doc = "pointer.enter event during grabs at position 0, 0 and is"]
            #[doc = "expected to set an appropriate cursor image as described by"]
            #[doc = "the grab_cursor event sent just before the enter event."]
            #[doc = ""]
            fn set_grab_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Tell the server, that enough desktop elements have been drawn"]
            #[doc = "to make the desktop look ready for use. During start-up, the"]
            #[doc = "server can wait for this request with a black screen before"]
            #[doc = "starting to fade in the desktop, for instance. If the client"]
            #[doc = "parts of a desktop take a long time to initialize, we avoid"]
            #[doc = "showing temporary garbage."]
            #[doc = ""]
            fn desktop_ready(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Tell the shell which side of the screen the panel is"]
            #[doc = "located. This is so that new windows do not overlap the panel"]
            #[doc = "and maximized windows maximize properly."]
            #[doc = ""]
            fn set_panel_position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                position: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                edges: u32,
                surface: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.configure({}, {}, {}, {})",
                        sender_id,
                        edges,
                        surface,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(edges)
                        .put_object(Some(surface))
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Tell the client we want it to create and set the lock surface, which is"]
            #[doc = "a GUI asking the user to unlock the screen. The lock surface is"]
            #[doc = "announced with 'set_lock_surface'. Whether or not the client actually"]
            #[doc = "implements locking, it MUST send 'unlock' request to let the normal"]
            #[doc = "desktop resume."]
            #[doc = ""]
            fn prepare_lock_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.prepare_lock_surface()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event will be sent immediately before a fake enter event on the"]
            #[doc = "grab surface."]
            #[doc = ""]
            fn grab_cursor(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                cursor: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_desktop_shell#{}.grab_cursor({})",
                        sender_id,
                        cursor
                    );
                    let (payload, fds) =
                        crate::wire::PayloadBuilder::new().put_uint(cursor).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_screensaver {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the weston_screensaver interface. See the module level documentation for more info"]
        pub trait WestonScreensaver: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_screensaver";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_screensaver#{}.set_surface({}, {})",
                                sender_id,
                                surface,
                                output
                            );
                            self.set_surface(client, sender_id, surface, output).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "A screensaver surface is normally hidden, and only visible after an"]
            #[doc = "idle timeout."]
            #[doc = ""]
            fn set_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_direct_display {
    #[doc = ""]
    #[doc = "Weston extension to instruct the compositor to avoid any import"]
    #[doc = "of the dmabuf created by 'linux-dmabuf' protocol other than the display"]
    #[doc = "controller."]
    #[doc = ""]
    #[doc = "Compositors are already going to use direct scan-out as much as possible but"]
    #[doc = "there's no assurance that while doing so, they won't first import the dmabuf"]
    #[doc = "in to the GPU. This extension assures the client that the compositor will"]
    #[doc = "never attempt to import in to the GPU and pass it directly to the display"]
    #[doc = "controller."]
    #[doc = ""]
    #[doc = "Clients can make use of this extension to pass the dmabuf buffer to the"]
    #[doc = "display controller, potentially increasing the performance and lowering the"]
    #[doc = "bandwidth usage."]
    #[doc = ""]
    #[doc = "Lastly, clients can make use of this extension in tandem with content-protection"]
    #[doc = "one thus avoiding any GPU interaction and providing a secure-content path."]
    #[doc = "Also, in some cases, the memory where dmabuf are allocated are in specially"]
    #[doc = "crafted memory zone which would be seen as an illegal memory access when the"]
    #[doc = "GPU will attempt to read it."]
    #[doc = ""]
    #[doc = "WARNING: This interface by design might break screenshoting functionality"]
    #[doc = "as compositing might be involved while doing that. Also, do note, that in"]
    #[doc = "case the dmabufer provided can't be imported by KMS, the client connection"]
    #[doc = "will be terminated."]
    #[doc = ""]
    #[doc = "WARNING: This extension requires 'linux-dmabuf' protocol and"]
    #[doc = "'zwp_linux_buffer_params_v1' be already created by 'zwp_linux_buffer_v1'."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_direct_display_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the weston_direct_display_v1 interface. See the module level documentation for more info"]
        pub trait WestonDirectDisplayV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_direct_display_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let dmabuf = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_direct_display_v1#{}.enable({})",
                                sender_id,
                                dmabuf
                            );
                            self.enable(client, sender_id, dmabuf).await
                        }
                        1u16 => {
                            tracing::debug!("weston_direct_display_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This request tells the compositor not to import the dmabuf to the GPU"]
            #[doc = "in order to bypass it entirely, such that the buffer will be directly"]
            #[doc = "scanned-out by the display controller. If HW is not capable/or there"]
            #[doc = "aren't any available resources to directly scan-out the buffer, a"]
            #[doc = "placeholder should be installed in-place by the compositor. The"]
            #[doc = "compositor may perform checks on the dmabuf and refuse to create a"]
            #[doc = "wl_buffer if the dmabuf seems unusable for being used directly."]
            #[doc = ""]
            #[doc = "Assumes that 'zwp_linux_buffer_params_v1' was already created"]
            #[doc = "by 'zwp_linux_dmabuf_v1_create_params'."]
            #[doc = ""]
            fn enable(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                dmabuf: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Destroys the factory object, but does not affect any other objects."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_output_capture {
    #[doc = ""]
    #[doc = "The global interface exposing Weston screenshooting functionality"]
    #[doc = "intended for single shots."]
    #[doc = ""]
    #[doc = "This is a privileged inteface."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_capture_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid source enum value"]
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
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Source {
            #[doc = "use hardware writeback"]
            Writeback = 0u32,
            #[doc = "copy from framebuffer, desktop area"]
            Framebuffer = 1u32,
            #[doc = "copy whole framebuffer, including borders"]
            FullFramebuffer = 2u32,
            #[doc = "copy from blending space"]
            Blending = 3u32,
        }
        impl TryFrom<u32> for Source {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Writeback),
                    1u32 => Ok(Self::Framebuffer),
                    2u32 => Ok(Self::FullFramebuffer),
                    3u32 => Ok(Self::Blending),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_capture_v1 interface. See the module level documentation for more info"]
        pub trait WestonCaptureV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_capture_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_capture_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let output = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let source = message.uint()?;
                            let capture_source_new_id = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_capture_v1#{}.create({}, {}, {})",
                                sender_id,
                                output,
                                source,
                                capture_source_new_id
                            );
                            self.create(
                                client,
                                sender_id,
                                output,
                                source.try_into()?,
                                capture_source_new_id,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Affects no other protocol objects in any way."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This creates a weston_capture_source_v1 object corresponding to the"]
            #[doc = "given wl_output. The object delivers information for allocating"]
            #[doc = "suitable buffers, and exposes the capture function."]
            #[doc = ""]
            #[doc = "The object will be using the given pixel source for capturing images."]
            #[doc = "If the source is not available, all attempts to capture will fail"]
            #[doc = "gracefully."]
            #[doc = ""]
            #[doc = "'writeback' source will use hardware writeback feature of DRM KMS for"]
            #[doc = "capturing. This may allow hardware planes to remain used"]
            #[doc = "during the capture. This source is often not available."]
            #[doc = ""]
            #[doc = "'framebuffer' source copies the contents of the final framebuffer."]
            #[doc = "Using this source temporarily disables all use of hardware planes and"]
            #[doc = "DRM KMS color pipeline features. This source is always available."]
            #[doc = ""]
            #[doc = "'full_framebuffer' is otherwise the same as 'framebuffer' except it"]
            #[doc = "will include also any borders (decorations) that the framebuffer may"]
            #[doc = "contain."]
            #[doc = ""]
            #[doc = "'blending' source copies the contents of the intermediate blending"]
            #[doc = "buffer, which should be in linear-light format.  Using this source"]
            #[doc = "temporarily disables all use of hardware planes. This source is only"]
            #[doc = "available when a blending buffer exists, e.g. when color management"]
            #[doc = "is active on the output."]
            #[doc = ""]
            #[doc = "If the pixel source is not one of the defined enumeration values,"]
            #[doc = "'invalid_source' protocol error is raised."]
            #[doc = ""]
            fn create(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
                source: Source,
                capture_source_new_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
        }
    }
    #[doc = ""]
    #[doc = "An object representing image capturing functionality for a single"]
    #[doc = "source. When created, it sends the initial events if and only if the"]
    #[doc = "output still exists and the specified pixel source is available on"]
    #[doc = "the output."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_capture_source_v1 {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the wl_buffer is not writable"]
            BadBuffer = 0u32,
            #[doc = "capture requested again before previous retired"]
            Sequence = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BadBuffer),
                    1u32 => Ok(Self::Sequence),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_capture_source_v1 interface. See the module level documentation for more info"]
        pub trait WestonCaptureSourceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_capture_source_v1";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_capture_source_v1#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_capture_source_v1#{}.capture({})",
                                sender_id,
                                buffer
                            );
                            self.capture(client, sender_id, buffer).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "If a capture is on-going on this object, this will cancel it and"]
            #[doc = "make the image buffer contents undefined."]
            #[doc = ""]
            #[doc = "This object is destroyed."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "If the given wl_buffer is compatible, the associated output will go"]
            #[doc = "through a repaint some time after this request has been processed,"]
            #[doc = "and that repaint will execute the capture."]
            #[doc = "Once the capture is complete, 'complete' event is emitted."]
            #[doc = ""]
            #[doc = "If the given wl_buffer is incompatible, the event 'retry' is"]
            #[doc = "emitted."]
            #[doc = ""]
            #[doc = "If the capture fails or the buffer type is unsupported, the event"]
            #[doc = "'failed' is emitted."]
            #[doc = ""]
            #[doc = "The client must wait for one of these events before attempting"]
            #[doc = "'capture' on this object again. If 'capture' is requested again before"]
            #[doc = "any of those events, 'sequence' protocol error is raised."]
            #[doc = ""]
            #[doc = "The wl_buffer object will not emit wl_buffer.release event due to"]
            #[doc = "this request."]
            #[doc = ""]
            #[doc = "The wl_buffer must refer to compositor-writable storage. If buffer"]
            #[doc = "storage is not writable, either the protocol error bad_buffer or"]
            #[doc = "wl_shm.error.invalid_fd is raised."]
            #[doc = ""]
            #[doc = "If the wl_buffer is destroyed before any event is emitted, the buffer"]
            #[doc = "contents become undefined."]
            #[doc = ""]
            #[doc = "A compositor is required to implement capture into wl_shm buffers."]
            #[doc = "Other buffer types may or may not be supported."]
            #[doc = ""]
            fn capture(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                buffer: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event delivers the pixel format that should be used for the"]
            #[doc = "image buffer. Any buffer is incompatible if it does not have"]
            #[doc = "this pixel format."]
            #[doc = ""]
            #[doc = "The format modifier is linear (DRM_FORMAT_MOD_LINEAR)."]
            #[doc = ""]
            #[doc = "This is an initial event, and sent whenever the required format"]
            #[doc = "changes."]
            #[doc = ""]
            fn format(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                drm_format: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_capture_source_v1#{}.format({})",
                        sender_id,
                        drm_format
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(drm_format)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event delivers the size that should be used for the"]
            #[doc = "image buffer. Any buffer is incompatible if it does not have"]
            #[doc = "this size."]
            #[doc = ""]
            #[doc = "Row alignment of the buffer must be 4 bytes, and it must not contain"]
            #[doc = "further row padding. Otherwise the buffer is unsupported."]
            #[doc = ""]
            #[doc = "This is an initial event, and sent whenever the required size"]
            #[doc = "changes."]
            #[doc = ""]
            fn size(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_capture_source_v1#{}.size({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "has successfully completed."]
            #[doc = ""]
            #[doc = "If the buffer used in the shot is a dmabuf, the client also needs to"]
            #[doc = "wait for any implicit fences on it before accessing the contents."]
            #[doc = ""]
            fn complete(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_capture_source_v1#{}.complete()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "cannot succeed due to an incompatible buffer. The client has already"]
            #[doc = "received the events delivering the new buffer parameters. The client"]
            #[doc = "should retry the capture with the new buffer parameters."]
            #[doc = ""]
            fn retry(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_capture_source_v1#{}.retry()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "has failed for reasons other than an incompatible buffer. The reasons"]
            #[doc = "may include: unsupported buffer type, unsupported buffer stride,"]
            #[doc = "unsupported image source, the image source (output) was removed, or"]
            #[doc = "compositor policy denied the capture."]
            #[doc = ""]
            #[doc = "The string 'msg' may contain a human-readable explanation of the"]
            #[doc = "failure to aid debugging."]
            #[doc = ""]
            fn failed(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                msg: Option<String>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_capture_source_v1#{}.failed(\"{}\")",
                        sender_id,
                        msg.as_ref().map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().put_string(msg).build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_test {
    #[doc = ""]
    #[doc = "Internal testing facilities for the weston compositor."]
    #[doc = ""]
    #[doc = "It can't be stressed enough that these should never ever be used"]
    #[doc = "outside of running weston's tests.  The weston-test.so module should"]
    #[doc = "never be installed."]
    #[doc = ""]
    #[doc = "These requests may allow clients to do very bad things."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_test {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid coordinate"]
            TouchUpWithCoordinate = 0u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::TouchUpWithCoordinate),
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
        pub enum Breakpoint {
            #[doc = "after output repaint (filter type: wl_output)"]
            PostRepaint = 0u32,
        }
        impl TryFrom<u32> for Breakpoint {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PostRepaint),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Breakpoint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_test interface. See the module level documentation for more info"]
        pub trait WestonTest: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_test";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            tracing::debug!(
                                "weston_test#{}.move_surface({}, {}, {})",
                                sender_id,
                                surface,
                                x,
                                y
                            );
                            self.move_surface(client, sender_id, surface, x, y).await
                        }
                        1u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_nsec = message.uint()?;
                            let x = message.int()?;
                            let y = message.int()?;
                            tracing::debug!(
                                "weston_test#{}.move_pointer({}, {}, {}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_nsec,
                                x,
                                y
                            );
                            self.move_pointer(
                                client, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec, x, y,
                            )
                            .await
                        }
                        2u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_nsec = message.uint()?;
                            let button = message.int()?;
                            let state = message.uint()?;
                            tracing::debug!(
                                "weston_test#{}.send_button({}, {}, {}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_nsec,
                                button,
                                state
                            );
                            self.send_button(
                                client, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec, button, state,
                            )
                            .await
                        }
                        3u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_nsec = message.uint()?;
                            let axis = message.uint()?;
                            let value = message.fixed()?;
                            tracing::debug!(
                                "weston_test#{}.send_axis({}, {}, {}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_nsec,
                                axis,
                                value
                            );
                            self.send_axis(
                                client, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec, axis, value,
                            )
                            .await
                        }
                        4u16 => {
                            let surface = message.object()?;
                            tracing::debug!(
                                "weston_test#{}.activate_surface({})",
                                sender_id,
                                surface
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.activate_surface(client, sender_id, surface).await
                        }
                        5u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_nsec = message.uint()?;
                            let key = message.uint()?;
                            let state = message.uint()?;
                            tracing::debug!(
                                "weston_test#{}.send_key({}, {}, {}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_nsec,
                                key,
                                state
                            );
                            self.send_key(
                                client, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec, key, state,
                            )
                            .await
                        }
                        6u16 => {
                            let device = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_test#{}.device_release(\"{}\")",
                                sender_id,
                                device
                            );
                            self.device_release(client, sender_id, device).await
                        }
                        7u16 => {
                            let device = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!("weston_test#{}.device_add(\"{}\")", sender_id, device);
                            self.device_add(client, sender_id, device).await
                        }
                        8u16 => {
                            let tv_sec_hi = message.uint()?;
                            let tv_sec_lo = message.uint()?;
                            let tv_nsec = message.uint()?;
                            let touch_id = message.int()?;
                            let x = message.fixed()?;
                            let y = message.fixed()?;
                            let touch_type = message.uint()?;
                            tracing::debug!(
                                "weston_test#{}.send_touch({}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                tv_sec_hi,
                                tv_sec_lo,
                                tv_nsec,
                                touch_id,
                                x,
                                y,
                                touch_type
                            );
                            self.send_touch(
                                client, sender_id, tv_sec_hi, tv_sec_lo, tv_nsec, touch_id, x, y,
                                touch_type,
                            )
                            .await
                        }
                        9u16 => {
                            let breakpoint = message.uint()?;
                            let resource_id = message.uint()?;
                            tracing::debug!(
                                "weston_test#{}.client_break({}, {})",
                                sender_id,
                                breakpoint,
                                resource_id
                            );
                            self.client_break(
                                client,
                                sender_id,
                                breakpoint.try_into()?,
                                resource_id,
                            )
                            .await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn move_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn move_pointer(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                x: i32,
                y: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn send_button(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                button: i32,
                state: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn send_axis(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                axis: u32,
                value: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn activate_surface(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: Option<crate::wire::ObjectId>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn send_key(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                key: u32,
                state: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn device_release(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                device: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn device_add(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                device: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn send_touch(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                touch_id: i32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
                touch_type: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "Request that the compositor pauses execution at a certain point. When"]
            #[doc = "execution is paused, the compositor will signal the shared semaphore"]
            #[doc = "to the client."]
            #[doc = ""]
            fn client_break(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                breakpoint: Breakpoint,
                resource_id: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn pointer_position(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_test#{}.pointer_position({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_fixed(x)
                        .put_fixed(y)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "This is a global singleton interface for Weston internal tests."]
    #[doc = ""]
    #[doc = "This interface allows a test client to trigger compositor-side"]
    #[doc = "test procedures. This is useful for cases, where the actual tests"]
    #[doc = "are in compositor plugins, but they also require the presence of"]
    #[doc = "a particular client."]
    #[doc = ""]
    #[doc = "This interface is implemented by the compositor plugins containing"]
    #[doc = "the testing code."]
    #[doc = ""]
    #[doc = "A test client starts a test with the \"run\" request. It must not send"]
    #[doc = "another \"run\" request until receiving the \"finished\" event. If the"]
    #[doc = "compositor-side test succeeds, the \"finished\" event is sent. If the"]
    #[doc = "compositor-side test fails, the compositor should send the protocol"]
    #[doc = "error \"test_failed\", but it may also exit with an error (e.g. SEGV)."]
    #[doc = ""]
    #[doc = "Unknown test name will raise \"unknown_test\" protocol error."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_test_runner {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "compositor test failed"]
            TestFailed = 0u32,
            #[doc = "unrecognized test name"]
            UnknownTest = 1u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::TestFailed),
                    1u32 => Ok(Self::UnknownTest),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_test_runner interface. See the module level documentation for more info"]
        pub trait WestonTestRunner: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_test_runner";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_test_runner#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let test_name = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_test_runner#{}.run(\"{}\")",
                                sender_id,
                                test_name
                            );
                            self.run(client, sender_id, test_name).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn run(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                test_name: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            fn finished(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_test_runner#{}.finished()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod weston_touch_calibration {
    #[doc = ""]
    #[doc = "This is the global interface for calibrating a touchscreen input"]
    #[doc = "coordinate transformation. It is recommended to make this interface"]
    #[doc = "privileged."]
    #[doc = ""]
    #[doc = "This interface can be used by a client to show a calibration pattern and"]
    #[doc = "receive uncalibrated touch coordinates, facilitating the computation of"]
    #[doc = "a calibration transformation that will align actual touch positions"]
    #[doc = "on screen with their expected coordinates."]
    #[doc = ""]
    #[doc = "Immediately after being bound by a client, the compositor sends the"]
    #[doc = "touch_device events."]
    #[doc = ""]
    #[doc = "The client chooses a touch device from the touch_device events, creates a"]
    #[doc = "wl_surface and then a weston_touch_calibrator for the wl_surface and the"]
    #[doc = "chosen touch device. The client waits for the compositor to send a"]
    #[doc = "configure event before it starts drawing the first calibration pattern."]
    #[doc = "After receiving the configure event, the client will iterate drawing a"]
    #[doc = "pattern, getting touch input via weston_touch_calibrator, and converting"]
    #[doc = "pixel coordinates to expected touch coordinates with"]
    #[doc = "weston_touch_calibrator.convert until it has enough correspondences to"]
    #[doc = "compute the calibration transformation or the compositor cancels the"]
    #[doc = "calibration."]
    #[doc = ""]
    #[doc = "Once the client has successfully computed a new calibration, it can use"]
    #[doc = "weston_touch_calibration.save request to load the new calibration into"]
    #[doc = "the compositor. The compositor may take this new calibration into use and"]
    #[doc = "may write it into persistent storage."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_touch_calibration {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the given wl_surface already has a role"]
            InvalidSurface = 0u32,
            #[doc = "the given device is not valid"]
            InvalidDevice = 1u32,
            #[doc = "a calibrator has already been created"]
            AlreadyExists = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InvalidSurface),
                    1u32 => Ok(Self::InvalidDevice),
                    2u32 => Ok(Self::AlreadyExists),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_touch_calibration interface. See the module level documentation for more info"]
        pub trait WestonTouchCalibration: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_touch_calibration";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_touch_calibration#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let device = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let cal = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_touch_calibration#{}.create_calibrator({}, \"{}\", {})",
                                sender_id,
                                surface,
                                device,
                                cal
                            );
                            self.create_calibrator(client, sender_id, surface, device, cal)
                                .await
                        }
                        2u16 => {
                            let device = message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            let matrix = message.array()?;
                            tracing::debug!(
                                "weston_touch_calibration#{}.save(\"{}\", array[{}])",
                                sender_id,
                                device,
                                matrix.len()
                            );
                            self.save(client, sender_id, device, matrix).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "Destroy the binding to the global interface, does not affect any"]
            #[doc = "objects already created through this interface."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This gives the calibrator role to the surface and ties it with the"]
            #[doc = "given touch input device."]
            #[doc = ""]
            #[doc = "If the surface already has a role, then invalid_surface error is raised."]
            #[doc = ""]
            #[doc = "If the device string is not one advertised with touch_device event's"]
            #[doc = "device argument, then invalid_device error is raised."]
            #[doc = ""]
            #[doc = "If a weston_touch_calibrator protocol object exists in the compositor"]
            #[doc = "already, then already_exists error is raised. This limitation is"]
            #[doc = "compositor-wide and not specific to any particular client."]
            #[doc = ""]
            fn create_calibrator(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
                device: String,
                cal: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This request asks the compositor to save the calibration data for the"]
            #[doc = "given touch input device. The compositor may ignore this request."]
            #[doc = ""]
            #[doc = "If the device string is not one advertised with touch_device event's"]
            #[doc = "device argument, then invalid_device error is raised."]
            #[doc = ""]
            #[doc = "The array must contain exactly six 'float' (the 32-bit floating"]
            #[doc = "point format used by the C language on the system) numbers. For a 3x3"]
            #[doc = "calibration matrix in the form"]
            #[doc = "@code"]
            #[doc = "( a b c )"]
            #[doc = "( d e f )"]
            #[doc = "( 0 0 1 )"]
            #[doc = "@endcode"]
            #[doc = "the array must contain the values { a, b, c, d, e, f }. For the"]
            #[doc = "definition of the coordinate spaces, see"]
            #[doc = "libinput_device_config_calibration_set_matrix()."]
            #[doc = ""]
            fn save(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                device: String,
                matrix: Vec<u8>,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "When a client binds to weston_touch_calibration, one touch_device event"]
            #[doc = "is sent for each touchscreen that is available to be calibrated. This"]
            #[doc = "is the only time the event is sent. Touch devices added in the"]
            #[doc = "compositor will not generate events for existing"]
            #[doc = "weston_touch_calibration objects."]
            #[doc = ""]
            #[doc = "An event carries the touch device identification and the associated"]
            #[doc = "output or head (display connector) name."]
            #[doc = ""]
            #[doc = "On platforms using udev, the device identification is the udev sys"]
            #[doc = "path. It is an absolute path and starts with the sys mount point."]
            #[doc = ""]
            fn touch_device(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                device: String,
                head: String,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_calibration#{}.touch_device(\"{}\", \"{}\")",
                        sender_id,
                        device,
                        head
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_string(Some(device))
                        .put_string(Some(head))
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[doc = ""]
    #[doc = "On creation, this object is tied to a specific touch device. The"]
    #[doc = "compositor sends a configure event which the client must obey with the"]
    #[doc = "associated wl_surface."]
    #[doc = ""]
    #[doc = "Once the client has committed content to the surface, the compositor can"]
    #[doc = "grab the touch input device, prevent it from emitting normal touch"]
    #[doc = "events, show the surface on the correct output, and relay input events"]
    #[doc = "from the touch device via this protocol object."]
    #[doc = ""]
    #[doc = "Touch events from other touch devices than the one tied to this object"]
    #[doc = "must generate wrong_touch events on at least touch-down and must not"]
    #[doc = "generate normal or calibration touch events."]
    #[doc = ""]
    #[doc = "At any time, the compositor can choose to cancel the calibration"]
    #[doc = "procedure by sending the cancel_calibration event. This should also be"]
    #[doc = "used if the touch device disappears or anything else prevents the"]
    #[doc = "calibration from continuing on the compositor side."]
    #[doc = ""]
    #[doc = "If the wl_surface is destroyed, the compositor must cancel the"]
    #[doc = "calibration."]
    #[doc = ""]
    #[doc = "The touch event coordinates and conversion results are delivered in"]
    #[doc = "calibration units. The calibration units cover the device coordinate"]
    #[doc = "range exactly. Calibration units are in the closed interval [0.0, 1.0]"]
    #[doc = "mapped into 32-bit unsigned integers. An integer can be converted into a"]
    #[doc = "real value by dividing by 2^32-1. A calibration matrix must be computed"]
    #[doc = "from the [0.0, 1.0] real values, but the matrix elements do not need to"]
    #[doc = "fall into that range."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod weston_touch_calibrator {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface size does not match"]
            BadSize = 0u32,
            #[doc = "requested operation is not possible without mapping the surface"]
            NotMapped = 1u32,
            #[doc = "surface-local coordinates are out of bounds"]
            BadCoordinates = 2u32,
        }
        impl TryFrom<u32> for Error {
            type Error = crate::wire::DecodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::BadSize),
                    1u32 => Ok(Self::NotMapped),
                    2u32 => Ok(Self::BadCoordinates),
                    _ => Err(crate::wire::DecodeError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the weston_touch_calibrator interface. See the module level documentation for more info"]
        pub trait WestonTouchCalibrator: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_touch_calibrator";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            tracing::debug!("weston_touch_calibrator#{}.destroy()", sender_id,);
                            let result = self.destroy(client, sender_id).await;
                            client.remove(sender_id);
                            result
                        }
                        1u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            let reply = message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?;
                            tracing::debug!(
                                "weston_touch_calibrator#{}.convert({}, {}, {})",
                                sender_id,
                                x,
                                y,
                                reply
                            );
                            self.convert(client, sender_id, x, y, reply).await
                        }
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This unmaps the surface if it was mapped. The input device grab"]
            #[doc = "is dropped, if it was present. The surface loses its role as a"]
            #[doc = "calibrator."]
            #[doc = ""]
            fn destroy(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This request asks the compositor to convert the surface-local"]
            #[doc = "coordinates into the expected touch input coordinates appropriate for"]
            #[doc = "the associated touch device. The intention is that a client uses this"]
            #[doc = "request to convert marker positions that the user is supposed to touch"]
            #[doc = "during calibration."]
            #[doc = ""]
            #[doc = "If the compositor has cancelled the calibration, the conversion result"]
            #[doc = "shall be zeroes and no errors will be raised."]
            #[doc = ""]
            #[doc = "The coordinates given as arguments to this request are relative to"]
            #[doc = "the associated wl_surface."]
            #[doc = ""]
            #[doc = "If a client asks for conversion before it has committed valid"]
            #[doc = "content to the wl_surface, the not_mapped error is raised."]
            #[doc = ""]
            #[doc = "If the coordinates x, y are outside of the wl_surface content, the"]
            #[doc = "bad_coordinates error is raised."]
            #[doc = ""]
            fn convert(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: i32,
                y: i32,
                reply: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send;
            #[doc = ""]
            #[doc = "This event tells the client what size to make the surface. The client"]
            #[doc = "must obey the size exactly on the next commit with a wl_buffer."]
            #[doc = ""]
            #[doc = "This event shall be sent once as a response to creating a"]
            #[doc = "weston_touch_calibrator object."]
            #[doc = ""]
            fn configure(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_calibrator#{}.configure({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "This is sent when the compositor wants to cancel the calibration and"]
            #[doc = "drop the touch device grab. The compositor unmaps the surface, if it"]
            #[doc = "was mapped."]
            #[doc = ""]
            #[doc = "The weston_touch_calibrator object will not send any more events. The"]
            #[doc = "client should destroy it."]
            #[doc = ""]
            fn cancel_calibration(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_calibrator#{}.cancel_calibration()",
                        sender_id,
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 1u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "For whatever reason, a touch event resulting from a user action cannot"]
            #[doc = "be used for calibration. The client should show feedback to the user"]
            #[doc = "that the touch was rejected."]
            #[doc = ""]
            #[doc = "Possible causes for this event include the user touching a wrong"]
            #[doc = "touchscreen when there are multiple ones present. This is particularly"]
            #[doc = "useful when the touchscreens are cloned and there is no other way to"]
            #[doc = "identify which screen the user should be touching."]
            #[doc = ""]
            #[doc = "Another cause could be a touch device that sends coordinates beyond its"]
            #[doc = "declared range. If motion takes a touch point outside the range, the"]
            #[doc = "compositor should also send 'cancel' event to undo the touch-down."]
            #[doc = ""]
            fn invalid_touch(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_touch_calibrator#{}.invalid_touch()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 2u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "A new touch point has appeared on the surface. This touch point is"]
            #[doc = "assigned a unique ID. Future events from this touch point reference"]
            #[doc = "this ID. The ID ceases to be valid after a touch up event and may be"]
            #[doc = "reused in the future."]
            #[doc = ""]
            #[doc = "For the coordinate units, see weston_touch_calibrator."]
            #[doc = ""]
            fn down(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                id: i32,
                x: u32,
                y: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_calibrator#{}.down({}, {}, {}, {})",
                        sender_id,
                        time,
                        id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(time)
                        .put_int(id)
                        .put_uint(x)
                        .put_uint(y)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 3u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "The touch point has disappeared. No further events will be sent for"]
            #[doc = "this touch point and the touch point's ID is released and may be"]
            #[doc = "reused in a future touch down event."]
            #[doc = ""]
            fn up(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                id: i32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_calibrator#{}.up({}, {})",
                        sender_id,
                        time,
                        id
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(time)
                        .put_int(id)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 4u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "A touch point has changed coordinates."]
            #[doc = ""]
            #[doc = "For the coordinate units, see weston_touch_calibrator."]
            #[doc = ""]
            fn motion(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                time: u32,
                id: i32,
                x: u32,
                y: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_calibrator#{}.motion({}, {}, {}, {})",
                        sender_id,
                        time,
                        id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(time)
                        .put_int(id)
                        .put_uint(x)
                        .put_uint(y)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 5u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Indicates the end of a set of events that logically belong together."]
            #[doc = "A client is expected to accumulate the data in all events within the"]
            #[doc = "frame before proceeding."]
            #[doc = ""]
            #[doc = "A wl_touch.frame terminates at least one event but otherwise no"]
            #[doc = "guarantee is provided about the set of events within a frame. A client"]
            #[doc = "must assume that any state not updated in a frame is unchanged from the"]
            #[doc = "previously known state."]
            #[doc = ""]
            fn frame(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_touch_calibrator#{}.frame()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 6u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
            #[doc = ""]
            #[doc = "Sent if the compositor decides the touch stream is a global"]
            #[doc = "gesture. No further events are sent to the clients from that"]
            #[doc = "particular gesture. Touch cancellation applies to all touch points"]
            #[doc = "currently active on this client's surface. The client is"]
            #[doc = "responsible for finalizing the touch points, future touch points on"]
            #[doc = "this surface may reuse the touch point ID."]
            #[doc = ""]
            fn cancel(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!("-> weston_touch_calibrator#{}.cancel()", sender_id,);
                    let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 7u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod weston_touch_coordinate {
        #[allow(unused)]
        use futures_util::SinkExt;
        #[allow(unused)]
        use std::os::fd::AsRawFd;
        #[doc = "Trait to implement the weston_touch_coordinate interface. See the module level documentation for more info"]
        pub trait WestonTouchCoordinate: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_touch_coordinate";
            const VERSION: u32 = 1u32;
            fn handle_request(
                &self,
                _client: &mut crate::server::Client,
                _sender_id: crate::wire::ObjectId,
                message: &mut crate::wire::Message,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(crate::server::error::Error::UnknownOpcode(opcode)),
                    }
                }
            }
            #[doc = ""]
            #[doc = "This event returns the conversion result from surface coordinates to"]
            #[doc = "the expected touch device coordinates."]
            #[doc = ""]
            #[doc = "For details, see weston_touch_calibrator.convert. For the coordinate"]
            #[doc = "units, see weston_touch_calibrator."]
            #[doc = ""]
            #[doc = "This event destroys the weston_touch_coordinate object."]
            #[doc = ""]
            fn result(
                &self,
                client: &mut crate::server::Client,
                sender_id: crate::wire::ObjectId,
                x: u32,
                y: u32,
            ) -> impl Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(
                        "-> weston_touch_coordinate#{}.result({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) = crate::wire::PayloadBuilder::new()
                        .put_uint(x)
                        .put_uint(y)
                        .build();
                    client
                        .send_message(crate::wire::Message::new(sender_id, 0u16, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        }
    }
}
