use serde::{Serialize, Deserialize};

use crate::structs::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticker {
    pub id: String,
    pub pack_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    // Deprecated
    pub asset: Option<String>,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormatType,
    pub available: Option<bool>,
    pub guild_id: Option<String>,
    pub user: Option<User>,
    pub sort_value: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StickerType {
    Standard = 1,
    Guild = 2
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StickerFormatType {
    Png = 1,
    Apng = 2,
    Lottie = 3,
    Gif = 4
}