use crate::key::PublicKey;
use crate::signature::Signature;

#[derive(Debug)]
pub struct Transaction {
	pub amount: f32,
	pub receiver_pk: PublicKey,
	pub sender_pk: PublicKey,
}

impl Transaction {
	pub fn valid(&self, signature: &Signature) -> bool {
		signature.verify(self.to_string(), &self.sender_pk)
	}

	pub fn to_string(&self) -> String {
		format!(
			"{}:{}:{}",
			self.amount,
			self.receiver_pk.to_string(),
			self.sender_pk.to_string()
		)
	}
}
