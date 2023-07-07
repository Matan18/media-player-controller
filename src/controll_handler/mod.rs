mod handle_input;
mod validate_extra_button;
mod validate_specific_button;
mod validate_trigger;

extern crate hidapi;

use event_emitter_rs::EventEmitter;
use hidapi::HidApi;
use std::{thread, time};

use crate::{controll_handler::handle_input::handle_input, player_controller::register_events};

const POSITION: usize = 24;
const SIZE: usize = POSITION + 2;

pub fn controll_handler() {
    let psvendor_id = 0x054c;
    let ps4product_id = 0x05c4;
    let mut buf = [0u8; SIZE];
    let ten_millis = time::Duration::from_millis(1000);

    let api = HidApi::new().unwrap();
    let device = HidApi::open(&api, psvendor_id, ps4product_id).unwrap();
    let mut event_emitter = EventEmitter::new();

    register_events(&mut event_emitter);
    loop {
        let res = device.read(&mut buf[..]).unwrap();
        // println!("{:?}", &buf);
        handle_input(&buf[..res], &mut event_emitter);

        thread::sleep(ten_millis);
    }
}
