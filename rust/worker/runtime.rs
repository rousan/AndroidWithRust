use lazy_static::lazy_static;
use std::future::Future;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(runtime_core_threads_count())
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
}

pub fn start_tokio_runtime() {
    info!(
        "Rust: Starting tokio runtime from thread id: {:?}, System: Logical cores: {}, Physical cores: {}, Tokio Runtime core threads: {}",
        std::thread::current().id(),
        num_cpus::get(),
        num_cpus::get_physical(),
        runtime_core_threads_count()
    );
    lazy_static::initialize(&TOKIO_RUNTIME);
}

pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    TOKIO_RUNTIME.spawn(task)
}

fn runtime_core_threads_count() -> usize {
    num_cpus::get().min(4).max(2)
}
