#[allow(dead_code, unused_imports)]
use futures_util::{stream::{StreamExt, SplitSink}, sink::SinkExt};
use rand::Rng;
use reqwest::{Client as ReqwestClient};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::sync::mpsc;

//use crate::util::log_message;

pub mod types;
pub use types::{
    Client,
    Connection,
    GatewayEvent,
    GatewayIntentBits,
    GatewayOpCode,
    GatewayOpCodeIndexer,
    WebsocketConnection,
    ReceiveEvent,
    ReceiveEventIndexer,
    KeepAliveConnection
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
            intents: bits,
            token: token.to_string(),
            cache: HashMap::new(),
            _connection: None
        }
    }

    pub async fn connect(&mut self) {
        // Establish a connection to the gateway
        let (socket, _) = connect_async("wss://gateway.discord.gg/?v=10&encoding=json")
            .await
            .expect("Failed to connect to gateway");

        let (sender, receiver) = socket.split();
        let (tx, rx) = mpsc::channel(10);

        let connection = Arc::new(Mutex::new(Connection {
            keepalive: KeepAliveConnection {
                sender: tx,
                receiver: rx,
            },
            socket: WebsocketConnection {
                sender,
                receiver
            },
            http_client: ReqwestClient::new()
        }));

        
        let token_clone = self.token.clone();
        let intents_clone = self.intents.clone();

        self._connection = Some(Arc::clone(&connection));
        let heartbeat_connection = Arc::clone(&connection);
        let events_connection = Arc::clone(&connection);

        let heartbeats_task = tokio::spawn(_start_beating(heartbeat_connection, token_clone, intents_clone));
        let events_task = tokio::spawn(_receive_events(events_connection));

        tokio::try_join!(heartbeats_task, events_task).unwrap();
    }
}

async fn _start_beating(connection: Arc<Mutex<Connection>>, token: String, intents: u64) {
    let thread_connection = Arc::clone(&connection);
    let mut connection = connection.lock().await;
    let (mut sequence, mut interval): (u64, u64) = (0, 0);

    // Send the initial identify payload
    let identify = _get_identify(token, intents);
    let _ = connection.socket.sender.send(identify).await;

    // Catch the initial hello event to kickstart the heartbeating process on the background
    if let Some(Ok(maybe_hello)) = connection.socket.receiver.next().await {
        match maybe_hello {
            Message::Text(text_message) => {
                let event = serde_json::from_str::<GatewayEvent>(&text_message)
                    .expect("Failed to deserialize incoming data JSON at handshake");

                // Ensure this is the right operation code
                let operation_code = GatewayOpCodeIndexer[event.op];
                if operation_code != GatewayOpCode::Hello || event.d.is_none() {
                    panic!("Failed to initiate beating");
                }

                let data = event.d.unwrap();
                interval = data["heartbeat_interval"].as_u64().unwrap();
            },
            _ => {}
        }  
    } else {
        panic!("Failed to handshake with gateway");
    }

    println!("Waiting for jitter...");
    // We need to wait for a small jitter before starting to send heartbeats
    tokio::time::sleep(Duration::from_millis(800)).await;
    println!("Spawning hearbeat thread...");

    tokio::spawn(async move {
        loop {
            let mut connection = thread_connection.lock().await;
    
            println!("Inside heartbeat loop");
            let heartbeat = _get_heartbeat(sequence).await;
            sequence += 1;
    
            let _= connection.socket.sender.send(heartbeat).await;
            
            println!("Sent heartbeat! Waiting until next loop");
            let _ = tokio::time::sleep(Duration::from_millis(interval)).await;
        }
    });
    
}

async fn _receive_events(connection: Arc<Mutex<Connection>>) {
    let mut connection = connection.lock().await;
    println!("Inside receive events");

    while let Some(Ok(message)) = connection.socket.receiver.next().await {
        match message {
            Message::Text(text_message) => {
                let event = serde_json::from_str::<GatewayEvent>(&text_message)
                    .expect("Failed to deserialize incoming data JSON");

                println!("Got new event: {:#?}", event);

                let operation_code = GatewayOpCodeIndexer[event.op];
                let res = match operation_code {
                    GatewayOpCode::Dispatch => {
                        _dispatch(event.to_owned()).await;
                        Ok(None)
                    },
                    GatewayOpCode::Heartbeat => Ok(None),
                    GatewayOpCode::Identify => todo!(),
                    GatewayOpCode::PresenceUpdate => todo!(),
                    GatewayOpCode::VoiceStateUpdate => todo!(),
                    GatewayOpCode::Resume => todo!(),
                    GatewayOpCode::Reconnect => todo!(),
                    GatewayOpCode::RequestGuildMembers => todo!(),
                    GatewayOpCode::InvalidSession => Err("Invalid session. Make sure your token is correct"),
                    GatewayOpCode::Hello => Ok(None),
                    GatewayOpCode::HeartbeatAcknowledge => Ok(None),
                };

                // If any of the arms returned a message, send it through the socket
                if let Some(response) = res.unwrap() {
                    let _ = connection.socket.sender.send(response).await;
                }
            }
            Message::Close(_) => {
                // Handle a close message and exit the loop
                println!("Received close message. Exiting...");
                break;
            }
            _ => {
                // Handle other types of messages if needed
                // For example: Message::Binary, Message::Pong, Message::Continuation
            }
        }
    }
}

fn _get_identify(token: String, intents: u64) -> Message {
    // Structure the initial identify request
    let identify = GatewayEvent {
        op: GatewayOpCode::Identify as usize,
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

async fn _get_heartbeat(sequence: u64) -> Message {
    let heartbeat = GatewayEvent {
        op: GatewayOpCode::Heartbeat as usize,
        d: Some(if sequence == 0 { Value::Null } else { Value::Number(sequence.into()) }),
        s: None,
        t: None,
    };

    // Serialize the heartbeat request into JSON
    let heartbeat = serde_json::to_string(&heartbeat).unwrap();
    return Message::text(heartbeat);
}

async fn _dispatch(event: GatewayEvent) {
    println!("Got dispatch event: {:#?}", event);
    //let event_type = event.t.unwrap();
    //let event_code = ReceiveEventIndexer[&event_type];
}