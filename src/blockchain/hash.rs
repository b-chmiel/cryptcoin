use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Hash {
    pub value: Vec<u8>,
}

impl Hash {
    pub fn new(data: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let value = hasher.finalize();

        Hash {
            value: value.as_slice().to_vec(),
        }
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(self.get_value()).into_owned()
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
