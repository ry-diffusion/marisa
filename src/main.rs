use std::{sync::Arc, thread};

use cli::Marisa;

mod cli;
mod clicker;
mod context;
mod devices;
mod keyboard;
use context::Context;
use devices::Devices;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let marisa: Marisa = argh::from_env();

    let devices = Devices::find(&marisa.toggle_keybind)?;
    let mut threads = Vec::new();
    let context = Arc::new(Context::new());

    for device in devices.mouses {
        println!("Listening Mouse {}", device.name().unwrap_or("unknown"));
        let context = context.clone();
        let marisa = marisa.clone();
        threads.push(thread::spawn(move || -> color_eyre::Result<()> {
            clicker::listen(device, &context, &marisa)
        }));
    }

    for device in devices.keyboards {
        println!("Listening Keyboard {}", device.name().unwrap_or("unknown"));
        let context = context.clone();
        let toggle_keybind = marisa.toggle_keybind.clone();
        threads.push(thread::spawn(move || -> color_eyre::Result<()> {
            keyboard::listen(device, &context, &toggle_keybind)
        }));
    }

    for thread in threads {
        thread.join().unwrap()?;
    }

    Ok(())
}
