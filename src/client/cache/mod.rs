use reqwest::Client as ReqwestClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::managers::GuildManager;

pub mod types;
pub use types::ClientCache;

impl ClientCache {
    pub fn new(client: Arc<Mutex<ReqwestClient>>) -> Self {
        Self {
            application: None,
            geo_ordered_rtc_regions: None,
            guild_join_requests: None,
            guilds: GuildManager::new(client),
            presences: None,
            private_channels: None,
            relationships: None,
            resume_gateway_url: None,
            session_id: None,
            session_type: None,
            user: None,
            user_settings: None,
            v: None,
        }
    }
}