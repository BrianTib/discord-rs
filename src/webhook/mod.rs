use serde_json::{Value};
use std::collections::HashMap;
use reqwest::Client;

use crate::embed::Embed;

pub mod types;
pub use types::{
    WebhookClient,
    MessageCreateOptions,
    MessagePayload
};

impl MessagePayload {
    //! # MessagePayload
    //! 
    //! `MessagePayload` is a struct that simplifies creating data to
    //! send through the Discord API

    pub fn new() -> &'static mut Self {
        Box::leak(Box::new(Self {
            content: None,
            embeds: None,
            username: None,
            avatar_url: None,
            tts: None,
        }))
    }

    pub fn set_content(&mut self, content: &str) -> &mut Self {
        if content.len() > 2000 {
            panic!("content exceeds length of 2000 allowed by Discord's API")
        }

        self.content = Some(content.to_string());
        self
    }

    pub fn set_username(&mut self, username: &str) -> &mut Self {
        if username.len() > 256 {
            panic!("username exceeds length of 256 allowed by Discord's API")
        }

        self.username = Some(username.to_string());
        self
    }

    pub fn set_avatar(&mut self, avatar_url: &str) -> &mut Self {
        self.avatar_url = Some(avatar_url.to_string());
        self
    }

    pub fn set_tts(&mut self, tts: bool) -> &mut Self {
        self.tts = Some(tts);
        self
    }

    /// Sets the embeds of the message.
    ///
    /// # Arguments
    ///
    /// * `embeds` - An array slice of `Embed` objects representing the embeds to set.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the length of `embeds` exceeds the maximum allowed by the Discord API.
    ///
    /// # Examples
    ///
    /// ```
    /// use discord-rs::webhook::MessagePayload;
    /// 
    /// let embeds = vec![embed1, embed2];
    /// 
    /// let message = MessagePayload::new()
    /// message.set_embeds(&embeds).expect("Failed to set embeds");
    /// ```
    pub fn set_embeds(&mut self, embeds: &[Embed]) -> Result<&mut Self, &'static str> {
        if let Some(existing_embeds) = &mut self.embeds {
            existing_embeds.extend_from_slice(embeds);
    
            if existing_embeds.len() > 10 {
                return Err("The length of 'embeds' has surpassed the amount allowed by the Discord API");
            }
        } else {
            self.embeds = Some(embeds.to_vec());
        }
    
        Ok(self)
    }
}

impl WebhookClient {
    pub fn new(id: &str, token: &str) -> Self {
        Self {
            client: Client::new(),
            id: id.to_string(),
            token: token.to_string(),
            url: format!("https://discord.com/api/webhooks/{}/{}", id, token)
        }
    }

    /// Sends a message through the webhook
    /// 
    /// # Arguments
    /// `payload` - A reference to a payload object
    /// 
    /// # Example
    /// ```
    /// use discord-rs::webhook::{WebhookClient, MessagePayload};
    /// 
    /// let webhook = WebhookClient::new("YOUR_ID", "YOUR_TOKEN");
    /// let message_payload = MessagePayload::new()
    ///     .set_username("Captain Hook")
    ///     .set_content("Hello World!");
    /// 
    /// webhook.send(message_payload).await.expect("Failed to send webhook");
    /// ```

    pub async fn send(&self, payload: &MessagePayload) -> Result<(), &'static str> {
        let mut body: HashMap<String, Value> = HashMap::new();
    
        if let Some(ref _content) = payload.content {
            body.insert("content".to_string(), Value::String(_content.to_string()));
        }

        if let Some(ref _username) = payload.username {
            if _username.len() > 256 {
                return Err("username length exceeded");
            }

            body.insert("username".to_string(), Value::String(_username.to_string()));
        }

        if let Some(ref _avatar_url) = payload.avatar_url {
            body.insert("avatar_url".to_string(), Value::String(_avatar_url.to_string()));
        }

        if let Some(_tts) = payload.tts {
            body.insert("tts".to_string(), Value::Bool(_tts));
        }
    
        if let Some(ref _embeds) = payload.embeds {
            if _embeds.len() > 10 {
                return Err("Embed length exceeded");
            }

            // Enforce embed limitations before sending the request
            // https://discord.com/developers/docs/resources/channel#embed-object
            let mut total_len: u32 = 0;
            for (i, embed) in _embeds.iter().enumerate() {
                total_len += check_field_length(
                    Some(&embed.title.as_ref().unwrap()),
                    256,
                    &format!("Embed title length exceeded for {}nth embed", i)
                ).unwrap() as u32;

                total_len += check_field_length(
                    Some(&embed.description.as_ref().unwrap()),
                    4096,
                    &format!("Embed description length exceeded for {}nth embed", i)
                ).unwrap() as u32;

                if let Some(_fields) = embed.fields.as_ref() {
                    for (j, field) in _fields.iter().enumerate() {
                        total_len += check_field_length(
                            Some(&field.name),
                            256,
                            &format!("Embed field name length exceeded for {}nth field on the {}nth embed", j, i)
                        ).unwrap() as u32;
                    }
                }

                if let Some(_footer) = embed.footer.as_ref() {
                    total_len += check_field_length(
                        Some(&_footer.text),
                        2048,
                        &format!("Embed footer text length exceeded for text footer on the {}nth embed", i)
                    ).unwrap() as u32;
                }
            }

            if total_len > 6000 {
                return Err("Total embed length exceeds 6,000 characters");
            }

            let embed_values = _embeds
                .into_iter()
                .map(|embed| serde_json::to_value(&embed).expect("Failed to serialize embed"))
                .collect();
        
            body.insert("embeds".to_string(), Value::Array(embed_values));
        }
    
        let url = format!("{}?wait=true", self.url);
        
        //println!("body: {:?}", body);
        let res = self.client
            .post(&url)
            .json(&body)
            .send()
            .await;

        if res.is_ok() {
            return Ok(());
        }

        return Err("An unexpected error occured");
        
        // let text = res
        //     .ok()
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();

        // println!("{:?}", res);
        // return Err(&format!("An unknown error occured. {:?}", res.));
    }
}

fn check_field_length(field: Option<&str>, limit: usize, error_message: &str) -> Result<usize, String> {
    let field_len = field.unwrap_or_default().len();

    if field_len > limit {
        Err(error_message.to_string())
    } else {
        Ok(field_len)
    }
}