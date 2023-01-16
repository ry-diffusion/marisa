use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use cli::Marisa;
use evdev::{Device, InputEventKind, Key};
use utils::find_mouse_devices;

mod cli;
mod utils;

fn listen(mut device: Device) -> color_eyre::Result<()> {
    loop {
        for event in device.fetch_events()? {
            if event.value() == 0 {
                continue;
            }

            if let InputEventKind::Key(key) = event.kind() {
                if matches!(key, Key::BTN_LEFT | Key::BTN_RIGHT) {
                    println!("*Click*");
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
