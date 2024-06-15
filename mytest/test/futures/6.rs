use futures::executor::block_on;
use futures_lite::io::{self, AsyncReadExt};

fn main(){
    let input: &[u8] = b"hello";
    let mut reader = io::BufReader::new(input);

    let mut contents = String::new();

    let fut = reader.read_to_string(&mut contents);
    let res = block_on(fut);
    println!("{:?}",res);
}