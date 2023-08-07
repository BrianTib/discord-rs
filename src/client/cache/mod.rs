use crate::managers::{
    GuildManager,
    ChannelManager
};

pub mod types;
pub use types::ClientCache;

impl ClientCache {
    pub fn new() -> Self {
        Self {
            application: None,
            geo_ordered_rtc_regions: None,
            guild_join_requests: None,
            guilds: GuildManager::new(),
            channels: ChannelManager::new(),
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