use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::structs::member::Member;

use crate::structs::user::User;
use crate::structs::channel::enums::{
    PermissionOverwriteType,
    ChannelType
};

//https://discord.com/developers/docs/resources/channel#channels-resource
#[derive(Serialize, Debug)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
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
    pub client: Arc<Mutex<ReqwestClient>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionOverwrite {
    pub id: String,
    pub permission_type: PermissionOverwriteType,
    pub allow: String,
    pub deny: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Debug)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    // TODO: ISO8601 timestamp
    pub join_timestamp: Option<String>,
    pub flags: u64,
    pub member: Option<Member>,
}

#[derive(Serialize, Debug)]
pub struct ForumTag {
    pub id: String,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>
}