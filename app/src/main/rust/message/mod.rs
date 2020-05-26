use crate::bridge::MessageData;
use crate::counter;

pub mod what;

/// Calls when the bridge is started or re-started.
pub async fn on_start() {
    info!("Bridge is started, thread: {:?}", std::thread::current().id());
}

/// Calls when a message is received from the Java end.
pub async fn on_message(what: i32, _data: MessageData) {
    info!(
        "Got a message: what: {}, thread: {:?}",
        what::to_string(what),
        std::thread::current().id()
    );

    match what {
        self::what::INITIATE => counter::on_initiate().await,
        self::what::INCREASE_COUNTER => counter::on_increase_counter().await,
        self::what::DECREASE_COUNTER => counter::on_decrease_counter().await,
        _ => (),
    }
}

/// Calls when the bridge is shut down.
pub async fn on_destroy() {
    info!("Bridge is shut down, thread: {:?}", std::thread::current().id());
}
