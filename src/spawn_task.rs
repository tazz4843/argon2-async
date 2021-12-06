#[cfg(feature = "any-rt")]
use once_cell::sync::OnceCell;
#[cfg(feature = "any-rt")]
use threadpool::ThreadPool;
use tokio::sync::oneshot;

#[cfg(feature = "any-rt")]
static THREAD_POOL: OnceCell<ThreadPool> = OnceCell::new();

pub fn spawn_task<T, F>(fut: F) -> oneshot::Receiver<T>
where
    T: 'static + Send,
    F: FnOnce(oneshot::Sender<T>) -> () + Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    let job = || fut(tx);

    #[cfg(feature = "async-std-rt")]
    async_std::task::spawn_blocking(job);

    #[cfg(feature = "any-rt")]
    {
        let pool = THREAD_POOL.get_or_init(|| threadpool::ThreadPool::default());
        pool.execute(job);
    }

    #[cfg(feature = "tokio-rt")]
    {
        tokio::task::spawn_blocking(job);
    }

    rx
}
