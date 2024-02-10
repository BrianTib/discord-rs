use chrono::DateTime;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, PartialEq)]
pub enum Timestamp {
    String(String),
    Number(usize)
}

impl Timestamp {
    /// A [Timestamp] of the current amount of seconds since [UNIX_EPOCH]
    pub fn now() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Timestamp::Number(timestamp as usize)
    }

    /// The string representation of the [Timestamp].
    /// # Example
    /// 2023-11-21T14:52:38.313Z
    pub fn as_string(&self) -> String {
        match self {
            Timestamp::String(t) => t.clone(),
            Timestamp::Number(t) => {
                let datetime = DateTime::from_timestamp(*t as i64, 0).unwrap();
                datetime.to_rfc3339()
            },
        }
    }
}

impl From<String> for Timestamp {
    fn from(value: String) -> Self {
        if let Ok(number) = value.parse::<usize>() {
            Timestamp::Number(number)
        } else {
            // Attempt to parse the ISO 8601 timestamp string
            if let Ok(parsed_time) = DateTime::parse_from_rfc3339(&value) {
                // Calculate the duration since UNIX epoch and convert it to seconds
                let duration_since_epoch = parsed_time.timestamp() as usize;
                Timestamp::Number(duration_since_epoch)
            } else {
                Timestamp::String(value)
            }
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Timestamp::String(s) => write!(f, "{}", s),
            Timestamp::Number(n) => {
                let datetime = DateTime::from_timestamp(*n as i64, 0).unwrap();
                write!(f, "{} [EPOCH {}]", datetime.to_rfc3339(), n)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::String(s) => Ok(Timestamp::String(s)),
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    Ok(Timestamp::Number(u as usize))
                } else {
                    Err(serde::de::Error::custom("Invalid u64 value"))
                }
            }
            _ => Err(serde::de::Error::custom("Invalid timestamp value")),
        }
    }
}