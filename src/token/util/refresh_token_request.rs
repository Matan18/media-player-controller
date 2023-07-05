use std::collections::HashMap;
use std::env;

use base64::{engine::general_purpose, Engine};
use reqwest::Client;

pub async fn refresh_token_request(
    refresh_token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let mut form = HashMap::new();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let spotify_access_token_uri = env::var("SPOTIFY_ACCESS_TOKEN_URI").unwrap();

    form.insert("grant_type", "refresh_token");
    form.insert("refresh_token", refresh_token);

    let auth_token: String =
        general_purpose::STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));

    return client
        .post(spotify_access_token_uri)
        .form(&form)
        .header("Authorization", format!("Basic {}", auth_token))
        .send()
        .await;
}
