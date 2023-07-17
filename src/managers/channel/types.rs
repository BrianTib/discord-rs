use serde::{Deserialize, Deserializer};
use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::managers::CacheManager;
use crate::structs::channel::Channel;

#[derive(Debug)]
pub struct ChannelManager {
    pub cache: CacheManager<Channel>,
    pub rest: Arc<Mutex<ReqwestClient>>
}

impl<'de> Deserialize<'de> for ChannelManager {
    fn deserialize<D>(_deserializer: D) -> Result<ChannelManager, D::Error>
    where
        D: Deserializer<'de>,
    {   
        let channel_manager = ChannelManager {
            cache: CacheManager::<Channel>::new(),
            rest: Arc::new(Mutex::new(ReqwestClient::new())),
        };

        Ok(channel_manager)
    }
}