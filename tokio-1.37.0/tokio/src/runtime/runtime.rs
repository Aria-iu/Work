use crate::runtime::blocking::BlockingPool;
use crate::runtime::scheduler::CurrentThread;
use crate::runtime::{context, EnterGuard, Handle};
use crate::task::JoinHandle;

use std::future::Future;
use std::time::Duration;

cfg_rt_multi_thread! {
    use crate::runtime::Builder;
    use crate::runtime::scheduler::MultiThread;

    cfg_unstable! {
        use crate::runtime::scheduler::MultiThreadAlt;
    }
}

/// The Tokio runtime.
///
/// The runtime provides an I/O driver, task scheduler, [timer], and
/// blocking pool, necessary for running asynchronous tasks.
///
/// 这些组件是运行异步任务所必需的，因为它们提供了异步任务运行所需的基本功能和支持：
///
/// * I/O 驱动程序：异步任务通常涉及 I/O 操作，如读取文件、网络通信等。I/O 驱动程序负责管理和调度这些异步 I/O 操作，确保它们能够以非阻塞的方式进行。
/// * 任务调度器：异步任务需要在执行时被调度和管理。任务调度器负责将任务分配给可用的线程或执行上下文，并确保它们在适当的时候执行。
/// * 定时器：许多异步任务需要在一定时间间隔内执行某些操作，或者在未来的特定时间点执行。定时器提供了管理和触发这些时间相关的操作的功能。
/// * 阻塞池：尽管异步任务旨在避免阻塞，但某些情况下可能仍然需要执行阻塞式操作，如调用同步 API 或执行计算密集型任务。阻塞池允许在异步环境中执行这些阻塞式操作，确保它们不会阻塞整个应用程序的执行。
///
/// 因此，这些组件是构建异步任务运行时环境所必需的，它们一起提供了支持和基础设施，使得异步编程能够高效、灵活地进行。
///
/// Instances of `Runtime` can be created using [`new`], or [`Builder`].
/// However, most users will use the [`#[tokio::main]`][main] annotation on
/// their entry point instead.
///
/// See [module level][mod] documentation for more details.
///
/// # Shutdown
///
/// Shutting down the runtime is done by dropping the value, or calling
/// [`shutdown_background`] or [`shutdown_timeout`].
///
/// Tasks spawned through [`Runtime::spawn`] keep running until they yield.
/// Then they are dropped. They are not *guaranteed* to run to completion, but
/// *might* do so if they do not yield until completion.
///
/// Blocking functions spawned through [`Runtime::spawn_blocking`] keep running
/// until they return.
///
/// The thread initiating the shutdown blocks until all spawned work has been
/// stopped. This can take an indefinite amount of time. The `Drop`
/// implementation waits forever for this.
///
/// The [`shutdown_background`] and [`shutdown_timeout`] methods can be used if
/// waiting forever is undesired. When the timeout is reached, spawned work that
/// did not stop in time and threads running it are leaked. The work continues
/// to run until one of the stopping conditions is fulfilled, but the thread
/// initiating the shutdown is unblocked.
///
/// Once the runtime has been dropped, any outstanding I/O resources bound to
/// it will no longer function. Calling any method on them will result in an
/// error.
///
/// # Sharing
///
/// There are several ways to establish shared access to a Tokio runtime:
///
///  * Using an <code>[Arc]\<Runtime></code>.
///  * Using a [`Handle`].
///  * Entering the runtime context.
///
/// Using an <code>[Arc]\<Runtime></code> or [`Handle`] allows you to do various
/// things with the runtime such as spawning new tasks or entering the runtime
/// context. Both types can be cloned to create a new handle that allows access
/// to the same runtime. By passing clones into different tasks or threads, you
/// will be able to access the runtime from those tasks or threads.
///
/// The difference between <code>[Arc]\<Runtime></code> and [`Handle`] is that
/// an <code>[Arc]\<Runtime></code> will prevent the runtime from shutting down,
/// whereas a [`Handle`] does not prevent that. This is because shutdown of the
/// runtime happens when the destructor of the `Runtime` object runs.
///
/// Calls to [`shutdown_background`] and [`shutdown_timeout`] require exclusive
/// ownership of the `Runtime` type. When using an <code>[Arc]\<Runtime></code>,
/// this can be achieved via [`Arc::try_unwrap`] when only one strong count
/// reference is left over.
///
/// The runtime context is entered using the [`Runtime::enter`] or
/// [`Handle::enter`] methods, which use a thread-local variable to store the
/// current runtime. Whenever you are inside the runtime context, methods such
/// as [`tokio::spawn`] will use the runtime whose context you are inside.
///
/// [timer]: crate::time
/// [mod]: index.html
/// [`new`]: method@Self::new
/// [`Builder`]: struct@Builder
/// [`Handle`]: struct@Handle
/// [main]: macro@crate::main
/// [`tokio::spawn`]: crate::spawn
/// [`Arc::try_unwrap`]: std::sync::Arc::try_unwrap
/// [Arc]: std::sync::Arc
/// [`shutdown_background`]: method@Runtime::shutdown_background
/// [`shutdown_timeout`]: method@Runtime::shutdown_timeout
#[derive(Debug)]
pub struct Runtime {
    /// Task scheduler
    /// 这个字段存储着 Tokio 运行时的任务调度器，负责管理和调度所有注册的异步任务。
    /// 调度器负责决定哪个任务在何时执行，以及如何分配系统资源，是整个 Tokio 运行时的核心组件之一。
    scheduler: Scheduler,

