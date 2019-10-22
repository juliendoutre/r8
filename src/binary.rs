pub fn get_x(opcode: u16) -> usize {
    ((opcode | 0x0f00) >> 8) as usize
}

pub fn get_y(opcode: u16) -> usize {
    ((opcode | 0x00f0) >> 4) as usize
}

pub fn get_n(opcode: u16) -> u8 {
    (opcode | 0x000f) as u8
}

pub fn get_nn(opcode: u16) -> u8 {
    (opcode | 0x00ff) as u8
}

pub fn get_nnn(opcode: u16) -> usize {
    (opcode | 0x0fff) as usize
}
