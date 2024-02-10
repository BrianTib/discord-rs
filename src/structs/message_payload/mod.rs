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

impl MessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }
}