use std::collections::HashMap;

pub type Keybindings = HashMap<raylib::consts::KeyboardKey, Key>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

#[derive(Debug, Clone)]
pub struct Keyboard {
    keybindings: Keybindings,
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new(swap_yz: bool) -> Keyboard {
        let mut keybindings = HashMap::new();
        keybindings.insert(raylib::consts::KeyboardKey::KEY_ONE, Key::Key1);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_TWO, Key::Key2);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_THREE, Key::Key3);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_FOUR, Key::KeyC);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_Q, Key::Key4);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_W, Key::Key5);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_E, Key::Key6);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_R, Key::KeyD);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_A, Key::Key7);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_S, Key::Key8);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_D, Key::Key9);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_F, Key::KeyE);
        if swap_yz {
            keybindings.insert(raylib::consts::KeyboardKey::KEY_Y, Key::KeyA);
        } else {
            keybindings.insert(raylib::consts::KeyboardKey::KEY_Z, Key::KeyA);
        }
        keybindings.insert(raylib::consts::KeyboardKey::KEY_X, Key::Key0);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_C, Key::KeyB);
        keybindings.insert(raylib::consts::KeyboardKey::KEY_V, Key::KeyF);

        Keyboard {
            keybindings,
            keys: [false; 16],
        }
    }

    pub fn new_with_keybindings(keybindings: Keybindings) -> Keyboard {
        Keyboard {
            keybindings,
            keys: [false; 16],
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn press_key(&mut self, key: raylib::consts::KeyboardKey) {
        if let Some(chip8_key) = self.keybindings.get(&key) {
            match chip8_key {
                Key::Key0 => self.keys[0] = true,
                Key::Key1 => self.keys[1] = true,
                Key::Key2 => self.keys[2] = true,
                Key::Key3 => self.keys[3] = true,
                Key::Key4 => self.keys[4] = true,
                Key::Key5 => self.keys[5] = true,
                Key::Key6 => self.keys[6] = true,
                Key::Key7 => self.keys[7] = true,
                Key::Key8 => self.keys[8] = true,
                Key::Key9 => self.keys[9] = true,
                Key::KeyA => self.keys[10] = true,
                Key::KeyB => self.keys[11] = true,
                Key::KeyC => self.keys[12] = true,
                Key::KeyD => self.keys[13] = true,
                Key::KeyE => self.keys[14] = true,
                Key::KeyF => self.keys[15] = true,
            }
        }
    }

    pub fn release_key(&mut self, key: raylib::consts::KeyboardKey) {
        if let Some(chip8_key) = self.keybindings.get(&key) {
            match chip8_key {
                Key::Key0 => self.keys[0] = false,
                Key::Key1 => self.keys[1] = false,
                Key::Key2 => self.keys[2] = false,
                Key::Key3 => self.keys[3] = false,
                Key::Key4 => self.keys[4] = false,
                Key::Key5 => self.keys[5] = false,
                Key::Key6 => self.keys[6] = false,
                Key::Key7 => self.keys[7] = false,
                Key::Key8 => self.keys[8] = false,
                Key::Key9 => self.keys[9] = false,
                Key::KeyA => self.keys[10] = false,
                Key::KeyB => self.keys[11] = false,
                Key::KeyC => self.keys[12] = false,
                Key::KeyD => self.keys[13] = false,
                Key::KeyE => self.keys[14] = false,
                Key::KeyF => self.keys[15] = false,
            }
        }
    }

    pub fn get_raylib_keys(&self) -> Vec<raylib::consts::KeyboardKey> {
        self.keybindings.keys().cloned().collect()
    }
}
