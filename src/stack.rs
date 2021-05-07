pub const STACK_SIZE: usize = 16;

pub struct Stack {
    array: [u16; STACK_SIZE],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            array: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, e: u16) {
        if self.sp < STACK_SIZE {
            self.array[self.sp] = e;
            self.sp += 1;
        } else {
            panic!("stack overflow");
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp > 0 {
            self.sp -= 1;
            self.array[self.sp]
        } else {
            panic!("stack underflow");
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn stack_underflow() {
        let mut s = Stack::new();
        s.pop();
    }

    #[test]
    fn stack_push() {
        let mut s = Stack::new();
        s.push(1);
        assert_eq!(s.array[0], 1);
        assert_eq!(s.sp, 1);
    }

    #[test]
    #[should_panic]
    fn stack_overflow() {
        let mut s = Stack::new();
        for _ in 0..STACK_SIZE + 1 {
            s.push(1);
        }
    }

    #[test]
    fn test_pop() {
        let mut s = Stack::new();
        s.push(1);

        let e = s.pop();
        assert_eq!(e, 1);
        assert_eq!(s.sp, 0);
    }
}
