use crate::logger;
use jni::errors::ErrorKind;
use jni::objects::{JObject, JString, JValue};
use jni::sys::jint;
use jni::{
    signature::{JavaType, Primitive},
    JNIEnv,
};
use lazy_static::lazy_static;
use std::ffi::{CStr, CString};
use std::sync::Once;
use std::task::Poll;
use std::thread;
use tokio::runtime::Runtime;
use tokio::time::{delay_for, Duration};

static START: Once = Once::new();

lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_Runtime_start(env: JNIEnv, obj: JObject) {
    START.call_once(|| {
        start_runtime(env, obj);
    });
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_Runtime_sendMessage(
    env: JNIEnv,
    obj: JObject,
    id: jint,
    data: JString,
) {
    let data = CString::from(CStr::from_ptr(env.get_string(data).unwrap().as_ptr()));

    handle_message(env, obj, id.into(), data);
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_rousan_androidwithrust_Runtime_sendMessage2(
    env: JNIEnv,
    j_runtime: JObject,
    id: jint,
    data: JObject,
) {
    handle_message2(env, j_runtime, id, data);
}

fn start_runtime(_env: JNIEnv, _obj: JObject) {
    logger::init();
    info!("Starting runtime..., thread id: {:?}", thread::current().id());

    lazy_static::initialize(&RUNTIME);

    //    thread::spawn(|| {
    //        info!(
    //            "Starting runtime in another thread..., thread id: {:?}",
    //            thread::current().id()
    //        );
    //        lazy_static::initialize(&RUNTIME);
    //
    //        RUNTIME
    //            .handle()
    //            .block_on(tokio::future::poll_fn(|_| Poll::<()>::Pending));
    //    });
}

fn handle_message(env: JNIEnv, obj: JObject, id: i32, data: CString) {
    let jvm = env.get_java_vm().unwrap();
    let obj = env.new_global_ref(obj).unwrap();

    RUNTIME.spawn(async move {
        info!(
            "id: {:?}, data: {:?}, thread id: {:?}",
            id,
            data,
            thread::current().id()
        );

        let env = match jvm.get_env() {
            Ok(env) => env,
            Err(err) => match err.kind() {
                ErrorKind::ThreadDetached => {
                    info!("ThreadDetached");
                    jvm.attach_current_thread_as_daemon().unwrap()
                }
                _ => panic!("{}", err),
            },
        };

        //        // https://docs.rs/jni/0.16.0/jni/struct.JNIEnv.html#method.get_method_id
        //        let method_id = env
        //            .get_method_id(
        //                "io/rousan/androidwithrust/Runtime",
        //                "handleMessage",
        //                "(ILjava/lang/String;)V",
        //            )
        //            .unwrap();
        //
        //        info!("{:?}", method_id);

        //        let data = env.new_string("Hi back3").unwrap();
        //        unsafe {
        //            env.call_method_unsafe(
        //                obj.as_obj(),
        //                method_id,
        //                JavaType::Primitive(Primitive::Void),
        //                &[2.into(), JValue::Object(data.into())],
        //            )
        //            .unwrap();
        //        }

        let data = env.new_string("Hi back3").unwrap();
        env.call_method(
            obj.as_obj(),
            "handleMessage",
            "(ILjava/lang/String;)V",
            &[2.into(), JValue::Object(data.into())],
        )
        .unwrap();
    });
}

fn handle_message2(env: JNIEnv, j_runtime: JObject, id: i32, data: JObject) {
    let jvm = env.get_java_vm().unwrap();
    //    let j_runtime = env.new_global_ref(j_runtime).unwrap();
    let data = env.new_global_ref(data).unwrap();

    // don't block the main thread, run the message processing in tokio runtime.

    RUNTIME.spawn(async move {
        {
            let env = match jvm.get_env() {
                Ok(env) => env,
                Err(err) => match err.kind() {
                    ErrorKind::ThreadDetached => {
                        info!("ThreadDetached");
                        jvm.attach_current_thread_as_daemon().unwrap()
                    }
                    _ => {
                        info!("paniced");
                        panic!("{}", err);
                    }
                },
            };

            let key = env.new_string("abcd").unwrap();
            let msg = env
                .call_method(
                    data.as_obj(),
                    "getString",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    &[JValue::Object(key.into())],
                )
                .unwrap();

            if let JValue::Object(o) = msg {
                let msg = JString::from(o);
                let msg = unsafe { CString::from(CStr::from_ptr(env.get_string(msg).unwrap().as_ptr())) };
                info!("id: {:?}, msg: {:?}, thread id: {:?}", id, msg, thread::current().id());
            }
        }

        delay_for(Duration::from_secs(3)).await;
        info!("Waited 2");
    });
}
