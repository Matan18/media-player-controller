use event_emitter_rs::EventEmitter;

pub fn validate_extra_button(byte: u8, event: &mut EventEmitter) {
    match byte {
        1 => event.emit("prev_music", "L1"),
        2 => event.emit("next_music", "R1"),
        4 => event.emit("key", "L2"),
        8 => event.emit("key", "R2"),
        16 => event.emit("play_stop", "Option"),
        32 => event.emit("unset_random_music", "Share"),
        64 => event.emit("key", "L3"),
        128 => event.emit("key", "R3"),
        rest => event.emit("extra", rest),
    };
}
