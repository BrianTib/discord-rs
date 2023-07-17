use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use serde::{Deserialize, Deserializer};

use crate::managers::cache::CacheManager;
use crate::structs::guild::Guild;

#[derive(Debug)]
pub struct GuildManager {
    pub rest: Arc<Mutex<ReqwestClient>>,
    pub cache: CacheManager<Guild>
}

impl<'de> Deserialize<'de> for GuildManager {
    fn deserialize<D>(_deserializer: D) -> Result<GuildManager, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Create and return the GuildManager instance
        let guild_manager = GuildManager {
            cache: CacheManager::<Guild>::new(),
            rest: Arc::new(Mutex::new(ReqwestClient::new())),
        };

        Ok(guild_manager)
    }
}