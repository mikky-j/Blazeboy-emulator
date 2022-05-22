use crate::{
    bus_read,
    cpu::{CpuRegisters, Instruction},
    Command, Memory,
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
        let mut instruction = Instruction::new();
        if !self.halted {
            let opcode = bus_read(&memory, self.registers.pc).unwrap();
            let command = Command::get_instruction(opcode);
            let (opcode, command) = match command {
                Command::CB => (
                    bus_read(&memory, self.registers.pc.wrapping_add(1)).unwrap(),
                    Command::get_instruction_cb(
                        bus_read(&memory, self.registers.pc.wrapping_add(1)).unwrap(),
                    ),
                ),
                _ => (opcode, command),
            };
            instruction.execute(&mut self.registers, memory, opcode, command);
            self.registers.pc = self.registers.pc.wrapping_add(instruction.length as u16);
        }
    }
}
