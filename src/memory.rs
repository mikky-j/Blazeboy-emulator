use rand::Rng;

use crate::{get_bit, rom::Catridge};

pub enum RomMode {
    Simple,
    Advanced,
}
pub struct Memory {
    pub data: [u8; Self::MEMORY_SIZE],
    rom_bank_number: u16,
    ram_bank_number: u16,
    ram_access: bool,
    rom_mode: RomMode,
    catridge: Catridge,
}

pub fn bus_read(memory: &Memory, address: u16) -> Option<u8> {
    match address {
        0x0..=0x3FFF => match memory.rom_mode {
            RomMode::Advanced => {
                let effective_address = ((memory.ram_bank_number as usize) & 0x60) << 19
                    | (address as usize) & ((1 << 14) - 1);
                return Some(memory.catridge.data[effective_address]);
            }
            _ => Some(0),
        },
        0x4000..=0x7000 => {
            let new_address = address - 0x4000;
            let effective_address =
                (new_address) as usize + (memory.rom_bank_number as usize * 0x4000);
            return Some(memory.catridge.data[effective_address]);
        }
        0xA000..=0xBFFF => match memory.rom_mode {
            RomMode::Advanced => {
                let effective_address = ((memory.rom_bank_number as usize) >> 5) << 13
                    | ((address as usize) & ((1 << 13) - 1));
                return Some(memory.catridge.data[effective_address]);
            }
            _ => Some(0),
        },
        _ => Some(memory.data[address as usize]),
    }
}

pub fn bus_read_16bit_value(memory: &Memory, addr: u16) -> Option<u16> {
    let value =
        (bus_read(memory, addr.wrapping_add(1))? as u16) << 8 | bus_read(memory, addr)? as u16;
    Some(value)
}

pub fn bus_write(memory: &mut Memory, address: u16, data: u8) {
    match address {
        0x0..=0x1FFF => {
            if data & 0xF == 0b1010 {
                memory.ram_access = true;
            } else {
                memory.ram_access = false;
            }
        }
        0x2000..=0x3FFF => {
            let data = if data & 0x1F == 0 { data + 1 } else { data };
            memory.rom_bank_number = memory.rom_bank_number & 0x60 | (data & 0x1F) as u16;
        }
        0x4000..=0x5FFF => {
            let data = data & 0x3;
            match memory.rom_mode {
                RomMode::Simple => memory.ram_bank_number = data as u16,
                RomMode::Advanced => {
                    memory.rom_bank_number = (data as u16) << 5 | memory.rom_bank_number & 0x1F
                }
            }
        }
        0x6000..=0x7FFF => {
            memory.rom_mode = if get_bit(data, 0) == 1 {
                RomMode::Advanced
            } else {
                RomMode::Simple
            };
        }
        0xA000..=0xBFFF => {
            // let offset = address.wrapping_sub(0xA000);
            // let effective_address = (memory.ram_bank_number * 0x4000) + offset;
            if memory
                .catridge
                .catridge_type
                .contains(&crate::rom::CatridgeType::Mbc1)
            {
                let effective_address =
                    (memory.ram_bank_number as usize) << 13 | (address & ((1 << 14) - 1)) as usize;
                memory.catridge.ram[effective_address] = data;
            } else if memory
                .catridge
                .catridge_type
                .contains(&crate::rom::CatridgeType::Mbc2)
            {
                let effective_address = address & 0xFF;
                memory.catridge.ram[effective_address as usize] = data & 0xF;
            } else {
                memory.data[address as usize] = data; 
            }
        }

        _ => memory.data[address as usize] = data,
    }
}

impl Memory {
    pub const MEMORY_SIZE: usize = 65536;

    pub fn new() -> Memory {
        let rom = Catridge::new_empty();
        Memory {
            data: [0u8; Self::MEMORY_SIZE],
            catridge: rom,
            rom_mode: RomMode::Simple,
            ram_bank_number: 0,
            rom_bank_number: 0,
            ram_access: false,
        }
    }

    pub fn new_random_values() -> Memory {
        let mut data = [0u8; Self::MEMORY_SIZE];
        let mut thread_rng = rand::thread_rng();
        for i in 0..Self::MEMORY_SIZE {
            data[i] = thread_rng.gen::<u8>();
        }
        let rom = Catridge::new_empty();
        Memory {
            data,
            catridge: rom,
            rom_mode: RomMode::Simple,
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_access: false,
        }
    }

    pub fn load_section(&mut self, start: usize, data: &[u8]) {
        for i in start..data.len() {
            self.data[i] = data[i];
        }
    }

    pub fn set_random_number_at_addr(&mut self, data: u16) {
        let mut thread_rng = rand::thread_rng();

        let random_value = thread_rng.gen::<u8>();
        self.data[data as usize] = random_value;
    }

    pub fn check(&mut self, addr: u16, data: u8) -> Option<()> {
        Some(())
    }
}
