use crate::client::{Client, GatewayIntentBits};

pub async fn run() {
    let token = "MTExOTAyNzI4MDE5NTM2MjgzNg.GT43gu.QPivhaZu3nmWoHytboTwDxm_kB4ZSCNcmApPEk";
    let client = Client::new(token, &[
        GatewayIntentBits::Guilds,
        GatewayIntentBits::GuildMessages,
        GatewayIntentBits::DirectMessages,
    ]);

    client.login().expect("Failed to login");
}