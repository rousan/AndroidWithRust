use crate::prelude::*;
use crate::worker::MessageData;
use jni::JavaVM;

pub mod what;

pub async fn on_message(jvm: &JavaVM, what: i32, _data: MessageData) -> (i32, MessageData) {
    info!(
        "Rust: Got a message: what: {}, thread: {:?}",
        what::to_string(what),
        std::thread::current().id()
    );

    match what {
        what::PING => (what::PONG, MessageData::empty(&jvm.env())),
        _ => (what::NOOP, MessageData::empty(&jvm.env())),
    }
}
