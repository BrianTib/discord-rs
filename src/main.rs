#[allow(dead_code)]
// use dotenv::dotenv;
// use std::env;
// use std::collections::HashMap;

mod webhook;
use webhook::{WebhookClient};

mod embed;
use embed::Embed;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // https://discord.com/api/webhooks/1118160739799158794/
    let id = "1118160739799158794";
    let token = "cVoyyJSQEwgDicj3M41EDIhVoGiIg1PDTN4wmaywIBQooXCThOLG6KfTFhjwEbI8IPRq";

    let mut embed = Embed::new();
    embed
        .set_author(
            "Your mom",
            None,
            Some("https://e1.pxfuel.com/desktop-wallpaper/153/568/desktop-wallpaper-discord-neon-icon-discord-logo.jpg"),
            None
        )
        .set_title("This is a test title")
        .set_description("This is a description")
        .set_footer("This is a footer", None, None);

    let webhook = WebhookClient::new(id, token);
    let _ = webhook.send(
        Some("this is a test content".to_string()),
        Some(vec![&embed]),
        None,
        None,
        None
    ).await.map_err(|_e| println!("Error wee woo"));
    Ok(())
}

// fn test() {
//     let client = ClientBuilder::new("ws://127.0.0.1:3012");
// //     listen("127.0.0.1:3012", |out| {
// //         move |msg| {
// //             out.send(msg)
// //         }
// //     })
// }

// async fn main() {
//     dotenv().ok();
    
//     let database_url = env::var("TEST").expect("TEST must be set");
//     println!("{}", database_url);
// }


