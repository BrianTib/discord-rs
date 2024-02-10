use super::CacheManager;
use crate::structs::{
    channel::Channel,
    snowflake::Snowflake,
    guild::Guild
};

pub enum ClientResource {
    Channel(Channel),
    Guild(Guild)
}

pub type ClientManager = CacheManager<ClientResource>;

impl ClientManager {
    /// Attempt to get a channel from cache. If the channel doesnt exist, fetch it and cache it.
    pub fn get_channel(&mut self, channel_id: &Snowflake) -> Result<&Channel, &'static str> {
        let key = channel_id.to_string();

        // If we dont have a cache of the channel...
        if !self.has(&key) {
             // Fetch the channel from the API
             let channel = ClientResource::Channel(Channel::new(&channel_id)?);
             // Commit the channel to cache
             self.set(channel_id.to_string(), channel);
        }

        // Make sure the given ID belongs to the appropriate variant
        match self.get(&key).expect("Could not retrieve the channel") {
            ClientResource::Channel(channel) => Ok(&channel),
            _ => Err("Given id does not belong to a channel"),
        }
    }
}