use android_logger::Config;
use log::Level;

pub fn init() {
    android_logger::init_once(Config::default().with_min_level(Level::Info).with_tag("rust"));
}
