use std::{str::FromStr, sync::Arc, thread};

use cli::Marisa;

mod cli;
mod clicker;
mod context;
mod devices;
mod keyboard;
use context::Context;
use devices::Devices;
use evdev::Key;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let marisa: Marisa = argh::from_env();

    let devices = Devices::find()?;
    let mut threads = Vec::new();
    let context = Arc::new(Context::new());
    for device in devices.mouses {
        println!("Running on {}", device.name().unwrap_or("unknown"));
        let context = context.clone();
        threads.push(thread::spawn(move || -> color_eyre::Result<()> {
            clicker::listen(device, &context)
        }));
    }

    for device in devices.keyboards {
        println!("Running on {}", device.name().unwrap_or("unknown"));
        let context = context.clone();
        threads.push(thread::spawn(move || -> color_eyre::Result<()> {
            keyboard::listen(device, &context)
        }));
    }

    for thread in threads {
        thread.join().unwrap()?;
    }

    Ok(())
}
