#![allow(dead_code)]
use crate::managers::CacheManager;
use crate::structs::channel::Channel;
use crate::util::rest::get;

pub mod types;
pub use types::*;

impl ChannelManager {
    pub fn new() -> Self {
        Self { cache: CacheManager::<Channel>::new() }
    }

    pub fn set_by_id(&mut self, id: &String) {
        Self::_patch(self, id);
    }

    pub fn set(&mut self, channel: Channel) {
        Self::_patch(self, &channel.id);
    }

    pub fn fetch_by_id(&mut self, id: &str) -> Result<Channel, &'static str> {
        let channels = Self::fetch(self, &[id]);
        if channels.len() == 1 {
            return Ok(channels[0].clone());
        }
    
        Err("Could not find channel")
    }

    pub fn fetch(&mut self, ids: &[&str]) -> Vec<Channel> {
        let mut collection = Vec::<Channel>::new();

        for id in ids.iter() {
            if let Some(channel) = self.cache.get(id) {
                collection.push(channel.to_owned());
                continue;
            }

            let channel = _fetch(id);
            collection.push(channel.to_owned());
            self.cache.set(id.to_string(), channel);
        }

        collection
    }

    fn _patch(&mut self, channel_id: &String) -> Channel {
        let channel = _fetch(channel_id);
        self.cache.set(channel_id.to_owned(), channel.to_owned());
        channel
    }
}

fn _fetch(id: &str) -> Channel {
    let response = get(&format!("/channels/{id}")).unwrap();
    let response = response.text().unwrap();
    serde_json::from_str(&response).unwrap()
}