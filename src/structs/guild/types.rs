use serde::{Serialize, Deserialize};

use crate::structs::member::Member;
use crate::structs::channel::Channel;
use crate::structs::role::Role;

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub afk_channel_id: Option<serde_json::Value>,
    pub afk_timeout: u32,
    pub application_command_counts: serde_json::Value,
    pub application_id: Option<serde_json::Value>,
    pub banner: Option<serde_json::Value>,
    pub channels: Vec<Channel>,
    pub default_message_notifications: u32,
    pub description: Option<serde_json::Value>,
    pub discovery_splash: Option<serde_json::Value>,
    pub embedded_activities: Vec<serde_json::Value>,
    pub emojis: Vec<serde_json::Value>,
    pub explicit_content_filter: u32,
    pub features: Vec<String>,
    pub guild_hashes: GuildHashes,
    pub guild_scheduled_events: Vec<serde_json::Value>,
    pub home_header: Option<serde_json::Value>,
    pub hub_type: Option<serde_json::Value>,
    pub icon: String,
    pub id: String,
    pub incidents_data: Option<serde_json::Value>,
    pub joined_at: String,
    pub large: bool,
    pub latest_onboarding_question_id: Option<serde_json::Value>,
    pub lazy: bool,
    pub max_members: u32,
    pub max_stage_video_channel_users: u32,
    pub max_video_channel_users: u32,
    pub member_count: u32,
    pub members: Vec<Member>,
    pub mfa_level: u32,
    pub name: String,
    pub nsfw: bool,
    pub nsfw_level: u32,
    pub owner_id: String,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: u32,
    pub premium_tier: u32,
    pub presences: Vec<serde_json::Value>,
    pub public_updates_channel_id: String,
    pub region: String,
    pub roles: Vec<Role>,
    pub rules_channel_id: String,
    pub safety_alerts_channel_id: Option<serde_json::Value>,
    pub splash: Option<serde_json::Value>,
    pub stage_instances: Vec<serde_json::Value>,
    pub stickers: Vec<serde_json::Value>,
    pub system_channel_flags: u32,
    pub system_channel_id: String,
    pub threads: Vec<serde_json::Value>,
    pub unavailable: bool,
    pub vanity_url_code: Option<serde_json::Value>,
    pub verification_level: u32,
    pub voice_states: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildHashes {
    channels: Hash,
    metadata: Hash,
    roles: Hash,
    version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hash {
    hash: String,
    omitted: bool,
}