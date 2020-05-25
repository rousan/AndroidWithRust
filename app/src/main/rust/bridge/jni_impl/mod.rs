#![allow(non_snake_case)]

use crate::bridge::{self, MessageData};
use jni::objects::JObject;
use jni::sys::jint;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_bridge_Bridge_startNativeBridge(
    env: JNIEnv,
    jni_bridge: JObject,
) {
    bridge::start_bridge(env, jni_bridge);
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_bridge_Bridge_shutdownNativeBridge(
    env: JNIEnv,
    jni_bridge: JObject,
) {
    bridge::shutdown_bridge(env, jni_bridge);
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_bridge_Bridge_sendMessageToNativeBridge(
    env: JNIEnv,
    jni_bridge: JObject,
    what: jint,
    bundle: JObject,
) {
    let data = MessageData::wrap(&env, bundle);
    bridge::handle_message(env, jni_bridge, what as i32, data);
}
