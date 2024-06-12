use crate::config::Configurable;
use nostr_sdk::Keys;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "parse_keys")]
    pub keys: Keys,
}

impl Configurable for Config {
    fn key() -> &'static str {
        "reportinator"
    }
}

fn parse_keys<'de, D>(deserializer: D) -> Result<Keys, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Keys::parse(s).map_err(de::Error::custom)
}

impl TryFrom<&crate::config::Config> for Config {
    type Error = anyhow::Error;
    fn try_from(value: &crate::config::Config) -> Result<Self, Self::Error> {
        value.get()
    }
}
