use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;
use std::cmp::PartialEq;
use serde_with::skip_serializing_none;
use serde_repr::Serialize_repr;

use crate::structs::channel::ChannelType;

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct ApplicationCommand {
    pub id: String,
    #[serde(default, rename = "type")]
    pub command_type: ApplicationCommandType,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub default_member_permissions: Option<String>,
    pub dm_permissions: Option<bool>,
    pub default_permissions: Option<bool>,
    pub nsfw: Option<bool>,
    pub version: String
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub command_type: ApplicationCommandOptionType,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
    pub required: Option<bool>,
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub channel_types: Option<ChannelType>,
    pub min_value: Option<ApplicationCommandOptionNumericChoiceValue>,
    pub max_value: Option<ApplicationCommandOptionNumericChoiceValue>,
    pub min_length: Option<u16>,
    pub max_length: Option<u16>,
    pub autocomplete: Option<bool>
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: Value
}

#[derive(Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3
}

#[derive(Serialize_repr, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11
}

#[derive(Serialize, Debug, Clone)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i64),
    Double(f64)
}

#[derive(Serialize, Debug, Clone)]
pub enum ApplicationCommandOptionNumericChoiceValue {
    Integer(i64),
    Double(f64)
}