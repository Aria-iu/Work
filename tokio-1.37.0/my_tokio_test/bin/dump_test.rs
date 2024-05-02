use tokio::runtime::Handle;
use tokio::time::{timeout, Duration};
#[tokio::main]
async fn main() {
    // Inside an async block or function.
    let handle = Handle::current();
    if let Ok(dump) = timeout(Duration::from_secs(2), handle.dump()).await {
        for (i, task) in dump.tasks().iter().enumerate() {
            let trace = task.trace();
            println!("TASK {i}:");
            println!("{trace}\n");
        }
    }
}
