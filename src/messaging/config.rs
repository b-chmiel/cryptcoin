use config::Config as LibConfig;
use std::collections::HashMap;
use std::env;

pub struct Config {
	vars: HashMap<String, String>,
}

impl Config {
	pub fn new() -> Self {
		let file_name: String = match env::var("ENV_FILE") {
			Ok(val) => val,
			_ => String::from("Settings"),
		};

		let mut settings = LibConfig::default();
		settings
			.merge(config::File::with_name(&file_name))
			.expect("Unable to open config file.");

		Self {
			vars: settings.try_into::<HashMap<String, String>>().unwrap(),
		}
	}

	pub fn get_host(&self) -> String {
		self.get_value("messaging_host")
	}

	pub fn get_subject(&self) -> String {
		self.get_value("subject")
	}

	pub fn get_data_filename(&self) -> String {
		self.get_value("data_file")
	}

	fn get_value(&self, name: &str) -> String {
		return self
			.vars
			.get(name)
			.expect("Cannot find requested value in config.")
			.to_string();
	}
}
