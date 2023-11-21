![alt text](/assets/discord-rs-banner-white.png)

# Discord-RS

**discord-rs** aims to be a blazingly fast  Rust library for interacting with the Discord API. It provides a simple and efficient way to build Discord bots, webhooks, create rich embeds, send messages, create slash commands, manage channels, and much more, all in the Rust programming language.

## Status

**discord-rs** is in it's very early infancy and is not yet usable. However, I will be working eagerly to get us to a functional point. Stay tuned!

You can keep up to date with development on our [Discord Server](https://discord.gg/RT4q6Y7Xkh) as well as receive one-on-one help from either me or other members of the community regarding best-practices, questions or anything else related to this package.

## Features
- Blazingly Fast!
- Intuitive
- Synchronous & Multi-Threaded
- Up to date (Uses Discord's API v10 which at the time of this being written is the latest version)

## Getting Started

Although not necessary, it is recommended to use `.env` variables alongside **discord-rs** for safe-keeping tokens. Here is a quick example of how to use `.env` variables in rust.

- Create a `.env` file in the root of your project (outside of your src folder)
- Inside the `.env` file you can write your token as such
```env
// .env
DISCORD_TOKEN = "YOUR_DISCORD_BOT_TOKEN"
```
- Inside your rust script, use `std::env`
```rust
// src/main.rs
use std::env;

fn main() {
  let token = &std::env::var("DISCORD_TOKEN").unwrap();
}
```

## Examples

### Creating a client
```rs
// src/main.rs
use dotenv;

use discord_rs::{
    structs::Message,
    builders::ClientBuilder,
    client::{
        Client,
        GatewayIntents,
        ExternalDispatchEvent
    },
};

fn main() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = &[
        atewayIntents::Guilds,
        atewayIntents::GuildMessages,
        atewayIntents::DirectMessages,
        atewayIntents::MessageContent
    ];

    let mut client = ClientBuilder::new(&token, intents);

    // Connect to the Gateway Discord API
    let events = client.connect()
        .expect("Failed to login");
    
    // Here you can either handle each event in-file or
    // separate each call into it's own handler
    loop {
        if let Ok((event_type, event_data)) = events.recv() {
            match event_type {
                ExternalDispatchEvent::Ready => {
                    on_ready(&client);
                },
                ExternalDispatchEvent::MessageCreate => {
                    on_message(&client, event_data.into());
                },
                _ => {}
            }
        }
    }
}

fn on_ready(_client: &Client) {
    println!("My bot is online!");
}

fn on_message(_client: &Client, message: Message) {
    println!("I received a message: {}", &message.content);
}
```

### Webhooks
Before you can setup webhooks using discord-rs, you will need to create a webhook integration. Go to the settings for the channel you want to make a webhook for and in the integrations tab select "Create Webhook". From there you can copy the webhook token and use it in your code.

```rust
// src/main.rs
use discord_rs::embed::Embed;
use discord_rs::webhook::{WebhookClient, MessagePayload};

fn main() {
    // Your webhook's credentials
    let id = "YOUR_WEBHOOK_ID";
    let token = "YOUR_WEBHOOK_TOKEN";

    // Create the webhook client
    let webhook = WebhookClient::new(id, token);

    let embed = Embed::new()
        .set_author(
            "Captain Hook",
            None,
            Some("https://site.com/icon_url.jpg"),
            None
        )
        .set_title("This is a test title")
        .set_description("This is a description")
        .set_footer("This is a footer", None, None);

    let message_payload =  MessagePayload::new()
        .set_username("Captain Hook")
        .set_content("Hello World!")
        .set_embeds(&[embed])
        .unwrap();

    webhook.send(message_payload)
        .expect("Failed to send webhook");
}
```

## Coffee = Code
If you can and want to support the project a cup of coffee goes a long way. ☕

[Paypal](https://www.paypal.me/bptiburcio)

## License
This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).