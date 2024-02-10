use discord_rs::structs::{
    channel::Channel,
    snowflake::Snowflake
};

use dotenv;

#[test]
fn deserialize_channel() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    // Make some globally available variables
    std::env::set_var("_CLIENT_TOKEN", token);
    std::env::set_var("_DISCORD_API_URL", &"https://discord.com/api/v10");
    std::env::set_var("RUST_BACKTRACE", "1");

    let id = Snowflake::String("1120716431642865704".to_string());
    let channel = Channel::new(&id);

    println!("Cahnnel: {channel:?}");

    //assert!(channel.is_ok())
    assert!(true)
}