use crate::prelude::*;
use jni::objects::JString;
use jni::JNIEnv;

pub fn convert_java_string_to_rust_string(env: &JNIEnv, j_string: JString) -> crate::Result<String> {
    let j_string = env
        .get_string(j_string)
        .context("Couldn't convert java script to rust string")?;

    Ok(String::from(j_string.to_owned()))
}
