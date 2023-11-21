mod types;
pub use types::*;

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

impl Default for MessagePayload {
    fn default() -> Self {
        Self {
            content: Default::default(),
            embeds: Default::default(),
            tts: Default::default(),
            allowed_mentions: Default::default(),
            nonce: Default::default(),
            message_reference: Default::default(),
            components: Default::default(),
            sticker_ids: Default::default(),
            files: Default::default(),
            payload_json: Default::default(),
            attachments: Default::default(),
            flags: Default::default(),
        }
    }
}

impl MessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }
}