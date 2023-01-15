use cryptcoin::account_manager::AccountManager;
use rand::Rng;
use std::{thread, time};

fn main() {
    println!("Starting user");
    sleep_random_time();
    let mut user = AccountManager::new_user();
    user.init();

    for chain in user.fetch_blockchain() {
        println!("{:#?}", chain);
    }
}

fn sleep_random_time() {
    let mut rng = rand::thread_rng();

    let time = time::Duration::from_secs(rng.gen_range(2..10));
    println!("Sleeping for {}", time.as_secs());
    thread::sleep(time);
}
