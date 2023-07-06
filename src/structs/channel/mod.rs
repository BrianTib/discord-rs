#[allow(unconditional_recursion)]

use reqwest::{Response, Client as ReqwestClient};
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Deserialize, Deserializer};

pub mod types;
pub use types::*;

pub mod enums;
pub use enums::*;

use crate::structs::message::MessagePayload;

const API: &str = "https://discord.com/api/";

impl Channel {
    pub async fn new(channel_id: String) {
        let client = ReqwestClient::new();
        let data = _fetch(&client, &channel_id).await;

        println!("Data: {:?}", data);
        // let mut channel = Self {
        //     id: todo!(),
        //     channel_type: todo!(),
        //     guild_id: todo!(),
        //     position: todo!(),
        //     permission_overwrites: todo!(),
        //     name: todo!(),
        //     topic: todo!(),
        //     nsfw: todo!(),
        //     last_message_id: todo!(),
        //     bitrate: todo!(),
        //     user_limit: todo!(),
        //     rate_limit_per_user: todo!(),
        //     recipients: todo!(),
        //     icon: todo!(),
        //     owner_id: todo!(),
        //     application_id: todo!(),
        //     managed: todo!(),
        //     parent_id: todo!(),
        //     last_pin_timestamp: todo!(),
        //     rtc_region: todo!(),
        //     video_quality_mode: todo!(),
        //     message_count: todo!(),
        //     thread_metadata: todo!(),
        //     member: todo!(),
        //     default_auto_archive_duration: todo!(),
        //     permissions: todo!(),
        //     client: todo!(),
        // };

        //&mut channel
        
        // Self::_fetch_and_update(self)
        //     .await
        //     .expect(&format!("Could not update channel with id: {}", channel_id));
        // self
    }

    pub async fn send(&self, payload: MessagePayload) -> Result<(), &'static str> {
        let client = self.client.lock().await;
        let body = serde_json::to_string(&payload).unwrap();

        client
            .post(&body)
            .send()
            .await
            .expect(&format!("Failed to send payload: {} to channel: {}", body, self.id));

        Ok(())
    }

    /// Private function for fetching and updating onto itself current data about the channel
    async fn _fetch_and_update(&mut self) -> Result<(), &'static str> {
        let client = self.client.lock().await;
        let data = _fetch(&client, &self.id).await;
        println!("Data for channel {:#?}", data);

        Ok(())
    }
}

/// Anonymous function for fetching updated data about the channel
async fn _fetch(client: &ReqwestClient, channel_id: &String) -> Response {
    client.get(format!("{API}/channels/{}", channel_id))
        .send()
        .await
        .expect(&format!("Could not fetch channel with ID: {}", channel_id))
}

impl PermissionOverwrite {
    pub fn to_bit(&self, bits: &[PermissionOverwriteType]) -> u64 {
        bits.iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            })
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let mut channel: Channel = Deserialize::deserialize(deserializer)?;

        channel.client = Arc::new(Mutex::new(ReqwestClient::new()));

        Ok(channel)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        // Default implementation just delegates to `deserialize` impl.
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}