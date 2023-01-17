use std::str::FromStr;

use argh::FromArgs;
use evdev::Key;

#[derive(FromArgs)]
/// Marisa is a click duplicater
pub struct Marisa {
    /// a keybind to toggle (default: LeftCtrl g)
    #[argh(
        option,
        default = "vec![Key::KEY_LEFTCTRL, Key::KEY_G]",
        from_str_fn(keybind_from_str)
    )]
    pub toggle_keybind: Vec<Key>,
}

fn keybind_from_str(keybind: &str) -> Result<Vec<Key>, String> {
    let mut result = vec![];

    for key in keybind.split_whitespace() {
        let s = format!("KEY_{}", key.to_uppercase());

        result.push(Key::from_str(&s).map_err(|_| String::from("invalid key"))?)
    }

    Ok(result)
}
