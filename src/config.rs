use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub channel: String,
    pub target: String,
}