use evdev::{Device, Key};

pub struct Devices {
    pub mouses: Vec<Device>,
    pub keyboards: Vec<Device>,
}

impl Devices {
    pub fn find(kb_deps: &[Key]) -> Self {
        let mut mouses = vec![];
        let mut keyboards = vec![];

        for (_, device) in evdev::enumerate() {
            if let Some(sup) = device.supported_keys() {
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

        Self { mouses, keyboards }
    }
}
