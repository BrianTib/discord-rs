use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embed {
    pub author: Option<EmbedAuthor>,
    pub color: Option<u32>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: u8,
    pub fields: Option<Vec<EmbedField>>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub provider: Option<EmbedProvider>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub timestamp: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub video: Option<EmbedVideo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedVideo {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EmbedTypes {
    Rich = 0,
    Image = 1,
    Video = 2,
    Gifv = 3,
    Article = 4,
    Link = 5
}