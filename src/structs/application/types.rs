use serde::{Deserialize, Serialize};

use crate::structs::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,
    // Will be deprecated in v11
    pub summary: String,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<String>,
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<u64>,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<InstallParams>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub icon: Option<String>,
    pub id: String,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMember {
    pub membership_state: MembershipState,
    pub permissions: Vec<String>,
    pub team_id: String,
    pub user: User
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MembershipState {
    Invited,
    Accepted
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String
}