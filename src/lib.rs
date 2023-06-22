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
//! - `embed`: Defines structures and utilities for creating and manipulating rich embeds.
//! - `util`: Contains utility functions and helpers used throughout the library.
//! - `webhook`: Offers functionality for managing webhooks, including creation, deletion, and message sending.
//!
//! For detailed usage examples, please refer to the documentation of each module.

pub mod client;
pub mod embed;
pub mod util;
pub mod webhook;