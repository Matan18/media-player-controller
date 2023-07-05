use event_emitter_rs::EventEmitter;

pub fn validate_specific_button(byte: u8, event: &mut EventEmitter) {
    match byte {
        0_u8 => event.emit("key", "Up"),
        2_u8 => event.emit("key", "Right"),
        4_u8 => event.emit("key", "Down"),
        6_u8 => event.emit("key", "Left"),
        24_u8 => event.emit("unset_random_music", "Square"),
        40_u8 => event.emit("set_random_music", "X"),
        72_u8 => event.emit("set_repeat_playlist", "Circle"),
        136_u8 => event.emit("set_repeat_music", "Triangle"),
        rest => event.emit("specific", rest),
    };
}
