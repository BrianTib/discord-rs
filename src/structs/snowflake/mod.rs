use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Value;

pub struct SnowflakeBuilder {
    worker_id: u16,
    sequence: u16,
    /// Milliseconds since Discord Epoch, the first second of 2015
    epoch: u64,
    last_timestamp: u64,
}

/// A [snowflake ID](https://en.wikipedia.org/wiki/Snowflake_ID) adjusted to
/// fit [Discord's implementation of snowflakes](https://discord.com/developers/docs/reference#snowflakes)
#[derive(Debug, Serialize)]
pub enum Snowflake {
    String(String),
    Number(u64)
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Snowflake, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::String(s) => Ok(Snowflake::String(s)),
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    return Ok(Snowflake::Number(u))
                }

                Err(serde::de::Error::custom("Invalid u64 value"))
            }
            _ => Err(serde::de::Error::custom("Invalid nonce value")),
        }
    }
}

impl ToString for Snowflake {
    fn to_string(&self) -> String {
        match self {
            Snowflake::String(s) => s.to_owned(),
            Snowflake::Number(s) => s.to_string(),
        }
    }
}

impl From<Snowflake> for u64 {
    fn from(value: Snowflake) -> Self {
        match value {
            Snowflake::String(s) => s.parse::<u64>().unwrap_or(0),
            Snowflake::Number(s) => s,
        }
    }
}

impl SnowflakeBuilder {
    pub fn new(worker_id: u16) -> Self {
        Self {
            worker_id,
            sequence: 0,
            epoch: 1420070400000,
            last_timestamp: 0,
        }
    }

    pub fn generate_id(&mut self) -> Snowflake {
        let timestamp = self.get_timestamp();
        
        if timestamp < self.last_timestamp {
            panic!("Clock moved backwards!");
        }

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & 0xFFF; // 12-bit sequence number
            if self.sequence == 0 {
                self.wait_next_millisecond();
                self.sequence = 1;
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        // Construct the Snowflake ID using a 64-bit format:
        // - 42 bits for timestamp (ms)
        // - 10 bits for worker ID
        // - 12 bits for sequence number
        let snowflake = ((timestamp - self.epoch) << 22) | ((self.worker_id as u64) << 12) | self.sequence as u64;
        Snowflake::Number(snowflake)
    }

    pub fn timestamp_to_snowflake(&self, timestamp: u64) -> Snowflake {
        Snowflake::Number((timestamp - self.epoch) << 22)
    }
    
    pub fn snowflake_to_timestamp(&self, snowflake: Snowflake) -> u64 {
        //let snowflake = snowflake.parse::<u64>().unwrap_or(0);
        let snowflake = u64::from(snowflake);
        (snowflake >> 22) + self.epoch
    }

    fn get_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock went backwards")
            .as_millis() as u64
    }

    fn wait_next_millisecond(&self) {
        let mut timestamp = self.get_timestamp();
        while timestamp <= self.last_timestamp {
            timestamp = self.get_timestamp();
        }
    }
}