use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AllowedMentionsType {
    RoleMentions,
    UserMentions,
    EveryoneMentions
}