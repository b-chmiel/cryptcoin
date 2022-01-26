use rand::rngs::OsRng;
use rsa::PublicKey as RsaPublicKeyVerify;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::fmt;

const KEY_LENGTH: usize = 512;

pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let private_key =
            RsaPrivateKey::new(&mut rng, KEY_LENGTH).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        Self {
            public_key: PublicKey::from(public_key),
            private_key: PrivateKey::from(private_key),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct PublicKey {
    value: RsaPublicKey,
}

impl PublicKey {
    fn from(key: RsaPublicKey) -> Self {
        PublicKey { value: key }
    }

    pub fn to_string(&self) -> String {
        let raw = format!("{}:{}", self.value.n(), self.value.e());
        base64::encode(raw)
    }

    pub fn verify(&self, padding: PaddingScheme, hashed: &[u8], sig: &[u8]) -> bool {
        match self.value.verify(padding, hashed, sig) {
            Ok(_) => true,
            Err(error) => {
                println!("Signature validation failed: {}", error);
                false
            }
        }
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct PrivateKey {
    pub value: RsaPrivateKey,
}

impl PrivateKey {
    fn from(key: RsaPrivateKey) -> Self {
        PrivateKey { value: key }
    }
}
