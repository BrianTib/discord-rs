#[allow(dead_code)]
pub mod client;
pub mod embed;
pub mod util;
pub mod webhook;

mod examples {
    pub mod client;
}

#[tokio::main]
async fn main() {
    examples::client::run().await;
}