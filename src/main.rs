mod config;
mod events;
use lazy_static::lazy_static;
use std::env;
use std::error::Error;

use crate::config::AppConfig;

lazy_static! {
    pub static ref ENVIRONMENT: String = match env::var("ENV") {
        Ok(val) => val,
        Err(_) => "LOCAL".to_string(),
    };
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(log_level) = env::var("LOG_LEVEL") {
        env::set_var("RUST_LOG", log_level);
    }

    env_logger::init();
    log::info!("Starting Devpulse Channels CLI Tool");

    use clap::{App, Arg};

    let matches = App::new("Devpulse Channels")
        .version("1.0")
        .author("Your Name")
        .about("Listens to multiple channels for events and forwards them locally")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches
        .value_of("config")
        .expect("Config file path is required");
    let config = load_config_from_file(config_path)?;

    if config.api_key.is_empty() {
        return Err("API Key is required in config".into());
    }

    let app_config = AppConfig {
        api_key: config.api_key,
        channels: config.channels, // Fix: Fully qualify the ChannelConfig type
    };

    events::listen_and_forward_events(app_config).await?;

    Ok(())
}

fn load_config_from_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let config: AppConfig = serde_yaml::from_reader(file)?;
    Ok(config)
}
