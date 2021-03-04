use hidapi::{HidDevice, HidResult};
use fxhash::FxHashMap;
use std::error::Error;
use std::fmt::Display;
use std::iter::once;

const KEYBOARD_VENDOR_ID: u16 = 0x4B50;
const KEYBOARD_PRODUCT_ID: u16 = 0xEF8C;
const KEYBOARD_MAX_KEYS: u8 = 69;

pub struct Keyboard(HidDevice);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct KeyboardColor(u8);

impl From<u8> for KeyboardColor {
    fn from(rgb: u8) -> Self {
        KeyboardColor(rgb)
    }
}

pub const BLACK: KeyboardColor = KeyboardColor(0);
pub const BLUE: KeyboardColor = KeyboardColor(0b00000011);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct KeyboardKey(u8);

pub const T_KEY: KeyboardKey = KeyboardKey(19);

impl Keyboard {
    pub fn light_key(&self, index: KeyboardKey, color: KeyboardColor) -> HidResult<usize> {
        self.0.write(&[0, 0, index.0, color.0])
    }

    pub fn clear_all_keys(&self) -> HidResult<usize> {
        self.0.write(&[0, 2])
    }

    pub fn light_keys(&self, mut iterator: impl ExactSizeIterator<Item = (KeyboardKey, KeyboardColor)>) -> HidResult<()> {
        fn two_item_iter(pair: (KeyboardKey, KeyboardColor)) -> impl Iterator<Item = u8> {
            once(pair.0.0).chain(once(pair.1.0))
        }

        match iterator.size_hint() {
            (lower, Some(upper)) if lower == upper && lower == 1 => {
                if let Some((key, value)) = iterator.next() {
                    self.light_key(key, value)?;
                }
            }
            (lower, Some(upper)) if lower == upper && lower <= 14 => {
                let prefix = [0, 1, lower as u8];
                let data = prefix.iter()
                    .copied()
                    .chain(iterator.flat_map(two_item_iter))
                    .collect::<Vec<_>>();
                self.0.write(&data)?;
            },
            _ => {
                for (key, color) in iterator {
                    self.light_key(key, color)?;
                }
            }
        }

        Ok(())
    }

    pub fn new() -> Result<Keyboard, Box<dyn Error>> {
        let hid_api = hidapi::HidApi::new()?;
        let second_path = hid_api.device_list()
            .filter(|x| x.vendor_id() == KEYBOARD_VENDOR_ID && x.product_id() == KEYBOARD_PRODUCT_ID)
            .nth(1).ok_or(NoDeviceFoundError)?
            .path();
        let device = hid_api.open_path(second_path)?;

        Ok(Keyboard(device))
    }
}

#[derive(Debug)]
pub struct NoDeviceFoundError;

impl Display for NoDeviceFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No correct device found on 2nd path")
    }
}

impl Error for NoDeviceFoundError {

}

#[derive(Debug)]
pub struct KeyboardState {
    snapshot: Vec<KeyboardColor>,
    updates: FxHashMap<KeyboardKey, KeyboardColor>
}

impl Default for KeyboardState {
    fn default() -> Self {
        KeyboardState {
            snapshot: vec![BLACK; KEYBOARD_MAX_KEYS as usize],
            updates: FxHashMap::default()
        }
    }
}

impl KeyboardState {
    pub fn set(&mut self, key: KeyboardKey, color: KeyboardColor) {
        let value = self.snapshot[key.0 as usize];
        if value != color {
            self.snapshot[key.0 as usize] = color;
            self.updates.insert(key, color);
        }
    }

    pub fn get_updates<'a>(&'a self) -> impl ExactSizeIterator<Item = (KeyboardKey, KeyboardColor)> + 'a {
        self.updates.iter().map(|(k, v)| (*k, *v))
    }

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }
}
