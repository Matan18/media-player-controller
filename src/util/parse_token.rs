use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyToken {
    pub scope: String,
    pub expires_in: i32,
    pub token_type: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyAccessToken {
    pub scope: String,
    pub expires_in: i32,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub fn parse_access_token(text: String) -> Result<SpotifyAccessToken, serde_json::Error> {
    return serde_json::from_str::<SpotifyAccessToken>(text.as_str());
}

pub fn parse_refresh_token(text: String) -> Result<SpotifyToken, serde_json::Error> {
    return serde_json::from_str::<SpotifyToken>(text.as_str());
}
