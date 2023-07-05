use std::env;

use reqwest::Url;

use super::generate_random_string::generate_random_string;

const SCOPE: &'static str = "user-read-private user-modify-playback-state user-read-currently-playing user-read-recently-played user-read-playback-state";

pub fn format_spotify_url() -> String {
    let state: String = generate_random_string(10);
    let spotify_authorize_uri = env::var("SPOTIFY_AUTHORIZE_URI").unwrap();
    let client_id = env::var("CLIENT_ID").unwrap();
    let response_type = env::var("RESPONSE_TYPE").unwrap();
    let redirect_uri = env::var("REDIRECT_URI").unwrap();

    let url = Url::parse_with_params(
        spotify_authorize_uri.as_str(),
        [
            ("state", &state),
            ("scope", &SCOPE.to_string()),
            ("client_id", &client_id),
            ("redirect_uri", &redirect_uri),
            ("response_type", &response_type),
        ],
    );

    match url {
        Ok(url_string) => return url_string.to_string(),
        Err(err) => {
            print!("error: {}", err);
            panic!("{}", err)
        }
    }
}
