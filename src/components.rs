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
pub struct Stack{
    pub stack: [Addr; 0x10],
    pub pointer: u8
}

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
    pub debug: bool,
}

impl Memory {
    pub fn get_instruction(&self, addr: Addr) -> Instruction {
        Instruction::from_int(
            (self.0[addr.0 as usize] as u16) << 8
                | self.0[addr.0 as usize + 1] as u16,
        )
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [Addr(0x0); 0x10],
            pointer: 0x00
        }
    }
    pub fn push(&mut self, val: Addr) {
        self.pointer += 1;
        self.stack[self.pointer as usize - 1] = val;
    }
    
    pub fn pop(&mut self) -> Addr {
        let val = self.stack[self.pointer as usize - 1];
        self.pointer -= 1;
        val
    }
}
