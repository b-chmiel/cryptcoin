use crate::blockchain::Blockchain;
use crate::key::{KeyPair, PublicKey};
use crate::signature::Signature;
use crate::transaction::Transaction;
use std::fmt;

pub struct Wallet {
	pub key_pair: KeyPair,
}

impl Wallet {
	pub fn new() -> Self {
		Self {
			key_pair: KeyPair::new(),
		}
	}

	pub fn send(&self, chain: &mut Blockchain, amount: f32, receiver_pk: &PublicKey) {
		let trans = Transaction {
			amount,
			receiver_pk: receiver_pk.clone(),
			sender_pk: self.key_pair.public_key.clone(),
		};

		let signature = Signature::new(trans.to_string(), &self.key_pair.private_key);

		match chain.add_block(trans, &signature) {
			true => println!("Transfered: {} coins", amount),
			false => println!("Cannot transfer!"),
		}
	}

	pub fn balance(&self, chain: &Blockchain) -> f32 {
		let mut balance = 0.0;

		for block in &chain.blocks {
			if block.transaction.receiver_pk == self.key_pair.public_key {
				balance += block.transaction.amount;
			} else if block.transaction.sender_pk == self.key_pair.public_key {
				balance -= block.transaction.amount;
			}
		}

		balance
	}
}

impl fmt::Debug for Wallet {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.key_pair.public_key)
	}
}
