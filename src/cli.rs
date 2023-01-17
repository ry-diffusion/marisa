use std::str::FromStr;

use argh::FromArgs;
use evdev::Key;

#[derive(FromArgs, Clone)]
/// Marisa is a click duplicater
pub struct Marisa {
    /// a keybind to toggle (default: LeftCtrl g)
    #[argh(
        option,
        default = "vec![Key::KEY_LEFTCTRL, Key::KEY_G]",
        from_str_fn(keybind_from_str)
    )]
    pub toggle_keybind: Vec<Key>,

    /// repeat N times (default: 2)
    #[argh(option, default = "2")]
    pub repeat_by: u64,

    /// wait T time to click (default: 20)
    #[argh(option, default = "30")]
    pub delta_time: u64,
}

fn keybind_from_str(keybind: &str) -> Result<Vec<Key>, String> {
    let mut result = vec![];

    for key in keybind.split_whitespace() {
        let s = format!("KEY_{}", key.to_uppercase());

        result.push(Key::from_str(&s).map_err(|_| String::from("invalid key"))?);
    }

    Ok(result)
}
