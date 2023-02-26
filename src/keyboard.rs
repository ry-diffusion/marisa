use rdev::Key;

use crate::context::Context;

pub fn on_keypress(context: &Context, key: Key) {
    if key == Key::F12 {
        context.set_enabled(!context.is_enabled());
        println!("toggled to: {}", context.is_enabled());
    }
}
