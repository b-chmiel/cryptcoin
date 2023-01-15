use rand::rngs::OsRng;
use rsa::PublicKey as RsaPublicKeyVerify;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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

// struct DeserializedPublicKey {
//     value: String,
//     e: BigUint,
//     n: BigUint,
// }

// impl DeserializedPublicKey {
//     fn from(key: PublicKey) -> Self {
//         let n = *key.value.n();
//         let e = *key.value.e();
//         let raw = format!("{}:{}", n, e);

//         Self {
//             value: base64::encode(raw),
//             e,
//             n,
//         }
//     }

//     fn to_key(self) -> PublicKey {
//         PublicKey {
//             value: RsaPublicKey::e(&self)
//         }
//     }
// }

// impl Serialize for PublicKey {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let raw = DeserializedPublicKey::from(*self);
//         serializer.serialize_str(&raw.value)
//     }
// }

// struct PublicKeyVisitor;

// impl<'de> Visitor<'de> for PublicKeyVisitor {
//     type Value = PublicKey;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("Expected to receive string")
//     }

//     fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         Ok(PublicKey::from(key))
//     }
// }

// impl<'de> Deserialize<'de> for PublicKey {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_string(Visitor)
//     }
// }

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
