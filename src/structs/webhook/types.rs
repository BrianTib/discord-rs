#[allow(dead_code)]
use crate::structs::embed::Embed;

pub struct WebhookClient {
    pub id: Option<String>,
    pub token: Option<String>,
    pub url: Option<String>
}

pub struct MessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: Option<bool>,
}