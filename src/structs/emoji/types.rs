use serde::{Deserialize, Serialize};

use crate::structs::role::Role;
use crate::structs::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Emoji {
    pub id: String,
    pub name: Option<String>,
    pub roles: Option<Role>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>
}