    /// Handle to runtime, also contains driver handles
    /// 这个字段是运行时的句柄，提供了对运行时的访问和控制接口。
    /// 其中包含了对驱动程序的句柄，用于管理底层资源，以及阻塞池的句柄，用于管理阻塞任务的执行。
    /// 通过这个句柄，用户可以与运行时进行交互，如创建任务、控制任务的执行等。
    handle: Handle,

    /// Blocking pool handle, used to signal shutdown
    /// 这个字段存储着阻塞池的句柄，用于管理阻塞任务的执行。
    /// 在异步编程中，阻塞任务会阻塞当前线程的执行，为了避免整个系统被阻塞，通常会将阻塞任务交给专门的线程池来执行。
    /// 阻塞池负责管理这些阻塞任务的执行，确保它们能够在独立的线程上执行而不会阻塞整个系统的其他任务。
    blocking_pool: BlockingPool,
}

/// The flavor of a `Runtime`.
///
/// This is the return type for [`Handle::runtime_flavor`](crate::runtime::Handle::runtime_flavor()).
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RuntimeFlavor {
    /// The flavor that executes all tasks on the current thread.
    CurrentThread,
    /// The flavor that executes tasks across multiple threads.
    MultiThread,
    /// The flavor that executes tasks across multiple threads.
    #[cfg(tokio_unstable)]
    MultiThreadAlt,
}

/// The runtime scheduler is either a multi-thread or a current-thread executor.
///
/// 多线程执行器和当前线程执行器在运行时调度任务时有一些重要区别：
///
/// 1. **多线程执行器**：
///    - 在多线程执行器中，任务可以在多个线程上并发执行。
///    - 这种执行器通常会创建一个线程池，任务被分配给线程池中的空闲线程来执行。
///    - 由于任务可以并发执行，因此适用于需要大量并发性的场景，可以充分利用多核处理器的优势。
///    - 多线程执行器通常会自动管理线程的生命周期和资源分配。
///
/// 2. **当前线程执行器**：
///    - 在当前线程执行器中，所有任务都在同一个线程上执行。
///    - 这意味着任务是顺序执行的，一个任务必须等待另一个任务完成后才能开始执行。
///    - 当前线程执行器通常用于单线程应用程序或者对并发性要求不高的场景。
///    - 由于任务在同一个线程上执行，因此不需要线程间的同步和通信，可以减少线程切换的开销和复杂性。
///
/// 总的来说，多线程执行器适用于需要大量并发性和并行性的场景，而当前线程执行器适用于对并发性要求不高或者需要简单设计的场景。选择使用哪种执行器取决于应用程序的需求和性能目标。
///
#[derive(Debug)]
pub(super) enum Scheduler {
    /// Execute all tasks on the current-thread.
    CurrentThread(CurrentThread),

    /// Execute tasks across multiple threads.
    #[cfg(all(feature = "rt-multi-thread", not(target_os = "wasi")))]
    MultiThread(MultiThread),

    /// Execute tasks across multiple threads.
    #[cfg(all(tokio_unstable, feature = "rt-multi-thread", not(target_os = "wasi")))]
    MultiThreadAlt(MultiThreadAlt),
}

impl Runtime {
    pub(super) fn from_parts(
        scheduler: Scheduler,
        handle: Handle,
        blocking_pool: BlockingPool,
    ) -> Runtime {
        Runtime {
            scheduler,
            handle,
            blocking_pool,
        }
    }

    cfg_not_wasi! {
        /// Creates a new runtime instance with default configuration values.
        ///
        /// This results in the multi threaded scheduler, I/O driver, and time driver being
        /// initialized.
        ///
        /// Most applications will not need to call this function directly. Instead,
        /// they will use the  [`#[tokio::main]` attribute][main]. When a more complex
        /// configuration is necessary, the [runtime builder] may be used.
        ///
        /// See [module level][mod] documentation for more details.
        ///
        /// # Examples
        ///
        /// Creating a new `Runtime` with default configuration values.
        ///
        /// ```
        /// use tokio::runtime::Runtime;
        ///
        /// let rt = Runtime::new()
        ///     .unwrap();
        ///
        /// // Use the runtime...
        /// ```
        ///
        /// [mod]: index.html
        /// [main]: ../attr.main.html
        /// [threaded scheduler]: index.html#threaded-scheduler
        /// [runtime builder]: crate::runtime::Builder
        #[cfg(feature = "rt-multi-thread")]
        #[cfg_attr(docsrs, doc(cfg(feature = "rt-multi-thread")))]
        pub fn new() -> std::io::Result<Runtime> {
            Builder::new_multi_thread().enable_all().build()
        }
    }

