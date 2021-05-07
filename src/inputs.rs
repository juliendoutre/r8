use piston_window::keyboard;
use piston_window::Button::Keyboard;

pub const KEYS_NUMBER: usize = 16;

pub struct Controller {
    keys: [bool; KEYS_NUMBER],
}

impl Controller {
    pub fn new() -> Self {
        Self {
            keys: [false; KEYS_NUMBER],
        }
    }

    pub fn press(&mut self, event: &piston_window::Button) {
        match *event {
            Keyboard(keyboard::Key::D1) => {
                self.keys[1] = true;
            }
            Keyboard(keyboard::Key::D2) => {
                self.keys[2] = true;
            }
            Keyboard(keyboard::Key::D3) => {
                self.keys[3] = true;
            }
            Keyboard(keyboard::Key::D4) => {
                self.keys[12] = true;
            }
            Keyboard(keyboard::Key::Q) => {
                self.keys[4] = true;
            }
            Keyboard(keyboard::Key::W) => {
                self.keys[5] = true;
            }
            Keyboard(keyboard::Key::E) => {
                self.keys[6] = true;
            }
            Keyboard(keyboard::Key::R) => {
                self.keys[13] = true;
            }
            Keyboard(keyboard::Key::A) => {
                self.keys[7] = true;
            }
            Keyboard(keyboard::Key::S) => {
                self.keys[8] = true;
            }
            Keyboard(keyboard::Key::D) => {
                self.keys[9] = true;
            }
            Keyboard(keyboard::Key::F) => {
                self.keys[14] = true;
            }
            Keyboard(keyboard::Key::Z) => {
                self.keys[10] = true;
            }
            Keyboard(keyboard::Key::X) => {
                self.keys[0] = true;
            }
            Keyboard(keyboard::Key::C) => {
                self.keys[11] = true;
            }
            Keyboard(keyboard::Key::V) => {
                self.keys[15] = true;
            }
            _ => {}
        }
    }

    pub fn release(&mut self, event: &piston_window::Button) {
        match *event {
            Keyboard(keyboard::Key::D1) => {
                self.keys[1] = false;
            }
            Keyboard(keyboard::Key::D2) => {
                self.keys[2] = false;
            }
            Keyboard(keyboard::Key::D3) => {
                self.keys[3] = false;
            }
            Keyboard(keyboard::Key::D4) => {
                self.keys[12] = false;
            }
            Keyboard(keyboard::Key::Q) => {
                self.keys[4] = false;
            }
            Keyboard(keyboard::Key::W) => {
                self.keys[5] = false;
            }
            Keyboard(keyboard::Key::E) => {
                self.keys[6] = false;
            }
            Keyboard(keyboard::Key::R) => {
                self.keys[13] = false;
            }
            Keyboard(keyboard::Key::A) => {
                self.keys[7] = false;
            }
            Keyboard(keyboard::Key::S) => {
                self.keys[8] = false;
            }
            Keyboard(keyboard::Key::D) => {
                self.keys[9] = false;
            }
            Keyboard(keyboard::Key::F) => {
                self.keys[14] = false;
            }
            Keyboard(keyboard::Key::Z) => {
                self.keys[10] = false;
            }
            Keyboard(keyboard::Key::X) => {
                self.keys[0] = false;
            }
            Keyboard(keyboard::Key::C) => {
                self.keys[11] = false;
            }
            Keyboard(keyboard::Key::V) => {
                self.keys[15] = false;
            }
            _ => {}
        }
    }

    pub fn is_active(&self, idx: usize) -> bool {
        self.keys[idx]
    }

    pub fn any(&self) -> Option<usize> {
        self.keys.iter().position(|key| *key)
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
