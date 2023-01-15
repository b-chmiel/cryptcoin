use std::fs::File;

use serde_yaml::{from_reader, from_str};

pub trait Serializer {
    fn serialize_file<T>(file: File) -> Serialized {
        Serialized {
            payload: from_reader(file).expect("Unable to serialize file."),
        }
    }

    fn serialize_str<T>(str: &String) -> Serialized {
        Serialized {
            payload: from_str(str).expect("Unable to serialize file."),
        }
    }
}

pub struct Serialized {
    payload: String,
}
