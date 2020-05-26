use crate::bridge::{self, MessageData};
use crate::message::what;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<i64> = Mutex::new(0);
}

pub async fn on_initiate() {
    let lock = COUNTER.lock().await;
    let value = *lock;
    publish_counter_value(value).await;
}

pub async fn on_increase_counter() {
    let mut lock = COUNTER.lock().await;
    *lock += 1;
    let value = *lock;
    publish_counter_value(value).await;
}

pub async fn on_decrease_counter() {
    let mut lock = COUNTER.lock().await;
    *lock -= 1;
    let value = *lock;
    publish_counter_value(value).await;
}

pub async fn publish_counter_value(value: i64) {
    let data = MessageData::new().put_string("value", value.to_string());
    bridge::send_message(what::COUNTER_VALUE, data);
}
