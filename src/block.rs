use crate::hash::Hash;
use crate::transaction::Transaction;
use chrono::{NaiveDateTime, Utc};
use rand::RngCore;

#[derive(Debug)]
pub struct Block {
	pub transaction: Transaction,
	pub time: NaiveDateTime,
	pub nonce: u32,
	pub previous_hash: Hash,
}

impl Block {
	pub fn new(transaction: Transaction, prev: Hash) -> Self {
		Block {
			transaction,
			time: Utc::now().naive_utc(),
			nonce: rand::thread_rng().next_u32(),
			previous_hash: prev,
		}
	}
	pub fn hash(&self) -> Hash {
		Hash::new(self.to_string())
	}

	fn to_string(&self) -> String {
		format!(
			"{}:{}:{}:{}",
			self.transaction.to_string(),
			self.time,
			self.nonce,
			self.previous_hash.to_string()
		)
	}
}
