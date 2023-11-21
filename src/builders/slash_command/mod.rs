use std::collections::HashMap;
use std::any::Any;
use serde_json::Value;

use crate::structs::{
    snowflake::SnowflakeBuilder,
    locale::Locale,
    application_command::{
        ApplicationCommand,
        ApplicationCommandOption,
        ApplicationCommandOptionType,
        ApplicationCommandType,
        ApplicationCommandOptionChoice
    }
};

pub type SlashCommandBuilder = ApplicationCommand;
pub type SlashCommandOptionBuilder = ApplicationCommandOption;
pub type SlashCommandOptionType = ApplicationCommandOptionType;

impl SlashCommandBuilder {
    pub fn new(application_id: &str) -> Self {
        let mut snowflake = SnowflakeBuilder::new(1);

        Self {
            id: snowflake.generate_id().to_string(),
            command_type: ApplicationCommandType::ChatInput,
            application_id: application_id.to_string(),
            guild_id: None,
            name: String::new(),
            name_localizations: None,
            description: String::new(),
            description_localizations: None,
            options: None,
            default_member_permissions: None,
            dm_permissions: None,
            default_permissions: None,
            nsfw: None,
            version: snowflake.generate_id().to_string(),
        }
    }

    /// Sets the name of the slash command
    /// # Panics
    /// This function panics if `name` has more than 32 characters
    /// or fails to pass the RegEx check against `^[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$`
    pub fn set_name(mut self, name: &str) -> Self {
        let length = name.len() as u32;
        if length < 1 || length > 32 {
            panic!("'{name}' is not the right length! Slash command names must be between 1-32 characters");
        }
        
        if !is_valid_name(name) {
            panic!("{name} is not a valid SlashCommandOption name! Did not pass RegExp test");
        }

        self.name = name.to_string();
        self
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Sets [ApplicationCommand] `name` localizations
    pub fn add_name_localization(mut self, locale: Locale, translation: String) -> Self {
        let name_localizations = self.name_localizations.get_or_insert_with(HashMap::new);
        name_localizations.insert(locale.to_string(), translation);
        self
    }

    /// Appends a collection of [ApplicationCommand] `name` localizations at once
    pub fn add_name_localizations(mut self, localizations: &[(Locale, String)]) -> Self {
        let name_localizations = self.name_localizations.get_or_insert_with(HashMap::new);
        for &(locale, ref translation) in localizations {
            name_localizations.entry(locale.to_string()).or_insert(translation.clone());
        }
        self
    }

    /// Sets a collection of [ApplicationCommand] `name` localizations at once
    pub fn set_name_localization(mut self, localizations: HashMap<Locale, String>) -> Self {
        let mut name_localizations = HashMap::<String, String>::new();
        for (locale, translation) in localizations {
            name_localizations.entry(locale.to_string()).or_insert(translation.clone());
        }
        
        self.name_localizations = Some(name_localizations);
        self
    }

    /// Sets a collection of [ApplicationCommand] `description` localizations at once
    pub fn add_description_localizations(mut self, localizations: &[(Locale, String)]) -> Self {
        let description_localizations = self.description_localizations.get_or_insert_with(HashMap::new);
        for &(locale, ref translation) in localizations {
            description_localizations.entry(locale.to_string()).or_insert(translation.clone());
        }
        self
    }

    /// Sets a singular [ApplicationCommand] `description` localization
    pub fn add_description_localization(mut self, locale: Locale, translation: String) -> Self {
        let description_localizations = self.description_localizations.get_or_insert_with(HashMap::new);
        description_localizations.insert(locale.to_string(), translation);
        self
    }

    /// Adds a [ApplicationCommandOptionType::String] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::String]
    pub fn add_string_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::String)
    }

    /// Adds a [ApplicationCommandOptionType::Integer] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Integer]
    pub fn add_integer_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Integer)
    }

    /// Adds a [ApplicationCommandOptionType::Boolean] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Boolean]
    pub fn add_boolean_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Boolean)
    }

    /// Adds a [ApplicationCommandOptionType::User] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::User]
    pub fn add_user_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::User)
    }

    /// Adds a [ApplicationCommandOptionType::Channel] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Channel]
    pub fn add_channel_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Channel)
    }

    /// Adds a [ApplicationCommandOptionType::Role] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Role]
    pub fn add_role_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Role)
    }

    /// Adds a [ApplicationCommandOptionType::Mentionable] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Mentionable]
    pub fn add_mentionable_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Mentionable)
    }

    /// Adds a [ApplicationCommandOptionType::Number] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Number]
    pub fn add_number_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Number)
    }

    /// Adds a [ApplicationCommandOptionType::Attachment] option to the Slash Command
    /// # Panics
    /// Panics if `option` is not of type [ApplicationCommandOptionType::Attachment]
    pub fn add_attachment_option(self, option: ApplicationCommandOption) -> Result<Self, String> {
        Self::add_option(self, option, ApplicationCommandOptionType::Attachment)
    }

    fn add_option(
        mut self,
        option: ApplicationCommandOption,
        intended_type: ApplicationCommandOptionType
    ) -> Result<Self, String> {
        // Check that this type supports options
        if self.command_type != ApplicationCommandType::ChatInput {
            return Err(String::from("Options can only be added to ChatInput application commands"));
        }

        if option.command_type != intended_type {
            return Err(format!("Invalid type! Expected {:?} but got {:?}", intended_type, option.command_type));
        }
        
        match self.options {
            Some(ref mut options) => options.push(option),
            None => self.options = Some(vec![option]),
        }

        Ok(self)
    }

    pub fn set_nsfw(mut self, state: bool) -> Self {
        self.nsfw = Some(state);
        self
    }

    /// Whether or not the Slash Command can be used in DMs. Defaults to `true`
    pub fn set_dm_permission(mut self, state: bool) -> Self {
        self.nsfw = Some(state);
        self
    }

    pub fn set_default_member_permissions(mut self, default_member_permissions: &str) -> Self {
        self.default_member_permissions = Some(default_member_permissions.to_owned());
        self
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    // pub fn build(self) -> ApplicationCommand {
    //     ApplicationCommand { ..self }
    // }
}

