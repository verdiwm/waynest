#[doc = "This protocol supports creating/destroying seats, assigning input devices to"]
#[doc = "seats, and configuring input devices (e.g. setting keyboard repeat rate)."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\", \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod river_input_management_v1 {
    #[doc = "Input manager global interface."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_input_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            InvalidDestroy = 0u32,
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
                    0u32 => Ok(Self::InvalidDestroy),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_input_manager_v1 interface. See the module level documentation for more info"]
        pub trait RiverInputManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_input_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events on this object."]
            #[doc = ""]
            #[doc = "The Wayland protocol is asynchronous, which means the server may send"]
            #[doc = "further events until the stop request is processed. The client must wait"]
            #[doc = "for a river_input_manager_v1.finished event before destroying this"]
            #[doc = "object."]
            fn stop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_input_manager_v1#{}.stop()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request should be called after the finished event has been received"]
            #[doc = "to complete destruction of the object."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request before the finished event"]
            #[doc = "has been received."]
            #[doc = ""]
            #[doc = "If a client wishes to destroy this object it should send a"]
            #[doc = "river_input_manager_v1.stop request and wait for a"]
            #[doc = "river_input_manager_v1.finished event. Once the finished event is"]
            #[doc = "received it is safe to destroy this object and any other objects created"]
            #[doc = "through this interface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_input_manager_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a new seat with the given name. Has no effect if a seat with the"]
            #[doc = "given name already exists."]
            #[doc = ""]
            #[doc = "The default seat with name \"default\" always exists and does not need to"]
            #[doc = "be explicitly created."]
            fn create_seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_manager_v1#{}.create_seat(\"{}\")",
                        sender_id,
                        name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Destroy the seat with the given name. Has no effect if a seat with the"]
            #[doc = "given name does not exist."]
            #[doc = ""]
            #[doc = "The default seat with name \"default\" cannot be destroyed and attempting"]
            #[doc = "to destroy it will have no effect."]
            #[doc = ""]
            #[doc = "Any input devices assigned to the destroyed seat at the time of"]
            #[doc = "destruction are assigned to the default seat."]
            fn destroy_seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_manager_v1#{}.destroy_seat(\"{}\")",
                        sender_id,
                        name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the server will send no further events on this"]
            #[doc = "object. The client should destroy the object. See"]
            #[doc = "river_input_manager_v1.destroy for more information."]
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new input device has been created."]
            fn input_device(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_input_manager_v1#{}.finished()", sender_id,);
                            self.finished(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_input_manager_v1#{}.input_device({})",
                                sender_id,
                                id
                            );
                            self.input_device(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An input device represents a physical keyboard, mouse, touchscreen, or"]
    #[doc = "drawing tablet tool. It is assigned to exactly one seat at a time."]
    #[doc = "By default, all input devices are assigned to the default seat."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_input_device_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            InvalidRepeatInfo = 0u32,
            InvalidScrollFactor = 1u32,
            InvalidMapToRectangle = 2u32,
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
                    0u32 => Ok(Self::InvalidRepeatInfo),
                    1u32 => Ok(Self::InvalidScrollFactor),
                    2u32 => Ok(Self::InvalidMapToRectangle),
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
        pub enum Type {
            Keyboard = 0u32,
            Pointer = 1u32,
            Touch = 2u32,
            Tablet = 3u32,
        }
        impl From<Type> for u32 {
            fn from(value: Type) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Type {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Keyboard),
                    1u32 => Ok(Self::Pointer),
                    2u32 => Ok(Self::Touch),
                    3u32 => Ok(Self::Tablet),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_input_device_v1 interface. See the module level documentation for more info"]
        pub trait RiverInputDeviceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_input_device_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the input"]
            #[doc = "device object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_input_device_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Assign the input device to a seat. All input devices not explicitly"]
            #[doc = "assigned to a seat are considered assigned to the default seat."]
            #[doc = ""]
            #[doc = "Has no effect if a seat with the given name does not exist."]
            fn assign_to_seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_device_v1#{}.assign_to_seat(\"{}\")",
                        sender_id,
                        name
                    );
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
            #[doc = "Set repeat rate and delay for a keyboard input device. Has no effect if"]
            #[doc = "the device is not a keyboard."]
            #[doc = ""]
            #[doc = "Negative values for either rate or delay are illegal. A rate of zero"]
            #[doc = "will disable any repeating (regardless of the value of delay)."]
            fn set_repeat_info(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                rate: i32,
                delay: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_device_v1#{}.set_repeat_info({}, {})",
                        sender_id,
                        rate,
                        delay
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(rate)
                        .put_int(delay)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the scroll factor for a pointer input device. Has no effect if the"]
            #[doc = "device is not a pointer."]
            #[doc = ""]
            #[doc = "For example, a factor of 0.5 will make scrolling twice as slow while a"]
            #[doc = "factor of 3.0 will make scrolling 3 times as fast."]
            #[doc = ""]
            #[doc = "Setting a scroll factor less than 0 is a protocol error."]
            fn set_scroll_factor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                factor: waynest::Fixed,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_device_v1#{}.set_scroll_factor({})",
                        sender_id,
                        factor
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_fixed(factor).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Map the input device to the given output. Has no effect if the device is"]
            #[doc = "not a pointer, touch, or tablet device."]
            #[doc = ""]
            #[doc = "If mapped to both an output and a rectangle, the rectangle has priority."]
            #[doc = ""]
            #[doc = "Passing null clears an existing mapping."]
            fn map_to_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_device_v1#{}.map_to_output({})",
                        sender_id,
                        output
                            .as_ref()
                            .map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_object(output).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Map the input device to the given rectangle in the global compositor"]
            #[doc = "coordinate space. Has no effect if the device is not a pointer, touch,"]
            #[doc = "or tablet device."]
            #[doc = ""]
            #[doc = "If mapped to both an output and a rectangle, the rectangle has priority."]
            #[doc = ""]
            #[doc = "Width and height must be greater than or equal to 0."]
            #[doc = ""]
            #[doc = "Passing 0 for width or height clears an existing mapping."]
            fn map_to_rectangle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_input_device_v1#{}.map_to_rectangle({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the input device has been removed."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object and ignore any"]
            #[doc = "request (other than river_input_device_v1.destroy) made after this event is"]
            #[doc = "sent. The client should destroy this object with the"]
            #[doc = "river_input_device_v1.destroy request to free up resources."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The type of the input device. This event is sent once when the"]
            #[doc = "river_input_device_v1 object is created. The device type cannot"]
            #[doc = "change during the lifetime of the object."]
            fn r#type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                r#type: Type,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The name of the input device. This event is sent once when the"]
            #[doc = "river_input_device_v1 object is created. The device name cannot"]
            #[doc = "change during the lifetime of the object."]
            fn name(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_input_device_v1#{}.removed()", sender_id,);
                            self.removed(connection, sender_id).await
                        }
                        1u16 => {
                            let r#type = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_input_device_v1#{}.type({})", sender_id, r#type);
                            self.r#type(connection, sender_id, r#type.try_into()?).await
                        }
                        2u16 => {
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_input_device_v1#{}.name(\"{}\")",
                                sender_id,
                                name
                            );
                            self.name(connection, sender_id, name).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows the river-window-management-v1 window manager to"]
#[doc = "support the wlr-layer-shell-v1 protocol."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\", \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod river_layer_shell_v1 {
    #[doc = "This global interface should only be advertised to the client if the"]
    #[doc = "river_window_manager_v1 global is also advertised. Binding this interface"]
    #[doc = "indicates that the window manager supports layer shell."]
    #[doc = ""]
    #[doc = "If the window manager does not bind this interface, the compositor should"]
    #[doc = "not allow clients to map layer surfaces. This can be achieved by"]
    #[doc = "closing layer surfaces immediately."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_layer_shell_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "the layer_shell_output/seat object was already created."]
            ObjectAlreadyCreated = 0u32,
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
                    0u32 => Ok(Self::ObjectAlreadyCreated),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_layer_shell_v1 interface. See the module level documentation for more info"]
        pub trait RiverLayerShellV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_layer_shell_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the"]
            #[doc = "river_layer_shell_v1 object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_layer_shell_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "It is a protocol error to make this request more than once for a given"]
            #[doc = "river_output_v1 object."]
            fn get_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_layer_shell_v1#{}.get_output({}, {})",
                        sender_id,
                        id,
                        output
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "It is a protocol error to make this request more than once for a given"]
            #[doc = "river_seat_v1 object."]
            fn get_seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_layer_shell_v1#{}.get_seat({}, {})",
                        sender_id,
                        id,
                        seat
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(seat))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
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
    #[doc = "The lifetime of this object is tied to the corresponding river_output_v1."]
    #[doc = "This object is made inert when the river_output_v1.removed event is sent"]
    #[doc = "and should be destroyed."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_layer_shell_output_v1 {
        #[doc = "Trait to implement the river_layer_shell_output_v1 interface. See the module level documentation for more info"]
        pub trait RiverLayerShellOutputV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_layer_shell_output_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the"]
            #[doc = "river_layer_shell_output_v1 object and that it may be safely destroyed."]
            #[doc = ""]
            #[doc = "This request should be made after the river_output_v1.removed event is"]
            #[doc = "received to complete destruction of the output."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_layer_shell_output_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Mark this output as the default for new layer surfaces which do not"]
            #[doc = "request a specific output themselves. This request overrides any"]
            #[doc = "previous set_default request on any river_layer_shell_output_v1 object."]
            #[doc = ""]
            #[doc = "If no set_default request is made or if the default output is destroyed,"]
            #[doc = "the default output is undefined until the next set_default request."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn set_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_layer_shell_output_v1#{}.set_default()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates the area of the output remaining after subtracting"]
            #[doc = "the exclusive zones of layer surfaces. Exclusive zones are a hint, the"]
            #[doc = "window manager is free to ignore this area hint if it wishes."]
            #[doc = ""]
            #[doc = "The x and y values are in the global coordinate space, not relative to"]
            #[doc = "the position of the output."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn non_exclusive_area(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_layer_shell_output_v1#{}.non_exclusive_area({}, {}, {}, {})",
                                sender_id,
                                x,
                                y,
                                width,
                                height
                            );
                            self.non_exclusive_area(connection, sender_id, x, y, width, height)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The lifetime of this object is tied to the corresponding river_seat_v1."]
    #[doc = "This object is made inert when the river_seat_v1.removed event is sent and"]
    #[doc = "should be destroyed."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_layer_shell_seat_v1 {
        #[doc = "Trait to implement the river_layer_shell_seat_v1 interface. See the module level documentation for more info"]
        pub trait RiverLayerShellSeatV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_layer_shell_seat_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the"]
            #[doc = "river_layer_shell_seat_v1 object and that it may be safely destroyed."]
            #[doc = ""]
            #[doc = "This request should be made after the river_seat_v1.removed event is"]
            #[doc = "received to complete destruction of the seat."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_layer_shell_seat_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "A layer shell surface will be given exclusive keyboard focus at the end"]
            #[doc = "of the manage sequence in which this event is sent. The window manager"]
            #[doc = "may want to update window decorations or similar to indicate that no"]
            #[doc = "window is focused."]
            #[doc = ""]
            #[doc = "Until the focus_non_exclusive or focus_none event is sent, all window"]
            #[doc = "manager requests to change focus are ignored."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn focus_exclusive(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A layer shell surface will be given non-exclusive keyboard focus at the"]
            #[doc = "end of the manage sequence in which this event is sent. The window"]
            #[doc = "manager may want to update window decorations or similar to indicate"]
            #[doc = "that no window is focused."]
            #[doc = ""]
            #[doc = "The window manager continues to control focus and may choose to focus a"]
            #[doc = "different window/shell surface at any time. If the window manager sets"]
            #[doc = "focus during the same manage sequence in which this event is sent, the"]
            #[doc = "layer surface will not be focused."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn focus_non_exclusive(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "No layer shell surface will have keyboard focus at the end of the manage"]
            #[doc = "sequence in which this event is sent. The window manager may want to"]
            #[doc = "return focus to whichever window last had focus, for example."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn focus_none(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                                "river_layer_shell_seat_v1#{}.focus_exclusive()",
                                sender_id,
                            );
                            self.focus_exclusive(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_layer_shell_seat_v1#{}.focus_non_exclusive()",
                                sender_id,
                            );
                            self.focus_non_exclusive(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_layer_shell_seat_v1#{}.focus_none()", sender_id,);
                            self.focus_none(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol exposes libinput device configuration APIs. The libinput"]
#[doc = "documentation should be referred to for detailed information on libinput's"]
#[doc = "behavior."]
#[doc = ""]
#[doc = "This protocol is designed so that (hopefully) any backwards compatible"]
#[doc = "change to libinput's API can be matched with a backwards compatible change"]
#[doc = "to this protocol."]
#[doc = ""]
#[doc = "Note: the libinput API uses floating point types (float and double in C)"]
#[doc = "which are not (yet?) natively supported by the Wayland protocol. However,"]
#[doc = "the Wayland protocol does support sending arbitrary bytes through the array"]
#[doc = "argument type. This protocol uses e.g. type=\"array\" summary=\"double\" to"]
#[doc = "indicate a native-endian IEEE-754 64-bit double value."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\", \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod river_libinput_config_v1 {
    #[doc = "Global interface for configuring libinput devices. This global should"]
    #[doc = "only be advertised if river_input_manager_v1 is advertised as well."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_libinput_config_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid enum value or similar"]
            InvalidArg = 0u32,
            InvalidDestroy = 1u32,
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
                    0u32 => Ok(Self::InvalidArg),
                    1u32 => Ok(Self::InvalidDestroy),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_libinput_config_v1 interface. See the module level documentation for more info"]
        pub trait RiverLibinputConfigV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_libinput_config_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events on this object."]
            #[doc = ""]
            #[doc = "The Wayland protocol is asynchronous, which means the server may send"]
            #[doc = "further events until the stop request is processed. The client must wait"]
            #[doc = "for a river_libinput_config_v1.finished event before destroying this"]
            #[doc = "object."]
            fn stop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_libinput_config_v1#{}.stop()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request should be called after the finished event has been received"]
            #[doc = "to complete destruction of the object."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request before the finished event"]
            #[doc = "has been received."]
            #[doc = ""]
            #[doc = "If a client wishes to destroy this object it should send a"]
            #[doc = "river_libinput_config_v1.stop request and wait for a"]
            #[doc = "river_libinput_config_v1.finished event. Once the finished event is"]
            #[doc = "received it is safe to destroy this object and any other objects created"]
            #[doc = "through this interface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_libinput_config_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a acceleration config which can be applied"]
            #[doc = "with river_libinput_device_v1.apply_accel_config."]
            fn create_accel_config(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                profile : super :: super :: super :: river :: river_libinput_config_v1 :: river_libinput_device_v1 :: AccelProfile,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_config_v1#{}.create_accel_config({}, {})",
                        sender_id,
                        id,
                        profile
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_uint(profile.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the server will send no further events on this"]
            #[doc = "object. The client should destroy the object. See"]
            #[doc = "river_libinput_config_v1.destroy for more information."]
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new libinput device has been created. Not every river_input_device_v1"]
            #[doc = "is necessarily a libinput device as well."]
            fn libinput_device(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_libinput_config_v1#{}.finished()", sender_id,);
                            self.finished(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_config_v1#{}.libinput_device({})",
                                sender_id,
                                id
                            );
                            self.libinput_device(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "In general, *_support events will be sent exactly once directly after the"]
    #[doc = "river_libinput_device_v1 is created. *_default events will be sent after"]
    #[doc = "*_support events if the config option is supported, and *_current events"]
    #[doc = "willl be sent after the *_default events and again whenever the config"]
    #[doc = "option is changed."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_libinput_device_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid enum value or similar"]
            InvalidArg = 0u32,
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
                    0u32 => Ok(Self::InvalidArg),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct SendEventsModes : u32 { const Enabled = 0u32 ; const Disabled = 1u32 ; const DisabledOnExternalMouse = 2u32 ; } }
        impl From<SendEventsModes> for u32 {
            fn from(value: SendEventsModes) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for SendEventsModes {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for SendEventsModes {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TapState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<TapState> for u32 {
            fn from(value: TapState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for TapState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TapState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum TapButtonMap {
            #[doc = "1/2/3 finger tap maps to left/right/middle"]
            Lrm = 0u32,
            #[doc = "1/2/3 finger tap maps to left/middle/right"]
            Lmr = 1u32,
        }
        impl From<TapButtonMap> for u32 {
            fn from(value: TapButtonMap) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for TapButtonMap {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Lrm),
                    1u32 => Ok(Self::Lmr),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for TapButtonMap {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum DragState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<DragState> for u32 {
            fn from(value: DragState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for DragState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DragState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum DragLockState {
            Disabled = 0u32,
            EnabledTimeout = 1u32,
            EnabledSticky = 2u32,
        }
        impl From<DragLockState> for u32 {
            fn from(value: DragLockState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for DragLockState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::EnabledTimeout),
                    2u32 => Ok(Self::EnabledSticky),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DragLockState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ThreeFingerDragState {
            Disabled = 0u32,
            Enabled3fg = 1u32,
            Enabled4fg = 2u32,
        }
        impl From<ThreeFingerDragState> for u32 {
            fn from(value: ThreeFingerDragState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ThreeFingerDragState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled3fg),
                    2u32 => Ok(Self::Enabled4fg),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ThreeFingerDragState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum AccelProfile {
            None = 0u32,
            Flat = 1u32,
            Adaptive = 2u32,
            Custom = 4u32,
        }
        impl From<AccelProfile> for u32 {
            fn from(value: AccelProfile) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for AccelProfile {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::Flat),
                    2u32 => Ok(Self::Adaptive),
                    4u32 => Ok(Self::Custom),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AccelProfile {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct AccelProfiles : u32 { const None = 0u32 ; const Flat = 1u32 ; const Adaptive = 2u32 ; const Custom = 4u32 ; } }
        impl From<AccelProfiles> for u32 {
            fn from(value: AccelProfiles) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for AccelProfiles {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for AccelProfiles {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum NaturalScrollState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<NaturalScrollState> for u32 {
            fn from(value: NaturalScrollState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for NaturalScrollState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for NaturalScrollState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum LeftHandedState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<LeftHandedState> for u32 {
            fn from(value: LeftHandedState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for LeftHandedState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for LeftHandedState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ClickMethod {
            None = 0u32,
            ButtonAreas = 1u32,
            Clickfinger = 2u32,
        }
        impl From<ClickMethod> for u32 {
            fn from(value: ClickMethod) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ClickMethod {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::None),
                    1u32 => Ok(Self::ButtonAreas),
                    2u32 => Ok(Self::Clickfinger),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ClickMethod {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ClickMethods : u32 { const None = 0u32 ; const ButtonAreas = 1u32 ; const Clickfinger = 2u32 ; } }
        impl From<ClickMethods> for u32 {
            fn from(value: ClickMethods) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for ClickMethods {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ClickMethods {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ClickfingerButtonMap {
            Lrm = 0u32,
            Lmr = 1u32,
        }
        impl From<ClickfingerButtonMap> for u32 {
            fn from(value: ClickfingerButtonMap) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ClickfingerButtonMap {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Lrm),
                    1u32 => Ok(Self::Lmr),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ClickfingerButtonMap {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum MiddleEmulationState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<MiddleEmulationState> for u32 {
            fn from(value: MiddleEmulationState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for MiddleEmulationState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for MiddleEmulationState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ScrollMethod {
            NoScroll = 0u32,
            TwoFinger = 1u32,
            Edge = 2u32,
            OnButtonDown = 4u32,
        }
        impl From<ScrollMethod> for u32 {
            fn from(value: ScrollMethod) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ScrollMethod {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::NoScroll),
                    1u32 => Ok(Self::TwoFinger),
                    2u32 => Ok(Self::Edge),
                    4u32 => Ok(Self::OnButtonDown),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ScrollMethod {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ScrollMethods : u32 { const NoScroll = 0u32 ; const TwoFinger = 1u32 ; const Edge = 2u32 ; const OnButtonDown = 4u32 ; } }
        impl From<ScrollMethods> for u32 {
            fn from(value: ScrollMethods) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for ScrollMethods {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ScrollMethods {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum ScrollButtonLockState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<ScrollButtonLockState> for u32 {
            fn from(value: ScrollButtonLockState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ScrollButtonLockState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ScrollButtonLockState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum DwtState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<DwtState> for u32 {
            fn from(value: DwtState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for DwtState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DwtState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum DwtpState {
            Disabled = 0u32,
            Enabled = 1u32,
        }
        impl From<DwtpState> for u32 {
            fn from(value: DwtpState) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for DwtpState {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Disabled),
                    1u32 => Ok(Self::Enabled),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DwtpState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_libinput_device_v1 interface. See the module level documentation for more info"]
        pub trait RiverLibinputDeviceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_libinput_device_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the input"]
            #[doc = "device object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_libinput_device_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the send events mode for the device."]
            fn set_send_events(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                mode: SendEventsModes,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_send_events({}, {})",
                        sender_id,
                        result,
                        mode
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(mode.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Configure tap-to-click on this device, with a default mapping of"]
            #[doc = "1, 2, 3 finger tap mapping to left, right, middle click, respectively."]
            fn set_tap(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: TapState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_tap({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the finger number to button number mapping for tap-to-click. The"]
            #[doc = "default mapping on most devices is to have a 1, 2 and 3 finger tap to"]
            #[doc = "map to the left, right and middle button, respectively."]
            fn set_tap_button_map(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                button_map: TapButtonMap,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_tap_button_map({}, {})",
                        sender_id,
                        result,
                        button_map
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(button_map.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Configure tap-and-drag functionality on the device."]
            fn set_drag(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: DragState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_drag({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Configure drag-lock during tapping on this device. When enabled, a"]
            #[doc = "finger may be lifted and put back on the touchpad and the drag process"]
            #[doc = "continues. A timeout for lifting the finger is optional. When disabled,"]
            #[doc = "lifting the finger during a tap-and-drag will immediately stop the drag."]
            #[doc = "See the libinput documentation for more details."]
            fn set_drag_lock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: DragLockState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_drag_lock({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Configure three finger drag functionality for the device."]
            fn set_three_finger_drag(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: ThreeFingerDragState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_three_finger_drag({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set calibration matrix."]
            fn set_calibration_matrix(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                matrix: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_calibration_matrix({}, array[{}])",
                        sender_id,
                        result,
                        matrix.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_array(matrix)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the acceleration profile."]
            fn set_accel_profile(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                profile: AccelProfile,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_accel_profile({}, {})",
                        sender_id,
                        result,
                        profile
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(profile.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the acceleration speed within a range of [-1, 1], where 0 is"]
            #[doc = "the default acceleration for this device, -1 is the slowest acceleration"]
            #[doc = "and 1 is the maximum acceleration available on this device."]
            fn set_accel_speed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                speed: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_accel_speed({}, array[{}])",
                        sender_id,
                        result,
                        speed.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_array(speed)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Apply a pointer accleration config."]
            fn apply_accel_config(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                config: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.apply_accel_config({}, {})",
                        sender_id,
                        result,
                        config
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_object(Some(config))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set natural scroll state."]
            fn set_natural_scroll(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: NaturalScrollState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_natural_scroll({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set left-handed mode state."]
            fn set_left_handed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: LeftHandedState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_left_handed({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set click method."]
            fn set_click_method(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                method: ClickMethod,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_click_method({}, {})",
                        sender_id,
                        result,
                        method
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(method.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 13u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set clickfinger button map."]
            #[doc = "Supported if click_methods.clickfinger is supported."]
            fn set_clickfinger_button_map(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                button_map: ClickfingerButtonMap,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_clickfinger_button_map({}, {})",
                        sender_id,
                        result,
                        button_map
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(button_map.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 14u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set middle mouse button emulation state."]
            fn set_middle_emulation(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: MiddleEmulationState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_middle_emulation({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 15u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set scroll method."]
            fn set_scroll_method(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                method: ScrollMethod,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_scroll_method({}, {})",
                        sender_id,
                        result,
                        method
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(method.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 16u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set scroll button."]
            #[doc = "Supported if scroll_methods.on_button_down is supported."]
            fn set_scroll_button(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                button: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_scroll_button({}, {})",
                        sender_id,
                        result,
                        button
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(button)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 17u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set scroll button lock state."]
            #[doc = "Supported if scroll_methods.on_button_down is supported."]
            fn set_scroll_button_lock(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: ScrollButtonLockState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_scroll_button_lock({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 18u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set disable-while-typing state."]
            fn set_dwt(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: DwtState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_dwt({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 19u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set disable-while-trackpointing state."]
            fn set_dwtp(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                state: DwtpState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_dwtp({}, {})",
                        sender_id,
                        result,
                        state
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(state.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 20u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set rotation angle in degrees clockwise off the logical neutral"]
            #[doc = "position. Angle must be in the range [0-360)."]
            fn set_rotation(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                angle: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_device_v1#{}.set_rotation({}, {})",
                        sender_id,
                        result,
                        angle
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(angle)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 21u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the libinput device has been removed."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object and ignore any"]
            #[doc = "request (other than river_libinput_device_v1.destroy) made after this"]
            #[doc = "event is sent. The client should destroy this object with the"]
            #[doc = "river_libinput_device_v1.destroy request to free up resources."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The river_input_device_v1 corresponding to this libinput device."]
            #[doc = "This event will always be the first event sent on the"]
            #[doc = "river_libinput_device_v1 object, and it will be sent exactly once."]
            fn input_device(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                device: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Supported send events modes."]
            fn send_events_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                modes: SendEventsModes,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default send events mode."]
            fn send_events_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: SendEventsModes,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current send events mode."]
            fn send_events_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                mode: SendEventsModes,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The number of fingers supported for tap-to-click/drag."]
            #[doc = "If finger_count is 0, tap-to-click and drag are unsupported."]
            fn tap_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                finger_count: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default tap-to-click state."]
            fn tap_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: TapState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current tap-to-click state."]
            fn tap_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: TapState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default tap-to-click button map."]
            fn tap_button_map_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button_map: TapButtonMap,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current tap-to-click button map."]
            fn tap_button_map_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button_map: TapButtonMap,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default tap-and-drag state."]
            fn drag_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DragState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current tap-and-drag state."]
            fn drag_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DragState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default drag lock state."]
            fn drag_lock_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DragLockState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current drag lock state."]
            fn drag_lock_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DragLockState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The number of fingers supported for three/four finger drag."]
            #[doc = "If finger_count is less than 3, three finger drag is unsupported."]
            fn three_finger_drag_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                finger_count: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default three finger drag state."]
            fn three_finger_drag_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: ThreeFingerDragState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current three finger drag state."]
            fn three_finger_drag_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: ThreeFingerDragState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A calibration matrix is supported if the supported argument is non-zero."]
            fn calibration_matrix_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default calibration matrix."]
            fn calibration_matrix_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                matrix: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current calibration matrix."]
            fn calibration_matrix_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                matrix: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Supported acceleration profiles."]
            fn accel_profiles_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                profiles: AccelProfiles,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default acceleration profile."]
            fn accel_profile_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                profile: AccelProfile,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current acceleration profile."]
            fn accel_profile_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                profile: AccelProfile,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default acceleration speed."]
            fn accel_speed_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                speed: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current acceleration speed."]
            fn accel_speed_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                speed: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Natural scroll is supported if the supported argument is non-zero."]
            fn natural_scroll_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default natural scroll."]
            fn natural_scroll_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: NaturalScrollState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current natural scroll."]
            fn natural_scroll_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: NaturalScrollState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Left-handed mode is supported if the supported argument is non-zero."]
            fn left_handed_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default left-handed mode."]
            fn left_handed_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: LeftHandedState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current left-handed mode."]
            fn left_handed_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: LeftHandedState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The click methods supported by the device."]
            fn click_method_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                methods: ClickMethods,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default click method."]
            fn click_method_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                method: ClickMethod,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current click method."]
            fn click_method_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                method: ClickMethod,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default clickfinger button map."]
            #[doc = "Supported if click_methods.clickfinger is supported."]
            fn clickfinger_button_map_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button_map: ClickfingerButtonMap,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current clickfinger button map."]
            #[doc = "Supported if click_methods.clickfinger is supported."]
            fn clickfinger_button_map_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button_map: ClickfingerButtonMap,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Middle mouse button emulation is supported if the supported argument is"]
            #[doc = "non-zero."]
            fn middle_emulation_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default middle mouse button emulation."]
            fn middle_emulation_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: MiddleEmulationState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current middle mouse button emulation."]
            fn middle_emulation_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: MiddleEmulationState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The scroll methods supported by the device."]
            fn scroll_method_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                methods: ScrollMethods,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default scroll method."]
            fn scroll_method_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                method: ScrollMethod,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current scroll method."]
            fn scroll_method_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                method: ScrollMethod,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default scroll button."]
            #[doc = "Supported if scroll_methods.on_button_down is supported."]
            fn scroll_button_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current scroll button."]
            #[doc = "Supported if scroll_methods.on_button_down is supported."]
            fn scroll_button_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                button: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default scroll button lock state."]
            #[doc = "Supported if scroll_methods.on_button_down is supported."]
            fn scroll_button_lock_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: ScrollButtonLockState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current scroll button lock state."]
            #[doc = "Supported if scroll_methods.on_button_down is supported."]
            fn scroll_button_lock_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: ScrollButtonLockState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Disable-while-typing is supported if the supported argument is"]
            #[doc = "non-zero."]
            fn dwt_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default disable-while-typing state."]
            fn dwt_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DwtState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current disable-while-typing state."]
            fn dwt_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DwtState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Disable-while-trackpointing is supported if the supported argument is"]
            #[doc = "non-zero."]
            fn dwtp_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default disable-while-trackpointing state."]
            fn dwtp_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DwtpState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current disable-while-trackpointing state."]
            fn dwtp_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                state: DwtpState,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Rotation is supported if the supported argument is non-zero."]
            fn rotation_support(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                supported: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Default rotation angle."]
            fn rotation_default(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                angle: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Current rotation angle."]
            fn rotation_current(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                angle: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_libinput_device_v1#{}.removed()", sender_id,);
                            self.removed(connection, sender_id).await
                        }
                        1u16 => {
                            let device = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.input_device({})",
                                sender_id,
                                device
                            );
                            self.input_device(connection, sender_id, device).await
                        }
                        2u16 => {
                            let modes = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.send_events_support({})",
                                sender_id,
                                modes
                            );
                            self.send_events_support(connection, sender_id, modes.try_into()?)
                                .await
                        }
                        3u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.send_events_default({})",
                                sender_id,
                                mode
                            );
                            self.send_events_default(connection, sender_id, mode.try_into()?)
                                .await
                        }
                        4u16 => {
                            let mode = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.send_events_current({})",
                                sender_id,
                                mode
                            );
                            self.send_events_current(connection, sender_id, mode.try_into()?)
                                .await
                        }
                        5u16 => {
                            let finger_count = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.tap_support({})",
                                sender_id,
                                finger_count
                            );
                            self.tap_support(connection, sender_id, finger_count).await
                        }
                        6u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.tap_default({})",
                                sender_id,
                                state
                            );
                            self.tap_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        7u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.tap_current({})",
                                sender_id,
                                state
                            );
                            self.tap_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        8u16 => {
                            let button_map = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.tap_button_map_default({})",
                                sender_id,
                                button_map
                            );
                            self.tap_button_map_default(
                                connection,
                                sender_id,
                                button_map.try_into()?,
                            )
                            .await
                        }
                        9u16 => {
                            let button_map = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.tap_button_map_current({})",
                                sender_id,
                                button_map
                            );
                            self.tap_button_map_current(
                                connection,
                                sender_id,
                                button_map.try_into()?,
                            )
                            .await
                        }
                        10u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.drag_default({})",
                                sender_id,
                                state
                            );
                            self.drag_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        11u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.drag_current({})",
                                sender_id,
                                state
                            );
                            self.drag_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        12u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.drag_lock_default({})",
                                sender_id,
                                state
                            );
                            self.drag_lock_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        13u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.drag_lock_current({})",
                                sender_id,
                                state
                            );
                            self.drag_lock_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        14u16 => {
                            let finger_count = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.three_finger_drag_support({})",
                                sender_id,
                                finger_count
                            );
                            self.three_finger_drag_support(connection, sender_id, finger_count)
                                .await
                        }
                        15u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.three_finger_drag_default({})",
                                sender_id,
                                state
                            );
                            self.three_finger_drag_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        16u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.three_finger_drag_current({})",
                                sender_id,
                                state
                            );
                            self.three_finger_drag_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        17u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.calibration_matrix_support({})",
                                sender_id,
                                supported
                            );
                            self.calibration_matrix_support(connection, sender_id, supported)
                                .await
                        }
                        18u16 => {
                            let matrix = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.calibration_matrix_default(array[{}])",
                                sender_id,
                                matrix.len()
                            );
                            self.calibration_matrix_default(connection, sender_id, matrix)
                                .await
                        }
                        19u16 => {
                            let matrix = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.calibration_matrix_current(array[{}])",
                                sender_id,
                                matrix.len()
                            );
                            self.calibration_matrix_current(connection, sender_id, matrix)
                                .await
                        }
                        20u16 => {
                            let profiles = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.accel_profiles_support({})",
                                sender_id,
                                profiles
                            );
                            self.accel_profiles_support(connection, sender_id, profiles.try_into()?)
                                .await
                        }
                        21u16 => {
                            let profile = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.accel_profile_default({})",
                                sender_id,
                                profile
                            );
                            self.accel_profile_default(connection, sender_id, profile.try_into()?)
                                .await
                        }
                        22u16 => {
                            let profile = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.accel_profile_current({})",
                                sender_id,
                                profile
                            );
                            self.accel_profile_current(connection, sender_id, profile.try_into()?)
                                .await
                        }
                        23u16 => {
                            let speed = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.accel_speed_default(array[{}])",
                                sender_id,
                                speed.len()
                            );
                            self.accel_speed_default(connection, sender_id, speed).await
                        }
                        24u16 => {
                            let speed = message.array()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.accel_speed_current(array[{}])",
                                sender_id,
                                speed.len()
                            );
                            self.accel_speed_current(connection, sender_id, speed).await
                        }
                        25u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.natural_scroll_support({})",
                                sender_id,
                                supported
                            );
                            self.natural_scroll_support(connection, sender_id, supported)
                                .await
                        }
                        26u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.natural_scroll_default({})",
                                sender_id,
                                state
                            );
                            self.natural_scroll_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        27u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.natural_scroll_current({})",
                                sender_id,
                                state
                            );
                            self.natural_scroll_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        28u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.left_handed_support({})",
                                sender_id,
                                supported
                            );
                            self.left_handed_support(connection, sender_id, supported)
                                .await
                        }
                        29u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.left_handed_default({})",
                                sender_id,
                                state
                            );
                            self.left_handed_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        30u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.left_handed_current({})",
                                sender_id,
                                state
                            );
                            self.left_handed_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        31u16 => {
                            let methods = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.click_method_support({})",
                                sender_id,
                                methods
                            );
                            self.click_method_support(connection, sender_id, methods.try_into()?)
                                .await
                        }
                        32u16 => {
                            let method = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.click_method_default({})",
                                sender_id,
                                method
                            );
                            self.click_method_default(connection, sender_id, method.try_into()?)
                                .await
                        }
                        33u16 => {
                            let method = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.click_method_current({})",
                                sender_id,
                                method
                            );
                            self.click_method_current(connection, sender_id, method.try_into()?)
                                .await
                        }
                        34u16 => {
                            let button_map = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.clickfinger_button_map_default({})",
                                sender_id,
                                button_map
                            );
                            self.clickfinger_button_map_default(
                                connection,
                                sender_id,
                                button_map.try_into()?,
                            )
                            .await
                        }
                        35u16 => {
                            let button_map = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.clickfinger_button_map_current({})",
                                sender_id,
                                button_map
                            );
                            self.clickfinger_button_map_current(
                                connection,
                                sender_id,
                                button_map.try_into()?,
                            )
                            .await
                        }
                        36u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.middle_emulation_support({})",
                                sender_id,
                                supported
                            );
                            self.middle_emulation_support(connection, sender_id, supported)
                                .await
                        }
                        37u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.middle_emulation_default({})",
                                sender_id,
                                state
                            );
                            self.middle_emulation_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        38u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.middle_emulation_current({})",
                                sender_id,
                                state
                            );
                            self.middle_emulation_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        39u16 => {
                            let methods = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_method_support({})",
                                sender_id,
                                methods
                            );
                            self.scroll_method_support(connection, sender_id, methods.try_into()?)
                                .await
                        }
                        40u16 => {
                            let method = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_method_default({})",
                                sender_id,
                                method
                            );
                            self.scroll_method_default(connection, sender_id, method.try_into()?)
                                .await
                        }
                        41u16 => {
                            let method = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_method_current({})",
                                sender_id,
                                method
                            );
                            self.scroll_method_current(connection, sender_id, method.try_into()?)
                                .await
                        }
                        42u16 => {
                            let button = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_button_default({})",
                                sender_id,
                                button
                            );
                            self.scroll_button_default(connection, sender_id, button)
                                .await
                        }
                        43u16 => {
                            let button = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_button_current({})",
                                sender_id,
                                button
                            );
                            self.scroll_button_current(connection, sender_id, button)
                                .await
                        }
                        44u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_button_lock_default({})",
                                sender_id,
                                state
                            );
                            self.scroll_button_lock_default(
                                connection,
                                sender_id,
                                state.try_into()?,
                            )
                            .await
                        }
                        45u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.scroll_button_lock_current({})",
                                sender_id,
                                state
                            );
                            self.scroll_button_lock_current(
                                connection,
                                sender_id,
                                state.try_into()?,
                            )
                            .await
                        }
                        46u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.dwt_support({})",
                                sender_id,
                                supported
                            );
                            self.dwt_support(connection, sender_id, supported).await
                        }
                        47u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.dwt_default({})",
                                sender_id,
                                state
                            );
                            self.dwt_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        48u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.dwt_current({})",
                                sender_id,
                                state
                            );
                            self.dwt_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        49u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.dwtp_support({})",
                                sender_id,
                                supported
                            );
                            self.dwtp_support(connection, sender_id, supported).await
                        }
                        50u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.dwtp_default({})",
                                sender_id,
                                state
                            );
                            self.dwtp_default(connection, sender_id, state.try_into()?)
                                .await
                        }
                        51u16 => {
                            let state = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.dwtp_current({})",
                                sender_id,
                                state
                            );
                            self.dwtp_current(connection, sender_id, state.try_into()?)
                                .await
                        }
                        52u16 => {
                            let supported = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.rotation_support({})",
                                sender_id,
                                supported
                            );
                            self.rotation_support(connection, sender_id, supported)
                                .await
                        }
                        53u16 => {
                            let angle = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.rotation_default({})",
                                sender_id,
                                angle
                            );
                            self.rotation_default(connection, sender_id, angle).await
                        }
                        54u16 => {
                            let angle = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_libinput_device_v1#{}.rotation_current({})",
                                sender_id,
                                angle
                            );
                            self.rotation_current(connection, sender_id, angle).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The result returned by libinput on setting configuration for a device."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_libinput_accel_config_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid enum value or similar"]
            InvalidArg = 0u32,
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
                    0u32 => Ok(Self::InvalidArg),
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
        pub enum AccelType {
            Fallback = 0u32,
            Motion = 1u32,
            Scroll = 2u32,
        }
        impl From<AccelType> for u32 {
            fn from(value: AccelType) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for AccelType {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::Fallback),
                    1u32 => Ok(Self::Motion),
                    2u32 => Ok(Self::Scroll),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for AccelType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_libinput_accel_config_v1 interface. See the module level documentation for more info"]
        pub trait RiverLibinputAccelConfigV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_libinput_accel_config_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the accel"]
            #[doc = "config object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_libinput_accel_config_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Defines the acceleration function for a given movement type"]
            #[doc = "in an acceleration configuration with custom accel profile."]
            fn set_points(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                result: waynest::ObjectId,
                r#type: AccelType,
                step: Vec<u8>,
                points: Vec<u8>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_libinput_accel_config_v1#{}.set_points({}, {}, array[{}], array[{}])",
                        sender_id,
                        result,
                        r#type,
                        step.len(),
                        points.len()
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(result))
                        .put_uint(r#type.into())
                        .put_array(step)
                        .put_array(points)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
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
    #[doc = "The result returned by libinput on setting configuration for a device."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_libinput_result_v1 {
        #[doc = "Trait to implement the river_libinput_result_v1 interface. See the module level documentation for more info"]
        pub trait RiverLibinputResultV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_libinput_result_v1";
            const VERSION: u32 = 1u32;
            #[doc = "The configuration was successfully applied to the device."]
            fn success(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The configuration is unsupported by the device and was ignored."]
            fn unsupported(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The configuration is invalid and was ignored."]
            fn invalid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_libinput_result_v1#{}.success()", sender_id,);
                            self.success(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_libinput_result_v1#{}.unsupported()", sender_id,);
                            self.unsupported(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_libinput_result_v1#{}.invalid()", sender_id,);
                            self.invalid(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows a single \"window manager\" client to determine the"]
#[doc = "window management policy of the compositor. State is globally"]
#[doc = "double-buffered allowing for frame perfect state changes involving multiple"]
#[doc = "windows."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\", \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod river_window_management_v1 {
    #[doc = "This global interface should only be advertised to the window manager"]
    #[doc = "process. Only one window management client may be active at a time. The"]
    #[doc = "compositor should use the unavailable event if necessary to enforce this."]
    #[doc = ""]
    #[doc = "There are two disjoint categories of state managed by this protocol:"]
    #[doc = ""]
    #[doc = "Window management state influences the communication between the server"]
    #[doc = "and individual window clients (e.g. xdg_toplevels). Window management"]
    #[doc = "state includes window dimensions, fullscreen state, keyboard focus,"]
    #[doc = "keyboard bindings, and more."]
    #[doc = ""]
    #[doc = "Rendering state only affects the rendered output of the compositor and"]
    #[doc = "does not influence communication between the server and individual window"]
    #[doc = "clients. Rendering state includes the position and rendering order of"]
    #[doc = "windows, shell surfaces, decoration surfaces, borders, and more."]
    #[doc = ""]
    #[doc = "Window management state may only be modified by the window manager as part"]
    #[doc = "of a manage sequence. A manage sequence is started with the manage_start"]
    #[doc = "event and ended with the manage_finish request. It is a protocol error to"]
    #[doc = "modify window management state outside of a manage sequence."]
    #[doc = ""]
    #[doc = "A manage sequence is always followed by at least one render sequence. A"]
    #[doc = "render sequence is started with the render_start event and ended with the"]
    #[doc = "render_finish request."]
    #[doc = ""]
    #[doc = "Rendering state may be modified by the window manager during a manage"]
    #[doc = "sequence or a render sequence. Regardless of when the rendering state is"]
    #[doc = "modified, it is applied with the next render_finish request. It is a"]
    #[doc = "protocol error to modify rendering state outside of a manage or render"]
    #[doc = "sequence."]
    #[doc = ""]
    #[doc = "The server will start a manage sequence by sending new state and the"]
    #[doc = "manage_start event as soon as possible whenever there is a change in state"]
    #[doc = "that must be communicated with the window manager."]
    #[doc = ""]
    #[doc = "If the window manager client needs to ensure a manage sequence is started"]
    #[doc = "due to a state change the compositor is not aware of, it may send the"]
    #[doc = "manage_dirty request."]
    #[doc = ""]
    #[doc = "The server will start a render sequence by sending new state and the"]
    #[doc = "render_start event as soon as possible whenever there is a change in"]
    #[doc = "window dimensions that must be communicated with the window manager."]
    #[doc = "Multiple render sequences may be made consecutively without a manage"]
    #[doc = "sequence in between, for example if a window independently changes its own"]
    #[doc = "dimensions."]
    #[doc = ""]
    #[doc = "To summarize, the main loop of this protocol is as follows:"]
    #[doc = ""]
    #[doc = "1. The server sends events indicating all changes since the last"]
    #[doc = "manage sequence followed by the manage_start event."]
    #[doc = ""]
    #[doc = "2. The client sends requests modifying window management state or"]
    #[doc = "rendering state (as defined above) followed by the manage_finish"]
    #[doc = "request."]
    #[doc = ""]
    #[doc = "3. The server sends new state to windows and waits for responses."]
    #[doc = ""]
    #[doc = "4. The server sends new window dimensions to the client followed by the"]
    #[doc = "render_start event."]
    #[doc = ""]
    #[doc = "5. The client sends requests modifying rendering state (as defined above)"]
    #[doc = "followed by the render_finish request."]
    #[doc = ""]
    #[doc = "6. If window dimensions change, loop back to step 4."]
    #[doc = "If state that requires a manage sequence changes or if the client makes"]
    #[doc = "a manage_dirty request, loop back to step 1."]
    #[doc = ""]
    #[doc = "For the purposes of frame perfection, the server may delay rendering new"]
    #[doc = "state committed by the windows in step 3 until after step 5 is finished."]
    #[doc = ""]
    #[doc = "It is a protocol error for the client to make a manage_finish or"]
    #[doc = "render_finish request that violates this ordering."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_window_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "request violates manage/render sequence ordering"]
            SequenceOrder = 0u32,
            #[doc = "given wl_surface already has a role"]
            Role = 1u32,
            #[doc = "window manager unresponsive"]
            Unresponsive = 2u32,
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
                    0u32 => Ok(Self::SequenceOrder),
                    1u32 => Ok(Self::Role),
                    2u32 => Ok(Self::Unresponsive),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_window_manager_v1 interface. See the module level documentation for more info"]
        pub trait RiverWindowManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_window_manager_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events on this object."]
            #[doc = ""]
            #[doc = "The Wayland protocol is asynchronous, which means the server may send"]
            #[doc = "further events until the stop request is processed. The client must wait"]
            #[doc = "for a river_window_manager_v1.finished event before destroying this"]
            #[doc = "object."]
            fn stop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_manager_v1#{}.stop()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request should be called after the finished event has been received"]
            #[doc = "to complete destruction of the object."]
            #[doc = ""]
            #[doc = "If a client wishes to destroy this object it should send a"]
            #[doc = "river_window_manager_v1.stop request and wait for a"]
            #[doc = "river_window_manager_v1.finished event. Once the finished event is"]
            #[doc = "received it is safe to destroy this object and any other objects created"]
            #[doc = "through this interface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_manager_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request indicates that the client has made all changes to window"]
            #[doc = "management state it wishes to include in the current manage sequence and"]
            #[doc = "that the server should atomically send these state changes to the"]
            #[doc = "windows and continue with the manage sequence."]
            #[doc = ""]
            #[doc = "After sending this request, it is a protocol error for the client to"]
            #[doc = "make further changes to window management state until the next"]
            #[doc = "manage_start event is received."]
            #[doc = ""]
            #[doc = "See the description of the river_window_manager_v1 interface for a"]
            #[doc = "complete overview of the manage/render sequence loop."]
            fn manage_finish(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_manager_v1#{}.manage_finish()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request ensures a manage sequence is started and that a"]
            #[doc = "manage_start event is sent by the server. If this request is made during"]
            #[doc = "an ongoing manage sequence, a new manage sequence will be started as"]
            #[doc = "soon as the current one is completed."]
            #[doc = ""]
            #[doc = "The client may want to use this request due to an internal state change"]
            #[doc = "that the compositor is not aware of (e.g. a dbus event) which should"]
            #[doc = "affect window management or rendering state."]
            fn manage_dirty(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_manager_v1#{}.manage_dirty()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request indicates that the client has made all changes to rendering"]
            #[doc = "state it wishes to include in the current manage sequence and that the"]
            #[doc = "server should atomically apply and display these state changes to the"]
            #[doc = "user."]
            #[doc = ""]
            #[doc = "After sending this request, it is a protocol error for the client to"]
            #[doc = "make further changes to rendering state until the next manage_start or"]
            #[doc = "render_start event is received, whichever comes first."]
            #[doc = ""]
            #[doc = "See the description of the river_window_manager_v1 interface for a"]
            #[doc = "complete overview of the manage/render sequence loop."]
            fn render_finish(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_manager_v1#{}.render_finish()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a new shell surface for window manager UI and assign the"]
            #[doc = "river_shell_surface_v1 role to the surface."]
            #[doc = ""]
            #[doc = "Providing a wl_surface which already has a role or already has a buffer"]
            #[doc = "attached or committed is a protocol error."]
            fn get_shell_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_manager_v1#{}.get_shell_surface({}, {})",
                        sender_id,
                        id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that window management is not available to the"]
            #[doc = "client, perhaps due to another window management client already running."]
            #[doc = "The circumstances causing this event to be sent are compositor policy."]
            #[doc = ""]
            #[doc = "If sent, this event is guaranteed to be the first and only event sent by"]
            #[doc = "the server."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object. The client should"]
            #[doc = "destroy this object and all objects created through this interface."]
            fn unavailable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the server will send no further events on this"]
            #[doc = "object. The client should destroy the object. See"]
            #[doc = "river_window_manager_v1.destroy for more information."]
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the server has sent events indicating all"]
            #[doc = "state changes since the last manage sequence."]
            #[doc = ""]
            #[doc = "In response to this event, the client should make requests modifying"]
            #[doc = "window management state as it chooses. Then, the client must make the"]
            #[doc = "manage_finish request."]
            #[doc = ""]
            #[doc = "See the description of the river_window_manager_v1 interface for a"]
            #[doc = "complete overview of the manage/render sequence loop."]
            fn manage_start(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the server has sent all river_node_v1.position"]
            #[doc = "and river_window_v1.dimensions events necessary."]
            #[doc = ""]
            #[doc = "In response to this event, the client should make requests modifying"]
            #[doc = "rendering state as it chooses. Then, the client must make the"]
            #[doc = "render_finish request."]
            #[doc = ""]
            #[doc = "See the description of the river_window_manager_v1 interface for a"]
            #[doc = "complete overview of the manage/render sequence loop."]
            fn render_start(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the session has been locked."]
            #[doc = ""]
            #[doc = "The window manager may wish to restrict which key bindings are available"]
            #[doc = "while locked or otherwise use this information."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn session_locked(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the session has been unlocked."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn session_unlocked(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new window has been created."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new logical output has been created, perhaps due to a new physical"]
            #[doc = "monitor being plugged in or perhaps due to a change in configuration."]
            #[doc = ""]
            #[doc = "This event will be followed by river_output_v1.position and dimensions"]
            #[doc = "events as well as a manage_start event after all other new state has"]
            #[doc = "been sent by the server."]
            fn output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new seat has been created."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_window_manager_v1#{}.unavailable()", sender_id,);
                            self.unavailable(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_manager_v1#{}.finished()", sender_id,);
                            self.finished(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_manager_v1#{}.manage_start()", sender_id,);
                            self.manage_start(connection, sender_id).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_manager_v1#{}.render_start()", sender_id,);
                            self.render_start(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_manager_v1#{}.session_locked()",
                                sender_id,
                            );
                            self.session_locked(connection, sender_id).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_manager_v1#{}.session_unlocked()",
                                sender_id,
                            );
                            self.session_unlocked(connection, sender_id).await
                        }
                        6u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_manager_v1#{}.window({})", sender_id, id);
                            self.window(connection, sender_id, id).await
                        }
                        7u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_manager_v1#{}.output({})", sender_id, id);
                            self.output(connection, sender_id, id).await
                        }
                        8u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_manager_v1#{}.seat({})", sender_id, id);
                            self.seat(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This represents a logical window. For example, a window may correspond to"]
    #[doc = "an xdg_toplevel or Xwayland window."]
    #[doc = ""]
    #[doc = "A newly created window will not be displayed until the window manager"]
    #[doc = "proposes window dimensions with the propose_dimensions request as part of"]
    #[doc = "a manage sequence, the server replies with a dimensions event as part of"]
    #[doc = "a render sequence, and that render sequence is finished."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_window_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "window already has a node object"]
            NodeExists = 0u32,
            #[doc = "proposed dimensions out of bounds"]
            InvalidDimensions = 1u32,
            #[doc = "invalid arg to set_borders"]
            InvalidBorder = 2u32,
            #[doc = "invalid arg to set_clip_box"]
            InvalidClipBox = 3u32,
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
                    0u32 => Ok(Self::NodeExists),
                    1u32 => Ok(Self::InvalidDimensions),
                    2u32 => Ok(Self::InvalidBorder),
                    3u32 => Ok(Self::InvalidClipBox),
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
        pub enum DecorationHint {
            #[doc = "only supports client side decoration"]
            OnlySupportsCsd = 0u32,
            #[doc = "client side decoration preferred, both CSD and SSD supported"]
            PrefersCsd = 1u32,
            #[doc = "server side decoration preferred, both CSD and SSD supported"]
            PrefersSsd = 2u32,
            #[doc = "no preference, both CSD and SSD supported"]
            NoPreference = 3u32,
        }
        impl From<DecorationHint> for u32 {
            fn from(value: DecorationHint) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for DecorationHint {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::OnlySupportsCsd),
                    1u32 => Ok(Self::PrefersCsd),
                    2u32 => Ok(Self::PrefersSsd),
                    3u32 => Ok(Self::NoPreference),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for DecorationHint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Edges : u32 { const None = 0u32 ; const Top = 1u32 ; const Bottom = 2u32 ; const Left = 4u32 ; const Right = 8u32 ; } }
        impl From<Edges> for u32 {
            fn from(value: Edges) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Edges {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Edges {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        bitflags::bitflags! { # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Capabilities : u32 { const WindowMenu = 1u32 ; const Maximize = 2u32 ; const Fullscreen = 4u32 ; const Minimize = 8u32 ; } }
        impl From<Capabilities> for u32 {
            fn from(value: Capabilities) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Capabilities {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Capabilities {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the river_window_v1 interface. See the module level documentation for more info"]
        pub trait RiverWindowV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_window_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the window"]
            #[doc = "object and that it may be safely destroyed."]
            #[doc = ""]
            #[doc = "This request should be made after the river_window_v1.closed event or"]
            #[doc = "river_window_manager_v1.finished is received to complete destruction of"]
            #[doc = "the window."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the window be closed. The window may ignore this request or"]
            #[doc = "only close after some delay, perhaps opening a dialog asking the user to"]
            #[doc = "save their work or similar."]
            #[doc = ""]
            #[doc = "The server will send a river_window_v1.closed event if/when the window"]
            #[doc = "has been closed."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn close(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.close()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the node in the render list corresponding to the window."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request more than once for a single"]
            #[doc = "window."]
            fn get_node(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.get_node({})", sender_id, id);
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request proposes dimensions for the window in the compositor's"]
            #[doc = "logical coordinate space."]
            #[doc = ""]
            #[doc = "The width and height must be greater than or equal to zero. If the width"]
            #[doc = "or height is zero the window will be allowed to decide its own"]
            #[doc = "dimensions."]
            #[doc = ""]
            #[doc = "The window may not take the exact dimensions proposed. The actual"]
            #[doc = "dimensions taken by the window will be sent in a subsequent"]
            #[doc = "river_window_v1.dimensions event. For example, a terminal emulator may"]
            #[doc = "only allow dimensions that are multiple of the cell size."]
            #[doc = ""]
            #[doc = "When a propose_dimensions request is made, the server must send a"]
            #[doc = "dimensions event in response as soon as possible. It may not be possible"]
            #[doc = "to send a dimensions event in the very next render sequence if, for"]
            #[doc = "example, the window takes too long to respond to the first proposed"]
            #[doc = "dimensions. In this case, the server will send the dimensions event in a"]
            #[doc = "future render sequence. The window will not be displayed until the first"]
            #[doc = "dimensions event is received and the render sequence is finished."]
            #[doc = ""]
            #[doc = "Note that the dimensions of a river_window_v1 refer to the dimensions of"]
            #[doc = "the window content and are unaffected by the presence of borders or"]
            #[doc = "decoration surfaces."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn propose_dimensions(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.propose_dimensions({}, {})",
                        sender_id,
                        width,
                        height
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the window be hidden. Has no effect if the window is"]
            #[doc = "already hidden. Hides any window borders and decorations as well."]
            #[doc = ""]
            #[doc = "Newly created windows are considered shown unless explicitly hidden with"]
            #[doc = "the hide request."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn hide(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.hide()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the window be shown. Has no effect if the window is not"]
            #[doc = "hidden. Does not guarantee that the window is visible as it may be"]
            #[doc = "completely obscured by other windows placed above it for example."]
            #[doc = ""]
            #[doc = "Newly created windows are considered shown unless explicitly hidden with"]
            #[doc = "the hide request."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn show(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.show()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tell the client to use client side decoration and draw its own title"]
            #[doc = "bar, borders, etc."]
            #[doc = ""]
            #[doc = "This is the default if neither this request nor the use_ssd request is"]
            #[doc = "ever made."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn use_csd(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.use_csd()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tell the client to use server side decoration and not draw any client"]
            #[doc = "side decorations."]
            #[doc = ""]
            #[doc = "This request will have no effect if the client only supports client side"]
            #[doc = "decoration, see the decoration_hint event."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn use_ssd(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.use_ssd()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request decorates the window with borders drawn by the compositor"]
            #[doc = "on the specified edges of the window. Borders are drawn above the window"]
            #[doc = "content."]
            #[doc = ""]
            #[doc = "Corners are drawn only between borders on adjacent edges. If e.g. the"]
            #[doc = "left edge has a border and the top edge does not, the border drawn on"]
            #[doc = "the left edge will not extend vertically beyond the top edge of the"]
            #[doc = "window."]
            #[doc = ""]
            #[doc = "Borders are not drawn while the window is fullscreen."]
            #[doc = ""]
            #[doc = "The color is defined by four 32-bit RGBA values. Unless specified in"]
            #[doc = "another protocol extension, the RGBA values use pre-multiplied alpha."]
            #[doc = ""]
            #[doc = "Setting the edges to none or the width to 0 disables the borders."]
            #[doc = "Setting a negative width is a protocol error."]
            #[doc = ""]
            #[doc = "This request completely overrides all previous set_borders requests."]
            #[doc = "Only the most recent set_borders request has an effect."]
            #[doc = ""]
            #[doc = "Note that the position/dimensions of a river_window_v1 refer to the"]
            #[doc = "position/dimensions of the window content and are unaffected by the"]
            #[doc = "presence of borders or decoration surfaces."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn set_borders(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                edges: Edges,
                width: i32,
                r: u32,
                g: u32,
                b: u32,
                a: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.set_borders({}, {}, {}, {}, {}, {})",
                        sender_id,
                        edges,
                        width,
                        r,
                        g,
                        b,
                        a
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(edges.into())
                        .put_int(width)
                        .put_uint(r)
                        .put_uint(g)
                        .put_uint(b)
                        .put_uint(a)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is part of a tiled layout and adjacent to"]
            #[doc = "other elements in the tiled layout on the given edges."]
            #[doc = ""]
            #[doc = "The window should use this information to change the style of its client"]
            #[doc = "side decorations and avoid drawing e.g. drop shadows outside of the"]
            #[doc = "window dimensions on the tiled edges."]
            #[doc = ""]
            #[doc = "Setting the edges argument to none informs the window that it is not"]
            #[doc = "part of a tiled layout. If this request is never made, the window is"]
            #[doc = "informed that it is not part of a tiled layout."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn set_tiled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                edges: Edges,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.set_tiled({})", sender_id, edges);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(edges.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 9u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a decoration surface and assign the river_decoration_v1 role to"]
            #[doc = "the surface. The created decoration is placed above the window in"]
            #[doc = "rendering order, see the description of river_decoration_v1."]
            #[doc = ""]
            #[doc = "Providing a wl_surface which already has a role or already has a buffer"]
            #[doc = "attached or committed is a protocol error."]
            fn get_decoration_above(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.get_decoration_above({}, {})",
                        sender_id,
                        id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 10u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create a decoration surface and assign the river_decoration_v1 role to"]
            #[doc = "the surface. The created decoration is placed below the window in"]
            #[doc = "rendering order, see the description of river_decoration_v1."]
            #[doc = ""]
            #[doc = "Providing a wl_surface which already has a role or already has a buffer"]
            #[doc = "attached or committed is a protocol error."]
            fn get_decoration_below(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.get_decoration_below({}, {})",
                        sender_id,
                        id,
                        surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 11u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is being resized. The window manager should"]
            #[doc = "use this request to inform windows that are the target of an interactive"]
            #[doc = "resize for example."]
            #[doc = ""]
            #[doc = "The window manager remains responsible for handling the position and"]
            #[doc = "dimensions of the window while it is resizing."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn inform_resize_start(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.inform_resize_start()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 12u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is no longer being resized. The window manager"]
            #[doc = "should use this request to inform windows that are the target of an"]
            #[doc = "interactive resize that the interactive resize has ended for example."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn inform_resize_end(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.inform_resize_end()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 13u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request informs the window of the capabilities supported by the"]
            #[doc = "window manager. If the window manager, for example, ignores requests to"]
            #[doc = "be maximized from the window it should not tell the window that it"]
            #[doc = "supports the maximize capability."]
            #[doc = ""]
            #[doc = "The window might use this information to, for example, only show a"]
            #[doc = "maximize button if the window manager supports the maximize capability."]
            #[doc = ""]
            #[doc = "The window manager client should use this request to set capabilities"]
            #[doc = "for all new windows. If this request is never made, the compositor will"]
            #[doc = "inform windows that all capabilities are supported."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn set_capabilities(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                caps: Capabilities,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.set_capabilities({})",
                        sender_id,
                        caps
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_uint(caps.into()).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 14u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is maximized. The window might use this"]
            #[doc = "information to adapt the style of its client-side window decorations for"]
            #[doc = "example."]
            #[doc = ""]
            #[doc = "The window manager remains responsible for handling the position and"]
            #[doc = "dimensions of the window while it is maximized."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn inform_maximized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.inform_maximized()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 15u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is unmaximized. The window might use this"]
            #[doc = "information to adapt the style of its client-side window decorations for"]
            #[doc = "example."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn inform_unmaximized(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.inform_unmaximized()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 16u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is fullscreen. The window might use this"]
            #[doc = "information to adapt the style of its client-side window decorations for"]
            #[doc = "example."]
            #[doc = ""]
            #[doc = "This request does not affect the size/position of the window or cause it"]
            #[doc = "to become the only window rendered, see the river_window_v1.fullscreen"]
            #[doc = "and exit_fullscreen requests for that."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn inform_fullscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.inform_fullscreen()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 17u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Inform the window that it is not fullscreen. The window might use this"]
            #[doc = "information to adapt the style of its client-side window decorations for"]
            #[doc = "example."]
            #[doc = ""]
            #[doc = "This request does not affect the size/position of the window or cause it"]
            #[doc = "to become the only window rendered, see the river_window_v1.fullscreen"]
            #[doc = "and exit_fullscreen requests for that."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn inform_not_fullscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.inform_not_fullscreen()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 18u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Make the window fullscreen on the given output. If multiple windows are"]
            #[doc = "fullscreen on the same output at the same time only the \"top\" window in"]
            #[doc = "rendering order shall be displayed."]
            #[doc = ""]
            #[doc = "All river_shell_surface_v1 objects above the top fullscreen window in"]
            #[doc = "the rendering order will continue to be rendered."]
            #[doc = ""]
            #[doc = "The compositor will handle the position and dimensions of the window"]
            #[doc = "while it is fullscreen. The set_position and propose_dimensions requests"]
            #[doc = "shall not affect the current position and dimensions of a fullscreen"]
            #[doc = "window."]
            #[doc = ""]
            #[doc = "The compositor will clip window content, decoration surfaces, and"]
            #[doc = "borders to the given output's dimensions while the window is fullscreen."]
            #[doc = "The effects of set_clip_box and set_content_clip_box are ignored while"]
            #[doc = "the window is fullscreen."]
            #[doc = ""]
            #[doc = "If the output on which a window is currently fullscreen is removed, the"]
            #[doc = "windowing state is modified as if there were an exit_fullscreen request"]
            #[doc = "made in the same manage sequence as the river_output_v1.removed event."]
            #[doc = ""]
            #[doc = "This request does not inform the window that it is fullscreen, see the"]
            #[doc = "river_window_v1.inform_fullscreen and inform_not_fullscreen requests."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn fullscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.fullscreen({})", sender_id, output);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(output))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 19u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Make the window not fullscreen."]
            #[doc = ""]
            #[doc = "The position and dimensions are undefined after this request is made"]
            #[doc = "until a manage sequence in which the window manager makes the"]
            #[doc = "propose_dimensions and set_position requests is completed."]
            #[doc = ""]
            #[doc = "The window manager should make propose_dimensions and set_position"]
            #[doc = "requests in the same manage sequence as the exit_fullscreen request for"]
            #[doc = "frame perfection."]
            #[doc = ""]
            #[doc = "This request does not inform the window that it is fullscreen, see the"]
            #[doc = "river_window_v1.inform_fullscreen and inform_not_fullscreen requests."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn exit_fullscreen(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_window_v1#{}.exit_fullscreen()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 20u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Clip the window, including borders and decoration surfaces, to the box"]
            #[doc = "specified by the x, y, width, and height arguments. The x/y position of"]
            #[doc = "the box is relative to the top left corner of the window."]
            #[doc = ""]
            #[doc = "The width and height arguments must be greater than or equal to 0."]
            #[doc = ""]
            #[doc = "Setting a clip box with 0 width or height disables clipping."]
            #[doc = ""]
            #[doc = "The clip box is ignored while the window is fullscreen."]
            #[doc = ""]
            #[doc = "Both set_clip_box and set_content_clip_box may be enabled simultaneously."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn set_clip_box(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.set_clip_box({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 21u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Clip the content of the window, excluding borders and decoration"]
            #[doc = "surfaces, to the box specified by the x, y, width, and height arguments."]
            #[doc = "The x/y position of the box is relative to the top left corner of the"]
            #[doc = "window."]
            #[doc = ""]
            #[doc = "Borders drawn by the compositor (see set_borders) are placed around the"]
            #[doc = "intersection of the window content (as defined by the dimensions event)"]
            #[doc = "and the content clip box when content clipping is enabled."]
            #[doc = ""]
            #[doc = "The width and height arguments must be greater than or equal to 0."]
            #[doc = ""]
            #[doc = "Setting a box with 0 width or height disables content clipping."]
            #[doc = ""]
            #[doc = "The content clip box is ignored while the window is fullscreen."]
            #[doc = ""]
            #[doc = "Both set_clip_box and set_content_clip_box may be enabled simultaneously."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn set_content_clip_box(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_window_v1#{}.set_content_clip_box({}, {}, {}, {})",
                        sender_id,
                        x,
                        y,
                        width,
                        height
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_int(x)
                        .put_int(y)
                        .put_int(width)
                        .put_int(height)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 22u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The window has been closed by the server, perhaps due to an"]
            #[doc = "xdg_toplevel.close request or similar."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object and ignore any"]
            #[doc = "request other than river_window_v1.destroy made after this event is"]
            #[doc = "sent. The client should destroy this object with the"]
            #[doc = "river_window_v1.destroy request to free up resources."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn closed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event informs the window manager of the window's preferred min/max"]
            #[doc = "dimensions. These preferences are a hint, and the window manager is free"]
            #[doc = "to propose dimensions outside of these bounds."]
            #[doc = ""]
            #[doc = "All min/max width/height values must be strictly greater than or equal"]
            #[doc = "to 0. A value of 0 indicates that the window has no preference for that"]
            #[doc = "value."]
            #[doc = ""]
            #[doc = "The min_width/min_height must be strictly less than or equal to the"]
            #[doc = "max_width/max_height."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn dimensions_hint(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                min_width: i32,
                min_height: i32,
                max_width: i32,
                max_height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates the dimensions of the window in the compositor's"]
            #[doc = "logical coordinate space. The width and height must be strictly greater"]
            #[doc = "than zero."]
            #[doc = ""]
            #[doc = "Note that the dimensions of a river_window_v1 refer to the dimensions of"]
            #[doc = "the window content and are unaffected by the presence of borders or"]
            #[doc = "decoration surfaces."]
            #[doc = ""]
            #[doc = "This event is sent as part of a render sequence before the render_start"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "It may be sent due to a propose_dimensions request in a previous manage"]
            #[doc = "sequence or because a window independently decides to change its"]
            #[doc = "dimensions."]
            fn dimensions(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The window set an application ID."]
            #[doc = ""]
            #[doc = "The app_id argument will be null if the window has never set an"]
            #[doc = "application ID or if the window cleared its application ID. (Xwayland"]
            #[doc = "windows may do this for example, though xdg-toplevels may not.)"]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn app_id(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                app_id: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The window set a title."]
            #[doc = ""]
            #[doc = "The title argument will be null if the window has never set a title or"]
            #[doc = "if the window cleared its title. (Xwayland windows may do this for"]
            #[doc = "example, though xdg-toplevels may not.)"]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn title(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                title: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The window set a parent window. If this event is never received or if"]
            #[doc = "the parent argument is null then the window has no parent."]
            #[doc = ""]
            #[doc = "A surface with a parent set might be a dialog, file picker, or similar"]
            #[doc = "for the parent window."]
            #[doc = ""]
            #[doc = "Child windows should generally be rendered directly above their parent."]
            #[doc = ""]
            #[doc = "The compositor must guarantee that there are no loops in the window"]
            #[doc = "tree: a parent must not be the descendant of one of its children."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn parent(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                parent: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Information from the window about the supported and preferred client"]
            #[doc = "side/server side decoration options."]
            #[doc = ""]
            #[doc = "This event may be sent multiple times over the lifetime of the window if"]
            #[doc = "the window changes its preferences."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn decoration_hint(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: DecorationHint,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event informs the window manager that the window has requested to"]
            #[doc = "be interactively moved using the pointer. The seat argument indicates the"]
            #[doc = "seat for the move."]
            #[doc = ""]
            #[doc = "The xdg-shell protocol for example allows windows to request that an"]
            #[doc = "interactive move be started, perhaps when a client-side rendered"]
            #[doc = "titlebar is dragged."]
            #[doc = ""]
            #[doc = "The window manager may use the river_seat_v1.op_start_pointer request to"]
            #[doc = "interactively move the window or ignore this event entirely."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn pointer_move_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event informs the window manager that the window has requested to"]
            #[doc = "be interactively resized using the pointer. The seat argument indicates"]
            #[doc = "the seat for the resize."]
            #[doc = ""]
            #[doc = "The edges argument indicates which edges the window has requested to be"]
            #[doc = "resized from. The edges argument will never be none and will never have"]
            #[doc = "both top and bottom or both left and right edges set."]
            #[doc = ""]
            #[doc = "The xdg-shell protocol for example allows windows to request that an"]
            #[doc = "interactive resize be started, perhaps when the corner of client-side"]
            #[doc = "rendered decorations is dragged."]
            #[doc = ""]
            #[doc = "The window manager may use the river_seat_v1.op_start_pointer request to"]
            #[doc = "interactively resize the window or ignore this event entirely."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn pointer_resize_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                edges: Edges,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The xdg-shell protocol for example allows windows to request that a"]
            #[doc = "window menu be shown, for example when the user right clicks on client"]
            #[doc = "side window decorations."]
            #[doc = ""]
            #[doc = "A window menu might include options to maximize or minimize the window."]
            #[doc = ""]
            #[doc = "The window manager is free to ignore this request and decide what the"]
            #[doc = "window menu contains if it does choose to show one."]
            #[doc = ""]
            #[doc = "The x and y arguments indicate where the window requested that the"]
            #[doc = "window menu be shown."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn show_window_menu_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The xdg-shell protocol for example allows windows to request to be"]
            #[doc = "maximized."]
            #[doc = ""]
            #[doc = "The window manager is free to honor this request using"]
            #[doc = "river_window_v1.inform_maximize or ignore it."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn maximize_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The xdg-shell protocol for example allows windows to request to be"]
            #[doc = "unmaximized."]
            #[doc = ""]
            #[doc = "The window manager is free to honor this request using"]
            #[doc = "river_window_v1.inform_unmaximized or ignore it."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn unmaximize_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The xdg-shell protocol for example allows windows to request that they"]
            #[doc = "be made fullscreen and allows them to provide an output preference."]
            #[doc = ""]
            #[doc = "The window manager is free to honor this request using"]
            #[doc = "river_window_v1.fullscreen or ignore it."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn fullscreen_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                output: Option<waynest::ObjectId>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The xdg-shell protocol for example allows windows to request to exit"]
            #[doc = "fullscreen."]
            #[doc = ""]
            #[doc = "The window manager is free to honor this request using"]
            #[doc = "river_window_v1.exit_fullscreen or ignore it."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn exit_fullscreen_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The xdg-shell protocol for example allows windows to request to be"]
            #[doc = "minimized."]
            #[doc = ""]
            #[doc = "The window manager is free to ignore this request, hide the window, or"]
            #[doc = "do whatever else it chooses."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn minimize_requested(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event gives an unreliable PID of the process that created the"]
            #[doc = "window. Obtaining this information is inherently racy due to PID reuse."]
            #[doc = "Therefore, this PID must not be used for anything security sensitive."]
            #[doc = ""]
            #[doc = "Note also that a single process may create multiple windows, so there is"]
            #[doc = "not necessarily a 1-to-1 mapping from PID to window. Multiple windows"]
            #[doc = "may have the same PID."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_window_v1 is created and never"]
            #[doc = "sent again."]
            fn unreliable_pid(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                unreliable_pid: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_window_v1#{}.closed()", sender_id,);
                            self.closed(connection, sender_id).await
                        }
                        1u16 => {
                            let min_width = message.int()?;
                            let min_height = message.int()?;
                            let max_width = message.int()?;
                            let max_height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.dimensions_hint({}, {}, {}, {})",
                                sender_id,
                                min_width,
                                min_height,
                                max_width,
                                max_height
                            );
                            self.dimensions_hint(
                                connection, sender_id, min_width, min_height, max_width, max_height,
                            )
                            .await
                        }
                        2u16 => {
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.dimensions({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.dimensions(connection, sender_id, width, height).await
                        }
                        3u16 => {
                            let app_id = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.app_id(\"{}\")",
                                sender_id,
                                app_id
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.app_id(connection, sender_id, app_id).await
                        }
                        4u16 => {
                            let title = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.title(\"{}\")",
                                sender_id,
                                title.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.title(connection, sender_id, title).await
                        }
                        5u16 => {
                            let parent = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.parent({})",
                                sender_id,
                                parent
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.parent(connection, sender_id, parent).await
                        }
                        6u16 => {
                            let hint = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.decoration_hint({})",
                                sender_id,
                                hint
                            );
                            self.decoration_hint(connection, sender_id, hint.try_into()?)
                                .await
                        }
                        7u16 => {
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.pointer_move_requested({})",
                                sender_id,
                                seat
                            );
                            self.pointer_move_requested(connection, sender_id, seat)
                                .await
                        }
                        8u16 => {
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let edges = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.pointer_resize_requested({}, {})",
                                sender_id,
                                seat,
                                edges
                            );
                            self.pointer_resize_requested(
                                connection,
                                sender_id,
                                seat,
                                edges.try_into()?,
                            )
                            .await
                        }
                        9u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.show_window_menu_requested({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.show_window_menu_requested(connection, sender_id, x, y)
                                .await
                        }
                        10u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_v1#{}.maximize_requested()", sender_id,);
                            self.maximize_requested(connection, sender_id).await
                        }
                        11u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_v1#{}.unmaximize_requested()", sender_id,);
                            self.unmaximize_requested(connection, sender_id).await
                        }
                        12u16 => {
                            let output = message.object()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.fullscreen_requested({})",
                                sender_id,
                                output
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.fullscreen_requested(connection, sender_id, output)
                                .await
                        }
                        13u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.exit_fullscreen_requested()",
                                sender_id,
                            );
                            self.exit_fullscreen_requested(connection, sender_id).await
                        }
                        14u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_window_v1#{}.minimize_requested()", sender_id,);
                            self.minimize_requested(connection, sender_id).await
                        }
                        15u16 => {
                            let unreliable_pid = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_window_v1#{}.unreliable_pid({})",
                                sender_id,
                                unreliable_pid
                            );
                            self.unreliable_pid(connection, sender_id, unreliable_pid)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The rendering order of windows with decorations is follows:"]
    #[doc = ""]
    #[doc = "1. Decorations created with get_decoration_below at the bottom"]
    #[doc = "2. Window content"]
    #[doc = "3. Borders configured with river_window_v1.set_borders"]
    #[doc = "4. Decorations created with get_decoration_above at the top"]
    #[doc = ""]
    #[doc = "The relative ordering of decoration surfaces above/below a window is"]
    #[doc = "undefined by this protocol and left up to the compositor."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_decoration_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "failed to commit the surface before the window manager commit"]
            NoCommit = 0u32,
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
                    0u32 => Ok(Self::NoCommit),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_decoration_v1 interface. See the module level documentation for more info"]
        pub trait RiverDecorationV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_decoration_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the decoration"]
            #[doc = "object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_decoration_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request sets the offset of the decoration surface from the top left"]
            #[doc = "corner of the window."]
            #[doc = ""]
            #[doc = "If this request is never sent, the x and y offsets are undefined by this"]
            #[doc = "protocol and left up to the compositor."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn set_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_decoration_v1#{}.set_offset({}, {})",
                        sender_id,
                        x,
                        y
                    );
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_int(x).put_int(y).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Synchronize application of the next wl_surface.commit request on the"]
            #[doc = "decoration surface with rest of the state atomically applied with the"]
            #[doc = "next river_window_manager_v1.render_finish request."]
            #[doc = ""]
            #[doc = "The client must make a wl_surface.commit request on the decoration"]
            #[doc = "surface after this request and before the render_finish request, failure"]
            #[doc = "to do so is a protocol error."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn sync_next_commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_decoration_v1#{}.sync_next_commit()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
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
    #[doc = "The window manager might use a shell surface to display a status bar,"]
    #[doc = "background image, desktop notifications, launcher, desktop menu, or"]
    #[doc = "whatever else it wants."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_shell_surface_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "shell surface already has a node object"]
            NodeExists = 0u32,
            #[doc = "failed to commit the surface before the window manager commit"]
            NoCommit = 1u32,
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
                    0u32 => Ok(Self::NodeExists),
                    1u32 => Ok(Self::NoCommit),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_shell_surface_v1 interface. See the module level documentation for more info"]
        pub trait RiverShellSurfaceV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_shell_surface_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the shell"]
            #[doc = "surface object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_shell_surface_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Get the node in the render list corresponding to the shell surface."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request more than once for a single"]
            #[doc = "shell surface."]
            fn get_node(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_shell_surface_v1#{}.get_node({})", sender_id, id);
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_object(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Synchronize application of the next wl_surface.commit request on the"]
            #[doc = "shell surface with rest of the rendering state atomically applied with"]
            #[doc = "the next river_window_manager_v1.render_finish request."]
            #[doc = ""]
            #[doc = "The client must make a wl_surface.commit request on the shell surface"]
            #[doc = "after this request and before the render_finish request, failure to do"]
            #[doc = "so is a protocol error."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn sync_next_commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_shell_surface_v1#{}.sync_next_commit()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
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
    #[doc = "The render list is a list of nodes that determines the rendering order of"]
    #[doc = "the compositor. Nodes may correspond to windows or shell surfaces. The"]
    #[doc = "relative ordering of nodes may be changed with the place_above and"]
    #[doc = "place_below requests, changing the rendering order."]
    #[doc = ""]
    #[doc = "The initial position of a node in the render list is undefined, the window"]
    #[doc = "manager client must use the place_above or place_below request to"]
    #[doc = "guarantee a specific rendering order."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_node_v1 {
        #[doc = "Trait to implement the river_node_v1 interface. See the module level documentation for more info"]
        pub trait RiverNodeV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_node_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the node"]
            #[doc = "object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_node_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the absolute position of the node in the compositor's logical"]
            #[doc = "coordinate space. The x and y coordinates may be positive or negative."]
            #[doc = ""]
            #[doc = "Note that the position of a river_window_v1 refers to the position of"]
            #[doc = "the window content and is unaffected by the presence of borders or"]
            #[doc = "decoration surfaces."]
            #[doc = ""]
            #[doc = "If this request is never sent, the position of the node is undefined by"]
            #[doc = "this protocol and left up to the compositor."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn set_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_node_v1#{}.set_position({}, {})", sender_id, x, y);
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_int(x).put_int(y).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request places the node above all other nodes in the compositor's"]
            #[doc = "render list."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn place_top(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_node_v1#{}.place_top()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request places the node below all other nodes in the compositor's"]
            #[doc = "render list."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn place_bottom(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_node_v1#{}.place_bottom()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request places the node directly above another node in the"]
            #[doc = "compositor's render list."]
            #[doc = ""]
            #[doc = "Attempting to place a node above itself has no effect."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn place_above(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                other: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_node_v1#{}.place_above({})", sender_id, other);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(other))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request places the node directly below another node in the"]
            #[doc = "compositor's render list."]
            #[doc = ""]
            #[doc = "Attempting to place a node below itself has no effect."]
            #[doc = ""]
            #[doc = "This request modifies rendering state and may only be made as part of a"]
            #[doc = "render sequence, see the river_window_manager_v1 description."]
            fn place_below(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                other: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_node_v1#{}.place_below({})", sender_id, other);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(other))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
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
    #[doc = "An area in the compositor's logical coordinate space that should be"]
    #[doc = "treated as a single output for window management purposes. This area may"]
    #[doc = "correspond to a single physical output or multiple physical outputs in the"]
    #[doc = "case of mirroring or tiled monitors depending on the hardware and"]
    #[doc = "compositor configuration."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_output_v1 {
        #[doc = "Trait to implement the river_output_v1 interface. See the module level documentation for more info"]
        pub trait RiverOutputV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_output_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the output"]
            #[doc = "object and that it may be safely destroyed."]
            #[doc = ""]
            #[doc = "This request should be made after the river_output_v1.removed event is"]
            #[doc = "received to complete destruction of the output."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_output_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the logical output is no longer conceptually"]
            #[doc = "part of window management space."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object and ignore any"]
            #[doc = "request (other than river_output_v1.destroy) made after this event is"]
            #[doc = "sent. The client should destroy this object with the"]
            #[doc = "river_output_v1.destroy request to free up resources."]
            #[doc = ""]
            #[doc = "This event may be sent because a corresponding physical output has been"]
            #[doc = "physically unplugged or because some output configuration has changed."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The wl_output object corresponding to the river_output_v1. The argument"]
            #[doc = "is the global name of the wl_output advertised with wl_registry.global."]
            #[doc = ""]
            #[doc = "It is guaranteed that the corresponding wl_output is advertised before"]
            #[doc = "this event is sent."]
            #[doc = ""]
            #[doc = "This event is sent exactly once. The wl_output associated with a"]
            #[doc = "river_output_v1 cannot change. It is guaranteed that there is a 1-to-1"]
            #[doc = "mapping between wl_output and river_output_v1 objects."]
            #[doc = ""]
            #[doc = "The global_remove event for the corresponding wl_output may be sent"]
            #[doc = "before the river_output_v1.remove event. This is due to the fact that"]
            #[doc = "river_output_v1 state changes are synced to the river window management"]
            #[doc = "manage sequence while changes to globals are not."]
            #[doc = ""]
            #[doc = "Rationale: The window manager may need information provided by the"]
            #[doc = "wl_output interface such as the name/description. It also may need the"]
            #[doc = "wl_output object to start screencopy for example."]
            fn wl_output(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates the position of the output in the compositor's"]
            #[doc = "logical coordinate space. The x and y coordinates may be positive or"]
            #[doc = "negative."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_output_v1 is created and again"]
            #[doc = "whenever the position changes."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            #[doc = ""]
            #[doc = "The server must guarantee that the position and dimensions events do not"]
            #[doc = "cause the areas of multiple logical outputs to overlap when the"]
            #[doc = "corresponding manage_start event is received."]
            fn position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates the dimensions of the output in the compositor's"]
            #[doc = "logical coordinate space. The width and height will always be strictly"]
            #[doc = "greater than zero."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_output_v1 is created and again"]
            #[doc = "whenever the dimensions change."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            #[doc = ""]
            #[doc = "The server must guarantee that the position and dimensions events do not"]
            #[doc = "cause the areas of multiple logical outputs to overlap when the"]
            #[doc = "corresponding manage_start event is received."]
            fn dimensions(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_output_v1#{}.removed()", sender_id,);
                            self.removed(connection, sender_id).await
                        }
                        1u16 => {
                            let name = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_output_v1#{}.wl_output({})", sender_id, name);
                            self.wl_output(connection, sender_id, name).await
                        }
                        2u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_output_v1#{}.position({}, {})", sender_id, x, y);
                            self.position(connection, sender_id, x, y).await
                        }
                        3u16 => {
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_output_v1#{}.dimensions({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.dimensions(connection, sender_id, width, height).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This object represents a single user's collection of input devices. It"]
    #[doc = "allows the window manager to route keyboard input to windows, get"]
    #[doc = "high-level information about pointer input, define keyboard and pointer"]
    #[doc = "bindings, etc."]
    #[doc = ""]
    #[doc = "TODO:"]
    #[doc = "- touch input"]
    #[doc = "- tablet input"]
    #[allow(clippy::too_many_arguments)]
    pub mod river_seat_v1 {
        bitflags::bitflags! { # [doc = "This enum is used to describe the keyboard modifiers that must be held"] # [doc = "down to trigger a key binding or pointer binding."] # [doc = ""] # [doc = "Note that river and wlroots use the values 2 and 16 for capslock and"] # [doc = "numlock internally. It doesn't make sense to use locked modifiers for"] # [doc = "bindings however so these values are not included in this enum."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Modifiers : u32 { const None = 0u32 ; const Shift = 1u32 ; const Ctrl = 4u32 ; # [doc = "commonly called alt"] const Mod1 = 8u32 ; const Mod3 = 32u32 ; # [doc = "commonly called super or logo"] const Mod4 = 64u32 ; const Mod5 = 128u32 ; } }
        impl From<Modifiers> for u32 {
            fn from(value: Modifiers) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for Modifiers {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for Modifiers {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the river_seat_v1 interface. See the module level documentation for more info"]
        pub trait RiverSeatV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_seat_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the seat"]
            #[doc = "object and that it may be safely destroyed."]
            #[doc = ""]
            #[doc = "This request should be made after the river_seat_v1.removed event is"]
            #[doc = "received to complete destruction of the seat."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_seat_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the compositor send keyboard input to the given window."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn focus_window(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                window: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_seat_v1#{}.focus_window({})", sender_id, window);
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(window))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the compositor send keyboard input to the given shell"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn focus_shell_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                shell_surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_seat_v1#{}.focus_shell_surface({})",
                        sender_id,
                        shell_surface
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(shell_surface))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Request that the compositor not send keyboard input to any client."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn clear_focus(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_seat_v1#{}.clear_focus()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Start an interactive pointer operation. During the operation, op_delta"]
            #[doc = "events will be sent based on pointer input."]
            #[doc = ""]
            #[doc = "When all pointer buttons are released, the op_release event is sent."]
            #[doc = ""]
            #[doc = "The pointer operation continues until the op_end request is made during"]
            #[doc = "a manage sequence and that manage sequence is finished."]
            #[doc = ""]
            #[doc = "The window manager may use this operation to implement interactive"]
            #[doc = "move/resize of windows by setting the position of windows and proposing"]
            #[doc = "dimensions based off of the op_delta events."]
            #[doc = ""]
            #[doc = "This request is ignored if an operation is already in progress."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn op_start_pointer(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_seat_v1#{}.op_start_pointer()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "End an interactive operation."]
            #[doc = ""]
            #[doc = "This request is ignored if there is no operation in progress."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn op_end(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_seat_v1#{}.op_end()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Define a pointer binding in terms of a pointer button, modifiers, and"]
            #[doc = "other configurable properties."]
            #[doc = ""]
            #[doc = "The button argument is a Linux input event code defined in the"]
            #[doc = "linux/input-event-codes.h header file (e.g. BTN_RIGHT)."]
            #[doc = ""]
            #[doc = "The new pointer binding is not enabled until initial configuration is"]
            #[doc = "completed and the enable request is made during a manage sequence."]
            fn get_pointer_binding(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                button: u32,
                modifiers: Modifiers,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_seat_v1#{}.get_pointer_binding({}, {}, {})",
                        sender_id,
                        id,
                        button,
                        modifiers
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_uint(button)
                        .put_uint(modifiers.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the XCursor theme for the seat. This theme is used for cursors"]
            #[doc = "rendered by the compositor, but not necessarily for cursors rendered by"]
            #[doc = "clients."]
            #[doc = ""]
            #[doc = "Note: The window manager may also wish to set the XCURSOR_THEME and"]
            #[doc = "XCURSOR_SIZE environment variable for programs it starts."]
            fn set_xcursor_theme(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
                size: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_seat_v1#{}.set_xcursor_theme(\"{}\", {})",
                        sender_id,
                        name,
                        size
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .put_uint(size)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Warp the pointer to the given position in the compositor's logical"]
            #[doc = "coordinate space."]
            #[doc = ""]
            #[doc = "If the given position is outside the bounds of all outputs, the pointer"]
            #[doc = "will be warped to the closest point inside an output instead."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn pointer_warp(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_seat_v1#{}.pointer_warp({}, {})", sender_id, x, y);
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_int(x).put_int(y).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 8u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that seat is no longer in use and should be"]
            #[doc = "destroyed."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object and ignore any"]
            #[doc = "request (other than river_seat_v1.destroy) made after this event is"]
            #[doc = "sent.  The client should destroy this object with the"]
            #[doc = "river_seat_v1.destroy request to free up resources."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The wl_seat object corresponding to the river_seat_v1. The argument is"]
            #[doc = "the global name of the wl_seat advertised with wl_registry.global."]
            #[doc = ""]
            #[doc = "It is guaranteed that the corresponding wl_seat is advertised before"]
            #[doc = "this event is sent."]
            #[doc = ""]
            #[doc = "This event is sent exactly once. The wl_seat associated with a"]
            #[doc = "river_seat_v1 cannot change. It is guaranteed that there is a 1-to-1"]
            #[doc = "mapping between wl_seat and river_seat_v1 objects."]
            #[doc = ""]
            #[doc = "The global_remove event for the corresponding wl_seat may be sent before"]
            #[doc = "the river_seat_v1.remove event. This is due to the fact that"]
            #[doc = "river_seat_v1 state changes are synced to the river window management"]
            #[doc = "manage sequence while changes to globals are not."]
            #[doc = ""]
            #[doc = "Rationale: The window manager may want to trigger window management"]
            #[doc = "state changes based on normal input events received by its shell"]
            #[doc = "surfaces for example."]
            fn wl_seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The seat's pointer entered the given window's area."]
            #[doc = ""]
            #[doc = "The area of a window is defined to include the area defined by the"]
            #[doc = "window dimensions, borders configured using river_window_v1.set_borders,"]
            #[doc = "and the input regions of decoration surfaces. In particular, it does not"]
            #[doc = "include input regions of surfaces belonging to the window that extend"]
            #[doc = "outside the window dimensions."]
            #[doc = ""]
            #[doc = "The pointer of a seat may only enter a single window at a time. When the"]
            #[doc = "pointer moves between windows, the pointer_leave event for the old"]
            #[doc = "window must be sent before the pointer_enter event for the new window."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn pointer_enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                window: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The seat's pointer left the window for which pointer_enter was most"]
            #[doc = "recently sent. See pointer_enter for details."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn pointer_leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A window has been interacted with beyond the pointer merely passing over"]
            #[doc = "it. This event might be sent due to a pointer button press or due to a"]
            #[doc = "touch/tablet tool interaction with the window."]
            #[doc = ""]
            #[doc = "There are no guarantees regarding how this event is sent in relation to"]
            #[doc = "the pointer_enter and pointer_leave events as the interaction may use"]
            #[doc = "touch or tablet tool input."]
            #[doc = ""]
            #[doc = "Rationale: this event gives window managers necessary information to"]
            #[doc = "determine when to send keyboard focus, raise a window that already has"]
            #[doc = "keyboard focus, etc. Rather than expose all pointer, touch, and tablet"]
            #[doc = "events to window managers, a policy over mechanism approach is taken."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn window_interaction(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                window: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A shell surface has been interacted with beyond the pointer merely"]
            #[doc = "passing over it. This event might be sent due to a pointer button press"]
            #[doc = "or due to a touch/tablet tool interaction with the shell_surface."]
            #[doc = ""]
            #[doc = "There are no guarantees regarding how this event is sent in relation to"]
            #[doc = "the pointer_enter and pointer_leave events as the interaction may use"]
            #[doc = "touch or tablet tool input."]
            #[doc = ""]
            #[doc = "Rationale: While the shell surface does receive all wl_pointer,"]
            #[doc = "wl_touch, etc. input events for the surface directly, these events do"]
            #[doc = "not necessarily trigger a manage sequence and therefore do not allow the"]
            #[doc = "window manager to update focus or perform other actions in response to"]
            #[doc = "the input in a race-free way."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn shell_surface_interaction(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                shell_surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates the total change in position since the start of the"]
            #[doc = "operation of the pointer/touch point/etc."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn op_delta(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                dx: i32,
                dy: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The input driving the current interactive operation has been released."]
            #[doc = "For a pointer op for example, all pointer buttons have been released."]
            #[doc = ""]
            #[doc = "Depending on the op type, op_delta events may continue to be sent until"]
            #[doc = "the op is ended with the op_end request."]
            #[doc = ""]
            #[doc = "This event is sent at most once during an interactive operation."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn op_release(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The current position of the pointer in the compositor's logical"]
            #[doc = "coordinate space."]
            #[doc = ""]
            #[doc = "This state is special in that a change in pointer position alone must"]
            #[doc = "not cause the compositor to start a manage sequence."]
            #[doc = ""]
            #[doc = "Assuming the seat has a pointer, this event must be sent in every manage"]
            #[doc = "sequence unless there is no change in x/y position since the last time this"]
            #[doc = "event was sent."]
            fn pointer_position(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_seat_v1#{}.removed()", sender_id,);
                            self.removed(connection, sender_id).await
                        }
                        1u16 => {
                            let name = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_seat_v1#{}.wl_seat({})", sender_id, name);
                            self.wl_seat(connection, sender_id, name).await
                        }
                        2u16 => {
                            let window = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_seat_v1#{}.pointer_enter({})",
                                sender_id,
                                window
                            );
                            self.pointer_enter(connection, sender_id, window).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_seat_v1#{}.pointer_leave()", sender_id,);
                            self.pointer_leave(connection, sender_id).await
                        }
                        4u16 => {
                            let window = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_seat_v1#{}.window_interaction({})",
                                sender_id,
                                window
                            );
                            self.window_interaction(connection, sender_id, window).await
                        }
                        5u16 => {
                            let shell_surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_seat_v1#{}.shell_surface_interaction({})",
                                sender_id,
                                shell_surface
                            );
                            self.shell_surface_interaction(connection, sender_id, shell_surface)
                                .await
                        }
                        6u16 => {
                            let dx = message.int()?;
                            let dy = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_seat_v1#{}.op_delta({}, {})", sender_id, dx, dy);
                            self.op_delta(connection, sender_id, dx, dy).await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_seat_v1#{}.op_release()", sender_id,);
                            self.op_release(connection, sender_id).await
                        }
                        8u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_seat_v1#{}.pointer_position({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.pointer_position(connection, sender_id, x, y).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This object allows the window manager to configure a pointer binding and"]
    #[doc = "receive events when the binding is triggered."]
    #[doc = ""]
    #[doc = "The new pointer binding is not enabled until the enable request is made"]
    #[doc = "during a manage sequence."]
    #[doc = ""]
    #[doc = "Normally, all pointer button events are sent to the surface with pointer"]
    #[doc = "focus by the compositor. Pointer button events that trigger a pointer"]
    #[doc = "binding are not sent to the surface with pointer focus."]
    #[doc = ""]
    #[doc = "If multiple pointer bindings would be triggered by a single physical"]
    #[doc = "pointer event on the compositor side, it is compositor policy which"]
    #[doc = "pointer binding(s) will receive press/release events or if all of the"]
    #[doc = "matched pointer bindings receive press/release events."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_pointer_binding_v1 {
        #[doc = "Trait to implement the river_pointer_binding_v1 interface. See the module level documentation for more info"]
        pub trait RiverPointerBindingV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_pointer_binding_v1";
            const VERSION: u32 = 3u32;
            #[doc = "This request indicates that the client will no longer use the pointer"]
            #[doc = "binding object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_pointer_binding_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request should be made after all initial configuration has been"]
            #[doc = "completed and the window manager wishes the pointer binding to be able"]
            #[doc = "to be triggered."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_pointer_binding_v1#{}.enable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request may be used to temporarily disable the pointer binding. It"]
            #[doc = "may be later re-enabled with the enable request."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_pointer_binding_v1#{}.disable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the pointer button triggering the binding has"]
            #[doc = "been pressed."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            #[doc = ""]
            #[doc = "The compositor should wait for the manage sequence to complete before"]
            #[doc = "processing further input events. This allows the window manager client"]
            #[doc = "to, for example, modify key bindings and keyboard focus without racing"]
            #[doc = "against future input events. The window manager should of course respond"]
            #[doc = "as soon as possible as the capacity of the compositor to buffer incoming"]
            #[doc = "input events is finite."]
            fn pressed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the pointer button triggering the binding has"]
            #[doc = "been released."]
            #[doc = ""]
            #[doc = "Releasing the modifiers for the binding without releasing the pointer"]
            #[doc = "button does not trigger the release event. This event is sent when the"]
            #[doc = "pointer button is released, even if the modifiers have changed since the"]
            #[doc = "pressed event."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            #[doc = ""]
            #[doc = "The compositor should wait for the manage sequence to complete before"]
            #[doc = "processing further input events. This allows the window manager client"]
            #[doc = "to, for example, modify key bindings and keyboard focus without racing"]
            #[doc = "against future input events. The window manager should of course respond"]
            #[doc = "as soon as possible as the capacity of the compositor to buffer incoming"]
            #[doc = "input events is finite."]
            fn released(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_pointer_binding_v1#{}.pressed()", sender_id,);
                            self.pressed(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_pointer_binding_v1#{}.released()", sender_id,);
                            self.released(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allows the river-window-management-v1 window manager to"]
#[doc = "define key bindings in terms of xkbcommon keysyms and other configurable"]
#[doc = "properties."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\", \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod river_xkb_bindings_v1 {
    #[doc = "This global interface should only be advertised to the client if the"]
    #[doc = "river_window_manager_v1 global is also advertised."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_xkb_bindings_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            ObjectAlreadyCreated = 0u32,
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
                    0u32 => Ok(Self::ObjectAlreadyCreated),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_xkb_bindings_v1 interface. See the module level documentation for more info"]
        pub trait RiverXkbBindingsV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_xkb_bindings_v1";
            const VERSION: u32 = 2u32;
            #[doc = "This request indicates that the client will no longer use the"]
            #[doc = "river_xkb_bindings_v1 object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_bindings_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Define a key binding for the given seat in terms of an xkbcommon keysym"]
            #[doc = "and other configurable properties."]
            #[doc = ""]
            #[doc = "The new key binding is not enabled until initial configuration is"]
            #[doc = "completed and the enable request is made during a manage sequence."]
            fn get_xkb_binding(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                id: waynest::ObjectId,
                keysym: u32,
                modifiers : super :: super :: super :: river :: river_window_management_v1 :: river_seat_v1 :: Modifiers,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_bindings_v1#{}.get_xkb_binding({}, {}, {}, {})",
                        sender_id,
                        seat,
                        id,
                        keysym,
                        modifiers
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(seat))
                        .put_object(Some(id))
                        .put_uint(keysym)
                        .put_uint(modifiers.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Create an object to manage seat-specific xkb bindings state."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request more than once for a given"]
            #[doc = "river_seat_v1 object."]
            fn get_seat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                seat: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_bindings_v1#{}.get_seat({}, {})",
                        sender_id,
                        id,
                        seat
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_object(Some(seat))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            fn handle_event(
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
    #[doc = "This object allows the window manager to configure a xkbcommon key binding"]
    #[doc = "and receive events when the key binding is triggered."]
    #[doc = ""]
    #[doc = "The new key binding is not enabled until the enable request is made during"]
    #[doc = "a manage sequence."]
    #[doc = ""]
    #[doc = "Normally, all key events are sent to the surface with keyboard focus by"]
    #[doc = "the compositor. Key events that trigger a key binding are not sent to the"]
    #[doc = "surface with keyboard focus."]
    #[doc = ""]
    #[doc = "If multiple key bindings would be triggered by a single physical key event"]
    #[doc = "on the compositor side, it is compositor policy which key binding(s) will"]
    #[doc = "receive press/release events or if all of the matched key bindings receive"]
    #[doc = "press/release events."]
    #[doc = ""]
    #[doc = "Key bindings might be matched by the same physical key event due to shared"]
    #[doc = "keysym and modifiers. The layout override feature may also cause the same"]
    #[doc = "physical key event to trigger two key bindings with different keysyms and"]
    #[doc = "different layout overrides configured."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_xkb_binding_v1 {
        #[doc = "Trait to implement the river_xkb_binding_v1 interface. See the module level documentation for more info"]
        pub trait RiverXkbBindingV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_xkb_binding_v1";
            const VERSION: u32 = 2u32;
            #[doc = "This request indicates that the client will no longer use the xkb key"]
            #[doc = "binding object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_binding_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Specify an xkb layout that should be used to translate key events for"]
            #[doc = "the purpose of triggering this key binding irrespective of the currently"]
            #[doc = "active xkb layout."]
            #[doc = ""]
            #[doc = "The layout argument is a 0-indexed xkbcommon layout number for the"]
            #[doc = "keyboard that generated the key event."]
            #[doc = ""]
            #[doc = "If this request is never made, the currently active xkb layout of the"]
            #[doc = "keyboard that generated the key event will be used."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn set_layout_override(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                layout: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_binding_v1#{}.set_layout_override({})",
                        sender_id,
                        layout
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(layout).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request should be made after all initial configuration has been"]
            #[doc = "completed and the window manager wishes the key binding to be able to be"]
            #[doc = "triggered."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_binding_v1#{}.enable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request may be used to temporarily disable the key binding. It may"]
            #[doc = "be later re-enabled with the enable request."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_binding_v1#{}.disable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the physical key triggering the binding has"]
            #[doc = "been pressed."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            #[doc = ""]
            #[doc = "The compositor should wait for the manage sequence to complete before"]
            #[doc = "processing further input events. This allows the window manager client"]
            #[doc = "to, for example, modify key bindings and keyboard focus without racing"]
            #[doc = "against future input events. The window manager should of course respond"]
            #[doc = "as soon as possible as the capacity of the compositor to buffer incoming"]
            #[doc = "input events is finite."]
            fn pressed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that the physical key triggering the binding has"]
            #[doc = "been released."]
            #[doc = ""]
            #[doc = "Releasing the modifiers for the binding without releasing the \"main\""]
            #[doc = "physical key that produces the bound keysym does not trigger the release"]
            #[doc = "event. This event is sent when the \"main\" key is released, even if the"]
            #[doc = "modifiers have changed since the pressed event."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            #[doc = ""]
            #[doc = "The compositor should wait for the manage sequence to complete before"]
            #[doc = "processing further input events. This allows the window manager client"]
            #[doc = "to, for example, modify key bindings and keyboard focus without racing"]
            #[doc = "against future input events. The window manager should of course respond"]
            #[doc = "as soon as possible as the capacity of the compositor to buffer incoming"]
            #[doc = "input events is finite."]
            fn released(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This event indicates that repeating should be stopped for the binding if"]
            #[doc = "the window manager has been repeating some action since the pressed"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "This event is generally sent when some other (possible unbound) key is"]
            #[doc = "pressed after the pressed event is sent and before the released event"]
            #[doc = "is sent for this binding."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn stop_repeat(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_xkb_binding_v1#{}.pressed()", sender_id,);
                            self.pressed(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_xkb_binding_v1#{}.released()", sender_id,);
                            self.released(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("river_xkb_binding_v1#{}.stop_repeat()", sender_id,);
                            self.stop_repeat(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This object manages xkb bindings state associated with a specific seat."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_xkb_bindings_seat_v1 {
        #[doc = "Trait to implement the river_xkb_bindings_seat_v1 interface. See the module level documentation for more info"]
        pub trait RiverXkbBindingsSeatV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_xkb_bindings_seat_v1";
            const VERSION: u32 = 2u32;
            #[doc = "This request indicates that the client will no longer use the object and"]
            #[doc = "that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_bindings_seat_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Ensure that the next non-modifier key press and corresponding release"]
            #[doc = "events for this seat are not sent to the currently focused surface."]
            #[doc = ""]
            #[doc = "If the next non-modifier key press triggers a binding, the"]
            #[doc = "pressed/released events are sent to the river_xkb_binding_v1 object as"]
            #[doc = "usual."]
            #[doc = ""]
            #[doc = "If the next non-modifier key press does not trigger a binding, the"]
            #[doc = "ate_unbound_key event is sent instead."]
            #[doc = ""]
            #[doc = "Rationale: the window manager may wish to implement \"chorded\""]
            #[doc = "keybindings where triggering a binding activates a \"submap\" with a"]
            #[doc = "different set of keybindings. Without a way to eat the next key"]
            #[doc = "press event, there is no good way for the window manager to know that it"]
            #[doc = "should error out and exit the submap when a key not bound in the submap"]
            #[doc = "is pressed."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn ensure_next_key_eaten(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_bindings_seat_v1#{}.ensure_next_key_eaten()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This requests cancels the effect of the latest ensure_next_key_eaten"]
            #[doc = "request if no key has been eaten due to the request yet. This request"]
            #[doc = "has no effect if a key has already been eaten or no"]
            #[doc = "ensure_next_key_eaten was made."]
            #[doc = ""]
            #[doc = "Rationale: the window manager may wish cancel an uncompleted \"chorded\""]
            #[doc = "keybinding after a timeout of a few seconds. Note that since this"]
            #[doc = "timeout use-case requires the window manager to trigger a manage sequence"]
            #[doc = "with the river_window_manager_v1.manage_dirty request it is possible that"]
            #[doc = "the ate_unbound_key key event may be sent before the window manager has"]
            #[doc = "a chance to make the cancel_ensure_next_key_eaten request."]
            #[doc = ""]
            #[doc = "This request modifies window management state and may only be made as"]
            #[doc = "part of a manage sequence, see the river_window_manager_v1 description."]
            fn cancel_ensure_next_key_eaten(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_bindings_seat_v1#{}.cancel_ensure_next_key_eaten()",
                        sender_id,
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "An unbound key press event was eaten due to the ensure_next_key_eaten"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "This event will be followed by a manage_start event after all other new"]
            #[doc = "state has been sent by the server."]
            fn ate_unbound_key(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                                "river_xkb_bindings_seat_v1#{}.ate_unbound_key()",
                                sender_id,
                            );
                            self.ate_unbound_key(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This protocol allow a client to set the xkbcommon keymap of individual"]
#[doc = "keyboard input devices. It also allows switching between the layouts of a"]
#[doc = "keymap and toggling capslock/numlock state."]
#[doc = ""]
#[doc = "The key words \"must\", \"must not\", \"required\", \"shall\", \"shall not\","]
#[doc = "\"should\", \"should not\", \"recommended\", \"may\", and \"optional\" in this"]
#[doc = "document are to be interpreted as described in IETF RFC 2119."]
#[allow(clippy::module_inception)]
pub mod river_xkb_config_v1 {
    #[doc = "Global interface for configuring xkb devices."]
    #[doc = ""]
    #[doc = "This global should only be advertised if river_input_manager_v1 is"]
    #[doc = "advertised as well."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_xkb_config_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            InvalidDestroy = 0u32,
            InvalidFormat = 1u32,
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
                    0u32 => Ok(Self::InvalidDestroy),
                    1u32 => Ok(Self::InvalidFormat),
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
        pub enum KeymapFormat {
            #[doc = "XKB_KEYMAP_FORMAT_TEXT_V1"]
            TextV1 = 1u32,
            #[doc = "XKB_KEYMAP_FORMAT_TEXT_V2"]
            TextV2 = 2u32,
        }
        impl From<KeymapFormat> for u32 {
            fn from(value: KeymapFormat) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for KeymapFormat {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::TextV1),
                    2u32 => Ok(Self::TextV2),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for KeymapFormat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_xkb_config_v1 interface. See the module level documentation for more info"]
        pub trait RiverXkbConfigV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_xkb_config_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client no longer wishes to receive"]
            #[doc = "events on this object."]
            #[doc = ""]
            #[doc = "The Wayland protocol is asynchronous, which means the server may send"]
            #[doc = "further events until the stop request is processed. The client must wait"]
            #[doc = "for a river_xkb_config_v1.finished event before destroying this object."]
            fn stop(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_config_v1#{}.stop()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This request should be called after the finished event has been received"]
            #[doc = "to complete destruction of the object."]
            #[doc = ""]
            #[doc = "It is a protocol error to make this request before the finished event"]
            #[doc = "has been received."]
            #[doc = ""]
            #[doc = "If a client wishes to destroy this object it should send a"]
            #[doc = "river_xkb_config_v1.stop request and wait for a"]
            #[doc = "river_xkb_config_v1.finished event. Once the finished event is received"]
            #[doc = "it is safe to destroy this object and any other objects created through"]
            #[doc = "this interface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_config_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The server must be able to mmap the fd with MAP_PRIVATE."]
            #[doc = "The server will fstat the fd to obtain the size of the keymap."]
            #[doc = "The client must not modify the contents of the fd after making this request."]
            #[doc = "The client should seal the fd with fcntl."]
            fn create_keymap(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                fd: std::os::fd::BorrowedFd,
                format: KeymapFormat,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_config_v1#{}.create_keymap({}, {}, {})",
                        sender_id,
                        id,
                        std::os::fd::AsRawFd::as_raw_fd(&fd),
                        format
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(id))
                        .put_fd(fd)
                        .put_uint(format.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the server will send no further events on this"]
            #[doc = "object. The client should destroy the object. See"]
            #[doc = "river_xkb_config_v1.destroy for more information."]
            fn finished(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "A new xkbcommon keyboard has been created. Not every"]
            #[doc = "river_input_device_v1 is necessarily an xkbcommon keyboard as well."]
            fn xkb_keyboard(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_xkb_config_v1#{}.finished()", sender_id,);
                            self.finished(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_config_v1#{}.xkb_keyboard({})",
                                sender_id,
                                id
                            );
                            self.xkb_keyboard(connection, sender_id, id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This object is the result of attempting to create an xkbcommon keymap."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_xkb_keymap_v1 {
        #[doc = "Trait to implement the river_xkb_keymap_v1 interface. See the module level documentation for more info"]
        pub trait RiverXkbKeymapV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_xkb_keymap_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the keymap"]
            #[doc = "object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_keymap_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The keymap object was successfully created and may be used with the"]
            #[doc = "river_xkb_keyboard_v1.set_keymap request."]
            fn success(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The compositor failed to create a keymap from the given parameters."]
            #[doc = ""]
            #[doc = "It is a protocol error to use this keymap object with"]
            #[doc = "river_xkb_keyboard_v1.set_keymap."]
            fn failure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                error_msg: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_xkb_keymap_v1#{}.success()", sender_id,);
                            self.success(connection, sender_id).await
                        }
                        1u16 => {
                            let error_msg = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keymap_v1#{}.failure(\"{}\")",
                                sender_id,
                                error_msg
                            );
                            self.failure(connection, sender_id, error_msg).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "This object represent a physical keyboard which has its configuration and"]
    #[doc = "state managed by xkbcommon."]
    #[allow(clippy::too_many_arguments)]
    pub mod river_xkb_keyboard_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            InvalidKeymap = 0u32,
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
                    0u32 => Ok(Self::InvalidKeymap),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the river_xkb_keyboard_v1 interface. See the module level documentation for more info"]
        pub trait RiverXkbKeyboardV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "river_xkb_keyboard_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This request indicates that the client will no longer use the keyboard"]
            #[doc = "object and that it may be safely destroyed."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_keyboard_v1#{}.destroy()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the keymap for the keyboard."]
            #[doc = ""]
            #[doc = "It is a protocol error to pass a keymap object for which the"]
            #[doc = "river_xkb_keymap_v1.success event was not received."]
            fn set_keymap(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                keymap: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_keyboard_v1#{}.set_keymap({})",
                        sender_id,
                        keymap
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_object(Some(keymap))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the active layout for the keyboard's keymap. Has no effect if the"]
            #[doc = "layout index is out of bounds for the current keymap."]
            fn set_layout_by_index(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_keyboard_v1#{}.set_layout_by_index({})",
                        sender_id,
                        index
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_int(index).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Set the active layout for the keyboard's keymap. Has no effect if there"]
            #[doc = "is no layout with the give name for the keyboard's keymap."]
            fn set_layout_by_name(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> river_xkb_keyboard_v1#{}.set_layout_by_name(\"{}\")",
                        sender_id,
                        name
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(name))
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Enable capslock for the keyboard."]
            fn capslock_enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_keyboard_v1#{}.capslock_enable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Disable capslock for the keyboard."]
            fn capslock_disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_keyboard_v1#{}.capslock_disable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Enable numlock for the keyboard."]
            fn numlock_enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_keyboard_v1#{}.numlock_enable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 6u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Disable numlock for the keyboard."]
            fn numlock_disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> river_xkb_keyboard_v1#{}.numlock_disable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 7u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "This event indicates that the xkb keyboard has been removed."]
            #[doc = ""]
            #[doc = "The server will send no further events on this object and ignore any"]
            #[doc = "request (other than river_xkb_keyboard_v1.destroy) made after this event"]
            #[doc = "is sent. The client should destroy this object with the"]
            #[doc = "river_xkb_keyboard_v1.destroy request to free up resources."]
            fn removed(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The river_input_device_v1 corresponding to this xkb keyboard. This event"]
            #[doc = "will always be the first event sent on the river_xkb_keyboard_v1 object,"]
            #[doc = "and it will be sent exactly once."]
            fn input_device(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                device: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The currently active layout index and name. The name arg may be null if"]
            #[doc = "the active layout does not have a name."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_xkb_keyboard_v1 is created and"]
            #[doc = "again whenever the layout changes."]
            fn layout(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                index: u32,
                name: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Capslock is currently enabled for the keyboard."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_xkb_keyboard_v1 is created and"]
            #[doc = "again whenever the capslock state changes."]
            fn capslock_enabled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Capslock is currently disabled for the keyboard."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_xkb_keyboard_v1 is created and"]
            #[doc = "again whenever the capslock state changes."]
            fn capslock_disabled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Numlock is currently enabled for the keyboard."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_xkb_keyboard_v1 is created and"]
            #[doc = "again whenever the numlock state changes."]
            fn numlock_enabled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Numlock is currently disabled for the keyboard."]
            #[doc = ""]
            #[doc = "This event is sent once when the river_xkb_keyboard_v1 is created and"]
            #[doc = "again whenever the numlock state changes."]
            fn numlock_disabled(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            fn handle_event(
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
                            tracing::debug!("river_xkb_keyboard_v1#{}.removed()", sender_id,);
                            self.removed(connection, sender_id).await
                        }
                        1u16 => {
                            let device = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keyboard_v1#{}.input_device({})",
                                sender_id,
                                device
                            );
                            self.input_device(connection, sender_id, device).await
                        }
                        2u16 => {
                            let index = message.uint()?;
                            let name = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keyboard_v1#{}.layout({}, \"{}\")",
                                sender_id,
                                index,
                                name.as_ref().map_or("null".to_string(), |v| v.to_string())
                            );
                            self.layout(connection, sender_id, index, name).await
                        }
                        3u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keyboard_v1#{}.capslock_enabled()",
                                sender_id,
                            );
                            self.capslock_enabled(connection, sender_id).await
                        }
                        4u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keyboard_v1#{}.capslock_disabled()",
                                sender_id,
                            );
                            self.capslock_disabled(connection, sender_id).await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keyboard_v1#{}.numlock_enabled()",
                                sender_id,
                            );
                            self.numlock_enabled(connection, sender_id).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "river_xkb_keyboard_v1#{}.numlock_disabled()",
                                sender_id,
                            );
                            self.numlock_disabled(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
