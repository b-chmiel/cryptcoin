use sha2::digest::block_buffer::Block;

use crate::{
    blockchain::{Blockchain, Currency, PublicKey, Wallet},
    messenger::{Messenger, NatsMessenger},
};
use std::error::Error;

pub struct AccountManager {
    chains: Vec<Blockchain>,
    wallet: Wallet,
    users: Vec<User>,
    messenger: Box<dyn Messenger>,
}

pub trait Account {
    fn new() -> Self;
    fn init() -> Self;
}

pub struct User {
    public_key: PublicKey,
}

impl AccountManager {
    pub fn new_user() -> Self {
        let chains = vec![];
        let wallet = Wallet::new();
        let users = vec![];
        let messenger = Box::new(NatsMessenger::new());

        Self {
            chains,
            wallet,
            users,
            messenger,
        }
    }

    pub fn new_satoshi() -> Self {
        let mut wallet = Wallet::new();
        let chain = Blockchain::create_and_send_init_to(&mut wallet);

        let chains = vec![chain];
        let users = vec![];
        let messenger = Box::new(NatsMessenger::new());

        Self {
            chains,
            wallet,
            users,
            messenger,
        }
    }

    pub fn init(&mut self) -> Result<(), ()> {
        self.messenger.init();

        if !self.chains.is_empty() {
            self.save_blockchain(self.chains[0].clone());
        }

        Ok(())
    }

    pub fn fetch_blockchain(&mut self) -> Result<Vec<Blockchain>, ()> {
        let messages = self.messenger.fetch_blockchains();

        match messages {
            Ok(list) => Ok(list
                .iter()
                .map(|msg| serde_yaml::from_str(msg).expect("Unable to parse blockchain."))
                .collect()),
            _ => Err(()),
        }
    }

    fn save_blockchain(&self, chain: Blockchain) -> Result<(), ()> {
        let serialized = serde_yaml::to_string(&chain).expect("Unable to serialize blockchain.");

        self.messenger.save_blockchain(serialized);

        Ok(())
    }

    pub fn send(&mut self, user: &User, amount: f32) -> Result<(), Box<dyn Error>> {
        // self.save_blockchain();
        Ok(())
    }

    pub fn fetch_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        self.messenger.fetch_users();
        Ok(vec![])
    }

    pub fn balance(self) -> Result<Currency, ()> {
        Err(())
    }
}
