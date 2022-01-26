use crate::messaging::config::Config;
use nats::{Connection, Headers, Subscription};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write, time::Duration};

pub trait Messenger {
    fn init(&self);
    fn publish(&self, msg: Message) -> Result<Message, Error>;
    fn get_last(&self) -> Result<Message, Error>;
}

pub struct NatsMessenger {
    conn: Connection,
    config: Config,
}

impl NatsMessenger {
    pub fn new() -> Self {
        let config = Config::new();
        let conn = Self::connect(&config);

        Self { conn, config }
    }

    fn connect(config: &Config) -> Connection {
        let host = config.get_host();

        nats::connect(&host).expect("Cannot connect to broker.")
    }

    fn subscribe(&self) -> Subscription {
        let subject = self.config.get_subject();

        self.conn
            .subscribe(&subject)
            .expect("Cannot subscribe to broker.")
    }

    fn publish_init_request(&self) {
        let subject = self.config.get_subject();
        let reply = self.conn.new_inbox();

        self.conn
            .publish_request(&subject, &reply, "Init")
            .expect("Cannot publish init message.");
    }

    fn fetch_messages(&self, sub: Subscription) -> Vec<Message> {
        let messages: Vec<Message> = sub
            .timeout_iter(Duration::from_secs(2))
            .filter(|msg| msg.headers.is_some())
            .map(Message::new)
            .collect();

        if messages.is_empty() {
            return self.load_messages();
        } else {
            return messages;
        }
    }

    fn load_messages(&self) -> Vec<Message> {
        let name = self.config.get_data_filename();
        let file = match File::open(&name) {
            Ok(file) => file,
            Err(_) => {
                println!("Generating mock message.");
                self.generate_first_message();
                File::open(name).expect("Error while generating first message.")
            }
        };

        serde_yaml::from_reader(file).expect("Unable to parse messages file.")
    }

    fn generate_first_message(&self) {
        let init = vec![Self::init_message()];
        self.save_messages(init);
    }

    fn init_message() -> Message {
        Message {
            payload: String::from("Init"),
        }
    }

    fn save_messages(&self, messages: Vec<Message>) {
        let serialized = serde_yaml::to_string(&messages).unwrap();

        let filename = self.config.get_data_filename();
        println!("Saving to {}", filename);
        let mut file = File::create(filename).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn setup_subscription_handler(&self) {
        let conn = Self::connect(&self.config);
        let sub = self.subscribe();
        let subject = self.config.get_subject();

        sub.with_handler(move |msg| {
            if msg.data == "Init".as_bytes() {
                if let Some(reply) = msg.reply {
                    println!("Received init request.");
                    match conn.publish_with_reply_or_headers(
                        &subject,
                        Some(&reply),
                        Some(&Headers {
                            inner: HashMap::default(),
                        }),
                        chrono::offset::Local::now().to_rfc2822(),
                    ) {
                        Err(err) => println!("{}", err),
                        _ => println!("Succ"),
                    };
                }
            }
            Ok(())
        });
    }
}

impl Messenger for NatsMessenger {
    fn init(&self) {
        self.publish_init_request();
        let sub = self.subscribe();
        let messages = self.fetch_messages(sub);

        for message in &messages {
            println!("{}", &message.payload);
        }

        self.save_messages(messages);
        self.setup_subscription_handler();
    }

    fn publish(&self, msg: Message) -> Result<Message, Error> {
        Ok(msg)
    }

    fn get_last(&self) -> Result<Message, Error> {
        Ok(Message {
            payload: String::from(""),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    payload: String,
}

impl Message {
    pub fn new(message: nats::Message) -> Self {
        let payload = String::from_utf8_lossy(&message.data).to_string();

        Self { payload }
    }
}

pub enum Error {}
