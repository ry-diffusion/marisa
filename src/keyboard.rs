use std::str::FromStr;

use evdev::{Device, InputEventKind, Key};

use crate::context::Context;

pub fn listen(mut device: Device, context: &Context) -> color_eyre::Result<()> {
    loop {
        for event in device.fetch_events()? {
            if event.value() == 0 {
                continue;
            }

            if let InputEventKind::Key(key) = event.kind() {}
        }
    }
}
