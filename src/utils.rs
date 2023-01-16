use color_eyre::eyre::ContextCompat;
use evdev::{Device, Key};

pub fn find_mouse_devices() -> color_eyre::Result<Vec<Device>> {
    let mut devices = vec![];

    for (_, device) in evdev::enumerate() {
        if let Ok(sup) = device
            .supported_keys()
            .context("Failed to get supported keys")
        {
            if sup.contains(Key::BTN_LEFT) {
                devices.push(device)
            }
        }
    }

    Ok(devices)
}
