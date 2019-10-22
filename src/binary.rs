fn get_x(opcode: u16) -> u8 {
    ((opcode | 0x0f00) >> 8) as u8
}

fn get_y(opcode: u16) -> u8 {
    ((opcode | 0x00f0) >> 4) as u8
}

fn get_n(opcode: u16) -> u8 {
    (opcode | 0x000f) as u8
}

fn get_nn(opcode: u16) -> u8 {
    (opcode | 0x00ff) as u8
}

fn get_nnn(opcode: u16) -> u16 {
    opcode | 0x0fff
}
