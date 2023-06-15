#[allow(dead_code)]
use reqwest::Error;

use crate::embed::Embed;
use crate::webhook::{WebhookClient, MessagePayload};

pub async fn run() -> Result<(), Error> {
    let id = "1118160739799158794";
    let token = "cVoyyJSQEwgDicj3M41EDIhVoGiIg1PDTN4wmaywIBQooXCThOLG6KfTFhjwEbI8IPRq";

    let mut embed = Embed::new();
    embed
        .set_author(
            "Embed Example",
            None,
            Some("https://e1.pxfuel.com/desktop-wallpaper/153/568/desktop-wallpaper-discord-neon-icon-discord-logo.jpg"),
            None
        )
        .set_title("This is a test title")
        .set_description("This is a description")
        .set_footer("This is a footer", None, None);

    let webhook = WebhookClient::new(id, token);

    let message_payload = MessagePayload::new()
        .set_username("Captain Hook")
        .set_content("Hello World!")
        .set_embeds(&[embed])
        .unwrap();

    webhook.send(message_payload).await.expect("Failed to send webhook");
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
