use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f32>,
    pub waveform: Option<String>
}