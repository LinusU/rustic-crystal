use crate::{cpu::Cpu, game::constants::move_constants::Move};

/// Print the type of move `b` at `hl`
pub fn print_move_type(cpu: &mut Cpu) {
    cpu.b = Move::from(cpu.b).r#type().into();
    cpu.jump(0x4953); // PrintType
}
