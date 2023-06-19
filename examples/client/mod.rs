use crate::client::{Client, GatewayIntentBits};
use dotenv;

pub async fn main() {
    let token = &dotenv::var("DISCORD_TOKEN").unwrap();

    let client = Client::new(token, &[
        GatewayIntentBits::Guilds,
        GatewayIntentBits::GuildMessages,
        GatewayIntentBits::DirectMessages,
    ]);

    client.login().expect("Failed to login");
}