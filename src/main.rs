use std::sync::Arc;

use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};

mod context;
mod keyboard;
mod mouse;
use context::Context;

static CONTEXT: Lazy<Arc<Context>> = Lazy::new(|| Arc::new(Context::new()));

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(kp) => keyboard::on_keypress(&CONTEXT, kp),
        EventType::ButtonPress(bp) => {
            CONTEXT.replace_click();
            mouse::on_click(CONTEXT.clone(), bp);
        }
        EventType::ButtonRelease(_) => {
            CONTEXT.update_average();
        }
        _ => (),
    }
}

fn main() {
    println!("Marisa v{} BETA", env!("CARGO_PKG_VERSION"));
    println!("Press <F12> To toggle Marisa.");
    if let Err(e) = listen(callback) {
        println!("listen error: {e:?}");
    }
}
