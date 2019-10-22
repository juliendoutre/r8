use crate::binary;
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
    I: usize,
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
            I: 0,
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
                    // TODO: return
                    self.pc += 2;
                }
                _ => {
                    // RCA1802 is not implemented
                    self.pc += 2;
                }
            },
            0x1 => {
                self.pc = binary::get_nnn(opcode) as usize;
            }
            0x2 => {
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = binary::get_nnn(opcode) as usize;
            }
            0x3 => {
                if self.registers[binary::get_x(opcode) as usize] == binary::get_nn(opcode) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.registers[binary::get_x(opcode) as usize] != binary::get_nn(opcode) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => match opcode | 0x000f {
                0x0 => {
                    if self.registers[binary::get_x(opcode) as usize]
                        == self.registers[binary::get_y(opcode) as usize]
                    {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0x6 => {
                self.registers[binary::get_x(opcode) as usize] = binary::get_nn(opcode);
                self.pc += 2;
            }
            0x7 => {
                self.registers[binary::get_x(opcode) as usize] += binary::get_nn(opcode);
                self.pc += 2;
            }
            0x8 => match opcode | 0x000f {
                0x0 => {}
                0x1 => {}
                0x2 => {}
                0x3 => {}
                0x4 => {}
                0x5 => {}
                0x6 => {}
                0x7 => {}
                0xe => {}
                _ => panic!("unknow opcode {}", opcode),
            },
            0x9 => {}
            0xa => {}
            0xb => {}
            0xc => {}
            0xd => {}
            0xe => match opcode | 0x00ff {
                0x9e => {}
                0xa1 => {}
                _ => panic!("unknow opcode {}", opcode),
            },
            0xf => match opcode | 0x00ff {
                0x07 => {}
                0x0a => {}
                0x15 => {}
                0x18 => {}
                0x1e => {}
                0x29 => {}
                0x33 => {}
                0x55 => {}
                0x65 => {}
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
