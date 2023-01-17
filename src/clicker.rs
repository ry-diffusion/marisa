use std::time::Duration;

use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, Device, EventType, InputEvent, InputEventKind, Key,
};

use crate::context::Context;

pub fn listen(mut device: Device, context: &Context) -> color_eyre::Result<()> {
    let mut attribs = AttributeSet::<Key>::new();
    attribs.insert(Key::BTN_LEFT);
    attribs.insert(Key::BTN_RIGHT);
    let mut dev = VirtualDeviceBuilder::new()?
        .name(format!("Marisa for: {}", device.name().unwrap_or("unknown")).as_str())
        .with_keys(&attribs)?
        .build()?;

    loop {
        for event in device.fetch_events()? {
            // if event.value() == 0 {
            //     continue;
            // }

            if let InputEventKind::Key(key) = event.kind() {
                if matches!(key, Key::BTN_LEFT | Key::BTN_RIGHT) {
                    if !context.is_enabled() {
                        continue;
                    }

                    std::thread::sleep(Duration::from_millis(15));

                    let press = InputEvent::new(EventType::KEY, key.0, 1);
                    dev.emit(&[press])?;

                    let release = InputEvent::new(EventType::KEY, key.0, 0);
                    std::thread::sleep(Duration::from_millis(15));
                    dev.emit(&[release])?;
                }
            }
        }
    }
}
