extern crate dotenv;

use dotenv::dotenv;
use std::thread;

use controll_handler::controll_handler;
use token::{oauth_start::oauth_start, success_callback::success_callback};

#[macro_use]
extern crate rocket;

mod controll_handler;
mod player_controller;
mod token;
mod util;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let handle = thread::spawn(|| {
        controll_handler();
    });

    let _rocket = rocket::build()
        .mount("/", routes![oauth_start, success_callback])
        .launch()
        .await?;

    handle.join().unwrap();

    Ok(())
}
