use color_eyre::eyre::ContextCompat;
use evdev::{Device, Key};

pub struct Devices {
    pub mouses: Vec<Device>,
    pub keyboards: Vec<Device>,
}

impl Devices {
    pub fn find(kb_deps: &[Key]) -> color_eyre::Result<Self> {
        let mut mouses = vec![];
        let mut keyboards = vec![];

        for (_, device) in evdev::enumerate() {
            if let Ok(sup) = device
                .supported_keys()
                .context("Failed to get supported keys")
            {
                if sup.contains(Key::BTN_LEFT) {
                    mouses.push(device);
                    continue;
                }

                let mut passed_count = 0;

                for key in kb_deps {
                    if sup.contains(*key) {
                        passed_count += 1;
                    }
                }

                if passed_count == kb_deps.len() {
                    keyboards.push(device);
                }
            }
        }
        Ok(Self { mouses, keyboards })
    }
}
