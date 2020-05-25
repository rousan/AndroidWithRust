#![cfg(target_os = "android")]

pub use error::{Error, ErrorExt, ResultExt};

#[macro_use]
mod logger;

mod bridge;
mod error;
mod message;
mod prelude;
mod transfer;

pub type Result<T> = std::result::Result<T, Error>;
