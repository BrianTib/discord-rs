/// Error indicating that the provided URL is invalid.
#[derive(Debug)]
pub struct InvalidUrlError {
    message: String,
}

impl InvalidUrlError {
    pub fn new(message: &str) -> Self {
        InvalidUrlError {
            message: message.to_string(),
        }
    }
}

impl std::error::Error for InvalidUrlError {}

impl std::fmt::Display for InvalidUrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid URL: {}", self.message)
    }
}

/// Error indicating that the ID or token extraction from the URL failed.
#[derive(Debug)]
pub struct ExtractionError {
    message: String,
}

impl ExtractionError {
    pub fn new(message: &str) -> Self {
        ExtractionError {
            message: message.to_string(),
        }
    }
}

impl std::error::Error for ExtractionError {}

impl std::fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extraction Error: {}", self.message)
    }
}