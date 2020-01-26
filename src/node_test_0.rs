use samurai::upkeep::upkeepr::Upkeepr;
use std::time::Duration;
use std::thread;

fn main() {
    let mut upk = Upkeepr::new();
    upk.add_task(move || println!("Every 2 seconds!"), Duration::from_secs(2));
    upk.add_task(move || println!("Every 4 seconds!"), Duration::from_secs(4));
    upk.add_task(move || println!("Every 5 seconds!"), Duration::from_secs(5));
    upk.add_task(move || println!("Every 6 seconds!"), Duration::from_secs(6));
    upk.start();

    thread::sleep(Duration::from_secs( 20));
}