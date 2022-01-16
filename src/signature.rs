use crate::hash::Hash as Hasher;
use crate::key::{PrivateKey, PublicKey};
use rsa::{Hash, PaddingScheme};

const PADDING: PaddingScheme = PaddingScheme::PKCS1v15Sign {
	hash: Some(Hash::SHA2_256),
};

pub struct Signature {
	value: Vec<u8>,
}

impl Signature {
	pub fn new(payload: String, key: &PrivateKey) -> Self {
		let hash = Hasher::new(payload);
		let sig = key.value.sign(PADDING, hash.get_value());

		Self {
			value: match sig {
				Ok(result) => result,
				Err(error) => panic!("rsa error: {}. Private key too short.", error),
			},
		}
	}

	pub fn verify(&self, payload: String, key: &PublicKey) -> bool {
		let hash = Hasher::new(payload);
		let sig = &self.value;

		key.verify(PADDING, hash.get_value(), &sig)
	}
}
