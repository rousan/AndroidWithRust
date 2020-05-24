use crate::helpers;
use jni::objects::{JObject, JString};
use jni::{
    objects::{GlobalRef, JValue},
    JNIEnv,
};

pub struct MessageData {
    inner: GlobalRef,
}

impl MessageData {
    pub fn new(env: &JNIEnv) -> MessageData {
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

    pub fn empty(env: &JNIEnv) -> MessageData {
        MessageData::new(env)
    }

    pub fn put_string<K: AsRef<str>, V: AsRef<str>>(self, env: &JNIEnv, key: K, value: V) -> Self {
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

    pub fn get_string<K: AsRef<str>>(&self, env: &JNIEnv, key: K) -> Option<String> {
        let value = env
            .call_method(
                &self.inner,
                "getString",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[JValue::Object(env.new_string(key).unwrap().into())],
            )
            .unwrap();

        match value {
            JValue::Object(obj) => match helpers::convert_java_string_to_rust_string(&env, JString::from(obj)) {
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
}
