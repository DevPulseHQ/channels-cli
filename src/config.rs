use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub channels: Vec<ChannelConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub name: String,
    pub channel_id: String,
    pub target: String,
}
