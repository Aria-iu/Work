use std::thread;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
    // Inside an async block or function.
    let handle = Handle::current();
    handle.spawn(async {
        println!("now running in the existing Runtime");
    });
    /*
       thread::spawn(move || {
           // Notice that the handle is created outside of this thread and then moved in
           handle.spawn(async { /* ... */ });
           // This next line would cause a panic because we haven't entered the runtime
           // and created an EnterGuard
           // let handle2 = Handle::current(); // panic
           // So we create a guard here with Handle::enter();
           let _guard = handle.enter();
           // Now we can call Handle::current();
           let handle2 = Handle::current();
       });
    */
}
