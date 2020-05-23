#![allow(non_snake_case)]

use crate::worker;
use crate::worker::MessageData;
use jni::objects::JObject;
use jni::sys::jint;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_worker_Worker_startRustWorker(
    _env: JNIEnv,
    _jni_worker: JObject,
) {
    worker::start_worker();
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_worker_Worker_sendMessageToWorker(
    env: JNIEnv,
    jni_worker: JObject,
    what: jint,
    bundle: JObject,
) {
    let data = MessageData::wrap(&env, bundle);
    worker::handle_message(env, jni_worker, what as i32, data);
}
