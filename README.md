![alt text](/assets/discord-rs-banner-white.png)

# Discord-RS

**discord-rs** aims to be a blazingly fast  Rust library for interacting with the Discord API. It provides a simple and efficient way to build Discord bots, webhooks, create rich embeds, send messages, create slash commands, manage channels, and more.

## Status

**discord-rs** is in it's very early infancy and is not yet usable. However, I will be working eagerly to get us to a functional point. Stay tuned!

You can keep up to date with development on our [Discord Server](https://discord.gg/RT4q6Y7Xkh)

## Getting Started

It is recommended to use [dotenv](https://crates.io/crates/dotenv) alongside **discord-rs** for safe-keeping tokens. Although it is not necessary, here is a quick example of how to use **dotenv** in rust.

- Create a `.env` file in the root of your project
```env
// Inside .env
DISCORD_TOKEN = "MYDISCORD_TOKEN"
```
- Install the `dotenv` dependency on your `Cargo.toml` file
```toml
[dependencies]
# Replace `*` with the latest version of dot env
dotenv = "*"
```
- Inside your rust script, use `dotenv`
```rust
use dotenv;

fn main() {
  let token = dotenv::var("DISCORD_TOKEN").unwrap();
}
```

# Examples

## Webhooks
Before you can setup webhooks using discord-rs, you will need to create a webhook integration. Go to the settings for the channel you want to make a webhook for and in the integrations tab select "Create Webhook". From ther you can copy the webhook token and use it in your code.

```rust
use discord_rs::embed::Embed;
use discord_rs::webhook::{WebhookClient, MessagePayload};

#[tokio::main]
async fn main() {
  // Your webhook's credentials
  let id =  "YOUR_ID";
  let token =  "YOUR_TOKEN";

  let webhook =  WebhookClient::new(id, token);

  let embed =  Embed::new()
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
    .await
    .expect("Failed to send webhook");
}
```
 
### License

This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).