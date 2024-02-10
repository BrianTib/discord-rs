use serde_json::{Map, Value};

use super::{
    Channel,
    PermissionOverwrite
};

use crate::{
    util::rest::{get, post},
    structs::{
        message_payload::MessagePayload,
        permissions::Permissions,
        snowflake::Snowflake
    }
};

impl Channel {
    pub fn new(channel_id: &Snowflake) -> Result<Self, &'static str> {
        _fetch(&channel_id)
    }

    /// Send a [MessagePayload] to this channel
    pub fn send(&self, payload: MessagePayload) -> Result<(), &'static str> {
        let payload = serde_json::to_string(&payload).unwrap();
        
        let res = post(&format!("/channels/{}/messages", &self.id), &payload)
            .expect("Failed to send message to channel");

        if res.status() != 200 {
            return Err("Message was not sent successfully");
        }

        let res_json = res.json::<Map<String, Value>>().expect("Failed to deserialize message");

        println!("Sent message => {}. Response => {:?}. ", payload, res_json);
        Ok(())
    }
}

impl PermissionOverwrite {
    pub fn to_bit(&self, bits: &[Permissions]) -> u64 {
        bits.iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            })
    }
}

fn _fetch(channel_id: &Snowflake) -> Result<Channel, &'static str> {
    let request = get(&format!("/channels/{}", channel_id))
        .expect("Request failed to send");

    let json_string = request.text()
        .expect("Failed to fetch channel from API");

    Ok(serde_json::from_str(&json_string)
        .expect("Failed to fetch channel from API"))
}