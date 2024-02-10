pub mod builders;
pub(crate) mod managers;

pub mod structs {
    pub mod application_command;
    pub mod application;
    pub mod attachment;
    pub mod channel;
    pub mod client;
    pub mod embed;
    pub mod emoji;
    pub mod guild;
    pub mod locale;
    pub mod member;
    pub mod message_payload;
    pub mod message;
    pub mod nonce;
    pub mod permissions;
    pub mod presence;
    pub mod reaction;
    pub mod role;
    pub mod snowflake;
    pub mod sticker;
    pub mod timestamp;
    pub mod user;
    #[cfg(feature = "webhook")]
    pub mod webhook;
}

pub(crate) mod util {
    pub mod rest;
    pub mod threadpool;
    pub mod socket;
    pub mod env;
}