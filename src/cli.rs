use std::str::FromStr;

use argh::FromArgs;
use evdev::Key;

#[derive(FromArgs, Clone)]
/// Marisa is a click duplicater
pub struct Marisa {
    /// a keybind to toggle (default: LeftCtrl g)
    #[argh(
        option,
        short = 'k',
        default = "vec![Key::KEY_LEFTCTRL, Key::KEY_G]",
        from_str_fn(keybind_from_str)
    )]
    pub keybind: Vec<Key>,

    /// repeat N times (default: 4)
    #[argh(option, short = 'r', default = "4")]
    pub repeat_by: u64,

    /// wait T time to click (default: 25)
    #[argh(option, short = 'd', default = "25")]
    pub delta_time: u64,

    /// when values timeout, starts to click (default: 400)
    #[argh(option, short = 'w', default = "400")]
    pub deadline: u64,

    /// requires N clicks to start. (default: 2)
    #[argh(option, short = 'm', default = "2")]
    pub min_clicks: u64,
}

fn keybind_from_str(keybind: &str) -> Result<Vec<Key>, String> {
    let mut result = vec![];

    for key in keybind.split_whitespace() {
        let s = format!("KEY_{}", key.to_uppercase());

        result.push(Key::from_str(&s).map_err(|_| String::from("invalid key"))?);
    }

    Ok(result)
}
