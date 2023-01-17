use std::str::FromStr;

use evdev::{Device, Key};

use crate::context::Context;

pub fn listen(mut device: Device, context: &Context) -> color_eyre::Result<()> {
    Ok(())
}
