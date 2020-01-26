use std::time::Duration;
use std::thread;
use std::sync::{Mutex, Arc};

type Task = Box<dyn FnMut() + Send + 'static>;

pub struct Upkeepr {
    tasks: Arc<Mutex<Vec<Task>>>
}

impl Upkeepr {
    pub fn new() -> Self
    {
        Upkeepr {
            tasks: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn start(&self) {
        let tasks = self.tasks.clone();

        thread::spawn(move || {
            let mut task_list = tasks.lock().unwrap();

            loop {
                for task in task_list.iter_mut() {
                    task();
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    pub fn add_task<F>(&mut self, f: F)
        where F: FnMut() + Send + 'static
    {
        self.tasks.lock().unwrap().push(Box::new(f));
    }
}