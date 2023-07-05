extern crate dotenv;

use dotenv::dotenv;

use token::{oauth_start::oauth_start, success_callback::success_callback};

#[macro_use]
extern crate rocket;

mod token;
mod util;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let _rocket = rocket::build()
        .mount("/", routes![oauth_start, success_callback])
        .launch()
        .await?;

    Ok(())
}
