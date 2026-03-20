#[doc = "This protocol allows applications to act as input methods for compositors."]
#[doc = ""]
#[doc = "An input method context is used to manage the state of the input method."]
#[doc = ""]
#[doc = "Text strings are UTF-8 encoded, their indices and lengths are in bytes."]
#[doc = ""]
#[doc = "This document adheres to the RFC 2119 when using words like \"must\","]
#[doc = "\"should\", \"may\", etc."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the"]
#[doc = "experimental phase. Backwards incompatible major versions of the"]
#[doc = "protocol are to be expected. Exposing this protocol without an opt-in"]
#[doc = "mechanism is discouraged."]
#[allow(clippy::module_inception)]
pub mod input_method_experimental_v2 {
    #[doc = "An input method object allows for clients to compose text."]
    #[doc = ""]
    #[doc = "The objects connects the client to a text input in an application, and"]
    #[doc = "lets the client to serve as an input method for a seat."]
    #[doc = ""]
    #[doc = "The xx_input_method_v1 object can occupy two distinct states: active and"]
    #[doc = "inactive. In the active state, the object is associated to and"]
    #[doc = "communicates with a text input. In the inactive state, there is no"]
    #[doc = "associated text input, and the only communication is with the compositor."]
    #[doc = "Initially, the input method is in the inactive state."]
    #[doc = ""]
    #[doc = "Requests issued in the inactive state must be accepted by the compositor."]
    #[doc = "Because of the serial mechanism, and the state reset on activate event,"]
    #[doc = "they will not have any effect on the state of the next text input."]
    #[doc = ""]
    #[doc = "There must be no more than one input method object per seat."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_input_method_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "surface already has a role"]
            SurfaceHasRole = 0u32,
            #[doc = "operation requires the input method to be active"]
            Inactive = 1u32,
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
                    0u32 => Ok(Self::SurfaceHasRole),
                    1u32 => Ok(Self::Inactive),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_input_method_v1 interface. See the module level documentation for more info"]
        pub trait XxInputMethodV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_input_method_v1";
            const VERSION: u32 = 2u32;
            #[doc = "Send the commit string text for insertion to the application."]
            #[doc = ""]
            #[doc = "Inserts a string at current cursor position (see commit event"]
            #[doc = "sequence). The string to commit could be either just a single character"]
            #[doc = "after a key press or the result of some composing."]
            #[doc = ""]
            #[doc = "The argument text is a buffer containing the string to insert. There is"]
            #[doc = "a maximum length of wayland messages, so text can not be longer than"]
            #[doc = "4000 bytes."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next .commit request."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string."]
            fn commit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Send the pre-edit string text to the application text input."]
            #[doc = ""]
            #[doc = "Place a new composing text (pre-edit) at the current cursor position."]
            #[doc = "Any previously set composing text must be removed. Any previously"]
            #[doc = "existing selected text must be removed. The cursor is moved to a new"]
            #[doc = "position within the preedit string."]
            #[doc = ""]
            #[doc = "The argument text is a buffer containing the preedit string. There is"]
            #[doc = "a maximum length of wayland messages, so text can not be longer than"]
            #[doc = "4000 bytes."]
            #[doc = ""]
            #[doc = "The arguments cursor_begin and cursor_end are counted in bytes relative"]
            #[doc = "to the beginning of the submitted string buffer. Cursor should be"]
            #[doc = "hidden by the text input when both are equal to -1."]
            #[doc = ""]
            #[doc = "cursor_begin indicates the beginning of the cursor. cursor_end"]
            #[doc = "indicates the end of the cursor. It may be equal or different than"]
            #[doc = "cursor_begin."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They must be applied on"]
            #[doc = "the next xx_input_method_v1.commit request."]
            #[doc = "They must be reset to initial on the next committed disable event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string. The initial value of"]
            #[doc = "cursor_begin, and cursor_end are both 0."]
            fn set_preedit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                cursor_begin: i32,
                cursor_end: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove the surrounding text."]
            #[doc = ""]
            #[doc = "before_length and after_length are the number of bytes before and after"]
            #[doc = "the current cursor index (excluding the preedit text) to delete."]
            #[doc = ""]
            #[doc = "If any preedit text is present, it is replaced with the cursor for the"]
            #[doc = "purpose of this event. In effect before_length is counted from the"]
            #[doc = "beginning of preedit text, and after_length from its end (see commit"]
            #[doc = "event sequence)."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next xx_input_method_v1.commit request."]
            #[doc = ""]
            #[doc = "The initial values of both before_length and after_length are 0."]
            fn delete_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                before_length: u32,
                after_length: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Apply state changes from commit_string, set_preedit_string and"]
            #[doc = "delete_surrounding_text requests."]
            #[doc = ""]
            #[doc = "The state relating to these events is double-buffered, and each one"]
            #[doc = "modifies the pending state. This request replaces the current state"]
            #[doc = "with the pending state."]
            #[doc = ""]
            #[doc = "The connected text input is expected to proceed by evaluating the"]
            #[doc = "changes in the following order:"]
            #[doc = ""]
            #[doc = "1. Replace existing preedit string with the cursor."]
            #[doc = "2. Delete requested surrounding text."]
            #[doc = "3. Insert commit string with the cursor at its end."]
            #[doc = "4. Calculate surrounding text to send."]
            #[doc = "5. Insert new preedit text in cursor position."]
            #[doc = "6. Place cursor inside preedit text."]
            #[doc = ""]
            #[doc = "The serial number reflects the last state of the xx_input_method_v1"]
            #[doc = "object known to the client. The value of the serial argument must be"]
            #[doc = "equal to the number of done events already issued by that object. When"]
            #[doc = "the compositor receives a commit request with a serial different than"]
            #[doc = "the number of past done events, it must proceed as normal, except it"]
            #[doc = "should not change the current state of the xx_input_method_v1 object."]
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Creates a new xx_input_popup_surface_v2 object wrapping a given"]
            #[doc = "surface."]
            #[doc = ""]
            #[doc = "The surface gets assigned the \"input_popup\" role. If the surface"]
            #[doc = "already has an assigned role, the compositor must issue a protocol"]
            #[doc = "error."]
            #[doc = ""]
            #[doc = "Issuing this request before receiving a committed .activate causes the \"inactive\" error."]
            fn get_input_popup_surface(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                surface: waynest::ObjectId,
                positioner: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroys the xx_input_method_v1 object and any associated child"]
            #[doc = "objects."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notification that a text input focused on this seat requested the input"]
            #[doc = "method to be activated."]
            #[doc = ""]
            #[doc = "This event serves the purpose of providing the compositor with an"]
            #[doc = "active input method."]
            #[doc = ""]
            #[doc = "This event resets all state associated with previous"]
            #[doc = "surrounding_text, text_change_cause, and content_type events, as well"]
            #[doc = "as the state associated with set_preedit_string, commit_string, and"]
            #[doc = "delete_surrounding_text requests, and destroys any existing input_popup_surface objects."]
            #[doc = "In addition, it marks the xx_input_method_v1 object as active."]
            #[doc = ""]
            #[doc = "The surrounding_text, and content_type events must follow before the"]
            #[doc = "next done event if the text input supports the respective"]
            #[doc = "functionality."]
            #[doc = ""]
            #[doc = "State set with this event is double-buffered. It will get applied on"]
            #[doc = "the next xx_input_method_v1.done event, and stay valid until changed."]
            fn activate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_input_method_v1#{}.activate()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notification that no focused text input currently needs an active"]
            #[doc = "input method on this seat."]
            #[doc = ""]
            #[doc = "This event marks the xx_input_method_v1 object as inactive."]
            #[doc = "compositor must destroy all existing xx_input_popup_surface_v2 objects."]
            #[doc = ""]
            #[doc = "This event resets all state associated with previous"]
            #[doc = "surrounding_text, text_change_cause, and content_type events, as well"]
            #[doc = "as the state associated with set_preedit_string, commit_string, and"]
            #[doc = "delete_surrounding_text requests."]
            #[doc = ""]
            #[doc = "State set with this event is double-buffered. It will get applied on"]
            #[doc = "the next xx_input_method_v1.done event, and stay valid until changed."]
            fn deactivate(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_input_method_v1#{}.deactivate()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Updates the surrounding plain text around the cursor, excluding the"]
            #[doc = "preedit text."]
            #[doc = ""]
            #[doc = "If any preedit text is present, it is replaced with the cursor for the"]
            #[doc = "purpose of this event."]
            #[doc = ""]
            #[doc = "The argument text is a buffer containing the preedit string, and must"]
            #[doc = "include the cursor position, and the complete selection. It should"]
            #[doc = "contain additional characters before and after these. There is a"]
            #[doc = "maximum length of wayland messages, so text can not be longer than 4000"]
            #[doc = "bytes."]
            #[doc = ""]
            #[doc = "cursor is the byte offset of the cursor within the text buffer."]
            #[doc = ""]
            #[doc = "anchor is the byte offset of the selection anchor within the text"]
            #[doc = "buffer. If there is no selected text, anchor must be the same as"]
            #[doc = "cursor."]
            #[doc = ""]
            #[doc = "If this event does not arrive before the first done event, the input"]
            #[doc = "method may assume that the text input does not support this"]
            #[doc = "functionality and ignore following surrounding_text events."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They will get applied"]
            #[doc = "and set to initial values on the next xx_input_method_v1.done"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The initial state for affected fields is empty, meaning that the text"]
            #[doc = "input does not support sending surrounding text. If the empty values"]
            #[doc = "get applied, subsequent attempts to change them may have no effect."]
            fn surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                cursor: u32,
                anchor: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_input_method_v1#{}.surrounding_text(\"{}\", {}, {})",
                        sender_id,
                        text,
                        cursor,
                        anchor
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(Some(text))
                        .put_uint(cursor)
                        .put_uint(anchor)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Tells the input method why the text surrounding the cursor changed."]
            #[doc = ""]
            #[doc = "Whenever the client detects an external change in text, cursor, or"]
            #[doc = "anchor position, it must issue this request to the compositor. This"]
            #[doc = "request is intended to give the input method a chance to update the"]
            #[doc = "preedit text in an appropriate way, e.g. by removing it when the user"]
            #[doc = "starts typing with a keyboard."]
            #[doc = ""]
            #[doc = "cause describes the source of the change."]
            #[doc = ""]
            #[doc = "The value set with this event is double-buffered. It will get applied"]
            #[doc = "and set to its initial value on the next xx_input_method_v1.done"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "The initial value of cause is input_method."]
            fn text_change_cause(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                cause : super :: super :: super :: unstable :: text_input_unstable_v3 :: zwp_text_input_v3 :: ChangeCause,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_input_method_v1#{}.text_change_cause({})",
                        sender_id,
                        cause
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(cause.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Indicates the content type and hint for the current"]
            #[doc = "xx_input_method_v1 instance."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They will get applied"]
            #[doc = "on the next xx_input_method_v1.done event."]
            #[doc = "They get reset to initial on the next committed deactivate event."]
            #[doc = ""]
            #[doc = "The initial value for hint is none, and the initial value for purpose"]
            #[doc = "is normal."]
            fn content_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint : super :: super :: super :: unstable :: text_input_unstable_v3 :: zwp_text_input_v3 :: ContentHint,
                purpose : super :: super :: super :: unstable :: text_input_unstable_v3 :: zwp_text_input_v3 :: ContentPurpose,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_input_method_v1#{}.content_type({}, {})",
                        sender_id,
                        hint,
                        purpose
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(hint.into())
                        .put_uint(purpose.into())
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Atomically applies state changes recently sent to the client."]
            #[doc = ""]
            #[doc = "The done event establishes and updates the state of the client, and"]
            #[doc = "must be issued after any changes to apply them."]
            #[doc = ""]
            #[doc = "Text input state (content purpose, content hint, surrounding text, and"]
            #[doc = "change cause) is conceptually double-buffered within an input method"]
            #[doc = "context."]
            #[doc = ""]
            #[doc = "Events modify the pending state, as opposed to the current state in use"]
            #[doc = "by the input method. A done event atomically applies all pending state,"]
            #[doc = "replacing the current state. After done, the new pending state is as"]
            #[doc = "documented for each related request."]
            #[doc = ""]
            #[doc = "Events must be applied in the order of arrival."]
            #[doc = ""]
            #[doc = "Neither current nor pending state are modified unless noted otherwise."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_input_method_v1#{}.done()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 5u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The input method ceased to be available."]
            #[doc = ""]
            #[doc = "The compositor must issue this event as the only event on the object if"]
            #[doc = "there was another input_method object associated with the same seat at"]
            #[doc = "the time of its creation."]
            #[doc = ""]
            #[doc = "The compositor must issue this request when the object is no longer"]
            #[doc = "useable, e.g. due to seat removal."]
            #[doc = ""]
            #[doc = "The input method context becomes inert and should be destroyed after"]
            #[doc = "deactivation is handled. Any further requests and events except for the"]
            #[doc = "destroy request must be ignored."]
            fn unavailable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_input_method_v1#{}.unavailable()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
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
                            let text = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_method_v1#{}.commit_string(\"{}\")",
                                sender_id,
                                text
                            );
                            self.commit_string(connection, sender_id, text).await
                        }
                        1u16 => {
                            let text = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let cursor_begin = message.int()?;
                            let cursor_end = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_method_v1#{}.set_preedit_string(\"{}\", {}, {})",
                                sender_id,
                                text,
                                cursor_begin,
                                cursor_end
                            );
                            self.set_preedit_string(
                                connection,
                                sender_id,
                                text,
                                cursor_begin,
                                cursor_end,
                            )
                            .await
                        }
                        2u16 => {
                            let before_length = message.uint()?;
                            let after_length = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_method_v1#{}.delete_surrounding_text({}, {})",
                                sender_id,
                                before_length,
                                after_length
                            );
                            self.delete_surrounding_text(
                                connection,
                                sender_id,
                                before_length,
                                after_length,
                            )
                            .await
                        }
                        3u16 => {
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_input_method_v1#{}.commit({})", sender_id, serial);
                            self.commit(connection, sender_id, serial).await
                        }
                        4u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let surface = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let positioner = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_method_v1#{}.get_input_popup_surface({}, {}, {})",
                                sender_id,
                                id,
                                surface,
                                positioner
                            );
                            self.get_input_popup_surface(
                                connection, sender_id, id, surface, positioner,
                            )
                            .await
                        }
                        5u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_input_method_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "An input method popup surface is a short-lived, temporary surface."]
    #[doc = "It is meant as an area to show suggestions, candidates, or for other input-related uses."]
    #[doc = ""]
    #[doc = "The compositor should anchor it at the active text input cursor area."]
    #[doc = ""]
    #[doc = "The client must call wl_surface.commit on the corresponding wl_surface"]
    #[doc = "for input_popup_surface state updates to take effect, unless otherwise noted."]
    #[doc = ""]
    #[doc = "After the initial wl_surface.commit, the compositor must reply with a configure sequence (see .start_configure) initializing all the compositor-provided state of the popup. That means providing values for:"]
    #[doc = ""]
    #[doc = "- width"]
    #[doc = "- height"]
    #[doc = "- anchor_x"]
    #[doc = "- anchor_y"]
    #[doc = "- anchor_width"]
    #[doc = "- anchor_height"]
    #[doc = "- serial"]
    #[doc = ""]
    #[doc = "using the appropriate events."]
    #[doc = ""]
    #[doc = "The popup will only be presented to the user after the client receives the configure sequence and replies with .ack_configure."]
    #[doc = ""]
    #[doc = "An example init sequence could look like this:"]
    #[doc = ""]
    #[doc = "1. client (Cl): popup = input_method.get_popup(wl_surface, positioner)"]
    #[doc = "2. Cl: wl_surface.commit()"]
    #[doc = "3. compositor (Co): popup.start_configure(150, 150, 10, -2, 5, 30)"]
    #[doc = "5. Co: input_method.done()"]
    #[doc = "6. Cl: ack_configure()"]
    #[doc = "7. Cl: wl_surface.commit()"]
    #[doc = ""]
    #[doc = "A newly created input_popup_surface will be stacked on top of all previously created"]
    #[doc = "input_popup_surfaces associated with the same text input."]
    #[doc = ""]
    #[doc = "A typical sequence resulting from the user selecting a new text field and typing some text:"]
    #[doc = ""]
    #[doc = "1. compositor (Co): input_method.enable()"]
    #[doc = "2. Co: input_method.done()"]
    #[doc = "3. [init sequence]"]
    #[doc = "4. Co: input_method.set_surrounding_text(\"new text\")"]
    #[doc = "5. Co: popup.start_configure(150, 150, -60, -2, 55, 30)"]
    #[doc = "6. Co: input_method.done()"]
    #[doc = "7. client (Cl): ack_configure()"]
    #[doc = "8. Cl: wl_surface.commit()"]
    #[doc = ""]
    #[doc = "When the corresponding input_method receives a commited .disable event, the popup gets destroyed and becomes invalid and its surface gets unmapped."]
    #[doc = ""]
    #[doc = "The client must not destroy the underlying wl_surface while the"]
    #[doc = "xx_input_popup_surface_v2 object exists."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_input_popup_surface_v2 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "received acknowledgement for a serial which has already been acknowledged or has never been issued"]
            InvalidSerial = 0u32,
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
                    0u32 => Ok(Self::InvalidSerial),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_input_popup_surface_v2 interface. See the module level documentation for more info"]
        pub trait XxInputPopupSurfaceV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_input_popup_surface_v2";
            const VERSION: u32 = 1u32;
            #[doc = "This request notifies the compositor that the client updated its surface in response to a configure sequence."]
            #[doc = ""]
            #[doc = "The purpose of this request is to synchronize the updates of the surface geometry with the surface contents."]
            #[doc = "For example, when the compositor assigns a size larger than prevously, the client must fill the additional space before the popup gets displayed to the user with the new size. When the compositor receives .ack_configure, it can proceed to draw the new size."]
            #[doc = ""]
            #[doc = ".ack_configure should be sent after every submitted configure sequence, passing along the serial received in it."]
            #[doc = ""]
            #[doc = "An .ack_configure request is conceptually double-buffered."]
            #[doc = "Every request overrides the previous one. The request takes effect once the .commit request is sent on the corresponding surface."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure sequences before it"]
            #[doc = "can respond to one, it may acknowledge only the last configure sequence by using its serial in the .ack_configure request."]
            #[doc = ""]
            #[doc = "Committing an .ack_configure request consumes the serial number sent with"]
            #[doc = "the request, as well as serial numbers sent by all configure sequences"]
            #[doc = "submitted on this input_popup_surface prior to the configure sequence referenced by"]
            #[doc = "the committed serial."]
            #[doc = ""]
            #[doc = "Committing this request with a serial that, for this surface, never appeared in a submitted configure sequence, or one that was already committed before, raises an invalid_serial"]
            #[doc = "error."]
            fn ack_configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Reposition an already-mapped popup. The popup will be placed given the"]
            #[doc = "details in the passed input_popup_positioner object."]
            #[doc = ""]
            #[doc = "The request is processed immediately, without the need to issue wl_surface.commit, but the actual repositioning takes place later, after .ack_configure."]
            #[doc = ""]
            #[doc = "The compositor should reply with a configure sequence including:"]
            #[doc = "- input_popup_surface.start_configure,"]
            #[doc = "- input_popup_surface.repositioned, including the token passed in this request."]
            #[doc = ""]
            #[doc = "This will discard any parameters set by the previous positioner."]
            #[doc = ""]
            #[doc = "If multiple .reposition requests are sent before the .repositioned event is submitted as part of a configure sequence, the compositor may ignore all"]
            #[doc = "but the last one."]
            #[doc = ""]
            #[doc = "The new popup position will not take"]
            #[doc = "effect until the corresponding configure sequence is acknowledged by the"]
            #[doc = "client. See input_popup_surface.repositioned for details."]
            #[doc = ""]
            #[doc = "The token itself is opaque, and has no other special meaning."]
            fn reposition(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                positioner: waynest::ObjectId,
                token: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "This destroys the popup. Explicitly destroying the input_popup_surface"]
            #[doc = "object will also dismiss the popup, and unmap the surface."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The start_configure event updates the popup geometry and marks the start of a configure sequence."]
            #[doc = ""]
            #[doc = "The anchor_* arguments represent the geometry of the anchor to which the popup was attached, relative to the upper left corner of the"]
            #[doc = "popup's surface. Note that this makes anchor_x, anchor_y the reverse of the what they represent in xdg_popup."]
            #[doc = ""]
            #[doc = "A configure sequence is a set of one or more events configuring the state of the"]
            #[doc = "input_popup_surface, starting with this event and ending with input_method.done. After the input_method.done event, the configure sequence is considered submitted."]
            #[doc = ""]
            #[doc = "State set by event in a configure sequence is conceptually double-buffered."]
            #[doc = "Every argument overwrites its previous value. The state change should get applied atomically with the input_method.done ending the sequence, and the value of serial should return to the undefined value."]
            #[doc = ""]
            #[doc = "Events on the input_popup_surface object received outside a configure sequence (while serial is undefined) must be ignored by the client."]
            #[doc = ""]
            #[doc = "A configure sequence shall be sent every time the compositor (re)positions the popup, or the shape of the anchor changes, for example after popup creation, or in response to text being typed and the text cursor moving."]
            #[doc = ""]
            #[doc = "The client may update the surface in response to input_method.done. Unless the popup is destroyed by the input_method.done, the client must reply with"]
            #[doc = "an .ack_configure request with the serial sent in the start_configure event at"]
            #[doc = "some point after the sequence ends and before committing the new surface."]
            #[doc = ""]
            #[doc = "If the client receives multiple configure sequences before it can respond"]
            #[doc = "to one, it is free to discard all but the last event it received."]
            #[doc = ""]
            #[doc = ""]
            fn start_configure(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: u32,
                height: u32,
                anchor_x: i32,
                anchor_y: i32,
                anchor_width: u32,
                anchor_height: u32,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_input_popup_surface_v2#{}.start_configure({}, {}, {}, {}, {}, {}, {})",
                        sender_id,
                        width,
                        height,
                        anchor_x,
                        anchor_y,
                        anchor_width,
                        anchor_height,
                        serial
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(width)
                        .put_uint(height)
                        .put_int(anchor_x)
                        .put_int(anchor_y)
                        .put_uint(anchor_width)
                        .put_uint(anchor_height)
                        .put_uint(serial)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "The compositor sends the .repositioned event in response to the .reposition request to notify about its completion."]
            #[doc = ""]
            #[doc = "The new geometry of the popup can be communicated using additional events within a configure sequence including:"]
            #[doc = "- input_popup_surface.start_configure, and"]
            #[doc = "- the .anchor_position event to update the relative position to the anchor."]
            #[doc = ""]
            #[doc = "When responding to a .reposition request, the token argument is the token passed in the that request."]
            #[doc = ""]
            #[doc = "This event is sent as part of a configure sequence."]
            #[doc = "State set by this event is conceptually double-buffered."]
            #[doc = "Every argument overwrites its previous value. The state change should get applied atomically with the next input_method.done event."]
            #[doc = ""]
            #[doc = "The client should optionally update the content of the popup, but must"]
            #[doc = "acknowledge the new popup configuration for the new position to take"]
            #[doc = "effect. See input_popup_surface.ack_configure for details."]
            fn repositioned(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                token: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_input_popup_surface_v2#{}.repositioned({})",
                        sender_id,
                        token
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(token).build();
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
                            let serial = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_surface_v2#{}.ack_configure({})",
                                sender_id,
                                serial
                            );
                            self.ack_configure(connection, sender_id, serial).await
                        }
                        1u16 => {
                            let positioner = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let token = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_surface_v2#{}.reposition({}, {})",
                                sender_id,
                                positioner,
                                token
                            );
                            self.reposition(connection, sender_id, positioner, token)
                                .await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_input_popup_surface_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The input_popup_positioner provides a collection of rules for the placement of an input method popup surface relative to the cursor."]
    #[doc = "Rules can be defined to ensure"]
    #[doc = "the text input area remains within the visible area's borders, and to"]
    #[doc = "specify how the popup changes its position, such as sliding along"]
    #[doc = "an axis, or flipping around a rectangle. These positioner-created rules are"]
    #[doc = "constrained by the requirement that a popup must intersect with or"]
    #[doc = "be at least partially adjacent to the surface containing the text input."]
    #[doc = ""]
    #[doc = "See the various requests for details about possible rules."]
    #[doc = ""]
    #[doc = "A newly created positioner has the following state:"]
    #[doc = "- 0 surface width"]
    #[doc = "- 0 surface height"]
    #[doc = "- anchor at the center (\"none\")"]
    #[doc = "- gravity towards the center (\"none\")"]
    #[doc = "- constraints adjustment set to none"]
    #[doc = "- offset at x = 0, y = 0"]
    #[doc = "- not reactive"]
    #[doc = ""]
    #[doc = "Upon receiving a request taking the positioner as an argument, the compositor makes a copy of the rules"]
    #[doc = "specified by the input_popup_positioner. Thus, after the request is complete the"]
    #[doc = "input_popup_positioner object can be destroyed or reused; further changes to the"]
    #[doc = "object will have no effect on previous usages."]
    #[doc = ""]
    #[doc = "For an input_popup_positioner object to be considered complete, its state must contain a non-zero width and height. Passing an incomplete input_popup_positioner object when"]
    #[doc = "positioning a surface raises an invalid_positioner error."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_input_popup_positioner_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "invalid input provided"]
            InvalidInput = 0u32,
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
                    0u32 => Ok(Self::InvalidInput),
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
        pub enum Anchor {
            #[doc = "no edge, specfies center"]
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
        impl From<Anchor> for u32 {
            fn from(value: Anchor) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Anchor {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Anchor {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Gravity {
            #[doc = "center to center"]
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
        impl From<Gravity> for u32 {
            fn from(value: Gravity) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Gravity {
            type Error = waynest::ProtocolError;
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
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Gravity {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "The constraint adjustment value define ways the compositor will adjust"] # [doc = "the position of the surface, if the unadjusted position would result"] # [doc = "in the surface being partly constrained."] # [doc = ""] # [doc = "Whether a surface is considered 'constrained' is left to the compositor"] # [doc = "to determine. For example, the surface may be partly outside the"] # [doc = "compositor's defined 'work area', thus necessitating the child surface's"] # [doc = "position be adjusted until it is entirely inside the work area."] # [doc = ""] # [doc = "The adjustments can be combined, according to a defined precedence: 1)"] # [doc = "Flip, 2) Slide, 3) Resize."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ConstraintAdjustment : u32 { const None = 0u32 ; const SlideX = 1u32 ; const SlideY = 2u32 ; const FlipX = 4u32 ; const FlipY = 8u32 ; const ResizeX = 16u32 ; const ResizeY = 32u32 ; } }
        impl From<ConstraintAdjustment> for u32 {
            fn from(value: ConstraintAdjustment) -> Self {
                value.bits()
            }
        }
        impl TryFrom<u32> for ConstraintAdjustment {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(waynest::ProtocolError::MalformedPayload)
            }
        }
        impl std::fmt::Display for ConstraintAdjustment {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.bits().fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_input_popup_positioner_v1 interface. See the module level documentation for more info"]
        pub trait XxInputPopupPositionerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_input_popup_positioner_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Notify the compositor that the positioner will no longer be used."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Set the size of the surface that is to be positioned with the positioner"]
            #[doc = "object. The size is in surface-local coordinates and corresponds to the"]
            #[doc = "window geometry. See xdg_surface.set_window_geometry."]
            #[doc = ""]
            #[doc = "If any dimension is set to zero, the invalid_input error is raised."]
            fn set_size(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                width: u32,
                height: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Defines the anchor point for the anchor rectangle. The specified anchor"]
            #[doc = "is used to derive an anchor point that the popup surface will be"]
            #[doc = "positioned relative to. If a corner anchor is set (e.g. 'top_left' or"]
            #[doc = "'bottom_right'), the anchor point will be at the specified corner;"]
            #[doc = "otherwise, the derived anchor point will be centered on the specified"]
            #[doc = "edge, or in the center of the anchor rectangle if no edge is specified."]
            fn set_anchor(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                anchor: Anchor,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Defines in what direction the surface should be positioned, relative to"]
            #[doc = "the anchor point of the anchor rectangle. If a corner gravity is"]
            #[doc = "specified (e.g. 'bottom_right' or 'top_left'), then the surface"]
            #[doc = "will be placed towards the specified gravity; otherwise, the child"]
            #[doc = "surface will be centered over the anchor point on any axis that had no"]
            #[doc = "gravity specified. If the gravity is not in the ‘gravity’ enum, an"]
            #[doc = "invalid_input error is raised."]
            fn set_gravity(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                gravity: Gravity,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Specify how the popup should be positioned if the originally intended"]
            #[doc = "position caused the surface to be constrained, meaning at least"]
            #[doc = "partially outside positioning boundaries set by the compositor. The"]
            #[doc = "adjustment is set by constructing a bitmask describing the adjustment to"]
            #[doc = "be made when the surface is constrained on that axis."]
            #[doc = ""]
            #[doc = "If no bit for one axis is set, the compositor will assume that the child"]
            #[doc = "surface should not change its position on that axis when constrained."]
            #[doc = ""]
            #[doc = "If more than one bit for one axis is set, the order of how adjustments"]
            #[doc = "are applied is specified in the corresponding adjustment descriptions."]
            #[doc = ""]
            #[doc = "The default adjustment is none."]
            fn set_constraint_adjustment(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                constraint_adjustment: ConstraintAdjustment,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Specify the surface position offset relative to the position of the"]
            #[doc = "anchor on the anchor rectangle and the anchor on the surface. For"]
            #[doc = "example if the anchor of the anchor rectangle is at (x, y), the surface"]
            #[doc = "has the gravity bottom|right, and the offset is (ox, oy), the calculated"]
            #[doc = "surface position will be (x + ox, y + oy). The offset position of the"]
            #[doc = "surface is the one used for constraint testing. See"]
            #[doc = "set_constraint_adjustment."]
            #[doc = ""]
            #[doc = "An example use case is placing a popup menu on top of a user interface"]
            #[doc = "element, while aligning the user interface element of the parent surface"]
            #[doc = "with some user interface element placed somewhere in the popup surface."]
            fn set_offset(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "When set reactive, the surface is reconstrained if the conditions used"]
            #[doc = "for constraining changed, e.g. the window containing the text input moved."]
            #[doc = ""]
            #[doc = "Whenever the conditions change and the popup gets reconstrained, a"]
            #[doc = "configure sequence is sent with updated geometry."]
            fn set_reactive(
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
                            tracing::debug!("xx_input_popup_positioner_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let width = message.uint()?;
                            let height = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_positioner_v1#{}.set_size({}, {})",
                                sender_id,
                                width,
                                height
                            );
                            self.set_size(connection, sender_id, width, height).await
                        }
                        2u16 => {
                            let anchor = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_positioner_v1#{}.set_anchor({})",
                                sender_id,
                                anchor
                            );
                            self.set_anchor(connection, sender_id, anchor.try_into()?)
                                .await
                        }
                        3u16 => {
                            let gravity = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_positioner_v1#{}.set_gravity({})",
                                sender_id,
                                gravity
                            );
                            self.set_gravity(connection, sender_id, gravity.try_into()?)
                                .await
                        }
                        4u16 => {
                            let constraint_adjustment = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_positioner_v1#{}.set_constraint_adjustment({})",
                                sender_id,
                                constraint_adjustment
                            );
                            self.set_constraint_adjustment(
                                connection,
                                sender_id,
                                constraint_adjustment.try_into()?,
                            )
                            .await
                        }
                        5u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_positioner_v1#{}.set_offset({}, {})",
                                sender_id,
                                x,
                                y
                            );
                            self.set_offset(connection, sender_id, x, y).await
                        }
                        6u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_popup_positioner_v1#{}.set_reactive()",
                                sender_id,
                            );
                            self.set_reactive(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "The input method manager allows the client to become the input method on"]
    #[doc = "a chosen seat."]
    #[doc = ""]
    #[doc = "No more than one input method must be associated with any seat at any"]
    #[doc = "given time."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_input_method_manager_v2 {
        #[doc = "Trait to implement the xx_input_method_manager_v2 interface. See the module level documentation for more info"]
        pub trait XxInputMethodManagerV2
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_input_method_manager_v2";
            const VERSION: u32 = 2u32;
            #[doc = "Request a new input xx_input_method_v1 object associated with a given"]
            #[doc = "seat."]
            fn get_input_method(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                seat: waynest::ObjectId,
                input_method: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a positioner object. A positioner object is used to position"]
            #[doc = "surfaces relative to some parent surface. See the interface description"]
            #[doc = "and xdg_surface.get_popup for details."]
            fn get_positioner(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Destroys the xx_input_method_manager_v2 object."]
            #[doc = ""]
            #[doc = "The xx_input_method_v1 objects originating from it remain valid."]
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
                            let seat = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let input_method = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_method_manager_v2#{}.get_input_method({}, {})",
                                sender_id,
                                seat,
                                input_method
                            );
                            self.get_input_method(connection, sender_id, seat, input_method)
                                .await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_input_method_manager_v2#{}.get_positioner({})",
                                sender_id,
                                id
                            );
                            self.get_positioner(connection, sender_id, id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_input_method_manager_v2#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
}
#[doc = "This description provides a high-level overview of the interplay between"]
#[doc = "the interfaces defined this protocol. For details, see the protocol"]
#[doc = "specification."]
#[doc = ""]
#[doc = "The xx_session_manager protocol declares interfaces necessary to"]
#[doc = "allow clients to restore toplevel state from previous executions. The"]
#[doc = "xx_session_manager_v1.get_session request can be used to obtain a"]
#[doc = "xx_session_v1 resource representing the state of a set of toplevels."]
#[doc = ""]
#[doc = "Clients may obtain the session string to use in future calls through"]
#[doc = "the xx_session_v1.created event. Compositors will use this string"]
#[doc = "as an identifiable token for future runs, possibly storing data about"]
#[doc = "the related toplevels in persistent storage."]
#[doc = ""]
#[doc = "Toplevels are managed through the xx_session_v1.add_toplevel and"]
#[doc = "xx_session_toplevel_v1.remove pair of requests. Clients will explicitly"]
#[doc = "request a toplevel to be restored according to prior state through the"]
#[doc = "xx_session_v1.restore_toplevel request before the toplevel is mapped."]
#[doc = ""]
#[doc = "Warning! The protocol described in this file is currently in the"]
#[doc = "experimental phase. Backwards incompatible major versions of the"]
#[doc = "protocol are to be expected. Exposing this protocol without an opt-in"]
#[doc = "mechanism is discouraged."]
#[allow(clippy::module_inception)]
pub mod xx_session_management_v1 {
    #[doc = "The xx_session_manager interface defines base requests for creating and"]
    #[doc = "managing a session for an application. Sessions persist across application"]
    #[doc = "and compositor restarts unless explicitly destroyed. A session is created"]
    #[doc = "for the purpose of maintaining an application's xdg_toplevel surfaces"]
    #[doc = "across compositor or application restarts. The compositor should remember"]
    #[doc = "as many states as possible for surfaces in a given session, but there is"]
    #[doc = "no requirement for which states must be remembered."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_session_manager_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "a requested session is already in use"]
            InUse = 1u32,
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
                    1u32 => Ok(Self::InUse),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "The reason may determine in what way a session restores the window"]
        #[doc = "management state of associated toplevels."]
        #[doc = ""]
        #[doc = "For example newly launched applications might be launched on the active"]
        #[doc = "workspace with restored size and position, while a recovered"]
        #[doc = "applications might restore additional state such as active workspace and"]
        #[doc = "stacking order."]
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Reason {
            Launch = 1u32,
            Recover = 2u32,
            SessionRestore = 3u32,
        }
        impl From<Reason> for u32 {
            fn from(value: Reason) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for Reason {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    1u32 => Ok(Self::Launch),
                    2u32 => Ok(Self::Recover),
                    3u32 => Ok(Self::SessionRestore),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Reason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_session_manager_v1 interface. See the module level documentation for more info"]
        pub trait XxSessionManagerV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_session_manager_v1";
            const VERSION: u32 = 1u32;
            #[doc = "This has no effect other than to destroy the xx_session_manager object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Create a session object corresponding to either an existing session"]
            #[doc = "identified by the given session identifier string or a new session."]
            #[doc = "While the session object exists, the session is considered to be \"in"]
            #[doc = "use\"."]
            #[doc = ""]
            #[doc = "If a identifier string represents a session that is currently actively"]
            #[doc = "in use by the the same client, an 'in_use' error is raised. If some"]
            #[doc = "other client is currently using the same session, the new session will"]
            #[doc = "replace managing the associated state."]
            #[doc = ""]
            #[doc = "NULL is passed to initiate a new session. If an id is passed which does"]
            #[doc = "not represent a valid session, the compositor treats it as if NULL had"]
            #[doc = "been passed."]
            #[doc = ""]
            #[doc = "A client is allowed to have any number of in use sessions at the same"]
            #[doc = "time."]
            fn get_session(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                reason: Reason,
                session: Option<String>,
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
                            tracing::debug!("xx_session_manager_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let reason = message.uint()?;
                            let session = message.string()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_session_manager_v1#{}.get_session({}, {}, \"{}\")",
                                sender_id,
                                id,
                                reason,
                                session
                                    .as_ref()
                                    .map_or("null".to_string(), |v| v.to_string())
                            );
                            self.get_session(connection, sender_id, id, reason.try_into()?, session)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A xx_session_v1 object represents a session for an application. While the"]
    #[doc = "object exists, all surfaces which have been added to the session will"]
    #[doc = "have states stored by the compositor which can be reapplied at a later"]
    #[doc = "time. Two sessions cannot exist for the same identifier string."]
    #[doc = ""]
    #[doc = "States for surfaces added to a session are automatically updated by the"]
    #[doc = "compositor when they are changed."]
    #[doc = ""]
    #[doc = "Surfaces which have been added to a session are automatically removed from"]
    #[doc = "the session if xdg_toplevel.destroy is called for the surface."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_session_v1 {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        pub enum Error {
            #[doc = "restore cannot be performed after initial toplevel commit"]
            InvalidRestore = 1u32,
            #[doc = "toplevel name is already in used"]
            NameInUse = 2u32,
            #[doc = "toplevel was already mapped when restored"]
            AlreadyMapped = 3u32,
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
                    1u32 => Ok(Self::InvalidRestore),
                    2u32 => Ok(Self::NameInUse),
                    3u32 => Ok(Self::AlreadyMapped),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_session_v1 interface. See the module level documentation for more info"]
        pub trait XxSessionV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_session_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy a session object, preserving the current state but not continuing"]
            #[doc = "to make further updates if state changes occur. This makes the associated"]
            #[doc = "xx_toplevel_session_v1 objects inert."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove the session, making it no longer available for restoration. A"]
            #[doc = "compositor should in response to this request remove the data related to"]
            #[doc = "this session from its storage."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Attempt to add a given surface to the session. The passed name is used"]
            #[doc = "to identify what window is being restored, and may be used store window"]
            #[doc = "specific state within the session."]
            #[doc = ""]
            #[doc = "Calling this with a toplevel that is already managed by the session with"]
            #[doc = "the same associated will raise an in_use error."]
            fn add_toplevel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Inform the compositor that the toplevel associated with the passed name"]
            #[doc = "should have its window management state restored."]
            #[doc = ""]
            #[doc = "Calling this with a toplevel that is already managed by the session with"]
            #[doc = "the same associated will raise an in_use error."]
            #[doc = ""]
            #[doc = "This request must be called prior to the first commit on the associated"]
            #[doc = "wl_surface, otherwise an already_mapped error is raised."]
            #[doc = ""]
            #[doc = "As part of the initial configure sequence, if the toplevel was"]
            #[doc = "successfully restored, a xx_toplevel_session_v1.restored event is"]
            #[doc = "emitted. See the xx_toplevel_session_v1.restored event for further"]
            #[doc = "details."]
            fn restore_toplevel(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: waynest::ObjectId,
                toplevel: waynest::ObjectId,
                name: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Emitted at most once some time after getting a new session object. It"]
            #[doc = "means that no previous state was restored, and a new session was created."]
            #[doc = "The passed id can be used to restore previous sessions."]
            fn created(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                id: String,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.created(\"{}\")", sender_id, id);
                    let (payload, fds) =
                        waynest::PayloadBuilder::new().put_string(Some(id)).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 0u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Emitted at most once some time after getting a new session object. It"]
            #[doc = "means that previous state was at least partially restored. The same id"]
            #[doc = "can again be used to restore previous sessions."]
            fn restored(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.restored()", sender_id,);
                    let (payload, fds) = waynest::PayloadBuilder::new().build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 1u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Emitted at most once, if the session was taken over by some other"]
            #[doc = "client. When this happens, the session and all its toplevel session"]
            #[doc = "objects become inert, and should be destroyed."]
            fn replaced(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_session_v1#{}.replaced()", sender_id,);
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
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_session_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_session_v1#{}.remove()", sender_id,);
                            self.remove(connection, sender_id).await
                        }
                        2u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_session_v1#{}.add_toplevel({}, {}, \"{}\")",
                                sender_id,
                                id,
                                toplevel,
                                name
                            );
                            self.add_toplevel(connection, sender_id, id, toplevel, name)
                                .await
                        }
                        3u16 => {
                            let id = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let toplevel = message
                                .object()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let name = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_session_v1#{}.restore_toplevel({}, {}, \"{}\")",
                                sender_id,
                                id,
                                toplevel,
                                name
                            );
                            self.restore_toplevel(connection, sender_id, id, toplevel, name)
                                .await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub mod xx_toplevel_session_v1 {
        #[doc = "Trait to implement the xx_toplevel_session_v1 interface. See the module level documentation for more info"]
        pub trait XxToplevelSessionV1
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_toplevel_session_v1";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the object. This has no effect window management of the"]
            #[doc = "associated toplevel."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Remove a specified surface from the session and render any corresponding"]
            #[doc = "xx_toplevel_session_v1 object inert. The compositor should remove any"]
            #[doc = "data related to the toplevel in the corresponding session from its internal"]
            #[doc = "storage."]
            fn remove(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "The \"restored\" event is emitted prior to the first"]
            #[doc = "xdg_toplevel.configure for the toplevel. It will only be emitted after"]
            #[doc = "xx_session_v1.restore_toplevel, and the initial empty surface state has"]
            #[doc = "been applied, and it indicates that the surface's session is being"]
            #[doc = "restored with this configure event."]
            fn restored(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_toplevel_session_v1#{}.restored({})",
                        sender_id,
                        surface
                    );
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
                            tracing::debug!("xx_toplevel_session_v1#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_toplevel_session_v1#{}.remove()", sender_id,);
                            self.remove(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
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
#[doc = "Once the protocol is to be declared stable, the 'xx' prefix and the"]
#[doc = "version number in the protocol and interface names are removed and the"]
#[doc = "interface version number is reset."]
#[allow(clippy::module_inception)]
pub mod xx_text_input_unstable_v3 {
    #[doc = "The xx_text_input_v3 interface represents text input and input methods"]
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
    #[doc = "xx_text_input_v3.enter and xx_text_input_v3.leave events. The focused"]
    #[doc = "surface must commit xx_text_input_v3.enable and"]
    #[doc = "xx_text_input_v3.disable requests as the keyboard focus moves across"]
    #[doc = "editable and non-editable elements of the UI. Those two requests are not"]
    #[doc = "expected to be paired with each other, the compositor must be able to"]
    #[doc = "handle consecutive series of the same request."]
    #[doc = ""]
    #[doc = "State is sent by the state requests (set_surrounding_text,"]
    #[doc = "set_content_type and set_cursor_rectangle) and a commit request. After an"]
    #[doc = "enter event or disable request all state information is invalidated and"]
    #[doc = "needs to be resent by the client."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_text_input_v3 {
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
        impl From<ChangeCause> for u32 {
            fn from(value: ChangeCause) -> Self {
                value as u32
            }
        }
        impl TryFrom<u32> for ChangeCause {
            type Error = waynest::ProtocolError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    0u32 => Ok(Self::InputMethod),
                    1u32 => Ok(Self::Other),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ChangeCause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        bitflags::bitflags! { # [doc = "Content hint is a bitmask to allow to modify the behavior of the text"] # [doc = "input."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct ContentHint : u32 { # [doc = "no special behavior"] const None = 0u32 ; # [doc = "suggest word completions"] const Completion = 1u32 ; # [doc = "suggest word corrections"] const Spellcheck = 2u32 ; # [doc = "switch to uppercase letters at the start of a sentence"] const AutoCapitalization = 4u32 ; # [doc = "prefer lowercase letters"] const Lowercase = 8u32 ; # [doc = "prefer uppercase letters"] const Uppercase = 16u32 ; # [doc = "prefer casing for titles and headings (can be language dependent)"] const Titlecase = 32u32 ; # [doc = "characters should be hidden"] const HiddenText = 64u32 ; # [doc = "typed text should not be stored"] const SensitiveData = 128u32 ; # [doc = "just Latin characters should be entered"] const Latin = 256u32 ; # [doc = "the text input is multiline"] const Multiline = 512u32 ; } }
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
                    9u32 => Ok(Self::Pin),
                    10u32 => Ok(Self::Date),
                    11u32 => Ok(Self::Time),
                    12u32 => Ok(Self::Datetime),
                    13u32 => Ok(Self::Terminal),
                    _ => Err(waynest::ProtocolError::MalformedPayload),
                }
            }
        }
        impl std::fmt::Display for ContentPurpose {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (*self as u32).fmt(f)
            }
        }
        #[doc = "Trait to implement the xx_text_input_v3 interface. See the module level documentation for more info"]
        pub trait XxTextInputV3
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_text_input_v3";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the xx_text_input object. Also disables all surfaces enabled"]
            #[doc = "through this xx_text_input object."]
            fn destroy(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Requests text input on the surface previously obtained from the enter"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "This request must be issued every time the active text input changes"]
            #[doc = "to a new one, including within the current surface. Use"]
            #[doc = "xx_text_input_v3.disable when there is no longer any input focus on"]
            #[doc = "the current surface."]
            #[doc = ""]
            #[doc = "Clients must not enable more than one text input on the single seat"]
            #[doc = "and should disable the current text input before enabling the new one."]
            #[doc = "At most one instance of text input may be in enabled state per instance,"]
            #[doc = "Requests to enable the another text input when some text input is active"]
            #[doc = "must be ignored by compositor."]
            #[doc = ""]
            #[doc = "This request resets all state associated with previous enable, disable,"]
            #[doc = "set_surrounding_text, set_text_change_cause, set_content_type, and"]
            #[doc = "set_cursor_rectangle requests, as well as the state associated with"]
            #[doc = "preedit_string, commit_string, and delete_surrounding_text events."]
            #[doc = ""]
            #[doc = "The set_surrounding_text, set_content_type and set_cursor_rectangle"]
            #[doc = "requests must follow if the text input supports the necessary"]
            #[doc = "functionality."]
            #[doc = ""]
            #[doc = "State set with this request is double-buffered. It will get applied on"]
            #[doc = "the next xx_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The changes must be applied by the compositor after issuing a"]
            #[doc = "xx_text_input_v3.commit request."]
            fn enable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Explicitly disable text input on the current surface (typically when"]
            #[doc = "there is no focus on any text entry inside the surface)."]
            #[doc = ""]
            #[doc = "State set with this request is double-buffered. It will get applied on"]
            #[doc = "the next xx_text_input_v3.commit request."]
            fn disable(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the surrounding plain text around the input, excluding the preedit"]
            #[doc = "text."]
            #[doc = ""]
            #[doc = "The client should notify the compositor of any changes in any of the"]
            #[doc = "values carried with this request, including changes caused by handling"]
            #[doc = "incoming text-input events as well as changes caused by other"]
            #[doc = "mechanisms like keyboard typing."]
            #[doc = ""]
            #[doc = "If the client is unaware of the text around the cursor, it should not"]
            #[doc = "issue this request, to signify lack of support to the compositor."]
            #[doc = ""]
            #[doc = "Text is UTF-8 encoded, and should include the cursor position, the"]
            #[doc = "complete selection and additional characters before and after them."]
            #[doc = "There is a maximum length of wayland messages, so text can not be"]
            #[doc = "longer than 4000 bytes."]
            #[doc = ""]
            #[doc = "Cursor is the byte offset of the cursor within text buffer."]
            #[doc = ""]
            #[doc = "Anchor is the byte offset of the selection anchor within text buffer."]
            #[doc = "If there is no selected text, anchor is the same as cursor."]
            #[doc = ""]
            #[doc = "If any preedit text is present, it is replaced with a cursor for the"]
            #[doc = "purpose of this event."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next xx_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial state for affected fields is empty, meaning that the text"]
            #[doc = "input does not support sending surrounding text. If the empty values"]
            #[doc = "get applied, subsequent attempts to change them may have no effect."]
            fn set_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: String,
                cursor: i32,
                anchor: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Tells the compositor why the text surrounding the cursor changed."]
            #[doc = ""]
            #[doc = "Whenever the client detects an external change in text, cursor, or"]
            #[doc = "anchor posision, it must issue this request to the compositor. This"]
            #[doc = "request is intended to give the input method a chance to update the"]
            #[doc = "preedit text in an appropriate way, e.g. by removing it when the user"]
            #[doc = "starts typing with a keyboard."]
            #[doc = ""]
            #[doc = "cause describes the source of the change."]
            #[doc = ""]
            #[doc = "The value set with this request is double-buffered. It must be applied"]
            #[doc = "and reset to initial at the next xx_text_input_v3.commit request."]
            #[doc = ""]
            #[doc = "The initial value of cause is input_method."]
            fn set_text_change_cause(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                cause: ChangeCause,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Sets the content purpose and content hint. While the purpose is the"]
            #[doc = "basic purpose of an input field, the hint flags allow to modify some of"]
            #[doc = "the behavior."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next xx_text_input_v3.commit request."]
            #[doc = "Subsequent attempts to update them may have no effect. The values"]
            #[doc = "remain valid until the next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial value for hint is none, and the initial value for purpose"]
            #[doc = "is normal."]
            fn set_content_type(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                hint: ContentHint,
                purpose: ContentPurpose,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Marks an area around the cursor as a x, y, width, height rectangle in"]
            #[doc = "surface local coordinates."]
            #[doc = ""]
            #[doc = "Allows the compositor to put a window with word suggestions near the"]
            #[doc = "cursor, without obstructing the text being input."]
            #[doc = ""]
            #[doc = "If the client is unaware of the position of edited text, it should not"]
            #[doc = "issue this request, to signify lack of support to the compositor."]
            #[doc = ""]
            #[doc = "Values set with this request are double-buffered. They will get applied"]
            #[doc = "on the next xx_text_input_v3.commit request, and stay valid until the"]
            #[doc = "next committed enable or disable request."]
            #[doc = ""]
            #[doc = "The initial values describing a cursor rectangle are empty. That means"]
            #[doc = "the text input does not support describing the cursor area. If the"]
            #[doc = "empty values get applied, subsequent attempts to change them may have"]
            #[doc = "no effect."]
            fn set_cursor_rectangle(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Atomically applies state changes recently sent to the compositor."]
            #[doc = ""]
            #[doc = "The commit request establishes and updates the state of the client, and"]
            #[doc = "must be issued after any changes to apply them."]
            #[doc = ""]
            #[doc = "Text input state (enabled status, content purpose, content hint,"]
            #[doc = "surrounding text and change cause, cursor rectangle) is conceptually"]
            #[doc = "double-buffered within the context of a text input, i.e. between a"]
            #[doc = "committed enable request and the following committed enable or disable"]
            #[doc = "request."]
            #[doc = ""]
            #[doc = "Protocol requests modify the pending state, as opposed to the current"]
            #[doc = "state in use by the input method. A commit request atomically applies"]
            #[doc = "all pending state, replacing the current state. After commit, the new"]
            #[doc = "pending state is as documented for each related request."]
            #[doc = ""]
            #[doc = "Requests are applied in the order of arrival."]
            #[doc = ""]
            #[doc = "Neither current nor pending state are modified unless noted otherwise."]
            #[doc = ""]
            #[doc = "The compositor must count the number of commit requests coming from"]
            #[doc = "each xx_text_input_v3 object and use the count as the serial in done"]
            #[doc = "events."]
            fn commit(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send;
            #[doc = "Notification that this seat's text-input focus is on a certain surface."]
            #[doc = ""]
            #[doc = "If client has created multiple text input objects, compositor must send"]
            #[doc = "this event to all of them."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus. This event sets the current surface for the"]
            #[doc = "text-input object."]
            fn enter(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_text_input_v3#{}.enter({})", sender_id, surface);
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
            #[doc = "Notification that this seat's text-input focus is no longer on a"]
            #[doc = "certain surface. The client should reset any preedit string previously"]
            #[doc = "set."]
            #[doc = ""]
            #[doc = "The leave notification clears the current surface. It is sent before"]
            #[doc = "the enter notification for the new focus. After leave event, compositor"]
            #[doc = "must ignore requests from any text input instances until next enter"]
            #[doc = "event."]
            #[doc = ""]
            #[doc = "When the seat has the keyboard capability the text-input focus follows"]
            #[doc = "the keyboard focus."]
            fn leave(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                surface: waynest::ObjectId,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_text_input_v3#{}.leave({})", sender_id, surface);
                    let (payload, fds) = waynest::PayloadBuilder::new()
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
            #[doc = "Notify when a new composing text (pre-edit) should be set at the"]
            #[doc = "current cursor position. Any previously set composing text must be"]
            #[doc = "removed. Any previously existing selected text must be removed."]
            #[doc = ""]
            #[doc = "The argument text contains the pre-edit string buffer."]
            #[doc = ""]
            #[doc = "The parameters cursor_begin and cursor_end are counted in bytes"]
            #[doc = "relative to the beginning of the submitted text buffer. Cursor should"]
            #[doc = "be hidden when both are equal to -1."]
            #[doc = ""]
            #[doc = "They could be represented by the client as a line if both values are"]
            #[doc = "the same, or as a text highlight otherwise."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next xx_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string, and cursor_begin,"]
            #[doc = "cursor_end and cursor_hidden are all 0."]
            fn preedit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: Option<String>,
                cursor_begin: i32,
                cursor_end: i32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_text_input_v3#{}.preedit_string(\"{}\", {}, {})",
                        sender_id,
                        text.as_ref().map_or("null".to_string(), |v| v.to_string()),
                        cursor_begin,
                        cursor_end
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_string(text)
                        .put_int(cursor_begin)
                        .put_int(cursor_end)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 2u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when text should be inserted into the editor widget. The text to"]
            #[doc = "commit could be either just a single character after a key press or the"]
            #[doc = "result of some composing (pre-edit)."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next xx_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial value of text is an empty string."]
            fn commit_string(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                text: Option<String>,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_text_input_v3#{}.commit_string(\"{}\")",
                        sender_id,
                        text.as_ref().map_or("null".to_string(), |v| v.to_string())
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new().put_string(text).build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 3u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Notify when the text around the current cursor position should be"]
            #[doc = "deleted."]
            #[doc = ""]
            #[doc = "Before_length and after_length are the number of bytes before and after"]
            #[doc = "the current cursor index (excluding the selection) to delete."]
            #[doc = ""]
            #[doc = "If a preedit text is present, in effect before_length is counted from"]
            #[doc = "the beginning of it, and after_length from its end (see done event"]
            #[doc = "sequence)."]
            #[doc = ""]
            #[doc = "Values set with this event are double-buffered. They must be applied"]
            #[doc = "and reset to initial on the next xx_text_input_v3.done event."]
            #[doc = ""]
            #[doc = "The initial values of both before_length and after_length are 0."]
            fn delete_surrounding_text(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                before_length: u32,
                after_length: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        "-> xx_text_input_v3#{}.delete_surrounding_text({}, {})",
                        sender_id,
                        before_length,
                        after_length
                    );
                    let (payload, fds) = waynest::PayloadBuilder::new()
                        .put_uint(before_length)
                        .put_uint(after_length)
                        .build();
                    futures_util::SinkExt::send(
                        connection,
                        waynest::Message::new(sender_id, 4u16, payload, fds),
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
            #[doc = "Instruct the application to apply changes to state requested by the"]
            #[doc = "preedit_string, commit_string and delete_surrounding_text events. The"]
            #[doc = "state relating to these events is double-buffered, and each one"]
            #[doc = "modifies the pending state. This event replaces the current state with"]
            #[doc = "the pending state."]
            #[doc = ""]
            #[doc = "The application must proceed by evaluating the changes in the following"]
            #[doc = "order:"]
            #[doc = ""]
            #[doc = "1. Replace existing preedit string with the cursor."]
            #[doc = "2. Delete requested surrounding text."]
            #[doc = "3. Insert commit string with the cursor at its end."]
            #[doc = "4. Calculate surrounding text to send."]
            #[doc = "5. Insert new preedit text in cursor position."]
            #[doc = "6. Place cursor inside preedit text."]
            #[doc = ""]
            #[doc = "The serial number reflects the last state of the xx_text_input_v3"]
            #[doc = "object known to the compositor. The value of the serial argument must"]
            #[doc = "be equal to the number of commit requests already issued on that object."]
            #[doc = ""]
            #[doc = "When the client receives a done event with a serial different than the"]
            #[doc = "number of past commit requests, it must proceed with evaluating and"]
            #[doc = "applying the changes as normal, except it should not change the current"]
            #[doc = "state of the xx_text_input_v3 object. All pending state requests"]
            #[doc = "(set_surrounding_text, set_content_type and set_cursor_rectangle) on"]
            #[doc = "the xx_text_input_v3 object should be sent and committed after"]
            #[doc = "receiving a xx_text_input_v3.done event with a matching serial."]
            fn done(
                &self,
                connection: &mut Self::Connection,
                sender_id: waynest::ObjectId,
                serial: u32,
            ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
            {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("-> xx_text_input_v3#{}.done({})", sender_id, serial);
                    let (payload, fds) = waynest::PayloadBuilder::new().put_uint(serial).build();
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
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_text_input_v3#{}.destroy()", sender_id,);
                            self.destroy(connection, sender_id).await
                        }
                        1u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_text_input_v3#{}.enable()", sender_id,);
                            self.enable(connection, sender_id).await
                        }
                        2u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_text_input_v3#{}.disable()", sender_id,);
                            self.disable(connection, sender_id).await
                        }
                        3u16 => {
                            let text = message
                                .string()?
                                .ok_or(waynest::ProtocolError::MalformedPayload)?;
                            let cursor = message.int()?;
                            let anchor = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_text_input_v3#{}.set_surrounding_text(\"{}\", {}, {})",
                                sender_id,
                                text,
                                cursor,
                                anchor
                            );
                            self.set_surrounding_text(connection, sender_id, text, cursor, anchor)
                                .await
                        }
                        4u16 => {
                            let cause = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_text_input_v3#{}.set_text_change_cause({})",
                                sender_id,
                                cause
                            );
                            self.set_text_change_cause(connection, sender_id, cause.try_into()?)
                                .await
                        }
                        5u16 => {
                            let hint = message.uint()?;
                            let purpose = message.uint()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_text_input_v3#{}.set_content_type({}, {})",
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
                        6u16 => {
                            let x = message.int()?;
                            let y = message.int()?;
                            let width = message.int()?;
                            let height = message.int()?;
                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                "xx_text_input_v3#{}.set_cursor_rectangle({}, {}, {}, {})",
                                sender_id,
                                x,
                                y,
                                width,
                                height
                            );
                            self.set_cursor_rectangle(connection, sender_id, x, y, width, height)
                                .await
                        }
                        7u16 => {
                            #[cfg(feature = "tracing")]
                            tracing::debug!("xx_text_input_v3#{}.commit()", sender_id,);
                            self.commit(connection, sender_id).await
                        }
                        opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                    }
                }
            }
        }
    }
    #[doc = "A factory for text-input objects. This object is a global singleton."]
    #[allow(clippy::too_many_arguments)]
    pub mod xx_text_input_manager_v3 {
        #[doc = "Trait to implement the xx_text_input_manager_v3 interface. See the module level documentation for more info"]
        pub trait XxTextInputManagerV3
        where
            Self: std::marker::Sync,
        {
            type Connection: waynest::Connection;
            const INTERFACE: &'static str = "xx_text_input_manager_v3";
            const VERSION: u32 = 1u32;
            #[doc = "Destroy the xx_text_input_manager object."]
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
                            tracing::debug!("xx_text_input_manager_v3#{}.destroy()", sender_id,);
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
                                "xx_text_input_manager_v3#{}.get_text_input({}, {})",
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
