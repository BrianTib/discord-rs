#[allow(dead_code, unused_variables, unused_imports)]
use reqwest::Client as ReqwestClient;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::ops::Index;

use super::{
    GatewayEvent,
    DispatchEvent,
    InternalDispatchEvent,
    ExternalDispatchEvent
};

pub type GatewayDispatchEventData = Value;

pub struct Client {
    // pub cache: Arc<Mutex<ClientCache>>,
    //pub events: HashMap<DispatchEvent, BoxedDispatchEventHandler>,
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
    type Output = DispatchEvent;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "HELLO" => &DispatchEvent::Internal(InternalDispatchEvent::Hello),
            "READY" => &DispatchEvent::External(ExternalDispatchEvent::Ready),
            "RESUMED" => &DispatchEvent::Internal(InternalDispatchEvent::Resumed),
            "RECONNECT" => &DispatchEvent::Internal(InternalDispatchEvent::Reconnect),
            "INVALID_SESSION" => &DispatchEvent::Internal(InternalDispatchEvent::InvalidSession),
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::ApplicationCommandPermissionsUpdate),
            "AUTO_MODERATION_RULE_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::AutoModerationRuleCreate),
            "AUTO_MODERATION_RULE_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::AutoModerationRuleUpdate),
            "AUTO_MODERATION_RULE_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::AutoModerationRuleDelete),
            "AUTO_MODERATION_ACTION_EXECUTION" => &DispatchEvent::External(ExternalDispatchEvent::AutoModerationActionExecution),
            "CHANNEL_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::ChannelCreate),
            "CHANNEL_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::ChannelUpdate),
            "CHANNEL_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::ChannelPinsUpdate),
            "THREAD_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::ThreadCreate),
            "THREAD_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::ThreadUpdate),
            "THREAD_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::ThreadDelete),
            "THREAD_LIST_SYNC" => &DispatchEvent::External(ExternalDispatchEvent::ThreadListSync),
            "THREAD_MEMBER_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::ThreadMemberUpdate),
            "THREAD_MEMBERS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::ThreadMembersUpdate),
            "GUILD_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildCreate),
            "GUILD_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildUpdate),
            "GUILD_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::GuildDelete),
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildAuditLogEntryCreate),
            "GUILD_BAN_ADD" => &DispatchEvent::External(ExternalDispatchEvent::GuildBanAdd),
            "GUILD_BAN_REMOVE" => &DispatchEvent::External(ExternalDispatchEvent::GuildBanRemove),
            "GUILD_EMOJIS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildEmojisUpdate),
            "GUILD_STICKERS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildStickersUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildIntegrationsUpdate),
            "GUILD_MEMBER_ADD" => &DispatchEvent::External(ExternalDispatchEvent::GuildMemberAdd),
            "GUILD_MEMBER_REMOVE" => &DispatchEvent::External(ExternalDispatchEvent::GuildMemberRemove),
            "GUILD_MEMBER_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildMemberUpdate),
            "GUILD_MEMBERS_CHUNK" => &DispatchEvent::External(ExternalDispatchEvent::GuildMembersChunk),
            "GUILD_ROLE_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildRoleCreate),
            "GUILD_ROLE_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildRoleUpdate),
            "GUILD_ROLE_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::GuildRoleDelete),
            "GUILD_SCHEDULED_EVENT_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildScheduledEventCreate),
            "GUILD_SCHEDULED_EVENT_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::GuildScheduledEventUpdate),
            "GUILD_SCHEDULED_EVENT_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::GuildScheduledEventDelete),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => &DispatchEvent::External(ExternalDispatchEvent::GuildScheduledEventUserAdd),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => &DispatchEvent::External(ExternalDispatchEvent::GuildScheduledEventUserRemove),
            "INTEGRATION_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::IntegrationCreate),
            "INTEGRATION_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::IntegrationUpdate),
            "INTEGRATION_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::IntegrationDelete),
            "INTERACTION_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::InteractionCreate),
            "INVITE_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::InviteCreate),
            "INVITE_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::InviteDelete),
            "MESSAGE_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::MessageCreate),
            "MESSAGE_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::MessageUpdate),
            "MESSAGE_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::MessageDelete),
            "MESSAGE_DELETE_BULK" => &DispatchEvent::External(ExternalDispatchEvent::MessageDeleteBulk),
            "MESSAGE_REACTION_ADD" => &DispatchEvent::External(ExternalDispatchEvent::MessageReactionAdd),
            "MESSAGE_REACTION_REMOVE" => &DispatchEvent::External(ExternalDispatchEvent::MessageReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => &DispatchEvent::External(ExternalDispatchEvent::MessageReactionRemoveAll),
            "MESSAGE_REACTION_REMOVE_EMOJI" => &DispatchEvent::External(ExternalDispatchEvent::MessageReactionRemoveEmoji),
            "PRESENCE_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::PresenceUpdate),
            "STAGE_INSTANCE_CREATE" => &DispatchEvent::External(ExternalDispatchEvent::StageInstanceCreate),
            "STAGE_INSTANCE_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::StageInstanceUpdate),
            "STAGE_INSTANCE_DELETE" => &DispatchEvent::External(ExternalDispatchEvent::StageInstanceDelete),
            "TYPING_START" => &DispatchEvent::External(ExternalDispatchEvent::TypingStart),
            "USER_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::UserUpdate),
            "VOICE_STATE_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::VoiceStateUpdate),
            "VOICE_SERVER_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::VoiceServerUpdate),
            "WEBHOOKS_UPDATE" => &DispatchEvent::External(ExternalDispatchEvent::WebhooksUpdate),
            _ => panic!("Index out of bounds"),
        }
    }
}