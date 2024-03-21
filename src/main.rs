mod config;
mod events;
use std::env;
use std::error::Error;

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
        .about("Listens to a channel for events and forwards them locally")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("API_KEY")
                .long("API_KEY")
                .help("Sets the API key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CHANNEL")
                .long("CHANNEL")
                .help("Sets the channel URL")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("TARGET")
                .long("TARGET")
                .help("Sets the target URL for forwarding events")
                .takes_value(true),
        )
        .get_matches();

    let config = if let Some(config_path) = matches.value_of("config") {
        load_config_from_file(config_path)?
    } else {
        config::Config {
            api_key: matches.value_of("API_KEY").unwrap_or_default().to_string(),
            channel: matches.value_of("CHANNEL").unwrap_or_default().to_string(),
            target: matches.value_of("TARGET").unwrap_or_default().to_string(),
        }
    };

    // validate the configuration
    if config.api_key.is_empty() {
        return Err("API Key is required".into());
    }
    if config.channel.is_empty() {
        return Err("Channel URL is required".into());
    }
    if config.target.is_empty() {
        return Err("Target URL is required".into());
    }

    // For testing: print out the configuration
    println!("Loaded Configuration:");
    println!("API Key: {}", config.api_key);
    println!("Channel: {}", config.channel);
    println!("Target: {}", config.target);

    events::listen_and_forward_events(config).await?;
    Ok(())
}

fn load_config_from_file(path: &str) -> Result<config::Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let config: config::Config = serde_yaml::from_reader(file)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_from_file() {
        let test_config_path = "tests/test_config.yml"; // Adjust the path as necessary
        let expected_config = config::Config {
            api_key: "test_api_key".to_string(),
            channel: "http://localhost:3099/v1/channels/2dyxqmRqgYfP4uZiiUadVrbqccN/events"
                .to_string(),
            target: "http://localhost:3099/v1/channels/2dyyNxmy2hmaAnvjWWOzWfQtY6v".to_string(),
        };

        let config = load_config_from_file(test_config_path).unwrap();

        assert_eq!(config, expected_config);
    }

    impl PartialEq for config::Config {
        fn eq(&self, other: &Self) -> bool {
            self.api_key == other.api_key
                && self.channel == other.channel
                && self.target == other.target
        }
    }
}
