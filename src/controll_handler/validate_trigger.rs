use event_emitter_rs::EventEmitter;

pub fn validate_trigger(byte: u8, trigger: &str, event: &mut EventEmitter) {
    match byte {
        1_u8..=128_u8 => event.emit(trigger, "first half"),
        129_u8..=254_u8 => event.emit(trigger, "second half"),
        255_u8 => event.emit(trigger, "full"),
        _ => event.emit(trigger, "none"),
    };
}
