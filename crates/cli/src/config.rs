use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    // database
    pub postgres_url: String,

    // jwt keys
    pub secret_key: String,
    pub public_key: String,

    #[serde(default)]
    pub disable_signup: bool,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("LOCKPAD"))
            .build()?;

        config.try_deserialize()
    }
}
