use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    // database
    pub postgres_url: String,
    pub dynamodb_endpoint: String,
    pub dynamodb_table: String,

    // jwt keys
    pub secret_key: String,
    pub public_key: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("LOCKPAD"))
            .build()?;

        config.try_deserialize()
    }
}
