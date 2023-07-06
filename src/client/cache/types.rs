#[allow(dead_code, unused_variables, unused_imports)]
use serde::{Serialize, Deserialize};

use crate::structs::user::User;

#[derive(Deserialize, Clone, Debug)]
pub struct ClientCache {
    pub application: Option<Application>,
    pub geo_ordered_rtc_regions: Option<Vec<String>>,
    pub guild_join_requests: Option<Vec<String>>,
    pub guilds: Option<Vec<Guild>>,
    pub presences: Option<Vec<String>>,
    pub private_channels: Option<Vec<String>>,
    pub relationships: Option<Vec<String>>,
    pub resume_gateway_url: Option<String>,
    pub session_id: Option<String>,
    pub session_type: Option<String>,
    pub user: Option<User>,
    pub user_settings: Option<UserSettings>,
    pub v: Option<u32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Application {
    pub flags: u64,
    pub id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Guild {
    pub id: String,
    pub unavailable: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserSettings {}