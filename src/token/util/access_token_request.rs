use std::collections::HashMap;
use std::env;

use base64::{engine::general_purpose, Engine};
use reqwest::Client;

pub async fn access_token_request(code: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let mut form = HashMap::new();

    let grant_type = env::var("GRANT_TYPE").unwrap();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let redirect_uri = env::var("REDIRECT_URI").unwrap();
    let spotify_access_token_uri = env::var("SPOTIFY_ACCESS_TOKEN_URI").unwrap();

    form.insert("code", code.to_string());
    form.insert("grant_type", grant_type);
    form.insert("redirect_uri", redirect_uri);

    let auth_token: String =
        general_purpose::STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));

    let response = client
        .post(spotify_access_token_uri)
        .form(&form)
        .header("Authorization", format!("Basic {}", auth_token))
        .send()
        .await;
    return response;
}
