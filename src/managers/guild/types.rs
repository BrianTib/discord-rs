use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::Value;
use serde::{Serialize, Deserialize, Deserializer};

use crate::managers::cache::CacheManager;
use crate::structs::guild::Guild;

#[derive(Debug)]
pub struct GuildManager {
    pub rest: Arc<Mutex<ReqwestClient>>,
    pub cache: CacheManager<Guild>
}

impl<'de> Deserialize<'de> for GuildManager {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut guild: Self = serde::Deserialize::deserialize(deserializer)?;
        println!("GUILD {:?}", guild);
        // Perform custom logic here to set the `skipped` field based on the deserialized data
        //my_struct.skipped = Some(Value::Null);

        Ok(guild)
    }
}