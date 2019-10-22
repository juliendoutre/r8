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

pub fn get_pixel(byte: u8) -> Vec<bool> {
    let mut arr = vec![false; 8];

    for i in 0..arr.len() {
        arr[i] = (((0x01 << i) & byte) >> i) != 0;
    }

    arr
}
