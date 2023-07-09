use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: i32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u64,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTag>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleTag {
    pub bot_id: Option<String>,
    pub integration_id: Option<String>,
    pub premium_subscriber: Option<bool>,
    pub subscription_listing_id: Option<String>,
    pub available_for_purchase: Option<bool>,
    pub guild_connections: Option<bool>
}