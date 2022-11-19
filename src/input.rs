//Module Todo:
// Update module name to "keypad" name?

use sdl2::keyboard::Keycode;

use std::collections::HashSet;

pub struct Keypad {
    pub keypad: [bool; 16],
    pub key_pressed: bool,
    keys: HashSet<Keycode>,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keypad: [false; 16],
            key_pressed: false,
            keys: HashSet::new(),
        }
    }

    pub fn reset_keypad(&mut self) {
        self.keypad = [false; 16];
        self.key_pressed = false;
    }

    pub fn update_keys(&mut self, pressed_keys: HashSet<Keycode>) {
        self.keys = pressed_keys;
    }

    pub fn update_keypad(&mut self) {
        for key in &self.keys {
            // Match each key and return the Chip 8 hex value
            let hex_key: Option<usize> = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xC),
                Keycode::Q    => Some(0x4),
                Keycode::W    => Some(0x5),
                Keycode::E    => Some(0x6),
                Keycode::R    => Some(0xD),
                Keycode::A    => Some(0x7),
                Keycode::S    => Some(0x8),
                Keycode::D    => Some(0x9),
                Keycode::F    => Some(0xE),
                Keycode::Z    => Some(0xA),
                Keycode::X    => Some(0x0),
                Keycode::C    => Some(0xB),
                Keycode::V    => Some(0xF),
                _ => None,
            };
    
            // If valid key, set keypad[hexvalue] = true and key_pressed = true
            if let Some(i) = hex_key {
                self.keypad[i] = true;
                if !self.key_pressed {
                    self.key_pressed = true;
                }
            }
        }
    }
}