#![allow(unused)]
#![allow(async_fn_in_trait)]
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
pub mod color_management_v1 {
    #[doc = "A global interface used for getting color management extensions for"]
    #[doc = "wl_surface and wl_output objects, and for creating client defined image"]
    #[doc = "description objects. The extension interfaces allow"]
    #[doc = "getting the image description of outputs and setting the image"]
    #[doc = "description of surfaces."]
    pub mod xx_color_manager_v4 {
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
        #[doc = "See the ICC.1:2022 specification from the International Color Consortium"]
        #[doc = "for more details about rendering intents."]
        #[doc = ""]
        #[doc = "The principles of ICC defined rendering intents apply with all types of"]
        #[doc = "image descriptions, not only those with ICC file profiles."]
        #[doc = ""]
        #[doc = "Compositors must support the perceptual rendering intent. Other"]
        #[doc = "rendering intents are optional."]
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
        #[doc = "Named color primaries used to encode well-known sets of primaries. H.273"]
        #[doc = "is the authority, when it comes to the exact values of primaries and"]
        #[doc = "authoritative specifications, where an equivalent code point exists."]
        #[doc = ""]
        #[doc = "Descriptions do list the specifications for convenience."]
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
        #[doc = "Named transfer functions used to encode well-known transfer"]
        #[doc = "characteristics. H.273 is the authority, when it comes to the exact"]
        #[doc = "formulas and authoritative specifications, where an equivalent code"]
        #[doc = "point exists."]
        #[doc = ""]
        #[doc = "Descriptions do list the specifications for convenience."]
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
        #[doc = "Trait to implement the xx_color_manager_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagerV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_manager_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xx_color_manager_v4#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xx_color_manager_v4#{}.get_output()", object.id);
                        self.get_output(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("xx_color_manager_v4#{}.get_surface()", object.id);
                        self.get_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("xx_color_manager_v4#{}.get_feedback_surface()", object.id);
                        self.get_feedback_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4u16 => {
                        tracing::debug!("xx_color_manager_v4#{}.new_icc_creator()", object.id);
                        self.new_icc_creator(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!(
                            "xx_color_manager_v4#{}.new_parametric_creator()",
                            object.id
                        );
                        self.new_parametric_creator(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the xx_color_manager_v4 object. This does not affect any other"]
            #[doc = "objects in any way."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "This creates a new xx_color_management_output_v4 object for the"]
            #[doc = "given wl_output."]
            #[doc = ""]
            #[doc = "See the xx_color_management_output_v4 interface for more details."]
            async fn get_output(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "If a xx_color_management_surface_v4 object already exists for the given"]
            #[doc = "wl_surface, the protocol error surface_exists is raised."]
            #[doc = ""]
            #[doc = "This creates a new color xx_color_management_surface_v4 object for the"]
            #[doc = "given wl_surface."]
            #[doc = ""]
            #[doc = "See the xx_color_management_surface_v4 interface for more details."]
            async fn get_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This creates a new color xx_color_management_feedback_surface_v4 object"]
            #[doc = "for the given wl_surface."]
            #[doc = ""]
            #[doc = "See the xx_color_management_feedback_surface_v4 interface for more"]
            #[doc = "details."]
            async fn get_feedback_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Makes a new ICC-based image description creator object with all"]
            #[doc = "properties initially unset. The client can then use the object's"]
            #[doc = "interface to define all the required properties for an image description"]
            #[doc = "and finally create a xx_image_description_v4 object."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "xx_color_manager_v4.feature.icc_v2_v4."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            async fn new_icc_creator(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                obj: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Makes a new parametric image description creator object with all"]
            #[doc = "properties initially unset. The client can then use the object's"]
            #[doc = "interface to define all the required properties for an image description"]
            #[doc = "and finally create a xx_image_description_v4 object."]
            #[doc = ""]
            #[doc = "This request can be used when the compositor advertises"]
            #[doc = "xx_color_manager_v4.feature.parametric."]
            #[doc = "Otherwise this request raises the protocol error unsupported_feature."]
            async fn new_parametric_creator(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                obj: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each rendering intent the compositor supports."]
            async fn supported_intent(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                render_intent: RenderIntent,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_color_manager_v4#{}.supported_intent()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(render_intent as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each compositor supported feature listed in the enumeration."]
            async fn supported_feature(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                feature: Feature,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_color_manager_v4#{}.supported_feature()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(feature as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each named transfer function the compositor supports with the"]
            #[doc = "parametric image description creator."]
            async fn supported_tf_named(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tf: TransferFunction,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_color_manager_v4#{}.supported_tf_named()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(tf as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "When this object is created, it shall immediately send this event once"]
            #[doc = "for each named set of primaries the compositor supports with the"]
            #[doc = "parametric image description creator."]
            async fn supported_primaries_named(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                primaries: Primaries,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_color_manager_v4#{}.supported_primaries_named()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(primaries as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A xx_color_management_output_v4 describes the color properties of an"]
    #[doc = "output."]
    #[doc = ""]
    #[doc = "The xx_color_management_output_v4 is associated with the wl_output global"]
    #[doc = "underlying the wl_output object. Therefore the client destroying the"]
    #[doc = "wl_output object has no impact, but the compositor removing the output"]
    #[doc = "global makes the xx_color_management_output_v4 object inert."]
    pub mod xx_color_management_output_v4 {
        #[doc = "Trait to implement the xx_color_management_output_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagementOutputV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_management_output_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xx_color_management_output_v4#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "xx_color_management_output_v4#{}.get_image_description()",
                            object.id
                        );
                        self.get_image_description(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the color xx_color_management_output_v4 object. This does not"]
            #[doc = "affect any remaining protocol objects."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn get_image_description(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                image_description: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event is sent whenever the image description of the output changed,"]
            #[doc = "followed by one wl_output.done event common to output events across all"]
            #[doc = "extensions."]
            #[doc = ""]
            #[doc = "If the client wants to use the updated image description, it needs to do"]
            #[doc = "get_image_description again, because image description objects are"]
            #[doc = "immutable."]
            async fn image_description_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_color_management_output_v4#{}.image_description_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "A xx_color_management_surface_v4 allows the client to set the color"]
    #[doc = "space and HDR properties of a surface."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with the xx_color_management_surface_v4 is"]
    #[doc = "destroyed, the xx_color_management_surface_v4 object becomes inert."]
    pub mod xx_color_management_surface_v4 {
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
        #[doc = "Trait to implement the xx_color_management_surface_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagementSurfaceV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_management_surface_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xx_color_management_surface_v4#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "xx_color_management_surface_v4#{}.set_image_description()",
                            object.id
                        );
                        self.set_image_description(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "xx_color_management_surface_v4#{}.unset_image_description()",
                            object.id
                        );
                        self.unset_image_description(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the xx_color_management_surface_v4 object and do the same as"]
            #[doc = "unset_image_description."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn set_image_description(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                image_description: crate::wire::ObjectId,
                render_intent: super::super::color_management_v1::xx_color_manager_v4::RenderIntent,
            ) -> crate::server::Result<()>;
            #[doc = "This request removes any image description from the surface. See"]
            #[doc = "set_image_description for how a compositor handles a surface without"]
            #[doc = "an image description. This is double-buffered state, see"]
            #[doc = "wl_surface.commit."]
            async fn unset_image_description(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "A xx_color_management_feedback_surface_v4 allows the client to get the"]
    #[doc = "preferred color description of a surface."]
    #[doc = ""]
    #[doc = "If the wl_surface associated with this object is destroyed, the"]
    #[doc = "xx_color_management_feedback_surface_v4 object becomes inert."]
    pub mod xx_color_management_feedback_surface_v4 {
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
        #[doc = "Trait to implement the xx_color_management_feedback_surface_v4 interface. See the module level documentation for more info"]
        pub trait XxColorManagementFeedbackSurfaceV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_color_management_feedback_surface_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "xx_color_management_feedback_surface_v4#{}.destroy()",
                            object.id
                        );
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "xx_color_management_feedback_surface_v4#{}.get_preferred()",
                            object.id
                        );
                        self.get_preferred(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the xx_color_management_feedback_surface_v4 object."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn get_preferred(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                image_description: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
            async fn preferred_changed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_color_management_feedback_surface_v4#{}.preferred_changed()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
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
    pub mod xx_image_description_creator_icc_v4 {
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
        #[doc = "Trait to implement the xx_image_description_creator_icc_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionCreatorIccV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_creator_icc_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_icc_v4#{}.create()",
                            object.id
                        );
                        self.create(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_icc_v4#{}.set_icc_file()",
                            object.id
                        );
                        self.set_icc_file(
                            object,
                            client,
                            message.fd()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                image_description: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
            async fn set_icc_file(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                icc_profile: rustix::fd::OwnedFd,
                offset: u32,
                length: u32,
            ) -> crate::server::Result<()>;
        }
    }
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
    pub mod xx_image_description_creator_params_v4 {
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
        #[doc = "Trait to implement the xx_image_description_creator_params_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionCreatorParamsV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_creator_params_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.create()",
                            object.id
                        );
                        self.create(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_tf_named()",
                            object.id
                        );
                        self.set_tf_named(object, client, message.uint()?.try_into()?)
                            .await
                    }
                    2u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_tf_power()",
                            object.id
                        );
                        self.set_tf_power(object, client, message.uint()?).await
                    }
                    3u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_primaries_named()",
                            object.id
                        );
                        self.set_primaries_named(object, client, message.uint()?.try_into()?)
                            .await
                    }
                    4u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_primaries()",
                            object.id
                        );
                        self.set_primaries(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_luminances()",
                            object.id
                        );
                        self.set_luminances(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    6u16 => {
                        tracing :: debug ! ("xx_image_description_creator_params_v4#{}.set_mastering_display_primaries()" , object . id);
                        self.set_mastering_display_primaries(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    7u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_mastering_luminance()",
                            object.id
                        );
                        self.set_mastering_luminance(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_max_cll()",
                            object.id
                        );
                        self.set_max_cll(object, client, message.uint()?).await
                    }
                    9u16 => {
                        tracing::debug!(
                            "xx_image_description_creator_params_v4#{}.set_max_fall()",
                            object.id
                        );
                        self.set_max_fall(object, client, message.uint()?).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                image_description: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
            async fn set_tf_named(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tf: super::super::color_management_v1::xx_color_manager_v4::TransferFunction,
            ) -> crate::server::Result<()>;
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
            async fn set_tf_power(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                eexp: u32,
            ) -> crate::server::Result<()>;
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
            async fn set_primaries_named(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                primaries: super::super::color_management_v1::xx_color_manager_v4::Primaries,
            ) -> crate::server::Result<()>;
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
            async fn set_primaries(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::server::Result<()>;
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
            async fn set_luminances(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                min_lum: u32,
                max_lum: u32,
                reference_lum: u32,
            ) -> crate::server::Result<()>;
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
            async fn set_mastering_display_primaries(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::server::Result<()>;
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
            async fn set_mastering_luminance(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                min_lum: u32,
                max_lum: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the maximum content light level (max_cll) as defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This can only be set when set_tf_cicp is used to set the transfer"]
            #[doc = "characteristic to Rec. ITU-R BT.2100-2 perceptual quantization system."]
            #[doc = "Otherwise, 'create' request shall raise inconsistent_set protocol"]
            #[doc = "error."]
            #[doc = ""]
            #[doc = "max_cll is undefined by default."]
            async fn set_max_cll(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                max_cll: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Sets the maximum frame-average light level (max_fall) as defined by"]
            #[doc = "CTA-861-H."]
            #[doc = ""]
            #[doc = "This can only be set when set_tf_cicp is used to set the transfer"]
            #[doc = "characteristic to Rec. ITU-R BT.2100-2 perceptual quantization system."]
            #[doc = "Otherwise, 'create' request shall raise inconsistent_set protocol error."]
            #[doc = ""]
            #[doc = "max_fall is undefined by default."]
            async fn set_max_fall(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                max_fall: u32,
            ) -> crate::server::Result<()>;
        }
    }
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
    pub mod xx_image_description_v4 {
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
        #[doc = "Trait to implement the xx_image_description_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("xx_image_description_v4#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("xx_image_description_v4#{}.get_information()", object.id);
                        self.get_information(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy this object. It is safe to destroy an object which is not ready."]
            #[doc = ""]
            #[doc = "Destroying a xx_image_description_v4 object has no side-effects, not"]
            #[doc = "even if a xx_color_management_surface_v4.set_image_description has not"]
            #[doc = "yet been followed by a wl_surface.commit."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Creates a xx_image_description_info_v4 object which delivers the"]
            #[doc = "information that makes up the image description."]
            #[doc = ""]
            #[doc = "Not all image description protocol objects allow get_information"]
            #[doc = "request. Whether it is allowed or not is defined by the request that"]
            #[doc = "created the object. If get_information is not allowed, the protocol"]
            #[doc = "error no_information is raised."]
            async fn get_information(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                information: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
            async fn failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                cause: Cause,
                msg: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_v4#{}.failed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(cause as u32)
                    .put_string(Some(msg))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
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
            async fn ready(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                identity: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_v4#{}.ready()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(identity)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "Sends all matching events describing an image description object exactly"]
    #[doc = "once and finally sends the 'done' event."]
    #[doc = ""]
    #[doc = "Once a xx_image_description_info_v4 object has delivered a 'done' event it"]
    #[doc = "is automatically destroyed."]
    #[doc = ""]
    #[doc = "Every xx_image_description_info_v4 created from the same"]
    #[doc = "xx_image_description_v4 shall always return the exact same data."]
    pub mod xx_image_description_info_v4 {
        #[doc = "Trait to implement the xx_image_description_info_v4 interface. See the module level documentation for more info"]
        pub trait XxImageDescriptionInfoV4: crate::server::Dispatcher {
            const INTERFACE: &'static str = "xx_image_description_info_v4";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Signals the end of information events and destroys the object."]
            async fn done(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_info_v4#{}.done()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The icc argument provides a file descriptor to the client which may be"]
            #[doc = "memory-mapped to provide the ICC profile matching the image description."]
            #[doc = "The fd is read-only, and if mapped then it must be mapped with"]
            #[doc = "MAP_PRIVATE by the client."]
            #[doc = ""]
            #[doc = "The ICC profile version and other details are determined by the"]
            #[doc = "compositor. There is no provision for a client to ask for a specific"]
            #[doc = "kind of a profile."]
            async fn icc_file(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                icc: rustix::fd::OwnedFd,
                icc_size: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_info_v4#{}.icc_file()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fd(icc)
                    .put_uint(icc_size)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Delivers the primary color volume primaries and white point using CIE"]
            #[doc = "1931 xy chromaticity coordinates."]
            #[doc = ""]
            #[doc = "Each coordinate value is multiplied by 10000 to get the argument value"]
            #[doc = "to carry precision of 4 decimals."]
            async fn primaries(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_info_v4#{}.primaries()", object.id);
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
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Delivers the primary color volume primaries and white point using an"]
            #[doc = "explicitly enumerated named set."]
            async fn primaries_named(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                primaries: super::super::color_management_v1::xx_color_manager_v4::Primaries,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_image_description_info_v4#{}.primaries_named()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(primaries as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The color component transfer characteristic of this image description is"]
            #[doc = "a pure power curve. This event provides the exponent of the power"]
            #[doc = "function. This curve represents the conversion from electrical to"]
            #[doc = "optical pixel or color values."]
            #[doc = ""]
            #[doc = "The curve exponent has been multiplied by 10000 to get the argument eexp"]
            #[doc = "value to carry the precision of 4 decimals."]
            async fn tf_power(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                eexp: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_info_v4#{}.tf_power()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(eexp).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Delivers the transfer characteristic using an explicitly enumerated"]
            #[doc = "named function."]
            async fn tf_named(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tf: super::super::color_management_v1::xx_color_manager_v4::TransferFunction,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_info_v4#{}.tf_named()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(tf as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Delivers the primary color volume luminance range and the reference"]
            #[doc = "white luminance level."]
            #[doc = ""]
            #[doc = "The minimum luminance is multiplied by 10000 to get the argument"]
            #[doc = "'min_lum' value and carries precision of 4 decimals. The maximum"]
            #[doc = "luminance and reference white luminance values are unscaled."]
            async fn luminances(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                min_lum: u32,
                max_lum: u32,
                reference_lum: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> xx_image_description_info_v4#{}.luminances()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(min_lum)
                    .put_uint(max_lum)
                    .put_uint(reference_lum)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
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
            async fn target_primaries(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                r_x: i32,
                r_y: i32,
                g_x: i32,
                g_y: i32,
                b_x: i32,
                b_y: i32,
                w_x: i32,
                w_y: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_image_description_info_v4#{}.target_primaries()",
                    object.id
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
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Provides the luminance range that the image description is targeting as"]
            #[doc = "the minimum and maximum absolute luminance L. This is compatible with"]
            #[doc = "the SMPTE ST 2086 definition of HDR static metadata."]
            #[doc = ""]
            #[doc = "This luminance range is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            #[doc = ""]
            #[doc = "Min L value is multiplied by 10000 to get the argument min_lum value and"]
            #[doc = "carry precision of 4 decimals. Max L value is unscaled for max_lum."]
            async fn target_luminance(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                min_lum: u32,
                max_lum: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_image_description_info_v4#{}.target_luminance()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(min_lum)
                    .put_uint(max_lum)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 8u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Provides the targeted max_cll of the image description. max_cll is"]
            #[doc = "defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This luminance is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            async fn target_max_cll(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                max_cll: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_image_description_info_v4#{}.target_max_cll()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(max_cll).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 9u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Provides the targeted max_fall of the image description. max_fall is"]
            #[doc = "defined by CTA-861-H."]
            #[doc = ""]
            #[doc = "This luminance is only theoretical and may not correspond to the"]
            #[doc = "luminance of light emitted on an actual display."]
            async fn target_max_fall(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                max_fall: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> xx_image_description_info_v4#{}.target_max_fall()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(max_fall)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 10u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod ivi_application {
    pub mod ivi_surface {
        #[doc = "Trait to implement the ivi_surface interface. See the module level documentation for more info"]
        pub trait IviSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_surface";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ivi_surface#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This removes the link from ivi_id to wl_surface and destroys ivi_surface."]
            #[doc = "The ID, ivi_id, is free and can be used for surface_create again."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn configure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> ivi_surface#{}.configure()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "This interface is exposed as a global singleton."]
    #[doc = "This interface is implemented by servers that provide IVI-style user interfaces."]
    #[doc = "It allows clients to associate an ivi_surface with wl_surface."]
    pub mod ivi_application {
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
        #[doc = "Trait to implement the ivi_application interface. See the module level documentation for more info"]
        pub trait IviApplication: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_application";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ivi_application#{}.surface_create()", object.id);
                        self.surface_create(
                            object,
                            client,
                            message.uint()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
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
            async fn surface_create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                ivi_id: u32,
                surface: crate::wire::ObjectId,
                id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod ivi_hmi_controller {
    pub mod ivi_hmi_controller {
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
        #[doc = "Trait to implement the ivi_hmi_controller interface. See the module level documentation for more info"]
        pub trait IviHmiController: crate::server::Dispatcher {
            const INTERFACE: &'static str = "ivi_hmi_controller";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("ivi_hmi_controller#{}.ui_ready()", object.id);
                        self.ui_ready(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("ivi_hmi_controller#{}.workspace_control()", object.id);
                        self.workspace_control(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("ivi_hmi_controller#{}.switch_mode()", object.id);
                        self.switch_mode(object, client, message.uint()?).await
                    }
                    3u16 => {
                        tracing::debug!("ivi_hmi_controller#{}.home()", object.id);
                        self.home(object, client, message.uint()?).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn ui_ready(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Reference protocol to control a surface by server."]
            #[doc = "To control a surface by server, it gives seat to the server"]
            #[doc = "to e.g. control Home screen. Home screen has several workspaces"]
            #[doc = "to group launchers of wayland application. These workspaces"]
            #[doc = "are drawn on a horizontally long surface to be controlled"]
            #[doc = "by motion of input device. E.g. A motion from right to left"]
            #[doc = "happens, the viewport of surface is controlled in the ivi-shell"]
            #[doc = "by using ivi-layout. client can recognizes the end of controlling"]
            #[doc = "by event \"workspace_end_control\"."]
            async fn workspace_control(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                seat: crate::wire::ObjectId,
                serial: u32,
            ) -> crate::server::Result<()>;
            #[doc = "hmi-controller loaded to ivi-shall implements 4 types of layout"]
            #[doc = "as a reference; tiling, side by side, full_screen, and random."]
            async fn switch_mode(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                layout_mode: u32,
            ) -> crate::server::Result<()>;
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
            async fn home(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                home: u32,
            ) -> crate::server::Result<()>;
            async fn workspace_end_control(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                is_controlled: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> ivi_hmi_controller#{}.workspace_end_control()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(is_controlled)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod text_cursor_position {
    pub mod text_cursor_position {
        #[doc = "Trait to implement the text_cursor_position interface. See the module level documentation for more info"]
        pub trait TextCursorPosition: crate::server::Dispatcher {
            const INTERFACE: &'static str = "text_cursor_position";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("text_cursor_position#{}.notify()", object.id);
                        self.notify(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.fixed()?,
                            message.fixed()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn notify(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
        }
    }
}
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
pub mod weston_content_protection {
    #[doc = "The global interface weston_content_protection is used for exposing the"]
    #[doc = "content protection capabilities to a client. It provides a way for clients"]
    #[doc = "to request their wl_surface contents to not be displayed on an output"]
    #[doc = "below their required level of content-protection."]
    #[doc = "Using this interface clients can request for a weston_protected_surface"]
    #[doc = "which is an extension to the wl_surface to provide content-protection, and"]
    #[doc = "set the censored-visibility on the non-secured-outputs."]
    pub mod weston_content_protection {
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
        #[doc = "Trait to implement the weston_content_protection interface. See the module level documentation for more info"]
        pub trait WestonContentProtection: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_content_protection";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_content_protection#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_content_protection#{}.get_protection()", object.id);
                        self.get_protection(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Informs the server that the client will not be using this"]
            #[doc = "protocol object anymore. This does not affect any other objects,"]
            #[doc = "protected_surface objects included."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Instantiate an interface extension for the given wl_surface to"]
            #[doc = "provide surface protection. If the given wl_surface already has"]
            #[doc = "a weston_protected_surface associated, the surface_exists protocol"]
            #[doc = "error is raised."]
            async fn get_protection(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                id: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
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
    pub mod weston_protected_surface {
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
        #[doc = "Description of a particular type of content protection."]
        #[doc = ""]
        #[doc = "A server may not necessarily support all of these types."]
        #[doc = ""]
        #[doc = "Note that there is no ordering between enum members unless specified."]
        #[doc = "Over time, different types of content protection may be added, which"]
        #[doc = "may be considered less secure than what is already here."]
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
        #[doc = "Trait to implement the weston_protected_surface interface. See the module level documentation for more info"]
        pub trait WestonProtectedSurface: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_protected_surface";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_protected_surface#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_protected_surface#{}.set_type()", object.id);
                        self.set_type(object, client, message.uint()?.try_into()?)
                            .await
                    }
                    2u16 => {
                        tracing::debug!("weston_protected_surface#{}.enforce()", object.id);
                        self.enforce(object, client).await
                    }
                    3u16 => {
                        tracing::debug!("weston_protected_surface#{}.relax()", object.id);
                        self.relax(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "If the protected_surface is destroyed, the wl_surface desired protection"]
            #[doc = "level returns to unprotected, as if set_type request was sent with type"]
            #[doc = "as 'unprotected'."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn set_type(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                r#type: Type,
            ) -> crate::server::Result<()>;
            #[doc = "Censor the visibility of the wl_surface contents on non-secure outputs."]
            #[doc = "See weston_protected_surface for the description."]
            #[doc = ""]
            #[doc = "The force constrain mode is double-buffered, see wl_surface.commit"]
            async fn enforce(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Do not enforce censored-visibility of the wl_surface contents on"]
            #[doc = "non-secure-outputs. See weston_protected_surface for the description."]
            #[doc = ""]
            #[doc = "The relax mode is selected by default, if no explicit request is made"]
            #[doc = "for enforcing the censored-visibility."]
            #[doc = ""]
            #[doc = "The relax mode is double-buffered, see wl_surface.commit"]
            async fn relax(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn status(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                r#type: Type,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_protected_surface#{}.status()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(r#type as u32)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod weston_debug {
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
    pub mod weston_debug_v1 {
        #[doc = "Trait to implement the weston_debug_v1 interface. See the module level documentation for more info"]
        pub trait WestonDebugV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_debug_v1";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_debug_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_debug_v1#{}.subscribe()", object.id);
                        self.subscribe(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.fd()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the factory object, but does not affect any other objects."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn subscribe(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                streamfd: rustix::fd::OwnedFd,
                stream: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Advertises an available debug scope which the client may be able to"]
            #[doc = "bind to. No information is provided by the server about the content"]
            #[doc = "contained within the debug streams provided by the scope, once a"]
            #[doc = "client has subscribed."]
            async fn available(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                name: String,
                description: Option<String>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_debug_v1#{}.available()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .put_string(description)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "Represents one subscribed debug stream, created with"]
    #[doc = "weston_debug_v1.subscribe. When the object is created, it is associated"]
    #[doc = "with a given file descriptor. The server will continue writing to the"]
    #[doc = "file descriptor until the object is destroyed or the server sends an"]
    #[doc = "event through the object."]
    pub mod weston_debug_stream_v1 {
        #[doc = "Trait to implement the weston_debug_stream_v1 interface. See the module level documentation for more info"]
        pub trait WestonDebugStreamV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_debug_stream_v1";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_debug_stream_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroys the object, which causes the server to stop writing into"]
            #[doc = "and closes the associated file descriptor if it was not closed"]
            #[doc = "already."]
            #[doc = ""]
            #[doc = "Use a wl_display.sync if the clients needs to guarantee the file"]
            #[doc = "descriptor is closed before continuing."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The server has successfully finished writing to and has closed the"]
            #[doc = "associated file descriptor."]
            #[doc = ""]
            #[doc = "This event is delivered only for one-shot debug streams where the"]
            #[doc = "server dumps some data and stop. This is never delivered for"]
            #[doc = "continuous debbug streams because they by definition never complete."]
            async fn complete(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_debug_stream_v1#{}.complete()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The server has stopped writing to and has closed the"]
            #[doc = "associated file descriptor. The data already written to the file"]
            #[doc = "descriptor is correct, but it may be truncated."]
            #[doc = ""]
            #[doc = "This event may be delivered at any time and for any kind of debug"]
            #[doc = "stream. It may be due to a failure in or shutdown of the server."]
            #[doc = "The message argument may provide a hint of the reason."]
            async fn failure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: Option<String>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_debug_stream_v1#{}.failure()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(message)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod weston_desktop {
    #[doc = "Traditional user interfaces can rely on this interface to define the"]
    #[doc = "foundations of typical desktops. Currently it's possible to set up"]
    #[doc = "background, panels and locking surfaces."]
    pub mod weston_desktop_shell {
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
        #[doc = "Trait to implement the weston_desktop_shell interface. See the module level documentation for more info"]
        pub trait WestonDesktopShell: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_desktop_shell";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_desktop_shell#{}.set_background()", object.id);
                        self.set_background(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("weston_desktop_shell#{}.set_panel()", object.id);
                        self.set_panel(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("weston_desktop_shell#{}.set_lock_surface()", object.id);
                        self.set_lock_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("weston_desktop_shell#{}.unlock()", object.id);
                        self.unlock(object, client).await
                    }
                    4u16 => {
                        tracing::debug!("weston_desktop_shell#{}.set_grab_surface()", object.id);
                        self.set_grab_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    5u16 => {
                        tracing::debug!("weston_desktop_shell#{}.desktop_ready()", object.id);
                        self.desktop_ready(object, client).await
                    }
                    6u16 => {
                        tracing::debug!("weston_desktop_shell#{}.set_panel_position()", object.id);
                        self.set_panel_position(object, client, message.uint()?)
                            .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn set_background(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn set_panel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn set_lock_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            async fn unlock(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "The surface set by this request will receive a fake"]
            #[doc = "pointer.enter event during grabs at position 0, 0 and is"]
            #[doc = "expected to set an appropriate cursor image as described by"]
            #[doc = "the grab_cursor event sent just before the enter event."]
            async fn set_grab_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Tell the server, that enough desktop elements have been drawn"]
            #[doc = "to make the desktop look ready for use. During start-up, the"]
            #[doc = "server can wait for this request with a black screen before"]
            #[doc = "starting to fade in the desktop, for instance. If the client"]
            #[doc = "parts of a desktop take a long time to initialize, we avoid"]
            #[doc = "showing temporary garbage."]
            async fn desktop_ready(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            #[doc = "Tell the shell which side of the screen the panel is"]
            #[doc = "located. This is so that new windows do not overlap the panel"]
            #[doc = "and maximized windows maximize properly."]
            async fn set_panel_position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                position: u32,
            ) -> crate::server::Result<()>;
            async fn configure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                edges: u32,
                surface: crate::wire::ObjectId,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_desktop_shell#{}.configure()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(edges)
                    .put_object(Some(surface))
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Tell the client we want it to create and set the lock surface, which is"]
            #[doc = "a GUI asking the user to unlock the screen. The lock surface is"]
            #[doc = "announced with 'set_lock_surface'. Whether or not the client actually"]
            #[doc = "implements locking, it MUST send 'unlock' request to let the normal"]
            #[doc = "desktop resume."]
            async fn prepare_lock_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> weston_desktop_shell#{}.prepare_lock_surface()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event will be sent immediately before a fake enter event on the"]
            #[doc = "grab surface."]
            async fn grab_cursor(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                cursor: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_desktop_shell#{}.grab_cursor()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_uint(cursor).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    #[doc = "Only one client can bind this interface at a time."]
    pub mod weston_screensaver {
        #[doc = "Trait to implement the weston_screensaver interface. See the module level documentation for more info"]
        pub trait WestonScreensaver: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_screensaver";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_screensaver#{}.set_surface()", object.id);
                        self.set_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "A screensaver surface is normally hidden, and only visible after an"]
            #[doc = "idle timeout."]
            async fn set_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
                output: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod weston_direct_display {
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
    pub mod weston_direct_display_v1 {
        #[doc = "Trait to implement the weston_direct_display_v1 interface. See the module level documentation for more info"]
        pub trait WestonDirectDisplayV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_direct_display_v1";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_direct_display_v1#{}.enable()", object.id);
                        self.enable(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("weston_direct_display_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
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
            async fn enable(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                dmabuf: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "Destroys the factory object, but does not affect any other objects."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
        }
    }
}
pub mod weston_output_capture {
    #[doc = "The global interface exposing Weston screenshooting functionality"]
    #[doc = "intended for single shots."]
    #[doc = ""]
    #[doc = "This is a privileged inteface."]
    pub mod weston_capture_v1 {
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
        #[doc = "Trait to implement the weston_capture_v1 interface. See the module level documentation for more info"]
        pub trait WestonCaptureV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_capture_v1";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_capture_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_capture_v1#{}.create()", object.id);
                        self.create(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?.try_into()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Affects no other protocol objects in any way."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn create(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                output: crate::wire::ObjectId,
                source: Source,
                capture_source_new_id: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
        }
    }
    #[doc = "An object representing image capturing functionality for a single"]
    #[doc = "source. When created, it sends the initial events if and only if the"]
    #[doc = "output still exists and the specified pixel source is available on"]
    #[doc = "the output."]
    pub mod weston_capture_source_v1 {
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
        #[doc = "Trait to implement the weston_capture_source_v1 interface. See the module level documentation for more info"]
        pub trait WestonCaptureSourceV1: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_capture_source_v1";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_capture_source_v1#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_capture_source_v1#{}.capture()", object.id);
                        self.capture(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "If a capture is on-going on this object, this will cancel it and"]
            #[doc = "make the image buffer contents undefined."]
            #[doc = ""]
            #[doc = "This object is destroyed."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn capture(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                buffer: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event delivers the pixel format that should be used for the"]
            #[doc = "image buffer. Any buffer is incompatible if it does not have"]
            #[doc = "this pixel format."]
            #[doc = ""]
            #[doc = "The format modifier is linear (DRM_FORMAT_MOD_LINEAR)."]
            #[doc = ""]
            #[doc = "This is an initial event, and sent whenever the required format"]
            #[doc = "changes."]
            async fn format(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                drm_format: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_capture_source_v1#{}.format()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(drm_format)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event delivers the size that should be used for the"]
            #[doc = "image buffer. Any buffer is incompatible if it does not have"]
            #[doc = "this size."]
            #[doc = ""]
            #[doc = "Row alignment of the buffer must be 4 bytes, and it must not contain"]
            #[doc = "further row padding. Otherwise the buffer is unsupported."]
            #[doc = ""]
            #[doc = "This is an initial event, and sent whenever the required size"]
            #[doc = "changes."]
            async fn size(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_capture_source_v1#{}.size()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "has successfully completed."]
            #[doc = ""]
            #[doc = "If the buffer used in the shot is a dmabuf, the client also needs to"]
            #[doc = "wait for any implicit fences on it before accessing the contents."]
            async fn complete(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_capture_source_v1#{}.complete()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "cannot succeed due to an incompatible buffer. The client has already"]
            #[doc = "received the events delivering the new buffer parameters. The client"]
            #[doc = "should retry the capture with the new buffer parameters."]
            async fn retry(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_capture_source_v1#{}.retry()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This event is emitted as a response to 'capture' request when it"]
            #[doc = "has failed for reasons other than an incompatible buffer. The reasons"]
            #[doc = "may include: unsupported buffer type, unsupported buffer stride,"]
            #[doc = "unsupported image source, the image source (output) was removed, or"]
            #[doc = "compositor policy denied the capture."]
            #[doc = ""]
            #[doc = "The string 'msg' may contain a human-readable explanation of the"]
            #[doc = "failure to aid debugging."]
            async fn failed(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                msg: Option<String>,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_capture_source_v1#{}.failed()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().put_string(msg).build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod weston_test {
    #[doc = "Internal testing facilities for the weston compositor."]
    #[doc = ""]
    #[doc = "It can't be stressed enough that these should never ever be used"]
    #[doc = "outside of running weston's tests.  The weston-test.so module should"]
    #[doc = "never be installed."]
    #[doc = ""]
    #[doc = "These requests may allow clients to do very bad things."]
    pub mod weston_test {
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
        #[doc = "Trait to implement the weston_test interface. See the module level documentation for more info"]
        pub trait WestonTest: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_test";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_test#{}.move_surface()", object.id);
                        self.move_surface(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    1u16 => {
                        tracing::debug!("weston_test#{}.move_pointer()", object.id);
                        self.move_pointer(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("weston_test#{}.send_button()", object.id);
                        self.send_button(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.int()?,
                            message.uint()?,
                        )
                        .await
                    }
                    3u16 => {
                        tracing::debug!("weston_test#{}.send_axis()", object.id);
                        self.send_axis(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.fixed()?,
                        )
                        .await
                    }
                    4u16 => {
                        tracing::debug!("weston_test#{}.activate_surface()", object.id);
                        self.activate_surface(object, client, message.object()?)
                            .await
                    }
                    5u16 => {
                        tracing::debug!("weston_test#{}.send_key()", object.id);
                        self.send_key(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    6u16 => {
                        tracing::debug!("weston_test#{}.device_release()", object.id);
                        self.device_release(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    7u16 => {
                        tracing::debug!("weston_test#{}.device_add()", object.id);
                        self.device_add(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    8u16 => {
                        tracing::debug!("weston_test#{}.send_touch()", object.id);
                        self.send_touch(
                            object,
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.int()?,
                            message.fixed()?,
                            message.fixed()?,
                            message.uint()?,
                        )
                        .await
                    }
                    9u16 => {
                        tracing::debug!("weston_test#{}.client_break()", object.id);
                        self.client_break(
                            object,
                            client,
                            message.uint()?.try_into()?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn move_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
            async fn move_pointer(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                x: i32,
                y: i32,
            ) -> crate::server::Result<()>;
            async fn send_button(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                button: i32,
                state: u32,
            ) -> crate::server::Result<()>;
            async fn send_axis(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                axis: u32,
                value: crate::wire::Fixed,
            ) -> crate::server::Result<()>;
            async fn activate_surface(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: Option<crate::wire::ObjectId>,
            ) -> crate::server::Result<()>;
            async fn send_key(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                key: u32,
                state: u32,
            ) -> crate::server::Result<()>;
            async fn device_release(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                device: String,
            ) -> crate::server::Result<()>;
            async fn device_add(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                device: String,
            ) -> crate::server::Result<()>;
            async fn send_touch(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                tv_sec_hi: u32,
                tv_sec_lo: u32,
                tv_nsec: u32,
                touch_id: i32,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
                touch_type: u32,
            ) -> crate::server::Result<()>;
            #[doc = "Request that the compositor pauses execution at a certain point. When"]
            #[doc = "execution is paused, the compositor will signal the shared semaphore"]
            #[doc = "to the client."]
            async fn client_break(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                breakpoint: Breakpoint,
                resource_id: u32,
            ) -> crate::server::Result<()>;
            async fn pointer_position(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: crate::wire::Fixed,
                y: crate::wire::Fixed,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_test#{}.pointer_position()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
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
    pub mod weston_test_runner {
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
        #[doc = "Trait to implement the weston_test_runner interface. See the module level documentation for more info"]
        pub trait WestonTestRunner: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_test_runner";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_test_runner#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_test_runner#{}.run()", object.id);
                        self.run(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
            async fn run(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                test_name: String,
            ) -> crate::server::Result<()>;
            async fn finished(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_test_runner#{}.finished()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
pub mod weston_touch_calibration {
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
    pub mod weston_touch_calibration {
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
        #[doc = "Trait to implement the weston_touch_calibration interface. See the module level documentation for more info"]
        pub trait WestonTouchCalibration: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_touch_calibration";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_touch_calibration#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!(
                            "weston_touch_calibration#{}.create_calibrator()",
                            object.id
                        );
                        self.create_calibrator(
                            object,
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2u16 => {
                        tracing::debug!("weston_touch_calibration#{}.save()", object.id);
                        self.save(
                            object,
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.array()?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "Destroy the binding to the global interface, does not affect any"]
            #[doc = "objects already created through this interface."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn create_calibrator(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                surface: crate::wire::ObjectId,
                device: String,
                cal: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
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
            async fn save(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                device: String,
                matrix: Vec<u8>,
            ) -> crate::server::Result<()>;
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
            async fn touch_device(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                device: String,
                head: String,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibration#{}.touch_device()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_string(Some(device))
                    .put_string(Some(head))
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
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
    pub mod weston_touch_calibrator {
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
        #[doc = "Trait to implement the weston_touch_calibrator interface. See the module level documentation for more info"]
        pub trait WestonTouchCalibrator: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_touch_calibrator";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    0u16 => {
                        tracing::debug!("weston_touch_calibrator#{}.destroy()", object.id);
                        self.destroy(object, client).await
                    }
                    1u16 => {
                        tracing::debug!("weston_touch_calibrator#{}.convert()", object.id);
                        self.convert(
                            object,
                            client,
                            message.int()?,
                            message.int()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This unmaps the surface if it was mapped. The input device grab"]
            #[doc = "is dropped, if it was present. The surface loses its role as a"]
            #[doc = "calibrator."]
            async fn destroy(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()>;
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
            async fn convert(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: i32,
                y: i32,
                reply: crate::wire::ObjectId,
            ) -> crate::server::Result<()>;
            #[doc = "This event tells the client what size to make the surface. The client"]
            #[doc = "must obey the size exactly on the next commit with a wl_buffer."]
            #[doc = ""]
            #[doc = "This event shall be sent once as a response to creating a"]
            #[doc = "weston_touch_calibrator object."]
            async fn configure(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                width: i32,
                height: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.configure()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "This is sent when the compositor wants to cancel the calibration and"]
            #[doc = "drop the touch device grab. The compositor unmaps the surface, if it"]
            #[doc = "was mapped."]
            #[doc = ""]
            #[doc = "The weston_touch_calibrator object will not send any more events. The"]
            #[doc = "client should destroy it."]
            async fn cancel_calibration(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!(
                    "-> weston_touch_calibrator#{}.cancel_calibration()",
                    object.id
                );
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 1u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
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
            async fn invalid_touch(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.invalid_touch()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 2u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A new touch point has appeared on the surface. This touch point is"]
            #[doc = "assigned a unique ID. Future events from this touch point reference"]
            #[doc = "this ID. The ID ceases to be valid after a touch up event and may be"]
            #[doc = "reused in the future."]
            #[doc = ""]
            #[doc = "For the coordinate units, see weston_touch_calibrator."]
            async fn down(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                time: u32,
                id: i32,
                x: u32,
                y: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.down()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_int(id)
                    .put_uint(x)
                    .put_uint(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 3u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "The touch point has disappeared. No further events will be sent for"]
            #[doc = "this touch point and the touch point's ID is released and may be"]
            #[doc = "reused in a future touch down event."]
            async fn up(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                time: u32,
                id: i32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.up()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_int(id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 4u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "A touch point has changed coordinates."]
            #[doc = ""]
            #[doc = "For the coordinate units, see weston_touch_calibrator."]
            async fn motion(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                time: u32,
                id: i32,
                x: u32,
                y: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.motion()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_int(id)
                    .put_uint(x)
                    .put_uint(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 5u16, payload, fds))
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
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.frame()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 6u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
            #[doc = "Sent if the compositor decides the touch stream is a global"]
            #[doc = "gesture. No further events are sent to the clients from that"]
            #[doc = "particular gesture. Touch cancellation applies to all touch points"]
            #[doc = "currently active on this client's surface. The client is"]
            #[doc = "responsible for finalizing the touch points, future touch points on"]
            #[doc = "this surface may reuse the touch point ID."]
            async fn cancel(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_calibrator#{}.cancel()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(object.id, 7u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
    pub mod weston_touch_coordinate {
        #[doc = "Trait to implement the weston_touch_coordinate interface. See the module level documentation for more info"]
        pub trait WestonTouchCoordinate: crate::server::Dispatcher {
            const INTERFACE: &'static str = "weston_touch_coordinate";
            const VERSION: u32 = 1u32;
            fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object
            where
                Self: Sized,
            {
                crate::server::Object::new(id, self)
            }
            async fn handle_request(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                message: &mut crate::wire::Message,
            ) -> crate::server::Result<()> {
                match message.opcode {
                    _ => Err(crate::server::error::Error::UnknownOpcode),
                }
            }
            #[doc = "This event returns the conversion result from surface coordinates to"]
            #[doc = "the expected touch device coordinates."]
            #[doc = ""]
            #[doc = "For details, see weston_touch_calibrator.convert. For the coordinate"]
            #[doc = "units, see weston_touch_calibrator."]
            #[doc = ""]
            #[doc = "This event destroys the weston_touch_coordinate object."]
            async fn result(
                &self,
                object: &crate::server::Object,
                client: &mut crate::server::Client,
                x: u32,
                y: u32,
            ) -> crate::server::Result<()> {
                tracing::debug!("-> weston_touch_coordinate#{}.result()", object.id);
                let (payload, fds) = crate::wire::PayloadBuilder::new()
                    .put_uint(x)
                    .put_uint(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(object.id, 0u16, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        }
    }
}
