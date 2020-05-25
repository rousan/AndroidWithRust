use android_logger::Config;
use log::Level;

pub fn init() {
    android_logger::init_once(Config::default().with_min_level(Level::Info).with_tag("my-app"));
}

#[macro_export]
macro_rules! error {
  ($($args:tt)*) => {{
    log::error!("Rust: {}", format!($($args)*));
  }}
}

#[macro_export]
macro_rules! warn {
  ($($args:tt)*) => {{
    log::warn!("Rust: {}", format!($($args)*));
  }}
}

#[macro_export]
macro_rules! info {
  ($($args:tt)*) => {{
    log::info!("Rust: {}", format!($($args)*));
  }}
}

#[macro_export]
macro_rules! debug {
  ($($args:tt)*) => {{
    log::debug!("Rust: {}", format!($($args)*));
  }}
}

#[macro_export]
macro_rules! trace {
  ($($args:tt)*) => {{
    log::trace!("Rust: {}", format!($($args)*));
  }}
}
