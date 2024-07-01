#[cfg(all(not(target_os = "linux"), feature = "io-uring"))]
compile_error!("io_uring is a linux only feature.");

use std::future::Future;

// Cannot define a main macro when compiled into test harness.
// Workaround for https://github.com/rust-lang/rust/issues/62127.
#[cfg(all(feature = "macros", not(test)))]
pub use actix_macros::main;
#[cfg(feature = "macros")]
pub use actix_macros::test;

mod arbiter;
mod runtime;
mod system;

pub use tokio::pin;
use tokio::task::JoinHandle;

pub use self::{
    arbiter::{Arbiter, ArbiterHandle},
    runtime::Runtime,
    system::{System, SystemRunner},
};

pub mod signal {
    //! Asynchronous signal handling (Tokio re-exports).

    #[cfg(unix)]
    pub mod unix {
        //! Unix specific signals (Tokio re-exports).
        pub use tokio::signal::unix::*;
    }
    pub use tokio::signal::ctrl_c;
}

pub mod net {
    //! TCP/UDP/Unix bindings (mostly Tokio re-exports).

    use std::{
        future::Future,
        io,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, Interest};
    #[cfg(unix)]
    pub use tokio::net::{UnixDatagram, UnixListener, UnixStream};
    pub use tokio::{
        io::Ready,
        net::{TcpListener, TcpSocket, TcpStream, UdpSocket},
    };

    /// Extension trait over async read+write types that can also signal readiness.
    #[doc(hidden)]
    pub trait ActixStream: AsyncRead + AsyncWrite + Unpin {
        /// Poll stream and check read readiness of Self.
        ///
        /// See [tokio::net::TcpStream::poll_read_ready] for detail on intended use.
        fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>>;

        /// Poll stream and check write readiness of Self.
        ///
        /// See [tokio::net::TcpStream::poll_write_ready] for detail on intended use.
        fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>>;
    }

    impl ActixStream for TcpStream {
        fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>> {
            let ready = self.ready(Interest::READABLE);
            tokio::pin!(ready);
            ready.poll(cx)
        }

        fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>> {
            let ready = self.ready(Interest::WRITABLE);
            tokio::pin!(ready);
            ready.poll(cx)
        }
    }

    #[cfg(unix)]
    impl ActixStream for UnixStream {
        fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>> {
            let ready = self.ready(Interest::READABLE);
            tokio::pin!(ready);
            ready.poll(cx)
        }

        fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>> {
            let ready = self.ready(Interest::WRITABLE);
            tokio::pin!(ready);
            ready.poll(cx)
        }
    }

    impl<Io: ActixStream + ?Sized> ActixStream for Box<Io> {
        fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>> {
            (**self).poll_read_ready(cx)
        }

        fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<Ready>> {
            (**self).poll_write_ready(cx)
        }
    }
}

pub mod time {
    //! Utilities for tracking time (Tokio re-exports).

    pub use tokio::time::{
        interval, interval_at, sleep, sleep_until, timeout, Instant, Interval, Sleep, Timeout,
    };
}

pub mod task {
    //! Task management (Tokio re-exports).

    pub use tokio::task::{spawn_blocking, yield_now, JoinError, JoinHandle};
}

#[track_caller]
#[inline]
pub fn spawn<Fut>(f: Fut) -> JoinHandle<Fut::Output>
where
    Fut: Future + 'static,
    Fut::Output: 'static,
{
    tokio::task::spawn_local(f)
}
