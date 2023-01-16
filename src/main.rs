use std::thread;

use cli::Marisa;
use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, Device, EventType, InputEvent, InputEventKind, Key,
};
use utils::find_mouse_devices;

mod cli;
mod utils;

fn listen(mut device: Device) -> color_eyre::Result<()> {
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
                    for _ in 0..10 {
                        std::thread::sleep(std::time::Duration::from_millis(15));
                        let event = InputEvent::new(EventType::KEY, key.0, 1);
                        dev.emit(&[event])?;

                        std::thread::sleep(std::time::Duration::from_millis(15));
                        let event = InputEvent::new(EventType::KEY, key.0, 0);
                        dev.emit(&[event])?;
                    }
                }
            }
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Marisa = argh::from_env();
    let devices = if args.device_path == *"auto" {
        find_mouse_devices()?
    } else {
        let device = Device::open(args.device_path)?;

        vec![device]
    };
    let mut threads = Vec::with_capacity(devices.len());
    println!("Found {} device(s)", devices.len());
    println!("Listening started.");

    for device in devices {
        println!("Running on {}", device.name().unwrap_or("unknown"));
        threads.push(thread::spawn(move || -> color_eyre::Result<()> {
            listen(device)
        }));
    }

    for thread in threads {
        thread.join().unwrap()?;
    }

    Ok(())
}
