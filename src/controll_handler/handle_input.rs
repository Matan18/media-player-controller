use super::{
    validate_extra_button::validate_extra_button,
    validate_specific_button::validate_specific_button, validate_trigger::validate_trigger,
};
use event_emitter_rs::EventEmitter;

pub fn handle_input(buf: &[u8], event: &mut EventEmitter) {
    validate_specific_button(buf[7], event);
    validate_extra_button(buf[8], event);
    validate_trigger(buf[10], "L2", event);
    validate_trigger(buf[11], "R2", event);
}
