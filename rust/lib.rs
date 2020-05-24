#![cfg(target_os = "android")]

#[macro_use]
extern crate log;

pub use error::{Error, ErrorExt, ResultExt};

mod error;
mod ext;
mod helpers;
mod jni;
mod logger;
mod message;
mod prelude;
mod worker;

pub type Result<T> = std::result::Result<T, Error>;