    /// Returns a handle to the runtime's spawner.
    ///
    /// The returned handle can be used to spawn tasks that run on this runtime, and can
    /// be cloned to allow moving the `Handle` to other threads.
    ///
    /// Calling [`Handle::block_on`] on a handle to a `current_thread` runtime is error-prone.
    /// Refer to the documentation of [`Handle::block_on`] for more.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// let rt = Runtime::new()
    ///     .unwrap();
    ///
    /// let handle = rt.handle();
    ///
    /// // Use the handle...
    /// ```
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Spawns a future onto the Tokio runtime.
    ///
    /// This spawns the given future onto the runtime's executor, usually a
    /// thread pool. The thread pool is then responsible for polling the future
    /// until it completes.
    ///
    /// The provided future will start running in the background immediately
    /// when `spawn` is called, even if you don't await the returned
    /// `JoinHandle`.
    ///
    /// See [module level][mod] documentation for more details.
    ///
    /// [mod]: index.html
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// # fn dox() {
    /// // Create the runtime
    /// let rt = Runtime::new().unwrap();
    ///
    /// // Spawn a future onto the runtime
    /// rt.spawn(async {
    ///     println!("now running on a worker thread");
    /// });
    /// # }
    /// ```
    #[track_caller]
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.handle.spawn(future)
    }

    /// Runs the provided function on an executor dedicated to blocking operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// # fn dox() {
    /// // Create the runtime
    /// let rt = Runtime::new().unwrap();
    ///
    /// // Spawn a blocking function onto the runtime
    /// rt.spawn_blocking(|| {
    ///     println!("now running on a worker thread");
    /// });
    /// # }
    /// ```
    #[track_caller]
    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.handle.spawn_blocking(func)
    }

    /// Runs a future to completion on the Tokio runtime. This is the
    /// runtime's entry point.
    ///
    /// This runs the given future on the current thread, blocking until it is
    /// complete, and yielding its resolved result. Any tasks or timers
    /// which the future spawns internally will be executed on the runtime.
    ///
    /// # Non-worker future
    ///
    /// Note that the future required by this function does not run as a
    /// worker. The expectation is that other tasks are spawned by the future here.
    /// Awaiting on other futures from the future provided here will not
    /// perform as fast as those spawned as workers.
    ///
    /// # Multi thread scheduler
    ///
    /// When the multi thread scheduler is used this will allow futures
    /// to run within the io driver and timer context of the overall runtime.
    ///
    /// Any spawned tasks will continue running after `block_on` returns.
    ///
    /// # Current thread scheduler
    ///
    /// When the current thread scheduler is enabled `block_on`
    /// can be called concurrently from multiple threads. The first call
    /// will take ownership of the io and timer drivers. This means
    /// other threads which do not own the drivers will hook into that one.
    /// When the first `block_on` completes, other threads will be able to
    /// "steal" the driver to allow continued execution of their futures.
    ///
    /// Any spawned tasks will be suspended after `block_on` returns. Calling
    /// `block_on` again will resume previously spawned tasks.
    ///
    /// # Panics
    ///
    /// This function panics if the provided future panics, or if called within an
    /// asynchronous execution context.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::runtime::Runtime;
    ///
    /// // Create the runtime
    /// let rt  = Runtime::new().unwrap();
    ///
    /// // Execute the future, blocking the current thread until completion
    /// rt.block_on(async {
    ///     println!("hello");
    /// });
    /// ```
    ///
    /// [handle]: fn@Handle::block_on
    #[track_caller]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        #[cfg(all(
            tokio_unstable,
            tokio_taskdump,
            feature = "rt",
            target_os = "linux",
            any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
        ))]
        let future = super::task::trace::Trace::root(future);

        #[cfg(all(tokio_unstable, feature = "tracing"))]
        let future = crate::util::trace::task(
            future,
            "block_on",
            None,
            crate::runtime::task::Id::next().as_u64(),
        );

        let _enter = self.enter();

        match &self.scheduler {
            Scheduler::CurrentThread(exec) => exec.block_on(&self.handle.inner, future),
            #[cfg(all(feature = "rt-multi-thread", not(target_os = "wasi")))]
            Scheduler::MultiThread(exec) => exec.block_on(&self.handle.inner, future),
            #[cfg(all(tokio_unstable, feature = "rt-multi-thread", not(target_os = "wasi")))]
            Scheduler::MultiThreadAlt(exec) => exec.block_on(&self.handle.inner, future),
        }
    }

    /// Enters the runtime context.
    ///
    /// This allows you to construct types that must have an executor
    /// available on creation such as [`Sleep`] or [`TcpStream`]. It will
    /// also allow you to call methods such as [`tokio::spawn`].
    ///
    /// [`Sleep`]: struct@crate::time::Sleep
    /// [`TcpStream`]: struct@crate::net::TcpStream
    /// [`tokio::spawn`]: fn@crate::spawn
    ///
    /// # Example
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    /// use tokio::task::JoinHandle;
    ///
    /// fn function_that_spawns(msg: String) -> JoinHandle<()> {
    ///     // Had we not used `rt.enter` below, this would panic.
    ///     tokio::spawn(async move {
    ///         println!("{}", msg);
    ///     })
    /// }
    ///
    /// fn main() {
    ///     let rt = Runtime::new().unwrap();
    ///
    ///     let s = "Hello World!".to_string();
    ///
    ///     // By entering the context, we tie `tokio::spawn` to this executor.
    ///     let _guard = rt.enter();
    ///     let handle = function_that_spawns(s);
    ///
    ///     // Wait for the task before we end the test.
    ///     rt.block_on(handle).unwrap();
    /// }
    /// ```
    pub fn enter(&self) -> EnterGuard<'_> {
        self.handle.enter()
    }

    /// Shuts down the runtime, waiting for at most `duration` for all spawned
    /// work to stop.
    ///
    /// See the [struct level documentation](Runtime#shutdown) for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    /// use tokio::task;
    ///
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///    let runtime = Runtime::new().unwrap();
    ///
    ///    runtime.block_on(async move {
    ///        task::spawn_blocking(move || {
    ///            thread::sleep(Duration::from_secs(10_000));
    ///        });
    ///    });
    ///
    ///    runtime.shutdown_timeout(Duration::from_millis(100));
    /// }
    /// ```
    pub fn shutdown_timeout(mut self, duration: Duration) {
        // Wakeup and shutdown all the worker threads
        self.handle.inner.shutdown();
        self.blocking_pool.shutdown(Some(duration));
    }

    /// Shuts down the runtime, without waiting for any spawned work to stop.
    ///
    /// This can be useful if you want to drop a runtime from within another runtime.
    /// Normally, dropping a runtime will block indefinitely for spawned blocking tasks
    /// to complete, which would normally not be permitted within an asynchronous context.
    /// By calling `shutdown_background()`, you can drop the runtime from such a context.
    ///
    /// Note however, that because we do not wait for any blocking tasks to complete, this
    /// may result in a resource leak (in that any blocking tasks are still running until they
    /// return.
    ///
    /// See the [struct level documentation](Runtime#shutdown) for more details.
    ///
    /// This function is equivalent to calling `shutdown_timeout(Duration::from_nanos(0))`.
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// fn main() {
    ///    let runtime = Runtime::new().unwrap();
    ///
    ///    runtime.block_on(async move {
    ///        let inner_runtime = Runtime::new().unwrap();
    ///        // ...
    ///        inner_runtime.shutdown_background();
    ///    });
    /// }
    /// ```
    pub fn shutdown_background(self) {
        self.shutdown_timeout(Duration::from_nanos(0));
    }
}

