#[allow(dead_code, unused_imports)]
use futures_util::{stream::{StreamExt, SplitSink}, sink::SinkExt};
use reqwest::{Client as ReqwestClient};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub mod types;
pub use types::{
    Client,
    Connection,
    GatewayEvent,
    EventHandler,
    GatewayIntentBits,
    GatewayEventType,
    GatewayEventTypeIndexer,
    EventPipelineConnection,
    GatewayDispatchEventType,
    GatewayDispatchEventTypeIndexer,
    WebsocketConnection
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
    pub fn new(token: &str, intents: &[GatewayIntentBits], handler: Box<dyn EventHandler>) -> Self {
        let bits = intents
            .iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            });
        
        Self {
            intents: bits,
            token: token.to_string(),
            cache: HashMap::new(),
            handler,
            _connection: None
        }
    }

    pub async fn connect(&mut self) {
        // Establish a connection to the gateway
        let (socket, _) = connect_async("wss://gateway.discord.gg/?v=10&encoding=json")
            .await
            .expect("Failed to connect to gateway");

        let (sender, receiver) = socket.split();
        let (tx, rx) = mpsc::channel::<(GatewayEventType, GatewayEvent)>(100);

        let connection = Arc::new(Mutex::new(Connection {
            event_pipeline: EventPipelineConnection {
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

        let catcher_task = tokio::spawn(_catch_events(Arc::clone(&connection)));
        let heartbeats_task = tokio::spawn(_start_beating(Arc::clone(&connection), token_clone, intents_clone));

        tokio::try_join!(
            catcher_task,
            heartbeats_task
        ).unwrap();

        // Begin catching and handling the events being received from the 
        // socket at a struct level
        self._start_pipeline().await;
    }

    // Catches events from the socket and dispels the appropriate ones
    // to the handler
    async fn _start_pipeline(&mut self) {
        let mut connection = self._connection.as_mut().unwrap().lock().await;

        // Receive events from the channel
        while let Some((event_type, _event)) = connection.event_pipeline.receiver.recv().await {
            match event_type {
                GatewayEventType::Dispatch => {
                    println!("Got pipeline dispatch event");
                },
                _ => {}
            }
        }
    }
}

/// Catches all incoming events directly from the gateway socket
/// and sends them over to either the heatbeat loop or the handler to
/// be consumed by the client
async fn _catch_events(connection: Arc<Mutex<Connection>>) {
    tokio::spawn(async move {
        let mut connection = connection.lock().await;

        while let Some(Ok(message)) = connection.socket.receiver.next().await {
            match message {
                Message::Text(text_message) => {
                    let event = serde_json::from_str::<GatewayEvent>(&text_message)
                        .expect("Failed to deserialize incoming data JSON");
    
                    let event_type = GatewayEventTypeIndexer[event.op];
                    let _ = connection.event_pipeline.sender.send((event_type, event.to_owned()));
                    println!("Caught event! {:?}", event_type);
                },
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
    });
}

async fn _start_beating(connection: Arc<Mutex<Connection>>, token: String, intents: u64) {
    let thread_connection = Arc::clone(&connection);

    let mut connection = connection.lock().await;
    let (mut sequence, mut interval): (u64, u64) = (0, 0);

    // Cannot do something like while interval == 0 && let Some()...
    // https://github.com/rust-lang/rust/issues/53667
    // Use loop instead

    println!("Going inside beating loop");

    loop {
        if interval != 0 { break; }
        println!("Going inside beating loop");

        // Catch the hello event which will tell us at what
        // pace we need to heartbeat to
        while let Some((event_type, event)) = connection.event_pipeline.receiver.recv().await {
            println!("Got event to iniate heartbeat loop: {:?}", event_type);
            match event_type {
                GatewayEventType::Hello => {
                    let data = event.d.unwrap();
                    interval = data["heartbeat_interval"].as_u64()
                        .expect("Failed to extract interval from hearbeat init");
                },
                _ => {}
            }

            // Send the initial identify payload
            let identify = _get_identify(token.to_owned(), intents);
            let _ = connection.socket.sender.send(identify).await;
        }
    }

    // We need to wait for a small jitter before starting to send heartbeats
    tokio::time::sleep(Duration::from_millis(800)).await;

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

fn _get_identify(token: String, intents: u64) -> Message {
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

async fn _get_heartbeat(sequence: u64) -> Message {
    let heartbeat = GatewayEvent {
        op: GatewayEventType::Heartbeat as usize,
        d: Some(if sequence == 0 { Value::Null } else { Value::Number(sequence.into()) }),
        s: None,
        t: None,
    };

    // Serialize the heartbeat request into JSON
    let heartbeat = serde_json::to_string(&heartbeat).unwrap();
    return Message::text(heartbeat);
}