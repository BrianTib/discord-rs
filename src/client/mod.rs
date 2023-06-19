#[allow(dead_code, unused_imports)]
use rand::Rng;
use reqwest::{Client as ReqwestClient, header as ReqwestHeader};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde_json::{Value, json};
use websocket::client::builder;
use std::sync::mpsc;
use std::time::Duration;
use tokio::time::sleep;
use tokio::runtime::Builder;
use std::thread;
use std::io::{Write, Read};
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;

pub mod types;
pub use types::{
    Client,
    GatewayEvent,
    GatewayIntentBits,
    GatewayOpCode,
    GatewayOpCodeIndexer,
    WebsocketConnection
};

use crate::client;

impl Client {
    /// Creates a new Discord Bot Client
    /// 
    /// # Arguments
    /// * `token` - A string slice for the bot's token provided by https://discord.com/developers/applications/{YourApplicationId}/bot
    /// * `intents` - An array of [GatewayIntentBits]. This represents a bitfield
    /// which determines what events your bot will receive. [GatewayIntentBits] directly
    /// maps to https://discord.com/developers/docs/topics/gateway#gateway-intents
    /// 
    /// # Example
    /// ```
    /// use discord_rs::client::{Client, GatewayIntentBits};
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new(&[
    ///         GatewayIntentBits::Guilds,
    ///         GatewayIntentBits::GuildMessages,
    ///         GatewayIntentBits::DirectMessages,
    ///     ]);
    /// 
    ///     let token = "YOUR_TOKEN";
    ///     client.login(token)
    ///         .await
    ///         .expect("Failed to login");
    /// }
    /// ```
    pub fn new(token: &str, intents: &[GatewayIntentBits]) -> Self {
        let bits = intents
            .iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            });
        
        Self {
            intents: (bits, intents.to_vec()),
            token: token.to_string(),
            cache: None,
            ws: WebsocketConnection {
                connection: None,
                client: ReqwestClient::new()
            },
        }
    }

    /// This function should only be called once per process
    /// 
    /// Sends a [GatewayOpCode::Identify] [GatewayEvent] to Discord
    /// which includes the bot's token. This initiates the websocket
    /// connection from discord to the user and kickstarts all websocket
    /// events essentially making your bot 'online'
    /// 
    /// # Panics
    /// * If a connection to wss://gateway.discord.gg cannot be established
    /// * If an initial identify message cannot be sent through the websocket
    /// * If incoming data through the socket cannot be deserialized
    /// 
    /// # Errors
    /// * Can error if contained websocket handler events fail ()
    pub fn login(&self) -> Result<(), &'static str> {
        // Establish a connection to the Discord event socket
        let (mut socket, _) = connect("wss://gateway.discord.gg/?v=10&encoding=json")
            .expect("Failed to connect to gateway");

        // Send the initial handshake
        let identify = GatewayEvent {
            op: GatewayOpCode::Identify as usize,
            d: Some(json!({
                "token": self.token,
                "intents": self.intents.0,
                "properties": {
                    "os": "win",
                    "browser": "discord-rs",
                    "device": "discord_rs"
                }
            })),
            s: None,
            t: None,
        };

        let identify = serde_json::to_string(&identify).unwrap();

        socket.write_message(Message::text(identify))
            .expect("Failed to identify");

        loop {
            match socket.read_message().unwrap() {
                Message::Text(text_message) => {
                    let event = serde_json::from_str::<GatewayEvent>(&text_message)
                        .expect("Failed to deserialize incoming data JSON");

                    let operation_code = GatewayOpCodeIndexer[event.op];

                    let res = match operation_code {
                        GatewayOpCode::Dispatch => todo!(),
                        GatewayOpCode::Heartbeat => self.on_heartbeat(event),
                        GatewayOpCode::Identify => todo!(),
                        GatewayOpCode::PresenceUpdate => todo!(),
                        GatewayOpCode::VoiceStateUpdate => todo!(),
                        GatewayOpCode::Resume => todo!(),
                        GatewayOpCode::Reconnect => todo!(),
                        GatewayOpCode::RequestGuildMembers => todo!(),
                        GatewayOpCode::InvalidSession => todo!(),
                        GatewayOpCode::Hello => self.on_hello(event),
                        GatewayOpCode::HeartbeatAcknowledge => self.on_heartbeat_ack(event)
                    };

                    if let Some(response) = res.unwrap() {
                        socket.write_message(response)
                            .expect(&format!("Failed to reply to event {:?}", operation_code));
                    }
                },

                Message::Binary(binary_message) => {
                    println!("Received binary message: {:?}", binary_message);
                },

                Message::Ping(ping_message) => {
                    println!("Received ping message: {:?}", ping_message);
                },

                Message::Pong(pong_message) => {
                    println!("Received pong message: {:?}", pong_message);
                },

                Message::Close(close_message) => {
                    println!("Received close message: {:?}", close_message);
                },

                Message::Frame(frame) => {
                    println!("Received frame message: {:?}", frame);
                },
            }
        }
    }

    fn on_hello(&self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
        println!("Recieved hello event!");
        if !event.d.is_some() {
            return Err("Error")
        }

        Ok(None)
    }

    fn on_heartbeat(&self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
        println!("Recieved heartbeat!: Event: {:#?}", event);

        if !event.d.is_some() {
            return Err("Error")
        }

        let data = event.d.unwrap();
        // Base duration of 5 seconds
        let base_duration = Duration::from_millis(data["heartbeat_interval"].as_u64().unwrap());

        // Generate a random jitter between 0 and 1000 milliseconds
        let mut rng = rand::thread_rng();
        let jitter_ms = rng.gen_range(0..=1000);
        let jitter_duration = Duration::from_millis(jitter_ms);

        // Calculate the total duration with jitter
        let total_duration = base_duration + jitter_duration;
        thread::sleep(total_duration);

        let response = GatewayEvent {
            op: 1,
            d: None,
            s: None,
            t: None
        };

        let message = Message::text(serde_json::to_string(&response).expect("Failed to stringify response gateway event"));
        println!("Response message: {:#?}", message);
        Ok(Some(message))
    }

    fn on_heartbeat_ack(&self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
        println!("Recieved heartbeat acknowledgement! {:#?}", event);
        Ok(None)
    }
}