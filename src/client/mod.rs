use futures_util::TryStreamExt;
#[allow(dead_code, unused_imports)]
use futures_util::{stream::{StreamExt, SplitSink}, sink::SinkExt};
use reqwest::Client as ReqwestClient;
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::sync::{mpsc::{self, Sender}, Mutex};

pub mod cache;
pub mod types;
pub use cache::types::ClientCache;
pub use types::*;

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
    ///     let token = "YOUR_TOKEN";
    ///     let intents = &[
    ///         GatewayIntentBits::Guilds,
    ///         GatewayIntentBits::GuildMessages,
    ///         GatewayIntentBits::DirectMessages
    ///     ];
    ///     let mut client = Client::new(token, intents);
    ///     client.connect(token)
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
            intents: bits,
            token: token.to_string(),
            cache: Arc::new(Mutex::new(ClientCache::new())),
            client: ReqwestClient::new(),
            events: None,
        }
    }

    /// Connects the client to the Discord Gateway webhook
    pub async fn connect(&mut self) -> Result<(), &'static str> {
        let (socket, _) = connect_async("wss://gateway.discord.gg/?v=10&encoding=json")
            .await
            .expect("Failed to connect to gateway");

        let (mut sender, receiver) = socket.split();
        let (etx, erx) = mpsc::channel::<(GatewayDispatchEventType, Value)>(100);

        // Send the identify payload
        let identify = _get_identify(&self.token, &self.intents);
        let _ = sender.send(identify).await;

        let socket_mutex = Arc::new(Mutex::new(WebsocketConnection { sender, receiver }));
        self.events = Some(erx);

        tokio::task::spawn(
            _event_listener(
                socket_mutex,
                Arc::clone(&self.cache),
                Arc::new(Mutex::new(etx))
            )
        );

        Ok(())
    }
}

async fn _event_listener(
    socket: Arc<Mutex<WebsocketConnection>>,
    cache: Arc<Mutex<ClientCache>>,
    event_channel: Arc<Mutex<Sender<(GatewayDispatchEventType, Value)>>>,
) {
    let event_channel = event_channel.lock().await;
    
    let mut socket = socket.lock().await;
    let mut next_heartbeat: Option<Instant> = None;
    let mut interval: u64 = 0;
    let mut last_sequence: u64 = 0;

    // Create a timer that checks if we should reply to a heartbeat every 500 milliseconds
    let mut heartbeat_timer = tokio::time::interval(Duration::from_millis(500));

    loop {
        tokio::select! {
            message = socket.receiver.try_next() => {
                // If there are no incoming messages to handle, continue to the next iteration
                let message = message.expect("Error while reading socket stream");
                if message.is_none() { continue; }

                // At this point, there is a message we can handle.
                let message = message.unwrap();
                last_sequence += 1;

                match message {
                    Message::Text(text_message) => {
                        let event = serde_json::from_str::<GatewayEvent>(&text_message)
                            .expect("Failed to deserialize incoming data JSON");

                        let event_type = GatewayEventTypeIndexer[event.op];

                        match event_type {
                            GatewayEventType::Dispatch => {
                                let dispatch_data = event.d.unwrap();
                                let dispatch_type = event.t
                                    .as_ref()
                                    .and_then(|t| Some(t.as_str()))
                                    .and_then(|dispatch_type| Some(GatewayDispatchEventTypeIndexer[dispatch_type]))
                                    .expect("Failed to deserialize event type for dispatch event");

                                if dispatch_type == GatewayDispatchEventType::Ready {
                                    let cache_data: ClientCache = serde_json::from_value(dispatch_data.clone()).unwrap();
                                    *cache.lock().await = cache_data;
                                }

                                let _ = event_channel.send((dispatch_type, dispatch_data)).await;
                            },
                            GatewayEventType::Heartbeat => todo!(),
                            GatewayEventType::Identify => todo!(),
                            GatewayEventType::PresenceUpdate => todo!(),
                            GatewayEventType::VoiceStateUpdate => todo!(),
                            GatewayEventType::Resume => todo!(),
                            GatewayEventType::Reconnect => todo!(),
                            GatewayEventType::RequestGuildMembers => todo!(),
                            GatewayEventType::InvalidSession => todo!(),
                            GatewayEventType::Hello => {
                                let data = event.d.unwrap();
                                interval = data["heartbeat_interval"].as_u64().unwrap();
                                next_heartbeat = None;

                                let heartbeat = _get_heartbeat(last_sequence); 
                                let _ = socket.sender.send(heartbeat).await;
                            },
                            GatewayEventType::HeartbeatAcknowledge => {
                                next_heartbeat = Some(Instant::now() + Duration::from_millis(interval));
                            },
                        };
                    },
                    Message::Close(_) => {
                        // Handle a close message and exit the loop
                        println!("Received close message. Exiting...");
                        break;
                    },
                    _ => {
                        // Handle other types of messages if needed
                        // For example: Message::Binary, Message::Pong, Message::Continuation
                        println!("Got unhandled message type");
                    }
                }
            },
            // Sends a heartbeat if needed
            _ = heartbeat_timer.tick() => {
                _check_heartbeat(&mut next_heartbeat, &mut socket, last_sequence).await;
            }
        }
    }
}

async fn _check_heartbeat(
    instant: &mut Option<Instant>,
    socket: &mut WebsocketConnection,
    sequence: u64
) {
    // We dont have a current heartbeat to check for
    if instant.is_none() { return; }
    let now = Instant::now();
    // Not enough time has passed
    if now < instant.unwrap() { return; }

    if let Err(_) = socket.sender.send(_get_heartbeat(sequence)).await {
        println!("Failed to send heartbeat reply");
        return;
    }

    *instant = None;
}

fn _get_heartbeat(sequence: u64) -> Message {
    let heartbeat = GatewayEvent {
        op: GatewayEventType::Heartbeat as usize,
        d: Some(Value::Number(sequence.into())),
        s: None,
        t: None,
    };

    let heartbeat = serde_json::to_string(&heartbeat).unwrap();
    Message::text(heartbeat)                            
}

fn _get_identify(token: &String, intents: &u64) -> Message {
    // Structure the initial identify request
    let identify = GatewayEvent {
        op: GatewayEventType::Identify as usize,
        s: None,
        t: None,
        d: Some(json!({
            "token": token,
            "intents": intents,
            "properties": {
                "os": std::env::consts::OS,
                "browser": "discord-rs",
                "device": "discord_rs"
            }
        }))
    };

    // Serialize the identify request into JSON
    let identify = serde_json::to_string(&identify).unwrap();
    return Message::text(identify);
}