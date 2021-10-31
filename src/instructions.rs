use super::components::*;
use super::types::*;


fn cls(chip: &mut Chip) {
    chip.display = Display([[false; 64]; 32]);
    if chip.debug {
        println!("Display: {:?}", chip.display);
    }
}

fn ret(chip: &mut Chip) {
    chip.program_counter = chip.stack.pop();
    if chip.debug {
        println!("RET PC: {:?}", chip.program_counter);
        println!("    Stack: {:?}", chip.stack);
    }
}

fn jp(chip: &mut Chip, addr: Addr) {
    chip.program_counter = addr;
    if chip.debug {
        println!("JP PC: {:?}", chip.program_counter);
    }
}

fn call(chip: &mut Chip, addr: Addr) {
    chip.stack.push(chip.program_counter);
    chip.program_counter = addr;
    if chip.debug {
        println!("CALL PC: {:?}", chip.program_counter);
        println!("     Stack: {:?}", chip.stack);
    }
}

fn se_byte(chip: &mut Chip, rx: u8, kk: u8) {
    if chip.registers.0[rx as usize] == kk {
        chip.program_counter.0 += 2
    }
    if chip.debug {
        println!("SE byte PC: {:?}", chip.program_counter);
    }
}

fn sne(chip: &mut Chip, rx: u8, kk: u8) {
    if chip.registers.0[rx as usize] != kk {
        chip.program_counter.0 += 2
    }
    if chip.debug {
        println!("SNE PC: {:?}", chip.program_counter);
    }
}

fn se_reg(chip: &mut Chip, rx: u8, ry: u8) {
    if chip.registers.0[rx as usize] == chip.registers.0[ry as usize] {
        chip.program_counter.0 += 2
    }
    if chip.debug {
        println!("SE reg PC: {:?}", chip.program_counter);
    }
}

fn ld(chip: &mut Chip, rx: u8, kk: u8) {
    chip.registers.0[rx as usize] = kk;
    if chip.debug {
        println!("LD R{:?}: {:?}", rx, chip.registers.0[rx as usize]);
    }
}


pub fn call_instr(chip: &mut Chip, instruction: Instruction) {
    match instruction {
        Instruction(0, 0, 0xE, 0) => cls(chip),
        Instruction(0, 0, 0xE, 0xE) => ret(chip),
        Instruction(0x1, n1, n2, n3) => jp(chip, Addr::from_nibbles(n1, n2, n3)),
        Instruction(0x2, n1, n2, n3) => call(chip, Addr::from_nibbles(n1, n2, n3)),
        Instruction(0x3, x, k1, k2) => se_byte(chip, x, (k1 << 4) | k2),
        Instruction(0x4, x, k1, k2) => sne(chip, x, (k1 << 4) | k2),
        Instruction(0x5, x, y, 0) => se_reg(chip, x, y),
        Instruction(0x6, x, k1, k2) => ld(chip, x, (k1 << 4) | k2),
        Instruction(0, 0, 0, 0) => (),
        _ => panic!("Unknown instruction {:?}", instruction),
    };
}
