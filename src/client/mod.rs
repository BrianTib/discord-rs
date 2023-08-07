#[allow(dead_code, unused_imports)]
use reqwest::Client as ReqwestClient;
use serde_json::{Value, json};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use std::sync::{Mutex, mpsc::Sender};
use futures_util::stream::StreamExt;

use std::{
    sync::Arc,
    time::{Duration, Instant}
};

use crate::util::ws::WebsocketConnection;
use crate::structs::{
    guild::Guild,
    application_command::ApplicationCommand
};

pub mod cache;
pub use cache::types::ClientCache;

pub mod types;
pub use types::*;

pub mod enums;
pub use enums::*;

const API_VERSION: u8 = 10;

impl Client {
    /// Creates a new Discord Bot Client
    /// 
    /// # Arguments
    /// * `token` - A string slice for the bot's token provided by https://discord.com/developers/applications/{YourApplicationId}/bot
    /// * `intents` - An array of [GatewayIntentBits]. This represents a bitfield
    /// which determines what events your bot will receive. [GatewayIntentBits] directly
    /// maps to https://discord.com/developers/docs/topics/gateway#gateway-intents
    pub fn new(token: &str, intents: &[GatewayIntentBits]) -> Self {
        let bits = intents
            .iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            });

        // Make some globally available variables
        std::env::set_var("_CLIENT_TOKEN", token);
        std::env::set_var("_DISCORD_API_URL", format!("https://discord.com/api/v{API_VERSION}"));

        let websocket = WebsocketConnection::new("wss://gateway.discord.gg/?v=10&encoding=json").unwrap();

        Self {
            intents: bits,
            token: token.to_string(),
            ws: websocket,
            cache: Arc::new(Mutex::new(ClientCache::new())),
            events: None,
            //event_callbacks: HashMap::new()
        }
    }

    /// Connects the client to the Discord Gateway webhook
    pub async fn login(&mut self) -> Result<(), &'static str> {
        let (sender, receiver) = socket.split();
        let (etx, erx) = mpsc::channel::<(GatewayDispatchEventType, Value)>(100);

        // // Send the identify payload
        // let identify = _get_identify(&self.token, &self.intents);
        // let _ = sender.send(identify).await;
        
        let token = self.token.clone();
        let socket_mutex = Arc::new(Mutex::new(WebsocketConnection { sender, receiver }));
        self.events = Some(erx);

        tokio::task::spawn(
            _event_listener(
                socket_mutex,
                Arc::clone(&self.cache),
                Arc::new(Mutex::new(etx)),
                token
            )
        );

        Ok(())
    }

    pub async fn register_guild_commands(&self, guilds: &str, commands: &[ApplicationCommand]) -> Result<(), &'static str> {
        let cache = self.cache.lock().await;
        //let application_id = &cache.application.unwrap().id;
        
        let commands = serde_json::to_string(commands).unwrap();

        println!("{:#?}, {:#?}", commands, cache);
        // guilds.iter().for_each(|guild_id| {
        //     let res = post(
        //         format!("/applications/{}/guilds/{}/commands", application_id, guild_id),
        //     )
        // });
        
        Ok(())
    }
}

async fn _event_listener(
    socket: Arc<Mutex<WebsocketConnection>>,
    cache: Arc<Mutex<ClientCache>>,
    event_channel: Arc<Mutex<Sender<(GatewayDispatchEventType, Value)>>>,
    token: String
) {
    let event_channel = event_channel.lock().await;
    
    let mut socket = socket.lock().await;
    let mut next_heartbeat: Option<Instant> = None;
    let mut interval: u64 = 0;
    let mut last_sequence: u64 = 0;

    // Create a timer that checks if we should reply to a heartbeat every second
    let mut heartbeat_timer = tokio::time::interval(Duration::from_millis(1000));

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
                                
                                _patch_cache(&cache, &dispatch_type, &dispatch_data).await;
                                let _ = event_channel.send((dispatch_type, dispatch_data)).await;
                            },
                            GatewayEventType::Heartbeat => todo!(),
                            GatewayEventType::Identify => todo!(),
                            GatewayEventType::PresenceUpdate => todo!(),
                            GatewayEventType::VoiceStateUpdate => todo!(),
                            GatewayEventType::Resume => {
                                println!("Got resume event: {:#?}", event);
                            },
                            // Connection was likely dropped on discord's end. Mend it
                            GatewayEventType::Reconnect => _reconnect_socket(&mut socket, &cache, &token, &last_sequence).await,
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
                    // Disconnecting this way from the socket is permanent
                    Message::Close(_) => {
                        println!("Disconnected from the socket");
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

/// Updates Cache managers when certain events are received
async fn _patch_cache(
    cache: &Arc<Mutex<ClientCache>>,
    dispatch_type: &GatewayDispatchEventType,
    data: &Value
) {
    match dispatch_type {
        GatewayDispatchEventType::Ready => {
            let cache_data: ClientCache = serde_json::from_value(data.clone()).unwrap();
            *cache.lock().await = cache_data;
        },
        GatewayDispatchEventType::GuildCreate => {
            //let mut cache = cache.lock().await;
            let guild: Guild = serde_json::from_value(data.clone()).expect("Error serializing guild");
            //cache.guilds.set(guild.to_owned());

            println!("Guild: {:#?}", guild);
        }
        _ => {}
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

// Mend the connection to the socket using the given resume_gateway_url
async fn _reconnect_socket(
    socket: &mut WebsocketConnection,
    cache: &Arc<Mutex<ClientCache>>,
    token: &String,
    sequence: &u64
) {
    println!("Disconnected from socket... reconnecting");

    let client_cache = cache.lock().await;
    let url = client_cache.resume_gateway_url.to_owned().unwrap();

    let (new_socket, _) = connect_async(url)
        .await
        .expect("Failed to connect to reconnect to socket");

    let (sender, receiver) = new_socket.split();
    
    let resume = _get_resume(token, &client_cache.session_id.to_owned().unwrap(), sequence);
    *socket = WebsocketConnection { sender, receiver};
    let _ = socket.sender.send(resume).await;
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
    Message::text(identify)
}

fn _get_resume(token: &String, session_id: &String, sequence: &u64) -> Message {
    let resume = GatewayEvent {
        op: GatewayEventType::Resume as usize,
        s: None,
        t: None,
        d: Some(json!({
            "token": token,
            "session_id": session_id,
            "seq": sequence
        }))
    };

    let resume = serde_json::to_string(&resume).unwrap();
    Message::Text(resume)
}