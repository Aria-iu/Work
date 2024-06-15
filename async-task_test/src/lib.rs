extern crate async_task;
extern crate flume;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::{self};
    #[test]
    fn it_works() {
        let (sender, receiver) = flume::unbounded();
    
        // A future that will be spawned.
        let future = async {
            println!("I'm polled!!");
            1 + 2
        };

        // A function that schedules the task when it gets woken up.
        let schedule = move |runnable| sender.send(runnable).unwrap();

        // Construct a task.
        let (runnable, task) = async_task::spawn(future, schedule);

        // Push the task into the queue by invoking its schedule function.
        runnable.schedule();
        
         // Poll the task to completion in a separate thread.
        let handle = thread::spawn(move || {
            while let Ok(runnable) = receiver.recv() {
                runnable.run();
            }
        });

        // Optionally, you can check the result of the task.
        let result = futures::executor::block_on(task);
        assert_eq!(result, 3);

        handle.join().unwrap()
    }
}
