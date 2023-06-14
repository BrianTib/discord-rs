#![allow(dead_code)]
use chrono::{Utc, DateTime, TimeZone};

mod types;
pub use types::{
    Embed,
    EmbedAuthor,
    EmbedField,
    EmbedFooter,
    EmbedImage,
    EmbedProvider,
    EmbedThumbnail,
    EmbedTypes,
    EmbedVideo,
};

impl Embed {
    pub fn new() -> Self {
        Self {
            title: None,
            embed_type: EmbedTypes::Rich as u8,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }

    pub fn from(embed: &Embed) -> Self {
        embed.clone()
    }

    pub fn to_json(&self, pretty: bool) -> String {
        if pretty == true {
            return serde_json::to_string_pretty(self).expect("Could not stringify embed");
        }

        return serde_json::to_string(self).expect("Could not stringify embed");
    }

    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn set_author(&mut self,
        name: String,
        url: Option<String>,
        icon_url: Option<String>,
        proxy_icon_url: Option<String>
    ) -> &mut Self {
        self.author = Some(EmbedAuthor { name, url, icon_url, proxy_icon_url });
        self
    }

    pub fn set_footer(&mut self,
        text: String,
        icon_url: Option<String>,
        proxy_icon_url: Option<String>
    ) -> &mut Self {
        self.footer = Some(EmbedFooter { text, icon_url, proxy_icon_url });
        self
    }

    pub fn set_thumbnail(&mut self,
        url: String,
        proxy_url: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> &mut Self {
        self.thumbnail = Some(EmbedThumbnail { url, proxy_url, width, height });
        self
    }

    pub fn set_image(&mut self,
        url: String,
        proxy_url: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> &mut Self {
        self.image = Some(EmbedImage { url, proxy_url, width, height });
        self
    }

    pub fn set_video(&mut self,
        url: String,
        proxy_url: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> &mut Self {
        self.video = Some(EmbedVideo { url, proxy_url, width, height });
        self
    }

    pub fn set_timestamp(&mut self, timestamp: String) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn set_color(&mut self, color: Option<u32>) -> &mut Self {
        self.color = color;
        self
    }

    pub fn set_url(&mut self, url: Option<String>) -> &mut Self {
        self.url = url;
        self
    }

    pub fn clear_title(&mut self) -> &mut Self {
        self.title = None;
        self
    }

    pub fn clear_description(&mut self) -> &mut Self {
        self.description = None;
        self
    }
    
    pub fn clear_author(&mut self) -> &mut Self {
        self.author = None;
        self
    }
    
    pub fn clear_footer(&mut self) -> &mut Self {
        self.footer = None;
        self
    }
    
    pub fn clear_thumbnail(&mut self) -> &mut Self {
        self.thumbnail = None;
        self
    }
    
    pub fn clear_image(&mut self) -> &mut Self {
        self.image = None;
        self
    }
    
    pub fn clear_video(&mut self) -> &mut Self {
        self.video = None;
        self
    }

    pub fn clear_timestamp(&mut self) -> &mut Self {
        self.timestamp = None;
        self
    }
}