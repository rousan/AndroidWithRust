use crate::logger;
use crate::prelude::*;
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
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
        let (reply_message_what, reply_message_data) = crate::message::on_message(&jvm, what, data).await;

        let env = jvm.env();
        env.call_method(
            &jni_worker,
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
