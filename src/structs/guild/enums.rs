use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageNotificationLevel {
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
    GuestsEnabled,
    WelcomeScreenEnabled
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
    SuppressJoinNotifications = 1 << 0,
    SuppressPremiumSubscriptions = 1 << 1,
    SuppressGuildReminderNotifications = 1 << 2,
    SuppressJoinNotificationReplies = 1 << 3,
    SuppressRoleSubscriptionPurchaseNotification = 1 << 4,
    SuppressRoleSubscriptionPurchaseNotificationReplies = 1 << 5
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PremiumTierLevel {
    None,
    Tier1,
    Tier2,
    Tier3
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GuildScheduledEventPrivacyLevel {
    GuildOnly = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GuildScheduledEventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Cancelled = 4
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GuildScheduledEventType {
    StageInstance = 1,
    Voice = 2,
    External = 3
}