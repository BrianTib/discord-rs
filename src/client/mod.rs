use futures_util::stream::SplitSink;
#[allow(dead_code, unused_imports)]
use rand::Rng;
use reqwest::Client as ReqwestClient;
use serde_json::{json};
use tokio_tungstenite::MaybeTlsStream;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::spawn;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt, FutureExt};
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod types;
pub use types::{
    Client,
    GatewayEvent,
    GatewayIntentBits,
    GatewayOpCode,
    GatewayOpCodeIndexer,
    WebsocketConnection,
    ReceiveEvent,
    ReceiveEventIndexer
};

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
            cache: HashMap::new(),
            ws: WebsocketConnection {
                client: ReqwestClient::new()
            },
        }
    }

    pub fn as_ref(&self) -> &Self { self }
    pub fn as_mut(&mut self) -> &mut Self { self }
    pub fn as_arc(self) -> Arc<Mutex<Self>> { Arc::new(Mutex::new(self)) }

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
    /// * Can error if contained websocket handler events fail
    pub async fn login(client_arc: Arc<Client>) -> Result<(), &'static str> {
        // Connect to the WebSocket endpoint
        let (socket, _) = connect_async("wss://gateway.discord.gg/?v=10&encoding=json")
            .await
            .expect("Failed to connect to WebSocket");

        let (mut writer, mut reader) = socket.split();

        // Create the initial payload
        let identify = GatewayEvent {
            op: GatewayOpCode::Identify as usize,
            s: None,
            t: None,
            d: Some(json!({
                "token": client_arc.token,
                "intents": client_arc.intents.0,
                "properties": {
                    "os": std::env::consts::OS,
                    "browser": "The discord",
                    "device": "discord_rs"
                }
            }))
        };

        let identify = serde_json::to_string(&identify).unwrap();
        let _ = writer.send(Message::text(identify));

        loop {
            let incoming = reader.next().await.unwrap();
            let client = client_arc.clone();

            let _ = tokio::spawn(async move {
                let lock = client.lock().await;

                match incoming.unwrap() {
                    Message::Text(text_message) => {
                        let event = serde_json::from_str::<GatewayEvent>(&text_message)
                            .expect("Failed to deserialize incoming data JSON");
            
                        let operation_code = GatewayOpCodeIndexer[event.op];
            
                        let res = match operation_code {
                            GatewayOpCode::Dispatch => client.on_dispatch(event),
                            GatewayOpCode::Heartbeat => client.on_heartbeat(event).await,
                            GatewayOpCode::Identify => todo!(),
                            GatewayOpCode::PresenceUpdate => todo!(),
                            GatewayOpCode::VoiceStateUpdate => todo!(),
                            GatewayOpCode::Resume => todo!(),
                            GatewayOpCode::Reconnect => todo!(),
                            GatewayOpCode::RequestGuildMembers => todo!(),
                            GatewayOpCode::InvalidSession => todo!(),
                            GatewayOpCode::Hello => client.on_heartbeat(event).await,
                            GatewayOpCode::HeartbeatAcknowledge => client.on_heartbeat_ack(event)
                        };
            
                        if let Some(response) = res.unwrap() {
                            println!("Sending response through socket: {:?}", response);
                            let _ = writer.send(response);
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
            });
        }
    }

    // fn on_hello(&self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
    //     Ok(Some(self.on_heartbeat(event)))
    // }

    async fn on_heartbeat(&self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
        println!("Recieved heartbeat!: Event data: {:#?}", event);

        if event.d.is_none() {
            return Err("Data is undefined")
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
        println!("Sleeping before sending heartbeat");
        sleep(total_duration).await;

        let response = GatewayEvent {
            op: 1,
            d: None,
            s: None,
            t: None
        };

        let message = Message::text(serde_json::to_string(&response).expect("Failed to stringify response gateway event"));
        Ok(Some(message))
    }

    // 
    fn on_heartbeat_ack(&self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
        println!("Recieved heartbeat acknowledgement! {:#?}", event);
        Ok(None)
    }

    /// Receives regular events from the socket
    fn on_dispatch(&mut self, event: GatewayEvent) -> Result<Option<Message>, &'static str> {
        if event.t.is_none() || event.d.is_none() {
            return Err("Received unidentified event type/data");
        }

        let event_data = event.d.unwrap();
        let event_type = event.t.unwrap();
        let event_code = ReceiveEventIndexer[&event_type];

        match event_code {
            ReceiveEvent::Ready => {
                self.cache.insert("ready".to_string(), event_data);
            }
            _ => {
                println!("Receieved dispatch event. Event name: {}. Data: {:#?}", event_type, event_data);
            }
        }
        
        Ok(None)
    }
}