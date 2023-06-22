#[allow(dead_code)]
pub mod client;
pub mod embed;
pub mod util;
pub mod webhook;

pub mod threader;

mod example;

#[tokio::main]
async fn main() {
    example::main().await;
}