use std::io;
use std::net::{self, SocketAddr};

use mio::{Events, Interest, Poll, Token};
use mio::net::TcpStream;

pub fn test1_1() -> io::Result<()>{
    // Bind a server socket to connect to.
    let addr: SocketAddr = "127.0.0.1:9093".parse().unwrap();
    let server = net::TcpListener::bind(addr).unwrap();

    // Construct a new `Poll` handle as well as the `Events` we'll store into
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);

    let mut stream = TcpStream::connect(server.local_addr().unwrap()).unwrap();

    poll.registry().register(&mut stream,Token(0),Interest::READABLE|Interest::WRITABLE)?;
    // 等待连接到达，结束
    loop{
        poll.poll(&mut events,None)?;
        for e in &events{
            if e.token() == Token(0) && e.is_writable(){
                return Ok(())
            }
        }
    }
}