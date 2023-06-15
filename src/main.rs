#[allow(dead_code)]
// use dotenv::dotenv;
// use std::env;
// use std::collections::HashMap;

pub mod client;
pub mod webhook;
pub mod embed;

mod examples {
    pub mod client;
    pub mod webhook;
}
fn main() {
    examples::client::run();
}