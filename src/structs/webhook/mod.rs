#[allow(dead_code, unused_imports)]
use serde_json::{Value};
use std::collections::HashMap;
use reqwest::Client;

use crate::structs::embed::Embed;

pub mod types;
pub use types::{
    WebhookClient,
    MessagePayload,
};

pub mod errors;
pub use errors::{
    InvalidUrlError,
    ExtractionError
};

impl WebhookClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            id: None,
            token: None,
            url: None
        }
    }

    pub fn with_credentials(&mut self, id: &str, token: &str) -> &mut Self {
        self.id = Some(id.to_string());
        self.token = Some(token.to_string());
        self.url = Some(format!("https://discord.com/api/webhooks/{}/{}", id, token));
        self
    }

    /// Sets the webhook URL by extracting the ID and token from the provided URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The webhook URL in the format: "https://discord.com/api/webhooks/{ID}/{TOKEN}"
    ///
    /// # Errors
    ///
    /// Returns an `InvalidUrlError` if the provided URL is invalid or doesn't match the expected format.
    /// Returns a `ExtractionError` if the ID or token cannot be extracted from the URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use discord_rs::webhook::WebhookClient;
    /// 
    /// fn main() {
    ///     let url = "https://discord.com/api/webhooks/1234567890/abcdefghijklmnopqrstuvwxyz-1234567890";
    /// 
    ///     let mut client = WebhookClient::new()
    ///       .with_url(url);
    /// }
    /// ```
    pub fn with_url(&mut self, url: &str) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let prefix = "https://discord.com/api/webhooks/";
        if let Some(rest) = url.strip_prefix(prefix) {
            if let Some(index) = rest.find('/') {
                let id = &rest[..index];
                let token = &rest[index + 1..];
                self.id = Some(id.to_string());
                self.token = Some(token.to_string());
                self.url = Some(url.to_string());
                Ok(self)
            } else {
                Err(Box::new(ExtractionError::new("Failed to extract ID and token from URL")))
            }
        } else {
            Err(Box::new(InvalidUrlError::new("Invalid webhook URL")))
        }
    }

    /// Sends a message through the webhook
    /// 
    /// # Arguments
    /// `payload` - A reference to a payload object
    /// 
    /// # Example
    /// ```
    /// use discord_rs::webhook::{WebhookClient, MessagePayload};
    /// 
    /// let webhook = WebhookClient::new("YOUR_ID", "YOUR_TOKEN");
    /// let message_payload = MessagePayload::new()
    ///     .set_username("Captain Hook")
    ///     .set_content("Hello World!");
    /// 
    /// webhook.send(message_payload).await.expect("Failed to send webhook");
    /// ```

    pub async fn send(&self, payload: MessagePayload) -> Result<(), &'static str> {
        if self.url.is_none() {
            return Err("No URL for webhook. Consider using WebhookClient::with_credentials() or WebhookClient::with_url()");
        }

        let mut body: HashMap<String, Value> = HashMap::new();
    
        if let Some(content) = payload.content {
            body.insert("content".to_string(), Value::String(content));
        }
    
        if let Some(username) = payload.username {
            if username.len() > 256 {
                return Err("Username length exceeded");
            }
            body.insert("username".to_string(), Value::String(username));
        }
    
        if let Some(avatar_url) = payload.avatar_url {
            body.insert("avatar_url".to_string(), Value::String(avatar_url));
        }
    
        if let Some(tts) = payload.tts {
            body.insert("tts".to_string(), Value::Bool(tts));
        }
    
        if let Some(embeds) = payload.embeds {
            if embeds.len() > 10 {
                return Err("Embed length exceeded");
            }
    
            let mut total_len: u32 = 0;
            for (i, embed) in embeds.iter().enumerate() {
                total_len += check_field_length(
                    Some(&embed.title.as_ref().unwrap()),
                    256,
                    &format!("Embed title length exceeded for {}nth embed", i)
                ).unwrap_or_default() as u32;
    
                total_len += check_field_length(
                    Some(&embed.description.as_ref().unwrap()),
                    4096,
                    &format!("Embed description length exceeded for {}nth embed", i)
                ).unwrap_or_default() as u32;
    
                if let Some(fields) = embed.fields.as_ref() {
                    for (j, field) in fields.iter().enumerate() {
                        total_len += check_field_length(
                            Some(&field.name),
                            256,
                            &format!("Embed field name length exceeded for {}nth field on the {}nth embed", j, i)
                        ).unwrap_or_default() as u32;
                    }
                }
    
                if let Some(footer) = embed.footer.as_ref() {
                    total_len += check_field_length(
                        Some(&footer.text),
                        2048,
                        &format!("Embed footer text length exceeded for text footer on the {}nth embed", i)
                    ).unwrap_or_default() as u32;
                }
            }
    
            if total_len > 6000 {
                return Err("Total embed length exceeds 6,000 characters");
            }
    
            let embed_values = embeds
                .into_iter()
                .map(|embed| serde_json::to_value(&embed).expect("Failed to serialize embed"))
                .collect();
            
            body.insert("embeds".to_string(), Value::Array(embed_values));
        }
    
        let url = format!("{}?wait=true", self.url.as_ref().unwrap());
        
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
    }
}

fn check_field_length(field: Option<&str>, limit: usize, error_message: &str) -> Result<usize, String> {
    let field_len = field.unwrap_or_default().len();

    if field_len > limit {
        return Err(error_message.to_string())
    }

    Ok(field_len)
}

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