use jni::objects::JString;
use jni::JNIEnv;

pub fn convert_java_string_to_rust_string(env: &JNIEnv, j_string: JString) -> Option<String> {
    let j_string = match env.get_string(j_string) {
        Ok(s) => s,
        Err(_) => {
            return None;
        }
    };

    Some(String::from(j_string.to_owned()))
}
