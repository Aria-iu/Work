#![allow(unused)]

use std::collections::HashMap;
use std::f64::consts::E;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};


/// 
/// 系统调用使用syscall!宏即可
/// 
#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}


/// 
/// 请求上下文的抽象
/// 
#[derive(Debug)]
pub struct RequestContext {
    pub stream: TcpStream,
    pub content_length: usize,
    pub buf: Vec<u8>,
}


impl RequestContext{
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buf: Vec::new(),
            content_length: 0,
        }
    }

    fn read_cb(&mut self, key: u64, epoll_fd: RawFd) -> io::Result<()>{
        let mut buf = [0u8;4096];
        match self.stream.read(&mut buf) {
            Ok(_) =>{
                if let Ok(data) = std::str::from_utf8(&buf){
                    self.parse_and_set_content_length(data);
                }
            },
            Err(e) if e.kind()==io::ErrorKind::WouldBlock => {},
            Err(e) => {
                return Err(e);
            },
        };
        // 增长缓冲区
        self.buf.extend_from_slice(&buf);
        // 水平触发
        if self.buf.len() >= self.content_length {
            println!("got all data: {} bytes", self.buf.len());
            modify_interest(epoll_fd, self.stream.as_raw_fd(), listener_write_event(key))?;
        } else {
            modify_interest(epoll_fd, self.stream.as_raw_fd(), listener_read_event(key))?;
        }
        Ok(())
    }

    fn parse_and_set_content_length(&mut self,data:&str){
        if data.contains("HTTP"){
            if let Some(content_length) = data.lines().find(|l|l.to_lowercase().starts_with("content-length: ")){
                if let Some(len) = content_length
                    .to_lowercase()
                    .strip_prefix("content-length: ")
                {
                    self.content_length = len.parse::<usize>().expect("content-length is valid");
                    println!("set content length: {} bytes", self.content_length);
                }
            }
        }
    }

    fn write_cb(&mut self, key: u64, epoll_fd: RawFd) -> io::Result<()> {
        match self.stream.write(HTTP_RESP) {
            Ok(_) => println!("answered from request {}", key),
            Err(e) => eprintln!("could not answer to request {}, {}", key, e),
        };
        self.stream.shutdown(std::net::Shutdown::Both)?;
        let fd = self.stream.as_raw_fd();
        remove_interest(epoll_fd, fd)?;
        close(fd);
        Ok(())
    }
}


///
/// libc::EPOLLONESHOT：表示事件在触发一次后会自动失效，需要重新设置才能再次触发。
/// 这用于确保每次事件处理完成后，都需要重新设置事件以避免重复触发。
/// libc::EPOLLIN：表示文件描述符上有数据可读。
/// libc::EPOLLOUT：表示文件描述符可以写入数据。
/// 

const READ_FLAGS: i32 = libc::EPOLLONESHOT | libc::EPOLLIN;
const WRITE_FLAGS: i32 = libc::EPOLLONESHOT | libc::EPOLLOUT;

const HTTP_RESP: &[u8] = br#"HTTP/1.1 200 OK
content-type: text/html
content-length: 5

Hello"#;

/// 
/// 封装epoll_create系统调用
/// 
/// pub type RawFd = raw::c_int;
fn epoll_create() -> io::Result<RawFd>{
    // pub fn epoll_create1(flags: ::c_int) -> ::c_int;
    // 参数 0 表示没有特殊的标志
    let fd = syscall!(epoll_create1(0))?;
    // pub fn fcntl(fd: ::c_int, cmd: ::c_int, ...) -> ::c_int;
    if let Ok(flags) = syscall!(fcntl(fd,libc::F_GETFD)){
        // FD_CLOEXEC 标志的作用是在执行 exec 系列函数（如 execve）时自动关闭该文件描述符。
        // 这是一种安全措施，防止文件描述符意外泄漏到子进程中。
        let _ = syscall!(fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC));
    }
    Ok(fd)
}

///
/// 封装epoll_event的创建
/// u64 字段设置为传入的 key，用于在事件发生时识别该事件。
/// ‵‵‵rust
/// pub struct epoll_event {
///    pub events: u32,
///    pub u64: u64,
/// }
/// ```
/// events：一个 u32 类型的字段，用于表示该事件的类型（如 EPOLLIN、EPOLLOUT 等）。
/// u64：一个 u64 类型的字段，通常用于存储用户数据，在事件发生时用于识别和处理事件。
/// 
fn listener_read_event(key: u64) -> libc::epoll_event {
    libc::epoll_event {
        events: READ_FLAGS as u32,
        u64: key,
    }
}
///
/// 封装epoll_event的创建
/// u64 字段设置为传入的 key，用于在事件发生时识别该事件。
/// ‵‵‵rust
/// pub struct epoll_event {
///    pub events: u32,
///    pub u64: u64,
/// }
/// ```
/// events：一个 u32 类型的字段，用于表示该事件的类型（如 EPOLLIN、EPOLLOUT 等）。
/// u64：一个 u64 类型的字段，通常用于存储用户数据，在事件发生时用于识别和处理事件。
/// 
fn listener_write_event(key: u64) -> libc::epoll_event {
    libc::epoll_event {
        events: WRITE_FLAGS as u32,
        u64: key,
    }
}

