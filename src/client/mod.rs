use websocket::{ClientBuilder, Message};
use websocket::header::{Headers, Authorization, Bearer};
use reqwest::Client as ReqwestClient;

pub mod types;
pub use types::{Client, GatewayIntentBits};

impl Client<'_> {
    pub fn new(intents: &[GatewayIntentBits]) -> Self {
        let bits = intents.iter()
            .fold(0, |acc, intent| {
                acc | (1 << *intent as usize)
            });
        
        Self {
            intents: (bits, intents.to_vec()),
            token: None,
            ws: ClientBuilder::new("wss://gateway.discord.gg/?encoding=json").unwrap(),
        }
    }

    pub async fn login(&mut self, token: &str) -> Result<(), &'static str> {
        self.token = Some(token.to_string());
        //println!("Test");

        // let client = ReqwestClient::new()
        //     .get("https://discord.com/api/gateway/bot")
        //     .send()
        //     .await;

        // if client.is_err() {
        //     return Err("Failed to get gateway");
        // }

        // println!("Debug {:?}", client);
        
        // let mut headers = Headers::new();
        // headers.set(
        //     Authorization(
        //         Bearer {
        //             token: token.to_owned()
        //         }
        //     )
        //  );

        // self.ws
        //     .custom_headers(&headers)
        //     .connect_secure(None)
        //     .unwrap();

        Ok(())
    }
}
