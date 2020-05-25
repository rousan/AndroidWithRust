use jni::{JNIEnv, JavaVM};

pub trait JavaVMExt {
    fn env(&self) -> JNIEnv;
}

impl JavaVMExt for JavaVM {
    fn env(&self) -> JNIEnv {
        self.attach_current_thread_as_daemon().unwrap()
    }
}
