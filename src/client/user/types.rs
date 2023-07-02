#[allow(dead_code, unused_variables, unused_imports)]

pub struct ClientUser {
    pub application: Application,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub guild_join_requests: Vec<String>,
    pub presences: Vec<String>,
    pub sesion_id: String,
    pub session_type: String,
    pub user: User,
    pub user_settings: Option<String>,
    pub v: u32
}

pub struct Application {
    pub flags: u64,
    pub id: String,
}

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
    pub verified: bool
}