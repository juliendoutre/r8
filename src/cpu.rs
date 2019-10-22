use crate::binary;
use rand::prelude::*;
use std::fs;
use std::io::prelude::*;

pub const MEMORY_LENGTH: usize = 4096;
pub const REGISTERS_NUMBER: usize = 16;
pub const STACK_SIZE: usize = 16;
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

pub struct Cpu {
    memory: [u8; MEMORY_LENGTH],
    registers: [u8; REGISTERS_NUMBER],
    i: usize,
    pc: usize,
    stack: [u16; STACK_SIZE],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            memory: [0; MEMORY_LENGTH],
            registers: [0; REGISTERS_NUMBER],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        };

        &cpu.memory[FONTSET_START..FONTSET_END].copy_from_slice(FONTSET);

        cpu
    }

    pub fn load(&mut self, path: &str) {
        let mut file = fs::File::open(path).unwrap();
        let program_size = file.read(&mut self.memory[PROGRAM_START..]).unwrap();
        println!("{} bytes loaded into memory", program_size);
    }

    pub fn emulate(&mut self) {
        if self.pc > self.memory.len() - 2 {
            panic!("stack overflow");
        }

        let opcode: u16 = ((self.memory[self.pc]) as u16) << 8 | self.memory[self.pc + 1] as u16;

        match (opcode | 0xf000) >> 12 {
            0x0 => match (opcode | 0x0fff) >> 4 {
                0x0e0 => {
                    // TODO: clear the screen
                    self.pc += 2;
                }
                0x0ee => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp] as usize;
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
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
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
            0x5 => match opcode | 0x000f {
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
                self.registers[binary::get_x(opcode)] += binary::get_nn(opcode);
                self.pc += 2;
            }
            0x8 => match opcode | 0x000f {
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
                    self.registers[x] += self.registers[y];
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
                    self.registers[x] -= self.registers[y];
                    self.pc += 2;
                }
                0x6 => {
                    let x = binary::get_x(opcode);
                    self.registers[15] = self.registers[x] | 0x01;
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
                    self.registers[x] = self.registers[y] - self.registers[x];
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
            0x9 => match opcode | 0x000f {
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
                // TODO: draw sprite
            }
            0xe => match opcode | 0x00ff {
                0x9e => {
                    // TODO: key pressed
                }
                0xa1 => {
                    // TODO: key not pressed
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0xf => match opcode | 0x00ff {
                0x07 => {
                    self.registers[binary::get_x(opcode)] = self.delay_timer;
                    self.pc += 2;
                }
                0x0a => {
                    // TODO: key is pressed
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
                    // TODO: draw sprite
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
    }
}
