use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::{sleep, Duration};

pub struct MyStruct {
    task_handle: Arc<tokio::task::JoinHandle<()>>,
    should_exit: Arc<Mutex<bool>>,
}

impl MyStruct {
    pub async fn start_task(&mut self) {
        let should_exit = Arc::clone(&self.should_exit);

        let handle = tokio::spawn(async move {
            loop {
                println!("Task running...");
                sleep(Duration::from_secs(1)).await; // Simulate some work

                if *should_exit.lock().unwrap() {
                    break;
                }
            }
        });

        self.task_handle = Arc::new(handle);
    }

    pub async fn join_task(&mut self) {
        if let Some(handle) = Arc::get_mut(&mut self.task_handle) {
            *self.should_exit.lock().unwrap() = true;
            let _ = handle.await;
            println!("Task joined");
        }
    }
}

pub async fn main() {
    let should_exit = Arc::new(Mutex::new(false));
    let mut my_struct = MyStruct {
        task_handle: Arc::new(tokio::task::spawn(async {})),
        should_exit: Arc::clone(&should_exit),
    };

    my_struct.start_task().await;

    // Perform other operations

    my_struct.join_task().await;
}