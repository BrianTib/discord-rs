use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
pub enum Nonce {
    String(String),
    Number(u64)
}

impl<'de> Deserialize<'de> for Nonce {
    fn deserialize<D>(deserializer: D) -> Result<Nonce, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::String(s) => Ok(Nonce::String(s)),
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    Ok(Nonce::Number(u))
                } else {
                    Err(serde::de::Error::custom("Invalid u64 value"))
                }
            }
            _ => Err(serde::de::Error::custom("Invalid nonce value")),
        }
    }
}