use reqwest::{
    Client as ReqwestClient,
    Response,
    Error as ReqwestError,
};

#[derive(Debug)]
pub enum RequestError {
    HttpClientError(ReqwestError),
    MissingEnvironmentVariable(String),
    InvalidMethod(String),
}

impl From<ReqwestError> for RequestError {
    fn from(error: ReqwestError) -> Self {
        RequestError::HttpClientError(error)
    }
}

pub async fn get(path: &str) -> Result<Response, RequestError> {
    perform_request("GET", path, None).await
}

pub async fn post(path: &str, body: &str) -> Result<Response, RequestError> {
    perform_request("POST", path, Some(body)).await
}

pub async fn patch(path: &str, body: &str) -> Result<Response, RequestError> {
    perform_request("PATCH", path, Some(body)).await
}

pub async fn put(path: &str, body: &str) -> Result<Response, RequestError> {
    perform_request("PUT", path, Some(body)).await
}

async fn perform_request(
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<Response, RequestError> {
    let base_url = std::env::var("_DISCORD_API_URL")
        .map_err(|_| RequestError::MissingEnvironmentVariable("_DISCORD_API_URL".to_owned()))?;
    let token = std::env::var("_CLIENT_TOKEN")
        .map_err(|_| RequestError::MissingEnvironmentVariable("_CLIENT_TOKEN".to_owned()))?;

    let client = ReqwestClient::new();
    let url = format!("{}/{}", base_url, path);

    let request_builder = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PATCH" => client.patch(&url),
        "PUT" => client.put(&url),
        _ => return Err(RequestError::InvalidMethod(method.to_owned())),
    };

    let response = request_builder
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .body(body.unwrap_or("").to_owned())
        .send()
        .await?;

    Ok(response)
}