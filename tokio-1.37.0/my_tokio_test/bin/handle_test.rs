use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
    let handle = Handle::current();
    std::thread::spawn(move || {
        // Using Handle::block_on to run async code in the new thread.
        handle.block_on(async {
            println!("hello");
        });
    });
}
