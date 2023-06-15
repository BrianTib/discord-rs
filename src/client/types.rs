use websocket::client::ClientBuilder;

pub struct Client<'a> {
    pub intents: (u32, Vec<GatewayIntentBits>),
    pub token: Option<String>,    
    pub ws: ClientBuilder<'a>
}

//https://discord.com/developers/docs/topics/gateway#gateway-intents
#[derive(Copy, Clone)]
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