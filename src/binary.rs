pub fn get_x(opcode: u16) -> usize {
    ((opcode & 0x0f00) >> 8) as usize
}

pub fn get_y(opcode: u16) -> usize {
    ((opcode & 0x00f0) >> 4) as usize
}

pub fn get_n(opcode: u16) -> usize {
    (opcode & 0x000f) as usize
}

pub fn get_nn(opcode: u16) -> u8 {
    (opcode & 0x00ff) as u8
}

pub fn get_nnn(opcode: u16) -> usize {
    (opcode & 0x0fff) as usize
}

pub fn parse_sprite_line(byte: u8, buffer: &mut [bool; 8]) {
    for i in 0..8 {
        buffer[7 - i] = (((0x01 << i) & byte) >> i) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_x() {
        assert_eq!(get_x(0xae45), 0x0e);
    }

    #[test]
    fn test_get_y() {
        assert_eq!(get_y(0xae45), 0x04);
    }

    #[test]
    fn test_get_n() {
        assert_eq!(get_n(0xae45), 0x05);
    }

    #[test]
    fn test_get_nn() {
        assert_eq!(get_nn(0xae45), 0x45);
    }

    #[test]
    fn test_get_nnn() {
        assert_eq!(get_nnn(0xae45), 0xe45);
    }

    #[test]
    fn test_get_pixel() {
        let mut buffer = [false; 8];
        parse_sprite_line(0b10101101, &mut buffer);

        assert_eq!(buffer, [true, false, true, false, true, true, false, true]);
    }
}