impl SlashCommandOptionBuilder {
    pub fn new(option_type: SlashCommandOptionType) -> Self {
        Self {
            command_type: option_type,
            name: String::new(),
            name_localizations: None,
            description: String::new(),
            description_localizations: None,
            required: None,
            choices: None,
            options: None,
            channel_types: None,
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            autocomplete: None,
        }
    }

    /// Sets the name of the command option
    /// # Panic
    /// This function panics if `name` has more than 100 characters
    /// or fails to pass the RegEx check against `^[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$`
    pub fn set_name(mut self, name: &str) -> Self {
        let length = name.len() as u32;
        if length < 1 || length > 100 {
            panic!("'{name}' is not the right length! Command option names must be between 1-100 characters");
        }
        
        if !is_valid_name(name) {
            panic!("{name} is not a valid SlashCommandOption name! Did not pass RegExp test");
        }

        self.name = name.to_string();
        self
    }

    /// Sets the description of the command option
    pub fn set_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn add_choice<T>(
        mut self,
        name: &str,
        value: T,
        localizations: Option<HashMap<Locale, String>>
    ) -> Self
    where T: Any {
        let mut option_choice = ApplicationCommandOptionChoice {
            name: name.to_string(),
            value: cast_choice_value(value).unwrap(),
            name_localizations: None
        };
    
        if let Some(map) = localizations {
            let name_localizations: HashMap<String, String> = map
                .into_iter()
                .map(|(locale, translation)| (locale.to_string(), translation))
                .collect();
    
            option_choice.name_localizations = Some(name_localizations);
        }
    
        self.choices.get_or_insert_with(Vec::new).push(option_choice);
        self
    }

    // pub fn add_choices<T>(mut self, choices: &[(String, T, Option<HashMap<Locale, String>>)]) -> Self
    // where T: Any {
    //     for choice in choices.iter() {
    //         self = self.add_choice(choice.clone());
    //     }
        
    //     self
    // }

    /// Sets a collection of [ApplicationCommandOption] `name` localizations at once
    pub fn add_name_localizations(mut self, localizations: &[(Locale, String)]) -> Self {
        let name_localizations = self.name_localizations.get_or_insert_with(HashMap::new);
        for &(locale, ref translation) in localizations {
            name_localizations.entry(locale.to_string()).or_insert(translation.clone());
        }
        self
    }

