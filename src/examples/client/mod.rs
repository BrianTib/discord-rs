use crate::client::{Client, GatewayIntentBits};

pub async fn run() {
    let mut client = Client::new(&[
        GatewayIntentBits::Guilds,
        GatewayIntentBits::GuildMessages,
        GatewayIntentBits::DirectMessages,
    ]);

    client.login("test").await.unwrap();
}