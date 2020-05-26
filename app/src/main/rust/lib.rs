#![cfg(target_os = "android")]

pub use error::{Error, ErrorExt, ResultExt};

#[macro_use]
mod logger;

mod bridge;
mod counter;
mod error;
mod message;
mod prelude;

pub type Result<T> = std::result::Result<T, Error>;
