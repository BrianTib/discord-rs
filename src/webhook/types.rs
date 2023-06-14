#[allow(dead_code)]
use crate::embed::Embed;
use reqwest::Client;

pub struct WebhookClient {
    pub id: String,
    pub token: String,
    pub client: Client
}

pub struct MessageCreateOptions {
    tts: Option<bool>,
    flags: Option<String>,
    username: Option<String>,
    avatar_url: Option<String>,
    thread_id: Option<String>,
    thread_name: Option<String>
}

pub struct MessageSendOptions<'a> {
    pub content: Option<String>,
    pub embeds: Option<Vec<&'a Embed>>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: Option<bool>,
}