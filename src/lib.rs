//! # Discord-RS
//!
//! `discord-rs` is a Rust library that provides utilities for interacting with the Discord API.
//!
//! ## Features
//!
//! - Rich Embeds: Create rich embeds that can be used in various Discord contexts, such as webhooks and bots.
//! - Webhooks: Manage and interact with individual webhooks, allowing for fine-grained control and customization.
//! - Bots: Develop powerful Discord bots that can interact with servers and users, implementing custom functionality.
//!
//! ## Modules
//!
//! - `client`: Provides a client implementation for connecting to the Discord API and handling events.
//! - `structs`: Contains several object definitions by the rust API into rust-friendly structs
//! - `util`: Contains useful console logging
//! 
//! For detailed usage examples, please refer to the documentation of each module.

pub mod client;
pub mod managers;
pub mod structs {
    pub mod application;
    pub mod attachment;
    pub mod channel;
    pub mod embed;
    pub mod emoji;
    pub mod guild;
    pub mod member;
    pub mod message;
    pub mod permissions;
    pub mod reaction;
    pub mod role;
    pub mod user;
    pub mod webhook;
    pub mod sticker;
}