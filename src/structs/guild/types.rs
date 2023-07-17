use serde::{Serialize, Deserialize};

use crate::structs::{
    member::Member,
    channel::Channel,
    role::Role,
    emoji::Emoji,
    sticker::Sticker,
    presence::Presence,
    user::User,
    locale::{Locale, locale_deserializer}
};

use super::{
    ExplicitContentFilterLevel,
    GuildFeature,
    MessageNotificationLevel,
    MFALevel,
    NSFWLevel,
    PremiumTierLevel,
    SystemChannelFlags,
    VerificationLevel,
    GuildScheduledEventPrivacyLevel,
    GuildScheduledEventStatus,
    GuildScheduledEventType,
    explicit_content_filter_level_deserializer,
    message_notification_level_deserializer,
    mfa_level_deserializer,
    nsfw_level_deserializer,
    premium_tier_level_deserializer,
    system_channel_flags_deserializer,
    verification_level_deserializer,
    guild_scheduled_event_privacy_level_deserializer,
    guild_scheduled_event_status_deserializer,
    guild_scheduled_event_type_deserializer,
    guild_features_deserializer
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Option<String>,
    pub permissions: Option<String>,
    // Deprecated
    pub region: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u32,
    pub widget_enabled: Option<bool>,
    #[serde(deserialize_with = "verification_level_deserializer")]
    pub verification_level: VerificationLevel,
    #[serde(deserialize_with = "message_notification_level_deserializer")]
    pub default_message_notifications: MessageNotificationLevel,
    #[serde(deserialize_with = "explicit_content_filter_level_deserializer")]
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    #[serde(deserialize_with = "guild_features_deserializer")]
    pub features: Vec<GuildFeature>,
    #[serde(deserialize_with = "mfa_level_deserializer")]
    pub mfa_level: MFALevel,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    #[serde(deserialize_with = "system_channel_flags_deserializer")]
    pub system_channel_flags: Vec<SystemChannelFlags>,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<u32>,
    pub max_members: Option<u32>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    #[serde(deserialize_with = "premium_tier_level_deserializer")]
    pub premium_tier: PremiumTierLevel,
    pub premium_subscription_count: u32,
    #[serde(deserialize_with = "locale_deserializer")]
    pub preferred_locale: Locale,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<u32>,
    pub max_stage_video_channel_users: Option<u32>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    pub welcome_screen: Option<WelcomeScreen>,
    #[serde(deserialize_with = "nsfw_level_deserializer")]
    pub nsfw_level: NSFWLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: bool,
    pub safety_alers_channel_id: Option<String>,
    // From here forth, properties from GuildCreate gateway events
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_states: Option<Vec<VoiceState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<Vec<Presence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_events: Option<Vec<GuildScheduledEvent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeScreenChannel {
    channel_id: String,
    description: String,
    emoji_id: Option<String>,
    emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user_id: String,
    pub member: Option<Member>,
    pub session_id: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEvent {
    pub id: String,
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub creator_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_start_time: Option<String>,
    pub scheduled_end_time: Option<String>,
    #[serde(deserialize_with = "guild_scheduled_event_privacy_level_deserializer")]
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    #[serde(deserialize_with = "guild_scheduled_event_status_deserializer")]
    pub status: GuildScheduledEventStatus,
    #[serde(deserialize_with = "guild_scheduled_event_type_deserializer")]
    pub enitity_type: GuildScheduledEventType,
    pub entity_id: Option<String>,
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,
    pub creator: Option<User>,
    pub user_count: u32,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildScheduledEventEntityMetadata {
    pub location: Option<String>
}