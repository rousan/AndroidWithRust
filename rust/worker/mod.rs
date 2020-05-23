use crate::logger;
use crate::prelude::*;
use jni::{
    objects::{JObject, JValue},
    JNIEnv, JavaVM,
};
use std::panic;
use std::sync::Once;

pub use data::MessageData;

mod data;
mod runtime;

static INIT_WORKER: Once = Once::new();

pub fn start_worker() {
    INIT_WORKER.call_once(|| {
        logger::init();

        panic::set_hook(Box::new(|info| error!("{}", info.to_string())));

        runtime::start_tokio_runtime();
    });
}

pub fn handle_message(env: JNIEnv, jni_worker: JObject, what: i32, data: MessageData) {
    let jvm = env.get_java_vm().unwrap();
    let jni_worker = env.new_global_ref(jni_worker).unwrap();

    runtime::spawn(async move {
        let (reply_message_what, reply_message_data) = on_message(&jvm, what, data).await;

        let env = jvm.env();
        env.call_method(
            jni_worker.as_obj(),
            "sendMessageToJava",
            "(ILandroid/os/Bundle;)V",
            &[
                reply_message_what.into(),
                JValue::Object(reply_message_data.bundle().as_obj()),
            ],
        )
        .unwrap();
    });
}

pub async fn on_message(jvm: &JavaVM, what: i32, data: MessageData) -> (i32, MessageData) {
    info!(
        "Rust: got a message: what: {:?}, data: {:?}, thread: {:?}",
        what,
        data.get_string(&jvm.env(), "key"),
        std::thread::current().id()
    );

    tokio::task::spawn(async move {
        info!("hey");
    })
    .await
    .unwrap();

    let env = jvm.env();
    let data = MessageData::new(&env)
        .put_string(&env, "key", "hijibiji, from rust")
        .put_string(&env, "key2", "mobile, from rust");

    // Reply message
    (200, data)
}
