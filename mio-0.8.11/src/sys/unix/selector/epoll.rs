use crate::{Interest, Token};

use libc::{EPOLLET, EPOLLIN, EPOLLOUT, EPOLLPRI, EPOLLRDHUP};
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(debug_assertions)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::{cmp, i32, io, ptr};

/// Unique id for use as `SelectorId`.
#[cfg(debug_assertions)]
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

///
/// Selector 是一个事件选择器，它允许注册一组文件描述符，
/// 并在这些文件描述符上的 I/O 事件发生时进行通知。
///
/// ep 字段是一个底层的文件描述符（File Descriptor），
/// 它代表了一个事件触发机制，例如在 Linux 中可以是 epoll 实例的文件描述符。
///
#[derive(Debug)]
pub struct Selector {
    // 在调试模式下，结构体可能会包含一个唯一标识符 id，
    // 用于识别不同的事件选择器实例。这个唯一标识符的生成使用了一个静态变量 NEXT_ID，
    // 在每次创建 Selector 实例时递增，以确保每个 Selector 实例都有唯一的标识符。
    #[cfg(debug_assertions)]
    id: usize,
    ep: RawFd,
}


/*
这些方法定义了 `Selector` 结构体的行为，用于管理事件选择器并操作注册在其中的文件描述符。下面是每个方法的作用：

1. **`new()` 方法：**
   - 创建一个新的 `Selector` 实例。
   - 在大多数情况下，会使用 `epoll_create1` 或 `epoll_create` 创建一个新的 epoll 实例，并将其关联的文件描述符保存在 `Selector` 结构体的 `ep` 字段中。
   - 如果系统不支持 `epoll_create1`，会尝试使用 `epoll_create` 创建，并设置 `FD_CLOEXEC` 标志，确保文件描述符在执行程序时不会被继承。
   - 返回创建的 `Selector` 实例或错误信息。

2. **`try_clone()` 方法：**
   - 克隆当前 `Selector` 实例，返回一个新的 `Selector` 实例。
   - 使用 `fcntl` 函数复制当前实例的文件描述符，并设置 `FD_CLOEXEC` 标志，确保文件描述符在执行程序时不会被继承。
   - 返回新的 `Selector` 实例或错误信息。

3. **`select()` 方法：**
   - 等待事件发生并获取事件。
   - 使用 `epoll_wait` 函数在事件选择器上等待事件的发生，设置超时时间。
   - 返回获取到的事件或错误信息。

4. **`register()` 方法：**
   - 注册一个文件描述符到事件选择器中。
   - 使用 `epoll_ctl` 函数将文件描述符添加到事件选择器的监听列表中，并指定对应的事件类型（读、写、错误等）。
   - 返回注册结果或错误信息。

5. **`reregister()` 方法：**
   - 更新已注册文件描述符的事件监听配置。
   - 使用 `epoll_ctl` 函数修改已注册文件描述符的监听配置，包括事件类型等。
   - 返回更新结果或错误信息。

6. **`deregister()` 方法：**
   - 从事件选择器中注销一个文件描述符。
   - 使用 `epoll_ctl` 函数将文件描述符从事件选择器的监听列表中移除。
   - 返回注销结果或错误信息。

这些方法提供了对事件选择器的创建、克隆、等待事件、注册、更新和注销等操作的支持，是在异步 I/O 编程中管理事件的关键组件。
*/
impl Selector {
    pub fn new() -> io::Result<Selector> {
        #[cfg(not(target_os = "android"))]
        let res = syscall!(epoll_create1(libc::EPOLL_CLOEXEC));

        // On Android < API level 16 `epoll_create1` is not defined, so use a
        // raw system call.
        // According to libuv, `EPOLL_CLOEXEC` is not defined on Android API <
        // 21. But `EPOLL_CLOEXEC` is an alias for `O_CLOEXEC` on that platform,
        // so we use it instead.
        #[cfg(target_os = "android")]
        let res = syscall!(syscall(libc::SYS_epoll_create1, libc::O_CLOEXEC));

        let ep = match res {
            Ok(ep) => ep as RawFd,
            Err(err) => {
                // When `epoll_create1` is not available fall back to use
                // `epoll_create` followed by `fcntl`.
                if let Some(libc::ENOSYS) = err.raw_os_error() {
                    match syscall!(epoll_create(1024)) {
                        Ok(ep) => match syscall!(fcntl(ep, libc::F_SETFD, libc::FD_CLOEXEC)) {
                            Ok(ep) => ep as RawFd,
                            Err(err) => {
                                // `fcntl` failed, cleanup `ep`.
                                let _ = unsafe { libc::close(ep) };
                                return Err(err);
                            }
                        },
                        Err(err) => return Err(err),
                    }
                } else {
                    return Err(err);
                }
            }
        };

        Ok(Selector {
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            ep,
        })
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        syscall!(fcntl(self.ep, libc::F_DUPFD_CLOEXEC, super::LOWEST_FD)).map(|ep| Selector {
            // It's the same selector, so we use the same id.
            #[cfg(debug_assertions)]
            id: self.id,
            ep,
        })
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        // A bug in kernels < 2.6.37 makes timeouts larger than LONG_MAX / CONFIG_HZ
        // (approx. 30 minutes with CONFIG_HZ=1200) effectively infinite on 32 bits
        // architectures. The magic number is the same constant used by libuv.
        #[cfg(target_pointer_width = "32")]
        const MAX_SAFE_TIMEOUT: u128 = 1789569;
        #[cfg(not(target_pointer_width = "32"))]
        const MAX_SAFE_TIMEOUT: u128 = libc::c_int::max_value() as u128;

        let timeout = timeout
            .map(|to| {
                // `Duration::as_millis` truncates, so round up. This avoids
                // turning sub-millisecond timeouts into a zero timeout, unless
                // the caller explicitly requests that by specifying a zero
                // timeout.
                let to_ms = to
                    .checked_add(Duration::from_nanos(999_999))
                    .unwrap_or(to)
                    .as_millis();
                cmp::min(MAX_SAFE_TIMEOUT, to_ms) as libc::c_int
            })
            .unwrap_or(-1);

        events.clear();
        syscall!(epoll_wait(
            self.ep,
            events.as_mut_ptr(),
            events.capacity() as i32,
            timeout,
        ))
        .map(|n_events| {
            // This is safe because `epoll_wait` ensures that `n_events` are
            // assigned.
            unsafe { events.set_len(n_events as usize) };
        })
    }

