#![no_main]
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use waynest::wire::Message;

fuzz_target!(|message: Message| {
    let mut bytes = BytesMut::new();
    let mut fds = Vec::new();

    message.encode(&mut bytes, &mut fds);
});
