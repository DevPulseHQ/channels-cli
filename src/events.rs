use crate::{
    config::{AppConfig, ChannelConfig},
    ENVIRONMENT,
};
use futures::stream::StreamExt;
use log::{error, info};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Event {
    id: String,
    payload: String,
    headers: HashMap<String, String>,
    #[serde(rename = "contentType")]
    content_type: String,
    #[serde(rename = "channelId")]
    channel_id: String,
    #[serde(rename = "receivedAt")]
    received_at: u64,
}

pub async fn listen_and_forward_events(
    config: AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = &config.api_key; // Assuming the API key is shared across channels

    let tasks: Vec<_> = config
        .channels
        .into_iter()
        .map(|channel_config| listen_and_forward_for_channel(api_key, channel_config))
        .collect();

    futures::future::join_all(tasks).await;
    Ok(())
}

async fn listen_and_forward_for_channel(
    api_key: &str,
    channel_config: ChannelConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Listening for events on channel: {}", channel_config.name);

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("{}", api_key))?,
    );
    headers.insert("Content-Type", HeaderValue::from_str("application/json")?);
    headers.insert("Accept", HeaderValue::from_str("application/json")?);

    let client = Client::builder().default_headers(headers).build()?;

    let mut channel_url = format!(
        "https://api.channels.devpulsehq.com/v1/channels/{}/events",
        channel_config.channel_id
    );

    if ENVIRONMENT.eq("development") {
        channel_url = format!(
            "http://localhost:3099/v1/channels/{}/events",
            channel_config.channel_id
        );
    }

    // listen for events from the channel
    let response = match client.get(&channel_url).send().await {
        Ok(response) => response,
        Err(e) => {
            error!("Error sending request to channel: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut response = match response.error_for_status() {
        Ok(response) => response.bytes_stream(),
        Err(e) => {
            error!("Error with response status: {}", e);
            return Err(Box::new(e));
        }
    };

    while let Some(item) = response.next().await {
        match item {
            Ok(bytes) => {
                let event_str = match std::str::from_utf8(&bytes) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Error decoding event bytes: {}", e);
                        continue;
                    }
                };

                let event: Event = match serde_json::from_str(event_str) {
                    Ok(event) => event,
                    Err(e) => {
                        error!("Error parsing event JSON: {}", e);
                        continue;
                    }
                };

                // log received event
                info!("Received event at: {:?}", event.received_at);
                let headers: HeaderMap<HeaderValue> = event
                    .headers
                    .iter()
                    .map(|(name, value)| {
                        (
                            HeaderName::from_bytes(name.as_bytes()).unwrap(),
                            HeaderValue::from_str(value).unwrap(),
                        )
                    })
                    .collect();

                // log the event payload
                info!("Event payload: {:?}", event.payload);

                match client
                    .post(&channel_config.target)
                    .header("Content-Type", &event.content_type)
                    .body(event.payload) // Assuming `event.payload` is already a serde_json::Value
                    .headers(headers)
                    .send()
                    .await
                {
                    Ok(_) => info!(
                        "Event {} forwarded successfully for {}",
                        event.id, event.channel_id
                    ),
                    Err(e) => error!("Error forwarding event {}: {}", event.channel_id, e),
                }
            }
            Err(e) => error!("Error receiving event: {}", e),
        }
    }

    Ok(())
}
