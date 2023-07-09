
#![allow(dead_code)]
use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::managers::cache::CacheManager;
use crate::structs::guild::Guild;

pub mod types;
pub use types::*;

impl GuildManager {
    pub fn new(rest: Arc<Mutex<ReqwestClient>>) -> Self {
        Self {
            rest,
            cache: CacheManager::<Guild>::new(),
        }
    }
    
    pub fn get(&self, ids: &[&str]) -> Vec<Guild> {
        let mut collection = Vec::<Guild>::new();

        for id in ids.iter() {
            if let Some(guild) = self.cache.get(id) {
                collection.push(guild.to_owned());
            }
        }

        collection
    }

    // TODO: Create guild
    pub async fn fetch(&mut self, ids: &[&str]) -> Vec<Guild> {
        let rest = self.rest.lock().await;
        let mut collection = Vec::<Guild>::new();

        for id in ids.iter() {
            if let Some(guild) = self.cache.get(id) {
                collection.push(guild.to_owned());
                continue;
            }

            let guild = _fetch(&rest, id).await;
            collection.push(guild.to_owned());
            self.cache.set(id.to_string(), guild);
        }

        collection
    }
}

async fn _fetch(rest: &ReqwestClient, id: &str) -> Guild {
    let base_url = std::env::var("_DISCORD_API_URL").unwrap();
    let token = std::env::var("_CLIENT_TOKEN").unwrap();
    let response = rest.get(format!("{base_url}/guilds/{id}"))
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await
        .expect(&format!("Failed to fetch guild with id {}", id));
 
    let response = response.text()
        .await
        .expect("Failed to deserialize channel data");

    serde_json::from_str(&response).unwrap()
}