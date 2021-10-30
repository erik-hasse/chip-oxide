use std::fs::read;
use std::path::Path;

use super::components::*;
use super::instructions::*;

fn init_chip(data: [u8; 0xFFF]) -> Chip {
    Chip {
        memory: Memory(data),
        registers: Registers([0x0; 0xf]),
        register_i: RegisterI(0x0),
        register_vf: RegisterVF(false),
        register_delay: RegisterDelay(0x0),
        register_sound: RegisterSound(0x0),
        display: Display([[true; 0x40]; 0x20]),
        program_counter: Addr(0x200),
        stack_pointer: StackPointer(0x0),
        stack: Stack([Addr(0x0); 0x10]),
    }
}

fn load_binary(path: &Path) -> [u8; 0xFFF] {
    let mut bytes = match read(path) {
        Err(why) => panic!("couldn't open binary: {}", why),
        Ok(file) => file,
    };
    bytes.resize(0xFFF, 0x0);
    bytes.try_into().unwrap()
}

pub fn run(path: &Path) {
    let mut chip = init_chip(load_binary(path));
    while chip.program_counter.0 < 0xFFE {
        let instruction = chip.memory.get_instruction(chip.program_counter);
        step(&mut chip, instruction);
    }
}

fn step(chip: &mut Chip, instruction: Instruction) {
    chip.program_counter.0 += 2;
    match instruction {
        Instruction(0x00E0) => cls(chip),
        Instruction(0x0000) => (),
        _ => panic!("Unknown instruction {:?}", instruction),
    };
}
