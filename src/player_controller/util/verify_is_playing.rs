use std::env;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CurrentlyPlaying {
    is_playing: bool,
}

const SPOTIFY_NEXT_TRACK: &'static str = "/me/player/currently-playing";

pub async fn verify_is_playing(access_token: String) -> bool {
    let spotify_player_base_url = env::var("SPOTIFY_PLAYER_BASE_URL").unwrap();

    let authoriation_header = format!("Bearer {}", access_token);

    let client = Client::new();

    let url = format!(
        "{}{}",
        spotify_player_base_url,
        SPOTIFY_NEXT_TRACK.to_string()
    );

    let response = client
        .get(url)
        .header("Authorization", authoriation_header)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();

    return serde_json::from_str::<CurrentlyPlaying>(text.as_str())
        .unwrap()
        .is_playing;
}
