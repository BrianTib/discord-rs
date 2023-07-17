use serde::{Deserialize, Deserializer};

use super::{
    ExplicitContentFilterLevel,
    GuildFeature,
    MessageNotificationLevel,
    MFALevel,
    NSFWLevel,
    SystemChannelFlags,
    VerificationLevel,
    PremiumTierLevel,
    GuildScheduledEventPrivacyLevel,
    GuildScheduledEventStatus,
    GuildScheduledEventType
};

pub fn guild_features_deserializer<'de, D>(deserializer: D) -> Result<Vec<GuildFeature>, D::Error>
where
    D: Deserializer<'de>,
{
    let feature_strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    println!("Feature String: {:?}", feature_strings);
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
                "GUESTS_ENABLED" => GuildFeature::GuestsEnabled,
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

pub fn verification_level_deserializer<'de, D>(deserializer: D) -> Result<VerificationLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u64 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(VerificationLevel::None),
        1 => Ok(VerificationLevel::Low),
        2 => Ok(VerificationLevel::Medium),
        3 => Ok(VerificationLevel::High),
        4 => Ok(VerificationLevel::VeryHigh),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized verification level: {}", index);
        }
    }
}

pub fn message_notification_level_deserializer<'de, D>(deserializer: D) -> Result<MessageNotificationLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u64 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(MessageNotificationLevel::AllMessages),
        1 => Ok(MessageNotificationLevel::OnlyMentions),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized notification level: {}", index);
        }
    }
}

pub fn nsfw_level_deserializer<'de, D>(deserializer: D) -> Result<NSFWLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u64 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(NSFWLevel::Default),
        1 => Ok(NSFWLevel::Explicit),
        2 => Ok(NSFWLevel::Safe),
        3 => Ok(NSFWLevel::AgeRestricted),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized nsfw level: {}", index);
        }
    }
}

pub fn explicit_content_filter_level_deserializer<'de, D>(deserializer: D) -> Result<ExplicitContentFilterLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(ExplicitContentFilterLevel::Disabled),
        1 => Ok(ExplicitContentFilterLevel::MembersWithoutRoles),
        2 => Ok(ExplicitContentFilterLevel::AllMembers),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized explicit content level: {}", index);
        }
    }
}

pub fn mfa_level_deserializer<'de, D>(deserializer: D) -> Result<MFALevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(MFALevel::None),
        1 => Ok(MFALevel::Elevated),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized MFA level: {}", index);
        }
    }
}

pub fn system_channel_flags_deserializer<'de, D>(deserializer: D) -> Result<Vec<SystemChannelFlags>, D::Error>
where
    D: Deserializer<'de>,
{
    let bits: u32 = Deserialize::deserialize(deserializer)?;
    let flags = [
        SystemChannelFlags::SuppressJoinNotifications,
        SystemChannelFlags::SuppressPremiumSubscriptions,
        SystemChannelFlags::SuppressGuildReminderNotifications,
        SystemChannelFlags::SuppressJoinNotificationReplies,
        SystemChannelFlags::SuppressRoleSubscriptionPurchaseNotification,
        SystemChannelFlags::SuppressRoleSubscriptionPurchaseNotificationReplies
    ];

    let enabled_flags: Vec<_> = flags
        .iter()
        .enumerate()
        .filter(|&(index, _)| bits & (1 << index) != 0)
        .map(|(_, flag)| flag.clone())
        .collect();

    Ok(enabled_flags)
}

pub fn premium_tier_level_deserializer<'de, D>(deserializer: D) -> Result<PremiumTierLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(PremiumTierLevel::None),
        1 => Ok(PremiumTierLevel::Tier1),
        2 => Ok(PremiumTierLevel::Tier2),
        3 => Ok(PremiumTierLevel::Tier3),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized premium tier level: {}", index);
        }
    }
}

pub fn guild_scheduled_event_privacy_level_deserializer<'de, D>(deserializer: D) -> Result<GuildScheduledEventPrivacyLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        2 => Ok(GuildScheduledEventPrivacyLevel::GuildOnly),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized guild scheduled event privacy level: {}", index);
        }
    }
}

pub fn guild_scheduled_event_status_deserializer<'de, D>(deserializer: D) -> Result<GuildScheduledEventStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        1 => Ok(GuildScheduledEventStatus::Scheduled),
        2 => Ok(GuildScheduledEventStatus::Active),
        3 => Ok(GuildScheduledEventStatus::Completed),
        4 => Ok(GuildScheduledEventStatus::Cancelled),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized guild scheduled event status: {}", index);
        }
    }
}

pub fn guild_scheduled_event_type_deserializer<'de, D>(deserializer: D) -> Result<GuildScheduledEventType, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        1 => Ok(GuildScheduledEventType::StageInstance),
        2 => Ok(GuildScheduledEventType::Voice),
        3 => Ok(GuildScheduledEventType::External),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized guild scheduled event status: {}", index);
        }
    }
}