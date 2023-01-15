use cryptcoin::messenger::{Messenger, NatsMessenger};
use rand::Rng;
use std::{thread, time};

fn main() {
    sleep_random_time();

    let mut mes: Box<dyn Messenger> = Box::new(NatsMessenger::new());
    (*mes).init();

    loop {}
}

fn sleep_random_time() {
    let mut rng = rand::thread_rng();

    let time = time::Duration::from_secs(rng.gen_range(0..20));
    println!("Sleeping for {}", time.as_secs());
    thread::sleep(time);
}
