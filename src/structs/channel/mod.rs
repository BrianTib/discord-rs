use reqwest::Client as ReqwestClient;
use serde_json::{Map, Value};
use std::sync::{Arc, Mutex};

pub mod types;
pub use types::*;

pub mod enums;
pub use enums::*;

use crate::util::rest::{get, post};

use crate::structs::{
    message_payload::MessagePayload,
    permissions::Permissions
};

impl Channel {
    pub fn new(channel_id: &String) -> Self {
        let channel = _fetch(&channel_id);
        channel
    }

    pub fn send(&self, payload: MessagePayload) -> Result<(), &'static str> {
        let body = serde_json::to_string(&payload).unwrap();
        
        let path = &format!("/channels/{}/messages", &self.id);
        let res = post(path, &body)
            .expect("Could not send message to channel");

        // if &res.status() != 200 {
        //     Err("Error while sending message to channel. API responded with status other than 200")
        // }

        let res_json = res.json::<Map<String, Value>>().unwrap();

        println!("Sent message => {}. Response: {:?}. ", body, res_json);
        Ok(())
    }

    /// Private function for fetching and updating onto itself current data about the channel
    fn _fetch_and_update(&mut self) -> Result<(), &'static str> {
        let data = _fetch(&self.id);
        println!("Data for channel {:#?}", data);

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

fn _fetch(channel_id: &String) -> Channel {
    let request = get(&format!("/channels/{}", channel_id))
        .expect("Request failed to send");
 
    let response = request.text()
        .expect("Failed to deserialize channel data");

    serde_json::from_str(&response).unwrap()
}