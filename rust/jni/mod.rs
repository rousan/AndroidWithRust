use crate::worker;
use jni::objects::{JObject, JString};
use jni::JNIEnv;
use std::ffi::{CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_worker_Worker_startRustWorker(_env: JNIEnv, _this: JObject) {
    worker::start_worker();
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_worker_Worker_sendMessageToWorker(
    env: JNIEnv,
    this: JObject,
    what: JString,
    data: JString,
) {
    let what = convert_to_cstr(&env, what);
    let data = convert_to_cstr(&env, data);

    worker::handle_message(env, this, what, data);
}

pub fn convert_to_cstr(env: &JNIEnv, string: JString) -> CString {
    unsafe { CString::from(CStr::from_ptr(env.get_string(string).unwrap().as_ptr())) }
}
