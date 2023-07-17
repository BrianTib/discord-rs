use std::time::{SystemTime, UNIX_EPOCH};

pub struct SnowflakeBuilder {
    worker_id: u16,
    sequence: u16,
    epoch: u64,
    last_timestamp: u64,
}

pub type Snowflake = String;

impl SnowflakeBuilder {
    pub fn new(worker_id: u16) -> Self {
        Self {
            worker_id,
            sequence: 0,
            epoch: 1420070400000, // Milliseconds since Discord Epoch, the first second of 2015
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
        snowflake.to_string()
    }

    pub fn timestamp_to_snowflake(&self, timestamp: u64) -> Snowflake {
        ((timestamp - self.epoch) << 22).to_string()
    }
    
    pub fn snowflake_to_timestamp(&self, snowflake: Snowflake) -> u64 {
        let snowflake = snowflake.parse::<u64>().unwrap_or(0);
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