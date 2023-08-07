use std::time::Instant;
use tungstenite::{
    connect,
    Error
};

pub use std::net::TcpStream;
pub use tungstenite::{
    WebSocket,
    stream::MaybeTlsStream
};

#[derive(Debug)]
pub struct WebsocketConnection {
    /// A url for the socket to connect to E.g. `wss://www.mysocket.com`
    pub client: WebSocket<MaybeTlsStream<TcpStream>>,
    pub host: String,
    pub interval: Option<u64>,
    pub last_sequence: u64,
    pub next_heartbeat: Option<Instant>
}

impl WebsocketConnection {
    pub fn new(host: &str) -> Result<Self, Error> {
        let (socket, _) = connect(host)?;

        Ok(Self {
            host: host.to_string(),
            client: socket,
            last_sequence: 0,
            interval: None,
            next_heartbeat: None
        })
    }

    /// Reconnects the socket by creating a new connection either to it's original host
    /// or a new one
    pub fn reconnect(&mut self, host: Option<&str>) -> Result<(), Error> {
        // In the case that we are changing hosts, store the new host
        if let Some(host) = host {
            self.host = host.to_string();
        }

        let (socket, _) = connect(&self.host)?;
        self.client = socket;
        Ok(())
    }
}