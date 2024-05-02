use tokio::*;

async fn do_stuff_async() {
    println!("async work");
}

async fn more_async_work() {
    println!("more here");
}

#[tokio::main]
async fn main() {
    let (first, second) = tokio::join!(more_async_work(), do_stuff_async());
    // do something with the values
}
