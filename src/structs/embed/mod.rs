#![allow(dead_code)]
//use chrono::TimeZone;

pub mod types;
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

impl Default for Embed {
    fn default() -> Self {
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
}

impl Embed {
    /// Creates a rich Embed object
    /// # Example
    /// ```
    /// use discord-rs::embed::Embed;
    /// 
    /// let embed = Embed::new();
    /// embed.set_author("Discord-rs", None, None, None)
    ///     .set_title("A new rich embed")
    ///     .set_description("A new rich embed has appeared")
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_author(&mut self,
        name: &str,
        url: Option<&str>,
        icon_url: Option<&str>,
        proxy_icon_url: Option<&str>
    ) -> &mut Self {
        self.author = Some(EmbedAuthor {
            name: name.to_string(),
            url: url.map(|s| s.to_string()),
            icon_url: icon_url.map(|s| s.to_string()),
            proxy_icon_url: proxy_icon_url.map(|s| s.to_string())
        });
        self
    }

    /// Sets just the name property for the author
    /// If the author property doesnt exist, it creates one
    pub fn set_author_name(&mut self, name: &str) {
        // Check if the `author` field is already initialized
        if let Some(author) = &mut self.author {
            // Update the name
            author.name = name.to_string();
        } else {
            // If `author` is None, create a new `EmbedAuthor` and set the name
            let new_author = EmbedAuthor {
                name: name.to_string(),
                url: None,
                icon_url: None,
                proxy_icon_url: None,
            };
            self.author = Some(new_author);
        }
    }

