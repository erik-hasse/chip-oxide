#[derive(Debug, Clone, Copy)]
pub struct Addr(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct Instruction(pub u8, pub u8, pub u8, pub u8);

impl Instruction {
    fn new(a: u8, b: u8, c: u8, d: u8) {
        if (a >= 0x10) | (b >= 0x10) | (c >= 0x10) | (d >= 0x10) {
            panic!(
                "Instruction values cannot be more than 4 bits: {} {} {} {}",
                a, b, c, d
            )
        }
    }

    fn as_int(&self) -> u16 {
        ((self.0 as u16) << 12)
            | ((self.1 as u16) << 8)
            | ((self.2 as u16) << 4)
            | (self.3 as u16)
    }

    pub fn from_int(inst: u16) -> Instruction {
        Instruction(
            ((inst >> 12) % 0x10) as u8,
            ((inst >> 8) % 0x10) as u8,
            ((inst >> 4) % 0x10) as u8,
            (inst % 0x10) as u8,
        )
    }
}

impl Addr {
    fn new(a: u16) -> Addr {
        if a >= 0x1000 {
            panic!("Address cannot be more than 12 bits: {}", a)
        }
        Addr(a)
    }

    pub fn from_nibbles(a: u8, b: u8, c: u8) -> Addr {
        if (a >= 0x10) | (b >= 0x10) | (c >= 0x10) {
            panic!(
                "Instruction values cannot be more than 4 bits: {} {} {}",
                a, b, c
            )
        }
        Addr(((a as u16) << 8) | ((b as u16) << 4) | (c as u16))
    }
}
