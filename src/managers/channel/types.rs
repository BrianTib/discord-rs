use serde::{Deserialize, Deserializer};

use crate::managers::CacheManager;
use crate::structs::channel::Channel;

#[derive(Debug)]
pub struct ChannelManager {
    pub cache: CacheManager<Channel>,
}

impl<'de> Deserialize<'de> for ChannelManager {
    fn deserialize<D>(_deserializer: D) -> Result<ChannelManager, D::Error>
    where
        D: Deserializer<'de>,
    {   
        let channel_manager = ChannelManager {
            cache: CacheManager::<Channel>::new(),
        };

        Ok(channel_manager)
    }
}