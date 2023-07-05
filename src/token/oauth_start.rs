use super::util::format_spotify_url::format_spotify_url;

#[get("/")]
pub fn oauth_start() -> rocket::response::Redirect {
    let redirect_uri = format_spotify_url();
    return rocket::response::Redirect::to(redirect_uri);
}