/// 
/// 关闭文件描述符
/// pub fn close(fd: ::c_int) -> ::c_int;
/// 
fn close(fd: RawFd) {
    let _ = syscall!(close(fd));
}

///
/// 添加感兴趣的事件
/// epoll_fd: epoll 实例的文件描述符。
/// fd: 要监控的文件描述符。
/// event: 一个 libc::epoll_event 结构体，包含感兴趣的事件类型和用户数据
/// 
fn add_interest(epoll_fd: RawFd, fd: RawFd, mut event: libc::epoll_event) -> io::Result<()> {
    syscall!(epoll_ctl(epoll_fd, libc::EPOLL_CTL_ADD, fd, &mut event))?;
    Ok(())
}

///
/// 修改感兴趣的事件
/// epoll_fd: epoll 实例的文件描述符。
/// fd: 要监控的文件描述符。
/// event: 一个 libc::epoll_event 结构体，包含新的感兴趣的事件类型和用户数据。
/// 
fn modify_interest(epoll_fd: RawFd, fd: RawFd, mut event: libc::epoll_event) -> io::Result<()> {
    syscall!(epoll_ctl(epoll_fd, libc::EPOLL_CTL_MOD, fd, &mut event))?;
    Ok(())
}

/// 
/// 删除感兴趣的事件
/// epoll_fd: epoll 实例的文件描述符。
/// fd: 要取消监控的文件描述符。
/// 
fn remove_interest(epoll_fd: RawFd, fd: RawFd) -> io::Result<()> {
    syscall!(epoll_ctl(
        epoll_fd,
        libc::EPOLL_CTL_DEL,
        fd,
        std::ptr::null_mut()
    ))?;
    Ok(())
}


fn main() -> io::Result<()>{
    // request_contexts：存储每个连接的上下文，键是唯一标识符，值是 RequestContext 对象。
    let mut request_contexts: HashMap<u64, RequestContext> = HashMap::new();
    // events：用于存储 epoll 返回的事件。
    let mut events: Vec<libc::epoll_event> = Vec::with_capacity(1024);
    // key：用于生成唯一标识符，初始值为 100。
    let mut key = 100;

    // 创建一个绑定到 127.0.0.1:8000 的 TcpListener。
    // 将监听器设置为非阻塞模式。
    // 获取监听器的文件描述符。
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    listener.set_nonblocking(true)?;
    let listener_fd = listener.as_raw_fd();

    // 创建一个 epoll 实例。
    let epoll_fd = epoll_create().expect("can create epoll queue");
    // 使用 add_interest 函数将监听器的读事件添加到 epoll 实例中
    add_interest(epoll_fd, listener_fd, listener_read_event(key))?;

    loop{
        // 打印当前正在处理的请求数量。
        println!("requests in flight: {}", request_contexts.len());
        // 清空事件向量。
        events.clear();
        // 调用 epoll_wait 等待事件，最多等待 1024 个事件，超时时间为 1000 毫秒（1 秒）。
        let res = match syscall!(epoll_wait(
            epoll_fd,
            events.as_mut_ptr() as *mut libc::epoll_event,
            1024,
            1000 as libc::c_int,
        )) {
            Ok(v) => v,
            Err(e) => panic!("error during epoll wait: {}", e),
        };
        // 设置事件向量的长度为返回的事件数量
        unsafe { events.set_len(res as usize) };
        // 处理每个事件
        for ev in &events {
            match ev.u64 {
                // 如果事件的 u64 字段是 100，表示监听器有新的连接到来
                100 => {
                    /*
                        接受新连接并设置为非阻塞模式。
                        打印新客户端地址。
                        为新连接生成一个新的 key，并将其读事件添加到 epoll 实例中。
                        创建一个新的 RequestContext 对象并插入到 request_contexts 中。
                        重新设置监听器的读事件。
                    */
                    match listener.accept() {
                        Ok((stream, addr)) => {
                            stream.set_nonblocking(true)?;
                            println!("new client: {}", addr);
                            key += 1;
                            add_interest(epoll_fd, stream.as_raw_fd(), listener_read_event(key))?;
                            request_contexts.insert(key, RequestContext::new(stream));
                        }
                        Err(e) => eprintln!("couldn't accept: {}", e),
                    };
                    modify_interest(epoll_fd, listener_fd, listener_read_event(100))?;
                },
                // 如果事件的 u64 字段是其他值，表示某个客户端连接的事件
                key => {
                    /*
                        检查该连接是否在 request_contexts 中。
                        根据事件类型调用相应的回调函数：
                        EPOLLIN：调用 context.read_cb 处理读事件。
                        EPOLLOUT：调用 context.write_cb 处理写事件，并标记该连接为待删除。
                        删除已处理完的连接。
                    */
                    let mut to_delete = None;
                    if let Some(context) = request_contexts.get_mut(&key) {
                        let events: u32 = ev.events;
                        match events {
                            v if v as i32 & libc::EPOLLIN == libc::EPOLLIN => {
                                context.read_cb(key, epoll_fd)?;
                            }
                            v if v as i32 & libc::EPOLLOUT == libc::EPOLLOUT => {
                                context.write_cb(key, epoll_fd)?;
                                to_delete = Some(key);
                            }
                            v => println!("unexpected events: {}", v),
                        };
                    }
                    if let Some(key) = to_delete {
                        request_contexts.remove(&key);
                    }
                }
            }
        }
    }
}
