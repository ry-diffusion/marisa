use argh::FromArgs;

#[derive(FromArgs)]
/// Marisa is a click duplicater
pub struct Marisa {
    /// a device path, example: /dev/input/event4
    #[argh(option, default = "String::from(\"auto\")")]
    pub device_path: String,
}
