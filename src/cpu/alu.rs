use crate::{
    bus_read, bus_write, get_bit, BitwiseOperator, CpuRegisters, Flag, Instruction, Memory,
    Register16Bit, Register8Bit,
};
impl Instruction {
    // ---------------------ALU INSTRUCTIONS--------------------

    // INC A
    pub fn inc_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let half_carry = ((value & 0xF) + 1) & 0x10 == 0x10;
        let value = value.overflowing_add(1);
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(false),
        ];
        for flag in flags {
            registers.set_flag(flag);
        }
        registers.ld_8bit_reg(reg, value.0);
        self.length = 1;
        self.cycle = 4;
    }

    // INC BC
    pub fn inc_16bit(&mut self, registers: &mut CpuRegisters, reg: Register16Bit) {
        let value = registers.get_16bit_reg_value(reg);
        let value = value.wrapping_add(1);
        registers.ld_16bit_reg(reg, value);
        self.length = 1;
        self.cycle = 8;
    }

    // INC (HL)
    pub fn inc_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let register_value = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&memory, register_value).unwrap();
        let res = value.wrapping_add(1);
        bus_write(memory, register_value, res);
        self.length = 1;
        self.cycle = 12;
    }

    // DEC A
    pub fn dec_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        // REMEMBER TO CHANGE THIS OOOOOOOOOOOOOOOOO
        let half_carry = (((registers.a as i16) & 0xf) - (1_i16)) & 0x10 == 0x10;
        let value = value.overflowing_sub(1);
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(true),
        ];
        for flag in flags {
            registers.set_flag(flag);
        }
        registers.ld_8bit_reg(reg, value.0);
        self.length = 1;
        self.cycle = 4;
    }

    // DEC BC
    pub fn dec_16bit(&mut self, registers: &mut CpuRegisters, reg: Register16Bit) {
        let value = registers.get_16bit_reg_value(reg);
        let value = value.wrapping_sub(1);
        registers.ld_16bit_reg(reg, value);
        self.length = 1;
        self.cycle = 8;
    }

    // DC (HL)
    pub fn dec_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let register_value = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&memory, register_value).unwrap();
        let res = value.wrapping_sub(1);
        bus_write(memory, register_value, res);
        self.length = 1;
        self.cycle = 12;
    }

    // ADD C
    pub fn add_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, from: Register8Bit) {
        let value = registers.get_8bit_reg_value(from);
        self.add_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 4;
    }

    // ADD HL, BC
    pub fn add_reg_16bit_to_reg_16_bit(
        &mut self,
        registers: &mut CpuRegisters,
        from: Register16Bit,
    ) {
        let value = registers.get_16bit_reg_value(from);
        let register_value = registers.get_16bit_reg_value(Register16Bit::HL);
        let half_carry = ((register_value & 0x0FFF) + (value & 0x0FFF)) & 0x1000 == 0x1000;
        let value = register_value.overflowing_add(value);
        let flags = [
            Flag::Subtraction(false),
            Flag::HalfCarry(half_carry),
            Flag::Carry(value.1),
        ];
        registers.set_flags(&flags);
        registers.ld_16bit_reg(Register16Bit::HL, value.0);
        self.length = 1;
        self.cycle = 8;
    }

    // ADD d8
    pub fn add_8bit_to_reg_8bit(&mut self, registers: &mut CpuRegisters, value: u8) {
        let half_carry = ((registers.a & 0xF) + (value & 0xf)) & 0x10 == 0x10;
        let value = registers.a.overflowing_add(value);
        registers.a = value.0;
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(false),
            Flag::Carry(value.1),
        ];
        for flag in flags {
            registers.set_flag(flag);
        }
        self.length = 2;
        self.cycle = 8;
    }

    // ADD (HL)
    pub fn add_mem_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let value = bus_read(memory, registers.get_16bit_reg_value(Register16Bit::HL)).unwrap();
        self.add_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 8;
    }

    // ADD SP, r8
    pub fn add_sp_r8(&mut self, registers: &mut CpuRegisters, data: i8) {
        let sp_signed = registers.sp as i32;
        let res = (sp_signed + data as i32).abs();
        let (carry, half_carry) = if data > 0 {
            (
                ((registers.sp & 0xFF) + ((data as u16) & 0xFF)) & 0x100 == 0x100,
                ((registers.sp & 0xF) + ((data as u16) & 0xF)) & 0x10 == 0x10,
            )
        } else {
            (
                ((registers.sp & 0xFF) - (data.abs() as u16) & 0xFF) & 0x100 == 0x100,
                ((registers.sp & 0xF) - ((data.abs() as u16) & 0xF)) & 0x10 == 0x10,
            )
        };
        let flags = [
            Flag::Carry(carry),
            Flag::HalfCarry(half_carry),
            Flag::Zero(false),
            Flag::Subtraction(false),
        ];
        registers.sp = (res & 0xFFFF) as u16;
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // ADC A
    pub fn adc_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        self.adc_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 4;
    }

    // ADC d8
    pub fn adc_8bit_to_reg_8bit(&mut self, registers: &mut CpuRegisters, value: u8) {
        let prev_carry_flag = get_bit(registers.f, 4) == 1;
        self.add_8bit_to_reg_8bit(registers, value);
        let prev_half_carry_flag = get_bit(registers.f, 5) == 1;
        if prev_carry_flag {
            self.inc_8bit(registers, Register8Bit::A);
        }
        let current_half_carry_flag = registers.get_flag(Flag::HalfCarry(true));
        let current_carry_flag = registers.get_flag(Flag::Carry(true));
        let flags = [
            Flag::Carry(prev_half_carry_flag & current_carry_flag),
            Flag::HalfCarry(prev_half_carry_flag | current_half_carry_flag),
        ];
        for flag in flags {
            registers.set_flag(flag);
        }
        self.length = 2;
        self.cycle = 8;
    }

    // ADC (HL)
    pub fn adc_mem_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&memory, addr).unwrap();
        self.adc_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 8;
    }

    // SUB C
    pub fn sub_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        self.sub_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 4;
    }

    // SUB d8
    pub fn sub_8bit_to_reg_8bit(&mut self, registers: &mut CpuRegisters, value: u8) {
        let half_carry = (((registers.a as i16) & 0xf) - ((value as i16) & 0xf)) & 0x10 == 0x10;
        let value = registers.a.overflowing_sub(value);
        let flags = [
            Flag::HalfCarry(half_carry),
            Flag::Zero(value.0 == 0),
            Flag::Subtraction(true),
            Flag::Carry(value.1),
        ];
        registers.set_flags(&flags);
        registers.a = value.0;
        self.length = 2;
        self.cycle = 8;
    }

    // SUB (HL)
    pub fn sub_mem_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let value = bus_read(memory, registers.get_16bit_reg_value(Register16Bit::HL)).unwrap();
        self.sub_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 8;
    }

    // SBC A
    pub fn sbc_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        self.sbc_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 4;
    }

    // SBC d8
    pub fn sbc_8bit_to_reg_8bit(&mut self, registers: &mut CpuRegisters, value: u8) {
        let prev_carry_flag = registers.get_flag(Flag::Carry(true));
        self.sub_8bit_to_reg_8bit(registers, value);
        let prev_half_carry_flag = registers.get_flag(Flag::HalfCarry(true));

        if prev_carry_flag {
            self.dec_8bit(registers, Register8Bit::A);
        }
        let current_half_carry_flag = registers.get_flag(Flag::HalfCarry(true));
        let current_carry_flag = registers.get_flag(Flag::Carry(true));
        let flags = [
            Flag::Carry(prev_half_carry_flag & current_carry_flag),
            Flag::HalfCarry(prev_half_carry_flag | current_half_carry_flag),
        ];
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // SBC (HL)
    pub fn sbc_mem_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&memory, addr).unwrap();
        self.sbc_8bit_to_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 8;
    }

    // BitWise B
    pub fn bitwise_reg_8bit(
        &mut self,
        registers: &mut CpuRegisters,
        reg: Register8Bit,
        operator: BitwiseOperator,
    ) {
        let value = registers.get_8bit_reg_value(reg);
        self.bitwise_8bit_reg_8bit(registers, value, operator);
        self.length = 1;
        self.cycle = 4;
    }

    // Bitwise d8
    pub fn bitwise_8bit_reg_8bit(
        &mut self,
        registers: &mut CpuRegisters,
        value: u8,
        operator: BitwiseOperator,
    ) {
        let res = match operator {
            BitwiseOperator::And => registers.a & value,
            BitwiseOperator::Or => registers.a | value,
            BitwiseOperator::Xor => registers.a ^ value,
            _ => return,
        };
        let half_carry = match operator {
            BitwiseOperator::And => true,
            _ => false,
        };
        let flags = [
            Flag::Carry(false),
            Flag::HalfCarry(half_carry),
            Flag::Subtraction(false),
            Flag::Zero(res == 0),
        ];
        for flag in flags {
            registers.set_flag(flag);
        }
        registers.a = res;
        self.length = 2;
        self.cycle = 8;
    }

    // Bitwise (HL)
    pub fn bitwise_mem_reg_to_reg_8bit(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        operator: BitwiseOperator,
    ) {
        let value = bus_read(memory, registers.get_16bit_reg_value(Register16Bit::HL)).unwrap();
        self.bitwise_8bit_reg_8bit(registers, value, operator);
        self.length = 1;
        self.cycle = 8;
    }

    // CP B
    pub fn cp_reg_8bit(&mut self, registers: &mut CpuRegisters, from: Register8Bit) {
        let value = registers.get_8bit_reg_value(from);
        self.cp_8bit_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 4;
    }

    // CP (HL)
    pub fn cp_mem_reg_to_reg_8bit(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        self.cp_8bit_reg_8bit(registers, value);
        self.length = 1;
        self.cycle = 8;
    }

    // CP d8
    pub fn cp_8bit_reg_8bit(&mut self, registers: &mut CpuRegisters, value: u8) {
        let prev_a = registers.a;
        self.sub_8bit_to_reg_8bit(registers, value);
        registers.a = prev_a;
        self.length = 2;
        self.cycle = 8;
    }
    // RLC B
    pub fn rlc_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = value << 1 | value >> 7;
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.ld_8bit_reg(reg, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RLC (HL)
    pub fn rlc_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = value << 1 | value >> 7;
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(memory, addr, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // RL A
    pub fn rl_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = value << 1 | ((registers.f >> 4) & 1);
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.ld_8bit_reg(reg, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RL (HL)
    pub fn rl_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = value << 1 | ((registers.f >> 4) & 1);
        let flags = [
            Flag::Carry((value >> 7) & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(memory, addr, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // RRC B
    pub fn rrc_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = (value & 1) << 7 | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.ld_8bit_reg(reg, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RRC (HL)
    pub fn rrc_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = (value & 1) << 7 | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(memory, addr, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // RR B
    pub fn rr_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = get_bit(registers.f, 4) | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.ld_8bit_reg(reg, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // RR (HL)
    pub fn rr_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = get_bit(registers.f, 4) | value >> 7;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(memory, addr, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // SLA B
    pub fn sla_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = value << 1;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SLA (HL)
    pub fn sla_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = value << 1;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        bus_write(memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // SRA B
    pub fn sra_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = get_bit(value, 7) << 7 | value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SRA (HL)
    pub fn sra_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = get_bit(value, 7) << 7 | value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        bus_write(memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // SRL B
    pub fn srl_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SRL (HL)
    pub fn srl_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = value >> 1;
        let flags = [
            Flag::Carry(get_bit(value, 0) == 1),
            Flag::Zero(res == 0),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        bus_write(memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // SWAP B
    pub fn swap_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit) {
        let value = registers.get_8bit_reg_value(reg);
        let res = (value & 0xF) << 4 | (value & 0xF0) >> 4;
        let flags = [
            Flag::Zero(res == 0),
            Flag::Carry(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.ld_8bit_reg(reg, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // SWAP (HL)
    pub fn swap_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = (value & 0xF) << 4 | (value & 0xF0) >> 4;
        let flags = [
            Flag::Zero(res == 0),
            Flag::Carry(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        bus_write(memory, addr, res);
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // BIT 6, D
    pub fn bit_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit, bit: u8) {
        let value = registers.get_8bit_reg_value(reg);
        let res = get_bit(value, bit) ^ 0b1;
        let flags = [
            Flag::Zero(res == 1),
            Flag::Subtraction(false),
            Flag::HalfCarry(true),
        ];
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 8;
    }

    // BIT 6, (HL)
    pub fn bit_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, bit: u8) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = get_bit(value, bit) ^ 0b1;
        let flags = [
            Flag::Zero(res == 1),
            Flag::Subtraction(false),
            Flag::HalfCarry(true),
        ];
        registers.set_flags(&flags);
        self.length = 2;
        self.cycle = 16;
    }

    // SET 4, B
    pub fn set_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit, bit: u8) {
        let value = registers.get_8bit_reg_value(reg);
        let res = value | (1 << bit);
        registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // SET 0, (HL)
    pub fn set_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, bit: u8) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = value | (1 << bit);
        bus_write(memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }

    // RES 0, A
    pub fn res_reg_8bit(&mut self, registers: &mut CpuRegisters, reg: Register8Bit, bit: u8) {
        let value = registers.get_8bit_reg_value(reg);
        let res = if get_bit(value, bit) != 0 {
            value ^ (1 << bit)
        } else {
            value
        };
        registers.ld_8bit_reg(reg, res);
        self.length = 2;
        self.cycle = 8;
    }

    // RES 0, (HL)
    pub fn res_mem_reg(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, bit: u8) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(memory, addr).unwrap();
        let res = if get_bit(value, bit) != 0 {
            value ^ (1 << bit)
        } else {
            value
        };
        bus_write(memory, addr, res);
        self.length = 2;
        self.cycle = 16;
    }
}
