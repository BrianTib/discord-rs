//use native_tls::TlsStream;
use serde_json::{json, Value};
use std::{
    sync::{Arc, Mutex, mpsc::{self, Receiver}},
    thread::{self, JoinHandle},
    time::{Instant, Duration}
};

use crate::util::socket::{Message, Socket};

mod enums;
mod types;

pub use enums::*;
pub use types::*;

// The gateway version of Discord's API to use
const API_VERSION: u8 = 10;

impl Client {
    pub fn new(token: &str, intents: &[GatewayIntents]) -> Self {
        // Make some globally available variables
        std::env::set_var("_CLIENT_TOKEN", token);
        std::env::set_var("_DISCORD_API_URL", format!("https://discord.com/api/v{API_VERSION}"));

        // Condense the intent permissions into bits
        let intents = intents
            .iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            });

        Self { intents }
    }

    /// Connects to Discord's gateway API and begins
    /// receiving and sending data
    pub fn connect(&mut self) -> Result<Receiver<(ExternalDispatchEvent, GatewayDispatchEventData)>, &'static str> {
        // Create a socket connection to Discord's Gateway API
        let socket = Socket::new(&format!("wss://gateway.discord.gg/?v={API_VERSION}&encoding=json"));
        // and turn the socket into an atomic reference that
        // can be shared accross threads
        let socket = Arc::new(Mutex::new(socket));

        let (tx, rx) = mpsc::channel();

        // Handle the incoming events as well as heartbeating
        // on a separate thread to ensure concurrency
        let _event_handler_thread = _handle_events(
            Arc::clone(&socket),
            tx,
            self.intents
        );

        // Ideally here we'd yield the receiver
        // but yielding is not yet stable in rust
        
        // yield Ok(rx);

        // Join the executuion of the event loop to the main
        // thread so that the main thread doesnt exit until
        // the event handler loop is done, which ideally
        // shouldnt happen as long as the bot is active
        
        // P.S. Will block the main thread.
        // Wait until yield stabilizes to implement
        
        // let _ = event_handler_thread.join();

        Ok(rx)
    }
}

/// Receives events from the Gateway API and forwards them to the main thread
fn _handle_events(
    socket: Arc<Mutex<Socket>>,
    dispatch_sender: mpsc::Sender<(ExternalDispatchEvent, GatewayDispatchEventData)>,
    intents: u64
) -> JoinHandle<()> {
    let intents = Arc::new(Mutex::new(intents));

    thread::spawn(move || {
        let mut socket = socket.lock().unwrap();
        let mut last_sequence = 0_usize;
        let mut interval = Duration::from_secs(999_999);
        let mut next_heartbeat = Instant::now();

        let intents = intents.lock().unwrap();

        loop {
            // Attempt to get the next event from the socket
            let event = socket.read();

            // Most common error is a no message
            // Use that as an opportunity to check whether or not we need to
            // send a hearbeat
            if let Err(_) = event {
                let now = Instant::now();
                // This means not enough time has passed for us to send a heartbeat
                if next_heartbeat > now { continue; }

                // Get the heartbeat payload and send it through the socket
                let heartbeat = _get_heartbeat(last_sequence);
                let _ = socket.send(heartbeat);

                // Mark the next time a heartbeat should be sent
                next_heartbeat = Instant::now() + interval;
                continue;
            }

            match event.unwrap() {
                Message::Text(message) => {
                    last_sequence += 1;

                    let event = serde_json::from_str::<GatewayEventBody>(&message)
                        .expect("Failed to deserialize incoming data JSON");

                    let event_type = GatewayEventIndexer[event.op];

                    match event_type {
                        GatewayEvent::Dispatch => {
                            let dispatch_data = event.d.unwrap();
                            let dispatch_type = event.t
                                .as_ref()
                                .and_then(|t| Some(t.as_str()))
                                .and_then(|dispatch_type| Some(DispatchEventIndexer[dispatch_type]))
                                .expect("Failed to deserialize event type for dispatch event");

                            // TODO: Patch the cache before forwarding the event to the end-user
                            //_patch_cache(&client.cache, &dispatch_type, &dispatch_data);

                            //println!("DispatchEvent({:?}): {:#?}", &dispatch_type, dispatch_data);

                            // Only inform the end user of dispatch events that they can handle
                            if let DispatchEvent::External(dispatch_type) = dispatch_type {
                                dispatch_sender.send((dispatch_type, dispatch_data)).unwrap();
                            }
                        },
                        GatewayEvent::Heartbeat => {
                            println!("Got heartbeat event: {:#?}", event);
                        },
                        GatewayEvent::Identify => {
                            println!("Got identify event: {:#?}", event);
                        },
                        GatewayEvent::PresenceUpdate => {
                            println!("Got presence update event: {:#?}", event);
                        },
                        GatewayEvent::VoiceStateUpdate => {
                            println!("Got voice update state event: {:#?}", event);
                        },
                        GatewayEvent::Resume => {
                            println!("Got resume event: {:#?}", event);
                        },
                        // Connection was likely dropped on discord's end. Mend it
                        GatewayEvent::Reconnect => {
                            //let _ = ws.reconnect(None);
                            panic!("Disconnected from the socket!")
                        },
                        GatewayEvent::RequestGuildMembers => {
                            println!("Got request guild members event: {:#?}", event);
                        },
                        GatewayEvent::InvalidSession => {
                            println!("Got invalid session event: {:#?}", event);
                        },
                        GatewayEvent::Hello => {
                            let token = std::env::var("_CLIENT_TOKEN")
                                .expect("Could not get user token!");

                            // Get and send the identify payload
                            // This allows to start receiving other events
                            let identify = _get_identify(&token, &intents);
                            let _ = socket.send(identify)
                                .expect("Failed to send identify payload");

                            // Set the interval and the next heartbeat
                            let data = event.d.unwrap();

                            if let Some(new_interval) = data.get("heartbeat_interval") {
                                let new_interval = new_interval.as_u64().expect("Could not assign the interval from hello event");
                                interval = Duration::from_millis(new_interval);
                                // 0.25 is an arbitraily chosen value meant to represent the jitter
                                // Since the jitter is only needed once, this is a better approach
                                // than using true randomness
                                next_heartbeat = Instant::now() + Duration::from_millis(((new_interval as f32) * 0.1) as u64);
                            }
                        },
                        GatewayEvent::HeartbeatAcknowledge => {
                            next_heartbeat = Instant::now() + interval;
                            //println!("Heartbeat acknowledged and reset");
                        },
                    };
                },
                Message::Binary(_) => {}
                Message::Ping(_) => {},
                Message::Pong(_) => {},
                Message::Close(_) => { break; },
                Message::Frame(_) => {},
            }
        }
    })
}

fn _get_heartbeat(sequence: usize) -> Message {
    let heartbeat = GatewayEventBody {
        op: GatewayEvent::Heartbeat as usize,
        d: Some(Value::Number(sequence.into())),
        s: None,
        t: None,
    };

    let heartbeat = serde_json::to_string(&heartbeat).unwrap();
    Message::text(heartbeat)                            
}

fn _get_identify(token: &String, intents: &u64) -> Message {
    // Structure the initial identify request
    let identify = GatewayEventBody {
        op: GatewayEvent::Identify as usize,
        s: None,
        t: None,
        d: Some(json!({
            "token": token,
            "intents": intents,
            "properties": {
                "os": std::env::consts::OS,
                "browser": "discord-rs",
                "device": "discord-rs"
            }
        }))
    };

    // Serialize the identify request into JSON
    let identify = serde_json::to_string(&identify).unwrap();
    Message::text(identify)
}