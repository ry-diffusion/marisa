use evdev::{Device, InputEventKind, Key};

use crate::context::Context;

pub fn listen(mut device: Device, context: &Context, kb_deps: &[Key]) -> color_eyre::Result<()> {
    let mut pressed = vec![];

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
                        println!("toggling from {}", context.is_enabled());

                        context.set_enabled(!context.is_enabled());
                    }

                    pressed.clear();
                }
            }
        }
    }
}