    /// Sets a singular [ApplicationCommandOption] `name` localization
    pub fn add_name_localization(mut self, locale: Locale, translation: String) -> Self {
        let name_localizations = self.name_localizations.get_or_insert_with(HashMap::new);
        name_localizations.insert(locale.to_string(), translation);
        self
    }

    /// Sets a collection of [ApplicationCommandOption] `description` localizations at once
    pub fn add_description_localizations(mut self, localizations: &[(Locale, String)]) -> Self {
        let description_localizations = self.description_localizations.get_or_insert_with(HashMap::new);
        for &(locale, ref translation) in localizations {
            description_localizations.entry(locale.to_string()).or_insert(translation.clone());
        }
        self
    }

    /// Sets a singular [ApplicationCommandOption] `description` localization
    pub fn add_description_localization(mut self, locale: Locale, translation: String) -> Self {
        let description_localizations = self.description_localizations.get_or_insert_with(HashMap::new);
        description_localizations.insert(locale.to_string(), translation);
        self
    }

    pub fn set_required(mut self, state: bool) -> Self {
        self.required = Some(state);
        self
    }
}

/// Aims to be similar to matching to the RegEx `^[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$`
fn is_valid_name(name: &str) -> bool {
    let mut valid_chars: Vec<char> = "-_".chars().collect();
    valid_chars.extend(('a'..='z').chain('A'..='Z').chain('0'..='9'));

    let thai_chars: Vec<char> = vec![
        'ก', 'ข', 'ค', 'ง', 'จ', 'ฉ', 'ช', 'ซ', 'ฌ',
        'ญ', 'ฎ', 'ฏ', 'ฐ', 'ฑ', 'ฒ', 'ณ', 'ด', 'ต',
        'ถ', 'ท', 'ธ', 'น', 'บ', 'ป', 'ผ', 'ฝ', 'พ',
        'ฟ', 'ภ', 'ม', 'ย', 'ร', 'ล', 'ว', 'ศ', 'ษ',
        'ส', 'ห', 'ฬ', 'อ', 'ฮ'
    ];

    let devanagari_chars: Vec<char> = vec![
        'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ',
        'ए', 'ऐ', 'ओ', 'औ', 'ं', 'ः', 'ँ'
    ];

    for c in name.chars() {
        if !valid_chars.contains(&c) && !thai_chars.contains(&c) && !devanagari_chars.contains(&c) {
            return false;
        }
    }

    true
}

fn validate_length(string: &str, max_length: u32, min_length: Option<u32>) -> Result<(), String> {
    if string.len() as u32 > max_length {
        return Err(format!("Length of '{}' exceeds {} characters", string, max_length));
    }

    if let Some(min_length) = min_length {
        if (string.len() as u32) < min_length {
            return Err(format!("Length of '{}' is less than {} characters", string, min_length));
        }
    }

    Ok(())
}

fn cast_choice_value<T>(value: T) -> Result<Value, &'static str>
where
    T: Any
{
    let type_name = std::any::type_name::<T>();

    match type_name {
        "std::string::String" => {
            if let Some(string_value) = (&value as &dyn Any).downcast_ref::<String>() {
                return Ok(Value::String(string_value.clone()));
            }
        },
        "&str" => {
            if let Some(str_value) = (&value as &dyn Any).downcast_ref::<&str>() {
                return Ok(Value::String(str_value.to_string()));
            }
        },
        "i64" | "i32" => {
            if let Some(integer_value) = (&value as &dyn Any).downcast_ref::<i64>() {
                return Ok(Value::Number(serde_json::Number::from(*integer_value)));
            }
            if let Some(integer_value) = (&value as &dyn Any).downcast_ref::<i32>() {
                return Ok(Value::Number(serde_json::Number::from(*integer_value)));
            }
        },
        "f64" | "f32" => {
            if let Some(float_value) = (&value as &dyn Any).downcast_ref::<f64>() {
                return Ok(Value::Number(serde_json::Number::from_f64(*float_value).unwrap()));
            }
            if let Some(float_value) = (&value as &dyn Any).downcast_ref::<f32>() {
                return Ok(Value::Number(serde_json::Number::from_f64(f64::from(*float_value)).unwrap()));
            }
        },
        _ => return Err("Not castable"),
    }

    Err("Not castable")
}