    pub fn set_author_icon(&mut self, icon_url: &str) {
        // Check if the `author` field is already initialized
        if let Some(author) = &mut self.author {
            // Update the name
            author.icon_url = Some(icon_url.to_string());
        }

        panic!("Author property does not exist!");
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn set_footer(&mut self,
        text: &str,
        icon_url: Option<&str>,
        proxy_icon_url: Option<&str>
    ) -> &mut Self {
        self.footer = Some(EmbedFooter {
            text: text.to_string(),
            icon_url: icon_url.map(|s| s.to_string()),
            proxy_icon_url: proxy_icon_url.map(|s| s.to_string())
        });
        self
    }

    pub fn set_thumbnail(&mut self,
        url: &str,
        proxy_url: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> &mut Self {
        self.thumbnail = Some(EmbedThumbnail {
            url: url.to_string(),
            proxy_url: proxy_url.map(|s| s.to_string()),
            width,
            height
        });
        self
    }

    pub fn set_image(&mut self,
        url: &str,
        proxy_url: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> &mut Self {
        self.image = Some(EmbedImage {
            url: url.to_string(),
            proxy_url: proxy_url.map(|s| s.to_string()),
            width,
            height
        });
        self
    }

    pub fn set_video(&mut self,
        url: &str,
        proxy_url: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> &mut Self {
        self.video = Some(EmbedVideo {
            url: url.to_string(),
            proxy_url: proxy_url.map(|s| s.to_string()),
            width,
            height
        });
        self
    }

    pub fn set_timestamp(&mut self, timestamp: &str) -> &mut Self {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    pub fn set_color(&mut self, color: Option<u32>) -> &mut Self {
        self.color = color;
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    /// Adds a field to the embed.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to add to the embed.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `Embed` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut embed = Embed::new();
    /// let field = EmbedField {
    ///     name: "Field Name".to_string(),
    ///     value: "Field Value".to_string(),
    ///     inline: Some(true),
    /// };
    /// embed.add_field(field);
    /// ```
    pub fn add_field(&mut self, field: EmbedField) -> &mut Self {
        if let Some(ref mut fields) = self.fields {
            fields.push(field);
            
            if fields.len() > 10 {
                panic!("The length of 'items' has reached the warning threshold");
            }
        } else {
            self.fields = Some(vec![field]);
        }

        self
    }

    /// Adds multiple fields to the embed.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of `EmbedField` objects to add to the embed.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `Embed` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut embed = Embed::new();
    /// let field1 = EmbedField {
    ///     name: "Field 1".to_string(),
    ///     value: "Value 1".to_string(),
    ///     inline: Some(true),
    /// };
    /// let field2 = EmbedField {
    ///     name: "Field 2".to_string(),
    ///     value: "Value 2".to_string(),
    ///     inline: Some(false),
    /// };
    /// embed.add_fields(&[field1, field2]);
    /// ```
    pub fn add_fields(&mut self, fields: &[EmbedField]) -> &mut Self {
        if let Some(ref mut existing_fields) = self.fields {
            existing_fields.extend_from_slice(fields);

            if existing_fields.len() > 10 {
                panic!("The length of 'fields' has surpassed the amount allowed by the Discord API");
            }
        } else {
            self.fields = Some(fields.to_vec());
        }

        self
    }

    /// Sets the fields of the embed to the provided vector of fields, replacing any existing fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of `EmbedField` instances representing the new fields.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `Embed` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut embed = Embed::new();
    /// let field1 = EmbedField {
    ///     name: "Field 1".to_string(),
    ///     value: "Value 1".to_string(),
    ///     inline: Some(true),
    /// };
    /// let field2 = EmbedField {
    ///     name: "Field 2".to_string(),
    ///     value: "Value 2".to_string(),
    ///     inline: Some(false),
    /// };
    /// let new_fields = vec![field1, field2];
    /// embed.set_fields(new_fields);
    /// ```
    pub fn set_fields(&mut self, fields: Vec<EmbedField>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    /// Removes a specific field from the embed.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the field to remove.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `Embed` instance.
    ///
    /// # Panics
    ///
    /// This function will panic if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut embed = Embed::new();
    /// let field1 = EmbedField {
    ///     name: "Field 1".to_string(),
    ///     value: "Value 1".to_string(),
    ///     inline: Some(true),
    /// };
    /// let field2 = EmbedField {
    ///     name: "Field 2".to_string(),
    ///     value: "Value 2".to_string(),
    ///     inline: Some(false),
    /// };
    /// embed.add_fields(&[field1, field2]);
    /// embed.remove_field(1);
    /// 
    /// assert_eq!(embed.fields.len(), 0)
    /// assert_eq!(embed.fields[0].name, "Field 2".to_string())
    /// ```
    pub fn remove_field(&mut self, index: usize) -> &mut Self {
        if let Some(ref mut fields) = self.fields {
            if index >= fields.len() {
                panic!("Field index out of bounds");
            }

            fields.remove(index);
        }

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

    pub fn clear_url(&mut self) -> &mut Self {
        self.url = None;
        self
    }

    pub fn clear_fields(&mut self) -> &mut Self {
        self.fields = None;
        self
    }

    /// Copies the structure of another embed object onto itself.
    ///
    /// # Arguments
    ///
    /// * `embed` - A reference to the `Embed` object from which the structure will be copied.
    ///
    /// # Examples
    ///
    /// ```
    /// use discord-rs::embed::Embed;
    ///
    /// let source_embed = Embed::new()
    /// source_embed
    ///     .set_title("Original Embed")
    ///     .set_description("This is the original embed.");
    ///
    /// let copied_embed = Embed::from(&source_embed);
    ///
    /// assert_eq!(copied_embed.title, source_embed.title);
    /// assert_eq!(copied_embed.description, source_embed.description);
    /// // ... copy other properties as needed
    /// ```
    ///
    /// # Returns
    ///
    /// The newly created `Embed` object with the same structure as the provided `embed`.
    pub fn from(embed: &Embed) -> Self {
        embed.clone()
    }

    /// Return the json representation of an embed object
    /// # Arguments
    ///
    /// * `pretty` - Whether to include indenting and line-breaking the make the output more human friendly.
    pub fn to_json(&self, pretty: bool) -> String {
        if pretty {
            return serde_json::to_string_pretty(self).expect("Could not stringify embed");
        }

        serde_json::to_string(self).expect("Could not stringify embed")
    }
}