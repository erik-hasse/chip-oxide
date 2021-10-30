// use super::macros::instructions;
use super::components::*;
use super::types::*;

// instructions! {
//     "00E0" pub fn cls(chip: &mut Chip) {
//         chip.display = Display([[false; 64]; 32]);
//     }
//
//     "1nnn" pub fn jmp(chip: &mut Chip, addr: Addr) {
//         chip.program_counter = addr
//     }
// }

fn cls(chip: &mut Chip) {
    chip.display = Display([[false; 64]; 32]);
}

fn jmp(chip: &mut Chip, addr: Addr) {
    chip.program_counter = addr;
}

pub fn call_instr(chip: &mut Chip, instruction: Instruction) {
    match instruction {
        Instruction(0, 0, 0xE, 0) => cls(chip),
        Instruction(0x1, a, b, c) => jmp(chip, Addr::from_nibbles(a, b, c)),
        Instruction(0, 0, 0, 0) => (),
        _ => panic!("Unknown instruction {:?}", instruction),
    };
}
