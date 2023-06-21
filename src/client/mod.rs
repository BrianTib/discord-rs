#[allow(dead_code, unused_imports)]
use futures_util::sink::SinkExt;
use futures_util::stream::{StreamExt, SplitSink};
use rand::Rng;
use reqwest::{Client as ReqwestClient};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, mpsc};
use serde_json::Value;
//use crate::util::log_message;

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
                keepalive: None,
                receiver: None,
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
    /// * Can error if contained websocket handler events fail
    pub async fn login(&mut self) -> Result<(), &'static str> {
        // Establish a connection to the Discord event socket
        let (socket, _) = connect_async("wss://gateway.discord.gg/?v=10&encoding=json")
            .await
            .expect("Failed to connect to gateway");

        let (mut writer, mut reader) = socket.split();

        // Structure the initial identify request
        let identify = GatewayEvent {
            op: GatewayOpCode::Identify as usize,
            d: Some(json!({
                "token": self.token.to_owned(),
                "intents": self.intents.0.to_owned(),
                "properties": {
                    "os": std::env::consts::OS,
                    "browser": "discord-rs",
                    "device": "discord_rs"
                }
            })),
            s: None,
            t: None,
        };

        // Serialize the identify request into JSON
        let identify = serde_json::to_string(&identify).unwrap();

        // Send the initial identify payload
        writer.send(Message::text(identify))
            .await
            .expect("Failed to identify with gateway");
        
        // Create an Arc-wrapped Mutex to share the writer across threads
        let writer_mutex = Arc::new(Mutex::new(writer));
        // Create a clone of the writer_mutex for the heartbeat loop
        let heartbeat_writer = Arc::clone(&writer_mutex);
        // Create the keep alive channel
        let (_tx, mut rx) = mpsc::channel(10);
        
        // Assign the keep alive channel
        //self.ws.keepalive = Some(tx);
        //self.ws.receiver = Some(rx);

        // Catch the initial hello event to kickstart the heartbeating process on the background
        if let Some(Ok(maybe_hello)) = reader.next().await {
            match maybe_hello {
                Message::Text(text_message) => {
                    let event = serde_json::from_str::<GatewayEvent>(&text_message)
                        .expect("Failed to deserialize incoming data JSON at handshake");

                    // Ensure this is the right operation code
                    let operation_code = GatewayOpCodeIndexer[event.op];
                    if operation_code != GatewayOpCode::Hello {
                        return Err("Received first operation that was not Hello");
                    }

                    // Check that there is data within the d object of the gateway event
                    // Ideally never happens unless there is a change on the gateway api
                    if event.d.is_none() { return Err("Received JSON at hanshake") }
                    let data = event.d.unwrap();
                    let heartbeat_interval = data["heartbeat_interval"].as_u64().unwrap();

                    // We need to apply a jitter before our first heartbeat
                    let mut rng = rand::thread_rng();
                    let jitter_duration = Duration::from_millis(rng.gen_range(0..=1000));

                    // Wait for the jitter
                    let _ = std::thread::sleep(jitter_duration);

                    tokio::spawn(async move {
                        on_heartbeat(heartbeat_interval, heartbeat_writer, &mut rx).await;
                    });
                },
                _ => return Err("Got unknown event when attempting to handshake")
            }  
        } else {
            return Err("Failed to handshake with gateway");
        }

        tokio::spawn(async move {
            while let Some(Ok(packet )) = reader.next().await {
                let mut writer = writer_mutex.lock().await;

                match packet {
                    Message::Text(text_message) => {
                        let event = serde_json::from_str::<GatewayEvent>(&text_message)
                            .expect("Failed to deserialize incoming data JSON");
    
                        let operation_code = GatewayOpCodeIndexer[event.op];
                        let res = match operation_code {
                            GatewayOpCode::Dispatch => on_dispatch(event),
                            GatewayOpCode::Heartbeat => Ok(None),
                            GatewayOpCode::Identify => todo!(),
                            GatewayOpCode::PresenceUpdate => todo!(),
                            GatewayOpCode::VoiceStateUpdate => todo!(),
                            GatewayOpCode::Resume => todo!(),
                            GatewayOpCode::Reconnect => todo!(),
                            GatewayOpCode::RequestGuildMembers => todo!(),
                            GatewayOpCode::InvalidSession => Err("Invalid session. Make sure your token is correct"),
                            GatewayOpCode::Hello => todo!(),
                            GatewayOpCode::HeartbeatAcknowledge => {
                                println!("Got heartbeat acknowledgement!");
                                Ok(None)
                            },
                        };

                        // If any of the arms returned a message, send it through the socket
                        if let Some(response) = res.unwrap() {
                            let _ = writer.send(response).await;
                        }
                    },
                    Message::Binary(_) => todo!(),
                    Message::Close(close_message) => {
                        println!("Received close message: {:?}", close_message);
                    },
                    Message::Frame(_) => todo!(),
                    Message::Ping(_) => todo!(),
                    Message::Pong(_) => todo!(),
                }
    
                // match packet {
                //     Message::Text(text_message) => {
                //         
                //         println!("Operation code: {:?}", operation_code);
    
                //         let _ = tx.send(event.to_owned());
    
                //         let res = match operation_code {
                //             GatewayOpCode::Dispatch => self.on_dispatch(event),
                //             GatewayOpCode::Heartbeat => {
                //                 println!("Called heartbeat");
                //                 //let _ = tx.send(event.to_owned());
                //                 Ok(None)
                //             },
                //             GatewayOpCode::Identify => {
                //                 println!("Called identify");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::PresenceUpdate => {
                //                 println!("Called presence update");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::VoiceStateUpdate => {
                //                 println!("Called state update");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::Resume => {
                //                 println!("Called resume");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::Reconnect => {
                //                 println!("Called reconnect");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::RequestGuildMembers => {
                //                 println!("Called request guild members");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::InvalidSession => {
                //                 return Err("Invalid session. Make sure your token is correct")
                //             },
                //             GatewayOpCode::Hello => {
                //                 println!("Called hello");
                //                 Ok(None)
                //             },
                //             GatewayOpCode::HeartbeatAcknowledge => {
                //                 //let _ = tx.send(event.to_owned());
                //                 println!("Got heartbeat ack. Sending through channel...");
                //                 Ok(None)
                //             }
                //         };
            
                //         if let Some(response) = res.unwrap() {
                //             let _ = writer.send(response).await;
                //         }
                //     },
                //     Message::Close(close_message) => {
                //         println!("Received close message: {:?}", close_message);
                //     },
                //     Message::Binary(binary_message) => {
                //         println!("Received binary message: {:?}", binary_message);
                //     },
                //     Message::Ping(ping_message) => {
                //         println!("Received ping message: {:?}", ping_message);
                //     },
                //     Message::Pong(pong_message) => {
                //         println!("Received pong message: {:?}", pong_message);
                //     },
                //     Message::Frame(frame) => {
                //         println!("Received frame message: {:?}", frame);
                //     }
                // }
            }
        });
    
        Ok(())
    }
}

