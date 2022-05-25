mod cpu;
mod memory;
mod rom;
pub use crate::memory::{bus_read, bus_write, Memory};
pub use cpu::*;
pub use rom::Catridge;

pub fn get_bit(data: u8, pos: u8) -> u8 {
    (data >> pos) & 1
}
