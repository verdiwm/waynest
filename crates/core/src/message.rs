use std::{collections::VecDeque, os::fd::RawFd};

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{DecodeError, Fixed, NewId, ObjectId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    object_id: ObjectId,
    opcode: u16,
    payload: Bytes,
    pub(crate) fds: VecDeque<RawFd>,
}

#[cfg(feature = "fuzz")]
impl<'a> arbitrary::Arbitrary<'a> for Message {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let len = u.arbitrary_len::<u8>()?;

        let payload = u.bytes(len).map(Bytes::copy_from_slice)?;

        Ok(Self {
            object_id: ObjectId::arbitrary(u)?,
            opcode: u16::arbitrary(u)?,
            payload,
            fds: VecDeque::<RawFd>::arbitrary(u)?,
        })
    }
}

impl Message {
    pub const fn new(
        object_id: ObjectId,
        opcode: u16,
        payload: Bytes,
        fds: VecDeque<RawFd>,
    ) -> Self {
        Self {
            object_id,
            opcode,
            payload,
            fds,
        }
    }

    pub const fn object_id(&self) -> ObjectId {
        self.object_id
    }

    pub const fn opcode(&self) -> u16 {
        self.opcode
    }

    pub fn encode(self, buf: &mut BytesMut) {
        buf.reserve(8 + self.payload.len());
        buf.put_u32_ne(self.object_id.as_raw());
        buf.put_u32_ne((((self.payload.len() + 8) as u32) << 16) | self.opcode as u32);
        buf.put_slice(&self.payload);
    }

    pub fn decode(bytes: &mut BytesMut) -> Result<Option<Self>, DecodeError> {
        let object_id = match bytes.chunk().get(..4) {
            Some(peek) => ObjectId::new(u32::from_ne_bytes(unsafe {
                *(peek as *const _ as *const [u8; 4])
            }))
            .ok_or(DecodeError::InvalidSenderId)?,
            None => return Ok(None),
        };

        let second = match bytes.chunk().get(4..8) {
            Some(peek) => u32::from_ne_bytes(unsafe { *(peek as *const _ as *const [u8; 4]) }),
            None => return Ok(None),
        };

        let len = (second >> 16) as usize;
        let opcode = (second & 65535) as u16;

        if len < 8 {
            return Err(DecodeError::InvalidLength(len));
        }

        if bytes.remaining() < len {
            return Ok(None);
        }

        bytes.advance(8); // Skip the header we've already processed

        let payload = bytes.copy_to_bytes(len - 8);

        Ok(Some(Message {
            object_id,
            opcode,
            payload,
            fds: VecDeque::new(),
        }))
    }

    pub fn int(&mut self) -> Result<i32, DecodeError> {
        self.payload
            .try_get_i32_ne()
            .map_err(|_| DecodeError::MalformedPayload)
    }

    pub fn uint(&mut self) -> Result<u32, DecodeError> {
        self.payload
            .try_get_u32_ne()
            .map_err(|_| DecodeError::MalformedPayload)
    }

    pub fn fixed(&mut self) -> Result<Fixed, DecodeError> {
        self.uint().map(|raw| unsafe { Fixed::from_raw(raw) })
    }

    pub fn string(&mut self) -> Result<Option<String>, DecodeError> {
        let mut array = self.array()?;

        if array.is_empty() {
            return Ok(None);
        }

        if let Some(b'\0') = array.pop() {
            return String::from_utf8(array)
                .map_err(|_| DecodeError::MalformedPayload)
                .map(Some);
        }

        Err(DecodeError::MalformedPayload)
    }

    pub fn object(&mut self) -> Result<Option<ObjectId>, DecodeError> {
        self.uint().map(ObjectId::new)
    }

    pub fn new_id(&mut self) -> Result<NewId, DecodeError> {
        let interface = self.string()?.ok_or(DecodeError::MalformedPayload)?;
        let version = self.uint()?;
        let object_id = self.object()?.ok_or(DecodeError::MalformedPayload)?;

        Ok(NewId {
            interface,
            version,
            object_id,
        })
    }

    pub fn array(&mut self) -> Result<Vec<u8>, DecodeError> {
        let len = self.uint()? as usize;

        if len == 0 {
            return Ok(Vec::new());
        }

        if self.payload.remaining() < len {
            return Err(DecodeError::MalformedPayload);
        }

        let array = self.payload.copy_to_bytes(len).to_vec();

        self.payload.advance(self.payload.remaining() % 4);

        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    // use std::collections::VecDeque;

    // use bytes::{Bytes, BytesMut};

    // use crate::{Message, ObjectId};

    #[test]
    fn encode_decode_roundtrip() {
        // let msg = Message {
        //     object_id: unsafe { ObjectId::from_raw(10) },
        //     opcode: 0,
        //     payload: Bytes::copy_from_slice(b"\x03\0\0\0"),
        //     fds: vec![10, 20, 0, 33, 48, 17].into(),
        // };

        // let mut bytes = BytesMut::new();
        // let mut fds = VecDeque::new();
        // msg.clone().encode(&mut bytes, &mut fds);

        // assert_eq!(
        //     Some(msg),
        //     Message::decode(&mut bytes, fds).expect("Failed to parse bytes")
        // );
    }
}
