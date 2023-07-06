use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::structs::channel::Channel;
use crate::structs::embed::Embed;
use crate::structs::member::Member;
use crate::structs::message::enums::AllowedMentionsType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub attachments: Vec<Value>,
    pub author: Author,
    #[serde(skip)]
    pub channel: Option<Channel>,
    pub channel_id: String,
    pub components: Vec<Value>,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Value>,
    pub flags: u64,
    pub guild_id: String,
    pub id: String,
    pub member: Member,
    pub mention_everyone: bool,
    pub mention_roles: Vec<Value>,
    pub mentions: Vec<Value>,
    pub nonce: String,
    pub pinned: bool,
    pub referenced_message: Option<Value>,
    pub timestamp: String,
    pub tts: bool,
    #[serde(rename = "type")]
    pub msg_type: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub avatar: String,
    pub avatar_decoration: Option<String>,
    pub discriminator: String,
    pub global_name: String,
    pub id: String,
    pub public_flags: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowedMentions {
    pub parse: Vec<AllowedMentionsType>,
    // Max size 100
    pub roles: Vec<String>,
    // Max size 100
    pub users: Vec<String>,
    pub replied_user: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: Option<bool>,
    pub allowed_mentions: Option<AllowedMentions>
}