use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Locale {
    Indonesian,
    Danish,
    German,
    EnglishUK,
    EnglisjUS,
    Spanish,
    French,
    Croatian,
    Italian,
    Lithuanian,
    Hungarian,
    Dutch,
    Norwegian,
    Polish,
    PortugueseBrazilian,
    Romania,
    Finnish,
    Swedish,
    Viernamese,
    Turkish,
    Czech,
    Greek,
    Bulgarian,
    Russian,
    Ukranian,
    Hindi,
    Thai,
    ChineseChina,
    Japonese,
    ChineseTaiwan,
    Korean
}

impl Default for Locale {
    fn default() -> Self {
        Self::EnglisjUS
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Locale::Indonesian => "id",
            Locale::Danish => "da",
            Locale::German => "de",
            Locale::EnglishUK => "en-GB",
            Locale::EnglisjUS => "en-US",
            Locale::Spanish => "es-ES",
            Locale::French => "fr",
            Locale::Croatian => "hr",
            Locale::Italian => "it",
            Locale::Lithuanian => "lt",
            Locale::Hungarian => "hu",
            Locale::Dutch => "nl",
            Locale::Norwegian => "no",
            Locale::Polish => "pl",
            Locale::PortugueseBrazilian => "pt-BR",
            Locale::Romania => "ro",
            Locale::Finnish => "fi",
            Locale::Swedish => "sv-SE",
            Locale::Viernamese => "vi",
            Locale::Turkish => "tr",
            Locale::Czech => "cs",
            Locale::Greek => "el",
            Locale::Bulgarian => "bg",
            Locale::Russian => "ru",
            Locale::Ukranian => "uk",
            Locale::Hindi => "hi",
            Locale::Thai => "th",
            Locale::ChineseChina => "zh-CN",
            Locale::Japonese => "ja",
            Locale::ChineseTaiwan => "zh-TW",
            Locale::Korean => "ko",
        };
        write!(f, "{}", string)
    }
}

pub fn locale_deserializer<'de, D>(deserializer: D) -> Result<Locale, D::Error>
where
    D: Deserializer<'de>,
{
    let locale_string: String = Deserialize::deserialize(deserializer)?;
    match locale_string.as_str() {
        "id" => Ok(Locale::Indonesian),
        "da" => Ok(Locale::Danish),
        "de" => Ok(Locale::German),
        "en-GB" => Ok(Locale::EnglishUK),
        "en-US" => Ok(Locale::EnglisjUS),
        "es-ES" => Ok(Locale::Spanish),
        "fr" => Ok(Locale::French),
        "hr" => Ok(Locale::Croatian),
        "it" => Ok(Locale::Italian),
        "lt" => Ok(Locale::Lithuanian),
        "hu" => Ok(Locale::Hungarian),
        "nl" => Ok(Locale::Dutch),
        "no" => Ok(Locale::Norwegian),
        "pl" => Ok(Locale::Polish),
        "pt-BR" => Ok(Locale::PortugueseBrazilian),
        "ro" => Ok(Locale::Romania),
        "fi" => Ok(Locale::Finnish),
        "sv-SE" => Ok(Locale::Swedish),
        "vi" => Ok(Locale::Viernamese),
        "tr" => Ok(Locale::Turkish),
        "cs" => Ok(Locale::Czech),
        "el" => Ok(Locale::Greek),
        "bg" => Ok(Locale::Bulgarian),
        "ru" => Ok(Locale::Russian),
        "uk" => Ok(Locale::Ukranian),
        "hi" => Ok(Locale::Hindi),
        "th" => Ok(Locale::Thai),
        "zh-CN" => Ok(Locale::ChineseChina),
        "ja" => Ok(Locale::Japonese),
        "zh-TW" => Ok(Locale::ChineseTaiwan),
        "ko" => Ok(Locale::Korean),
        _ => unimplemented!("Locale \"{}\" has not been implemented", locale_string),
    }
}

pub fn locale_optional_deserializer<'de, D>(deserializer: D) -> Result<Option<Locale>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Result<Locale, _> = Deserialize::deserialize(deserializer);
    match result {
        Ok(locale) => Ok(Some(locale)),
        Err(_) => Ok(None),
    }
}