use crate::{
    cpu::{CpuRegisters, Instruction},
    Memory,
};
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
        if !self.halted {
            let mut instruction = Instruction::new();
            instruction.execute(&mut self.registers, memory, 0, crate::Command::ADC_8Bit);
            self.registers.pc += instruction.length as u16;
        }
    }
}
