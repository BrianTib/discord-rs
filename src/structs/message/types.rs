use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::structs::{
    channel::{Channel, ChannelMention},
    embed::Embed,
    member::Member,
    role::Role,
    reaction::Reaction,
    message::enums::{MessageType, MessageActivity, AllowedMentionsType},
    attachment::Attachment
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub attachments: Vec<Attachment>,
    pub author: Author,
    pub channel_id: String,
    #[serde(skip)]
    pub channel: Option<Channel>,
    pub components: Vec<Value>,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: u64,
    pub guild_id: Option<String>,
    pub id: String,
    pub member: Option<Member>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<Role>,
    pub mentions: Vec<Value>,
    pub pinned: bool,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub webhook_id: Option<String>,
    pub activity: Option<MessageActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    pub timestamp: String,
    pub tts: bool,
    #[serde(rename = "type", deserialize_with = "deserialize_message_type")]
    pub message_type: MessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub avatar_decoration: Option<String>,
    pub avatar: String,
    pub discriminator: String,
    pub global_name: String,
    pub id: String,
    pub public_flags: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowedMentions {
    pub parse: Vec<AllowedMentionsType>,
    // Max size 100
    pub roles: Vec<String>,
    // Max size 100
    pub users: Vec<String>,
    pub replied_user: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: Option<bool>,
    pub allowed_mentions: Option<AllowedMentions>
}

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