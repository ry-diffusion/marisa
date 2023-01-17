use std::{sync::Arc, thread};

use cli::Marisa;

mod cli;
mod clicker;
mod context;
mod devices;
mod keyboard;
use color_eyre::owo_colors::OwoColorize;
use context::Context;
use devices::Devices;
use evdev::Device;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let marisa: Marisa = argh::from_env();
    println!(
        "{} {}\n",
        " Marisa ".on_bright_yellow().black(),
        concat!(" v", env!("CARGO_PKG_VERSION"), " ")
            .on_bright_cyan()
            .black()
    );

    let devices = Devices::find(&marisa.toggle_keybind);
    let mut threads = Vec::new();
    let context = Arc::new(Context::new());
    let listening = " Listening ".on_bright_white().black();
    let mouse = " Mouse ".on_bright_red().black();
    let keyboard = " Keyboard ".on_bright_green().black();
    let fmt_device = |d: &Device| {
        format!(" {} ", d.name().unwrap_or("unknown"))
            .on_bright_magenta()
            .black()
            .to_string()
    };

    for device in devices.mouses {
        println!("{listening} {mouse} {}", fmt_device(&device));
        let context = context.clone();
        let marisa = marisa.clone();
        threads.push(thread::spawn(move || -> color_eyre::Result<()> {
            clicker::listen(device, &context, &marisa)
        }));
    }

    for device in devices.keyboards {
        println!("{listening} {keyboard} {}", fmt_device(&device));
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
