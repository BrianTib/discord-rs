use dotenv;
use std::collections::HashMap;

use discord_rs::structs::client::{
    Client,
    GatewayIntents,
    ExternalDispatchEvent
};

use discord_rs::structs::{
    message::Message,
    locale::Locale,
    message_payload::MessagePayload
};

use discord_rs::builders::{
    ClientBuilder,
    SlashCommandBuilder,
    SlashCommandOptionBuilder,
    SlashCommandOptionType
};

pub fn main() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = &[
        GatewayIntents::Guilds,
        GatewayIntents::GuildMessages,
        GatewayIntents::DirectMessages,
        GatewayIntents::MessageContent
    ];

    let mut client = ClientBuilder::new(&token, intents);

    let events = client.connect()
        .expect("Failed to login");
    
    loop {
        if let Ok((event_type, event_data)) = events.recv() {
            match event_type {
                ExternalDispatchEvent::Ready => {
                    //on_ready(&client);
                },
                ExternalDispatchEvent::MessageCreate => {
                    // NOTE: This currently breaks
                    on_message(&mut client, event_data.into());
                },
                _ => {}
            }
        }
    }
}

fn on_ready(
    _client: &mut Client,
) {
    println!("Received on_ready command");
    
    let command = SlashCommandBuilder::new("12345678987654321")
        .set_name("test-command")
        .set_description("A command that tests things")
        .set_dm_permission(false)
        .add_name_localizations(&[
            (Locale::Spanish, String::from("commando-prueba")),
            (Locale::French, String::from("commando-prueba"))
        ])
        .add_string_option(
            SlashCommandOptionBuilder::new(SlashCommandOptionType::String)
                .set_name("favorite_number")
                .set_description("Your favorite number")
                .set_required(true)
                .add_choice("One", "1", None)
                .add_choice(
                    "Two",
                    "2",
                    Some(HashMap::<Locale, String>::from([
                        (Locale::Spanish, "Dos".to_string()),
                        (Locale::French, "Deux".to_string())
                    ]))
                )
        ).unwrap()
        .add_integer_option(
            SlashCommandOptionBuilder::new(SlashCommandOptionType::Integer)
                .set_name("usertwo")
                .set_description("The user to test on")
                .set_required(true)
                .add_choice("One", 100, None)
        ).unwrap();

    println!("Calling register command: {:#?}", &command);
    // let _ = client.register_guild_commands(
    //     "1118990126480097442",
    //     &[command]
    // ).await;
}

fn on_message(
    client: &mut Client,
    message: Message
) {
    println!("{:?}: \"{}\"", message.author.global_name, message.content);

    if message.author.is_bot() { return; }

    let payload = MessagePayload::new()
        .set_content("Hello world");

    let channel = client.cache.get_channel(&message.channel_id.unwrap());
        
    if let Ok(channel) = channel {
        let _ = channel.send(payload);
    }

    //let _ = message.channel.send(payload);
    // message.send()
}

// async fn on_message(message: Message) {
//     if message.author.is_bot() { return; }
//     println!("Got message by {}: {}", message.author.username, message.content);
//     let _ = message.reply_content("Hello!").await;
//     //println!("Result: {:?}", res.unwrap());
// }