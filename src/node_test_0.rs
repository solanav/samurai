use samurai::upkeep::upkeepr::Upkeepr;
use std::time::Duration;
use std::thread;

fn main() {
    let mut upk = Upkeepr::new();
    upk.add_task(move || println!("Task 1!"));
    upk.add_task(move || println!("Task 2!"));
    upk.start();

    thread::sleep(Duration::from_secs( 20));
}