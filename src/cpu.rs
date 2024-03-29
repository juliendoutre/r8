use crate::{binary, display::Screen, inputs::Controller, stack::Stack, time::Counter};
use rand::random;
use std::fs::File;
use std::io::Read;
use std::num;
use std::thread;
use std::time::Duration;

pub const MEMORY_LENGTH: usize = 4096;
pub const REGISTERS_NUMBER: usize = 16;
pub const PROGRAM_START: usize = 512;

pub const FONTSET_START: usize = 0;
pub const FONTSET_END: usize = 80;
pub const FONTSET: &[u8; 80] = &[
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
    stack: Stack,
    delay_timer: Counter,
    sound_timer: Counter,
    pub screen: Screen,
    sprite_line_buffer: [bool; 8],
    pub inputs: Controller,
    cycle_duration: Duration,
}

impl Cpu {
    pub fn new(frequency: u64) -> Cpu {
        let mut cpu = Cpu {
            memory: [0; MEMORY_LENGTH],
            registers: [0; REGISTERS_NUMBER],
            i: 0,
            pc: PROGRAM_START,
            stack: Stack::new(),
            delay_timer: Counter::new(),
            sound_timer: Counter::new(),
            screen: Screen::new(),
            sprite_line_buffer: [false; 8],
            inputs: Controller::new(),
            cycle_duration: Duration::from_millis(1000 / frequency),
        };

        cpu.memory[FONTSET_START..FONTSET_END].copy_from_slice(FONTSET);

        cpu
    }

    pub fn load(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let _ = file.read(&mut self.memory[PROGRAM_START..]).unwrap();
    }

    pub fn emulate(&mut self) {
        if self.pc > self.memory.len() - 2 {
            panic!("memory overflow");
        }

        let opcode: u16 = ((self.memory[self.pc]) as u16) << 8 | self.memory[self.pc + 1] as u16;

        match (opcode & 0xf000) >> 12 {
            0x0 => match opcode & 0x0fff {
                0x0e0 => {
                    self.screen.clear();
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
                    self.registers[x] >>= 1;
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
                    self.registers[x] <<= 1;
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
                    binary::parse_sprite_line(
                        self.memory[self.i + i],
                        &mut self.sprite_line_buffer,
                    );

                    for (j, col) in self.sprite_line_buffer.iter().enumerate().take(8) {
                        let pixel = self.screen.pixel(x + j, y + i);

                        if *col {
                            if pixel {
                                self.registers[15] = 1;
                            }

                            self.screen.flip(x + j, y + i);
                        }
                    }
                }

                self.pc += 2;
            }
            0xe => match opcode & 0x00ff {
                0x9e => {
                    if self
                        .inputs
                        .is_active(self.registers[binary::get_x(opcode)] as usize)
                    {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0xa1 => {
                    if !self
                        .inputs
                        .is_active(self.registers[binary::get_x(opcode)] as usize)
                    {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            0xf => match opcode & 0x00ff {
                0x07 => {
                    self.registers[binary::get_x(opcode)] = self.delay_timer.get();
                    self.pc += 2;
                }
                0x0a => {
                    if let Some(key) = self.inputs.any() {
                        self.registers[binary::get_x(opcode)] = key as u8;
                        self.pc += 2;
                    }
                }
                0x15 => {
                    self.delay_timer.set(self.registers[binary::get_x(opcode)]);
                    self.pc += 2;
                }
                0x18 => {
                    self.sound_timer.set(self.registers[binary::get_x(opcode)]);
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
                    self.memory[self.i..self.i + x + 1].copy_from_slice(&self.registers[..x + 1]);
                    self.pc += 2;
                }
                0x65 => {
                    let x = binary::get_x(opcode);
                    self.registers[..x + 1].copy_from_slice(&self.memory[self.i..self.i + x + 1]);
                    self.pc += 2;
                }
                _ => panic!("unknow opcode {}", opcode),
            },
            _ => panic!("unknow opcode {}", opcode),
        }

        self.delay_timer.count_down();
        self.sound_timer.count_down();

        thread::sleep(self.cycle_duration);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new(4 * 60)
    }
}
