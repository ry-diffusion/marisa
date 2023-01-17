use color_eyre::owo_colors::OwoColorize;
use evdev::{Device, InputEventKind, Key};

use crate::context::Context;

fn fmt_bool(b: bool) -> String {
    if b {
        " ON ".on_bright_green().black().to_string()
    } else {
        " OFF ".on_bright_red().black().to_string()
    }
}

pub fn listen(mut device: Device, context: &Context, kb_deps: &[Key]) -> color_eyre::Result<()> {
    let mut pressed = vec![];
    let toggle = " TOGGLE ".on_bright_blue().black();

    loop {
        for event in device.fetch_events()? {
            if kb_deps.len() > 1 && event.value() == 2 {
                continue;
            } else if kb_deps.len() == 1 && event.value() != 0 {
                continue;
            }

            if let InputEventKind::Key(key) = event.kind() {
                pressed.push(key);

                if pressed.len() >= kb_deps.len() {
                    if &pressed[0..kb_deps.len()] == kb_deps {
                        context.set_enabled(!context.is_enabled());
                        println!("{toggle} {}", fmt_bool(context.is_enabled()));
                    }

                    pressed.clear();
                }
            }
        }
    }
}
