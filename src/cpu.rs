use std::fs;
use std::io::prelude::*;

pub const MEMORY_LENGTH: usize = 4096;
pub const REGISTERS_NUMBER: usize = 16;
pub const STACK_SIZE: usize = 16;

pub struct Cpu {
    memory: [u8; MEMORY_LENGTH],
    registers: [u8; REGISTERS_NUMBER],
    I: usize,
    pc: usize,
    stack: [u16; STACK_SIZE],
    sp: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; MEMORY_LENGTH],
            registers: [0; REGISTERS_NUMBER],
            I: 0,
            pc: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn load(&mut self, path: &str) {
        let mut file = fs::File::open(path).unwrap();
        let program_size = file.read(&mut self.memory).unwrap();
        println!("{}", program_size);
    }
}
