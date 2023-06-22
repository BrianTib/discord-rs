use futures_util::sink::SinkExt;
use futures_util::stream::{StreamExt, SplitSink, SplitStream};
use reqwest::Client as ReqwestClient;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tungstenite::Message;
use std::collections::HashMap;
use std::ops::Index;
use tokio::{
    sync::mpsc::{Sender, Receiver},
    net::TcpStream
};

pub struct Client {
    /// A tuple of intents. First element is a bitfield equivalent to the bits
    /// of the second element
    pub intents: u64,
    /// A string representing the token used to connect to an applications's bot
    pub token: String,
    pub cache: HashMap<String, serde_json::Value>,
    pub _connection: Option<Arc<Mutex<Connection>>>
}

pub struct Connection {
    pub keepalive: KeepAliveConnection,
    pub socket: WebsocketConnection, 
    pub http_client: ReqwestClient
}

pub struct KeepAliveConnection {
    pub sender: Sender<GatewayEvent>,
    pub receiver: Receiver<GatewayEvent> 
}

pub struct WebsocketConnection {
    pub sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
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
    MessageContent,
    GuildScheduledEvents,
    AutoModerationConfiguration,
    AutoModerationExecution
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum GatewayOpCode {
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

pub struct GatewayOpCodeIndexer;
impl Index<usize> for GatewayOpCodeIndexer {
    type Output = GatewayOpCode;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &GatewayOpCode::Dispatch,
            1 => &GatewayOpCode::Heartbeat,
            2 => &GatewayOpCode::Identify,
            3 => &GatewayOpCode::PresenceUpdate,
            4 => &GatewayOpCode::VoiceStateUpdate,
            // 5 is purposefully skipped
            // https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
            6 => &GatewayOpCode::Resume,
            7 => &GatewayOpCode::Reconnect,
            8 => &GatewayOpCode::RequestGuildMembers,
            9 => &GatewayOpCode::InvalidSession,
            10 => &GatewayOpCode::Hello,
            11 => &GatewayOpCode::HeartbeatAcknowledge,
            _ => panic!("Index out of bounds"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ReceiveEvent {
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

pub struct ReceiveEventIndexer;
impl Index<&str> for ReceiveEventIndexer {
    type Output = ReceiveEvent;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "HELLO" => &ReceiveEvent::Hello,
            "READY" => &ReceiveEvent::Ready,
            "RESUMED" => &ReceiveEvent::Resumed,
            "RECONNECT" => &ReceiveEvent::Reconnect,
            "INVALID_SESSION" => &ReceiveEvent::InvalidSession,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => &ReceiveEvent::ApplicationCommandPermissionsUpdate,
            "AUTO_MODERATION_RULE_CREATE" => &ReceiveEvent::AutoModerationRuleCreate,
            "AUTO_MODERATION_RULE_UPDATE" => &ReceiveEvent::AutoModerationRuleUpdate,
            "AUTO_MODERATION_RULE_DELETE" => &ReceiveEvent::AutoModerationRuleDelete,
            "AUTO_MODERATION_ACTION_EXECUTION" => &ReceiveEvent::AutoModerationActionExecution,
            "CHANNEL_CREATE" => &ReceiveEvent::ChannelCreate,
            "CHANNEL_UPDATE" => &ReceiveEvent::ChannelUpdate,
            "CHANNEL_DELETE" => &ReceiveEvent::ChannelDelete,
            "CHANNEL_PINS_UPDATE" => &ReceiveEvent::ChannelPinsUpdate,
            "THREAD_CREATE" => &ReceiveEvent::ThreadCreate,
            "THREAD_UPDATE" => &ReceiveEvent::ThreadUpdate,
            "THREAD_DELETE" => &ReceiveEvent::ThreadDelete,
            "THREAD_LIST_SYNC" => &ReceiveEvent::ThreadListSync,
            "THREAD_MEMBER_UPDATE" => &ReceiveEvent::ThreadMemberUpdate,
            "THREAD_MEMBERS_UPDATE" => &ReceiveEvent::ThreadMembersUpdate,
            "GUILD_CREATE" => &ReceiveEvent::GuildCreate,
            "GUILD_UPDATE" => &ReceiveEvent::GuildUpdate,
            "GUILD_DELETE" => &ReceiveEvent::GuildDelete,
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => &ReceiveEvent::GuildAuditLogEntryCreate,
            "GUILD_BAN_ADD" => &ReceiveEvent::GuildBanAdd,
            "GUILD_BAN_REMOVE" => &ReceiveEvent::GuildBanRemove,
            "GUILD_EMOJIS_UPDATE" => &ReceiveEvent::GuildEmojisUpdate,
            "GUILD_STICKERS_UPDATE" => &ReceiveEvent::GuildStickersUpdate,
            "GUILD_INTEGRATIONS_UPDATE" => &ReceiveEvent::GuildIntegrationsUpdate,
            "GUILD_MEMBER_ADD" => &ReceiveEvent::GuildMemberAdd,
            "GUILD_MEMBER_REMOVE" => &ReceiveEvent::GuildMemberRemove,
            "GUILD_MEMBER_UPDATE" => &ReceiveEvent::GuildMemberUpdate,
            "GUILD_MEMBERS_CHUNK" => &ReceiveEvent::GuildMembersChunk,
            "GUILD_ROLE_CREATE" => &ReceiveEvent::GuildRoleCreate,
            "GUILD_ROLE_UPDATE" => &ReceiveEvent::GuildRoleUpdate,
            "GUILD_ROLE_DELETE" => &ReceiveEvent::GuildRoleDelete,
            "GUILD_SCHEDULED_EVENT_CREATE" => &ReceiveEvent::GuildScheduledEventCreate,
            "GUILD_SCHEDULED_EVENT_UPDATE" => &ReceiveEvent::GuildScheduledEventUpdate,
            "GUILD_SCHEDULED_EVENT_DELETE" => &ReceiveEvent::GuildScheduledEventDelete,
            "GUILD_SCHEDULED_EVENT_USER_ADD" => &ReceiveEvent::GuildScheduledEventUserAdd,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => &ReceiveEvent::GuildScheduledEventUserRemove,
            "INTEGRATION_CREATE" => &ReceiveEvent::IntegrationCreate,
            "INTEGRATION_UPDATE" => &ReceiveEvent::IntegrationUpdate,
            "INTEGRATION_DELETE" => &ReceiveEvent::IntegrationDelete,
            "INTERACTION_CREATE" => &ReceiveEvent::InteractionCreate,
            "INVITE_CREATE" => &ReceiveEvent::InviteCreate,
            "INVITE_DELETE" => &ReceiveEvent::InviteDelete,
            "MESSAGE_CREATE" => &ReceiveEvent::MessageCreate,
            "MESSAGE_UPDATE" => &ReceiveEvent::MessageUpdate,
            "MESSAGE_DELETE" => &ReceiveEvent::MessageDelete,
            "MESSAGE_DELETE_BULK" => &ReceiveEvent::MessageDeleteBulk,
            "MESSAGE_REACTION_ADD" => &ReceiveEvent::MessageReactionAdd,
            "MESSAGE_REACTION_REMOVE" => &ReceiveEvent::MessageReactionRemove,
            "MESSAGE_REACTION_REMOVE_ALL" => &ReceiveEvent::MessageReactionRemoveAll,
            "MESSAGE_REACTION_REMOVE_EMOJI" => &ReceiveEvent::MessageReactionRemoveEmoji,
            "PRESENCE_UPDATE" => &ReceiveEvent::PresenceUpdate,
            "STAGE_INSTANCE_CREATE" => &ReceiveEvent::StageInstanceCreate,
            "STAGE_INSTANCE_UPDATE" => &ReceiveEvent::StageInstanceUpdate,
            "STAGE_INSTANCE_DELETE" => &ReceiveEvent::StageInstanceDelete,
            "TYPING_START" => &ReceiveEvent::TypingStart,
            "USER_UPDATE" => &ReceiveEvent::UserUpdate,
            "VOICE_STATE_UPDATE" => &ReceiveEvent::VoiceStateUpdate,
            "VOICE_SERVER_UPDATE" => &ReceiveEvent::VoiceServerUpdate,
            "WEBHOOKS_UPDATE" => &ReceiveEvent::WebhooksUpdate,
            _ => panic!("Index out of bounds"),
        }
    }
}