use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub payload: String,
}

impl Message {
    pub fn new(message: nats::Message) -> Self {
        let payload = String::from_utf8_lossy(&message.data).to_string();

        Self { payload }
    }

    pub fn new_from_string(payload: String) -> Self {
        Self { payload }
    }
}
