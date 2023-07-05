// extern crate redis;
extern crate serde;
extern crate serde_json;

use redis::Commands;

use crate::util::parse_token::parse_access_token;

use super::util::access_token_request::access_token_request;

#[get("/callback?<code>&<state>")]
pub async fn success_callback(code: &str, state: &str) -> String {
    println!("callback state {}", state);
    let response = access_token_request(code).await.unwrap();

    let status = response.status().clone();
    if status.is_client_error() {
        let text = response.text().await.unwrap();
        return format!("text: {}", text);
    }
    let text = response.text().await.unwrap();
    let tok = parse_access_token(text.clone()).unwrap();

    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut con = client.get_connection().unwrap();
    con.set::<String, String, String>("refresh_token".to_string(), tok.refresh_token)
        .unwrap();
    con.set::<String, String, String>("token_data".to_string(), text.clone())
        .unwrap();

    return format!("{{ \"text\": {}, \"status\": {} }}", text, status.as_str());
}
