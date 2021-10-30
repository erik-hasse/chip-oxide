use super::components::*;

// [#instruction("00E0")]
pub fn cls(chip: &mut Chip) {
    chip.display = Display([[false; 64]; 32]);
}
