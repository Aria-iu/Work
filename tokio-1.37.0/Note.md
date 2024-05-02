##
最小需要feature "rt"，rt提供了 tokio::spawn 等，此时tokio::task才会被启用

"net"提供了 TcpStream等

"sync"提供了同步类型，设置时tokio::sync才会被启用

"time"提供了记录时间的功能，设置时tokio::time才会被启用

tokio提供了一个runtime来执行异步任务。

大多应用可以使用#[tokio::main]这个宏来使代码在异步运行时中运行。
这个宏只会提供基本的设置选项。tokio::runtime提供了丰富的API来设置和管理运行时。
若这个宏无法满足需求，使用tokio::runtime的内容。

"rt"或者"rt-multi-thread"属性设置时，分别启用了current-thread single-threaded scheduler 和 the multi-thread scheduler。

"macro"设置时会启用#[tokio::main] 和 #[tokio::test] 标签。

tokio::io 模块提供了Tokio的异步核心IO基元，AsyncRead、AsyncWrite 和 AsyncBufRead 特征。

Tokio还包括用于执行各种类型的IO和异步与操作系统交互的API。这些包括：

- tokio::net，其中包含非阻塞版本的TCP、UDP和Unix域套接字（通过启用 "net" 特性标志），
- tokio::fs，类似于 std::fs，但用于执行异步文件系统IO（通过启用 "fs" 特性标志），
- tokio::signal，用于异步处理Unix和Windows操作系统信号（通过启用 "signal" 特性标志），
- tokio::process，用于生成和管理子进程（通过启用 "process" 特性标志）。

## features
如果您是Tokio的新手，建议您使用完整的特性标志，这将启用所有公共API。但请注意，这将引入许多您可能不需要的额外依赖项。

- full: 启用下面列出的所有特性，但不包括 test-util 和 tracing。
- rt: 启用 tokio::spawn、当前线程调度器和非调度器实用程序。
- rt-multi-thread: 启用更重的、多线程的、工作窃取调度器。
- io-util: 启用基于IO的Ext特性。
- io-std: 启用 Stdout、Stdin 和 Stderr 类型。
- net: 启用 tokio::net 类型，如 TcpStream、UnixStream 和 UdpSocket，以及（在类Unix系统上）AsyncFd 和（在 FreeBSD 上）PollAio。
- time: 启用 tokio::time 类型，并允许调度器启用内置计时器。
- process: 启用 tokio::process 类型。
- macros: 启用 #[tokio::main] 和 #[tokio::test] 宏。
- sync: 启用所有 tokio::sync 类型。
- signal: 启用所有 tokio::signal 类型。
- fs: 启用 tokio::fs 类型。
- test-util: 启用基于测试的基础设施，用于 Tokio 运行时。
- parking_lot: 作为潜在的优化，使用 parking_lot crate 的同步原语内部。此外，此依赖项对于在 const 上下文中构造一些我们的原语是必需的。根据使用的 parking_lot 发布版本，最小支持Rust版本（MSRV）可能会有所增加。

AsyncRead 和 AsyncWrite 特性不需要任何特性，并且始终可用。

# tokio::runtime
需要feature “rt”

与其他 Rust 程序不同，异步应用程序需要运行时支持。特别是需要以下运行时服务：

- I/O 事件循环，称为驱动程序，用于驱动 I/O 资源并将 I/O 事件分派给依赖它们的任务。
- 一个调度器，用于执行使用这些 I/O 资源的任务。
- 一个定时器，用于安排在一段时间后运行的工作。

Tokio 的 Runtime 将所有这些服务捆绑为单个类型，允许它们一起启动、关闭和配置。然而，通常不需要手动配置 Runtime，用户可以简单地使用 tokio::main 属性宏，该宏在幕后创建一个 Runtime。

资源驱动程序

当手动配置运行时时，默认情况下不启用任何资源驱动程序。在这种情况下，尝试使用网络类型或时间类型将失败。为了启用这些类型，必须启用资源驱动程序。可以通过 Builder::enable_io 和 Builder::enable_time 来实现。作为一种简便方法，Builder::enable_all 启用了所有资源驱动程序。

```rust
#[derive(Debug)]
pub struct Runtime {
    /// Task scheduler
    scheduler: Scheduler,

    /// Handle to runtime, also contains driver handles
    handle: Handle,

    /// Blocking pool handle, used to signal shutdown
    blocking_pool: BlockingPool,
}
```
