#[allow(dead_code, unused_variables, unused_imports)]
use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use reqwest::Client as ReqwestClient;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::ops::Index;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tungstenite::Message as TungsteniteMessage;
use tokio::{
    sync::{mpsc::Receiver, Mutex},
    net::TcpStream
};

use crate::client::ClientCache;

pub struct Client {
    pub intents: u64,
    pub token: String,
    pub client: ReqwestClient,
    pub cache: Arc<Mutex<ClientCache>>,
    pub events: Option<Receiver<(GatewayDispatchEventType, Value)>>,
}
pub struct Connection {
    pub socket: WebsocketConnection, 
    pub http_client: ReqwestClient
}
pub struct WebsocketConnection {
    pub sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
    pub receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
}
pub struct SessionStartLimitObject {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: u32,
    pub max_concurrency: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayEvent {
    pub op: usize,
    pub d: Option<serde_json::Value>,
    pub s: Option<u32>,
    pub t: Option<String>
}

//https://discord.com/developers/docs/topics/gateway#gateway-intents
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum GatewayIntentBits {
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
pub enum GatewayEventType {
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

pub struct GatewayEventTypeIndexer;
impl Index<usize> for GatewayEventTypeIndexer {
    type Output = GatewayEventType;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &GatewayEventType::Dispatch,
            1 => &GatewayEventType::Heartbeat,
            2 => &GatewayEventType::Identify,
            3 => &GatewayEventType::PresenceUpdate,
            4 => &GatewayEventType::VoiceStateUpdate,
            // 5 is purposefully skipped
            // https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
            6 => &GatewayEventType::Resume,
            7 => &GatewayEventType::Reconnect,
            8 => &GatewayEventType::RequestGuildMembers,
            9 => &GatewayEventType::InvalidSession,
            10 => &GatewayEventType::Hello,
            11 => &GatewayEventType::HeartbeatAcknowledge,
            _ => panic!("Index out of bounds"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum GatewayDispatchEventType {
    Hello,
    Ready,
    Resumed,
    Reconnect,
    InvalidSession,
    ApplicationCommandPermissionsUpdate,
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
    VoiceServerUpdate,
    WebhooksUpdate
}

pub struct GatewayDispatchEventTypeIndexer;
impl Index<&str> for GatewayDispatchEventTypeIndexer {
    type Output = GatewayDispatchEventType;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "HELLO" => &GatewayDispatchEventType::Hello,
            "READY" => &GatewayDispatchEventType::Ready,
            "RESUMED" => &GatewayDispatchEventType::Resumed,
            "RECONNECT" => &GatewayDispatchEventType::Reconnect,
            "INVALID_SESSION" => &GatewayDispatchEventType::InvalidSession,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => &GatewayDispatchEventType::ApplicationCommandPermissionsUpdate,
            "AUTO_MODERATION_RULE_CREATE" => &GatewayDispatchEventType::AutoModerationRuleCreate,
            "AUTO_MODERATION_RULE_UPDATE" => &GatewayDispatchEventType::AutoModerationRuleUpdate,
            "AUTO_MODERATION_RULE_DELETE" => &GatewayDispatchEventType::AutoModerationRuleDelete,
            "AUTO_MODERATION_ACTION_EXECUTION" => &GatewayDispatchEventType::AutoModerationActionExecution,
            "CHANNEL_CREATE" => &GatewayDispatchEventType::ChannelCreate,
            "CHANNEL_UPDATE" => &GatewayDispatchEventType::ChannelUpdate,
            "CHANNEL_DELETE" => &GatewayDispatchEventType::ChannelDelete,
            "CHANNEL_PINS_UPDATE" => &GatewayDispatchEventType::ChannelPinsUpdate,
            "THREAD_CREATE" => &GatewayDispatchEventType::ThreadCreate,
            "THREAD_UPDATE" => &GatewayDispatchEventType::ThreadUpdate,
            "THREAD_DELETE" => &GatewayDispatchEventType::ThreadDelete,
            "THREAD_LIST_SYNC" => &GatewayDispatchEventType::ThreadListSync,
            "THREAD_MEMBER_UPDATE" => &GatewayDispatchEventType::ThreadMemberUpdate,
            "THREAD_MEMBERS_UPDATE" => &GatewayDispatchEventType::ThreadMembersUpdate,
            "GUILD_CREATE" => &GatewayDispatchEventType::GuildCreate,
            "GUILD_UPDATE" => &GatewayDispatchEventType::GuildUpdate,
            "GUILD_DELETE" => &GatewayDispatchEventType::GuildDelete,
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => &GatewayDispatchEventType::GuildAuditLogEntryCreate,
            "GUILD_BAN_ADD" => &GatewayDispatchEventType::GuildBanAdd,
            "GUILD_BAN_REMOVE" => &GatewayDispatchEventType::GuildBanRemove,
            "GUILD_EMOJIS_UPDATE" => &GatewayDispatchEventType::GuildEmojisUpdate,
            "GUILD_STICKERS_UPDATE" => &GatewayDispatchEventType::GuildStickersUpdate,
            "GUILD_INTEGRATIONS_UPDATE" => &GatewayDispatchEventType::GuildIntegrationsUpdate,
            "GUILD_MEMBER_ADD" => &GatewayDispatchEventType::GuildMemberAdd,
            "GUILD_MEMBER_REMOVE" => &GatewayDispatchEventType::GuildMemberRemove,
            "GUILD_MEMBER_UPDATE" => &GatewayDispatchEventType::GuildMemberUpdate,
            "GUILD_MEMBERS_CHUNK" => &GatewayDispatchEventType::GuildMembersChunk,
            "GUILD_ROLE_CREATE" => &GatewayDispatchEventType::GuildRoleCreate,
            "GUILD_ROLE_UPDATE" => &GatewayDispatchEventType::GuildRoleUpdate,
            "GUILD_ROLE_DELETE" => &GatewayDispatchEventType::GuildRoleDelete,
            "GUILD_SCHEDULED_EVENT_CREATE" => &GatewayDispatchEventType::GuildScheduledEventCreate,
            "GUILD_SCHEDULED_EVENT_UPDATE" => &GatewayDispatchEventType::GuildScheduledEventUpdate,
            "GUILD_SCHEDULED_EVENT_DELETE" => &GatewayDispatchEventType::GuildScheduledEventDelete,
            "GUILD_SCHEDULED_EVENT_USER_ADD" => &GatewayDispatchEventType::GuildScheduledEventUserAdd,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => &GatewayDispatchEventType::GuildScheduledEventUserRemove,
            "INTEGRATION_CREATE" => &GatewayDispatchEventType::IntegrationCreate,
            "INTEGRATION_UPDATE" => &GatewayDispatchEventType::IntegrationUpdate,
            "INTEGRATION_DELETE" => &GatewayDispatchEventType::IntegrationDelete,
            "INTERACTION_CREATE" => &GatewayDispatchEventType::InteractionCreate,
            "INVITE_CREATE" => &GatewayDispatchEventType::InviteCreate,
            "INVITE_DELETE" => &GatewayDispatchEventType::InviteDelete,
            "MESSAGE_CREATE" => &GatewayDispatchEventType::MessageCreate,
            "MESSAGE_UPDATE" => &GatewayDispatchEventType::MessageUpdate,
            "MESSAGE_DELETE" => &GatewayDispatchEventType::MessageDelete,
            "MESSAGE_DELETE_BULK" => &GatewayDispatchEventType::MessageDeleteBulk,
            "MESSAGE_REACTION_ADD" => &GatewayDispatchEventType::MessageReactionAdd,
            "MESSAGE_REACTION_REMOVE" => &GatewayDispatchEventType::MessageReactionRemove,
            "MESSAGE_REACTION_REMOVE_ALL" => &GatewayDispatchEventType::MessageReactionRemoveAll,
            "MESSAGE_REACTION_REMOVE_EMOJI" => &GatewayDispatchEventType::MessageReactionRemoveEmoji,
            "PRESENCE_UPDATE" => &GatewayDispatchEventType::PresenceUpdate,
            "STAGE_INSTANCE_CREATE" => &GatewayDispatchEventType::StageInstanceCreate,
            "STAGE_INSTANCE_UPDATE" => &GatewayDispatchEventType::StageInstanceUpdate,
            "STAGE_INSTANCE_DELETE" => &GatewayDispatchEventType::StageInstanceDelete,
            "TYPING_START" => &GatewayDispatchEventType::TypingStart,
            "USER_UPDATE" => &GatewayDispatchEventType::UserUpdate,
            "VOICE_STATE_UPDATE" => &GatewayDispatchEventType::VoiceStateUpdate,
            "VOICE_SERVER_UPDATE" => &GatewayDispatchEventType::VoiceServerUpdate,
            "WEBHOOKS_UPDATE" => &GatewayDispatchEventType::WebhooksUpdate,
            _ => panic!("Index out of bounds"),
        }
    }
}