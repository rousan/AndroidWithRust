use android_logger::Config;
use log::Level;
use std::sync::Once;

static START: Once = Once::new();

pub fn init() {
    START.call_once(|| {
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Info)
                .with_tag("rust"),
        );
    });
}
