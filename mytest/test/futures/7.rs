use async_io::{Async, Timer};
use futures_lite::{future::FutureExt, io};
use futures::executor::block_on;

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() -> std::io::Result<()>
{
    let addr = "www.baidu.com:80".to_socket_addrs()?.next().unwrap();

    let stream = Async::<TcpStream>::connect(addr).or(async {
        Timer::after(Duration::from_secs(10)).await;
        Err(io::ErrorKind::TimedOut.into())
    });
    let res = block_on(stream);
    if res.is_ok(){
        println!("conn success!");
    }
    Ok(())
}