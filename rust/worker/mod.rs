use crate::logger;
use jni::{
    errors::ErrorKind,
    objects::{JObject, JValue},
    JNIEnv,
};
use std::ffi::CString;
use std::sync::Once;

mod runtime;

static INIT_WORKER: Once = Once::new();

pub fn start_worker() {
    INIT_WORKER.call_once(|| {
        logger::init();
        runtime::start_tokio_runtime();
    });
}

pub fn handle_message(jni_env: JNIEnv, jni_worker: JObject, what: CString, data: CString) {
    let jvm = jni_env.get_java_vm().unwrap();
    let jni_worker = jni_env.new_global_ref(jni_worker).unwrap();

    runtime::spawn(async move {
        let (reply_what, reply_data) = on_message(what.to_str().unwrap(), data.to_str().unwrap()).await;

        let jni_env = match jvm.get_env() {
            Ok(env) => env,
            Err(err) => match err.kind() {
                ErrorKind::ThreadDetached => {
                    info!("ThreadDetached");
                    jvm.attach_current_thread_as_daemon().unwrap()
                }
                _ => panic!("Thread attached error: {}", err),
            },
        };

        let what = jni_env.new_string(reply_what).unwrap();
        let data = jni_env.new_string(reply_data).unwrap();

        jni_env
            .call_method(
                jni_worker.as_obj(),
                "sendMessageToJava",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                &[JValue::Object(what.into()), JValue::Object(data.into())],
            )
            .unwrap();
    });
}

pub async fn on_message(what: &str, data: &str) -> (String, String) {
    info!(
        "Rust: got a message: what: {:?}, data: {:?}, thread: {:?}",
        what,
        data,
        std::thread::current().id()
    );

    ("noop".to_owned(), "noop2".to_owned())
}
