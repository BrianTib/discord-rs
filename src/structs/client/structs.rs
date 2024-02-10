#[allow(dead_code, unused_variables, unused_imports)]
use reqwest::Client as ReqwestClient;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::ops::Index;

use crate::{
    structs::timestamp::Timestamp,
    managers::{
        ChannelManager,
        ClientManager,
        GuildManager
    }
};

use super::{
    GatewayEvent,
    DispatchEvent as DE,
    InternalDispatchEvent as IDE,
    ExternalDispatchEvent as EDE
};

pub type GatewayDispatchEventData = Value;

pub struct Client {
    pub cache: ClientManager,
    pub guilds: GuildManager,
    pub channels: ChannelManager,
    pub(crate) ready_at: Timestamp,
    pub(crate) token: String,
    // TODO: pub user: User,
    pub intents: u64
}

pub struct SessionStartLimitObject {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: u32,
    pub max_concurrency: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayEventBody {
    pub op: usize,
    pub d: Option<serde_json::Value>,
    pub s: Option<u32>,
    pub t: Option<String>
}

pub(crate) struct GatewayEventIndexer;
impl Index<usize> for GatewayEventIndexer {
    type Output = GatewayEvent;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &GatewayEvent::Dispatch,
            1 => &GatewayEvent::Heartbeat,
            2 => &GatewayEvent::Identify,
            3 => &GatewayEvent::PresenceUpdate,
            4 => &GatewayEvent::VoiceStateUpdate,
            // 5 is purposefully skipped
            // https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
            6 => &GatewayEvent::Resume,
            7 => &GatewayEvent::Reconnect,
            8 => &GatewayEvent::RequestGuildMembers,
            9 => &GatewayEvent::InvalidSession,
            10 => &GatewayEvent::Hello,
            11 => &GatewayEvent::HeartbeatAcknowledge,
            _ => panic!("Index out of bounds"),
        }
    }
}
pub(crate) struct DispatchEventIndexer;
impl Index<&str> for DispatchEventIndexer {
    type Output = DE;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "HELLO" => &DE::Internal(IDE::Hello),
            "READY" => &DE::External(EDE::Ready),
            "RESUMED" => &DE::Internal(IDE::Resumed),
            "RECONNECT" => &DE::Internal(IDE::Reconnect),
            "INVALID_SESSION" => &DE::Internal(IDE::InvalidSession),
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => &DE::External(EDE::ApplicationCommandPermissionsUpdate),
            "AUTO_MODERATION_RULE_CREATE" => &DE::External(EDE::AutoModerationRuleCreate),
            "AUTO_MODERATION_RULE_UPDATE" => &DE::External(EDE::AutoModerationRuleUpdate),
            "AUTO_MODERATION_RULE_DELETE" => &DE::External(EDE::AutoModerationRuleDelete),
            "AUTO_MODERATION_ACTION_EXECUTION" => &DE::External(EDE::AutoModerationActionExecution),
            "CHANNEL_CREATE" => &DE::External(EDE::ChannelCreate),
            "CHANNEL_UPDATE" => &DE::External(EDE::ChannelUpdate),
            "CHANNEL_DELETE" => &DE::External(EDE::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => &DE::External(EDE::ChannelPinsUpdate),
            "THREAD_CREATE" => &DE::External(EDE::ThreadCreate),
            "THREAD_UPDATE" => &DE::External(EDE::ThreadUpdate),
            "THREAD_DELETE" => &DE::External(EDE::ThreadDelete),
            "THREAD_LIST_SYNC" => &DE::External(EDE::ThreadListSync),
            "THREAD_MEMBER_UPDATE" => &DE::External(EDE::ThreadMemberUpdate),
            "THREAD_MEMBERS_UPDATE" => &DE::External(EDE::ThreadMembersUpdate),
            "GUILD_CREATE" => &DE::External(EDE::GuildCreate),
            "GUILD_UPDATE" => &DE::External(EDE::GuildUpdate),
            "GUILD_DELETE" => &DE::External(EDE::GuildDelete),
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => &DE::External(EDE::GuildAuditLogEntryCreate),
            "GUILD_BAN_ADD" => &DE::External(EDE::GuildBanAdd),
            "GUILD_BAN_REMOVE" => &DE::External(EDE::GuildBanRemove),
            "GUILD_EMOJIS_UPDATE" => &DE::External(EDE::GuildEmojisUpdate),
            "GUILD_STICKERS_UPDATE" => &DE::External(EDE::GuildStickersUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => &DE::External(EDE::GuildIntegrationsUpdate),
            "GUILD_MEMBER_ADD" => &DE::External(EDE::GuildMemberAdd),
            "GUILD_MEMBER_REMOVE" => &DE::External(EDE::GuildMemberRemove),
            "GUILD_MEMBER_UPDATE" => &DE::External(EDE::GuildMemberUpdate),
            "GUILD_MEMBERS_CHUNK" => &DE::External(EDE::GuildMembersChunk),
            "GUILD_ROLE_CREATE" => &DE::External(EDE::GuildRoleCreate),
            "GUILD_ROLE_UPDATE" => &DE::External(EDE::GuildRoleUpdate),
            "GUILD_ROLE_DELETE" => &DE::External(EDE::GuildRoleDelete),
            "GUILD_SCHEDULED_EVENT_CREATE" => &DE::External(EDE::GuildScheduledEventCreate),
            "GUILD_SCHEDULED_EVENT_UPDATE" => &DE::External(EDE::GuildScheduledEventUpdate),
            "GUILD_SCHEDULED_EVENT_DELETE" => &DE::External(EDE::GuildScheduledEventDelete),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => &DE::External(EDE::GuildScheduledEventUserAdd),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => &DE::External(EDE::GuildScheduledEventUserRemove),
            "INTEGRATION_CREATE" => &DE::External(EDE::IntegrationCreate),
            "INTEGRATION_UPDATE" => &DE::External(EDE::IntegrationUpdate),
            "INTEGRATION_DELETE" => &DE::External(EDE::IntegrationDelete),
            "INTERACTION_CREATE" => &DE::External(EDE::InteractionCreate),
            "INVITE_CREATE" => &DE::External(EDE::InviteCreate),
            "INVITE_DELETE" => &DE::External(EDE::InviteDelete),
            "MESSAGE_CREATE" => &DE::External(EDE::MessageCreate),
            "MESSAGE_UPDATE" => &DE::External(EDE::MessageUpdate),
            "MESSAGE_DELETE" => &DE::External(EDE::MessageDelete),
            "MESSAGE_DELETE_BULK" => &DE::External(EDE::MessageDeleteBulk),
            "MESSAGE_REACTION_ADD" => &DE::External(EDE::MessageReactionAdd),
            "MESSAGE_REACTION_REMOVE" => &DE::External(EDE::MessageReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => &DE::External(EDE::MessageReactionRemoveAll),
            "MESSAGE_REACTION_REMOVE_EMOJI" => &DE::External(EDE::MessageReactionRemoveEmoji),
            "PRESENCE_UPDATE" => &DE::External(EDE::PresenceUpdate),
            "STAGE_INSTANCE_CREATE" => &DE::External(EDE::StageInstanceCreate),
            "STAGE_INSTANCE_UPDATE" => &DE::External(EDE::StageInstanceUpdate),
            "STAGE_INSTANCE_DELETE" => &DE::External(EDE::StageInstanceDelete),
            "TYPING_START" => &DE::External(EDE::TypingStart),
            "USER_UPDATE" => &DE::External(EDE::UserUpdate),
            "VOICE_STATE_UPDATE" => &DE::External(EDE::VoiceStateUpdate),
            "VOICE_SERVER_UPDATE" => &DE::External(EDE::VoiceServerUpdate),
            "WEBHOOKS_UPDATE" => &DE::External(EDE::WebhooksUpdate),
            _ => panic!("Index out of bounds"),
        }
    }
}