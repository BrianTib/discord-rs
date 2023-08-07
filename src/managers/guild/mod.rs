
#[allow(dead_code)]
use crate::managers::cache::CacheManager;
use crate::structs::guild::Guild;
use crate::util::rest::get;

pub mod types;
pub use types::*;

impl GuildManager {
    pub fn new() -> Self {
        Self {
            cache: CacheManager::<Guild>::new(),
        }
    }

    pub fn set(&mut self, guild: Guild) {
        self.cache.set(guild.id.to_owned(), guild);
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
        let mut collection = Vec::<Guild>::new();

        for id in ids.iter() {
            if let Some(guild) = self.cache.get(id) {
                collection.push(guild.to_owned());
                continue;
            }

            let guild = _fetch(id).await;
            collection.push(guild.to_owned());
            self.cache.set(id.to_string(), guild);
        }

        collection
    }
}

async fn _fetch(id: &str) -> Guild {
    let response = get(&format!("/guilds/{id}"))
        .await
        .expect(&format!("Failed to fetch guild with id {}", id));
 
    let response = response.text()
        .await
        .expect("Failed to deserialize channel data");

    serde_json::from_str(&response).unwrap()
}