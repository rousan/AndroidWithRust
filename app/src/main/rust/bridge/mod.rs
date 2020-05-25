use self::ext::JavaVMExt;
use crate::logger;
use jni::{
    objects::{GlobalRef, JObject, JValue},
    JNIEnv, JavaVM,
};
use std::panic;
use std::sync::Once;

pub use message_data::MessageData;

mod ext;
mod jni_impl;
mod message_data;
mod runtime;

static mut INIT_BRIDGE_ONCE: bool = false;
static mut JVM: Option<JavaVM> = None;
static mut JNI_BRIDGE: Option<GlobalRef> = None;
static STARTUP_ONCE: Once = Once::new();

/// Calls this only once in app's lifetime.
fn startup_once() {
    STARTUP_ONCE.call_once(|| {
        logger::init();
        panic::set_hook(Box::new(|info| error!("{}", info.to_string())));
    });
}

/// Calls whenever the bridge is started or restarted.
fn start_bridge(env: JNIEnv, jni_bridge: JObject) {
    unsafe {
        if let Some(jni_bridge) = JNI_BRIDGE.take() {
            drop(jni_bridge);
        }

        if let Some(jvm) = JVM.take() {
            drop(jvm);
        }

        JVM.replace(env.get_java_vm().unwrap());
        JNI_BRIDGE.replace(env.new_global_ref(jni_bridge).unwrap());

        if !INIT_BRIDGE_ONCE {
            startup_once();

            runtime::start_runtime();
            runtime::spawn(async move {
                crate::message::on_start().await;
            });

            INIT_BRIDGE_ONCE = true;

            info!("The bridge is initialized");
        }
    }
}

/// Calls whenever the bridge is shut down.
fn shutdown_bridge(_: JNIEnv, _: JObject) {
    unsafe {
        if let Some(jni_bridge) = JNI_BRIDGE.take() {
            drop(jni_bridge);
        }

        if let Some(jvm) = JVM.take() {
            drop(jvm);
        }

        runtime::spawn(async move {
            crate::message::on_destroy().await;
        });
        runtime::shutdown_runtime();

        INIT_BRIDGE_ONCE = false;
    }
}

/// Handles the incoming messages from Java end.
fn handle_message(_: JNIEnv, _: JObject, what: i32, data: MessageData) {
    runtime::spawn(async move {
        crate::message::on_message(what, data).await;
    });
}

/// Sends a message to the Java end.
pub fn send_message(what: i32, data: MessageData) {
    let env = jvm().env();
    env.call_method(
        jni_bridge(),
        "sendMessageToJava",
        "(ILandroid/os/Bundle;)V",
        &[what.into(), JValue::Object(data.bundle().as_obj())],
    )
    .unwrap();
}

fn jvm() -> &'static JavaVM {
    unsafe { JVM.as_ref().unwrap() }
}

fn jni_bridge() -> &'static GlobalRef {
    unsafe { JNI_BRIDGE.as_ref().unwrap() }
}
