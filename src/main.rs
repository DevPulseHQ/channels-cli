mod config;
mod events;
use lazy_static::lazy_static;
use std::env;
use std::error::Error;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

    let matches = App::new("Devpulse Channels")
        .version("1.0")
        .author("Devpulse Team")
        .about("Listens to multiple channels for events and forwards them locally")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("init").about("Creates a default config.yml file"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        create_default_config_file()?;
        println!("Created default config.yml file.");
        return Ok(());
    }

    let config_path = matches.value_of("config").unwrap_or("channels-config.yml");
    if !Path::new(config_path).exists() {
        return Err(
            "Config file not found. Use 'init' command to create a default channels-config file."
                .into(),
        );
    }

    let config = load_config_from_file(config_path)?;

    if config.api_key.is_empty() {
        return Err("API Key is required in config".into());
    }

    let app_config = AppConfig {
        api_key: config.api_key,
        channels: config.channels,
    };

    events::listen_and_forward_events(app_config).await?;

    Ok(())
}

fn create_default_config_file() -> Result<(), Box<dyn Error>> {
    let path = "channels-config.yml";
    let mut file = File::create(path)?;
    let contents = "api_key: \"your_api_key_here\"\nchannels:\n  - name: \"Example Channel\"\n    channel_id: \"channel_id_here\"\n    target: \"localhost:3000\"";
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn load_config_from_file(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let config: AppConfig = serde_yaml::from_reader(file)?;
    Ok(config)
}
