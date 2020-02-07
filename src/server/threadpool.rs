use std::thread::JoinHandle;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::TryRecvError;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn stop(self) {
        for worker in self.workers {
            worker.join();
        }
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || {
            loop {
                let job = match receiver.lock().unwrap().try_recv() {
                    Ok(j) => j,
                    Err(TryRecvError::Disconnected) => {
                        println!("{} thread is stopping", id);
                        break;
                    }
                    Err(TryRecvError::Empty) => continue,
                };

                job();
            }
        }).unwrap();

        Worker {
            id,
            thread,
        }
    }

    pub fn join(self) {
        match self.thread.join() {
            Ok(_) => (),
            Err(e) => println!("Error joining worker {}: {:?}", self.id, e),
        }
    }
}