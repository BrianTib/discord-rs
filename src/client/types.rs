use reqwest::Client as ReqwestClient;
use serde::{Serialize, Deserialize};
use websocket::r#async::{Client as WebsocketClient, TcpListener};
use websocket::r#async::client::TlsStream;
use std::ops::Index;

pub struct Client {
    /// A tuple of intents. First element is a bitfield equivalent to the bits
    /// of the second element
    pub intents: (u32, Vec<GatewayIntentBits>),
    /// A string representing the token used to connect to an applications's bot
    pub token: String,
    pub cache: Option<serde_json::Value>,
    pub ws: WebsocketConnection
}

pub struct WebsocketConnection {
    /// A representation of the websocket connection to gateway.discord.com
    pub connection: Option<WebsocketClient<TlsStream<TcpListener<>>>>,
    /// Used to create HTTP requests to the discord API
    pub client: ReqwestClient
}

pub struct SessionStartLimitObject {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: u32,
    pub max_concurrency: u16
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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