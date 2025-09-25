#[allow(clippy::module_inception)]
pub mod appmenu {
    #[doc = "This interface allows a client to link a window (or wl_surface) to an com.canonical.dbusmenu"]
    #[doc = "interface registered on DBus."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_appmenu_manager {
        #[doc = "Trait to implement the org_kde_kwin_appmenu_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinAppmenuManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_appmenu_manager";
            const VERSION: u32 = 2u32;
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_appmenu_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_appmenu_manager#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The DBus service name and object path where the appmenu interface is present"]
    #[doc = "The object should be registered on the session bus before sending this request."]
    #[doc = "If not applicable, clients should remove this object."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_appmenu {
        #[doc = "Trait to implement the org_kde_kwin_appmenu interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinAppmenu
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_appmenu";
            const VERSION: u32 = 2u32;
            #[doc = "Set or update the service name and object path."]
            #[doc = "Strings should be formatted in Latin-1 matching the relevant DBus specifications."]
            fn set_address(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                service_name: String,
                object_path: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let service_name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let object_path = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_appmenu#{}.set_address(\"{}\", \"{}\")",
                                sender_id,
                                service_name,
                                object_path
                            );
                            self.set_address(connection, sender_id, service_name, object_path)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_appmenu#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod blur {
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_blur_manager {
        #[doc = "Trait to implement the org_kde_kwin_blur_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinBlurManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_blur_manager";
            const VERSION: u32 = 1u32;
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn unset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_blur_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_blur_manager#{}.unset({})",
                                sender_id,
                                surface
                            );
                            self.unset(connection, sender_id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_blur {
        #[doc = "Trait to implement the org_kde_kwin_blur interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinBlur
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_blur";
            const VERSION: u32 = 1u32;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_region(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                region: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_blur#{}.commit()", sender_id,);
                            self.commit(connection, sender_id).await
                        }
                        1u16 => {
                            let region = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_blur#{}.set_region({})",
                                sender_id,
                                region
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_region(connection, sender_id, region).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_blur#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod contrast {
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_contrast_manager {
        #[doc = "Trait to implement the org_kde_kwin_contrast_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinContrastManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_contrast_manager";
            const VERSION: u32 = 2u32;
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn unset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast_manager#{}.unset({})",
                                sender_id,
                                surface
                            );
                            self.unset(connection, sender_id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_contrast {
        #[doc = "Trait to implement the org_kde_kwin_contrast interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinContrast
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_contrast";
            const VERSION: u32 = 2u32;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_region(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                region: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_contrast(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                contrast: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_intensity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                intensity: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_saturation(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                saturation: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "enables 'frost' variant of contrast effect."]
            #[doc = ""]
            #[doc = "'frost' is an enhanced version of the contrast effect that"]
            #[doc = "uses different colour arithmetic to get backgrounds simultaneously"]
            #[doc = "higher in contrast and (apparent) transparency."]
            #[doc = ""]
            #[doc = "r, g, b, a are channels from 0-255, indicating a colour to use in contrast calculation."]
            #[doc = "should be based off of the \"main\" background colour of the surface."]
            fn set_frost(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                red: i32,
                green: i32,
                blue: i32,
                alpha: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn unset_frost(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_contrast#{}.commit()", sender_id,);
                            self.commit(connection, sender_id).await
                        }
                        1u16 => {
                            let region = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast#{}.set_region({})",
                                sender_id,
                                region
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.set_region(connection, sender_id, region).await
                        }
                        2u16 => {
                            let contrast = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast#{}.set_contrast({})",
                                sender_id,
                                contrast
                            );
                            self.set_contrast(connection, sender_id, contrast).await
                        }
                        3u16 => {
                            let intensity = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast#{}.set_intensity({})",
                                sender_id,
                                intensity
                            );
                            self.set_intensity(connection, sender_id, intensity).await
                        }
                        4u16 => {
                            let saturation = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast#{}.set_saturation({})",
                                sender_id,
                                saturation
                            );
                            self.set_saturation(connection, sender_id, saturation).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_contrast#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        6u16 => {
                            let red = message.int()?;
                            let green = message.int()?;
                            let blue = message.int()?;
                            let alpha = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_contrast#{}.set_frost({}, {}, {}, {})",
                                sender_id,
                                red,
                                green,
                                blue,
                                alpha
                            );
                            self.set_frost(connection, sender_id, red, green, blue, alpha)
                                .await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_contrast#{}.unset_frost()", sender_id,);
                            self.unset_frost(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod dpms {
    #[doc = "The Dpms manager allows to get a org_kde_kwin_dpms for a given wl_output."]
    #[doc = "The org_kde_kwin_dpms provides the currently used VESA Display Power Management"]
    #[doc = "Signaling state (see https://en.wikipedia.org/wiki/VESA_Display_Power_Management_Signaling )."]
    #[doc = "In addition it allows to request a state change. A compositor is not obliged to honor it"]
    #[doc = "and will normally automatically switch back to on state."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_dpms_manager {
        #[doc = "Trait to implement the org_kde_kwin_dpms_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinDpmsManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_dpms_manager";
            const VERSION: u32 = 1u32;
            #[doc = "Factory request to get the org_kde_kwin_dpms for a given wl_output."]
            fn get(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_dpms_manager#{}.get({}, {})",
                                sender_id,
                                id,
                                output
                            );
                            self.get(connection, sender_id, id, output).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface provides information about the VESA DPMS state for a wl_output."]
    #[doc = "It gets created through the request get on the org_kde_kwin_dpms_manager interface."]
    #[doc = ""]
    #[doc = "On creating the resource the server will push whether DPSM is supported for the output,"]
    #[doc = "the currently used DPMS state and notifies the client through the done event once all"]
    #[doc = "states are pushed. Whenever a state changes the set of changes is committed with the"]
    #[doc = "done event."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_dpms {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            On = 0u32,
            Standby = 1u32,
            Suspend = 2u32,
            Off = 3u32,
        }
        impl From<Mode> for u32 {
            fn from(value: Mode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Mode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::On),
                    1u32 => Ok(Self::Standby),
                    2u32 => Ok(Self::Suspend),
                    3u32 => Ok(Self::Off),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_dpms interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinDpms
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_dpms";
            const VERSION: u32 = 1u32;
            #[doc = "Requests that the compositor puts the wl_output into the passed mode. The compositor"]
            #[doc = "is not obliged to change the state. In addition the compositor might leave the mode"]
            #[doc = "whenever it seems suitable. E.g. the compositor might return to On state on user input."]
            #[doc = ""]
            #[doc = "The client should not assume that the mode changed after requesting a new mode."]
            #[doc = "Instead the client should listen for the mode event."]
            fn set(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event gets pushed on binding the resource and indicates whether the wl_output"]
            #[doc = "supports DPMS. There are operation modes of a Wayland server where DPMS might not"]
            #[doc = "make sense (e.g. nested compositors)."]
            fn supported(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(supported).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This mode gets pushed on binding the resource and provides the currently used"]
            #[doc = "DPMS mode. It also gets pushed if DPMS is not supported for the wl_output, in that"]
            #[doc = "case the value will be On."]
            #[doc = ""]
            #[doc = "The event is also pushed whenever the state changes."]
            fn mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(mode).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event gets pushed on binding the resource once all other states are pushed."]
            #[doc = ""]
            #[doc = "In addition it gets pushed whenever a state changes to tell the client that all"]
            #[doc = "state changes have been pushed."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_dpms#{}.set({})", sender_id, mode);
                            self.set(connection, sender_id, mode).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_dpms#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod fake_input {
    #[doc = "This interface allows other processes to provide fake input events."]
    #[doc = "Purpose is on the one hand side to provide testing facilities like XTest on X11."]
    #[doc = "But also to support use case like kdeconnect's mouse pad interface."]
    #[doc = ""]
    #[doc = "A compositor should not trust the input received from this interface."]
    #[doc = "Clients should not expect that the compositor honors the requests from this"]
    #[doc = "interface."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_fake_input {
        #[doc = "Trait to implement the org_kde_kwin_fake_input interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinFakeInput
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_fake_input";
            const VERSION: u32 = 6u32;
            #[doc = "A client should use this request to tell the compositor why it wants to"]
            #[doc = "use this interface. The compositor might use the information to decide"]
            #[doc = "whether it wants to grant the request. The data might also be passed to"]
            #[doc = "the user to decide whether the application should get granted access to"]
            #[doc = "this very privileged interface."]
            fn authenticate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                application: String,
                reason: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn pointer_motion(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                delta_x: waynest::Fixed,
                delta_y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn button(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button: u32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn axis(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                axis: u32,
                value: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A client should use this request to send touch down event at specific"]
            #[doc = "coordinates."]
            fn touch_down(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
                x: waynest::Fixed,
                y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A client should use this request to send touch motion to specific position."]
            fn touch_motion(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
                x: waynest::Fixed,
                y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A client should use this request to send touch up event."]
            fn touch_up(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A client should use this request to cancel the current"]
            #[doc = "touch event."]
            fn touch_cancel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A client should use this request to send touch frame event."]
            fn touch_frame(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn pointer_motion_absolute(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: waynest::Fixed,
                y: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn keyboard_key(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button: u32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn keyboard_keysym(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                keysym: u32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let application = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let reason = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.authenticate(\"{}\", \"{}\")",
                                sender_id,
                                application,
                                reason
                            );
                            self.authenticate(connection, sender_id, application, reason)
                                .await
                        }
                        1u16 => {
                            let delta_x = message.fixed()?;
                            let delta_y = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.pointer_motion({}, {})",
                                sender_id,
                                delta_x,
                                delta_y
                            );
                            self.pointer_motion(connection, sender_id, delta_x, delta_y)
                                .await
                        }
                        2u16 => {
                            let button = message.uint()?;
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.button({}, {})",
                                sender_id,
                                button,
                                state
                            );
                            self.button(connection, sender_id, button, state).await
                        }
                        3u16 => {
                            let axis = message.uint()?;
                            let value = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.axis({}, {})",
                                sender_id,
                                axis,
                                value
                            );
                            self.axis(connection, sender_id, axis, value).await
                        }
                        4u16 => {
                            let id = message.uint()?;
                            let x = message.fixed()?;
                            let y = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.touch_down({}, {}, {})",
                                sender_id,
                                id,
                                x,
                                y
                            );
                            self.touch_down(connection, sender_id, id, x, y).await
                        }
                        5u16 => {
                            let id = message.uint()?;
                            let x = message.fixed()?;
                            let y = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.touch_motion({}, {}, {})",
                                sender_id,
                                id,
                                x,
                                y
                            );
                            self.touch_motion(connection, sender_id, id, x, y).await
                        }
                        6u16 => {
                            let id = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.touch_up({})",
                                sender_id,
                                id
                            );
                            self.touch_up(connection, sender_id, id).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_fake_input#{}.touch_cancel()", sender_id,);
                            self.touch_cancel(connection, sender_id).await
                        }
                        8u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_fake_input#{}.touch_frame()", sender_id,);
                            self.touch_frame(connection, sender_id).await
                        }
                        9u16 => {
                            let x = message.fixed()?;
                            let y = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.pointer_motion_absolute({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.pointer_motion_absolute(connection, sender_id, x, y)
                                .await
                        }
                        10u16 => {
                            let button = message.uint()?;
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.keyboard_key({}, {})",
                                sender_id,
                                button,
                                state
                            );
                            self.keyboard_key(connection, sender_id, button, state)
                                .await
                        }
                        11u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_fake_input#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        12u16 => {
                            let keysym = message.uint()?;
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_fake_input#{}.keyboard_keysym({}, {})",
                                sender_id,
                                keysym,
                                state
                            );
                            self.keyboard_keysym(connection, sender_id, keysym, state)
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
pub mod fullscreen_shell {
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
    #[doc = "Requesting a surface be presented on an output that already has a"]
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
    #[allow(clippy::too_many_arguments)]
    pub mod _wl_fullscreen_shell {
        #[doc = "Various capabilities that can be advertised by the compositor.  They"]
        #[doc = "are advertised one-at-a-time when the wl_fullscreen_shell interface is"]
        #[doc = "bound.  See the wl_fullscreen_shell.capability event for more details."]
        #[doc = ""]
        #[doc = "ARBITRARY_MODE:"]
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
        impl From<Capability> for u32 {
            fn from(value: Capability) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Capability {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::ArbitraryModes),
                    2u32 => Ok(Self::CursorPlane),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl From<PresentMethod> for u32 {
            fn from(value: PresentMethod) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PresentMethod {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Default),
                    1u32 => Ok(Self::Center),
                    2u32 => Ok(Self::Zoom),
                    3u32 => Ok(Self::ZoomCrop),
                    4u32 => Ok(Self::Stretch),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PresentMethod {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These errors can be emitted in response to wl_fullscreen_shell requests"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "present_method is not known"]
            InvalidMethod = 0u32,
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
                    0u32 => Ok(Self::InvalidMethod),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the _wl_fullscreen_shell interface. See the module level documentation for more info"]
        pub trait WlFullscreenShell
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "_wl_fullscreen_shell";
            const VERSION: u32 = 1u32;
            #[doc = "Release the binding from the wl_fullscreen_shell interface"]
            #[doc = ""]
            #[doc = "This destroys the server-side object and frees this binding.  If"]
            #[doc = "the client binds to wl_fullscreen_shell multiple times, it may wish"]
            #[doc = "to free some of those bindings."]
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Present a surface on the given output."]
            #[doc = ""]
            #[doc = "If the output is null, the compositor will present the surface on"]
            #[doc = "whatever display (or displays) it thinks best.  In particular, this"]
            #[doc = "may replace any or all surfaces currently presented so it should"]
            #[doc = "not be used in combination with placing surfaces on specific"]
            #[doc = "outputs."]
            #[doc = ""]
            #[doc = "The method parameter is a hint to the compositor for how the surface"]
            #[doc = "is to be presented.  In particular, it tells the compostior how to"]
            #[doc = "handle a size mismatch between the presented surface and the"]
            #[doc = "output.  The compositor is free to ignore this parameter."]
            #[doc = ""]
            #[doc = "The \"zoom\", \"zoom_crop\", and \"stretch\" methods imply a scaling"]
            #[doc = "operation on the surface.  This will override any kind of output"]
            #[doc = "scaling, so the buffer_scale property of the surface is effectively"]
            #[doc = "ignored."]
            fn present_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: Option<waynest::ObjectId>,
                method: u32,
                output: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Presents a surface on the given output for a particular mode."]
            #[doc = ""]
            #[doc = "If the current size of the output differs from that of the surface,"]
            #[doc = "the compositor will attempt to change the size of the output to"]
            #[doc = "match the surface.  The result of the mode-switch operation will be"]
            #[doc = "returned via the provided wl_fullscreen_shell_mode_feedback object."]
            #[doc = ""]
            #[doc = "If the current output mode matches the one requested or if the"]
            #[doc = "compositor successfully switches the mode to match the surface,"]
            #[doc = "then the mode_successful event will be sent and the output will"]
            #[doc = "contain the contents of the given surface.  If the compositor"]
            #[doc = "cannot match the output size to the surface size, the mode_failed"]
            #[doc = "will be sent and the output will contain the contents of the"]
            #[doc = "previously presented surface (if any).  If another surface is"]
            #[doc = "presented on the given output before either of these has a chance"]
            #[doc = "to happen, the present_cancelled event will be sent."]
            #[doc = ""]
            #[doc = "Due to race conditions and other issues unknown to the client, no"]
            #[doc = "mode-switch operation is guaranteed to succeed.  However, if the"]
            #[doc = "mode is one advertised by wl_output.mode or if the compositor"]
            #[doc = "advertises the ARBITRARY_MODES capability, then the client should"]
            #[doc = "expect that the mode-switch operation will usually succeed."]
            #[doc = ""]
            #[doc = "If the size of the presented surface changes, the resulting output"]
            #[doc = "is undefined.  The compositor may attempt to change the output mode"]
            #[doc = "to compensate.  However, there is no guarantee that a suitable mode"]
            #[doc = "will be found and the client has no way to be notified of success"]
            #[doc = "or failure."]
            #[doc = ""]
            #[doc = "The framerate parameter specifies the desired framerate for the"]
            #[doc = "output in mHz.  The compositor is free to ignore this parameter.  A"]
            #[doc = "value of 0 indicates that the client has no preference."]
            #[doc = ""]
            #[doc = "If the value of wl_output.scale differs from wl_surface.buffer_scale,"]
            #[doc = "then the compositor may choose a mode that matches either the buffer"]
            #[doc = "size or the surface size.  In either case, the surface will fill the"]
            #[doc = "output."]
            fn present_surface_for_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
                output: waynest::ObjectId,
                framerate: i32,
                feedback: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Advertises a single capability of the compositor."]
            #[doc = ""]
            #[doc = "When the wl_fullscreen_shell interface is bound, this event is emitted"]
            #[doc = "once for each capability advertised.  Valid capabilities are given by"]
            #[doc = "the wl_fullscreen_shell.capability enum.  If clients want to take"]
            #[doc = "advantage of any of these capabilities, they should use a"]
            #[doc = "wl_display.sync request immediately after binding to ensure that they"]
            #[doc = "receive all the capability events."]
            fn capability(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                capability: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(capability).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("_wl_fullscreen_shell#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        1u16 => {
                            let surface = message.object()?;
                            let method = message.uint()?;
                            let output = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "_wl_fullscreen_shell#{}.present_surface({}, {}, {})",
                                sender_id,
                                surface
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string()),
                                method,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.present_surface(connection, sender_id, surface, method, output)
                                .await
                        }
                        2u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let framerate = message.int()?;
                            let feedback = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "_wl_fullscreen_shell#{}.present_surface_for_mode({}, {}, {}, {})",
                                sender_id,
                                surface,
                                output,
                                framerate,
                                feedback
                            );
                            self.present_surface_for_mode(
                                connection, sender_id, surface, output, framerate, feedback,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod _wl_fullscreen_shell_mode_feedback {
        #[doc = "Trait to implement the _wl_fullscreen_shell_mode_feedback interface. See the module level documentation for more info"]
        pub trait WlFullscreenShellModeFeedback
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "_wl_fullscreen_shell_mode_feedback";
            const VERSION: u32 = 1u32;
            #[doc = "This event indicates that the attempted mode switch operation was"]
            #[doc = "successful.  A surface of the size requested in the mode switch"]
            #[doc = "will fill the output without scaling."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            fn mode_successful(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the attempted mode switch operation"]
            #[doc = "failed. This may be because the requested output mode is not"]
            #[doc = "possible or it may mean that the compositor does not want to allow it."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            fn mode_failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the attempted mode switch operation was"]
            #[doc = "cancelled.  Most likely this is because the client requested a"]
            #[doc = "second mode switch before the first one completed."]
            #[doc = ""]
            #[doc = "Upon receiving this event, the client should destroy the"]
            #[doc = "wl_fullscreen_shell_mode_feedback object."]
            fn present_cancelled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod idle {
    #[doc = "This interface allows to monitor user idle time on a given seat. The interface"]
    #[doc = "allows to register timers which trigger after no user activity was registered"]
    #[doc = "on the seat for a given interval. It notifies when user activity resumes."]
    #[doc = ""]
    #[doc = "This is useful for applications wanting to perform actions when the user is not"]
    #[doc = "interacting with the system, e.g. chat applications setting the user as away, power"]
    #[doc = "management features to dim screen, etc.."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_idle {
        #[doc = "Trait to implement the org_kde_kwin_idle interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinIdle
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_idle";
            const VERSION: u32 = 1u32;
            fn get_idle_timeout(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
                timeout: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let timeout = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_idle#{}.get_idle_timeout({}, {}, {})",
                                sender_id,
                                id,
                                seat,
                                timeout
                            );
                            self.get_idle_timeout(connection, sender_id, id, seat, timeout)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_idle_timeout {
        #[doc = "Trait to implement the org_kde_kwin_idle_timeout interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinIdleTimeout
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_idle_timeout";
            const VERSION: u32 = 1u32;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn simulate_user_activity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn idle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn resumed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_idle_timeout#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_idle_timeout#{}.simulate_user_activity()",
                                sender_id,
                            );
                            self.simulate_user_activity(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod kde_external_brightness_v1 {
    #[doc = "Some brightness control mechanisms are somewhat unstable, or require root"]
    #[doc = "privileges, so putting them inside of the compositor is not desired."]
    #[doc = "This protocol is for outsourcing the actual brightness-setting to a"]
    #[doc = "process outside of the compositor."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_external_brightness_v1 {
        #[doc = "Trait to implement the kde_external_brightness_v1 interface. See the module level documentation for more info"]
        pub trait KdeExternalBrightnessV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_external_brightness_v1";
            const VERSION: u32 = 3u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn create_brightness_control(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_external_brightness_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_v1#{}.create_brightness_control({})",
                                sender_id,
                                id
                            );
                            self.create_brightness_control(connection, sender_id, id)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "After creating this object, the client should issue all relevant setup requests"]
    #[doc = "(set_internal, set_edid, set_max_brightness, optionally set_observed_brightness)"]
    #[doc = "and finish the sequence with commit."]
    #[doc = "Afterwards, for each change in values, the client must call commit again."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_external_brightness_device_v1 {
        #[doc = "Trait to implement the kde_external_brightness_device_v1 interface. See the module level documentation for more info"]
        pub trait KdeExternalBrightnessDeviceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_external_brightness_device_v1";
            const VERSION: u32 = 3u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_internal(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                internal: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_edid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                string: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_max_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                value: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The client can set this to notify the compositor of the device's initial brightness."]
            #[doc = "It can also set this again after the initial commit to notify the compositor that"]
            #[doc = "the brightness level has changed due to external factors."]
            #[doc = "The compositor is free to use or ignore this value as it sees fit."]
            fn set_observed_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                value: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The compositor can use this information to ignore this object if its commands"]
            #[doc = "expose monitor issues. The compositor may also reduce the amount of brightness"]
            #[doc = "requests given potentially slow response times and concerns about monitor EEPROM"]
            #[doc = "longevity/wear-out."]
            fn set_uses_ddc_ci(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                value: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The client must ensure that if the brightness level changes due to external factors,"]
            #[doc = "that it either overwrites those changes with what the compositor last requested,"]
            #[doc = "or commit again with set_observed_brightness specifying the changed brightness."]
            fn requested_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                value: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(value).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let internal = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.set_internal({})",
                                sender_id,
                                internal
                            );
                            self.set_internal(connection, sender_id, internal).await
                        }
                        2u16 => {
                            let string = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.set_edid(\"{}\")",
                                sender_id,
                                string
                            );
                            self.set_edid(connection, sender_id, string).await
                        }
                        3u16 => {
                            let value = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.set_max_brightness({})",
                                sender_id,
                                value
                            );
                            self.set_max_brightness(connection, sender_id, value).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.commit()",
                                sender_id,
                            );
                            self.commit(connection, sender_id).await
                        }
                        5u16 => {
                            let value = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.set_observed_brightness({})",
                                sender_id,
                                value
                            );
                            self.set_observed_brightness(connection, sender_id, value)
                                .await
                        }
                        6u16 => {
                            let value = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_external_brightness_device_v1#{}.set_uses_ddc_ci({})",
                                sender_id,
                                value
                            );
                            self.set_uses_ddc_ci(connection, sender_id, value).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod kde_lockscreen_overlay_v1 {
    #[doc = "Allows a client to request a surface to be visible when the system is locked."]
    #[doc = ""]
    #[doc = "This is meant to be used for specific high urgency cases like phone calls or alarms."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_lockscreen_overlay_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the client provided an invalid surface state"]
            InvalidSurfaceState = 0u32,
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
                    0u32 => Ok(Self::InvalidSurfaceState),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the kde_lockscreen_overlay_v1 interface. See the module level documentation for more info"]
        pub trait KdeLockscreenOverlayV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_lockscreen_overlay_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Informs the compositor that the surface could be shown when the screen is locked. This request should be called while the surface is unmapped."]
            fn allow(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This won't affect the surface previously marked with the allow request."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_lockscreen_overlay_v1#{}.allow({})",
                                sender_id,
                                surface
                            );
                            self.allow(connection, sender_id, surface).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_lockscreen_overlay_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod kde_output_device_v2 {
    #[doc = "An output device describes a display device available to the compositor."]
    #[doc = "output_device is similar to wl_output, but focuses on output"]
    #[doc = "configuration management."]
    #[doc = ""]
    #[doc = "A client can query all global output_device objects to enlist all"]
    #[doc = "available display devices, even those that may currently not be"]
    #[doc = "represented by the compositor as a wl_output."]
    #[doc = ""]
    #[doc = "The client sends configuration changes to the server through the"]
    #[doc = "outputconfiguration interface, and the server applies the configuration"]
    #[doc = "changes to the hardware and signals changes to the output devices"]
    #[doc = "accordingly."]
    #[doc = ""]
    #[doc = "This object is published as global during start up for every available"]
    #[doc = "display devices, or when one later becomes available, for example by"]
    #[doc = "being hotplugged via a physical connector."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_output_device_v2 {
        #[doc = "This enumeration describes how the physical pixels on an output are"]
        #[doc = "laid out."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Subpixel {
            Unknown = 0u32,
            None = 1u32,
            HorizontalRgb = 2u32,
            HorizontalBgr = 3u32,
            VerticalRgb = 4u32,
            VerticalBgr = 5u32,
        }
        impl From<Subpixel> for u32 {
            fn from(value: Subpixel) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Subpixel {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unknown),
                    1u32 => Ok(Self::None),
                    2u32 => Ok(Self::HorizontalRgb),
                    3u32 => Ok(Self::HorizontalBgr),
                    4u32 => Ok(Self::VerticalRgb),
                    5u32 => Ok(Self::VerticalBgr),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Subpixel {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "This describes the transform, that a compositor will apply to a"]
        #[doc = "surface to compensate for the rotation or mirroring of an"]
        #[doc = "output device."]
        #[doc = ""]
        #[doc = "The flipped values correspond to an initial flip around a"]
        #[doc = "vertical axis followed by rotation."]
        #[doc = ""]
        #[doc = "The purpose is mainly to allow clients to render accordingly and"]
        #[doc = "tell the compositor, so that for fullscreen surfaces, the"]
        #[doc = "compositor is still able to scan out directly client surfaces."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Transform {
            Normal = 0u32,
            _90 = 1u32,
            _180 = 2u32,
            _270 = 3u32,
            Flipped = 4u32,
            Flipped90 = 5u32,
            Flipped180 = 6u32,
            Flipped270 = 7u32,
        }
        impl From<Transform> for u32 {
            fn from(value: Transform) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Transform {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Transform {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Describes what capabilities this device has."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "if this output_device can use overscan"] const Overscan = 1u32 ; # [doc = "if this outputdevice supports variable refresh rate"] const Vrr = 2u32 ; # [doc = "if setting the rgb range is possible"] const RgbRange = 4u32 ; # [doc = "if this outputdevice supports high dynamic range"] const HighDynamicRange = 8u32 ; # [doc = "if this outputdevice supports a wide color gamut"] const WideColorGamut = 16u32 ; # [doc = "if this outputdevice supports autorotation"] const AutoRotate = 32u32 ; # [doc = "if this outputdevice supports icc profiles"] const IccProfile = 64u32 ; # [doc = "if this outputdevice supports the brightness setting"] const Brightness = 128u32 ; # [doc = "if this outputdevice supports the built-in color profile"] const BuiltInColor = 256u32 ; # [doc = "if this outputdevice supports DDC/CI"] const DdcCi = 512u32 ; # [doc = "if this outputdevice supports setting max bpc"] const MaxBitsPerColor = 1024u32 ; # [doc = "if this outputdevice supports EDR"] const Edr = 2048u32 ; # [doc = "if this outputdevice supports the sharpness setting"] const Sharpness = 4096u32 ; } }
        impl From<Capability> for u32 {
            fn from(value: Capability) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Capability {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl From<VrrPolicy> for u32 {
            fn from(value: VrrPolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for VrrPolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Whether full or limited color range should be used"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum RgbRange {
            Automatic = 0u32,
            Full = 1u32,
            Limited = 2u32,
        }
        impl From<RgbRange> for u32 {
            fn from(value: RgbRange) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for RgbRange {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Automatic),
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Limited),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for RgbRange {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AutoRotatePolicy {
            Never = 0u32,
            InTabletMode = 1u32,
            Always = 2u32,
        }
        impl From<AutoRotatePolicy> for u32 {
            fn from(value: AutoRotatePolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for AutoRotatePolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::InTabletMode),
                    2u32 => Ok(Self::Always),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AutoRotatePolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ColorProfileSource {
            SRgb = 0u32,
            Icc = 1u32,
            Edid = 2u32,
        }
        impl From<ColorProfileSource> for u32 {
            fn from(value: ColorProfileSource) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ColorProfileSource {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SRgb),
                    1u32 => Ok(Self::Icc),
                    2u32 => Ok(Self::Edid),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ColorProfileSource {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "The compositor can do a lot of things that trade between"]
        #[doc = "performance, power and color accuracy. This setting describes"]
        #[doc = "a high level preference from the user about in which direction"]
        #[doc = "that tradeoff should be made."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ColorPowerTradeoff {
            #[doc = "prefer efficiency and performance"]
            Efficiency = 0u32,
            #[doc = "prefer accuracy"]
            Accuracy = 1u32,
        }
        impl From<ColorPowerTradeoff> for u32 {
            fn from(value: ColorPowerTradeoff) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ColorPowerTradeoff {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Efficiency),
                    1u32 => Ok(Self::Accuracy),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ColorPowerTradeoff {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum EdrPolicy {
            Never = 0u32,
            Always = 1u32,
        }
        impl From<EdrPolicy> for u32 {
            fn from(value: EdrPolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for EdrPolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for EdrPolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the kde_output_device_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputDeviceV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_output_device_v2";
            const VERSION: u32 = 17u32;
            #[doc = "The geometry event describes geometric properties of the output."]
            #[doc = "The event is sent when binding to the output object and whenever"]
            #[doc = "any of the properties change."]
            fn geometry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                physical_width: i32,
                physical_height: i32,
                subpixel: i32,
                make: String,
                model: String,
                transform: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(physical_width)
                        .put_int(physical_height)
                        .put_int(subpixel)
                        .put_string(Some(make))
                        .put_string(Some(model))
                        .put_int(transform)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event describes the mode currently in use for this head. It is only"]
            #[doc = "sent if the output is enabled."]
            fn current_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(mode))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The mode event describes an available mode for the output."]
            #[doc = ""]
            #[doc = "When the client binds to the output_device object, the server sends this"]
            #[doc = "event once for every available mode the output_device can be operated by."]
            #[doc = ""]
            #[doc = "There will always be at least one event sent out on initial binding,"]
            #[doc = "which represents the current mode."]
            #[doc = ""]
            #[doc = "Later if an output changes, its mode event is sent again for the"]
            #[doc = "eventual added modes and lastly the current mode. In other words, the"]
            #[doc = "current mode is always represented by the latest event sent with the current"]
            #[doc = "flag set."]
            #[doc = ""]
            #[doc = "The size of a mode is given in physical hardware units of the output device."]
            #[doc = "This is not necessarily the same as the output size in the global compositor"]
            #[doc = "space. For instance, the output may be scaled, as described in"]
            #[doc = "kde_output_device_v2.scale, or transformed, as described in"]
            #[doc = "kde_output_device_v2.transform."]
            fn mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(mode))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after all other properties have been"]
            #[doc = "sent on binding to the output object as well as after any"]
            #[doc = "other output property change have been applied later on."]
            #[doc = "This allows to see changes to the output properties as atomic,"]
            #[doc = "even if multiple events successively announce them."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. If it is not sent, the client should assume a"]
            #[doc = "scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients track the"]
            #[doc = "current output of a surface, and if it is on a scaled"]
            #[doc = "output it should use wl_surface.set_buffer_scale with"]
            #[doc = "the scale of the output. That way the compositor can"]
            #[doc = "avoid scaling the surface, and the client can supply"]
            #[doc = "a higher detail image."]
            fn scale(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                factor: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_fixed(factor).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The edid event encapsulates the EDID data for the outputdevice."]
            #[doc = ""]
            #[doc = "The event is sent when binding to the output object. The EDID"]
            #[doc = "data may be empty, in which case this event is sent anyway."]
            #[doc = "If the EDID information is empty, you can fall back to the name"]
            #[doc = "et al. properties of the outputdevice."]
            fn edid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                raw: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(raw)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The enabled event notifies whether this output is currently"]
            #[doc = "enabled and used for displaying content by the server."]
            #[doc = "The event is sent when binding to the output object and"]
            #[doc = "whenever later on an output changes its state by becoming"]
            #[doc = "enabled or disabled."]
            fn enabled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                enabled: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(enabled).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The uuid can be used to identify the output. It's controlled by"]
            #[doc = "the server entirely. The server should make sure the uuid is"]
            #[doc = "persistent across restarts. An empty uuid is considered invalid."]
            fn uuid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                uuid: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(uuid))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Serial ID of the monitor, sent on startup before the first done event."]
            fn serial_number(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial_number: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(serial_number))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "EISA ID of the monitor, sent on startup before the first done event."]
            fn eisa_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                eisa_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(eisa_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "What capabilities this device has, sent on startup before the first"]
            #[doc = "done event."]
            fn capabilities(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: Capability,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(flags.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Overscan value of the monitor in percent, sent on startup before the"]
            #[doc = "first done event."]
            fn overscan(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                overscan: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(overscan).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "What policy the compositor will employ regarding its use of variable"]
            #[doc = "refresh rate."]
            fn vrr_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                vrr_policy: VrrPolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(vrr_policy.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "What rgb range the compositor is using for this output"]
            fn rgb_range(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rgb_range: RgbRange,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(rgb_range.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 13u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Name of the output, it's useful to cross-reference to an zxdg_output_v1 and ultimately QScreen"]
            fn name(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 14u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Whether or not high dynamic range is enabled for this output"]
            fn high_dynamic_range(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hdr_enabled: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(hdr_enabled).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 15u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "If high dynamic range is used, this value defines the brightness in nits for content"]
            #[doc = "that's in standard dynamic range format. Note that while the value is in nits, that"]
            #[doc = "doesn't necessarily translate to the same brightness on the screen."]
            fn sdr_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                sdr_brightness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(sdr_brightness)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 16u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Whether or not the use of a wide color gamut is enabled for this output"]
            fn wide_color_gamut(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                wcg_enabled: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(wcg_enabled).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 17u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn auto_rotate_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                policy: AutoRotatePolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(policy.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 18u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn icc_profile_path(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                profile_path: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(profile_path))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 19u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn brightness_metadata(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                max_peak_brightness: u32,
                max_frame_average_brightness: u32,
                min_brightness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(max_peak_brightness)
                        .put_uint(max_frame_average_brightness)
                        .put_uint(min_brightness)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 20u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn brightness_overrides(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                max_peak_brightness: i32,
                max_average_brightness: i32,
                min_brightness: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(max_peak_brightness)
                        .put_int(max_average_brightness)
                        .put_int(min_brightness)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 21u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This can be used to provide the colors users assume sRGB applications should have based on the"]
            #[doc = "default experience on many modern sRGB screens."]
            fn sdr_gamut_wideness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                gamut_wideness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(gamut_wideness)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 22u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn color_profile_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                source: ColorProfileSource,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(source.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 23u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This is the brightness modifier of the output. It doesn't specify"]
            #[doc = "any absolute values, but is merely a multiplier on top of other"]
            #[doc = "brightness values, like sdr_brightness and brightness_metadata."]
            #[doc = "0 is the minimum brightness (not completely dark) and 10000 is"]
            #[doc = "the maximum brightness."]
            #[doc = "This is currently only supported / meaningful while HDR is active."]
            fn brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                brightness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(brightness).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 24u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn color_power_tradeoff(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                preference: ColorPowerTradeoff,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(preference.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 25u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This is the dimming multiplier of the output. This is similar to"]
            #[doc = "the brightness setting, except it's meant to be a temporary setting"]
            #[doc = "only, not persistent and may be implemented differently depending"]
            #[doc = "on the display."]
            #[doc = "0 is the minimum dimming factor (not completely dark) and 10000"]
            #[doc = "means the output is not dimmed."]
            fn dimming(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                multiplier: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(multiplier).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 26u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn replication_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                source: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(source))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 27u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "If the ddc_ci capability is present, this determines if settings"]
            #[doc = "such as brightness, contrast or others should be set using DDC/CI."]
            fn ddc_ci_allowed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                allowed: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(allowed).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 28u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This limits the amount of bits per color that are sent to the display."]
            fn max_bits_per_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                max_bpc: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(max_bpc).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 29u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn max_bits_per_color_range(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                min_value: u32,
                max_value: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(min_value)
                        .put_uint(max_value)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 30u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn automatic_max_bits_per_color_limit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                max_bpc_limit: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(max_bpc_limit)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 31u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "When EDR is enabled, the compositor may increase the backlight beyond"]
            #[doc = "the user-specified setting, in order to present HDR content on displays"]
            #[doc = "without native HDR support."]
            #[doc = "This will usually result in better visuals, but also increases battery"]
            #[doc = "usage."]
            fn edr_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                policy: EdrPolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(policy.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 32u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This is the sharpness modifier of the output."]
            #[doc = "0 is sharpness disabled and 10000 is the maximum sharpness"]
            fn sharpness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                sharpness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(sharpness).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 33u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
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
    #[doc = "kde_output_device.done event. No guarantees are made regarding the order"]
    #[doc = "in which properties are sent."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_output_device_mode_v2 {
        #[doc = "Trait to implement the kde_output_device_mode_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputDeviceModeV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_output_device_mode_v2";
            const VERSION: u32 = 1u32;
            #[doc = "This event describes the mode size. The size is given in physical"]
            #[doc = "hardware units of the output device. This is not necessarily the same as"]
            #[doc = "the output size in the global compositor space. For instance, the output"]
            #[doc = "may be scaled or transformed."]
            fn size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event describes the mode's fixed vertical refresh rate. It is only"]
            #[doc = "sent if the mode has a fixed refresh rate."]
            fn refresh(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                refresh: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(refresh).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event advertises this mode as preferred."]
            fn preferred(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The compositor will destroy the object immediately after sending this"]
            #[doc = "event, so it will become invalid and the client should release any"]
            #[doc = "resources associated with it."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod kde_output_management_v2 {
    #[doc = "This interface enables clients to set properties of output devices for screen"]
    #[doc = "configuration purposes via the server. To this end output devices are referenced"]
    #[doc = "by global kde_output_device_v2 objects."]
    #[doc = ""]
    #[doc = "outputmanagement (wl_global)"]
    #[doc = "--------------------------"]
    #[doc = "request:"]
    #[doc = "* create_configuration -> outputconfiguration (wl_resource)"]
    #[doc = ""]
    #[doc = "outputconfiguration (wl_resource)"]
    #[doc = "--------------------------"]
    #[doc = "requests:"]
    #[doc = "* enable(outputdevice, bool)"]
    #[doc = "* mode(outputdevice, mode)"]
    #[doc = "* transformation(outputdevice, flag)"]
    #[doc = "* position(outputdevice, x, y)"]
    #[doc = "* apply"]
    #[doc = ""]
    #[doc = "events:"]
    #[doc = "* applied"]
    #[doc = "* failed"]
    #[doc = ""]
    #[doc = "The server registers one outputmanagement object as a global object. In order"]
    #[doc = "to configure outputs a client requests create_configuration, which provides a"]
    #[doc = "resource referencing an outputconfiguration for one-time configuration. That"]
    #[doc = "way the server knows which requests belong together and can group them by that."]
    #[doc = ""]
    #[doc = "On the outputconfiguration object the client calls for each output whether the"]
    #[doc = "output should be enabled, which mode should be set (by referencing the mode from"]
    #[doc = "the list of announced modes) and the output's global position. Once all outputs"]
    #[doc = "are configured that way, the client calls apply."]
    #[doc = "At that point and not earlier the server should try to apply the configuration."]
    #[doc = "If this succeeds the server emits the applied signal, otherwise the failed"]
    #[doc = "signal, such that the configuring client is noticed about the success of its"]
    #[doc = "configuration request."]
    #[doc = ""]
    #[doc = "Through this design the interface enables atomic output configuration changes if"]
    #[doc = "internally supported by the server."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment implementation"]
    #[doc = "detail. Regular clients must not use this protocol. Backward incompatible"]
    #[doc = "changes may be added without bumping the major version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_output_management_v2 {
        #[doc = "Trait to implement the kde_output_management_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputManagementV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_output_management_v2";
            const VERSION: u32 = 17u32;
            #[doc = "Request an outputconfiguration object through which the client can configure"]
            #[doc = "output devices."]
            fn create_configuration(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_management_v2#{}.create_configuration({})",
                                sender_id,
                                id
                            );
                            self.create_configuration(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "outputconfiguration is a client-specific resource that can be used to ask"]
    #[doc = "the server to apply changes to available output devices."]
    #[doc = ""]
    #[doc = "The client receives a list of output devices from the registry. When it wants"]
    #[doc = "to apply new settings, it creates a configuration object from the"]
    #[doc = "outputmanagement global, writes changes through this object's enable, scale,"]
    #[doc = "transform and mode calls. It then asks the server to apply these settings in"]
    #[doc = "an atomic fashion, for example through Linux' DRM interface."]
    #[doc = ""]
    #[doc = "The server signals back whether the new settings have applied successfully"]
    #[doc = "or failed to apply. outputdevice objects are updated after the changes have been"]
    #[doc = "applied to the hardware and before the server side sends the applied event."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_output_configuration_v2 {
        #[doc = "These error can be emitted in response to kde_output_configuration_v2 requests."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the config is already applied"]
            AlreadyApplied = 0u32,
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
                    0u32 => Ok(Self::AlreadyApplied),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl From<VrrPolicy> for u32 {
            fn from(value: VrrPolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for VrrPolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Whether this output should use full or limited rgb."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum RgbRange {
            Automatic = 0u32,
            Full = 1u32,
            Limited = 2u32,
        }
        impl From<RgbRange> for u32 {
            fn from(value: RgbRange) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for RgbRange {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Automatic),
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Limited),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for RgbRange {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AutoRotatePolicy {
            Never = 0u32,
            InTabletMode = 1u32,
            Always = 2u32,
        }
        impl From<AutoRotatePolicy> for u32 {
            fn from(value: AutoRotatePolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for AutoRotatePolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::InTabletMode),
                    2u32 => Ok(Self::Always),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AutoRotatePolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ColorProfileSource {
            SRgb = 0u32,
            Icc = 1u32,
            Edid = 2u32,
        }
        impl From<ColorProfileSource> for u32 {
            fn from(value: ColorProfileSource) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ColorProfileSource {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::SRgb),
                    1u32 => Ok(Self::Icc),
                    2u32 => Ok(Self::Edid),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ColorProfileSource {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "The compositor can do a lot of things that trade between"]
        #[doc = "performance, power and color accuracy. This setting describes"]
        #[doc = "a high level preference from the user about in which direction"]
        #[doc = "that tradeoff should be made."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ColorPowerTradeoff {
            #[doc = "prefer efficiency and performance"]
            Efficiency = 0u32,
            #[doc = "prefer accuracy"]
            Accuracy = 1u32,
        }
        impl From<ColorPowerTradeoff> for u32 {
            fn from(value: ColorPowerTradeoff) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ColorPowerTradeoff {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Efficiency),
                    1u32 => Ok(Self::Accuracy),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ColorPowerTradeoff {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum EdrPolicy {
            Never = 0u32,
            Always = 1u32,
        }
        impl From<EdrPolicy> for u32 {
            fn from(value: EdrPolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for EdrPolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for EdrPolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the kde_output_configuration_v2 interface. See the module level documentation for more info"]
        pub trait KdeOutputConfigurationV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_output_configuration_v2";
            const VERSION: u32 = 17u32;
            #[doc = "Mark the output as enabled or disabled."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                enable: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the mode for a given output."]
            fn mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                mode: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the transformation for a given output."]
            fn transform(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                transform: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the position for this output device. (x,y) describe the top-left corner"]
            #[doc = "of the output in global space, whereby the origin (0,0) of the global space"]
            #[doc = "has to be aligned with the top-left corner of the most left and in case this"]
            #[doc = "does not define a single one the top output."]
            #[doc = ""]
            #[doc = "There may be no gaps or overlaps between outputs, i.e. the outputs are"]
            #[doc = "stacked horizontally, vertically, or both on each other."]
            fn position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the scaling factor for this output device."]
            fn scale(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                scale: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Asks the server to apply property changes requested through this outputconfiguration"]
            #[doc = "object to all outputs on the server side."]
            #[doc = ""]
            #[doc = "The output configuration can be applied only once. The already_applied protocol error"]
            #[doc = "will be posted if the apply request is called the second time."]
            fn apply(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the overscan value of this output device with a value in percent."]
            fn overscan(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                overscan: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set what policy the compositor should employ regarding its use of"]
            #[doc = "variable refresh rate."]
            fn set_vrr_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                policy: VrrPolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Whether full or limited color range should be used"]
            fn set_rgb_range(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                rgb_range: RgbRange,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_primary_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The order of outputs can be used to assign desktop environment components to a specific screen,"]
            #[doc = "see kde_output_order_v1 for details. The priority is 1-based for outputs that will be enabled after"]
            #[doc = "this changeset is applied, all outputs that are disabled need to have the index set to zero."]
            fn set_priority(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                priority: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets whether or not the output should be set to HDR mode."]
            fn set_high_dynamic_range(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                enable_hdr: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the brightness of standard dynamic range content in nits. Only has an effect while the output is in HDR mode."]
            #[doc = "Note that while the value is in nits, that doesn't necessarily translate to the same brightness on the screen."]
            fn set_sdr_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                sdr_brightness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Whether or not the output should use a wide color gamut"]
            fn set_wide_color_gamut(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                enable_wcg: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_auto_rotate_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                policy: AutoRotatePolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_icc_profile_path(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                profile_path: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_brightness_overrides(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                max_peak_brightness: i32,
                max_frame_average_brightness: i32,
                min_brightness: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This can be used to provide the colors users assume sRGB applications should have based on the"]
            #[doc = "default experience on many modern sRGB screens."]
            fn set_sdr_gamut_wideness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                gamut_wideness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_color_profile_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                color_profile_source: ColorProfileSource,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the brightness modifier of the output. It doesn't specify"]
            #[doc = "any absolute values, but is merely a multiplier on top of other"]
            #[doc = "brightness values, like sdr_brightness and brightness_metadata."]
            #[doc = "0 is the minimum brightness (not completely dark) and 10000 is"]
            #[doc = "the maximum brightness."]
            #[doc = "This is supported while HDR is active in versions 8 and below,"]
            #[doc = "or when the device supports the \"brightness\" capability in"]
            #[doc = "versions 9 and above."]
            fn set_brightness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                brightness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_color_power_tradeoff(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                preference: ColorPowerTradeoff,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the dimming multiplier of the output. This is similar to the"]
            #[doc = "brightness setting, except it's meant to be a temporary setting"]
            #[doc = "only, not persistent and may be implemented differently depending"]
            #[doc = "on the display."]
            #[doc = "0 is the minimum dimming factor (not completely dark) and 10000"]
            #[doc = "means the output is not dimmed."]
            #[doc = ""]
            #[doc = "This is supported only when the \"brightness\" capability is"]
            #[doc = "also supported."]
            fn set_dimming(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                multiplier: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the source output that the outputdevice should mirror its"]
            #[doc = "viewport from."]
            fn set_replication_source(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                source: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_ddc_ci_allowed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                allowed: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This limits the amount of bits per color that are sent to the display."]
            fn set_max_bits_per_color(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                max_bpc: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "When EDR is enabled, the compositor may increase the backlight beyond"]
            #[doc = "the user-specified setting, in order to present HDR content on displays"]
            #[doc = "without native HDR support."]
            #[doc = "This will usually result in better visuals, but also increases battery"]
            #[doc = "usage."]
            fn set_edr_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                policy: EdrPolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This is the sharpness modifier of the output."]
            #[doc = "0 is sharpness disabled and 10000 is the maximum sharpness"]
            fn set_sharpness(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                sharpness: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sent after the server has successfully applied the changes."]
            #[doc = "."]
            fn applied(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sent if the server rejects the changes or failed to apply them."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Describes why applying the output configuration failed. Is only"]
            #[doc = "sent before the failure event."]
            fn failure_reason(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                reason: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(reason))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let enable = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.enable({}, {})",
                                sender_id,
                                outputdevice,
                                enable
                            );
                            self.enable(connection, sender_id, outputdevice, enable)
                                .await
                        }
                        1u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let mode = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.mode({}, {})",
                                sender_id,
                                outputdevice,
                                mode
                            );
                            self.mode(connection, sender_id, outputdevice, mode).await
                        }
                        2u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let transform = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.transform({}, {})",
                                sender_id,
                                outputdevice,
                                transform
                            );
                            self.transform(connection, sender_id, outputdevice, transform)
                                .await
                        }
                        3u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.position({}, {}, {})",
                                sender_id,
                                outputdevice,
                                x,
                                y
                            );
                            self.position(connection, sender_id, outputdevice, x, y)
                                .await
                        }
                        4u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let scale = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.scale({}, {})",
                                sender_id,
                                outputdevice,
                                scale
                            );
                            self.scale(connection, sender_id, outputdevice, scale).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_output_configuration_v2#{}.apply()", sender_id,);
                            self.apply(connection, sender_id).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_output_configuration_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        7u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let overscan = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.overscan({}, {})",
                                sender_id,
                                outputdevice,
                                overscan
                            );
                            self.overscan(connection, sender_id, outputdevice, overscan)
                                .await
                        }
                        8u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let policy = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_vrr_policy({}, {})",
                                sender_id,
                                outputdevice,
                                policy
                            );
                            self.set_vrr_policy(
                                connection,
                                sender_id,
                                outputdevice,
                                policy.try_into()?,
                            )
                            .await
                        }
                        9u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let rgb_range = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_rgb_range({}, {})",
                                sender_id,
                                outputdevice,
                                rgb_range
                            );
                            self.set_rgb_range(
                                connection,
                                sender_id,
                                outputdevice,
                                rgb_range.try_into()?,
                            )
                            .await
                        }
                        10u16 => {
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_primary_output({})",
                                sender_id,
                                output
                            );
                            self.set_primary_output(connection, sender_id, output).await
                        }
                        11u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let priority = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_priority({}, {})",
                                sender_id,
                                outputdevice,
                                priority
                            );
                            self.set_priority(connection, sender_id, outputdevice, priority)
                                .await
                        }
                        12u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let enable_hdr = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_high_dynamic_range({}, {})",
                                sender_id,
                                outputdevice,
                                enable_hdr
                            );
                            self.set_high_dynamic_range(
                                connection,
                                sender_id,
                                outputdevice,
                                enable_hdr,
                            )
                            .await
                        }
                        13u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let sdr_brightness = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_sdr_brightness({}, {})",
                                sender_id,
                                outputdevice,
                                sdr_brightness
                            );
                            self.set_sdr_brightness(
                                connection,
                                sender_id,
                                outputdevice,
                                sdr_brightness,
                            )
                            .await
                        }
                        14u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let enable_wcg = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_wide_color_gamut({}, {})",
                                sender_id,
                                outputdevice,
                                enable_wcg
                            );
                            self.set_wide_color_gamut(
                                connection,
                                sender_id,
                                outputdevice,
                                enable_wcg,
                            )
                            .await
                        }
                        15u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let policy = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_auto_rotate_policy({}, {})",
                                sender_id,
                                outputdevice,
                                policy
                            );
                            self.set_auto_rotate_policy(
                                connection,
                                sender_id,
                                outputdevice,
                                policy.try_into()?,
                            )
                            .await
                        }
                        16u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let profile_path = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_icc_profile_path({}, \"{}\")",
                                sender_id,
                                outputdevice,
                                profile_path
                            );
                            self.set_icc_profile_path(
                                connection,
                                sender_id,
                                outputdevice,
                                profile_path,
                            )
                            .await
                        }
                        17u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let max_peak_brightness = message.int()?;
                            let max_frame_average_brightness = message.int()?;
                            let min_brightness = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_brightness_overrides({}, {}, {}, {})",
                                sender_id,
                                outputdevice,
                                max_peak_brightness,
                                max_frame_average_brightness,
                                min_brightness
                            );
                            self.set_brightness_overrides(
                                connection,
                                sender_id,
                                outputdevice,
                                max_peak_brightness,
                                max_frame_average_brightness,
                                min_brightness,
                            )
                            .await
                        }
                        18u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let gamut_wideness = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_sdr_gamut_wideness({}, {})",
                                sender_id,
                                outputdevice,
                                gamut_wideness
                            );
                            self.set_sdr_gamut_wideness(
                                connection,
                                sender_id,
                                outputdevice,
                                gamut_wideness,
                            )
                            .await
                        }
                        19u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let color_profile_source = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_color_profile_source({}, {})",
                                sender_id,
                                outputdevice,
                                color_profile_source
                            );
                            self.set_color_profile_source(
                                connection,
                                sender_id,
                                outputdevice,
                                color_profile_source.try_into()?,
                            )
                            .await
                        }
                        20u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let brightness = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_brightness({}, {})",
                                sender_id,
                                outputdevice,
                                brightness
                            );
                            self.set_brightness(connection, sender_id, outputdevice, brightness)
                                .await
                        }
                        21u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let preference = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_color_power_tradeoff({}, {})",
                                sender_id,
                                outputdevice,
                                preference
                            );
                            self.set_color_power_tradeoff(
                                connection,
                                sender_id,
                                outputdevice,
                                preference.try_into()?,
                            )
                            .await
                        }
                        22u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let multiplier = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_dimming({}, {})",
                                sender_id,
                                outputdevice,
                                multiplier
                            );
                            self.set_dimming(connection, sender_id, outputdevice, multiplier)
                                .await
                        }
                        23u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let source = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_replication_source({}, \"{}\")",
                                sender_id,
                                outputdevice,
                                source
                            );
                            self.set_replication_source(connection, sender_id, outputdevice, source)
                                .await
                        }
                        24u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let allowed = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_ddc_ci_allowed({}, {})",
                                sender_id,
                                outputdevice,
                                allowed
                            );
                            self.set_ddc_ci_allowed(connection, sender_id, outputdevice, allowed)
                                .await
                        }
                        25u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let max_bpc = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_max_bits_per_color({}, {})",
                                sender_id,
                                outputdevice,
                                max_bpc
                            );
                            self.set_max_bits_per_color(
                                connection,
                                sender_id,
                                outputdevice,
                                max_bpc,
                            )
                            .await
                        }
                        26u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let policy = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_edr_policy({}, {})",
                                sender_id,
                                outputdevice,
                                policy
                            );
                            self.set_edr_policy(
                                connection,
                                sender_id,
                                outputdevice,
                                policy.try_into()?,
                            )
                            .await
                        }
                        27u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let sharpness = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_output_configuration_v2#{}.set_sharpness({}, {})",
                                sender_id,
                                outputdevice,
                                sharpness
                            );
                            self.set_sharpness(connection, sender_id, outputdevice, sharpness)
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
pub mod kde_output_order_v1 {
    #[doc = "Announce the order in which desktop environment components should be placed on outputs."]
    #[doc = "The compositor will send the list of outputs when the global is bound and whenever there is a change."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_output_order_v1 {
        #[doc = "Trait to implement the kde_output_order_v1 interface. See the module level documentation for more info"]
        pub trait KdeOutputOrderV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_output_order_v1";
            const VERSION: u32 = 1u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Specifies the output identified by their wl_output.name."]
            fn output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Specifies that the output list is complete. On the next output event, a new list begins."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_output_order_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod kde_primary_output_v1 {
    #[doc = "Protocol for telling which is the primary display among the selection"]
    #[doc = "of enabled outputs."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_primary_output_v1 {
        #[doc = "Trait to implement the kde_primary_output_v1 interface. See the module level documentation for more info"]
        pub trait KdePrimaryOutputV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_primary_output_v1";
            const VERSION: u32 = 2u32;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Specifies which output is the primary one identified by their uuid. See kde_output_device_v2 uuid event for more information about it."]
            fn primary_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(output_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_primary_output_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod kde_screen_edge_v1 {
    #[doc = "This interface allows clients to associate actions with screen edges. For"]
    #[doc = "example, showing a surface by moving the pointer to a screen edge."]
    #[doc = ""]
    #[doc = "Potential ways to trigger the screen edge are subject to compositor"]
    #[doc = "policies. As an example, the compositor may consider the screen edge to be"]
    #[doc = "triggered if the pointer hits its associated screen border. Other ways may"]
    #[doc = "include using touchscreen or touchpad gestures."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_screen_edge_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the specified border value is invalid"]
            InvalidBorder = 0u32,
            #[doc = "the surface has invalid role"]
            InvalidRole = 1u32,
            #[doc = "the surface already has a screen edge"]
            AlreadyConstructed = 2u32,
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
                    0u32 => Ok(Self::InvalidBorder),
                    1u32 => Ok(Self::InvalidRole),
                    2u32 => Ok(Self::AlreadyConstructed),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These values describe possible screen borders."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Border {
            #[doc = "top screen edge"]
            Top = 1u32,
            #[doc = "bottom screen edge"]
            Bottom = 2u32,
            #[doc = "left screen edge"]
            Left = 3u32,
            #[doc = "right screen edge"]
            Right = 4u32,
        }
        impl From<Border> for u32 {
            fn from(value: Border) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Border {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Bottom),
                    3u32 => Ok(Self::Left),
                    4u32 => Ok(Self::Right),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Border {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the kde_screen_edge_manager_v1 interface. See the module level documentation for more info"]
        pub trait KdeScreenEdgeManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_screen_edge_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the screen edge manager. This doesn't destroy objects created"]
            #[doc = "with this manager."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a new auto hide screen edge object associated with the specified"]
            #[doc = "surface and the border."]
            #[doc = ""]
            #[doc = "Creating a kde_auto_hide_screen_edge_v1 object does not change the"]
            #[doc = "visibility of the surface. The kde_auto_hide_screen_edge_v1.activate"]
            #[doc = "request must be issued in order to hide the surface."]
            #[doc = ""]
            #[doc = "The \"border\" argument must be a valid enum entry, otherwise the"]
            #[doc = "invalid_border protocol error is raised."]
            #[doc = ""]
            #[doc = "The invalid_role protocol error will be raised if the specified surface"]
            #[doc = "does not have layer_surface role."]
            fn get_auto_hide_screen_edge(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                border: Border,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_screen_edge_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let border = message.uint()?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_screen_edge_manager_v1#{}.get_auto_hide_screen_edge({}, {}, {})",
                                sender_id,
                                id,
                                border,
                                surface
                            );
                            self.get_auto_hide_screen_edge(
                                connection,
                                sender_id,
                                id,
                                border.try_into()?,
                                surface,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The auto hide screen edge object allows to hide the surface and make it"]
    #[doc = "visible by triggering the screen edge. The screen edge is inactive and"]
    #[doc = "the surface is visible by default."]
    #[doc = ""]
    #[doc = "This interface can be used to implement user interface elements such as"]
    #[doc = "auto-hide panels or docks."]
    #[doc = ""]
    #[doc = "kde_auto_hide_screen_edge_v1.activate activates the screen edge and makes"]
    #[doc = "the surface hidden. The surface can be made visible by triggering the"]
    #[doc = "screen edge or calling kde_auto_hide_screen_edge_v1.deactivate."]
    #[doc = ""]
    #[doc = "If the screen edge has been triggered, it won't be re-activated again."]
    #[doc = "Another kde_auto_hide_screen_edge_v1.activate request must be made by the"]
    #[doc = "client to activate the screen edge."]
    #[allow(clippy::too_many_arguments)]
    pub mod kde_auto_hide_screen_edge_v1 {
        #[doc = "Trait to implement the kde_auto_hide_screen_edge_v1 interface. See the module level documentation for more info"]
        pub trait KdeAutoHideScreenEdgeV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "kde_auto_hide_screen_edge_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the auto hide screen edge object. If the screen edge is active,"]
            #[doc = "it will be deactivated and the surface will be made visible."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Deactivate the screen edge. The surface will be made visible."]
            fn deactivate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Activate the screen edge. The surface will be hidden until the screen"]
            #[doc = "edge is triggered."]
            fn activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("kde_auto_hide_screen_edge_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_auto_hide_screen_edge_v1#{}.deactivate()",
                                sender_id,
                            );
                            self.deactivate(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "kde_auto_hide_screen_edge_v1#{}.activate()",
                                sender_id,
                            );
                            self.activate(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod keystate {
    #[doc = "Keeps track of the states of the different keys that have a state attached to it."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_keystate {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Key {
            Capslock = 0u32,
            Numlock = 1u32,
            Scrolllock = 2u32,
            Alt = 3u32,
            Control = 4u32,
            Shift = 5u32,
            Meta = 6u32,
            Altgr = 7u32,
        }
        impl From<Key> for u32 {
            fn from(value: Key) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Key {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Capslock),
                    1u32 => Ok(Self::Numlock),
                    2u32 => Ok(Self::Scrolllock),
                    3u32 => Ok(Self::Alt),
                    4u32 => Ok(Self::Control),
                    5u32 => Ok(Self::Shift),
                    6u32 => Ok(Self::Meta),
                    7u32 => Ok(Self::Altgr),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Key {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            Unlocked = 0u32,
            Latched = 1u32,
            Locked = 2u32,
            Pressed = 3u32,
        }
        impl From<State> for u32 {
            fn from(value: State) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for State {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unlocked),
                    1u32 => Ok(Self::Latched),
                    2u32 => Ok(Self::Locked),
                    3u32 => Ok(Self::Pressed),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_keystate interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinKeystate
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_keystate";
            const VERSION: u32 = 5u32;
            fn fetch_states(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn state_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                key: u32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(key)
                        .put_uint(state)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_keystate#{}.fetch_states()", sender_id,);
                            self.fetch_states(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_keystate#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod org_kde_plasma_virtual_desktop {
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_virtual_desktop_management {
        #[doc = "Trait to implement the org_kde_plasma_virtual_desktop_management interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaVirtualDesktopManagement
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_virtual_desktop_management";
            const VERSION: u32 = 3u32;
            #[doc = "Given the id of a particular virtual desktop, get the corresponding org_kde_plasma_virtual_desktop which represents only the desktop with that id."]
            #[doc = ""]
            #[doc = "Warning! The protocol described in this file is a desktop environment"]
            #[doc = "implementation detail. Regular clients must not use this protocol."]
            #[doc = "Backward incompatible changes may be added without bumping the major"]
            #[doc = "version of the extension."]
            fn get_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                desktop_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Ask the server to create a new virtual desktop, and position it at a specified position. If the position is zero or less, it will be positioned at the beginning, if the position is the count or more, it will be positioned at the end."]
            fn request_create_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                position: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Ask the server to get rid of a virtual desktop, the server may or may not acconsent to the request."]
            fn request_remove_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                desktop_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn desktop_created(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                desktop_id: String,
                position: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(desktop_id))
                        .put_uint(position)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn desktop_removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                desktop_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(desktop_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after all other properties have been sent after"]
            #[doc = "binding to the desktop manager global and after all changes to"]
            #[doc = "org_kde_plasma_virtual_desktop_management and org_kde_plasma_virtual_desktop"]
            #[doc = "properties have been sent."]
            #[doc = ""]
            #[doc = "This allows changes to org_kde_plasma_virtual_desktop_management and"]
            #[doc = "org_kde_plasma_virtual_desktop properties to be seen as atomic, even"]
            #[doc = "if they happen via multiple events."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn rows(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rows: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(rows).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let desktop_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_virtual_desktop_management#{}.get_virtual_desktop({}, \"{}\")",
                                sender_id,
                                id,
                                desktop_id
                            );
                            self.get_virtual_desktop(connection, sender_id, id, desktop_id)
                                .await
                        }
                        1u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let position = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_virtual_desktop_management#{}.request_create_virtual_desktop(\"{}\", {})",
                                sender_id,
                                name,
                                position
                            );
                            self.request_create_virtual_desktop(
                                connection, sender_id, name, position,
                            )
                            .await
                        }
                        2u16 => {
                            let desktop_id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_virtual_desktop_management#{}.request_remove_virtual_desktop(\"{}\")",
                                sender_id,
                                desktop_id
                            );
                            self.request_remove_virtual_desktop(connection, sender_id, desktop_id)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_virtual_desktop {
        #[doc = "Trait to implement the org_kde_plasma_virtual_desktop interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaVirtualDesktop
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_virtual_desktop";
            const VERSION: u32 = 3u32;
            #[doc = "Request the server to set the status of this desktop to active: The server is free to consent or deny the request. This will be the new \"current\" virtual desktop of the system."]
            fn request_activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The format of the id is decided by the compositor implementation. A desktop id univocally identifies a virtual desktop and must be guaranteed to never exist two desktops with the same id. The format of the string id is up to the server implementation."]
            fn desktop_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                desktop_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(desktop_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn name(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The desktop will be the new \"current\" desktop of the system. The server may support either one virtual desktop active at a time, or other combinations such as one virtual desktop active per screen."]
            #[doc = "Windows associated to this virtual desktop will be shown."]
            fn activated(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Windows that were associated only to this desktop will be hidden."]
            fn deactivated(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after all other properties has been"]
            #[doc = "sent after binding to the desktop object and after any"]
            #[doc = "other property changes done after that. This allows"]
            #[doc = "changes to the org_kde_plasma_virtual_desktop properties to be seen as"]
            #[doc = "atomic, even if they happen via multiple events."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This virtual desktop has just been removed by the server:"]
            #[doc = "All windows will lose the association to this desktop."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The position of the virtual desktop in the desktop list. The virtual"]
            #[doc = "desktop position is in the [0, N - 1] range, where N is the number of"]
            #[doc = "virtual desktops."]
            fn position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(index).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_virtual_desktop#{}.request_activate()",
                                sender_id,
                            );
                            self.request_activate(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod outputmanagement {
    #[doc = "This interface enables clients to set properties of output devices for screen"]
    #[doc = "configuration purposes via the server. To this end output devices are referenced"]
    #[doc = "by global org_kde_kwin_outputdevice objects."]
    #[doc = ""]
    #[doc = "outputmanagement (wl_global)"]
    #[doc = "--------------------------"]
    #[doc = "request:"]
    #[doc = "* create_configuration -> outputconfiguration (wl_resource)"]
    #[doc = ""]
    #[doc = "outputconfiguration (wl_resource)"]
    #[doc = "--------------------------"]
    #[doc = "requests:"]
    #[doc = "* enable(outputdevice, bool)"]
    #[doc = "* mode(outputdevice, mode_id)"]
    #[doc = "* transformation(outputdevice, flag)"]
    #[doc = "* position(outputdevice, x, y)"]
    #[doc = "* apply"]
    #[doc = ""]
    #[doc = "events:"]
    #[doc = "* applied"]
    #[doc = "* failed"]
    #[doc = ""]
    #[doc = "The server registers one outputmanagement object as a global object. In order"]
    #[doc = "to configure outputs a client requests create_configuration, which provides a"]
    #[doc = "resource referencing an outputconfiguration for one-time configuration. That"]
    #[doc = "way the server knows which requests belong together and can group them by that."]
    #[doc = ""]
    #[doc = "On the outputconfiguration object the client calls for each output whether the"]
    #[doc = "output should be enabled, which mode should be set (by referencing the mode from"]
    #[doc = "the list of announced modes) and the output's global position. Once all outputs"]
    #[doc = "are configured that way, the client calls apply."]
    #[doc = "At that point and not earlier the server should try to apply the configuration."]
    #[doc = "If this succeeds the server emits the applied signal, otherwise the failed"]
    #[doc = "signal, such that the configuring client is noticed about the success of its"]
    #[doc = "configuration request."]
    #[doc = ""]
    #[doc = "Through this design the interface enables atomic output configuration changes if"]
    #[doc = "internally supported by the server."]
    #[doc = ""]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_outputmanagement {
        #[doc = "Trait to implement the org_kde_kwin_outputmanagement interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinOutputmanagement
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_outputmanagement";
            const VERSION: u32 = 4u32;
            #[doc = "Request an outputconfiguration object through which the client can configure"]
            #[doc = "output devices."]
            fn create_configuration(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputmanagement#{}.create_configuration({})",
                                sender_id,
                                id
                            );
                            self.create_configuration(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "outputconfiguration is a client-specific resource that can be used to ask"]
    #[doc = "the server to apply changes to available output devices."]
    #[doc = ""]
    #[doc = "The client receives a list of output devices from the registry. When it wants"]
    #[doc = "to apply new settings, it creates a configuration object from the"]
    #[doc = "outputmanagement global, writes changes through this object's enable, scale,"]
    #[doc = "transform and mode calls. It then asks the server to apply these settings in"]
    #[doc = "an atomic fashion, for example through Linux' DRM interface."]
    #[doc = ""]
    #[doc = "The server signals back whether the new settings have applied successfully"]
    #[doc = "or failed to apply. outputdevice objects are updated after the changes have been"]
    #[doc = "applied to the hardware and before the server side sends the applied event."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_outputconfiguration {
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl From<VrrPolicy> for u32 {
            fn from(value: VrrPolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for VrrPolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_outputconfiguration interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinOutputconfiguration
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_outputconfiguration";
            const VERSION: u32 = 4u32;
            #[doc = "Mark the output as enabled or disabled."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                enable: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the mode for a given output by its mode size (width and height) and refresh rate."]
            fn mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                mode_id: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the transformation for a given output."]
            fn transform(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                transform: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the position for this output device. (x,y) describe the top-left corner"]
            #[doc = "of the output in global space, whereby the origin (0,0) of the global space"]
            #[doc = "has to be aligned with the top-left corner of the most left and in case this"]
            #[doc = "does not define a single one the top output."]
            #[doc = ""]
            #[doc = "There may be no gaps or overlaps between outputs, i.e. the outputs are"]
            #[doc = "stacked horizontally, vertically, or both on each other."]
            fn position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the scaling factor for this output device."]
            fn scale(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                scale: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Asks the server to apply property changes requested through this outputconfiguration"]
            #[doc = "object to all outputs on the server side."]
            fn apply(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the scaling factor for this output device."]
            #[doc = "Sending both scale and scalef is undefined."]
            fn scalef(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                scale: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set color curves of output devices through RGB color ramps. Allows color"]
            #[doc = "correction of output device from user space."]
            #[doc = ""]
            #[doc = "These are the raw values. A compositor might opt to adjust these values"]
            #[doc = "internally, for example to shift color temperature at night."]
            fn colorcurves(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                red: Vec<u8>,
                green: Vec<u8>,
                blue: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the overscan value of this output device with a value in percent."]
            fn overscan(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                overscan: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set what policy the compositor should employ regarding its use of"]
            #[doc = "variable refresh rate."]
            fn set_vrr_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                outputdevice: waynest::ObjectId,
                policy: VrrPolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sent after the server has successfully applied the changes."]
            #[doc = "."]
            fn applied(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sent if the server rejects the changes or failed to apply them."]
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let enable = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.enable({}, {})",
                                sender_id,
                                outputdevice,
                                enable
                            );
                            self.enable(connection, sender_id, outputdevice, enable)
                                .await
                        }
                        1u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let mode_id = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.mode({}, {})",
                                sender_id,
                                outputdevice,
                                mode_id
                            );
                            self.mode(connection, sender_id, outputdevice, mode_id)
                                .await
                        }
                        2u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let transform = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.transform({}, {})",
                                sender_id,
                                outputdevice,
                                transform
                            );
                            self.transform(connection, sender_id, outputdevice, transform)
                                .await
                        }
                        3u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.position({}, {}, {})",
                                sender_id,
                                outputdevice,
                                x,
                                y
                            );
                            self.position(connection, sender_id, outputdevice, x, y)
                                .await
                        }
                        4u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let scale = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.scale({}, {})",
                                sender_id,
                                outputdevice,
                                scale
                            );
                            self.scale(connection, sender_id, outputdevice, scale).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.apply()",
                                sender_id,
                            );
                            self.apply(connection, sender_id).await
                        }
                        6u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let scale = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.scalef({}, {})",
                                sender_id,
                                outputdevice,
                                scale
                            );
                            self.scalef(connection, sender_id, outputdevice, scale)
                                .await
                        }
                        7u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let red = message.array()?;
                            let green = message.array()?;
                            let blue = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.colorcurves({}, array[{}], array[{}], array[{}])",
                                sender_id,
                                outputdevice,
                                red.len(),
                                green.len(),
                                blue.len()
                            );
                            self.colorcurves(connection, sender_id, outputdevice, red, green, blue)
                                .await
                        }
                        8u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        9u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let overscan = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.overscan({}, {})",
                                sender_id,
                                outputdevice,
                                overscan
                            );
                            self.overscan(connection, sender_id, outputdevice, overscan)
                                .await
                        }
                        10u16 => {
                            let outputdevice = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let policy = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_outputconfiguration#{}.set_vrr_policy({}, {})",
                                sender_id,
                                outputdevice,
                                policy
                            );
                            self.set_vrr_policy(
                                connection,
                                sender_id,
                                outputdevice,
                                policy.try_into()?,
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
pub mod org_kde_kwin_outputdevice {
    #[doc = "An outputdevice describes a display device available to the compositor."]
    #[doc = "outputdevice is similar to wl_output, but focuses on output"]
    #[doc = "configuration management."]
    #[doc = ""]
    #[doc = "A client can query all global outputdevice objects to enlist all"]
    #[doc = "available display devices, even those that may currently not be"]
    #[doc = "represented by the compositor as a wl_output."]
    #[doc = ""]
    #[doc = "The client sends configuration changes to the server through the"]
    #[doc = "outputconfiguration interface, and the server applies the configuration"]
    #[doc = "changes to the hardware and signals changes to the outputdevices"]
    #[doc = "accordingly."]
    #[doc = ""]
    #[doc = "This object is published as global during start up for every available"]
    #[doc = "display devices, or when one later becomes available, for example by"]
    #[doc = "being hotplugged via a physical connector."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_outputdevice {
        #[doc = "This enumeration describes how the physical pixels on an output are"]
        #[doc = "laid out."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Subpixel {
            Unknown = 0u32,
            None = 1u32,
            HorizontalRgb = 2u32,
            HorizontalBgr = 3u32,
            VerticalRgb = 4u32,
            VerticalBgr = 5u32,
        }
        impl From<Subpixel> for u32 {
            fn from(value: Subpixel) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Subpixel {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Unknown),
                    1u32 => Ok(Self::None),
                    2u32 => Ok(Self::HorizontalRgb),
                    3u32 => Ok(Self::HorizontalBgr),
                    4u32 => Ok(Self::VerticalRgb),
                    5u32 => Ok(Self::VerticalBgr),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Subpixel {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "This describes the transform, that a compositor will apply to a"]
        #[doc = "surface to compensate for the rotation or mirroring of an"]
        #[doc = "output device."]
        #[doc = ""]
        #[doc = "The flipped values correspond to an initial flip around a"]
        #[doc = "vertical axis followed by rotation."]
        #[doc = ""]
        #[doc = "The purpose is mainly to allow clients to render accordingly and"]
        #[doc = "tell the compositor, so that for fullscreen surfaces, the"]
        #[doc = "compositor is still able to scan out directly client surfaces."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Transform {
            Normal = 0u32,
            _90 = 1u32,
            _180 = 2u32,
            _270 = 3u32,
            Flipped = 4u32,
            Flipped90 = 5u32,
            Flipped180 = 6u32,
            Flipped270 = 7u32,
        }
        impl From<Transform> for u32 {
            fn from(value: Transform) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Transform {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Transform {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "These flags describe properties of an output mode. They are"]
        #[doc = "used in the flags bitfield of the mode event."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "indicates this is the current mode"]
            Current = 1u32,
            #[doc = "indicates this is the preferred mode"]
            Preferred = 2u32,
        }
        impl From<Mode> for u32 {
            fn from(value: Mode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Mode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Current),
                    2u32 => Ok(Self::Preferred),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Describes whether a device is enabled, i.e. device is used to"]
        #[doc = "display content by the compositor. This wraps a boolean around"]
        #[doc = "an int to avoid a boolean trap."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Enablement {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<Enablement> for u32 {
            fn from(value: Enablement) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Enablement {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Enablement {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Describes what capabilities this device has."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capability : u32 { # [doc = "if this outputdevice can use overscan"] const Overscan = 1u32 ; # [doc = "if this outputdevice supports variable refresh rate"] const Vrr = 2u32 ; } }
        impl From<Capability> for u32 {
            fn from(value: Capability) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Capability {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Capability {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Describes when the compositor may employ variable refresh rate"]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum VrrPolicy {
            Never = 0u32,
            Always = 1u32,
            Automatic = 2u32,
        }
        impl From<VrrPolicy> for u32 {
            fn from(value: VrrPolicy) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for VrrPolicy {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Never),
                    1u32 => Ok(Self::Always),
                    2u32 => Ok(Self::Automatic),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for VrrPolicy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_outputdevice interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinOutputdevice
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_outputdevice";
            const VERSION: u32 = 4u32;
            #[doc = "The geometry event describes geometric properties of the output."]
            #[doc = "The event is sent when binding to the output object and whenever"]
            #[doc = "any of the properties change."]
            fn geometry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                physical_width: i32,
                physical_height: i32,
                subpixel: i32,
                make: String,
                model: String,
                transform: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(physical_width)
                        .put_int(physical_height)
                        .put_int(subpixel)
                        .put_string(Some(make))
                        .put_string(Some(model))
                        .put_int(transform)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The mode event describes an available mode for the output."]
            #[doc = ""]
            #[doc = "When the client binds to the outputdevice object, the server sends this"]
            #[doc = "event once for every available mode the outputdevice can be operated by."]
            #[doc = ""]
            #[doc = "There will always be at least one event sent out on initial binding,"]
            #[doc = "which represents the current mode."]
            #[doc = ""]
            #[doc = "Later on if an output changes its mode the event is sent again, whereby"]
            #[doc = "this event represents the mode that has now become current. In other"]
            #[doc = "words, the current mode is always represented by the latest event sent"]
            #[doc = "with the current flag set."]
            #[doc = ""]
            #[doc = "The size of a mode is given in physical hardware units of the output device."]
            #[doc = "This is not necessarily the same as the output size in the global compositor"]
            #[doc = "space. For instance, the output may be scaled, as described in"]
            #[doc = "org_kde_kwin_outputdevice.scale, or transformed, as described in"]
            #[doc = "org_kde_kwin_outputdevice.transform."]
            #[doc = ""]
            #[doc = "The id can be used to refer to a mode when calling set_mode on an"]
            #[doc = "org_kde_kwin_outputconfiguration object."]
            fn mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: u32,
                width: i32,
                height: i32,
                refresh: i32,
                mode_id: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(flags)
                        .put_int(width)
                        .put_int(height)
                        .put_int(refresh)
                        .put_int(mode_id)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event is sent after all other properties have been"]
            #[doc = "sent on binding to the output object as well as after any"]
            #[doc = "other output property change have been applied later on."]
            #[doc = "This allows to see changes to the output properties as atomic,"]
            #[doc = "even if multiple events successively announce them."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. If it is not sent, the client should assume a"]
            #[doc = "scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients track the"]
            #[doc = "current output of a surface, and if it is on a scaled"]
            #[doc = "output it should use wl_surface.set_buffer_scale with"]
            #[doc = "the scale of the output. That way the compositor can"]
            #[doc = "avoid scaling the surface, and the client can supply"]
            #[doc = "a higher detail image."]
            fn scale(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                factor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(factor).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The edid event encapsulates the EDID data for the outputdevice."]
            #[doc = ""]
            #[doc = "The event is sent when binding to the output object. The EDID"]
            #[doc = "data may be empty, in which case this event is sent anyway."]
            #[doc = "If the EDID information is empty, you can fall back to the name"]
            #[doc = "et al. properties of the outputdevice."]
            fn edid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                raw: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(raw)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The enabled event notifies whether this output is currently"]
            #[doc = "enabled and used for displaying content by the server."]
            #[doc = "The event is sent when binding to the output object and"]
            #[doc = "whenever later on an output changes its state by becoming"]
            #[doc = "enabled or disabled."]
            fn enabled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                enabled: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(enabled).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The uuid can be used to identify the output. It's controlled by"]
            #[doc = "the server entirely. The server should make sure the uuid is"]
            #[doc = "persistent across restarts. An empty uuid is considered invalid."]
            fn uuid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                uuid: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(uuid))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event contains scaling geometry information"]
            #[doc = "that is not in the geometry event. It may be sent after"]
            #[doc = "binding the output object or if the output scale changes"]
            #[doc = "later. If it is not sent, the client should assume a"]
            #[doc = "scale of 1."]
            #[doc = ""]
            #[doc = "A scale larger than 1 means that the compositor will"]
            #[doc = "automatically scale surface buffers by this amount"]
            #[doc = "when rendering. This is used for high resolution"]
            #[doc = "displays where applications rendering at the native"]
            #[doc = "resolution would be too small to be legible."]
            #[doc = ""]
            #[doc = "It is intended that scaling aware clients track the"]
            #[doc = "current output of a surface, and if it is on a scaled"]
            #[doc = "output it should use wl_surface.set_buffer_scale with"]
            #[doc = "the scale of the output. That way the compositor can"]
            #[doc = "avoid scaling the surface, and the client can supply"]
            #[doc = "a higher detail image."]
            #[doc = ""]
            #[doc = "wl_output will keep the output scale as an integer. In every situation except"]
            #[doc = "configuring the window manager you want to use that."]
            fn scalef(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                factor: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_fixed(factor).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Describes the color intensity profile of the output."]
            #[doc = "Commonly used for gamma/color correction."]
            #[doc = ""]
            #[doc = "The array contains all color ramp values of the output."]
            #[doc = "For example on 8bit screens there are 256 of them."]
            #[doc = ""]
            #[doc = "The array elements are unsigned 16bit integers."]
            fn colorcurves(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                red: Vec<u8>,
                green: Vec<u8>,
                blue: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_array(red)
                        .put_array(green)
                        .put_array(blue)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Serial ID of the monitor, sent on startup before the first done event."]
            fn serial_number(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial_number: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(serial_number))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "EISA ID of the monitor, sent on startup before the first done event."]
            fn eisa_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                eisa_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(eisa_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "What capabilities this device has, sent on startup before the first"]
            #[doc = "done event."]
            fn capabilities(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: Capability,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(flags.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Overscan value of the monitor in percent, sent on startup before the"]
            #[doc = "first done event."]
            fn overscan(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                overscan: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(overscan).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "What policy the compositor will employ regarding its use of variable"]
            #[doc = "refresh rate."]
            fn vrr_policy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                vrr_policy: VrrPolicy,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(vrr_policy.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 13u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod plasma_shell {
    #[doc = "This interface is used by KF5 powered Wayland shells to communicate with"]
    #[doc = "the compositor and can only be bound one time."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_shell {
        #[doc = "Trait to implement the org_kde_plasma_shell interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaShell
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_shell";
            const VERSION: u32 = 8u32;
            #[doc = "Create a shell surface for an existing surface."]
            #[doc = ""]
            #[doc = "Only one shell surface can be associated with a given"]
            #[doc = "surface."]
            fn get_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_shell#{}.get_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_surface(connection, sender_id, id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An interface that may be implemented by a wl_surface, for"]
    #[doc = "implementations that provide the shell user interface."]
    #[doc = ""]
    #[doc = "It provides requests to set surface roles, assign an output"]
    #[doc = "or set the position in output coordinates."]
    #[doc = ""]
    #[doc = "On the server side the object is automatically destroyed when"]
    #[doc = "the related wl_surface is destroyed.  On client side,"]
    #[doc = "org_kde_plasma_surface.destroy() must be called before"]
    #[doc = "destroying the wl_surface object."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_surface {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Role {
            Normal = 0u32,
            Desktop = 1u32,
            Panel = 2u32,
            Onscreendisplay = 3u32,
            Notification = 4u32,
            Tooltip = 5u32,
            Criticalnotification = 6u32,
            Appletpopup = 7u32,
        }
        impl From<Role> for u32 {
            fn from(value: Role) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Role {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::Desktop),
                    2u32 => Ok(Self::Panel),
                    3u32 => Ok(Self::Onscreendisplay),
                    4u32 => Ok(Self::Notification),
                    5u32 => Ok(Self::Tooltip),
                    6u32 => Ok(Self::Criticalnotification),
                    7u32 => Ok(Self::Appletpopup),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Role {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PanelBehavior {
            AlwaysVisible = 1u32,
            AutoHide = 2u32,
            WindowsCanCover = 3u32,
            WindowsGoBelow = 4u32,
        }
        impl From<PanelBehavior> for u32 {
            fn from(value: PanelBehavior) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PanelBehavior {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::AlwaysVisible),
                    2u32 => Ok(Self::AutoHide),
                    3u32 => Ok(Self::WindowsCanCover),
                    4u32 => Ok(Self::WindowsGoBelow),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PanelBehavior {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "Request panel_auto_hide performed on a surface which does not correspond to an auto-hide panel."]
            PanelNotAutoHide = 0u32,
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
                    0u32 => Ok(Self::PanelNotAutoHide),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_plasma_surface interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaSurface
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_surface";
            const VERSION: u32 = 8u32;
            #[doc = "The org_kde_plasma_surface interface is removed from the"]
            #[doc = "wl_surface object that was turned into a shell surface with the"]
            #[doc = "org_kde_plasma_shell.get_surface request."]
            #[doc = "The shell surface role is lost and wl_surface is unmapped."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Assign an output to this shell surface."]
            #[doc = "The compositor will use this information to set the position"]
            #[doc = "when org_kde_plasma_surface.set_position request is"]
            #[doc = "called."]
            fn set_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Move the surface to new coordinates."]
            #[doc = ""]
            #[doc = "Coordinates are global, for example 50,50 for a 1920,0+1920x1080 output"]
            #[doc = "is 1970,50 in global coordinates space."]
            #[doc = ""]
            #[doc = "Use org_kde_plasma_surface.set_output to assign an output"]
            #[doc = "to this surface."]
            fn set_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Assign a role to a shell surface."]
            #[doc = ""]
            #[doc = "The compositor handles surfaces depending on their role."]
            #[doc = "See the explanation below."]
            #[doc = ""]
            #[doc = "This request fails if the surface already has a role, this means"]
            #[doc = "the surface role may be assigned only once."]
            #[doc = ""]
            #[doc = "== Surfaces with splash role =="]
            #[doc = ""]
            #[doc = "Splash surfaces are placed above every other surface during the"]
            #[doc = "shell startup phase."]
            #[doc = ""]
            #[doc = "The surfaces are placed according to the output coordinates."]
            #[doc = "No size is imposed to those surfaces, the shell has to resize"]
            #[doc = "them according to output size."]
            #[doc = ""]
            #[doc = "These surfaces are meant to hide the desktop during the startup"]
            #[doc = "phase so that the user will always see a ready to work desktop."]
            #[doc = ""]
            #[doc = "A shell might not create splash surfaces if the compositor reveals"]
            #[doc = "the desktop in an alternative fashion, for example with a fade"]
            #[doc = "in effect."]
            #[doc = ""]
            #[doc = "That depends on how much time the desktop usually need to prepare"]
            #[doc = "the workspace or specific design decisions."]
            #[doc = "This specification doesn't impose any particular design."]
            #[doc = ""]
            #[doc = "When the startup phase is finished, the shell will send the"]
            #[doc = "org_kde_plasma.desktop_ready request to the compositor."]
            #[doc = ""]
            #[doc = "== Surfaces with desktop role =="]
            #[doc = ""]
            #[doc = "Desktop surfaces are placed below all other surfaces and are used"]
            #[doc = "to show the actual desktop view with icons, search results or"]
            #[doc = "controls the user will interact with. What to show depends on the"]
            #[doc = "shell implementation."]
            #[doc = ""]
            #[doc = "The surfaces are placed according to the output coordinates."]
            #[doc = "No size is imposed to those surfaces, the shell has to resize"]
            #[doc = "them according to output size."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the desktop role."]
            #[doc = ""]
            #[doc = "== Surfaces with dashboard role =="]
            #[doc = ""]
            #[doc = "Dashboard surfaces are placed above desktop surfaces and are used to"]
            #[doc = "show additional widgets and controls."]
            #[doc = ""]
            #[doc = "The surfaces are placed according to the output coordinates."]
            #[doc = "No size is imposed to those surfaces, the shell has to resize"]
            #[doc = "them according to output size."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the dashboard role."]
            #[doc = ""]
            #[doc = "== Surfaces with config role =="]
            #[doc = ""]
            #[doc = "A configuration surface is shown when the user wants to configure"]
            #[doc = "panel or desktop views."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the config role."]
            #[doc = ""]
            #[doc = "TODO: This should grab the input like popup menus, right?"]
            #[doc = ""]
            #[doc = "== Surfaces with overlay role =="]
            #[doc = ""]
            #[doc = "Overlays are special surfaces that shows for a limited amount"]
            #[doc = "of time.  Such surfaces are useful to display things like volume,"]
            #[doc = "brightness and status changes."]
            #[doc = ""]
            #[doc = "Compositors may decide to show those surfaces in a layer above"]
            #[doc = "all surfaces, even full screen ones if so is desired."]
            #[doc = ""]
            #[doc = "== Surfaces with notification role =="]
            #[doc = ""]
            #[doc = "Notification surfaces display informative content for a limited"]
            #[doc = "amount of time.  The compositor may decide to show them in a corner"]
            #[doc = "depending on the configuration."]
            #[doc = ""]
            #[doc = "These surfaces are shown in a layer above all other surfaces except"]
            #[doc = "for full screen ones."]
            #[doc = ""]
            #[doc = "== Surfaces with lock role =="]
            #[doc = ""]
            #[doc = "The lock surface is shown by the compositor when the session is"]
            #[doc = "locked, users interact with it to unlock the session."]
            #[doc = ""]
            #[doc = "Compositors should move lock surfaces to 0,0 in output"]
            #[doc = "coordinates space and hide all other surfaces for security sake."]
            #[doc = "For the same reason it is recommended that clients make the"]
            #[doc = "lock surface as big as the screen."]
            #[doc = ""]
            #[doc = "Only one surface per output can have the lock role."]
            fn set_role(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                role: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set flags bitmask as described by the flag enum."]
            #[doc = "Pass 0 to unset any flag, the surface will adjust its behavior to"]
            #[doc = "the default."]
            #[doc = ""]
            #[doc = "Deprecated in Plasma 6. Setting this flag will have no effect. Applications should use layer shell where appropriate."]
            fn set_panel_behavior(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flag: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Setting this bit to the window, will make it say it prefers to not be listed in the taskbar. Taskbar implementations may or may not follow this hint."]
            fn set_skip_taskbar(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A panel surface with panel_behavior auto_hide can perform this request to hide the panel"]
            #[doc = "on a screen edge without unmapping it. The compositor informs the client about the panel"]
            #[doc = "being hidden with the event auto_hidden_panel_hidden."]
            #[doc = ""]
            #[doc = "The compositor will restore the visibility state of the"]
            #[doc = "surface when the pointer touches the screen edge the panel borders. Once the compositor restores"]
            #[doc = "the visibility the event auto_hidden_panel_shown will be sent. This event will also be sent"]
            #[doc = "if the compositor is unable to hide the panel."]
            #[doc = ""]
            #[doc = "The client can also request to show the panel again with the request panel_auto_hide_show."]
            fn panel_auto_hide_hide(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A panel surface with panel_behavior auto_hide can perform this request to show the panel"]
            #[doc = "again which got hidden with panel_auto_hide_hide."]
            fn panel_auto_hide_show(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "By default various org_kde_plasma_surface roles do not take focus and cannot be"]
            #[doc = "activated. With this request the compositor can be instructed to pass focus also to this"]
            #[doc = "org_kde_plasma_surface."]
            fn set_panel_takes_focus(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                takes_focus: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Setting this bit will indicate that the window prefers not to be listed in a switcher."]
            fn set_skip_switcher(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                skip: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Request the initial position of this surface to be under the current"]
            #[doc = "cursor position. Has to be called before attaching any buffer to this surface."]
            fn open_under_cursor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "An auto-hiding panel got hidden by the compositor."]
            fn auto_hidden_panel_hidden(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "An auto-hiding panel got shown by the compositor."]
            fn auto_hidden_panel_shown(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_plasma_surface#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_output({})",
                                sender_id,
                                output
                            );
                            self.set_output(connection, sender_id, output).await
                        }
                        2u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_position({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.set_position(connection, sender_id, x, y).await
                        }
                        3u16 => {
                            let role = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_role({})",
                                sender_id,
                                role
                            );
                            self.set_role(connection, sender_id, role).await
                        }
                        4u16 => {
                            let flag = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_panel_behavior({})",
                                sender_id,
                                flag
                            );
                            self.set_panel_behavior(connection, sender_id, flag).await
                        }
                        5u16 => {
                            let skip = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_skip_taskbar({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_taskbar(connection, sender_id, skip).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.panel_auto_hide_hide()",
                                sender_id,
                            );
                            self.panel_auto_hide_hide(connection, sender_id).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.panel_auto_hide_show()",
                                sender_id,
                            );
                            self.panel_auto_hide_show(connection, sender_id).await
                        }
                        8u16 => {
                            let takes_focus = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_panel_takes_focus({})",
                                sender_id,
                                takes_focus
                            );
                            self.set_panel_takes_focus(connection, sender_id, takes_focus)
                                .await
                        }
                        9u16 => {
                            let skip = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.set_skip_switcher({})",
                                sender_id,
                                skip
                            );
                            self.set_skip_switcher(connection, sender_id, skip).await
                        }
                        10u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_surface#{}.open_under_cursor()",
                                sender_id,
                            );
                            self.open_under_cursor(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod plasma_window_management {
    #[doc = "This interface manages application windows."]
    #[doc = "It provides requests to show and hide the desktop and emits"]
    #[doc = "an event every time a window is created so that the client can"]
    #[doc = "use it to manage the window."]
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    #[doc = ""]
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_window_management {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum State {
            Active = 1u32,
            Minimized = 2u32,
            Maximized = 4u32,
            Fullscreen = 8u32,
            KeepAbove = 16u32,
            KeepBelow = 32u32,
            OnAllDesktops = 64u32,
            DemandsAttention = 128u32,
            Closeable = 256u32,
            Minimizable = 512u32,
            Maximizable = 1024u32,
            Fullscreenable = 2048u32,
            Skiptaskbar = 4096u32,
            Shadeable = 8192u32,
            Shaded = 16384u32,
            Movable = 32768u32,
            Resizable = 65536u32,
            VirtualDesktopChangeable = 131072u32,
            Skipswitcher = 262144u32,
            NoBorder = 524288u32,
            CanSetNoBorder = 1048576u32,
        }
        impl From<State> for u32 {
            fn from(value: State) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for State {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Active),
                    2u32 => Ok(Self::Minimized),
                    4u32 => Ok(Self::Maximized),
                    8u32 => Ok(Self::Fullscreen),
                    16u32 => Ok(Self::KeepAbove),
                    32u32 => Ok(Self::KeepBelow),
                    64u32 => Ok(Self::OnAllDesktops),
                    128u32 => Ok(Self::DemandsAttention),
                    256u32 => Ok(Self::Closeable),
                    512u32 => Ok(Self::Minimizable),
                    1024u32 => Ok(Self::Maximizable),
                    2048u32 => Ok(Self::Fullscreenable),
                    4096u32 => Ok(Self::Skiptaskbar),
                    8192u32 => Ok(Self::Shadeable),
                    16384u32 => Ok(Self::Shaded),
                    32768u32 => Ok(Self::Movable),
                    65536u32 => Ok(Self::Resizable),
                    131072u32 => Ok(Self::VirtualDesktopChangeable),
                    262144u32 => Ok(Self::Skipswitcher),
                    524288u32 => Ok(Self::NoBorder),
                    1048576u32 => Ok(Self::CanSetNoBorder),
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
        pub enum ShowDesktop {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<ShowDesktop> for u32 {
            fn from(value: ShowDesktop) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ShowDesktop {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ShowDesktop {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_plasma_window_management interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaWindowManagement
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_window_management";
            const VERSION: u32 = 19u32;
            #[doc = "Tell the compositor to show/hide the desktop."]
            fn show_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Deprecated: use get_window_by_uuid"]
            fn get_window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                internal_window_id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_window_by_uuid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                internal_window_uuid: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn get_stacking_order(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                stacking_order: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event will be sent whenever the show desktop mode changes. E.g. when it is entered"]
            #[doc = "or left."]
            #[doc = ""]
            #[doc = "On binding the interface the current state is sent."]
            fn show_desktop_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent immediately after a window is mapped."]
            fn window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(id).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when stacking order changed and on bind."]
            #[doc = ""]
            #[doc = "With version 17 this event is deprecated and will no longer be sent."]
            fn stacking_order_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                ids: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_array(ids).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when stacking order changed and on bind."]
            #[doc = ""]
            #[doc = "With version 17 this event is deprecated and will no longer be sent."]
            fn stacking_order_uuid_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                uuids: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(uuids))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent immediately after a window is mapped."]
            fn window_with_uuid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: u32,
                uuid: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(id)
                        .put_string(Some(uuid))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when stacking order changed."]
            fn stacking_order_changed_2(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window_management#{}.show_desktop({})",
                                sender_id,
                                state
                            );
                            self.show_desktop(connection, sender_id, state).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let internal_window_id = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window_management#{}.get_window({}, {})",
                                sender_id,
                                id,
                                internal_window_id
                            );
                            self.get_window(connection, sender_id, id, internal_window_id)
                                .await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let internal_window_uuid = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window_management#{}.get_window_by_uuid({}, \"{}\")",
                                sender_id,
                                id,
                                internal_window_uuid
                            );
                            self.get_window_by_uuid(connection, sender_id, id, internal_window_uuid)
                                .await
                        }
                        3u16 => {
                            let stacking_order = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window_management#{}.get_stacking_order({})",
                                sender_id,
                                stacking_order
                            );
                            self.get_stacking_order(connection, sender_id, stacking_order)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Manages and control an application window."]
    #[doc = ""]
    #[doc = "Only one client can bind this interface at a time."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_window {
        #[doc = "Trait to implement the org_kde_plasma_window interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaWindow
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_window";
            const VERSION: u32 = 18u32;
            #[doc = "Set window state."]
            #[doc = ""]
            #[doc = "Values for state argument are described by org_kde_plasma_window_management.state"]
            #[doc = "and can be used together in a bitfield. The flags bitfield describes which flags are"]
            #[doc = "supposed to be set, the state bitfield the value for the set flags"]
            fn set_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: u32,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Deprecated: use enter_virtual_desktop"]
            #[doc = "Maps the window to a different virtual desktop."]
            #[doc = ""]
            #[doc = "To show the window on all virtual desktops, call the"]
            #[doc = "org_kde_plasma_window.set_state request and specify a on_all_desktops"]
            #[doc = "state in the bitfield."]
            fn set_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                number: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the geometry of the taskbar entry for this window."]
            #[doc = "The geometry is relative to a panel in particular."]
            fn set_minimized_geometry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                panel: waynest::ObjectId,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove the task geometry information for a particular panel."]
            fn unset_minimized_geometry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                panel: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Close this window."]
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Request an interactive move for this window."]
            fn request_move(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Request an interactive resize for this window."]
            fn request_resize(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Removes the resource bound for this org_kde_plasma_window."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The compositor will write the window icon into the provided file descriptor."]
            #[doc = "The data is a serialized QIcon with QDataStream."]
            fn get_icon(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                fd: std::os::fd::OwnedFd,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Make the window enter a virtual desktop. A window can enter more"]
            #[doc = "than one virtual desktop. if the id is empty or invalid, no action will be performed."]
            fn request_enter_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "RFC: do this with an empty id to request_enter_virtual_desktop?"]
            #[doc = "Make the window enter a new virtual desktop. If the server consents the request,"]
            #[doc = "it will create a new virtual desktop and assign the window to it."]
            fn request_enter_new_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Make the window exit a virtual desktop. If it exits all desktops it will be considered on all of them."]
            fn request_leave_virtual_desktop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Make the window enter an activity. A window can enter more activity. If the id is empty or invalid, no action will be performed."]
            fn request_enter_activity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Make the window exit a an activity. If it exits all activities it will be considered on all of them."]
            fn request_leave_activity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests this window to be displayed in a specific output."]
            fn send_to_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event will be sent as soon as the window title is changed."]
            fn title_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                title: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(title))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent as soon as the application"]
            #[doc = "identifier is changed."]
            fn app_id_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                app_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent as soon as the window state changes."]
            #[doc = ""]
            #[doc = "Values for state argument are described by org_kde_plasma_window_management.state."]
            fn state_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(flags).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "DEPRECATED: use virtual_desktop_entered and virtual_desktop_left instead"]
            #[doc = "This event will be sent when a window is moved to another"]
            #[doc = "virtual desktop."]
            #[doc = ""]
            #[doc = "It is not sent if it becomes visible on all virtual desktops though."]
            fn virtual_desktop_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                number: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(number).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent whenever the themed icon name changes. May be null."]
            fn themed_icon_name_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent immediately after the window is closed"]
            #[doc = "and its surface is unmapped."]
            fn unmapped(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent immediately after all initial state been sent to the client."]
            #[doc = "If the Plasma window is already unmapped, the unmapped event will be sent before the"]
            #[doc = "initial_state event."]
            fn initial_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent whenever the parent window of this org_kde_plasma_window changes."]
            #[doc = "The passed parent is another org_kde_plasma_window and this org_kde_plasma_window is a"]
            #[doc = "transient window to the parent window. If the parent argument is null, this"]
            #[doc = "org_kde_plasma_window does not have a parent window."]
            fn parent_window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                parent: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_object(parent).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent whenever the window geometry of this org_kde_plasma_window changes."]
            #[doc = "The coordinates are in absolute coordinates of the windowing system."]
            fn geometry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent whenever the icon of the window changes, but there is no themed"]
            #[doc = "icon name. Common examples are Xwayland windows which have a pixmap based icon."]
            #[doc = ""]
            #[doc = "The client can request the icon using get_icon."]
            fn icon_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the compositor has set the process id this window belongs to."]
            #[doc = "This should be set once before the initial_state is sent."]
            fn pid_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                pid: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(pid).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the window has entered a new virtual desktop. The window can be on more than one desktop, or none: then is considered on all of them."]
            fn virtual_desktop_entered(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the window left a virtual desktop. If the window leaves all desktops, it can be considered on all."]
            #[doc = "If the window gets manually added on all desktops, the server has to send virtual_desktop_left for every previous desktop it was in for the window to be really considered on all desktops."]
            fn virtual_desktop_left(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                is: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(is)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent after the application menu"]
            #[doc = "for the window has changed."]
            fn application_menu(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                service_name: String,
                object_path: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(service_name))
                        .put_string(Some(object_path))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 13u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the window has entered an activity. The window can be on more than one activity, or none: then is considered on all of them."]
            fn activity_entered(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 14u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the window left an activity. If the window leaves all activities, it will be considered on all."]
            #[doc = "If the window gets manually added on all activities, the server has to send activity_left for every previous activity it was in for the window to be really considered on all activities."]
            fn activity_left(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 15u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent when the X11 resource name of the window has changed."]
            #[doc = "This is only set for XWayland windows."]
            fn resource_name_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                resource_name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(resource_name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 16u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event will be sent whenever the window geometry of this org_kde_plasma_window changes."]
            #[doc = "The coordinates are in absolute coordinates of the windowing system."]
            fn client_geometry(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: u32,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_uint(width)
                        .put_uint(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 17u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let flags = message.uint()?;
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.set_state({}, {})",
                                sender_id,
                                flags,
                                state
                            );
                            self.set_state(connection, sender_id, flags, state).await
                        }
                        1u16 => {
                            let number = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.set_virtual_desktop({})",
                                sender_id,
                                number
                            );
                            self.set_virtual_desktop(connection, sender_id, number)
                                .await
                        }
                        2u16 => {
                            let panel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let x = message.uint()?;
                            let y = message.uint()?;
                            let width = message.uint()?;
                            let height = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.set_minimized_geometry({}, {}, {}, {}, {})",
                                sender_id,
                                panel,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_minimized_geometry(
                                connection, sender_id, panel, x, y, width, height,
                            )
                            .await
                        }
                        3u16 => {
                            let panel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.unset_minimized_geometry({})",
                                sender_id,
                                panel
                            );
                            self.unset_minimized_geometry(connection, sender_id, panel)
                                .await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_plasma_window#{}.close()", sender_id,);
                            self.close(connection, sender_id).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_plasma_window#{}.request_move()", sender_id,);
                            self.request_move(connection, sender_id).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_plasma_window#{}.request_resize()", sender_id,);
                            self.request_resize(connection, sender_id).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_plasma_window#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        8u16 => {
                            let fd = waynest::Connection::fd(connection)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.get_icon({})",
                                sender_id,
                                std::os::fd::AsRawFd::as_raw_fd(&fd)
                            );
                            self.get_icon(connection, sender_id, fd).await
                        }
                        9u16 => {
                            let id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.request_enter_virtual_desktop(\"{}\")",
                                sender_id,
                                id
                            );
                            self.request_enter_virtual_desktop(connection, sender_id, id)
                                .await
                        }
                        10u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.request_enter_new_virtual_desktop()",
                                sender_id,
                            );
                            self.request_enter_new_virtual_desktop(connection, sender_id)
                                .await
                        }
                        11u16 => {
                            let id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.request_leave_virtual_desktop(\"{}\")",
                                sender_id,
                                id
                            );
                            self.request_leave_virtual_desktop(connection, sender_id, id)
                                .await
                        }
                        12u16 => {
                            let id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.request_enter_activity(\"{}\")",
                                sender_id,
                                id
                            );
                            self.request_enter_activity(connection, sender_id, id).await
                        }
                        13u16 => {
                            let id = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.request_leave_activity(\"{}\")",
                                sender_id,
                                id
                            );
                            self.request_leave_activity(connection, sender_id, id).await
                        }
                        14u16 => {
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_window#{}.send_to_output({})",
                                sender_id,
                                output
                            );
                            self.send_to_output(connection, sender_id, output).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The activation manager interface provides a way to get notified"]
    #[doc = "when an application is about to be activated."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_activation_feedback {
        #[doc = "Trait to implement the org_kde_plasma_activation_feedback interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaActivationFeedback
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_activation_feedback";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the activation manager object. The activation objects introduced"]
            #[doc = "by this manager object will be unaffected."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Will be issued when an app is set to be activated. It offers"]
            #[doc = "an instance of org_kde_plasma_activation that will tell us the app_id"]
            #[doc = "and the extent of the activation."]
            fn activation(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_plasma_activation_feedback#{}.destroy()",
                                sender_id,
                            );
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_activation {
        #[doc = "Trait to implement the org_kde_plasma_activation interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaActivation
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_activation";
            const VERSION: u32 = 1u32;
            #[doc = "Notify the compositor that the org_kde_plasma_activation object will no"]
            #[doc = "longer be used."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn app_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                app_id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(app_id))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_plasma_activation#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "When this object is created, the compositor sends a window event for"]
    #[doc = "each window in the stacking order, and afterwards sends the done event"]
    #[doc = "and destroys this object."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_plasma_stacking_order {
        #[doc = "Trait to implement the org_kde_plasma_stacking_order interface. See the module level documentation for more info"]
        pub trait OrgKdePlasmaStackingOrder
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_plasma_stacking_order";
            const VERSION: u32 = 17u32;
            fn window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                uuid: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(uuid))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                _connection: &mut Self::Connection,
                _sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod remote_access {
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_remote_access_manager {
        #[doc = "Trait to implement the org_kde_kwin_remote_access_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinRemoteAccessManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_remote_access_manager";
            const VERSION: u32 = 1u32;
            fn get_buffer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
                internal_buffer_id: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn buffer_ready(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: i32,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(id)
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let internal_buffer_id = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_remote_access_manager#{}.get_buffer({}, {})",
                                sender_id,
                                buffer,
                                internal_buffer_id
                            );
                            self.get_buffer(connection, sender_id, buffer, internal_buffer_id)
                                .await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_remote_access_manager#{}.release()",
                                sender_id,
                            );
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_remote_buffer {
        #[doc = "Trait to implement the org_kde_kwin_remote_buffer interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinRemoteBuffer
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_remote_buffer";
            const VERSION: u32 = 1u32;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn gbm_handle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                fd: std::os::fd::OwnedFd,
                width: u32,
                height: u32,
                stride: u32,
                format: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_fd(fd)
                        .put_uint(width)
                        .put_uint(height)
                        .put_uint(stride)
                        .put_uint(format)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_remote_buffer#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod server_decoration_palette {
    #[doc = "This interface allows a client to alter the palette of a server side decoration."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_server_decoration_palette_manager {
        #[doc = "Trait to implement the org_kde_kwin_server_decoration_palette_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecorationPaletteManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_palette_manager";
            const VERSION: u32 = 1u32;
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_server_decoration_palette_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This interface allows a client to alter the palette of a server side decoration."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_server_decoration_palette {
        #[doc = "Trait to implement the org_kde_kwin_server_decoration_palette interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecorationPalette
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_palette";
            const VERSION: u32 = 1u32;
            #[doc = "Color scheme that should be applied to the window decoration."]
            #[doc = "Absolute file path, or name of palette in the user's config directory."]
            #[doc = "The server may choose not to follow the requested style."]
            fn set_palette(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                palette: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let palette = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_server_decoration_palette#{}.set_palette(\"{}\")",
                                sender_id,
                                palette
                            );
                            self.set_palette(connection, sender_id, palette).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_server_decoration_palette#{}.release()",
                                sender_id,
                            );
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod server_decoration {
    #[doc = "This interface allows to coordinate whether the server should create"]
    #[doc = "a server-side window decoration around a wl_surface representing a"]
    #[doc = "shell surface (wl_shell_surface or similar). By announcing support"]
    #[doc = "for this interface the server indicates that it supports server"]
    #[doc = "side decorations."]
    #[doc = ""]
    #[doc = "Use in conjunction with zxdg_decoration_manager_v1 is undefined."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_server_decoration_manager {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "Undecorated: The surface is not decorated at all, neither server nor client-side. An example is a popup surface which should not be decorated."]
            None = 0u32,
            #[doc = "Client-side decoration: The decoration is part of the surface and the client."]
            Client = 1u32,
            #[doc = "Server-side decoration: The server embeds the surface into a decoration frame."]
            Server = 2u32,
        }
        impl From<Mode> for u32 {
            fn from(value: Mode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Mode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Client),
                    2u32 => Ok(Self::Server),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_server_decoration_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecorationManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_manager";
            const VERSION: u32 = 1u32;
            #[doc = "When a client creates a server-side decoration object it indicates"]
            #[doc = "that it supports the protocol. The client is supposed to tell the"]
            #[doc = "server whether it wants server-side decorations or will provide"]
            #[doc = "client-side decorations."]
            #[doc = ""]
            #[doc = "If the client does not create a server-side decoration object for"]
            #[doc = "a surface the server interprets this as lack of support for this"]
            #[doc = "protocol and considers it as client-side decorated. Nevertheless a"]
            #[doc = "client-side decorated surface should use this protocol to indicate"]
            #[doc = "to the server that it does not want a server-side deco."]
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted directly after binding the interface. It contains"]
            #[doc = "the default mode for the decoration. When a new server decoration object"]
            #[doc = "is created this new object will be in the default mode until the first"]
            #[doc = "request_mode is requested."]
            #[doc = ""]
            #[doc = "The server may change the default mode at any time."]
            fn default_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(mode).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_server_decoration_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_server_decoration {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Mode {
            #[doc = "Undecorated: The surface is not decorated at all, neither server nor client-side. An example is a popup surface which should not be decorated."]
            None = 0u32,
            #[doc = "Client-side decoration: The decoration is part of the surface and the client."]
            Client = 1u32,
            #[doc = "Server-side decoration: The server embeds the surface into a decoration frame."]
            Server = 2u32,
        }
        impl From<Mode> for u32 {
            fn from(value: Mode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Mode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Client),
                    2u32 => Ok(Self::Server),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Mode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_server_decoration interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinServerDecoration
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration";
            const VERSION: u32 = 1u32;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn request_mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event is emitted directly after the decoration is created and"]
            #[doc = "represents the base decoration policy by the server. E.g. a server"]
            #[doc = "which wants all surfaces to be client-side decorated will send Client,"]
            #[doc = "a server which wants server-side decoration will send Server."]
            #[doc = ""]
            #[doc = "The client can request a different mode through the decoration request."]
            #[doc = "The server will acknowledge this by another event with the same mode. So"]
            #[doc = "even if a server prefers server-side decoration it's possible to force a"]
            #[doc = "client-side decoration."]
            #[doc = ""]
            #[doc = "The server may emit this event at any time. In this case the client can"]
            #[doc = "again request a different mode. It's the responsibility of the server to"]
            #[doc = "prevent a feedback loop."]
            fn mode(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(mode).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_server_decoration#{}.release()",
                                sender_id,
                            );
                            self.release(connection, sender_id).await
                        }
                        1u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_server_decoration#{}.request_mode({})",
                                sender_id,
                                mode
                            );
                            self.request_mode(connection, sender_id, mode).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod shadow {
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_shadow_manager {
        #[doc = "Trait to implement the org_kde_kwin_shadow_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinShadowManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_shadow_manager";
            const VERSION: u32 = 2u32;
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn unset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the org_kde_kwin_shadow_manager object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow_manager#{}.unset({})",
                                sender_id,
                                surface
                            );
                            self.unset(connection, sender_id, surface).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_shadow_manager#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_shadow {
        #[doc = "Trait to implement the org_kde_kwin_shadow interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinShadow
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_shadow";
            const VERSION: u32 = 2u32;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_left(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_top_left(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_top(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_top_right(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_right(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_bottom_right(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_bottom(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn attach_bottom_left(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                buffer: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_left_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                offset: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_top_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                offset: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_right_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                offset: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_bottom_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                offset: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the org_kde_kwin_shadow object. If the org_kde_kwin_shadow is"]
            #[doc = "still set on a wl_surface the shadow will be immediately removed."]
            #[doc = "Prefer to first call the request unset on the org_kde_kwin_shadow_manager and"]
            #[doc = "commit the wl_surface to apply the change."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_shadow#{}.commit()", sender_id,);
                            self.commit(connection, sender_id).await
                        }
                        1u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_left({})",
                                sender_id,
                                buffer
                            );
                            self.attach_left(connection, sender_id, buffer).await
                        }
                        2u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_top_left({})",
                                sender_id,
                                buffer
                            );
                            self.attach_top_left(connection, sender_id, buffer).await
                        }
                        3u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_top({})",
                                sender_id,
                                buffer
                            );
                            self.attach_top(connection, sender_id, buffer).await
                        }
                        4u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_top_right({})",
                                sender_id,
                                buffer
                            );
                            self.attach_top_right(connection, sender_id, buffer).await
                        }
                        5u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_right({})",
                                sender_id,
                                buffer
                            );
                            self.attach_right(connection, sender_id, buffer).await
                        }
                        6u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_bottom_right({})",
                                sender_id,
                                buffer
                            );
                            self.attach_bottom_right(connection, sender_id, buffer)
                                .await
                        }
                        7u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_bottom({})",
                                sender_id,
                                buffer
                            );
                            self.attach_bottom(connection, sender_id, buffer).await
                        }
                        8u16 => {
                            let buffer = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.attach_bottom_left({})",
                                sender_id,
                                buffer
                            );
                            self.attach_bottom_left(connection, sender_id, buffer).await
                        }
                        9u16 => {
                            let offset = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.set_left_offset({})",
                                sender_id,
                                offset
                            );
                            self.set_left_offset(connection, sender_id, offset).await
                        }
                        10u16 => {
                            let offset = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.set_top_offset({})",
                                sender_id,
                                offset
                            );
                            self.set_top_offset(connection, sender_id, offset).await
                        }
                        11u16 => {
                            let offset = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.set_right_offset({})",
                                sender_id,
                                offset
                            );
                            self.set_right_offset(connection, sender_id, offset).await
                        }
                        12u16 => {
                            let offset = message.fixed()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_shadow#{}.set_bottom_offset({})",
                                sender_id,
                                offset
                            );
                            self.set_bottom_offset(connection, sender_id, offset).await
                        }
                        13u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_shadow#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod slide {
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_slide_manager {
        #[doc = "Trait to implement the org_kde_kwin_slide_manager interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinSlideManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_slide_manager";
            const VERSION: u32 = 1u32;
            fn create(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn unset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_slide_manager#{}.create({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.create(connection, sender_id, id, surface).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_slide_manager#{}.unset({})",
                                sender_id,
                                surface
                            );
                            self.unset(connection, sender_id, surface).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "Ask the compositor to move the surface from a location to another"]
    #[doc = "with a slide animation."]
    #[doc = ""]
    #[doc = "The from argument provides a clue about where the slide animation"]
    #[doc = "begins, offset is the distance from screen edge to begin the animation."]
    #[allow(clippy::too_many_arguments)]
    pub mod org_kde_kwin_slide {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Location {
            Left = 0u32,
            Top = 1u32,
            Right = 2u32,
            Bottom = 3u32,
        }
        impl From<Location> for u32 {
            fn from(value: Location) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Location {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Left),
                    1u32 => Ok(Self::Top),
                    2u32 => Ok(Self::Right),
                    3u32 => Ok(Self::Bottom),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Location {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the org_kde_kwin_slide interface. See the module level documentation for more info"]
        pub trait OrgKdeKwinSlide
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "org_kde_kwin_slide";
            const VERSION: u32 = 1u32;
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_location(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                location: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                offset: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_slide#{}.commit()", sender_id,);
                            self.commit(connection, sender_id).await
                        }
                        1u16 => {
                            let location = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_slide#{}.set_location({})",
                                sender_id,
                                location
                            );
                            self.set_location(connection, sender_id, location).await
                        }
                        2u16 => {
                            let offset = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "org_kde_kwin_slide#{}.set_offset({})",
                                sender_id,
                                offset
                            );
                            self.set_offset(connection, sender_id, offset).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("org_kde_kwin_slide#{}.release()", sender_id,);
                            self.release(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod surface_extension {
    #[allow(clippy::too_many_arguments)]
    pub mod qt_surface_extension {
        #[doc = "Trait to implement the qt_surface_extension interface. See the module level documentation for more info"]
        pub trait QtSurfaceExtension
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "qt_surface_extension";
            const VERSION: u32 = 1u32;
            fn get_extended_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "qt_surface_extension#{}.get_extended_surface({}, {})",
                                sender_id,
                                id,
                                surface
                            );
                            self.get_extended_surface(connection, sender_id, id, surface)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod qt_extended_surface {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Orientation {
            PrimaryOrientation = 0u32,
            PortraitOrientation = 1u32,
            LandscapeOrientation = 2u32,
            InvertedPortraitOrientation = 4u32,
            InvertedLandscapeOrientation = 8u32,
        }
        impl From<Orientation> for u32 {
            fn from(value: Orientation) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Orientation {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PrimaryOrientation),
                    1u32 => Ok(Self::PortraitOrientation),
                    2u32 => Ok(Self::LandscapeOrientation),
                    4u32 => Ok(Self::InvertedPortraitOrientation),
                    8u32 => Ok(Self::InvertedLandscapeOrientation),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Orientation {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Windowflag {
            OverridesSystemGestures = 1u32,
            StaysOnTop = 2u32,
            BypassWindowManager = 4u32,
        }
        impl From<Windowflag> for u32 {
            fn from(value: Windowflag) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Windowflag {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::OverridesSystemGestures),
                    2u32 => Ok(Self::StaysOnTop),
                    4u32 => Ok(Self::BypassWindowManager),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Windowflag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the qt_extended_surface interface. See the module level documentation for more info"]
        pub trait QtExtendedSurface
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "qt_extended_surface";
            const VERSION: u32 = 1u32;
            fn update_generic_property(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                value: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_content_orientation_mask(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                orientation: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_window_flags(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                flags: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn raise(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn lower(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn onscreen_visibility(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                visible: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(visible).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn set_generic_property(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                value: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_array(value)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let value = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "qt_extended_surface#{}.update_generic_property(\"{}\", array[{}])",
                                sender_id,
                                name,
                                value.len()
                            );
                            self.update_generic_property(connection, sender_id, name, value)
                                .await
                        }
                        1u16 => {
                            let orientation = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "qt_extended_surface#{}.set_content_orientation_mask({})",
                                sender_id,
                                orientation
                            );
                            self.set_content_orientation_mask(connection, sender_id, orientation)
                                .await
                        }
                        2u16 => {
                            let flags = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "qt_extended_surface#{}.set_window_flags({})",
                                sender_id,
                                flags
                            );
                            self.set_window_flags(connection, sender_id, flags).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("qt_extended_surface#{}.raise()", sender_id,);
                            self.raise(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("qt_extended_surface#{}.lower()", sender_id,);
                            self.lower(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod text_input_unstable_v2 {
    #[doc = "The zwp_text_input_v2 interface represents text input and input methods"]
    #[doc = "associated with a seat. It provides enter/leave events to follow the"]
    #[doc = "text input focus for a seat."]
    #[doc = ""]
    #[doc = "Requests are used to enable/disable the text-input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about the entered text is sent to the text-input object"]
    #[doc = "via the pre-edit and commit events. Using this interface removes the need"]
    #[doc = "for applications to directly process hardware key events and compose text"]
    #[doc = "out of them."]
    #[doc = ""]
    #[doc = "Text is valid UTF-8 encoded, indices and lengths are in bytes. Indices"]
    #[doc = "have to always point to the first byte of an UTF-8 encoded code point."]
    #[doc = "Lengths are not allowed to contain just a part of an UTF-8 encoded code"]
    #[doc = "point."]
    #[doc = ""]
    #[doc = "State is sent by the state requests (set_surrounding_text,"]
    #[doc = "set_content_type, set_cursor_rectangle and set_preferred_language) and"]
    #[doc = "an update_state request. After an enter or an input_method_change event"]
    #[doc = "all state information is invalidated and needs to be resent from the"]
    #[doc = "client. A reset or entering a new widget on client side also"]
    #[doc = "invalidates all current state information."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_v2 {
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behaviour"] const None = 0u32 ; # [doc = "suggest word completions"] const AutoCompletion = 1u32 ; # [doc = "suggest word corrections"] const AutoCorrection = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
        impl From<ContentHint> for u32 {
            fn from(value: ContentHint) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for ContentHint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ContentHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
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
        impl From<ContentPurpose> for u32 {
            fn from(value: ContentPurpose) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ContentPurpose {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ContentPurpose {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Defines the reason for sending an updated state."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum UpdateState {
            #[doc = "updated state because it changed"]
            Change = 0u32,
            #[doc = "full state after enter or input_method_changed event"]
            Full = 1u32,
            #[doc = "full state after reset"]
            Reset = 2u32,
            #[doc = "full state after switching focus to a different widget on client side"]
            Enter = 3u32,
        }
        impl From<UpdateState> for u32 {
            fn from(value: UpdateState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for UpdateState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Change),
                    1u32 => Ok(Self::Full),
                    2u32 => Ok(Self::Reset),
                    3u32 => Ok(Self::Enter),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for UpdateState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum InputPanelVisibility {
            #[doc = "the input panel (virtual keyboard) is hidden"]
            Hidden = 0u32,
            #[doc = "the input panel (virtual keyboard) is visible"]
            Visible = 1u32,
        }
        impl From<InputPanelVisibility> for u32 {
            fn from(value: InputPanelVisibility) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for InputPanelVisibility {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Hidden),
                    1u32 => Ok(Self::Visible),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for InputPanelVisibility {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PreeditStyle {
            #[doc = "default style for composing text"]
            Default = 0u32,
            #[doc = "composing text should be shown the same as non-composing text"]
            None = 1u32,
            #[doc = "composing text might be bold"]
            Active = 2u32,
            #[doc = "composing text might be cursive"]
            Inactive = 3u32,
            #[doc = "composing text might have a different background color"]
            Highlight = 4u32,
            #[doc = "composing text might be underlined"]
            Underline = 5u32,
            #[doc = "composing text should be shown the same as selected text"]
            Selection = 6u32,
            #[doc = "composing text might be underlined with a red wavy line"]
            Incorrect = 7u32,
        }
        impl From<PreeditStyle> for u32 {
            fn from(value: PreeditStyle) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PreeditStyle {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PreeditStyle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl From<TextDirection> for u32 {
            fn from(value: TextDirection) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for TextDirection {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Auto),
                    1u32 => Ok(Self::Ltr),
                    2u32 => Ok(Self::Rtl),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TextDirection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zwp_text_input_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "zwp_text_input_v2";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the wp_text_input object. Also disables all surfaces enabled"]
            #[doc = "through this wp_text_input object"]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Enable text input in a surface (usually when a text entry inside of it"]
            #[doc = "has focus)."]
            #[doc = ""]
            #[doc = "This can be called before or after a surface gets text (or keyboard)"]
            #[doc = "focus via the enter event. Text input to a surface is only active"]
            #[doc = "when it has the current text (or keyboard) focus and is enabled."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Disable text input in a surface (typically when there is no focus on any"]
            #[doc = "text entry inside the surface)."]
            fn disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests input panels (virtual keyboard) to show."]
            #[doc = ""]
            #[doc = "This should be used for example to show a virtual keyboard again"]
            #[doc = "(with a tap) after it was closed by pressing on a close button on the"]
            #[doc = "keyboard."]
            fn show_input_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests input panels (virtual keyboard) to hide."]
            fn hide_input_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the plain surrounding text around the input position. Text is"]
            #[doc = "UTF-8 encoded. Cursor is the byte offset within the surrounding text."]
            #[doc = "Anchor is the byte offset of the selection anchor within the"]
            #[doc = "surrounding text. If there is no selected text, anchor is the same as"]
            #[doc = "cursor."]
            #[doc = ""]
            #[doc = "Make sure to always send some text before and after the cursor"]
            #[doc = "except when the cursor is at the beginning or end of text."]
            #[doc = ""]
            #[doc = "When there was a configure_surrounding_text event take the"]
            #[doc = "before_cursor and after_cursor arguments into account for picking how"]
            #[doc = "much surrounding text to send."]
            #[doc = ""]
            #[doc = "There is a maximum length of wayland messages so text can not be"]
            #[doc = "longer than 4000 bytes."]
            fn set_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                cursor: i32,
                anchor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some"]
            #[doc = "of the behavior."]
            #[doc = ""]
            #[doc = "When no content type is explicitly set, a normal content purpose with"]
            #[doc = "none hint should be assumed."]
            fn set_content_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: ContentHint,
                purpose: ContentPurpose,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the cursor outline as a x, y, width, height rectangle in surface"]
            #[doc = "local coordinates."]
            #[doc = ""]
            #[doc = "Allows the compositor to put a window with word suggestions near the"]
            #[doc = "cursor."]
            fn set_cursor_rectangle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets a specific language. This allows for example a virtual keyboard to"]
            #[doc = "show a language specific layout. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            #[doc = ""]
            #[doc = "It could be used for example in a word processor to indicate language of"]
            #[doc = "currently edited document or in an instant message application which"]
            #[doc = "tracks languages of contacts."]
            fn set_preferred_language(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                language: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Allows to atomically send state updates from client."]
            #[doc = ""]
            #[doc = "This request should follow after a batch of state updating requests"]
            #[doc = "like set_surrounding_text, set_content_type, set_cursor_rectangle and"]
            #[doc = "set_preferred_language."]
            #[doc = ""]
            #[doc = "The flags field indicates why an updated state is sent to the input"]
            #[doc = "method."]
            #[doc = ""]
            #[doc = "Reset should be used by an editor widget after the text was changed"]
            #[doc = "outside of the normal input method flow."]
            #[doc = ""]
            #[doc = "For \"change\" it is enough to send the changed state, else the full"]
            #[doc = "state should be send."]
            #[doc = ""]
            #[doc = "Serial should be set to the serial from the last enter or"]
            #[doc = "input_method_changed event."]
            #[doc = ""]
            #[doc = "To make sure to not receive outdated input method events after a"]
            #[doc = "reset or switching to a new widget wl_display_sync() should be used"]
            #[doc = "after update_state in these cases."]
            fn update_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                reason: UpdateState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notification that this seat's text-input focus is on a certain surface."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notification that this seat's text-input focus is no longer on"]
            #[doc = "a certain surface."]
            #[doc = ""]
            #[doc = "The leave notification is sent before the enter notification"]
            #[doc = "for the new focus."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notification that the visibility of the input panel (virtual keyboard)"]
            #[doc = "changed."]
            #[doc = ""]
            #[doc = "The rectangle x, y, width, height defines the area overlapped by the"]
            #[doc = "input panel (virtual keyboard) on the surface having the text"]
            #[doc = "focus in surface local coordinates."]
            #[doc = ""]
            #[doc = "That can be used to make sure widgets are visible and not covered by"]
            #[doc = "a virtual keyboard."]
            fn input_panel_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: InputPanelVisibility,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(state.into())
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when a new composing text (pre-edit) should be set around the"]
            #[doc = "current cursor position. Any previously set composing text should"]
            #[doc = "be removed."]
            #[doc = ""]
            #[doc = "The commit text can be used to replace the composing text in some cases"]
            #[doc = "(for example when losing focus)."]
            #[doc = ""]
            #[doc = "The text input should also handle all preedit_style and preedit_cursor"]
            #[doc = "events occurring directly before preedit_string."]
            fn preedit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                commit: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(text))
                        .put_string(Some(commit))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets styling information on composing text. The style is applied for"]
            #[doc = "length bytes from index relative to the beginning of the composing"]
            #[doc = "text (as byte offset). Multiple styles can be applied to a composing"]
            #[doc = "text by sending multiple preedit_styling events."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            fn preedit_styling(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: u32,
                length: u32,
                style: PreeditStyle,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(index)
                        .put_uint(length)
                        .put_uint(style.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the cursor position inside the composing text (as byte"]
            #[doc = "offset) relative to the start of the composing text. When index is a"]
            #[doc = "negative number no cursor is shown."]
            #[doc = ""]
            #[doc = "When no preedit_cursor event is sent the cursor will be at the end of"]
            #[doc = "the composing text by default."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            fn preedit_cursor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(index).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit). It could be also an empty text"]
            #[doc = "when some text should be removed (see delete_surrounding_text) or when"]
            #[doc = "the input cursor should be moved (see cursor_position)."]
            #[doc = ""]
            #[doc = "Any previously set composing text should be removed."]
            fn commit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(text))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when the cursor or anchor position should be modified."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The text between anchor and index should be selected."]
            fn cursor_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: i32,
                anchor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(index)
                        .put_int(anchor)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted. BeforeLength and afterLength is the length (in bytes) of text"]
            #[doc = "before and after the current cursor position (excluding the selection)"]
            #[doc = "to delete."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "or preedit_string event."]
            fn delete_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                before_length: u32,
                after_length: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(before_length)
                        .put_uint(after_length)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Transfer an array of 0-terminated modifiers names. The position in"]
            #[doc = "the array is the index of the modifier as used in the modifiers"]
            #[doc = "bitmask in the keysym event."]
            fn modifiers_map(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                map: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_array(map).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when a key event was sent. Key events should not be used"]
            #[doc = "for normal text input operations, which should be done with"]
            #[doc = "commit_string, delete_surrounding_text, etc. The key event follows"]
            #[doc = "the wl_keyboard key event convention. Sym is a XKB keysym, state a"]
            #[doc = "wl_keyboard key_state. Modifiers are a mask for effective modifiers"]
            #[doc = "(where the modifier indices are set by the modifiers_map event)"]
            fn keysym(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                time: u32,
                sym: u32,
                state: u32,
                modifiers: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(time)
                        .put_uint(sym)
                        .put_uint(state)
                        .put_uint(modifiers)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the language of the input text. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            fn language(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                language: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(language))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the text direction of input text."]
            #[doc = ""]
            #[doc = "It is mainly needed for showing input cursor on correct side of the"]
            #[doc = "editor when there is no input yet done and making sure neutral"]
            #[doc = "direction text is laid out properly."]
            fn text_direction(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                direction: TextDirection,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(direction.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Configure what amount of surrounding text is expected by the"]
            #[doc = "input method. The surrounding text will be sent in the"]
            #[doc = "set_surrounding_text request on the following state information updates."]
            fn configure_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                before_cursor: i32,
                after_cursor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(before_cursor)
                        .put_int(after_cursor)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 13u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The input method changed on compositor side, which invalidates all"]
            #[doc = "current state information. New state information should be sent from"]
            #[doc = "the client via state requests (set_surrounding_text,"]
            #[doc = "set_content_hint, ...) and update_state."]
            fn input_method_changed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                flags: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_uint(flags)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 14u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v2#{}.enable({})", sender_id, surface);
                            self.enable(connection, sender_id, surface).await
                        }
                        2u16 => {
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v2#{}.disable({})", sender_id, surface);
                            self.disable(connection, sender_id, surface).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v2#{}.show_input_panel()", sender_id,);
                            self.show_input_panel(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_v2#{}.hide_input_panel()", sender_id,);
                            self.hide_input_panel(connection, sender_id).await
                        }
                        5u16 => {
                            let text = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let cursor = message.int()?;
                            let anchor = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v2#{}.set_surrounding_text(\"{}\", {}, {})",
                                sender_id,
                                text,
                                cursor,
                                anchor
                            );
                            self.set_surrounding_text(connection, sender_id, text, cursor, anchor)
                                .await
                        }
                        6u16 => {
                            let hint = message.uint()?;
                            let purpose = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v2#{}.set_content_type({}, {})",
                                sender_id,
                                hint,
                                purpose
                            );
                            self.set_content_type(
                                connection,
                                sender_id,
                                hint.try_into()?,
                                purpose.try_into()?,
                            )
                            .await
                        }
                        7u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v2#{}.set_cursor_rectangle({}, {}, {}, {})",
                                sender_id,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_cursor_rectangle(connection, sender_id, x, y, width, height)
                                .await
                        }
                        8u16 => {
                            let language = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v2#{}.set_preferred_language(\"{}\")",
                                sender_id,
                                language
                            );
                            self.set_preferred_language(connection, sender_id, language)
                                .await
                        }
                        9u16 => {
                            let serial = message.uint()?;
                            let reason = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_v2#{}.update_state({}, {})",
                                sender_id,
                                serial,
                                reason
                            );
                            self.update_state(connection, sender_id, serial, reason.try_into()?)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    #[allow(clippy::too_many_arguments)]
    pub mod zwp_text_input_manager_v2 {
        #[doc = "Trait to implement the zwp_text_input_manager_v2 interface. See the module level documentation for more info"]
        pub trait ZwpTextInputManagerV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "zwp_text_input_manager_v2";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the wp_text_input_manager object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Creates a new text-input object for a given seat."]
            fn get_text_input(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zwp_text_input_manager_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zwp_text_input_manager_v2#{}.get_text_input({}, {})",
                                sender_id,
                                id,
                                seat
                            );
                            self.get_text_input(connection, sender_id, id, seat).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod text {
    #[doc = "An object used for text input. Adds support for text input and input"]
    #[doc = "methods to applications. A text-input object is created from a"]
    #[doc = "wl_text_input_manager and corresponds typically to a text entry in an"]
    #[doc = "application."]
    #[doc = "Requests are used to activate/deactivate the text-input object and set"]
    #[doc = "state information like surrounding and selected text or the content type."]
    #[doc = "The information about entered text is sent to the text-input object via"]
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
    #[allow(clippy::too_many_arguments)]
    pub mod wl_text_input {
        #[doc = "Content hint is a bitmask to allow to modify the behavior of the text"]
        #[doc = "input."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ContentHint {
            #[doc = "no special behaviour"]
            None = 0u32,
            #[doc = "auto completion, correction and capitalization"]
            Default = 7u32,
            #[doc = "hidden and sensitive text"]
            Password = 192u32,
            #[doc = "suggest word completions"]
            AutoCompletion = 1u32,
            #[doc = "suggest word corrections"]
            AutoCorrection = 2u32,
            #[doc = "switch to uppercase letters at the start of a sentence"]
            AutoCapitalization = 4u32,
            #[doc = "prefer lowercase letters"]
            Lowercase = 8u32,
            #[doc = "prefer uppercase letters"]
            Uppercase = 16u32,
            #[doc = "prefer casing for titles and headings (can be language dependent)"]
            Titlecase = 32u32,
            #[doc = "characters should be hidden"]
            HiddenText = 64u32,
            #[doc = "typed text should not be stored"]
            SensitiveData = 128u32,
            #[doc = "just latin characters should be entered"]
            Latin = 256u32,
            #[doc = "the text input is multiline"]
            Multiline = 512u32,
        }
        impl From<ContentHint> for u32 {
            fn from(value: ContentHint) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ContentHint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    7u32 => Ok(Self::Default),
                    192u32 => Ok(Self::Password),
                    1u32 => Ok(Self::AutoCompletion),
                    2u32 => Ok(Self::AutoCorrection),
                    4u32 => Ok(Self::AutoCapitalization),
                    8u32 => Ok(Self::Lowercase),
                    16u32 => Ok(Self::Uppercase),
                    32u32 => Ok(Self::Titlecase),
                    64u32 => Ok(Self::HiddenText),
                    128u32 => Ok(Self::SensitiveData),
                    256u32 => Ok(Self::Latin),
                    512u32 => Ok(Self::Multiline),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ContentHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl From<ContentPurpose> for u32 {
            fn from(value: ContentPurpose) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ContentPurpose {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ContentPurpose {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl From<PreeditStyle> for u32 {
            fn from(value: PreeditStyle) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PreeditStyle {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PreeditStyle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
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
        impl From<TextDirection> for u32 {
            fn from(value: TextDirection) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for TextDirection {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Auto),
                    1u32 => Ok(Self::Ltr),
                    2u32 => Ok(Self::Rtl),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TextDirection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_text_input interface. See the module level documentation for more info"]
        pub trait WlTextInput
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "wl_text_input";
            const VERSION: u32 = 1u32;
            #[doc = "Requests the text-input object to be activated (typically when the"]
            #[doc = "text entry gets focus)."]
            #[doc = "The seat argument is a wl_seat which maintains the focus for this"]
            #[doc = "activation. The surface argument is a wl_surface assigned to the"]
            #[doc = "text-input object and tracked for focus lost. The enter event"]
            #[doc = "is emitted on successful activation."]
            fn activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests the text-input object to be deactivated (typically when the"]
            #[doc = "text entry lost focus). The seat argument is a wl_seat which was used"]
            #[doc = "for activation."]
            fn deactivate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests input panels (virtual keyboard) to show."]
            fn show_input_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests input panels (virtual keyboard) to hide."]
            fn hide_input_panel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Should be called by an editor widget when the input state should be"]
            #[doc = "reset, for example after the text was changed outside of the normal"]
            #[doc = "input method flow."]
            fn reset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the plain surrounding text around the input position. Text is"]
            #[doc = "UTF-8 encoded. Cursor is the byte offset within the"]
            #[doc = "surrounding text. Anchor is the byte offset of the"]
            #[doc = "selection anchor within the surrounding text. If there is no selected"]
            #[doc = "text anchor is the same as cursor."]
            fn set_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                cursor: u32,
                anchor: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some"]
            #[doc = "of the behavior."]
            #[doc = ""]
            #[doc = "When no content type is explicitly set, a normal content purpose with"]
            #[doc = "default hints (auto completion, auto correction, auto capitalization)"]
            #[doc = "should be assumed."]
            fn set_content_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: u32,
                purpose: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn set_cursor_rectangle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets a specific language. This allows for example a virtual keyboard to"]
            #[doc = "show a language specific layout. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            #[doc = ""]
            #[doc = "It could be used for example in a word processor to indicate language of"]
            #[doc = "currently edited document or in an instant message application which tracks"]
            #[doc = "languages of contacts."]
            fn set_preferred_language(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                language: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn commit_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn invoke_action(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button: u32,
                index: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notify the text-input object when it received focus. Typically in"]
            #[doc = "response to an activate request."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify the text-input object when it lost focus. Either in response"]
            #[doc = "to a deactivate request or when the assigned surface lost focus or was"]
            #[doc = "destroyed."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Transfer an array of 0-terminated modifiers names. The position in"]
            #[doc = "the array is the index of the modifier as used in the modifiers"]
            #[doc = "bitmask in the keysym event."]
            fn modifiers_map(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                map: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_array(map).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when the visibility state of the input panel changed."]
            fn input_panel_state(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(state).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when a new composing text (pre-edit) should be set around the"]
            #[doc = "current cursor position. Any previously set composing text should"]
            #[doc = "be removed."]
            #[doc = ""]
            #[doc = "The commit text can be used to replace the preedit text on reset"]
            #[doc = "(for example on unfocus)."]
            #[doc = ""]
            #[doc = "The text input should also handle all preedit_style and preedit_cursor"]
            #[doc = "events occurring directly before preedit_string."]
            fn preedit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                text: String,
                commit: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_string(Some(text))
                        .put_string(Some(commit))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets styling information on composing text. The style is applied for"]
            #[doc = "length bytes from index relative to the beginning of the composing"]
            #[doc = "text (as byte offset). Multiple styles can"]
            #[doc = "be applied to a composing text by sending multiple preedit_styling"]
            #[doc = "events."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            fn preedit_styling(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: u32,
                length: u32,
                style: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(index)
                        .put_uint(length)
                        .put_uint(style)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the cursor position inside the composing text (as byte"]
            #[doc = "offset) relative to the start of the composing text. When index is a"]
            #[doc = "negative number no cursor is shown."]
            #[doc = ""]
            #[doc = "This event is handled as part of a following preedit_string event."]
            fn preedit_cursor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(index).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit). It could be also an empty text"]
            #[doc = "when some text should be removed (see delete_surrounding_text) or when"]
            #[doc = "the input cursor should be moved (see cursor_position)."]
            #[doc = ""]
            #[doc = "Any previously set composing text should be removed."]
            fn commit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                text: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_string(Some(text))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when the cursor or anchor position should be modified."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            fn cursor_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: i32,
                anchor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(index)
                        .put_int(anchor)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted."]
            #[doc = ""]
            #[doc = "Index is relative to the current cursor (in bytes)."]
            #[doc = "Length is the length of deleted text (in bytes)."]
            #[doc = ""]
            #[doc = "This event should be handled as part of a following commit_string"]
            #[doc = "event."]
            fn delete_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: i32,
                length: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(index)
                        .put_uint(length)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when a key event was sent. Key events should not be used"]
            #[doc = "for normal text input operations, which should be done with"]
            #[doc = "commit_string, delete_surrounding_text, etc. The key event follows"]
            #[doc = "the wl_keyboard key event convention. Sym is a XKB keysym, state a"]
            #[doc = "wl_keyboard key_state. Modifiers are a mask for effective modifiers"]
            #[doc = "(where the modifier indices are set by the modifiers_map event)"]
            fn keysym(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                time: u32,
                sym: u32,
                state: u32,
                modifiers: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_uint(time)
                        .put_uint(sym)
                        .put_uint(state)
                        .put_uint(modifiers)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the language of the input text. The \"language\" argument is a RFC-3066"]
            #[doc = "format language tag."]
            fn language(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                language: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_string(Some(language))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Sets the text direction of input text."]
            #[doc = ""]
            #[doc = "It is mainly needed for showing input cursor on correct side of the"]
            #[doc = "editor when there is no input yet done and making sure neutral"]
            #[doc = "direction text is laid out properly."]
            fn text_direction(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
                direction: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(serial)
                        .put_uint(direction)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input#{}.activate({}, {})",
                                sender_id,
                                seat,
                                surface
                            );
                            self.activate(connection, sender_id, seat, surface).await
                        }
                        1u16 => {
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("wl_text_input#{}.deactivate({})", sender_id, seat);
                            self.deactivate(connection, sender_id, seat).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("wl_text_input#{}.show_input_panel()", sender_id,);
                            self.show_input_panel(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("wl_text_input#{}.hide_input_panel()", sender_id,);
                            self.hide_input_panel(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("wl_text_input#{}.reset()", sender_id,);
                            self.reset(connection, sender_id).await
                        }
                        5u16 => {
                            let text = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let cursor = message.uint()?;
                            let anchor = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input#{}.set_surrounding_text(\"{}\", {}, {})",
                                sender_id,
                                text,
                                cursor,
                                anchor
                            );
                            self.set_surrounding_text(connection, sender_id, text, cursor, anchor)
                                .await
                        }
                        6u16 => {
                            let hint = message.uint()?;
                            let purpose = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input#{}.set_content_type({}, {})",
                                sender_id,
                                hint,
                                purpose
                            );
                            self.set_content_type(connection, sender_id, hint, purpose)
                                .await
                        }
                        7u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input#{}.set_cursor_rectangle({}, {}, {}, {})",
                                sender_id,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_cursor_rectangle(connection, sender_id, x, y, width, height)
                                .await
                        }
                        8u16 => {
                            let language = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input#{}.set_preferred_language(\"{}\")",
                                sender_id,
                                language
                            );
                            self.set_preferred_language(connection, sender_id, language)
                                .await
                        }
                        9u16 => {
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("wl_text_input#{}.commit_state({})", sender_id, serial);
                            self.commit_state(connection, sender_id, serial).await
                        }
                        10u16 => {
                            let button = message.uint()?;
                            let index = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input#{}.invoke_action({}, {})",
                                sender_id,
                                button,
                                index
                            );
                            self.invoke_action(connection, sender_id, button, index)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    #[allow(clippy::too_many_arguments)]
    pub mod wl_text_input_manager {
        #[doc = "Trait to implement the wl_text_input_manager interface. See the module level documentation for more info"]
        pub trait WlTextInputManager
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "wl_text_input_manager";
            const VERSION: u32 = 1u32;
            #[doc = "Creates a new text-input object."]
            fn create_text_input(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_text_input_manager#{}.create_text_input({})",
                                sender_id,
                                id
                            );
                            self.create_text_input(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[allow(clippy::module_inception)]
pub mod wl_eglstream_controller {
    #[allow(clippy::too_many_arguments)]
    pub mod wl_eglstream_controller {
        #[doc = "- dont_care: Using this enum will tell the server to make its own"]
        #[doc = "decisions regarding present mode."]
        #[doc = ""]
        #[doc = "- fifo:      Tells the server to use a fifo present mode. The decision to"]
        #[doc = "use fifo synchronous is left up to the server."]
        #[doc = ""]
        #[doc = "- mailbox:   Tells the server to use a mailbox present mode."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum PresentMode {
            #[doc = "Let the Server decide present mode"]
            DontCare = 0u32,
            #[doc = "Use a fifo present mode"]
            Fifo = 1u32,
            #[doc = "Use a mailbox mode"]
            Mailbox = 2u32,
        }
        impl From<PresentMode> for u32 {
            fn from(value: PresentMode) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for PresentMode {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::DontCare),
                    1u32 => Ok(Self::Fifo),
                    2u32 => Ok(Self::Mailbox),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for PresentMode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "- present_mode: Must be one of wl_eglstream_controller_present_mode. Tells the"]
        #[doc = "server the desired present mode that should be used."]
        #[doc = ""]
        #[doc = "- fifo_length:  Only valid when the present_mode attrib is provided and its"]
        #[doc = "value is specified as fifo. Tells the server the desired fifo"]
        #[doc = "length to be used when the desired present_mode is fifo."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Attrib {
            #[doc = "Tells the server the desired present mode"]
            PresentMode = 0u32,
            #[doc = "Tells the server the desired fifo length when the desired presenation_mode is fifo."]
            FifoLength = 1u32,
        }
        impl From<Attrib> for u32 {
            fn from(value: Attrib) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Attrib {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::PresentMode),
                    1u32 => Ok(Self::FifoLength),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Attrib {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the wl_eglstream_controller interface. See the module level documentation for more info"]
        pub trait WlEglstreamController
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "wl_eglstream_controller";
            const VERSION: u32 = 2u32;
            #[doc = "Creates the corresponding server side EGLStream from the given wl_buffer"]
            #[doc = "and attaches a consumer to it."]
            fn attach_eglstream_consumer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                wl_surface: waynest::ObjectId,
                wl_resource: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Creates the corresponding server side EGLStream from the given wl_buffer"]
            #[doc = "and attaches a consumer to it using the given attributes."]
            fn attach_eglstream_consumer_attribs(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                wl_surface: waynest::ObjectId,
                wl_resource: waynest::ObjectId,
                attribs: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let wl_surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let wl_resource = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_eglstream_controller#{}.attach_eglstream_consumer({}, {})",
                                sender_id,
                                wl_surface,
                                wl_resource
                            );
                            self.attach_eglstream_consumer(
                                connection,
                                sender_id,
                                wl_surface,
                                wl_resource,
                            )
                            .await
                        }
                        1u16 => {
                            let wl_surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let wl_resource = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let attribs = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "wl_eglstream_controller#{}.attach_eglstream_consumer_attribs({}, {}, array[{}])",
                                sender_id,
                                wl_surface,
                                wl_resource,
                                attribs.len()
                            );
                            self.attach_eglstream_consumer_attribs(
                                connection,
                                sender_id,
                                wl_surface,
                                wl_resource,
                                attribs,
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
pub mod zkde_screencast_unstable_v1 {
    #[doc = "Warning! The protocol described in this file is a desktop environment"]
    #[doc = "implementation detail. Regular clients must not use this protocol."]
    #[doc = "Backward incompatible changes may be added without bumping the major"]
    #[doc = "version of the extension."]
    #[allow(clippy::too_many_arguments)]
    pub mod zkde_screencast_unstable_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Pointer {
            #[doc = "No cursor"]
            Hidden = 1u32,
            #[doc = "Render the cursor on the stream"]
            Embedded = 2u32,
            #[doc = "Send metadata about where the cursor is through PipeWire"]
            Metadata = 4u32,
        }
        impl From<Pointer> for u32 {
            fn from(value: Pointer) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Pointer {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Hidden),
                    2u32 => Ok(Self::Embedded),
                    4u32 => Ok(Self::Metadata),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Pointer {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the zkde_screencast_unstable_v1 interface. See the module level documentation for more info"]
        pub trait ZkdeScreencastUnstableV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "zkde_screencast_unstable_v1";
            const VERSION: u32 = 5u32;
            fn stream_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                stream: waynest::ObjectId,
                output: waynest::ObjectId,
                pointer: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn stream_window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                stream: waynest::ObjectId,
                window_uuid: String,
                pointer: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroy the zkde_screencast_unstable_v1 object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn stream_virtual_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                stream: waynest::ObjectId,
                name: String,
                width: i32,
                height: i32,
                scale: waynest::Fixed,
                pointer: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Since version 5, the compositor will choose the highest scale"]
            #[doc = "factor for the region if the given scale is 0.0."]
            fn stream_region(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                stream: waynest::ObjectId,
                x: i32,
                y: i32,
                width: u32,
                height: u32,
                scale: waynest::Fixed,
                pointer: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn stream_virtual_output_with_description(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                stream: waynest::ObjectId,
                name: String,
                description: String,
                width: i32,
                height: i32,
                scale: waynest::Fixed,
                pointer: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            let stream = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let output = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let pointer = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zkde_screencast_unstable_v1#{}.stream_output({}, {}, {})",
                                sender_id,
                                stream,
                                output,
                                pointer
                            );
                            self.stream_output(connection, sender_id, stream, output, pointer)
                                .await
                        }
                        1u16 => {
                            let stream = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let window_uuid = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let pointer = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zkde_screencast_unstable_v1#{}.stream_window({}, \"{}\", {})",
                                sender_id,
                                stream,
                                window_uuid,
                                pointer
                            );
                            self.stream_window(connection, sender_id, stream, window_uuid, pointer)
                                .await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("zkde_screencast_unstable_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        3u16 => {
                            let stream = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let width = message.int()?;
                            let height = message.int()?;
                            let scale = message.fixed()?;
                            let pointer = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zkde_screencast_unstable_v1#{}.stream_virtual_output({}, \"{}\", {}, {}, {}, {})",
                                sender_id,
                                stream,
                                name,
                                width,
                                height,
                                scale,
                                pointer
                            );
                            self.stream_virtual_output(
                                connection, sender_id, stream, name, width, height, scale, pointer,
                            )
                            .await
                        }
                        4u16 => {
                            let stream = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.uint()?;
                            let height = message.uint()?;
                            let scale = message.fixed()?;
                            let pointer = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zkde_screencast_unstable_v1#{}.stream_region({}, {}, {}, {}, {}, {}, {})",
                                sender_id,
                                stream,
                                x,
                                y,
                                width,
                                height,
                                scale,
                                pointer
                            );
                            self.stream_region(
                                connection, sender_id, stream, x, y, width, height, scale, pointer,
                            )
                            .await
                        }
                        5u16 => {
                            let stream = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let description = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let width = message.int()?;
                            let height = message.int()?;
                            let scale = message.fixed()?;
                            let pointer = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zkde_screencast_unstable_v1#{}.stream_virtual_output_with_description({}, \"{}\", \"{}\", {}, {}, {}, {})",
                                sender_id,
                                stream,
                                name,
                                description,
                                width,
                                height,
                                scale,
                                pointer
                            );
                            self.stream_virtual_output_with_description(
                                connection,
                                sender_id,
                                stream,
                                name,
                                description,
                                width,
                                height,
                                scale,
                                pointer,
                            )
                            .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod zkde_screencast_stream_unstable_v1 {
        #[doc = "Trait to implement the zkde_screencast_stream_unstable_v1 interface. See the module level documentation for more info"]
        pub trait ZkdeScreencastStreamUnstableV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "zkde_screencast_stream_unstable_v1";
            const VERSION: u32 = 5u32;
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn closed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn created(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                node: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(node).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn failed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                error: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(error))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_request(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[allow(clippy::match_single_binding)]
                    match message.opcode() {
                        0u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "zkde_screencast_stream_unstable_v1#{}.close()",
                                sender_id,
                            );
                            self.close(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
