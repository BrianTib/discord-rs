use serde_json::{Value};
//use std::result::Result;
use std::collections::HashMap;
use reqwest::Client;

use crate::embed::Embed;

//use crate::request::post;

mod types;
pub use types::{
    WebhookClient,
    MessageCreateOptions,
    MessageSendOptions
};

//body["embeds"] = json!(embeds);
impl WebhookClient {
    pub fn new(id: String, token: String) -> WebhookClient {
        WebhookClient { id, token, client: Client::new() }
    }

    pub async fn send(&self,
        content: Option<String>,
        embeds: Option<Vec<&Embed>>,
        username: Option<String>,
        avatar_url: Option<String>,
        tts: Option<bool>
    ) -> Result<(), &'static str> {
        let mut body: HashMap<String, Value> = HashMap::new();
    
        if let Some(_content) = content {
            body.insert("content".to_string(), Value::String(_content));
        }

        if let Some(_username) = username {
            if _username.len() > 256 {
                return Err("username length exceeded");
            }

            body.insert("username".to_string(), Value::String(_username));
        }

        if let Some(_avatar_url) = avatar_url {
            body.insert("avatar_url".to_string(), Value::String(_avatar_url));
        }

        if let Some(_tts) = tts {
            body.insert("tts".to_string(), Value::Bool(_tts));
        }
    
        if let Some(_embeds) = embeds {
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
    
        let url = format!("https://discord.com/api/webhooks/{}/{}?wait=true", self.id, self.token);
        

        println!("body: {:?}", body);
        let res = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .ok()
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{:?}", res);
        Ok(())
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