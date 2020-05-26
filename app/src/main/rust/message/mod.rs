use crate::bridge::MessageData;
use crate::transfer;

pub mod what;

/// Calls when the bridge is started or re-started.
pub async fn on_start() {
    info!("Bridge is started, thread: {:?}", std::thread::current().id());

    if let Err(err) = transfer::start_server().await {
        error!("Start server error: {}", err);
    }
}

/// Calls when a message is received from the Java end.
pub async fn on_message(what: i32, data: MessageData) {
    info!(
        "Got a message: what: {}, thread: {:?}",
        what::to_string(what),
        std::thread::current().id()
    );

    match what {
        self::what::INIT_DATA => {
            transfer::set_init_data(data);
        }
        self::what::SEND_FILE => {
            let ip = data.get_string("ip").unwrap();
            let path = data.get_string("path").unwrap();
            if let Err(err) = transfer::send_file(&ip, &path).await {
                error!("Send file error: {}", err);
            }
        }
        _ => (),
    }
}

/// Calls when the bridge is shut down.
pub async fn on_destroy() {
    info!("Bridge is shut down, thread: {:?}", std::thread::current().id());
}
