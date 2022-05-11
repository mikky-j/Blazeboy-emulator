mod cpu;
mod instruction;
mod memory;
mod register;
pub use crate::cpu::Cpu;
pub use crate::instruction::{BitwiseOperator, Instruction};
pub use crate::memory::{bus_read, bus_write, Memory};
pub use crate::register::{CpuRegisters, Flag, Register16Bit, Register8Bit};

pub fn get_bit(data: u8, pos: u8) -> u8 {
    (data >> pos) & 1
}
