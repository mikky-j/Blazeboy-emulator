use crate::{CpuRegisters, Instruction, Memory};
pub struct Cpu {
    pub registers: CpuRegisters,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let registers = CpuRegisters::new();
        let halted = false;

        Cpu { registers, halted }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let mut instruction = Instruction::new(&mut self.registers, memory);
        if !self.halted {
            instruction.set_pc();
            self.registers.pc += instruction.length as u16;
        }
    }
}
