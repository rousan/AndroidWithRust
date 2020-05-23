#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;
use crate::bridge::{Bridge, EventData};
use lazy_static::lazy_static;

mod bridge;
mod logger;

lazy_static! {
    static ref BRIDGE: Bridge = Bridge::new(on_init)
        .on("ping", |data| async move {
            info!("Got an event");
        })
        .on("ping2", |data| async move {
            info!("Got an event");
        })
        .on("ping3", |data| async move {
            info!("Got an event");
        });
}

async fn on_init() {
    logger::init();

    info!("Hello world");
}
