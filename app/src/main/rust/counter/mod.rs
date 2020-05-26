use crate::bridge::{self, MessageData};
use crate::message::what;

pub async fn on_initiate() {
    let data = MessageData::new().put_string("value", "10");
    bridge::send_message(what::COUNTER_VALUE, data);
}

pub async fn on_increase_counter() {}

pub async fn on_decrease_counter() {}
