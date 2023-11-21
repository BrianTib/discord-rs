use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::structs::{
    application::Application,
    attachment::Attachment,
    channel::{Channel, ChannelMention},
    embed::Embed,
    member::Member,
    message::enums::{MessageType, MessageActivity},
    reaction::Reaction,
    role::Role,
    snowflake::Snowflake,
    sticker::Sticker,
    user::User,
    nonce::Nonce
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// id of the message
    pub id: Snowflake,
    /// id of the channel the message was sent in
    pub channel_id: Option<Snowflake>,
    pub author: User,
    pub content: String,
    // TODO: ISO8601 timestamp
    pub timestamp: String,
    // TODO: ISO8601 timestamp
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<Value>,
    pub mention_roles: Vec<Role>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type", deserialize_with = "deserialize_message_type")]
    pub message_type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<Snowflake>,
    pub flags: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    // TODO: Make this a MessageInteraction object https://discord.com/developers/docs/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure
    pub interaction: Option<Value>,
    pub thread: Option<Channel>,
    // TODO: Make this a Component object https://discord.com/developers/docs/interactions/message-components#component-object
    pub components: Option<Vec<Value>>,
    // TODO: Make this a StickerItem object https://discord.com/developers/docs/resources/sticker#sticker-item-object
    pub sticker_items: Option<Vec<Value>>,
    pub stickers: Option<Vec<Sticker>>,
    pub position: Option<usize>,
    // TODO: Make this a RoleSubscriptionData object https://discord.com/developers/docs/resources/channel#role-subscription-data-object
    pub role_subscription_data: Option<Value>,
    // TODO: Make this is a ResolvedData object https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure
    pub resolved: Option<Value>,
    #[serde(skip)]
    pub channel: Option<Channel>,
    pub guild_id: Option<String>,
    pub member: Option<Member>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Author {
//     pub avatar_decoration: Option<String>,
//     pub avatar: String,
//     pub discriminator: String,
//     pub global_name: String,
//     pub id: String,
//     pub public_flags: u64,
//     pub username: String,
// }

fn deserialize_message_type<'de, D>(deserializer: D) -> Result<MessageType, D::Error>
where
    D: Deserializer<'de>,
{
    let message_type_index: u16 = Deserialize::deserialize(deserializer)?;
    
    match message_type_index {
        0 => Ok(MessageType::Default),
        1 => Ok(MessageType::RecipientAdd),
        2 => Ok(MessageType::RecipientRemove),
        3 => Ok(MessageType::Call),
        4 => Ok(MessageType::ChannelNameChange),
        5 => Ok(MessageType::ChannelIconChange),
        6 => Ok(MessageType::ChannelPinnedMessage),
        7 => Ok(MessageType::UserJoin),
        8 => Ok(MessageType::GuildBoost),
        9 => Ok(MessageType::GuildBoostTier1),
        10 => Ok(MessageType::GuildBoostTier2),
        11 => Ok(MessageType::GuildBoostTier3),
        12 => Ok(MessageType::ChannelFollowAdd),
        // 13 is purposefully skipped
        14 => Ok(MessageType::GuildDiscoveryDisqualified),
        15 => Ok(MessageType::GuildDiscoveryRequalified),
        16 => Ok(MessageType::GuildDiscoveryGracePeriodInitialWarning),
        17 => Ok(MessageType::GuildDiscoveryGracePeriodFinalWarning),
        18 => Ok(MessageType::ThreadCreated),
        19 => Ok(MessageType::Reply),
        20 => Ok(MessageType::ChatInputCommand),
        21 => Ok(MessageType::ThreadStarterMessage),
        22 => Ok(MessageType::GuildInviteReminder),
        23 => Ok(MessageType::ContextMenuCommand),
        24 => Ok(MessageType::AutoModerationAction),
        25 => Ok(MessageType::RoleSubscriptionPurchase),
        26 => Ok(MessageType::InteractionPremiumUpsell),
        27 => Ok(MessageType::StageStart),
        28 => Ok(MessageType::StageEnd),
        29 => Ok(MessageType::StageSpeaker),
        30 => Ok(MessageType::StageTopic),
        31 => Ok(MessageType::GuildApplicationPremiumSubscription),
        _ => Err(serde::de::Error::custom("Invalid message type index"))
    }
}

// impl TryFrom<Value> for Message {
//     type Error = &'static str;

//     fn try_from(value: Value) -> Result<Message, &'static str> {
//         let message = value.try_into();

//         if let Ok(message) = message {
//             return Ok(message);
//         }

//         panic!("Failed to deserialize message from value");
//     }
// }

impl From<Value> for Message {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}