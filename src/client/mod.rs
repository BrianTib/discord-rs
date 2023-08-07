#[allow(dead_code, unused_imports)]
use futures_util::{TryStreamExt, SinkExt};
use serde_json::{Value, json};
use std::{sync::{Mutex, Arc}, time::{Duration, Instant}};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::mpsc::{self, Sender};

use crate::{
    util::ws::WebsocketConnection,
    structs::{
        guild::Guild,
        application_command::ApplicationCommand
    }
};

//use crate::util::rest::post;

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

        Self {
            cache: Arc::new(Mutex::new(ClientCache::new())),
            events: None,
            intents: bits,
            token: token.to_string(),
            ws: None
        }
    }

    /// Connects the client to the Discord Gateway webhook
    pub async fn connect(mut self) -> Result<(), &'static str> {
        let websocket = WebsocketConnection::new("wss://gateway.discord.gg/?v=10&encoding=json")
            .expect("Could not connect to the websocket");

        let (mut etx, erx) = mpsc::channel::<(GatewayDispatchEventType, Value)>(100);
        self.events = Some(erx);
        self.ws = Some(websocket);

        let _ = _websocket_listener(
            Arc::new(Mutex::new(self)),
             &mut etx
        ).await;

        Ok(())
    }
}

async fn _websocket_listener(
    client: Arc<Mutex<Client>>,
    event_channel: &Sender<(GatewayDispatchEventType, Value)>
) {
    let client_clone = Arc::clone(&client);
    //let _heartbeat_loop = _check_heartbeat(Arc::clone(&client_clone));
    
    loop {
        let mut client = client_clone.lock().unwrap();
        let websocket = client.ws.as_mut().unwrap();

        // Check if we need to reply with a heartbeat
        match websocket.client.read() {
            Ok(message) => {
                match message {
                    Message::Text(text_message) => {
                        // Increase the sequence
                        websocket.last_sequence += 1;
    
                        let event = serde_json::from_str::<GatewayEvent>(&text_message)
                            .expect("Failed to deserialize incoming data JSON");
    
                        let event_type = GatewayEventTypeIndexer[event.op];
    
                        println!("Event type: {:?}", event_type);

                        match event_type {
                            GatewayEventType::Dispatch => handle_dispatch(Arc::clone(&client_clone), event, event_channel),
                            GatewayEventType::Heartbeat => todo!(),
                            GatewayEventType::Identify => {
                                println!("Got identify event"); 
                            },
                            GatewayEventType::PresenceUpdate => todo!(),
                            GatewayEventType::VoiceStateUpdate => todo!(),
                            GatewayEventType::Resume => {
                                println!("Got resume event: {:#?}", event);
                            },
                            // Connection was likely dropped on discord's end. Mend it
                            GatewayEventType::Reconnect => {
                                let _ = websocket.reconnect(None);
                                //socket.reconnect(None).await.unwrap();
                            },
                            GatewayEventType::RequestGuildMembers => todo!(),
                            GatewayEventType::InvalidSession => {
                                panic!("Invalid credentials! Please check that your token is correct");
                            },
                            GatewayEventType::Hello => {
                                // Send the initial identify payload
                                println!("Called handle_hello");
                                handle_hello(Arc::clone(&client_clone), event, websocket);
                                println!("Finished call to handle_hello");
                            },
                            GatewayEventType::HeartbeatAcknowledge => {
                                let interval = websocket.interval.unwrap();
                                websocket.next_heartbeat = Some(Instant::now() + Duration::from_millis(interval));
                                println!("Heartbeat acknowledged and reset");
                            },
                        };
                    },
                    Message::Close(_) => {
                        println!("Disconnected from the socket");
                        break;
                    },
                    _ => {
                        println!("Got unhandled message type");
                    }
                }
            },
            Err(err) => {
                eprintln!("Got error from socket: {:?}. Continuing", err);
                continue;
            }
        }
    }
}

