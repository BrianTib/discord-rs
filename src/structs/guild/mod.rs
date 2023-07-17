pub mod types;
pub use types::*;

pub mod enums;
pub use enums::*;

pub mod deserializers;
pub use deserializers::*;

impl Guild {
    // pub fn get_channel(&self, channel_id: &str) {
    //     //self.channels.iter().find(|&channel| channel.id == channel_id);
    // }
}

// Anonymous function for fetching updated data about the channel
// async fn _fetch(client: &ReqwestClient, channel_id: &String) -> Channel {
//     let token = std::env::var("_CLIENT_TOKEN").unwrap();
//     let response = client.get(format!("{API}/channels/{}", channel_id))
//         .header("Authorization", format!("Bot {token}"))
//         .send()
//         .await
//         .expect(&format!("Failed to fetch channel data for id {}", channel_id));
 
//     let response = response.text()
//         .await
//         .expect("Failed to deserialize channel data");

//     serde_json::from_str(&response).unwrap()
// }