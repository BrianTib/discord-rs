# discord-rs

discord-rs is a blazingly fast library for interacting with the Discord API. It provides a simple and efficient way to build Discord bots, create rich embeds, send messages, manage channels, and more.

## Features

- **High Performance:** Built with speed and efficiency in mind, allowing you to handle high loads without sacrificing performance.
- **Simple API:** Provides an easy-to-use interface for interacting with the Discord API, making it beginner-friendly and developer-friendly.
- **Rich Embeds:** Create visually appealing embeds with custom titles, descriptions, fields, and images to enhance your messages.
- **Event System:** Handle events such as message received, channel created, member joined, and more to create dynamic and interactive bots.
- **Asynchronous:** Designed with asynchronous programming in mind, enabling concurrent operations and non-blocking I/O.
- **Well-documented:** Comprehensive documentation with examples and guides to help you get started quickly.

## Getting Started

### Installation

You can add discord-rs as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
discord-rs = "0.1.0"
```

### Usage

Here's a simple example of how to use discord-rs to send a message:

```rust
use discord-rs::{Client, Context, EventHandler};

struct MyHandler;

impl EventHandler for MyHandler {
    fn on_message(&self, ctx: &Context, message: &Message) {
        if message.content == "!hello" {
            ctx.send_message(message.channel_id, "Hello, Discord!").unwrap();
        }
    }
}

fn main() {
    let token = "YOUR_DISCORD_TOKEN";
    let client = Client::new(token, MyHandler);
    client.start().expect("Failed to start the client.");
}
```

Make sure to replace YOUR_DISCORD_TOKEN with your actual Discord bot token.

### Documentation

For detailed usage instructions, examples, and API reference, please refer to the [Documentation](https://docs.rs/discord-rs)

### Contributing

Contributions are welcome! If you find any bugs, have suggestions, or would like to contribute to the project, please check out our [Contributing Guidelines].

### License

This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).