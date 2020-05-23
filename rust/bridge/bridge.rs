use crate::bridge::EventData;
use std::collections::HashMap;
use std::future::Future;

type OnInitCallback = Box<dyn Fn() -> OnInitCallbackReturn + Send + Sync + 'static>;
type OnInitCallbackReturn = Box<dyn Future<Output = ()> + Send + 'static>;

type OnEventListener = Box<dyn Fn(EventData) -> OnEventListenerReturn + Send + Sync + 'static>;
type OnEventListenerReturn = Box<dyn Future<Output = ()> + Send + 'static>;

pub struct Bridge {
    on_init_callback: OnInitCallback,
    listeners: HashMap<String, OnEventListener>,
}

impl Bridge {
    pub fn new<F, R>(on_init_callback: F) -> Bridge
    where
        F: Fn() -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        Bridge {
            on_init_callback: Box::new(move || Box::new(on_init_callback())),
            listeners: Default::default(),
        }
    }

    pub fn on<N, F, R>(mut self, event_name: N, listener: F) -> Self
    where
        N: Into<String>,
        F: Fn(EventData) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        let listener: OnEventListener = Box::new(move |data: EventData| Box::new(listener(data)));
        self.listeners.insert(event_name.into(), listener);
        self
    }

    pub fn emit<N>(&self, event_name: N, data: EventData)
    where
        N: Into<String>,
    {
        todo!()
    }
}
