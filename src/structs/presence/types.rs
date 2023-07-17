use serde::{Serialize, Deserialize, Deserializer};

use crate::structs::{
    user::User,
    emoji::Emoji
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Presence {
    pub user: User,
    pub guild_id: String,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: UserClientStatus
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type", deserialize_with = "activity_type_deserializer")]
    pub activity_type: ActivityType,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamp>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAsset>,
    pub secrets: Option<ActivitySecret>,
    pub instance: bool,
    #[serde(deserialize_with = "activity_flag_deserializer")]
    pub flags: Vec<ActivityFlag>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityFlag {
    Instance,
    Join,
    Spectate,
    JoinRequest,
    Sync,
    Play,
    PartyPrivacyFriends,
    PartyPrivacyVoice,
    Embedded
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivitySecret {
    pub join: Option<String>,
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    pub match_secret: Option<String> 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityAsset {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<(u32, u32)>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityTimestamp {
    pub start: Option<u64>,
    pub end: Option<u64>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityType {
    Game,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing
}

fn activity_type_deserializer<'de, D>(deserializer: D) -> Result<ActivityType, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u32 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(ActivityType::Game),
        1 => Ok(ActivityType::Streaming),
        2 => Ok(ActivityType::Listening),
        3 => Ok(ActivityType::Watching),
        4 => Ok(ActivityType::Custom),
        5 => Ok(ActivityType::Competing),
        _ => {
            // Handle unrecognized feature string
            // You can choose to return an error or fallback to a default value
            unimplemented!("Unrecognized activity type: {}", index);
        }
    }
}

fn activity_flag_deserializer<'de, D>(deserializer: D) -> Result<Vec<ActivityFlag>, D::Error>
where
    D: Deserializer<'de>,
{
    let bits: u64 = Deserialize::deserialize(deserializer)?;
    let flags = [
        ActivityFlag::Instance,
        ActivityFlag::Join,
        ActivityFlag::Spectate,
        ActivityFlag::JoinRequest,
        ActivityFlag::Sync,
        ActivityFlag::Play,
        ActivityFlag::PartyPrivacyFriends,
        ActivityFlag::PartyPrivacyVoice,
        ActivityFlag::Embedded
    ];

    let enabled_flags: Vec<_> = flags
        .iter()
        .enumerate()
        .filter(|&(index, _)| bits & (1 << index) != 0)
        .map(|(_, flag)| flag.clone())
        .collect();

    Ok(enabled_flags)
}