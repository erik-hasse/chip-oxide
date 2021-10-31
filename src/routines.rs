use std::fs::read;
use std::path::Path;

use super::components::*;
use super::instructions::*;
use super::types::*;

fn init_chip(data: [u8; 0xFFF], start: u16, verbose: bool) -> Chip {
    Chip {
        memory: Memory(data),
        registers: Registers([0x0; 0xf]),
        register_i: RegisterI(0x0),
        register_vf: RegisterVF(false),
        register_delay: RegisterDelay(0x0),
        register_sound: RegisterSound(0x0),
        display: Display([[true; 0x40]; 0x20]),
        program_counter: Addr(start),
        stack_pointer: StackPointer(0xFF),
        stack: Stack::new(),
        debug: verbose,
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

pub fn run(path: &Path, leading_zeros: bool, verbose: bool) {
    let start = match leading_zeros {
        true => 0x200,
        false => 0x000
    };
    let mut chip = init_chip(load_binary(path), start, verbose);
    println!("{:?}", chip.program_counter);
    while chip.program_counter.0 < 0xFFE {
        let instruction = chip.memory.get_instruction(chip.program_counter);
        println!("{:?}", instruction);
        step(&mut chip, instruction);
        println!("{:?}", chip.program_counter);
    }
}

fn step(chip: &mut Chip, instruction: Instruction) {
    chip.program_counter.0 += 2;
    call_instr(chip, instruction)
}
