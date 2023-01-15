use crate::messenger::{config::Config, id::Id, message::Message};
use nats;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    time::Duration,
};

pub trait Messenger {
    fn init(&mut self);
    fn fetch_users(&self) -> Result<Vec<String>, ()>;
    fn fetch_blockchains(&self) -> Result<Vec<String>, ()>;
    fn save_blockchain(&self, chain: String);
}

pub struct NatsMessenger {
    conn: nats::Connection,
    config: Config,
    subscriptions: HashMap<Channel, Subscription>,
    id: Id,
}

struct Subscription {
    channel: Channel,
    sub: nats::Subscription,
    status: Status,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Channel {
    Blockchain,
    Users,
}

impl Channel {
    fn value(&self) -> &str {
        match *self {
            Channel::Blockchain => "blockchain",
            Channel::Users => "users",
        }
    }
}

enum MessageHeaders {
    User,
    Type,
}

impl MessageHeaders {
    fn value(&self) -> &str {
        match *self {
            MessageHeaders::User => "user",
            MessageHeaders::Type => "type",
        }
    }
}

enum MessageType {
    Request,
    Response,
}

impl MessageType {
    fn value(&self) -> &str {
        match *self {
            MessageType::Request => "request",
            MessageType::Response => "response",
        }
    }
}

enum Status {
    Idle,
    Requesting,
    ObtainedResponse,
}

impl NatsMessenger {
    pub fn new() -> Self {
        let config = Config::new();
        let conn = Self::connect(&config);
        let subscriptions = HashMap::new();
        let id = Id::new();

        Self {
            conn,
            config,
            subscriptions,
            id,
        }
    }

    fn connect(config: &Config) -> nats::Connection {
        let host = config.get_host();

        nats::connect(&host).expect("Cannot connect to broker.")
    }

    fn subscribe(&self, channel: Channel) -> Subscription {
        let subject = channel.value();
        let sub = self
            .conn
            .subscribe(&subject)
            .expect("Cannot subscribe to broker.");
        let status = Status::Idle;

        Subscription {
            channel,
            sub,
            status,
        }
    }

    fn fetch_messages(&self, sub: &Subscription) -> Vec<Message> {
        self.publish_request(&sub.channel);

        let mut messages: Vec<Message> = sub
            .sub
            .timeout_iter(Duration::from_secs(2))
            .filter(|msg| msg.headers.is_some())
            .filter(|msg| {
                msg.headers
                    .as_ref()
                    .unwrap()
                    .get(MessageHeaders::Type.value())
                    == Some(&String::from(MessageType::Response.value()))
            })
            .map(Message::new)
            .collect();

        let file_name = self.config.get_data_filename();
        messages.append(&mut Self::load_messages_from_file(file_name));

        messages
    }

    fn publish_request(&self, channel: &Channel) {
        let subject = channel.value();
        let reply = self.conn.new_inbox();

        self.conn
            .publish_request(&subject, &reply, self.id.to_string())
            .expect("Cannot publish init message.");
    }

    fn load_messages_from_file(name: String) -> Vec<Message> {
        match File::open(&name) {
            Ok(file) => serde_yaml::from_reader(file).expect("Unable to parse messages file."),
            Err(_) => vec![],
        }
    }

    fn save_messages_to_file(&self, messages: &Vec<Message>) {
        let serialized = serde_yaml::to_string(&messages).expect("Unable to serialize message.");

        let filename = self.config.get_data_filename();
        println!("Saving to {}", filename);
        let mut file = File::create(filename).expect("Unable to create file.");
        file.write_all(serialized.as_bytes())
            .expect("Unable to save message to file.");
    }

    fn append_message_to_file(&self, message: Message) {
        let filename = self.config.get_data_filename();
        println!("Appending to {}", filename);
        let content = fs::read_to_string(filename).expect("Could not find messages file.");
        let mut messages: Vec<Message> =
            serde_yaml::from_str(&content).expect("Unable to parse messages file.");

        messages.push(message);

        self.save_messages_to_file(&messages);
    }

    fn add_subscription(&mut self, channel: Channel) {
        let sub = self.subscribe(channel.clone());
        self.setup_subscription_handler(channel);
        self.subscriptions.insert(sub.channel, sub);
    }

    fn setup_subscription_handler(&mut self, channel: Channel) {
        let conn = Self::connect(&self.config);
        let sub = self.subscribe(channel);

        let file_name = self.config.get_data_filename();
        let id = self.id.to_string().clone();

        sub.sub.with_handler(move |msg| {
            if msg.data != id.as_bytes() {
                let subject = &sub.channel.value();
                let headers = nats::header::HeaderMap::new();

                let message = Self::load_messages_from_file(file_name.clone());
                let message = serde_yaml::to_string(&message).unwrap();

                if let Some(reply) = msg.reply {
                    println!("Received request.");
                    match conn.publish_with_reply_or_headers(
                        subject,
                        Some(&reply),
                        Some(&headers),
                        message,
                    ) {
                        Err(err) => println!("{}", err),
                        _ => println!("Succ"),
                    };
                }
            }
            Ok(())
        });
    }

    fn fetch(&self, channel: Channel) -> Result<Vec<String>, ()> {
        let subscription = self.subscriptions.get(&channel);

        if let Some(sub) = subscription {
            let messages = self.fetch_messages(&sub);
            self.save_messages_to_file(&messages);

            for message in &messages {
                println!("{:#?}", message);
            }

            return Ok(messages.into_iter().map(|m| m.payload).collect());
        }

        return Err(());
    }

    fn init_messages_file(&self) {
        let filename = self.config.get_data_filename();

        match fs::remove_file(&filename) {
            _ => File::create(filename).expect("Unable to create file."),
        };

        self.save_messages_to_file(&vec![]);
    }
}

impl Messenger for NatsMessenger {
    fn init(&mut self) {
        self.add_subscription(Channel::Blockchain);
        self.add_subscription(Channel::Users);
        self.init_messages_file();
    }

    fn fetch_users(&self) -> Result<Vec<String>, ()> {
        self.fetch(Channel::Users)
    }

    fn fetch_blockchains(&self) -> Result<Vec<String>, ()> {
        self.fetch(Channel::Blockchain)
    }

    fn save_blockchain(&self, chain: String) {
        let message: Message = Message::new_from_string(chain);
        self.append_message_to_file(message);
    }
}
