use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Deserializer, Serialize};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::structs::{
    member::Member,
    user::User,
    channel::enums::ChannelType,
    permissions::Permissions
};

//https://discord.com/developers/docs/resources/channel#channels-resource
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type", deserialize_with = "deserialize_channel_type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub recipients: Option<User>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    // TODO: ISO8601
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u8>,
    pub message_count: Option<u64>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u64>,
    pub version: Option<u64>,
    pub available_tags: Option<Vec<ForumTag>>,
    #[serde(skip)]
    pub _client: Option<Arc<Mutex<ReqwestClient>>>,
}

fn deserialize_channel_type<'de, D>(deserializer: D) -> Result<ChannelType, D::Error>
where
    D: Deserializer<'de>,
{
    let channel_type_index: u16 = Deserialize::deserialize(deserializer)?;
    match channel_type_index {
        0 => Ok(ChannelType::GuildText),
        1 => Ok(ChannelType::DM),
        2 => Ok(ChannelType::GuildVoice),
        3 => Ok(ChannelType::GroupDM),
        4 => Ok(ChannelType::GuildCategory),
        5 => Ok(ChannelType::GuildAnnouncement),
        10 => Ok(ChannelType::AnnouncementThread),
        11 => Ok(ChannelType::PublicThread),
        12 => Ok(ChannelType::PrivateThread),
        13 => Ok(ChannelType::GuildStageVoice),
        14 => Ok(ChannelType::GuildDirectory),
        15 => Ok(ChannelType::GuildForum),
        _ => Err(serde::de::Error::custom("Invalid message type index"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelMention {
    pub id: String,
    pub guild_id: String,
    #[serde(rename = "type")]
    pub mention_type: u64,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub permission_type: String,
    pub allow: u32,
    pub deny: u32,
    pub allow_new: String,
    pub deny_new: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    // TODO: ISO8601 timestamp
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    // TODO: ISO8601 timestamp
    pub create_timestamp: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    // TODO: ISO8601 timestamp
    pub join_timestamp: Option<String>,
    pub flags: u64,
    pub member: Option<Member>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTag {
    pub id: String,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>
}