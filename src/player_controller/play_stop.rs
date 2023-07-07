use std::env;

use reqwest::{header::HeaderMap, Client};

use crate::util::get_refreshed_token::get_refreshed_token;

use super::util::verify_is_playing::verify_is_playing;

const SPOTIFY_NEXT_TRACK: &'static str = "/me/player/";

pub async fn play_stop() -> Result<reqwest::Response, reqwest::Error> {
    let spotify_player_base_url = env::var("SPOTIFY_PLAYER_BASE_URL").unwrap();

    let access_token = get_refreshed_token().await;

    let is_playing = verify_is_playing(access_token.clone()).await;

    let act = if is_playing { "pause" } else { "play" };

    let authoriation_header = format!("Bearer {}", access_token);
    let mut headers = HeaderMap::new();
    headers.insert("authorization", authoriation_header.parse().unwrap());
    headers.insert("Content-length", "0".parse().unwrap());

    let url = format!(
        "{}{}{}",
        spotify_player_base_url,
        SPOTIFY_NEXT_TRACK.to_string(),
        act
    );
    let client = Client::new();
    let response = client.put(url).headers(headers).send().await;

    return response;
}
