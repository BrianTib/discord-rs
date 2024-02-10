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