use chrono::DateTime;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Timestamp {
    String(String),
    Number(usize)
}

impl Timestamp {
    pub fn now() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        Timestamp::Number(timestamp as usize)
    }

    pub fn to_string(&self) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_timestamp() {
        let string_timestamp: Timestamp = String::from("2023-11-21T14:52:38.313Z").into();
        let number_timestamp = Timestamp::Number(1700578358);
        // Assert that both compile to the save value
        assert_eq!(string_timestamp, number_timestamp);
    }
}