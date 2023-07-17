#![allow(dead_code)]
use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::managers::CacheManager;
use crate::structs::{
    channel::Channel
};

pub mod types;
pub use types::*;

impl ChannelManager {
    pub fn new(rest: Arc<Mutex<ReqwestClient>>) -> Self {
        Self {
            cache: CacheManager::<Channel>::new(),
            rest
        }
    }

    pub async fn set_by_id(&mut self, id: &String) {
        Self::_patch(self, id).await;
    }

    pub async fn set(&mut self, channel: Channel) {
        Self::_patch(self, &channel.id).await;
    }

    pub async fn fetch_by_id(&mut self, id: &str) -> Result<Channel, &'static str> {
        let channels = Self::fetch(self, &[id]).await;
        if channels.len() == 1 {
            return Ok(channels[0].clone());
        }
    
        Err("Could not find channel")
    }

    pub async fn fetch(&mut self, ids: &[&str]) -> Vec<Channel> {
        let rest = self.rest.lock().await;
        let mut collection = Vec::<Channel>::new();

        for id in ids.iter() {
            if let Some(channel) = self.cache.get(id) {
                collection.push(channel.to_owned());
                continue;
            }

            let channel = _fetch(&rest, id).await;
            collection.push(channel.to_owned());
            self.cache.set(id.to_string(), channel);
        }

        collection
    }

    async fn _patch(&mut self, channel_id: &String) -> Channel {
        let rest = self.rest.lock().await;
        let channel = _fetch(&rest, channel_id).await;
        self.cache.set(channel_id.to_owned(), channel.to_owned());
        channel
    }
}

async fn _fetch(rest: &ReqwestClient, id: &str) -> Channel {
    let base_url = std::env::var("_DISCORD_API_URL").unwrap();
    let token = std::env::var("_CLIENT_TOKEN").unwrap();

    let response = rest.get(format!("{base_url}/channels/{id}"))
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await
        .expect(&format!("Failed to fetch guild with id {}", id));
 
    let response = response.text()
        .await
        .expect("Failed to deserialize channel data");

    serde_json::from_str(&response).unwrap()
}