fn handle_dispatch(
    client: Arc<Mutex<Client>>,
    event: GatewayEvent,
    event_channel: &Sender<(GatewayDispatchEventType, Value)>
) {
    let client = client.lock().unwrap();
    let dispatch_data = event.d.unwrap();
    let dispatch_type = event.t
        .as_ref()
        .and_then(|t| Some(t.as_str()))
        .and_then(|dispatch_type| Some(GatewayDispatchEventTypeIndexer[dispatch_type]))
        .expect("Failed to deserialize event type for dispatch event");
    
    _patch_cache(&client.cache, &dispatch_type, &dispatch_data);
    let _ = event_channel.send((dispatch_type, dispatch_data));
}

fn handle_hello(
    client: Arc<Mutex<Client>>,
    event: GatewayEvent,
    websocket: &mut WebsocketConnection
) {
    println!("Inside handle_hellow");
    let client = client.lock().unwrap();
    println!("Unwrapped client: {:?}", client);
    // let identify = _get_identify(&client.token, &client.intents);
    // println!("Inside handle hello. Identify: {:?}", &identify);
    // let _ = websocket.client.send(identify);
    // println!("Sent identify message!");

    // let data = event.d.unwrap();
    // let interval = data["heartbeat_interval"].as_u64().unwrap();
    // websocket.next_heartbeat = Some(Instant::now() + Duration::from_millis(interval));
    // websocket.interval = Some(interval);
    // println!("Heartbeaat is now: {:?}. Interval is now: {:?}", websocket.next_heartbeat, websocket.interval);
}

fn _check_heartbeat(client: Arc<Mutex<Client>>) -> std::thread::JoinHandle<()> {
    println!("Outside of spawned thread");

    std::thread::spawn(move || {
        println!("Inside spawned thread");

        let mut client = client.lock().unwrap();
        let websocket = client.ws.as_mut().unwrap();

        loop {
            // Add a delay or sleep between loop iterations
            std::thread::sleep(Duration::from_secs(1));

            // We dont have a current heartbeat to check for
            if websocket.next_heartbeat.is_none() {
                println!("Instant was none");
                continue;
            }

            let now = Instant::now();
            // Not enough time has passed
            if now < websocket.next_heartbeat.unwrap() {
                println!("Instant hasnt expired");
                continue;
            }

            let _ = websocket.client.send(_get_heartbeat(websocket.last_sequence));
            println!("Heartbeat sent");
            websocket.next_heartbeat = None;
        }
    })
}

/// Updates Cache managers when certain events are received
fn _patch_cache(
    cache: &Arc<Mutex<ClientCache>>,
    dispatch_type: &GatewayDispatchEventType,
    data: &Value
) {
    let mut cache = cache.lock().unwrap();

    match dispatch_type {
        GatewayDispatchEventType::Ready => {
            let cache_data = serde_json::from_value(data.clone()).unwrap();
            *cache = cache_data;
        },
        GatewayDispatchEventType::GuildCreate => {
            let guild: Guild = serde_json::from_value(data.clone()).expect("Error serializing guild");
            cache.guilds.set(guild.to_owned());
        }
        _ => {}
    }
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

// pub async fn register_guild_commands(&self, _guild: &str, commands: &[ApplicationCommand]) -> Result<(), &'static str> {
//     let cache = self.cache.lock().unwrap();
//     //let application_id = &cache.application.unwrap().id;
    
//     let commands = serde_json::to_string(commands).unwrap();

//     println!("{:#?}, {:#?}", commands, cache);
//     // guilds.iter().for_each(|guild_id| {
//     //     let res = post(
//     //         format!("/applications/{}/guilds/{}/commands", application_id, guild_id),
//     //     )
//     // });
    
//     Ok(())
// }

// Catches events sent by the gateway to by caught by the developer
// pub fn on_event(
//     mut self,
//     event_type: GatewayDispatchEventType,
//     callback: Box<dyn Fn(Client) -> dyn Future<Output = ()>>,
// ) -> Self {
//     self.event_callbacks
//         .insert(event_type, Box::new(callback(self)));

//     self
// }