#[allow(clippy::single_match)] // there are comments in the error branch, so we don't want if-let
impl Drop for Runtime {
    fn drop(&mut self) {
        match &mut self.scheduler {
            Scheduler::CurrentThread(current_thread) => {
                // This ensures that tasks spawned on the current-thread
                // runtime are dropped inside the runtime's context.
                let _guard = context::try_set_current(&self.handle.inner);
                current_thread.shutdown(&self.handle.inner);
            }
            #[cfg(all(feature = "rt-multi-thread", not(target_os = "wasi")))]
            Scheduler::MultiThread(multi_thread) => {
                // The threaded scheduler drops its tasks on its worker threads, which is
                // already in the runtime's context.
                multi_thread.shutdown(&self.handle.inner);
            }
            #[cfg(all(tokio_unstable, feature = "rt-multi-thread", not(target_os = "wasi")))]
            Scheduler::MultiThreadAlt(multi_thread) => {
                // The threaded scheduler drops its tasks on its worker threads, which is
                // already in the runtime's context.
                multi_thread.shutdown(&self.handle.inner);
            }
        }
    }
}

impl std::panic::UnwindSafe for Runtime {}

impl std::panic::RefUnwindSafe for Runtime {}

cfg_metrics! {
    impl Runtime {
        /// TODO
        pub fn metrics(&self) -> crate::runtime::RuntimeMetrics {
            self.handle.metrics()
        }
    }
}
