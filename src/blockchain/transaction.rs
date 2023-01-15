use serde::{Deserialize, Serialize};

use crate::blockchain::{currency::Currency, key::PublicKey, signature::Signature};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: Currency,
    pub receiver_pk: PublicKey,
    pub sender_pk: PublicKey,
}

impl Transaction {
    pub fn valid(&self, signature: &Signature) -> bool {
        signature.verify(self.to_string(), &self.sender_pk)
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:?}:{}:{}",
            self.amount,
            self.receiver_pk.to_string(),
            self.sender_pk.to_string()
        )
    }
}