fn on_heartbeat_ack(event: GatewayEvent) -> Result<Option<Message>, &'static str> {
    println!("Recieved heartbeat acknowledgement! {:#?}", event);
    Ok(None)
}

/// Receives regular events from the socket
fn on_dispatch(event: GatewayEvent) -> Result<Option<Message>, &'static str> {
    if event.t.is_none() || event.d.is_none() {
        return Err("Received unidentified event type/data");
    }

    let event_data = event.d.unwrap();
    let event_type = event.t.unwrap();
    let event_code = ReceiveEventIndexer[&event_type];

    // match event_code {
    //     ReceiveEvent::Ready => {
    //         self.cache.insert("ready".to_string(), event_data);
    //     }
    //     _ => {
    //         println!("Receieved dispatch event. Event name: {}. Data: {:#?}", event_type, event_data);
    //     }
    // }
    
    Ok(None)
}

async fn on_heartbeat(
    interval: u64,
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    channel: &mut mpsc::Receiver<GatewayEvent>
) {
    let mut writer = writer.lock().await;
    let mut last_sequence: u32 = 0;

    println!("Initiating heartbeat loop...");

    loop {
        // Structure the heartbeat message
        let heartbeat = GatewayEvent {
            op: GatewayOpCode::Heartbeat as usize,
            d: Some(if last_sequence == 0 { Value::Null } else { Value::Number(last_sequence.into()) }),
            s: None,
            t: None,
        };

        // Serialize the heartbeat request into JSON
        let heartbeat = serde_json::to_string(&heartbeat).unwrap();
        writer.send(Message::text(heartbeat))
            .await
            .expect("Failed tosend heartbeat");

        println!("Sent heart beat. Current sequence: {}. Sleeping {}ms before next beat...", last_sequence, interval);

        // let event = channel.recv().await;

        // if event.is_none() {
        //     println!("Errored out of heartbeat loop");
        //     continue;
        // }
        // let event = event.unwrap();

        // let operation_code = GatewayOpCodeIndexer[event.op];
        // println!("Operation code: {:?}", operation_code);

        // match operation_code {
        //     GatewayOpCode::Heartbeat => {
        //         println!("Incoming request inside heartbeat {:#?}", event);
        //     },
        //     GatewayOpCode::Resume => todo!(),
        //     GatewayOpCode::Reconnect => todo!(),
        //     _ => println!("Got unexpected event inside heartbeat loop")
        // }

        tokio::time::sleep(Duration::from_millis(interval)).await;
        last_sequence += 1;
    }
}