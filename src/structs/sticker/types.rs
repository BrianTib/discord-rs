use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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
    pub format_type: StickerFormatType
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StickerType {
    Standard = 1,
    Guild = 2
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StickerFormatType {
    Png = 1,
    Apng = 2,
    Lottie = 3,
    Gif = 4
}