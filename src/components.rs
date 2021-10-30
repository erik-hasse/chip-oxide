use super::types::{Addr, Instruction};

#[derive(Debug)]
pub struct Memory(pub [u8; 0xFFF]);
#[derive(Debug)]
pub struct Registers(pub [u8; 0xF]);
#[derive(Debug)]
pub struct RegisterI(pub u16);
#[derive(Debug)]
pub struct RegisterVF(pub bool);
#[derive(Debug)]
pub struct RegisterDelay(pub u8);
#[derive(Debug)]
pub struct RegisterSound(pub u8);
#[derive(Debug)]
pub struct Display(pub [[bool; 0x40]; 0x20]);
#[derive(Debug)]
pub struct StackPointer(pub u8);
#[derive(Debug)]
pub struct Stack(pub [Addr; 0x10]);

#[derive(Debug)]
pub struct Chip {
    pub memory: Memory,
    pub registers: Registers,
    pub register_i: RegisterI,
    pub register_vf: RegisterVF,
    pub register_delay: RegisterDelay,
    pub register_sound: RegisterSound,
    pub display: Display,
    pub program_counter: Addr,
    pub stack_pointer: StackPointer,
    pub stack: Stack,
}

impl Memory {
    pub fn get_instruction(&self, addr: Addr) -> Instruction {
        Instruction::from_int(
            (self.0[addr.0 as usize] as u16) << 8
                | self.0[addr.0 as usize + 1] as u16,
        )
    }
}
