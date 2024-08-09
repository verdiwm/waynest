#![no_main]
use bytes::{Bytes, BytesMut};
use libfuzzer_sys::fuzz_target;
use std::os::fd::RawFd;
use waynest::wire::Message;

#[derive(Debug)]
struct Data {
    bytes: BytesMut,
    fds: Vec<RawFd>,
}

impl<'a> arbitrary::Arbitrary<'a> for Data {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let len = u.arbitrary_len::<u8>()?;

        let bytes = u.bytes(len).map(Bytes::copy_from_slice)?.into();

        Ok(Self {
            bytes,
            fds: Vec::<RawFd>::arbitrary(u)?,
        })
    }
}

fuzz_target!(|data: Data| {
    let mut bytes = data.bytes;
    let mut fds = data.fds;

    let _ = Message::from_bytes(&mut bytes, &mut fds);
});
