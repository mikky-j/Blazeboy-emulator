use crate::memory::bus_read_16bit_value;
use crate::register::{Flag, Register16Bit, Register8Bit};
use crate::{bus_read, get_bit, Memory};
use crate::{bus_write, CpuRegisters};

#[derive(PartialEq)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
}

pub struct Instruction<'a, 'b> {
    pub registers: &'a mut CpuRegisters,
    pub memory: &'b mut Memory,
    pub length: u8,
    pub cycle: u8,
}

impl<'a, 'b> Instruction<'a, 'b> {
    // ---------------------START OF INSTRUCTIONS--------------------

    // ---------------------CPU CONTROL INSTRUCTIONS--------------------

    // NOP
    pub fn no_op(&mut self) {
        self.registers.pc += 1;
        self.length = 1;
        self.cycle = 4;
    }

    // CPL
    pub fn cpl(&mut self) {
        self.registers.a = self.registers.a ^ 0xFF;
        let flags = [Flag::Subtraction(true), Flag::HalfCarry(true)];
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // SCF
    pub fn scf(&mut self) {
        let flags = [
            Flag::Carry(true),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // CCF
    pub fn ccf(&mut self) {
        let flags = [
            Flag::Carry(get_bit(self.registers.f, 4) ^ 1 == 1),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // ---------------------JUMP INSTRUCTIONS--------------------

    // JP NZ, a16
    pub fn jp_not_eq(&mut self, flag: Flag, addr: u16) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                self.registers.pc = addr;
                (0, 16)
            }
            Flag::Zero(false) => {
                self.registers.pc = addr;
                (0, 16)
            }
            _ => (3, 12),
        };
    }

    // JP C, a16
    pub fn jp_eq(&mut self, flag: Flag, addr: u16) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                self.registers.pc = addr;
                (0, 16)
            }
            Flag::Zero(true) => {
                self.registers.pc = addr;
                (0, 16)
            }
            _ => (3, 12),
        };
    }

    // JP (HL)
    pub fn jp_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        self.registers.pc = addr;
        self.length = 0;
        self.cycle = 4;
    }

    // JP a16
    pub fn jp(&mut self, addr: u16) {
        self.registers.pc = addr;
        self.length = 0;
        self.cycle = 16;
    }

    // JR NZ, r8
    pub fn jr_not_eq(&mut self, flag: Flag, data: i8) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                self.registers.pc = ((self.registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            Flag::Zero(false) => {
                self.registers.pc = ((self.registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            _ => (2, 16),
        };
    }

    // JR Z, r8
    pub fn jr_eq(&mut self, flag: Flag, data: i8) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                self.registers.pc = ((self.registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            Flag::Zero(true) => {
                self.registers.pc = ((self.registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            _ => (2, 8),
        };
    }

    // JR r8
    pub fn jr(&mut self, data: i8) {
        self.registers.pc = ((self.registers.pc as i16) + (data as i16)) as u16;
        self.length = 0;
        self.cycle = 12;
    }

    // ---------------------CALL INSTRUCTIONS--------------------

    // CALL NZ, a16
    pub fn call_not_eq(&mut self, flag: Flag, addr: u16) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc >> 4) as u8,
                );
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc & 0xF) as u8,
                );
                self.registers.pc = addr;
                self.registers.sp -= 2;
                (0, 16)
            }
            Flag::Zero(false) => {
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc >> 4) as u8,
                );
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc & 0xF) as u8,
                );
                self.registers.pc = addr;
                self.registers.sp -= 2;
                (0, 16)
            }
            _ => (3, 24),
        };
    }

    // CALL Z, a16
    pub fn call_eq(&mut self, flag: Flag, addr: u16) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc >> 4) as u8,
                );
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc & 0xF) as u8,
                );
                self.registers.pc = addr;
                self.registers.sp -= 2;
                (0, 16)
            }
            Flag::Zero(true) => {
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc >> 4) as u8,
                );
                bus_write(
                    &mut self.memory,
                    self.registers.sp,
                    (self.registers.pc & 0xF) as u8,
                );
                self.registers.pc = addr;
                self.registers.sp -= 2;
                (0, 16)
            }
            _ => (3, 24),
        };
    }

    // CALL a16
    pub fn call(&mut self, addr: u16) {
        bus_write(
            &mut self.memory,
            self.registers.sp,
            (self.registers.pc >> 8) as u8,
        );
        bus_write(
            &mut self.memory,
            self.registers.sp,
            (self.registers.pc & 0xFF) as u8,
        );
        self.registers.pc = addr;
        self.registers.sp -= 2;
        self.cycle = 24;
    }

    // ---------------------RETURN INSTRUCTIONS--------------------

    // RET
    pub fn ret(&mut self) {
        let hi = bus_read(&self.memory, self.registers.sp + 1).unwrap() as u16;
        let lo = bus_read(&self.memory, self.registers.sp).unwrap() as u16;
        self.registers.pc = hi << 4 | lo;
        self.registers.sp += 2;
        self.length = 1;
        self.cycle = 16;
    }

    // RETI
    pub fn reti(&mut self) {
        let hi = bus_read(&self.memory, self.registers.sp + 1).unwrap() as u16;
        let lo = bus_read(&self.memory, self.registers.sp).unwrap() as u16;
        self.registers.pc = hi << 4 | lo;
        self.registers.sp += 2;
        self.length = 1;
        self.cycle = 16;
    }

    // ---------------------STACK INSTRUCTIONS--------------------

    // POP BC
    pub fn pop_16bit_reg(&mut self, reg: Register16Bit) {
        let hi = bus_read(&self.memory, self.registers.sp + 1).unwrap();
        let lo = bus_read(&self.memory, self.registers.sp).unwrap();
        let res = (hi as u16) << 8 | (lo as u16);
        self.registers.ld_16bit_reg(reg, res);
        self.registers.sp += 2;
        self.length = 1;
        self.cycle = 12;
    }

    // PUSH BC
    pub fn push_16bit_reg(&mut self, reg: Register16Bit) {
        let [hi, lo] = self.registers.get_16bit_reg_value(reg).to_be_bytes();
        bus_write(&mut self.memory, self.registers.sp - 2, lo);
        bus_write(&mut self.memory, self.registers.sp - 1, hi);
        self.registers.sp -= 2;
        self.length = 1;
        self.cycle = 16;
    }

    // ---------------------LOAD INSTRUCTIONS--------------------

    // RST
    pub fn rst(&mut self, addr: u16) {
        let ret_addr = addr & 0x38;
        let hi = (self.registers.pc >> 8) as u8;
        let lo = (self.registers.pc & 0xFF) as u8;
        bus_write(&mut self.memory, self.registers.sp - 1, hi);
        bus_write(&mut self.memory, self.registers.sp - 2, lo);
        self.registers.sp -= 2;
        self.registers.pc = ret_addr;
        self.length = 1;
        self.cycle = 16;
    }

    // ---------------------LOAD INSTRUCTIONS--------------------

    // LD A, d8
    pub fn ld_reg_8bit(&mut self, to: Register8Bit, data: u8) {
        self.registers.ld_8bit_reg(to, data);
        self.length = 2;
        self.cycle = 8;
    }

    // LD A, (HL+)
    pub fn ld_hli(&mut self) {
        self.ld_mem_reg_to_reg(Register8Bit::A, Register16Bit::HL);
        self.inc_16bit(Register16Bit::HL);
    }

    // LD A, (HL-)
    pub fn ld_hld(&mut self) {
        self.ld_mem_reg_to_reg(Register8Bit::A, Register16Bit::HL);
        self.dec_16bit(Register16Bit::HL);
    }

    // LD (HL+), A
    pub fn ld_mem_hli(&mut self) {
        self.ld_reg_to_mem_reg(Register8Bit::A, Register16Bit::HL);
        self.inc_16bit(Register16Bit::HL);
    }

    // LD (HL-), A
    pub fn ld_mem_hld(&mut self) {
        self.ld_reg_to_mem_reg(Register8Bit::A, Register16Bit::HL);
        self.dec_16bit(Register16Bit::HL);
    }

    // LD (HL), A
    pub fn ld_8bit_into_mem(&mut self, data: u8) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        bus_write(&mut self.memory, addr, data);
        self.length = 2;
        self.cycle = 12;
    }

    // LD (a16), SP
    pub fn ld_16bit_reg_to_mem(&mut self, address: u16) {
        bus_write(&mut self.memory, address, (self.registers.sp & 0xFF) as u8);
        bus_write(
            &mut self.memory,
            address + 1,
            (self.registers.sp >> 8) as u8,
        );
        self.length = 3;
        self.cycle = 20;
    }

    // LDH A, (a8)
    pub fn ld_addr_to_reg_8bit(&mut self, addr: u8) {
        let address = 0xFF00 + (addr as u16);
        let value = bus_read(&self.memory, address).unwrap();
        self.registers.a = value;
        self.length = 2;
        self.cycle = 12;
    }

    // LDH (a8), A
    pub fn ld_reg_8bit_to_addr(&mut self, addr: u8) {
        let address = 0xFF00 + (addr as u16);
        bus_write(&mut self.memory, address, self.registers.a);
        self.length = 2;
        self.cycle = 12;
    }

    // LD HL, SP+r8
    pub fn ld_sp_to_hl(&mut self, data: i8) {
        let temp = self.registers.sp;
        self.add_sp_r8(data);
        self.registers
            .ld_16bit_reg(Register16Bit::HL, self.registers.sp);
        self.registers.sp = temp;
    }

    // LD BC, d16
    pub fn ld_reg_16bit(&mut self, to: Register16Bit, data: u16) {
        self.registers.ld_16bit_reg(to, data);
        self.length = 3;
        self.cycle = 12;
    }

    // LD B, C
    pub fn ld_reg_reg(&mut self, to: Register8Bit, from: Register8Bit) {
        let val = self.registers.get_8bit_reg_value(from);
        self.registers.ld_8bit_reg(to, val);
        self.length = 1;
        self.cycle = 4
    }

    // LD (HL), C
    pub fn ld_reg_to_mem_reg(&mut self, from: Register8Bit, to: Register16Bit) {
        let address = self.registers.get_16bit_reg_value(to);
        let value = self.registers.get_8bit_reg_value(from);
        bus_write(&mut self.memory, address, value);
        self.length = 1;
        self.cycle = 8;
    }

    // LD C, (HL)
    pub fn ld_mem_reg_to_reg(&mut self, to: Register8Bit, from: Register16Bit) {
        let address = self.registers.get_16bit_reg_value(from);
        let result = bus_read(&mut self.memory, address).unwrap();
        self.registers.ld_8bit_reg(to, result);
        self.length = 1;
        self.cycle = 8;
    }

    // LD (C), A
    pub fn ld_a_c(&mut self) {
        let value = self.registers.get_8bit_reg_value(Register8Bit::A);
        let addr = self.registers.get_8bit_reg_value(Register8Bit::C) as u16;
        bus_write(&mut self.memory, 0xFF00 + addr, value);
        self.length = 2;
        self.cycle = 8;
    }

    // LD A, (C)
    pub fn ld_c_a(&mut self) {
        let value = 0xFF00 + (self.registers.get_8bit_reg_value(Register8Bit::C) as u16);
        let data = bus_read(&mut self.memory, value).unwrap();
        self.ld_reg_8bit(Register8Bit::A, data);
    }

    // LD (a16), A
    pub fn ld_reg_to_mem(&mut self, from: Register8Bit, to: u16) {
        let data = self.registers.get_8bit_reg_value(from);
        bus_write(&mut self.memory, to, data);
        self.length = 3;
        self.cycle = 16;
    }

    // LD A, (a16)
    pub fn ld_mem_to_reg(&mut self, to: Register8Bit, from: u16) {
        let data = bus_read(&mut self.memory, from).unwrap();
        self.registers.ld_8bit_reg(to, data);
        self.length = 3;
        self.cycle = 16;
    }

    // ---------------------ALU INSTRUCTIONS--------------------

    // INC A
    pub fn inc_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let half_carry = ((value & 0xF) + 1) & 0x10 == 0x10;
        let value = value.overflowing_add(1);
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(false),
        ];
        for flag in flags {
            self.registers.set_flag(flag);
        }
        self.ld_reg_8bit(reg, value.0);
        self.length = 1;
        self.cycle = 4;
    }

    // INC BC
    pub fn inc_16bit(&mut self, reg: Register16Bit) {
        let value = self.registers.get_16bit_reg_value(reg);
        let value = value.wrapping_add(1);
        self.registers.ld_16bit_reg(reg, value);
        self.length = 1;
        self.cycle = 8;
    }

    // INC (HL)
    pub fn inc_mem_reg(&mut self) {
        let register_value = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, register_value).unwrap();
        let res = value.wrapping_add(1);
        bus_write(&mut self.memory, register_value, res);
        self.length = 1;
        self.cycle = 12;
    }

    // DEC A
    pub fn dec_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        // REMEMBER TO CHANGE THIS OOOOOOOOOOOOOOOOO
        let half_carry = (((self.registers.a as i16) & 0xf) - (1_i16)) & 0x10 == 0x10;
        let value = value.overflowing_sub(1);
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(true),
        ];
        for flag in flags {
            self.registers.set_flag(flag);
        }
        self.ld_reg_8bit(reg, value.0);
        self.length = 1;
        self.cycle = 4;
    }

    // DEC BC
    pub fn dec_16bit(&mut self, reg: Register16Bit) {
        let value = self.registers.get_16bit_reg_value(reg);
        let value = value.wrapping_sub(1);
        self.registers.ld_16bit_reg(reg, value);
        self.length = 1;
        self.cycle = 8;
    }

    // DC (HL)
    pub fn dec_mem_reg(&mut self) {
        let register_value = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, register_value).unwrap();
        let res = value.wrapping_sub(1);
        bus_write(&mut self.memory, register_value, res);
        self.length = 1;
        self.cycle = 12;
    }

    // ADD C
    pub fn add_reg_to_reg_8bit(&mut self, from: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(from);
        self.add_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 4;
    }

    // ADD BC
    pub fn add_reg_16bit_to_reg_16_bit(&mut self, from: Register16Bit) {
        let value = self.registers.get_16bit_reg_value(from);
        let register_value = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let half_carry = ((register_value & 0x0FFF) + (value & 0x0FFF)) & 0x1000 == 0x1000;
        let value = register_value.overflowing_add(value);
        let flags = [
            Flag::Subtraction(false),
            Flag::HalfCarry(half_carry),
            Flag::Carry(value.1),
        ];
        self.registers.set_flags(&flags);
        self.registers.ld_16bit_reg(Register16Bit::HL, value.0);
        self.length = 1;
        self.cycle = 8;
    }

    // ADD d8
    pub fn add_8bit_to_reg_8bit(&mut self, value: u8) {
        let half_carry = ((self.registers.a & 0xF) + (value & 0xf)) & 0x10 == 0x10;
        let value = self.registers.a.overflowing_add(value);
        self.registers.a = value.0;
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(false),
            Flag::Carry(value.1),
        ];
        for flag in flags {
            self.registers.set_flag(flag);
        }
        self.length = 2;
        self.cycle = 8;
    }

    // ADD (HL)
    pub fn add_mem_reg_to_reg_8bit(&mut self) {
        let value = bus_read(
            &mut self.memory,
            self.registers.get_16bit_reg_value(Register16Bit::HL),
        )
        .unwrap();
        self.add_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 8;
    }

    // ADD SP, r8
    pub fn add_sp_r8(&mut self, data: i8) {
        let sp_signed = self.registers.sp as i32;
        let res = (sp_signed + data as i32).abs();
        let (carry, half_carry) = if data > 0 {
            (
                ((self.registers.sp & 0xFF) + ((data as u16) & 0xFF)) & 0x100 == 0x100,
                ((self.registers.sp & 0xF) + ((data as u16) & 0xF)) & 0x10 == 0x10,
            )
        } else {
            (
                ((self.registers.sp & 0xFF) - (data.abs() as u16) & 0xFF) & 0x100 == 0x100,
                ((self.registers.sp & 0xF) - ((data.abs() as u16) & 0xF)) & 0x10 == 0x10,
            )
        };
        let flags = [
            Flag::Carry(carry),
            Flag::HalfCarry(half_carry),
            Flag::Zero(false),
            Flag::Subtraction(false),
        ];
        self.registers.sp = (res & 0xFFFF) as u16;
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // ADC A
    pub fn adc_reg_to_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        self.adc_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 4;
    }

    // ADC d8
    pub fn adc_8bit_to_reg_8bit(&mut self, value: u8) {
        let prev_carry_flag = get_bit(self.registers.f, 4) == 1;
        self.add_8bit_to_reg_8bit(value);
        let prev_half_carry_flag = get_bit(self.registers.f, 5) == 1;
        if prev_carry_flag {
            self.inc_8bit(Register8Bit::A);
        }
        let current_half_carry_flag = self.registers.get_flag(Flag::HalfCarry(true));
        let current_carry_flag = self.registers.get_flag(Flag::Carry(true));
        let flags = [
            Flag::Carry(prev_half_carry_flag & current_carry_flag),
            Flag::HalfCarry(prev_half_carry_flag | current_half_carry_flag),
        ];
        for flag in flags {
            self.registers.set_flag(flag);
        }
        self.length = 2;
        self.cycle = 8;
    }

    // ADC (HL)
    pub fn adc_mem_reg_to_reg_8bit(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        self.adc_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 8;
    }

    // SUB C
    pub fn sub_reg_to_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        self.sub_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 4;
    }

    // SUB d8
    pub fn sub_8bit_to_reg_8bit(&mut self, value: u8) {
        let half_carry =
            (((self.registers.a as i16) & 0xf) - ((value as i16) & 0xf)) & 0x10 == 0x10;
        let value = self.registers.a.overflowing_sub(value);
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(true),
            Flag::Carry(value.1),
        ];
        self.registers.set_flags(&flags);
        self.registers.a = value.0;
        self.length = 2;
        self.cycle = 8;
    }

    // SUB (HL)
    pub fn sub_mem_reg_to_reg_8bit(&mut self) {
        let value = bus_read(
            &mut self.memory,
            self.registers.get_16bit_reg_value(Register16Bit::HL),
        )
        .unwrap();
        self.sub_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 8;
    }

    // SBC A
    pub fn sbc_reg_to_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        self.sbc_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 4;
    }

    // SBC d8
    pub fn sbc_8bit_to_reg_8bit(&mut self, value: u8) {
        let prev_carry_flag = self.registers.get_flag(Flag::Carry(true));
        self.sub_8bit_to_reg_8bit(value);
        let prev_half_carry_flag = self.registers.get_flag(Flag::HalfCarry(true));

        if prev_carry_flag {
            self.dec_8bit(Register8Bit::A);
        }
        let current_half_carry_flag = self.registers.get_flag(Flag::HalfCarry(true));
        let current_carry_flag = self.registers.get_flag(Flag::Carry(true));
        let flags = [
            Flag::Carry(prev_half_carry_flag & current_carry_flag),
            Flag::HalfCarry(prev_half_carry_flag | current_half_carry_flag),
        ];
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // SBC (HL)
    pub fn sbc_mem_reg_to_reg8bit(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        self.sbc_8bit_to_reg_8bit(value);
        self.length = 1;
        self.cycle = 8;
    }

    // BitWise B
    pub fn bitwise_reg_8bit(&mut self, reg: Register8Bit, operator: BitwiseOperator) {
        let value = self.registers.get_8bit_reg_value(reg);
        self.bitwise_8bit_reg_8bit(value, operator);
        self.length = 1;
        self.cycle = 4;
    }

    // Bitwise d8
    pub fn bitwise_8bit_reg_8bit(&mut self, value: u8, operator: BitwiseOperator) {
        let res = match operator {
            BitwiseOperator::And => self.registers.a & value,
            BitwiseOperator::Or => self.registers.a | value,
            BitwiseOperator::Xor => self.registers.a ^ value,
        };
        let flags = [
            Flag::Carry(false),
            Flag::HalfCarry(operator == BitwiseOperator::And),
            Flag::Subtraction(false),
            Flag::Zero(res == 0),
        ];
        for flag in flags {
            self.registers.set_flag(flag);
        }
        self.registers.a = res;
        self.length = 2;
        self.cycle = 8;
    }

    // Bitwise (HL)
    pub fn bitwise_mem_reg_to_reg_8bit(&mut self, operator: BitwiseOperator) {
        let value = bus_read(
            &mut self.memory,
            self.registers.get_16bit_reg_value(Register16Bit::HL),
        )
        .unwrap();
        self.bitwise_8bit_reg_8bit(value, operator);
        self.length = 1;
        self.cycle = 8;
    }

    // CP B
    pub fn cp_reg_8bit(&mut self, from: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(from);
        let prev_a = self.registers.a;
        self.sub_8bit_to_reg_8bit(value);
        self.registers.a = prev_a;
        self.length = 1;
        self.cycle = 4;
    }

    // RLC B
    pub fn rlc_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = value << 1 | value >> 7;
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.ld_8bit_reg(reg, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RLC (HL)
    pub fn rlc_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = value << 1 | value >> 7;
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(&mut self.memory, addr, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // RLA
    pub fn rla(&mut self) {
        let value = self.registers.a;
        let res = value << 1 | get_bit(self.registers.f, 4);
        self.registers.a = res;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RLCA
    pub fn rlca(&mut self) {
        let value = self.registers.a;
        let res = (value << 1) | value >> 7;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.a = res;
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RRCA
    pub fn rrca(&mut self) {
        let value = self.registers.a;
        let res = ((value & 1) << 7) | (value >> 1);
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.a = res;
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RRA
    pub fn rra(&mut self) {
        let value = self.registers.a;
        let res = (((self.registers.f >> 4) & 1) << 7) | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.a = res;
        self.registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RL A
    pub fn rl_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = value << 1 | ((self.registers.f >> 4) & 1);
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.ld_8bit_reg(reg, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RL (HL)
    pub fn rl_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = value << 1 | ((self.registers.f >> 4) & 1);
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(&mut self.memory, addr, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // RRC B
    pub fn rrc_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = (value & 1) << 7 | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.ld_8bit_reg(reg, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RRC (HL)
    pub fn rrc_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = (value & 1) << 7 | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(&mut self.memory, addr, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // RR B
    pub fn rr_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = get_bit(self.registers.f, 4) | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.ld_8bit_reg(reg, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RR (HL)
    pub fn rr_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = get_bit(self.registers.f, 4) | value >> 7;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(&mut self.memory, addr, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // SLA B
    pub fn sla_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = value << 1;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        self.registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SLA (HL)
    pub fn sla_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = value << 1;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        bus_write(&mut self.memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // SRA B
    pub fn sra_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = get_bit(value, 7) << 7 | value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        self.registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SRA (HL)
    pub fn sra_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = get_bit(value, 7) << 7 | value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        bus_write(&mut self.memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // SRL B
    pub fn srl_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        self.registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SRL (HL)
    pub fn srl_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.set_flags(&flags);
        bus_write(&mut self.memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // SWAP B
    pub fn swap_reg_8bit(&mut self, reg: Register8Bit) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = (value & 0xF) << 4 | (value & 0xF0) >> 4;
        let flags = [
            Flag::Zero(res == 0),
            Flag::Carry(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        self.registers.ld_8bit_reg(reg, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // SWAP (HL)
    pub fn swap_mem_reg(&mut self) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = (value & 0xF) << 4 | (value & 0xF0) >> 4;
        let flags = [
            Flag::Zero(res == 0),
            Flag::Carry(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(&mut self.memory, addr, res);
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // BIT 6, D
    pub fn bit_reg_8bit(&mut self, reg: Register8Bit, bit: u8) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = get_bit(value, bit) ^ 0b1;
        let flags = [
            Flag::Zero(res == 1),
            Flag::Subtraction(false),
            Flag::HalfCarry(true),
        ];
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // BIT 6, (HL)
    pub fn bit_mem_reg(&mut self, bit: u8) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = get_bit(value, bit) ^ 0b1;
        let flags = [
            Flag::Zero(res == 1),
            Flag::Subtraction(false),
            Flag::HalfCarry(true),
        ];
        self.registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // SET 4, B
    pub fn set_reg_8bit(&mut self, reg: Register8Bit, bit: u8) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = value | (1 << bit);
        self.registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SET 0, (HL)
    pub fn set_mem_reg(&mut self, bit: u8) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = value | (1 << bit);
        bus_write(&mut self.memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // RES 0, A
    pub fn res_reg_8bit(&mut self, reg: Register8Bit, bit: u8) {
        let value = self.registers.get_8bit_reg_value(reg);
        let res = if get_bit(value, bit) != 0 {
            value ^ (1 << bit)
        } else {
            value
        };
        self.registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // RES 0, (HL)
    pub fn res_mem_reg(&mut self, bit: u8) {
        let addr = self.registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&self.memory, addr).unwrap();
        let res = if get_bit(value, bit) != 0 {
            value ^ (1 << bit)
        } else {
            value
        };
        bus_write(&mut self.memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    pub fn set_pc(&mut self) {
        let val = bus_read_16bit_value(self.memory, 12, 13).unwrap();
        self.registers.pc = val;
        self.length = 3;
        self.cycle = 15;
    }

    pub fn new(registers: &'a mut CpuRegisters, memory: &'b mut Memory) -> Self {
        Instruction {
            registers,
            memory,
            length: 0,
            cycle: 0,
        }
    }
}
