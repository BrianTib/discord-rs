use serde::{Serialize, Deserialize};

use crate::structs::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    // TODO: ISO8601 timestamp
    pub joined_at: String,
    // TODO: ISO8601 timestamp
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u64,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    // TODO: ISO8601 timestamp
    pub communication_disabled_until: Option<String>
}