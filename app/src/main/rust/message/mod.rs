use crate::bridge::{self, MessageData};

pub mod what;

/// Calls when the bridge is started or re-started.
pub async fn on_start() {
    info!("Bridge is started, thread: {:?}", std::thread::current().id());
    bridge::send_message(2000, MessageData::empty());
}

/// Calls when a message is received from the Java end.
pub async fn on_message(what: i32, _data: MessageData) {
    info!(
        "Got a message: what: {}, thread: {:?}",
        what::to_string(what),
        std::thread::current().id()
    );
}

/// Calls when the bridge is shut down.
pub async fn on_destroy() {
    info!("Bridge is shut down, thread: {:?}", std::thread::current().id());
}