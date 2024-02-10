use serde::{Deserializer, Deserialize};

use super::{PermissionType, ChannelType};

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

pub fn channel_type_deserializer<'de, D>(deserializer: D) -> Result<ChannelType, D::Error>
where
    D: Deserializer<'de>,
{
    let channel_type_index: u8 = Deserialize::deserialize(deserializer)?;

    match channel_type_index {
        0 => Ok(ChannelType::GuildText),
        1 => Ok(ChannelType::DM),
        2 => Ok(ChannelType::GuildVoice),
        3 => Ok(ChannelType::GroupDM),
        4 => Ok(ChannelType::GuildCategory),
        5 => Ok(ChannelType::GuildAnnouncement),
        10 => Ok(ChannelType::AnnouncementThread),
        11 => Ok(ChannelType::PublicThread),
        12 => Ok(ChannelType::PrivateThread),
        13 => Ok(ChannelType::GuildStageVoice),
        14 => Ok(ChannelType::GuildDirectory),
        15 => Ok(ChannelType::GuildForum),
        _ => Err(serde::de::Error::custom("Invalid message type index"))
    }
}