#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;

mod logger;
mod runtime;

//use android_logger::{Config, FilterBuilder};
//use jni::objects::{JObject, JString};
//use jni::sys::jstring;
//use jni::JNIEnv;
//use log::Level;
//use std::ffi::{CStr, CString};
//use std::sync::Once;
//use std::thread;
//use tokio::runtime::Runtime;
//use tokio::time::{self, Duration};

//static START: Once = Once::new();
//
//#[no_mangle]
//pub unsafe extern "C" fn Java_io_rousan_androidwithrust_MainActivity_hello(
//    env: JNIEnv,
//    obj: JObject,
//    j_recipient: JString,
//) -> jstring {
//    init_logger();
//
//    let recipient = CString::from(CStr::from_ptr(
//        env.get_string(j_recipient).unwrap().as_ptr(),
//    ));
//
//    let output = env
//        .new_string(
//            "Yesss, I am running inside Rust, Yay!!!!, Hello ".to_owned()
//                + recipient.to_str().unwrap(),
//        )
//        .unwrap();
//
//    //    tokio_main();
//
//    let a = env.call_method(obj, "hi", "()V", &[]);
//    if let Err(err) = a {
//        info!("{:?}", err.kind());
//    }
//
//    output.into_inner()
//}
//
//fn init_logger() {
//    android_logger::init_once(
//        Config::default()
//            .with_min_level(Level::Info)
//            .with_tag("rust"),
//    );
//}
//
//pub fn tokio_main() {
//    START.call_once(|| {
//        init_logger();
//
//        info!("Thread id3: {:?}", std::thread::current().id());
//
//        thread::spawn(|| {
//            let mut basic_rt = runtime::Builder::new()
//                .basic_scheduler()
//                .enable_all()
//                .build()
//                .unwrap();
//            basic_rt.block_on(run());
//        });
//        info!("Initialized10");
//    });
//}
//
//pub async fn run() {
//    for i in 1..100 {
//        info!("Thread id: {:?} {}", std::thread::current().id(), i);
//        time::delay_for(Duration::from_secs(1)).await;
//    }
//}
