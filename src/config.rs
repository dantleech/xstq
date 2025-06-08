use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rule {
    pub name: String,
    pub xpath: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}

impl Config {
    pub fn load(path: String) -> Self {
        let source = match fs::read_to_string(path.clone()) {
            Ok(s) => s,
            Err(_) => panic!("Could not read config file: {}", path),
        };

        toml::from_str(&source).expect("Could not parse config file")
    }

    pub(crate) fn new() -> Self {
        Self { rules: Vec::new() }
    }
}
