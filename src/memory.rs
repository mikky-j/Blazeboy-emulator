use rand::Rng;

pub struct Memory {
    pub data: [u8; Self::MEMORY_SIZE],
}

pub fn bus_read(memory: &Memory, address: u16) -> Option<u8> {
    return Some(memory.data[address as usize]);
}

pub fn bus_read_16bit_value(memory: &Memory, addr: u16) -> Option<u16> {
    let value =
        (bus_read(memory, addr.wrapping_add(1))? as u16) << 8 | bus_read(memory, addr)? as u16;
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

    pub fn set_random_number_at_addr(&mut self, data: u16) {
        let mut thread_rng = rand::thread_rng();

        let random_value = thread_rng.gen::<u8>();
        self.data[data as usize] = random_value;
    }
}
