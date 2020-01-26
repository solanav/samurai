use std::time::Duration;
use std::thread;
use std::sync::{Mutex, Arc};

pub struct Upkeepr {
    tasks: Arc<Mutex<Vec<(Box<dyn FnMut() + Send + 'static>, Duration)>>>
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
            let mut times: Vec<u64> = tasks.lock().unwrap()
                .iter()
                .map(|(_, d)| (*d).as_secs())
                .collect();

            times.sort_unstable();

            loop {
                let mut time_pos: usize = 0;
                let max = *times.last().expect("no tasks");
                for sec in 0..max + 1 {
                    println!("SECOND {}", sec);
                    loop {
                        if time_pos >= times.len() {
                            break;
                        }

                        if sec == times[time_pos] {
                            println!("SHIT TASK DONE");
                            time_pos += 1;
                        } else if sec > times[time_pos] {
                            time_pos += 1;
                        } else if sec < times[time_pos] {
                            break;
                        }
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            }
        });
    }

    pub fn add_task<F>(&mut self, f: F, delay: Duration)
        where F: FnMut() + Send + 'static
    {
        self.tasks.lock().unwrap()
            .push((Box::new(f), delay));
    }
}