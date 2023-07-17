use serde::{Deserialize, Serialize, Deserializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelType {
    GuildText,
    DM,
    GuildVoice,
    GroupDM,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PermissionType {
    Role,
    Member
}

pub fn permission_type_deserializer<'de, D>(deserializer: D) -> Result<PermissionType, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u8 = Deserialize::deserialize(deserializer)?;
    match index {
        0 => Ok(PermissionType::Role),
        1 => Ok(PermissionType::Member),
        _ => Err(serde::de::Error::custom("Invalid message type index"))
    }
}