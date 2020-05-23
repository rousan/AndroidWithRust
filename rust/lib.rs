#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;

mod jni;
mod logger;
mod worker;
