use std::convert::TryInto;
use serde_json::Value;

use crate::structs::channel::Channel;

pub mod enums;
pub use enums::*;

pub mod types;
pub use types::*;

impl Message {
    /// Sends a text message to the channel
    pub async fn reply_content(&self, content: &str) -> Result<(), &'static str> {
        let mut payload = MessagePayload::new();
        payload.content = Some(content.to_string());
        Self::_reply(payload).await
    }

    // Sends payloads which may include text, embeds, tts and more to the channel
    pub async fn reply(&mut self, payload: MessagePayload) -> Result<(), &'static str> {
        // if self.channel.is_none() {
        //     self.channel = Channel::new(self.channel_id);
        // }
        // Self::_reply(payload).await
        Ok(())
    }

    pub async fn _reply(payload: MessagePayload) -> Result<(), &'static str> {
        Ok(())
    }
}

impl TryInto<Message> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<Message, Self::Error> {
        //let binding = self.clone();
        //let mut value = binding.as_object_mut().unwrap();
        let mut message: Message = serde_json::from_value(self).unwrap();
        //message.client = Channel::new(self.channel_id).await;
        // value.get_mut("attachments").ok_or("");

        println!("Message Value: {:#?}", message);
        
        Ok(message)
    }
}

impl MessagePayload {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: None,
            username: None,
            avatar_url: None,
            tts: None,
            allowed_mentions: None,
        }
    }
}

impl AllowedMentionsType {
    pub fn role_mentions() -> Self {
        Self::RoleMentions
    }

    pub fn user_mentions() -> Self {
        Self::UserMentions
    }

    pub fn everyone_mentions() -> Self {
        Self::EveryoneMentions
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::RoleMentions => "roles",
            Self::UserMentions => "users",
            Self::EveryoneMentions => "everyone",
        }
    }
}