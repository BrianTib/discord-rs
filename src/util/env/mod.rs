use std::env::{var, set_var, VarError};

fn get_prefixed(key: &str) -> String {
    format!("DISCORD_RS_{key}")
}

pub fn set_client_token(token: &str) {
    set_var(get_prefixed("CLIENT_TOKEN"), token);
}

pub fn get_client_token() -> Result<String, VarError> {
    var(get_prefixed("CLIENT_TOKEN"))
}

pub fn set_api_url(api_version: &u8) {
    set_var(
        get_prefixed("DISCORD_API_URL"), 
        format!("https://discord.com/api/v{api_version}")
    );
}

pub fn get_api_url() -> Result<String, VarError> {
    var(get_prefixed("DISCORD_API_URL"))
}