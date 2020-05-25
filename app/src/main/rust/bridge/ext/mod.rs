use jni::{JNIEnv, JavaVM};

pub trait JavaVMExt {
    /// Gets the env instance.
    fn env(&self) -> JNIEnv;

    /// Attaches the current thread to the JVM.
    fn attach_thread(&self);
}

impl JavaVMExt for JavaVM {
    fn env(&self) -> JNIEnv {
        self.attach_current_thread_as_daemon().unwrap()
    }

    fn attach_thread(&self) {
        self.env();
    }
}
