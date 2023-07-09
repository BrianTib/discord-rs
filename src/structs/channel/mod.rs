use reqwest::RequestBuilder;
#[allow(unconditional_recursion)]
use reqwest::{Response, Client as ReqwestClient};
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Deserialize, Deserializer};

pub mod types;
pub use types::*;

pub mod enums;
pub use enums::*;

use crate::structs::message::MessagePayload;
use crate::structs::permissions::Permissions;

impl Channel {
    pub async fn new(channel_id: &String) -> Self {
        let client = ReqwestClient::new();
        let mut channel = _fetch(&client, &channel_id).await;
        channel._client = Some(Arc::new(Mutex::new(client)));
        channel
    }

    pub async fn send(&self, payload: MessagePayload) -> Result<(), &'static str> {
        let client = self._client.as_ref().unwrap().lock().await;
        let body = serde_json::to_string(&payload).unwrap();

        client
            .post(&body)
            .send()
            .await
            .expect(&format!("Failed to send payload: {} to channel: {}", body, self.id));

        Ok(())
    }

    /// Private function for fetching and updating onto itself current data about the channel
    async fn _fetch_and_update(&mut self) -> Result<(), &'static str> {
        let client = self._client.as_ref().unwrap().lock().await;
        let data = _fetch(&client, &self.id).await;
        println!("Data for channel {:#?}", data);

        Ok(())
    }
}

impl PermissionOverwrite {
    pub fn to_bit(&self, bits: &[Permissions]) -> u64 {
        bits.iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            })
    }
}

async fn _fetch(client: &ReqwestClient, channel_id: &String) -> Channel {
    let base_url = std::env::var("_DISCORD_API_URL").unwrap();
    let token = std::env::var("_CLIENT_TOKEN").unwrap();
    let response = client.get(format!("{base_url}/channels/{}", channel_id))
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await
        .expect(&format!("Failed to fetch channel data for id {}", channel_id));
 
    let response = response.text()
        .await
        .expect("Failed to deserialize channel data");

    serde_json::from_str(&response).unwrap()
}