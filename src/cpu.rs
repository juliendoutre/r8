use crate::{binary, stack};
use piston_window::keyboard;
use piston_window::Button::Keyboard;
use rand::prelude::*;
use std::fs;
use std::io::prelude::*;
use std::num;
use std::thread;
use std::time;

pub const MEMORY_LENGTH: usize = 4096;
pub const REGISTERS_NUMBER: usize = 16;
pub const PROGRAM_START: usize = 512;
pub const FONTSET_START: usize = 0;
pub const FONTSET_END: usize = 80;
const FONTSET: &[u8; 80] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const KEYS_NUMBER: usize = 16;
pub const CYCLE_FREQUENCY: u64 = 60;
pub const CYCLE_DURATION: time::Duration = time::Duration::from_millis(1000 / CYCLE_FREQUENCY);

pub struct Cpu {
    memory: [u8; MEMORY_LENGTH],
    registers: [u8; REGISTERS_NUMBER],
    i: usize,
    pc: usize,
    stack: stack::Stack,
    delay_timer: u8,
    sound_timer: u8,
    pub screen: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],
    keys: [bool; KEYS_NUMBER],
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            memory: [0; MEMORY_LENGTH],
            registers: [0; REGISTERS_NUMBER],
            i: 0,
            pc: PROGRAM_START,
            stack: stack::Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            screen: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
            keys: [false; KEYS_NUMBER],
        };

        &cpu.memory[FONTSET_START..FONTSET_END].copy_from_slice(FONTSET);

        cpu
    }

    pub fn load(&mut self, path: &str) {
        let mut file = fs::File::open(path).unwrap();
        let program_size = file.read(&mut self.memory[PROGRAM_START..]).unwrap();
        println!("{} bytes loaded into memory", program_size);
    }

    pub fn press_key(&mut self, args: &piston_window::Button) {
        match *args {
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

    pub fn release_key(&mut self, args: &piston_window::Button) {
        match *args {
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

    pub fn emulate(&mut self) {
        if self.pc > self.memory.len() - 2 {
            panic!("memory overflow");
        }

        let opcode: u16 = ((self.memory[self.pc]) as u16) << 8 | self.memory[self.pc + 1] as u16;

        match (opcode & 0xf000) >> 12 {
            0x0 => match opcode & 0x0fff {
                0x0e0 => {
                    for i in 0..SCREEN_WIDTH {
                        for j in 0..SCREEN_HEIGHT {
                            self.screen[i][j] = false;
                        }
                    }

                    self.pc += 2;
                }
                0x0ee => {
                    self.pc = self.stack.pop() as usize;
                    self.pc += 2;
                }
                _ => {
                    // RCA1802 is not implemented
                    self.pc += 2;
                }
            },
            0x1 => {
                self.pc = binary::get_nnn(opcode);
            }
            0x2 => {
                self.stack.push(self.pc as u16);
                self.pc = binary::get_nnn(opcode);
            }
            0x3 => {
                if self.registers[binary::get_x(opcode)] == binary::get_nn(opcode) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.registers[binary::get_x(opcode)] != binary::get_nn(opcode) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => match opcode & 0x000f {
                0x0 => {
                    if self.registers[binary::get_x(opcode)]
                        == self.registers[binary::get_y(opcode)]
                    {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0x6 => {
                self.registers[binary::get_x(opcode)] = binary::get_nn(opcode);
                self.pc += 2;
            }
            0x7 => {
                self.registers[binary::get_x(opcode)] =
                    (num::Wrapping(self.registers[binary::get_x(opcode)])
                        + num::Wrapping(binary::get_nn(opcode)))
                    .0;
                self.pc += 2;
            }
            0x8 => match opcode & 0x000f {
                0x0 => {
                    self.registers[binary::get_x(opcode)] = self.registers[binary::get_y(opcode)];
                    self.pc += 2;
                }
                0x1 => {
                    self.registers[binary::get_x(opcode)] |= self.registers[binary::get_y(opcode)];
                    self.pc += 2;
                }
                0x2 => {
                    self.registers[binary::get_x(opcode)] &= self.registers[binary::get_y(opcode)];
                    self.pc += 2;
                }
                0x3 => {
                    self.registers[binary::get_x(opcode)] ^= self.registers[binary::get_y(opcode)];
                    self.pc += 2;
                }
                0x4 => {
                    let x = binary::get_x(opcode);
                    let y = binary::get_y(opcode);
                    if self.registers[x] > (0xff - self.registers[y]) {
                        self.registers[15] = 1;
                    } else {
                        self.registers[15] = 0;
                    }
                    self.registers[x] =
                        (num::Wrapping(self.registers[x]) + num::Wrapping(self.registers[y])).0;
                    self.pc += 2;
                }
                0x5 => {
                    let x = binary::get_x(opcode);
                    let y = binary::get_y(opcode);
                    if self.registers[y] > self.registers[x] {
                        self.registers[15] = 0;
                    } else {
                        self.registers[15] = 1;
                    }
                    self.registers[x] =
                        (num::Wrapping(self.registers[x]) - num::Wrapping(self.registers[y])).0;
                    self.pc += 2;
                }
                0x6 => {
                    let x = binary::get_x(opcode);
                    self.registers[15] = self.registers[x] & 0x01;
                    self.registers[x] = self.registers[x] >> 1;
                    self.pc += 2;
                }
                0x7 => {
                    let x = binary::get_x(opcode);
                    let y = binary::get_y(opcode);
                    if self.registers[x] > self.registers[y] {
                        self.registers[15] = 0;
                    } else {
                        self.registers[15] = 1;
                    }
                    self.registers[x] =
                        (num::Wrapping(self.registers[y]) - num::Wrapping(self.registers[x])).0;
                    self.pc += 2;
                }
                0xe => {
                    let x = binary::get_x(opcode);
                    self.registers[15] = self.registers[x] >> 7;
                    self.registers[x] = self.registers[x] << 1;
                    self.pc += 2;
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0x9 => match opcode & 0x000f {
                0 => {
                    if self.registers[binary::get_x(opcode)]
                        != self.registers[binary::get_y(opcode)]
                    {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0xa => {
                self.i = binary::get_nnn(opcode);
                self.pc += 2;
            }
            0xb => {
                self.pc = self.registers[0] as usize + binary::get_nnn(opcode);
            }
            0xc => {
                self.registers[binary::get_x(opcode)] = random::<u8>() & binary::get_nn(opcode);
                self.pc += 2;
            }
            0xd => {
                let n = binary::get_n(opcode);
                let x = self.registers[binary::get_x(opcode)] as usize;
                let y = self.registers[binary::get_y(opcode)] as usize;

                self.registers[15] = 0;

                for i in 0..n {
                    let sprite_line = binary::get_pixel(self.memory[self.i + i]);

                    for j in 0..8 {
                        let pixel = self.screen[(x + j) % SCREEN_WIDTH][(y + i) % SCREEN_HEIGHT];

                        if sprite_line[j] {
                            if pixel {
                                self.registers[15] = 1;
                            }

                            self.screen[(x + j) % SCREEN_WIDTH][(y + i) % SCREEN_HEIGHT] =
                                !self.screen[(x + j) % SCREEN_WIDTH][(y + i) % SCREEN_HEIGHT];
                        }
                    }
                }

                self.pc += 2;
            }

            0xe => match opcode & 0x00ff {
                0x9e => {
                    if self.keys[self.registers[binary::get_x(opcode)] as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0xa1 => {
                    if !self.keys[self.registers[binary::get_x(opcode)] as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0xf => match opcode & 0x00ff {
                0x07 => {
                    self.registers[binary::get_x(opcode)] = self.delay_timer;
                    self.pc += 2;
                }
                0x0a => {
                    // TODO: key is pressed
                    println!("waiting for a key to be pressed");
                    self.pc += 2;
                }
                0x15 => {
                    self.delay_timer = self.registers[binary::get_x(opcode)];
                    self.pc += 2;
                }
                0x18 => {
                    self.sound_timer = self.registers[binary::get_x(opcode)];
                    self.pc += 2;
                }
                0x1e => {
                    self.i += self.registers[binary::get_x(opcode)] as usize;
                    self.pc += 2;
                }
                0x29 => {
                    self.i = (self.registers[binary::get_x(opcode)] as usize % 16) * 5;
                    self.pc += 2;
                }
                0x33 => {
                    let x = binary::get_x(opcode);
                    self.memory[self.i] = self.registers[x] / 100;
                    self.memory[self.i + 1] = (self.registers[x] / 10) % 10;
                    self.memory[self.i + 2] = (self.registers[x] % 100) % 10;
                    self.pc += 2;
                }
                0x55 => {
                    let x = binary::get_x(opcode);
                    &self.memory[self.i..self.i + x + 1].copy_from_slice(&self.registers[..x + 1]);
                    self.pc += 2;
                }
                0x65 => {
                    let x = binary::get_x(opcode);
                    &self.registers[..x + 1].copy_from_slice(&self.memory[self.i..self.i + x + 1]);
                    self.pc += 2;
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            _ => panic!("unknow opcode {}", opcode),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        // thread::sleep(CYCLE_DURATION);
    }
}
