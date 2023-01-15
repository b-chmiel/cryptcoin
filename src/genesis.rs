use core::time;
use std::thread;

use cryptcoin::account_manager::{AccountManager, User};

const MAX_RETRIES: u8 = 5;
const SLEEP_TIME_SECONDS: u64 = 2;

fn main() {
    let mut satoshi = AccountManager::new_satoshi();
    satoshi.init();

    println!("Initialized satoshi");
    println!("Chains:");

    for chain in satoshi.fetch_blockchain() {
        println!("{:#?}", chain);
    }

    for _ in 0..MAX_RETRIES {
        match &satoshi.fetch_users() {
            Ok(list) => {
                if send_init_to_first_user(list, &mut satoshi).is_ok() {
                    break;
                }
            }
            Err(err) => println!("Retrying initial transfer: {}", *err),
        }

        wait();
    }
}

fn send_init_to_first_user(list: &Vec<User>, satoshi: &mut AccountManager) -> Result<(), ()> {
    if list.is_empty() {
        println!("Could not find any users, retrying");
        return Err(());
    } else {
        match &satoshi.send(&list[0], 100.0) {
            Ok(_) => return Ok(()),
            Err(err) => {
                println!("Could not send initial money to user: {}", err);
                return Err(());
            }
        }
    }
}

fn wait() {
    let time = time::Duration::from_secs(SLEEP_TIME_SECONDS);
    thread::sleep(time);
}
