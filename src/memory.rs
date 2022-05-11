use rand::Rng;

pub struct Memory {
    pub data: [u8; Self::MEMORY_SIZE],
}

pub fn bus_read(memory: &Memory, address: u16) -> Option<u8> {
    return Some(memory.data[address as usize]);
}

pub fn bus_read_16bit_value(memory: &mut Memory, address1: u16, address2: u16) -> Option<u16> {
    let value = (bus_read(memory, address2)? as u16) << 8 | bus_read(memory, address1)? as u16;
    Some(value)
}

pub fn bus_write(memory: &mut Memory, address: u16, data: u8) {
    memory.data[address as usize] = data;
}

impl Memory {
    pub const MEMORY_SIZE: usize = 65536;

    pub fn new() -> Memory {
        Memory {
            data: [0u8; Self::MEMORY_SIZE],
        }
    }

    pub fn new_random_values() -> Memory {
        let mut data = [0u8; Self::MEMORY_SIZE];
        let mut thread_rng = rand::thread_rng();
        for i in 0..Self::MEMORY_SIZE {
            data[i] = thread_rng.gen::<u8>();
        }
        Memory { data }
    }

    pub fn load_section(&mut self, start: usize, data: &[u8]) {
        for i in start..data.len() {
            self.data[i] = data[i];
        }
    }
}
