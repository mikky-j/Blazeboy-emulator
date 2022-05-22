mod cpu;
mod memory;
pub use crate::memory::{bus_read, bus_write, Memory};
pub use cpu::*;

pub fn get_bit(data: u8, pos: u8) -> u8 {
    (data >> pos) & 1
}
