use lazy_static::lazy_static;
use std::future::Future;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();
}

pub fn start_tokio_runtime() {
    info!(
        "Starting tokio runtime from thread id: {:?}",
        std::thread::current().id()
    );
    lazy_static::initialize(&RUNTIME);
}

pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    RUNTIME.spawn(task)
}
