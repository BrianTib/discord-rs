use serde::{Serialize, Deserialize};

// Note: You dont need to assign an explicit value to all enums
// If there is specificity needed, assign a value to just one
// and rust will automatically assign the appropriate values to the ones that follow

//https://discord.com/developers/docs/topics/gateway#gateway-intents
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum GatewayIntents {
    //#[deprecated(since="0.0.1")]
    Guilds,
    GuildMembers,
    GuildModeration,
    GuildEmojisAndStickers,
    GuildIntegrations,
    GuildWebhooks,
    GuildInvites,
    GuildVoiceStates,
    GuildPresences,
    GuildMessages,
    GuildMessageReactions,
    GuildMessageTyping,
    DirectMessages,
    DirectmessageReactions,
    DirectmessageTyping,
    MessageContent,
    GuildScheduledEvents,
    AutoModerationConfiguration,
    AutoModerationExecution
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum GatewayEvent {
    Dispatch,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatAcknowledge
}

// https://discord.com/developers/docs/topics/gateway-events#gateway-events
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InternalDispatchEvent {
    Hello,
    Resumed = 3,
    Reconnect,
    InvalidSession
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExternalDispatchEvent {
    Ready = 2,
    ApplicationCommandPermissionsUpdate = 5,
    AutoModerationRuleCreate,
    AutoModerationRuleUpdate,
    AutoModerationRuleDelete,
    AutoModerationActionExecution,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildAuditLogEntryCreate,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    GuildScheduledEventCreate,
    GuildScheduledEventUpdate,
    GuildScheduledEventDelete,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
    VoiceServerUpdate
}

/// [GatewayDispatch](https://discord.com/developers/docs/topics/gateway-events#gateway-events) events
#[derive(Debug, Copy, Clone)]
pub enum DispatchEvent {
    /// GatewayDispatchEvent(s) which are meant to be handled by discord-rs
    Internal(InternalDispatchEvent),
    /// GatewayDispatchEvent(s) which are meant to be handled by the end-user
    External(ExternalDispatchEvent)
}