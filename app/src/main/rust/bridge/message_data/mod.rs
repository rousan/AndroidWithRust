use crate::bridge::ext::JavaVMExt;
use crate::ResultExt;
use jni::objects::{JObject, JString};
use jni::{
    objects::{GlobalRef, JValue},
    JNIEnv,
};

pub struct MessageData {
    inner: GlobalRef,
}

impl MessageData {
    pub fn new() -> MessageData {
        let env = crate::bridge::jvm().env();
        let bundle = env.new_object("android/os/Bundle", "()V", &[]).unwrap();

        MessageData {
            inner: env.new_global_ref(bundle).unwrap(),
        }
    }

    pub fn wrap(env: &JNIEnv, bundle: JObject) -> MessageData {
        MessageData {
            inner: env.new_global_ref(bundle).unwrap(),
        }
    }

    pub fn empty() -> MessageData {
        MessageData::new()
    }

    /// Drops the internal `GlobalRef` after attaching the current thread to the JVM.
    pub fn drop(self) {
        crate::bridge::jvm().attach_thread();
    }

    pub fn put_string<K: AsRef<str>, V: AsRef<str>>(self, key: K, value: V) -> Self {
        let env = crate::bridge::jvm().env();

        env.call_method(
            &self.inner,
            "putString",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[
                JValue::Object(env.new_string(key).unwrap().into()),
                JValue::Object(env.new_string(value).unwrap().into()),
            ],
        )
        .unwrap();

        self
    }

    pub fn get_string<K: AsRef<str>>(&self, key: K) -> Option<String> {
        let env = crate::bridge::jvm().env();

        let value = env
            .call_method(
                &self.inner,
                "getString",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[JValue::Object(env.new_string(key).unwrap().into())],
            )
            .unwrap();

        match value {
            JValue::Object(obj) => match Self::convert_java_string_to_rust_string(&env, JString::from(obj)) {
                Ok(s) => Some(s),
                Err(err) => {
                    warn!("{}", err);
                    None
                }
            },
            _ => panic!("Failed to query value as string from MessageData in `get_string` method"),
        }
    }

    pub fn bundle(self) -> GlobalRef {
        self.inner
    }

    fn convert_java_string_to_rust_string(env: &JNIEnv, j_string: JString) -> crate::Result<String> {
        let j_string = env
            .get_string(j_string)
            .context("Couldn't convert java script to rust string")?;

        Ok(String::from(j_string.to_owned()))
    }
}
