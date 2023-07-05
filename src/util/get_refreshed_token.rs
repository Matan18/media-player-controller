use redis::Commands;

use crate::token::util::refresh_token_request::refresh_token_request;

use super::parse_token::parse_refresh_token;

pub async fn get_refreshed_token() -> std::string::String {
    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut con = client.get_connection().unwrap();
    let refresh_token = con
        .get::<String, String>("refresh_token".to_string())
        .unwrap();

    let response = refresh_token_request(refresh_token.as_str()).await.unwrap();

    let status = response.status().clone();
    if status.is_client_error() {
        let text = response.text().await.unwrap();
        panic!("{}", text);
    }
    let text = response.text().await.unwrap();

    let tok = parse_refresh_token(text.clone()).unwrap();
    return tok.access_token;
}
