use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::client::ClientCache;

pub mod enums;
pub use enums::*;

pub mod types;
pub use types::*;

impl Message {
    pub async fn from(json: Value, cache: Arc<Mutex<ClientCache>>) -> Result<Self, &'static str> {
        let mut message: Self = serde_json::from_value(json).expect("Failed to parse message from JSON");
        
        if let Some(channel_id) = &message.channel_id {
            let mut cache = cache.lock().await;
            let channel = cache.channels.fetch_by_id(channel_id).await.unwrap();
            message.channel = Some(channel);
        }

        Ok(message)
    }

    /// Sends a text message to the channel
    pub async fn reply_content(&self, content: &str) -> Result<(), &'static str> {
        let mut payload = MessagePayload::new();
        payload.content = Some(content.to_string());
        Self::reply(self, payload).await
    }

    /// Whether or not the message can be deleted
    pub fn is_deletable(&self) -> bool {
        match self.message_type {
            MessageType::Default
            | MessageType::ChannelPinnedMessage
            | MessageType::UserJoin
            | MessageType::GuildBoost
            | MessageType::GuildBoostTier1
            | MessageType::GuildBoostTier2
            | MessageType::GuildBoostTier3
            | MessageType::ChannelFollowAdd
            | MessageType::GuildDiscoveryGracePeriodInitialWarning
            | MessageType::GuildDiscoveryGracePeriodFinalWarning
            | MessageType::Reply
            | MessageType::ChatInputCommand
            | MessageType::GuildInviteReminder
            | MessageType::ContextMenuCommand
            | MessageType::AutoModerationAction
            | MessageType::RoleSubscriptionPurchase
            | MessageType::InteractionPremiumUpsell
            | MessageType::StageStart
            | MessageType::StageEnd
            | MessageType::StageSpeaker
            | MessageType::StageTopic
            | MessageType::GuildApplicationPremiumSubscription => true,
            _ => false,
        }
    }

    // Sends payloads which may include text, embeds, tts and more to the channel
    pub async fn reply(&self, payload: MessagePayload) -> Result<(), &'static str> {
        if self.channel.is_none() {
            return Err("No channel available to reply");
        }

        let channel = self.channel.as_ref().unwrap();
        let _ = channel.send(payload).await;

        Ok(())
    }
}

impl MessagePayload {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: None,
            username: None,
            avatar_url: None,
            tts: None,
            allowed_mentions: None,
        }
    }
}

impl AllowedMentionsType {
    pub fn role_mentions() -> Self {
        Self::RoleMentions
    }

    pub fn user_mentions() -> Self {
        Self::UserMentions
    }

    pub fn everyone_mentions() -> Self {
        Self::EveryoneMentions
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::RoleMentions => "roles",
            Self::UserMentions => "users",
            Self::EveryoneMentions => "everyone",
        }
    }
}