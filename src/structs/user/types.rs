use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub avatar: String,
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: u64,
    pub global_name: Option<String>,
    pub id: String,
    pub mfa_enabled: bool,
    pub username: String,
    pub verified: bool,
}