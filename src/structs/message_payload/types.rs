use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::structs::{
    attachment::Attachment,
    embed::Embed,
    snowflake::Snowflake,
    nonce::Nonce
};

#[derive(Serialize, Deserialize, Debug)]
pub enum AllowedMentionsType {
    RoleMentions,
    UserMentions,
    EveryoneMentions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowedMentions {
    pub parse: Vec<AllowedMentionsType>,
    /// Max size 100
    pub roles: Vec<String>,
    /// Max size 100
    pub users: Vec<String>,
    pub replied_user: bool
}

// Dont serialize any of the optionals if they are of the None variant
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub content: Option<String>,
    pub nonce: Option<Nonce>,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<AllowedMentions>,
    // TODO: Make MessageReference object
    pub message_reference: Option<String>,
    // TODO: Make Component object
    pub components: Option<Value>,
    pub sticker_ids: Option<Vec<Snowflake>>,
    // TODO: Make File object
    pub files: Option<Value>,
    pub payload_json: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub flags: Option<usize>

    // These fields are only usable on webhook payloads
    // pub username: Option<String>,
    // pub avatar_url: Option<String>,
}