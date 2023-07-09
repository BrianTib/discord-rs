use serde::{Serialize, Deserialize, Deserializer};

use crate::structs::{
    member::Member,
    channel::Channel,
    role::Role,
    emoji::Emoji
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u32,
    pub application_command_counts: serde_json::Value,
    pub application_id: Option<String>,
    pub banner: Option<String>,
    pub channels: Vec<Channel>,
    pub default_message_notifications: DefaultLevelNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub embedded_activities: Vec<serde_json::Value>,
    pub emojis: Vec<Emoji>,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    #[serde(deserialize_with = "features_deserializer")]
    pub features: Vec<GuildFeature>,
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
    pub mfa_level: MFALevel,
    pub name: String,
    pub nsfw_level: NSFWLevel,
    pub nsfw: bool,
    pub owner_id: String,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: u32,
    pub premium_tier: u32,
    pub presences: Vec<serde_json::Value>,
    pub public_updates_channel_id: Option<String>,
    pub region: String,
    pub roles: Vec<Role>,
    pub rules_channel_id: Option<String>,
    pub safety_alerts_channel_id: Option<String>,
    pub splash: Option<String>,
    pub stage_instances: Vec<serde_json::Value>,
    pub stickers: Vec<serde_json::Value>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: String,
    pub threads: Vec<serde_json::Value>,
    pub unavailable: bool,
    pub vanity_url_code: Option<serde_json::Value>,
    pub verification_level: u32,
    pub voice_states: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildHashes {
    channels: Hash,
    metadata: Hash,
    roles: Hash,
    version: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hash {
    hash: String,
    omitted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DefaultLevelNotificationLevel {
    AllMessages,
    OnlyMentions
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExplicitContentFilterLevel {
    Disabled,
    MembersWithoutRoles,
    AllMembers
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GuildFeature {
    AnimatedBanner,
    AnimatedIcon,
    ApplicationCommandPermissionsV2,
    AutoModeration,
    Banner,
    Community,
    CreatorMonetizableProvisional,
    CreatorStorePage,
    DeveloperSupportServer,
    Discoverable,
    Featurable,
    InvitesDisabled,
    InviteSplash,
    MemberVerificationGateEnabled,
    MoreStickers,
    News,
    Partnered,
    PreviewEnabled,
    RaidAlertsDisabled,
    RoleIcons,
    RoleSubscriptionsAvailableForPurchase,
    RoleSubscriptionsEnabled,
    TicketedEventsEnabled,
    VanityUrl,
    Verified,
    VIPRegions,
    WelcomeScreenEnabled
}

fn features_deserializer<'de, D>(deserializer: D) -> Result<Vec<GuildFeature>, D::Error>
where
    D: Deserializer<'de>,
{
    let feature_strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    let features = feature_strings
        .iter()
        .map(|feature_string| {
            match feature_string.as_str() {
                "ANIMATED_BANNER" => GuildFeature::AnimatedBanner,
                "ANIMATED_ICON" => GuildFeature::AnimatedIcon,
                "APPLICATION_COMMAND_PERMISSIONS_V2" => GuildFeature::ApplicationCommandPermissionsV2,
                "AUTO_MODERATION" => GuildFeature::AutoModeration,
                "BANNER" => GuildFeature::Banner,
                "COMMUNITY" => GuildFeature::Community,
                "CREATOR_MONETIZABLE_PROVISIONAL" => GuildFeature::CreatorMonetizableProvisional,
                "CREATOR_STORE_PAGE" => GuildFeature::CreatorStorePage,
                "DEVELOPER_SUPPORT_SERVER" => GuildFeature::DeveloperSupportServer,
                "DISCOVERABLE" => GuildFeature::Discoverable,
                "FEATURABLE" => GuildFeature::Featurable,
                "INVITES_DISABLED" => GuildFeature::InvitesDisabled,
                "INVITE_SPLASH" => GuildFeature::InviteSplash,
                "MEMBER_VERIFICATION_GATE_ENABLED" => GuildFeature::MemberVerificationGateEnabled,
                "MORE_STICKERS" => GuildFeature::MoreStickers,
                "NEWS" => GuildFeature::News,
                "PARTNERED" => GuildFeature::Partnered,
                "PREVIEW_ENABLED" => GuildFeature::PreviewEnabled,
                "RAID_ALERTS_DISABLED" => GuildFeature::RaidAlertsDisabled,
                "ROLE_ICONS" => GuildFeature::RoleIcons,
                "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE" => {
                    GuildFeature::RoleSubscriptionsAvailableForPurchase
                }
                "ROLE_SUBSCRIPTIONS_ENABLED" => GuildFeature::RoleSubscriptionsEnabled,
                "TICKETED_EVENTS_ENABLED" => GuildFeature::TicketedEventsEnabled,
                "VANITY_URL" => GuildFeature::VanityUrl,
                "VERIFIED" => GuildFeature::Verified,
                "VIP_REGIONS" => GuildFeature::VIPRegions,
                "WELCOME_SCREEN_ENABLED" => GuildFeature::WelcomeScreenEnabled,
                _ => {
                    // Handle unrecognized feature string
                    // You can choose to return an error or fallback to a default value
                    unimplemented!("Unrecognized feature: {}", feature_string);
                }
            }
        })
        .collect();

    Ok(features)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MFALevel {
    None,
    Elevated
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NSFWLevel {
    Default,
    Explicit,
    Safe,
    AgeRestricted
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SystemChannelFlags {
    SupressJoinNotifications = 1 << 0,
    SupressPremiumSubscriptions = 1 << 1,
    SuppressGuildReminderNotifications = 1 << 2,
    SuppressJoinNotificationReplies = 1 << 3,
    SuppressRoleSubscriptionPurchaseNotification = 1 << 4,
    SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5
}