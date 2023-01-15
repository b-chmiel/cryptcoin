use crate::blockchain::{hash::Hash, transaction::Transaction};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub transaction: Transaction,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    pub nonce: u32,
    pub previous_hash: Hash,
}

impl Block {
    pub fn new(transaction: Transaction, prev: Hash) -> Self {
        Block {
            transaction,
            time: Utc::now(),
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