    pub fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interests),
            u64: usize::from(token) as u64,
            #[cfg(target_os = "redox")]
            _pad: 0,
        };

        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_ADD, fd, &mut event)).map(|_| ())
    }

    pub fn reregister(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interests),
            u64: usize::from(token) as u64,
            #[cfg(target_os = "redox")]
            _pad: 0,
        };

        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_MOD, fd, &mut event)).map(|_| ())
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_DEL, fd, ptr::null_mut())).map(|_| ())
    }
}

cfg_io_source! {
    impl Selector {
        #[cfg(debug_assertions)]
        pub fn id(&self) -> usize {
            self.id
        }
    }
}

impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        self.ep
    }
}

impl Drop for Selector {
    fn drop(&mut self) {
        if let Err(err) = syscall!(close(self.ep)) {
            error!("error closing epoll: {}", err);
        }
    }
}

fn interests_to_epoll(interests: Interest) -> u32 {
    let mut kind = EPOLLET;

    if interests.is_readable() {
        kind = kind | EPOLLIN | EPOLLRDHUP;
    }

    if interests.is_writable() {
        kind |= EPOLLOUT;
    }

    if interests.is_priority() {
        kind |= EPOLLPRI;
    }

    kind as u32
}

pub type Event = libc::epoll_event;
pub type Events = Vec<Event>;

pub mod event {
    use std::fmt;

    use crate::sys::Event;
    use crate::Token;

    pub fn token(event: &Event) -> Token {
        Token(event.u64 as usize)
    }

    pub fn is_readable(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLIN) != 0
            || (event.events as libc::c_int & libc::EPOLLPRI) != 0
    }

    pub fn is_writable(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLOUT) != 0
    }

    pub fn is_error(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLERR) != 0
    }

    pub fn is_read_closed(event: &Event) -> bool {
        // Both halves of the socket have closed
        event.events as libc::c_int & libc::EPOLLHUP != 0
            // Socket has received FIN or called shutdown(SHUT_RD)
            || (event.events as libc::c_int & libc::EPOLLIN != 0
                && event.events as libc::c_int & libc::EPOLLRDHUP != 0)
    }

    pub fn is_write_closed(event: &Event) -> bool {
        // Both halves of the socket have closed
        event.events as libc::c_int & libc::EPOLLHUP != 0
            // Unix pipe write end has closed
            || (event.events as libc::c_int & libc::EPOLLOUT != 0
                && event.events as libc::c_int & libc::EPOLLERR != 0)
            // The other side (read end) of a Unix pipe has closed.
            || event.events as libc::c_int == libc::EPOLLERR
    }

    pub fn is_priority(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLPRI) != 0
    }

    pub fn is_aio(_: &Event) -> bool {
        // Not supported in the kernel, only in libc.
        false
    }

    pub fn is_lio(_: &Event) -> bool {
        // Not supported.
        false
    }

    pub fn debug_details(f: &mut fmt::Formatter<'_>, event: &Event) -> fmt::Result {
        #[allow(clippy::trivially_copy_pass_by_ref)]
        fn check_events(got: &u32, want: &libc::c_int) -> bool {
            (*got as libc::c_int & want) != 0
        }
        debug_detail!(
            EventsDetails(u32),
            check_events,
            libc::EPOLLIN,
            libc::EPOLLPRI,
            libc::EPOLLOUT,
            libc::EPOLLRDNORM,
            libc::EPOLLRDBAND,
            libc::EPOLLWRNORM,
            libc::EPOLLWRBAND,
            libc::EPOLLMSG,
            libc::EPOLLERR,
            libc::EPOLLHUP,
            libc::EPOLLET,
            libc::EPOLLRDHUP,
            libc::EPOLLONESHOT,
            #[cfg(target_os = "linux")]
            libc::EPOLLEXCLUSIVE,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            libc::EPOLLWAKEUP,
            libc::EPOLL_CLOEXEC,
        );

        // Can't reference fields in packed structures.
        let e_u64 = event.u64;
        f.debug_struct("epoll_event")
            .field("events", &EventsDetails(event.events))
            .field("u64", &e_u64)
            .finish()
    }
}

#[cfg(target_os = "android")]
#[test]
fn assert_close_on_exec_flag() {
    // This assertion need to be true for Selector::new.
    assert_eq!(libc::O_CLOEXEC, libc::EPOLL_CLOEXEC);
}
