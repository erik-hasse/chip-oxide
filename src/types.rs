#[derive(Debug, Clone, Copy)]
pub struct Addr(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct Instruction(pub u8, pub u8, pub u8, pub u8);

impl Instruction {

    fn _as_int(&self) -> u16 {
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
    pub fn from_nibbles(a: u8, b: u8, c: u8) -> Addr {
        Addr(((a as u16) << 8) | ((b as u16) << 4) | (c as u16))
    }
}
