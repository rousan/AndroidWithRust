use jni::{
    objects::{JObject, JString},
    JNIEnv,
};
use std::sync::Once;

static INITIATE_BRIDGE: Once = Once::new();

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_bridge_Bridge_initiateBridge(
    env: JNIEnv,
    java_bridge: JObject,
) {
    INITIATE_BRIDGE.call_once(|| {
        start_runtime(env, obj);
    });
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_bridge_Bridge_emitEventToRust(
    env: JNIEnv,
    java_bridge: JObject,
    event_name: JString,
    data: JObject,
) {
    todo!()
}
