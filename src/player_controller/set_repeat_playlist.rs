use std::env;

use reqwest::{header::HeaderMap, Client};

use crate::util::get_refreshed_token::get_refreshed_token;

const SPOTIFY_NEXT_TRACK: &'static str = "/me/player/repeat";

pub async fn set_repeat_playlist() -> Result<reqwest::Response, reqwest::Error> {
    let spotify_player_base_url = env::var("SPOTIFY_PLAYER_BASE_URL").unwrap();

    let access_token = get_refreshed_token().await;

    let authoriation_header = format!("Bearer {}", access_token);
    let mut headers = HeaderMap::new();
    headers.insert("authorization", authoriation_header.parse().unwrap());
    headers.insert("Content-length", "0".parse().unwrap());

    let query = &[("state", "context")];

    let url = format!(
        "{}{}",
        spotify_player_base_url,
        SPOTIFY_NEXT_TRACK.to_string()
    );
    let client = Client::new();
    let response = client.put(url).query(query).headers(headers).send().await;

    return response;
}
