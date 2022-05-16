use rand::Rng;

pub struct CpuRegisters {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Clone, Copy)]
pub enum Flag {
    Carry(bool),
    Zero(bool),
    Subtraction(bool),
    HalfCarry(bool),
    None,
}
#[derive(Clone, Copy)]
pub enum Register8Bit {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Register16Bit {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
    None,
}

impl CpuRegisters {
    pub fn new() -> Self {
        CpuRegisters {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn set_random_number_reg(&mut self, reg: Register8Bit) {
        let mut thread_gen = rand::thread_rng();
        let random_value = thread_gen.gen::<u8>();
        match reg {
            Register8Bit::A => self.a = random_value,
            Register8Bit::B => self.b = random_value,
            Register8Bit::C => self.c = random_value,
            Register8Bit::D => self.d = random_value,
            Register8Bit::E => self.e = random_value,
            Register8Bit::F => self.f = random_value,
            Register8Bit::H => self.h = random_value,
            Register8Bit::L => self.l = random_value,
            _ => (),
        }
    }
    pub fn clear_reg(&mut self, reg: Register8Bit) {
        let value = 0;
        match reg {
            Register8Bit::A => self.a = value,
            Register8Bit::B => self.b = value,
            Register8Bit::C => self.c = value,
            Register8Bit::D => self.d = value,
            Register8Bit::E => self.e = value,
            Register8Bit::F => self.f = value,
            Register8Bit::H => self.h = value,
            Register8Bit::L => self.l = value,
            _ => (),
        }
    }

    pub fn set_random_number_reg_16bit(&mut self, reg: Register16Bit) {
        match reg {
            Register16Bit::HL => {
                self.set_random_number_reg(Register8Bit::H);
                self.set_random_number_reg(Register8Bit::L);
            }
            Register16Bit::AF => {
                self.set_random_number_reg(Register8Bit::A);
                self.set_random_number_reg(Register8Bit::F);
            }
            Register16Bit::BC => {
                self.set_random_number_reg(Register8Bit::B);
                self.set_random_number_reg(Register8Bit::C);
            }
            Register16Bit::DE => {
                self.set_random_number_reg(Register8Bit::D);
                self.set_random_number_reg(Register8Bit::E);
            }
            Register16Bit::SP => {
                let mut rand_thread = rand::thread_rng();
                self.sp = rand_thread.gen::<u16>();
            }
            Register16Bit::PC => {
                let mut rand_thread = rand::thread_rng();
                self.pc = rand_thread.gen::<u16>();
            }
            _ => (),
        }
    }
    pub fn ld_8bit_reg(&mut self, reg: Register8Bit, data: u8) {
        match reg {
            Register8Bit::A => self.a = data,
            Register8Bit::B => self.b = data,
            Register8Bit::C => self.c = data,
            Register8Bit::D => self.d = data,
            Register8Bit::E => self.e = data,
            Register8Bit::F => self.f = data,
            Register8Bit::H => self.h = data,
            Register8Bit::L => self.l = data,
            _ => (),
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        let new_val = match flag {
            Flag::Carry(v) => {
                if v {
                    self.f | (1 << 4)
                } else {
                    self.f & (0xF0 ^ (1 << 4))
                }
            }
            Flag::HalfCarry(v) => {
                if v {
                    self.f | (1 << 5)
                } else {
                    self.f & (0xF0 ^ (1 << 5))
                }
            }
            Flag::Subtraction(v) => {
                if v {
                    self.f | (1 << 6)
                } else {
                    self.f & (0xF0 ^ (1 << 6))
                }
            }
            Flag::Zero(v) => {
                if v {
                    self.f | (1 << 7)
                } else {
                    self.f & (0xF0 ^ (1 << 7))
                }
            }
            _ => self.f,
        };
        self.f = new_val;
    }

    pub fn get_16bit_reg_value(&self, register: Register16Bit) -> u16 {
        let result = match register {
            Register16Bit::AF => (self.a as u16) << 8 | (self.f as u16),
            Register16Bit::BC => (self.b as u16) << 8 | (self.c as u16),
            Register16Bit::DE => (self.d as u16) << 8 | (self.e as u16),
            Register16Bit::HL => (self.h as u16) << 8 | (self.l as u16),
            Register16Bit::SP => self.sp,
            Register16Bit::PC => self.pc,
            _ => 0,
        };
        result
    }

    pub fn ld_16bit_reg(&mut self, register: Register16Bit, data: u16) {
        let values = [(data >> 8) as u8, (data & 0xff) as u8];
        match register {
            Register16Bit::AF => [self.a, self.f] = values,
            Register16Bit::BC => [self.b, self.c] = values,
            Register16Bit::DE => [self.d, self.e] = values,
            Register16Bit::HL => [self.h, self.l] = values,
            Register16Bit::SP => self.sp = data,
            Register16Bit::PC => self.pc = data,
            _ => (),
        }
    }

    pub fn get_8bit_reg_value(&self, register: Register8Bit) -> u8 {
        let result = match register {
            Register8Bit::A => self.a,
            Register8Bit::B => self.b,
            Register8Bit::C => self.c,
            Register8Bit::D => self.d,
            Register8Bit::E => self.e,
            Register8Bit::F => self.f,
            Register8Bit::H => self.h,
            Register8Bit::L => self.l,
            _ => 0,
        };
        result
    }
    pub fn get_mut_8bit_reg_value(&mut self, register: Register8Bit) -> &mut u8 {
        let result = match register {
            Register8Bit::A => &mut self.a,
            Register8Bit::B => &mut self.b,
            Register8Bit::C => &mut self.c,
            Register8Bit::D => &mut self.d,
            Register8Bit::E => &mut self.e,
            Register8Bit::F => &mut self.f,
            Register8Bit::H => &mut self.h,
            Register8Bit::L => &mut self.l,
            _ => &mut self.a,
        };
        result
    }

    pub fn get_flag(&mut self, flag: Flag) -> bool {
        let res = match flag {
            Flag::Carry(_) => (self.f >> 4) & 1 == 1,
            Flag::HalfCarry(_) => (self.f >> 5) & 1 == 1,
            Flag::Subtraction(_) => (self.f >> 6) & 1 == 1,
            Flag::Zero(_) => (self.f >> 7) & 1 == 1,
            _ => false,
        };
        res
    }

    pub fn set_flags(&mut self, flags: &[Flag]) {
        for flag in flags {
            self.set_flag(*flag);
        }
    }
}
