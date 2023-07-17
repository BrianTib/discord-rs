use serde::{Serialize, Deserialize, Deserializer};

use crate::structs::locale::{
    Locale,
    locale_optional_deserializer
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    pub avatar: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    #[serde(default, deserialize_with = "locale_optional_deserializer")]
    pub locale: Option<Locale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "user_flags_deserializer")]
    pub flags: Option<Vec<UserFlag>>,
    #[serde(default, deserialize_with = "premium_type_deserializer")]
    pub premium_type: Option<PremiumType>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "user_flags_deserializer")]
    pub public_flags: Option<Vec<UserFlag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserFlag {
    Staff,
    Partner,
    HypeSquad,
    BugHunterLevelOne,
    HypeSquadBraveryHouse,
    HypeSquadBrillianceHouse,
    HypeSquadBalanceHouse,
    EarlyNitroSupporter,
    TeamPseudoUser,
    BugHunterLevelTwo,
    VerifiedBot,
    VerifiedDeveloper,
    CertifiedModerator,
    BotHTTPInteractions,
    ActiveDeveloper
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PremiumType {
    None,
    NitroClassic,
    Nitro,
    NitroBasic
}

pub fn user_flags_deserializer<'de, D>(deserializer: D) -> Result<Option<Vec<UserFlag>>, D::Error>
where
    D: Deserializer<'de>,
{
    let bits: u32 = Deserialize::deserialize(deserializer)?;
    let flags = [
        UserFlag::Staff,
        UserFlag::Partner,
        UserFlag::HypeSquad,
        UserFlag::BugHunterLevelOne,
        UserFlag::HypeSquadBraveryHouse,
        UserFlag::HypeSquadBrillianceHouse,
        UserFlag::HypeSquadBalanceHouse,
        UserFlag::EarlyNitroSupporter,
        UserFlag::TeamPseudoUser,
        UserFlag::BugHunterLevelTwo,
        UserFlag::VerifiedBot,
        UserFlag::VerifiedDeveloper,
        UserFlag::CertifiedModerator,
        UserFlag::BotHTTPInteractions,
        UserFlag::ActiveDeveloper
    ];

    let enabled_flags = flags
        .iter()
        .enumerate()
        .filter(|&(index, _)| bits & (1 << index) != 0)
        .map(|(_, flag)| flag.clone())
        .collect();

    Ok(Some(enabled_flags))
}

pub fn premium_type_deserializer<'de, D>(deserializer: D) -> Result<Option<PremiumType>, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u16 = Deserialize::deserialize(deserializer)?;
    let premium_type = match index {
        0 => PremiumType::None,
        1 => PremiumType::NitroClassic,
        2 => PremiumType::Nitro,
        3 => PremiumType::NitroBasic,
        _ => unimplemented!("Got unexpected type of user premium type: {}", index),
    };
    Ok(Some(premium_type))
}