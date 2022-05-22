use crate::cpu::{fetch::Command, CpuRegisters, Flag, Register16Bit, Register8Bit};
use crate::memory::bus_read_16bit_value;
use crate::{bus_read, bus_write, get_bit, Memory};

pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    None,
}

impl BitwiseOperator {
    pub fn get_operator(opcode: u8) -> Self {
        let (row, col) = (opcode >> 4, opcode & 0xf);
        match (row, col) {
            (0xA, 0x0..=0x7) => BitwiseOperator::And,
            (0xA, 0x8..=0xF) => BitwiseOperator::Xor,
            (0xB, 0x0..=0x7) => BitwiseOperator::Or,
            _ => BitwiseOperator::None,
        }
    }
}

pub struct Instruction {
    pub flag: Vec<Flag>,
    pub length: u8,
    pub cycle: u8,
}

impl Instruction {
    // ---------------------HELPER FUNCTIONS--------------------
    pub fn execute(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        opcode: u8,
        command: Command,
    ) {
        let (row, col) = (opcode >> 4, opcode & 0xF);
        match command {
            Command::NOP => self.no_op(),
            Command::RLCA => self.rlca(registers),
            Command::RLA => self.rla(registers),
            Command::DAA => self.daa(registers),
            Command::SCF => self.scf(registers),
            Command::RRCA => self.rrca(registers),
            Command::RRA => self.rra(registers),
            Command::CPL => self.cpl(registers),
            Command::CCF => self.ccf(registers),

            Command::LD_Sp_To_Mem => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                self.ld_sp_to_mem(registers, memory, data);
            }
            Command::LD_16Bit => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let reg = Register16Bit::get_left_instruction_argument(opcode);
                self.ld_reg_16bit(registers, reg, data);
            }
            Command::LD_Mem_Reg_A => {
                let reg = Register16Bit::get_left_instruction_argument(opcode);
                self.ld_reg_to_mem_reg(registers, memory, Register8Bit::A, reg);
            }
            Command::LD_A_Mem_Reg => {
                let reg = Register16Bit::get_right_instruction_argument(opcode);
                self.ld_mem_reg_to_reg(registers, memory, Register8Bit::A, reg);
            }
            Command::LD_Mem_Hli => self.ld_mem_hli(registers, memory),
            Command::LD_Mem_Hld => self.ld_mem_hld(registers, memory),
            Command::LD_Hli => self.ld_hli(registers, memory),
            Command::LD_Hld => self.ld_hld(registers, memory),
            Command::INC_16Bit => {
                let reg = Register16Bit::get_left_instruction_argument(opcode);
                self.inc_16bit(registers, reg);
            }
            Command::INC_8Bit => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.inc_8bit(registers, reg);
            }
            Command::INC_Mem_Reg => {
                self.inc_mem_reg(registers, memory);
            }

            Command::DEC_8bit => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.dec_8bit(registers, reg);
            }
            Command::DEC_Mem_Reg => self.dec_mem_reg(registers, memory),

            Command::LD_8Bit_Reg => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.ld_reg_8bit(registers, reg, data);
            }

            Command::LD_8Bit_Mem => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.ld_8bit_into_mem(registers, memory, data);
            }

            Command::ADD_16Bit => {
                let reg = Register16Bit::get_right_instruction_argument(opcode);
                self.add_reg_16bit_to_reg_16_bit(registers, reg);
            }

            Command::JR_8Bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                self.jr(registers, signed_data);
            }
            Command::JR_Eq_Zero => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                let flag = registers.get_flag(Flag::Zero(true));
                self.jr_eq(registers, Flag::Zero(flag), signed_data);
            }
            Command::JR_Eq_Carry => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                let flag = registers.get_flag(Flag::Carry(true));

                self.jr_eq(registers, Flag::Carry(flag), signed_data);
            }
            Command::JR_Not_Eq_Zero => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                let flag = registers.get_flag(Flag::Zero(true));
                self.jr_not_eq(registers, Flag::Zero(flag), signed_data);
            }
            Command::JR_Not_Eq_Carry => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                let flag = registers.get_flag(Flag::Carry(true));

                self.jr_not_eq(registers, Flag::Carry(flag), signed_data);
            }
            Command::LD_Reg_Reg => {
                let left_reg = Register8Bit::get_left_instruction_argument(opcode);
                let right_reg = Register8Bit::get_right_instruction_argument(opcode);
                self.ld_reg_reg(registers, left_reg, right_reg);
            }
            Command::LD_Reg_Mem => {
                let left_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.ld_mem_reg_to_reg(registers, memory, left_reg, Register16Bit::HL);
            }
            Command::LD_Mem_Reg => {
                let right_reg = Register8Bit::get_right_instruction_argument(opcode);
                self.ld_reg_to_mem_reg(registers, memory, right_reg, Register16Bit::HL);
            }
            Command::ADD_Reg => {
                let right_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.add_reg_to_reg_8bit(registers, right_reg);
            }
            Command::ADD_Mem => {
                self.add_mem_reg_to_reg_8bit(registers, memory);
            }

            Command::ADC_Reg => {
                let right_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.adc_reg_to_reg_8bit(registers, right_reg);
            }
            Command::ADC_Mem => {
                self.adc_mem_reg_to_reg_8bit(registers, memory);
            }
            Command::SUB_Reg => {
                let right_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.sub_reg_to_reg_8bit(registers, right_reg);
            }
            Command::SUB_Mem => {
                self.sub_mem_reg_to_reg_8bit(registers, memory);
            }
            Command::SBC_Reg => {
                let right_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.sbc_reg_to_reg_8bit(registers, right_reg);
            }
            Command::SBC_Mem => {
                self.sbc_mem_reg_to_reg_8bit(registers, memory);
            }
            Command::Bitwise_Reg => {
                let operator = BitwiseOperator::get_operator(opcode);
                let right_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.bitwise_reg_8bit(registers, right_reg, operator);
            }
            Command::Bitwise_Mem => {
                let operator = BitwiseOperator::get_operator(opcode);
                self.bitwise_mem_reg_to_reg_8bit(registers, memory, operator);
            }

            Command::CP_Reg => {
                let right_reg = Register8Bit::get_left_instruction_argument(opcode);
                self.cp_reg_8bit(registers, right_reg);
            }
            Command::CP_Mem => {
                self.cp_mem_reg_to_reg_8bit(registers, memory);
            }
            Command::RET => self.ret(registers, memory),
            Command::RET_Eq_Zero => {
                let flag = registers.get_flag(Flag::Zero(true));
                self.ret_eq(registers, memory, Flag::Zero(flag));
            }
            Command::RET_Eq_Carry => {
                let flag = registers.get_flag(Flag::Carry(true));
                self.ret_eq(registers, memory, Flag::Carry(flag));
            }
            Command::RET_Not_Eq_Zero => {
                let flag = registers.get_flag(Flag::Zero(true));
                self.ret_not_eq(registers, memory, Flag::Zero(flag));
            }
            Command::RET_Not_Eq_Carry => {
                let flag = registers.get_flag(Flag::Carry(true));
                self.ret_not_eq(registers, memory, Flag::Carry(flag));
            }
            Command::JP => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                self.jp(registers, data);
            }
            Command::JP_Mem => {
                self.jp_mem_reg(registers);
            }
            Command::JP_Eq_Zero => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Zero(true));
                self.jp_eq(registers, Flag::Zero(flag), data);
            }
            Command::JP_Eq_Carry => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Carry(true));
                self.jp_eq(registers, Flag::Carry(flag), data);
            }
            Command::JP_Not_Eq_Zero => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Zero(true));
                self.jp_not_eq(registers, Flag::Zero(flag), data);
            }
            Command::JP_Not_Eq_Carry => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Carry(true));
                self.jp_not_eq(registers, Flag::Carry(flag), data);
            }
            Command::CALL => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                self.call(registers, memory, data);
            }
            Command::CALL_Eq_Zero => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Zero(true));
                self.call_eq(registers, memory, Flag::Zero(flag), data);
            }
            Command::CALL_Eq_Carry => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Carry(true));
                self.call_eq(registers, memory, Flag::Carry(flag), data);
            }
            Command::CALL_Not_Eq_Zero => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Zero(true));
                self.call_not_eq(registers, memory, Flag::Zero(flag), data);
            }
            Command::CALL_Not_Eq_Carry => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                let flag = registers.get_flag(Flag::Carry(true));
                self.call_not_eq(registers, memory, Flag::Carry(flag), data);
            }

            Command::POP => {
                let reg = Register16Bit::get_left_instruction_argument(opcode);
                self.pop_16bit_reg(registers, memory, reg);
            }
            Command::PUSH => {
                let reg = Register16Bit::get_left_instruction_argument(opcode);
                self.push_16bit_reg(registers, memory, reg);
            }
            Command::ADD_8Bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.add_8bit_to_reg_8bit(registers, data);
            }
            Command::ADC_8Bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.adc_8bit_to_reg_8bit(registers, data);
            }
            Command::SBC_8Bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.sbc_8bit_to_reg_8bit(registers, data);
            }
            Command::Bitwise_8Bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.bitwise_8bit_reg_8bit(registers, data, BitwiseOperator::Xor);
            }
            Command::CP_8Bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.cp_8bit_reg_8bit(registers, data);
            }
            Command::RST => {
                self.rst(registers, memory, opcode);
            }
            Command::LD_Reg_Addr_8bit => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.ld_reg_8bit_to_addr_8bit(registers, memory, data);
            }
            Command::LD_Addr_8bit_Reg => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                self.ld_8bit_addr_to_reg_8bit(registers, memory, data);
            }
            Command::LD_Reg_Addr_16Bit => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                self.ld_reg_to_mem(registers, memory, Register8Bit::A, data);
            }
            Command::LD_Addr_16bit_Reg => {
                let data = bus_read_16bit_value(memory, registers.pc.wrapping_add(1)).unwrap();
                self.ld_mem_to_reg(registers, memory, Register8Bit::A, data);
            }
            Command::ADD_SP_Signed => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                self.add_sp_r8(registers, signed_data);
            }
            Command::LD_SP_Signed_HL => {
                let data = bus_read(memory, registers.pc.wrapping_add(1)).unwrap();
                let signed_data = i8::from_be_bytes([data]);
                self.ld_sp_to_hl_signed(registers, signed_data);
            }
            Command::LD_HL_SP => {
                self.ld_hl_to_sp(registers);
            }
            Command::LD_A_C => {
                self.ld_a_c(registers, memory);
            }
            Command::LD_C_A => {
                self.ld_c_a(registers, memory);
            }
            Command::RLC_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.rlc_reg_8bit(registers, reg);
            }
            Command::RLC_Mem => {
                self.rlc_mem_reg(registers, memory);
            }
            Command::RRC_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.rrc_reg_8bit(registers, reg);
            }
            Command::RRC_Mem => {
                self.rrc_mem_reg(registers, memory);
            }
            Command::RL_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.rl_reg_8bit(registers, reg);
            }
            Command::RL_Mem => {
                self.rl_mem_reg(registers, memory);
            }
            Command::RR_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.rr_reg_8bit(registers, reg);
            }
            Command::RR_Mem => {
                self.rr_mem_reg(registers, memory);
            }
            Command::SLA_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.sla_reg_8bit(registers, reg);
            }
            Command::SLA_Mem => {
                self.sla_mem_reg(registers, memory);
            }
            Command::SRA_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.sra_reg_8bit(registers, reg);
            }
            Command::SRA_Mem => {
                self.sra_mem_reg(registers, memory);
            }
            Command::Swap_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.swap_reg_8bit(registers, reg);
            }
            Command::Swap_Mem => {
                self.swap_mem_reg(registers, memory);
            }
            Command::SRL_Reg => {
                let reg = Register8Bit::get_left_instruction_argument(opcode);
                self.srl_reg_8bit(registers, reg);
            }
            Command::SRL_Mem => {
                self.srl_mem_reg(registers, memory);
            }
            Command::BIT_Reg => {
                let bit = ((row - 0x4) * 2) + if col > 0x7 { 1 } else { 0 };
                let reg = Register8Bit::get_left_instruction_argument_cb(opcode);
                self.bit_reg_8bit(registers, reg, bit);
            }
            Command::BIT_Mem => {
                let bit = ((row - 0x4) * 2) + if col > 0x7 { 1 } else { 0 };
                self.bit_mem_reg(registers, memory, bit);
            }
            Command::RES_Reg => {
                let bit = ((row - 0x8) * 2) + if col > 0x7 { 1 } else { 0 };
                let reg = Register8Bit::get_left_instruction_argument_cb(opcode);
                self.res_reg_8bit(registers, reg, bit);
            }
            Command::RES_Mem => {
                let bit = ((row - 0x8) * 2) + if col > 0x7 { 1 } else { 0 };
                self.res_mem_reg(registers, memory, bit);
            }
            Command::SET_Reg => {
                let bit = ((row - 0xC) * 2) + if col > 0x7 { 1 } else { 0 };
                let reg = Register8Bit::get_left_instruction_argument_cb(opcode);
                self.set_reg_8bit(registers, reg, bit);
            }
            Command::SET_Mem => {
                let bit = ((row - 0xC) * 2) + if col > 0x7 { 1 } else { 0 };
                self.set_mem_reg(registers, memory, bit);
            }
            _ => (),
        }
    }

    // ---------------------START OF INSTRUCTIONS--------------------

    // ---------------------CPU CONTROL INSTRUCTIONS--------------------

    // NOP
    pub fn no_op(&mut self) {
        self.length = 1;
        self.cycle = 4;
    }

    // DAA
    pub fn daa(&mut self, registers: &mut CpuRegisters) {
        // todo!("Yet to be implemented");
        let carry = registers.get_flag(Flag::Carry(true));
        let half_carry = registers.get_flag(Flag::HalfCarry(true));
        let negative = registers.get_flag(Flag::Subtraction(true));
        let mut flags: Vec<Flag> = vec![];
        if !negative {
            if carry || registers.a > 0x99 {
                registers.a = registers.a.wrapping_add(0x60);
                flags.push(Flag::Carry(true));
            }
            if half_carry || (registers.a & 0xf > 0x9) {
                registers.a = registers.a.wrapping_add(0x6);
            }
        } else {
            if carry {
                registers.a = registers.a.wrapping_sub(0x60);
            }
            if half_carry {
                registers.a = registers.a.wrapping_sub(0x6);
            }
        }
        flags.push(Flag::Zero(registers.a == 0));
        flags.push(Flag::HalfCarry(false));
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // CPL
    pub fn cpl(&mut self, registers: &mut CpuRegisters) {
        registers.a = registers.a ^ 0xFF;
        let flags = [Flag::Subtraction(true), Flag::HalfCarry(true)];
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // SCF
    pub fn scf(&mut self, registers: &mut CpuRegisters) {
        let flags = [
            Flag::Carry(true),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // CCF
    pub fn ccf(&mut self, registers: &mut CpuRegisters) {
        let flags = [
            Flag::Carry(get_bit(registers.f, 4) ^ 1 == 1),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RLA
    pub fn rla(&mut self, registers: &mut CpuRegisters) {
        let value = registers.a;
        let res = value << 1 | get_bit(registers.f, 4);
        registers.a = res;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RLCA
    pub fn rlca(&mut self, registers: &mut CpuRegisters) {
        let value = registers.a;
        let res = (value << 1) | value >> 7;
        let flags = [
            Flag::Carry(get_bit(value, 7) == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.a = res;
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RRCA
    pub fn rrca(&mut self, registers: &mut CpuRegisters) {
        let value = registers.a;
        let res = ((value & 1) << 7) | (value >> 1);
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.a = res;
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // RRA
    pub fn rra(&mut self, registers: &mut CpuRegisters) {
        let value = registers.a;
        let res = (((registers.f >> 4) & 1) << 7) | value >> 1;
        let flags = [
            Flag::Carry(value & 1 == 1),
            Flag::Zero(false),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        registers.a = res;
        registers.set_flags(&flags);
        self.length = 1;
        self.cycle = 4;
    }

    // ---------------------JUMP INSTRUCTIONS--------------------

    // JP NZ, a16
    pub fn jp_not_eq(&mut self, registers: &mut CpuRegisters, flag: Flag, addr: u16) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                registers.pc = addr;
                (0, 16)
            }
            Flag::Zero(false) => {
                registers.pc = addr;
                (0, 16)
            }
            _ => (3, 12),
        };
    }

    // JP C, a16
    pub fn jp_eq(&mut self, registers: &mut CpuRegisters, flag: Flag, addr: u16) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                registers.pc = addr;
                (0, 16)
            }
            Flag::Zero(true) => {
                registers.pc = addr;
                (0, 16)
            }
            _ => (3, 12),
        };
    }

    // JP (HL)
    pub fn jp_mem_reg(&mut self, registers: &mut CpuRegisters) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        registers.pc = addr;
        self.length = 0;
        self.cycle = 4;
    }

    // JP a16
    pub fn jp(&mut self, registers: &mut CpuRegisters, addr: u16) {
        registers.pc = addr;
        self.length = 0;
        self.cycle = 16;
    }

    // JR NZ, r8
    pub fn jr_not_eq(&mut self, registers: &mut CpuRegisters, flag: Flag, data: i8) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                registers.pc = ((registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            Flag::Zero(false) => {
                registers.pc = ((registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            _ => (2, 16),
        };
    }

    // JR Z, r8
    pub fn jr_eq(&mut self, registers: &mut CpuRegisters, flag: Flag, data: i8) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                registers.pc = ((registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            Flag::Zero(true) => {
                registers.pc = ((registers.pc as i16) + (data as i16)) as u16;
                (0, 12)
            }
            _ => (2, 8),
        };
    }

    // JR r8
    pub fn jr(&mut self, registers: &mut CpuRegisters, data: i8) {
        registers.pc = ((registers.pc as i16) + (data as i16)) as u16;
        self.length = 0;
        self.cycle = 12;
    }

    // ---------------------CALL INSTRUCTIONS--------------------

    // CALL NZ, a16
    pub fn call_not_eq(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        flag: Flag,
        addr: u16,
    ) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                self.call(registers, memory, addr);
                (0, 24)
            }
            Flag::Zero(false) => {
                self.call(registers, memory, addr);
                (0, 24)
            }
            _ => (3, 12),
        };
    }

    // CALL Z, a16
    pub fn call_eq(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        flag: Flag,
        addr: u16,
    ) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                self.call(registers, memory, addr);
                (0, 24)
            }
            Flag::Zero(true) => {
                self.call(registers, memory, addr);
                (0, 24)
            }
            _ => (3, 12),
        };
    }

    // CALL a16
    pub fn call(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, addr: u16) {
        self.push_16bit_reg(registers, memory, Register16Bit::PC);
        registers.pc = addr;
        self.length = 0;
        self.cycle = 24;
    }

    // ---------------------RETURN INSTRUCTIONS--------------------

    // RET
    pub fn ret(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        self.pop_16bit_reg(registers, memory, Register16Bit::PC);
        self.length = 0;
        self.cycle = 16;
    }

    // RET C
    pub fn ret_eq(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, flag: Flag) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(true) => {
                self.ret(registers, memory);
                (0, 20)
            }
            Flag::Zero(true) => {
                self.ret(registers, memory);
                (0, 20)
            }
            _ => (1, 8),
        };
    }

    // RET NC
    pub fn ret_not_eq(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, flag: Flag) {
        (self.length, self.cycle) = match flag {
            Flag::Carry(false) => {
                self.ret(registers, memory);
                (0, 20)
            }
            Flag::Zero(false) => {
                self.ret(registers, memory);
                (0, 20)
            }
            _ => (1, 8),
        };
    }

    // RETI
    pub fn reti(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let hi = bus_read(memory, registers.sp + 1).unwrap() as u16;
        let lo = bus_read(memory, registers.sp).unwrap() as u16;
        registers.pc = hi << 4 | lo;
        registers.sp += 2;
        self.length = 1;
        self.cycle = 16;
    }

    // ---------------------STACK INSTRUCTIONS--------------------

    // POP BC
    pub fn pop_16bit_reg(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        reg: Register16Bit,
    ) {
        let hi = bus_read(memory, registers.sp + 1).unwrap();
        let lo = bus_read(memory, registers.sp).unwrap();
        let res = (hi as u16) << 8 | (lo as u16);
        registers.ld_16bit_reg(reg, res);
        registers.sp += 2;
        self.length = 1;
        self.cycle = 12;
    }

    // PUSH BC
    pub fn push_16bit_reg(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        reg: Register16Bit,
    ) {
        let [hi, lo] = registers.get_16bit_reg_value(reg).to_be_bytes();
        bus_write(memory, registers.sp - 2, lo);
        bus_write(memory, registers.sp - 1, hi);
        registers.sp = registers.sp.wrapping_sub(2);
        self.length = 1;
        self.cycle = 16;
    }

    // ---------------------RST INSTRUCTIONS--------------------

    // RST
    pub fn rst(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, opcode: u8) {
        let ret_addr = opcode & 0x38;
        self.push_16bit_reg(registers, memory, Register16Bit::PC);
        registers.pc = ret_addr as u16;
        self.length = 1;
        self.cycle = 16;
    }

    // ---------------------LOAD INSTRUCTIONS--------------------

    // LD A, d8
    pub fn ld_reg_8bit(&mut self, registers: &mut CpuRegisters, to: Register8Bit, data: u8) {
        registers.ld_8bit_reg(to, data);
        self.length = 2;
        self.cycle = 8;
    }

    // LD A, (HL+)
    pub fn ld_hli(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        self.ld_mem_reg_to_reg(registers, memory, Register8Bit::A, Register16Bit::HL);
        self.inc_16bit(registers, Register16Bit::HL);
    }

    // LD A, (HL-)
    pub fn ld_hld(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        self.ld_mem_reg_to_reg(registers, memory, Register8Bit::A, Register16Bit::HL);
        self.dec_16bit(registers, Register16Bit::HL);
    }

    // LD (HL+), A
    pub fn ld_mem_hli(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        self.ld_reg_to_mem_reg(registers, memory, Register8Bit::A, Register16Bit::HL);
        self.inc_16bit(registers, Register16Bit::HL);
    }

    // LD (HL-), A
    pub fn ld_mem_hld(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        self.ld_reg_to_mem_reg(registers, memory, Register8Bit::A, Register16Bit::HL);
        self.dec_16bit(registers, Register16Bit::HL);
    }

    // LD (HL), A
    pub fn ld_8bit_into_mem(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        data: u8,
    ) {
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        bus_write(memory, addr, data);
        self.length = 2;
        self.cycle = 12;
    }

    // LD (a16), SP
    pub fn ld_16bit_reg_to_mem(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        address: u16,
    ) {
        bus_write(memory, address, (registers.sp & 0xFF) as u8);
        bus_write(memory, address + 1, (registers.sp >> 8) as u8);
        self.length = 3;
        self.cycle = 20;
    }

    // LDH A, (a8)
    pub fn ld_8bit_addr_to_reg_8bit(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        addr: u8,
    ) {
        let address = 0xFF00 + (addr as u16);
        let value = bus_read(memory, address).unwrap();
        registers.a = value;
        self.length = 2;
        self.cycle = 12;
    }

    // LDH (a8), A
    pub fn ld_reg_8bit_to_addr_8bit(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        addr: u8,
    ) {
        let address = 0xFF00 + (addr as u16);
        bus_write(memory, address, registers.a);
        self.length = 2;
        self.cycle = 12;
    }

    // LD HL, SP+r8
    pub fn ld_sp_to_hl_signed(&mut self, registers: &mut CpuRegisters, data: i8) {
        let temp = registers.sp;
        self.add_sp_r8(registers, data);
        registers.ld_16bit_reg(Register16Bit::HL, registers.sp);
        registers.sp = temp;
    }

    // LD SP, HL
    pub fn ld_hl_to_sp(&mut self, registers: &mut CpuRegisters) {
        let value = registers.get_16bit_reg_value(Register16Bit::HL);
        self.ld_reg_16bit(registers, Register16Bit::SP, value);
        self.length = 1;
        self.cycle = 8;
    }

    // LD (a16), SP
    pub fn ld_sp_to_mem(&mut self, registers: &mut CpuRegisters, memory: &mut Memory, addr: u16) {
        let [hi, lo] = registers
            .get_16bit_reg_value(Register16Bit::SP)
            .to_be_bytes();
        bus_write(memory, addr, lo);
        bus_write(memory, addr.wrapping_add(1), hi);
        self.length = 3;
        self.cycle = 20;
    }

    // LD BC, d16
    pub fn ld_reg_16bit(&mut self, registers: &mut CpuRegisters, to: Register16Bit, data: u16) {
        registers.ld_16bit_reg(to, data);
        self.length = 3;
        self.cycle = 12;
    }

    // LD B, C
    pub fn ld_reg_reg(
        &mut self,
        registers: &mut CpuRegisters,
        to: Register8Bit,
        from: Register8Bit,
    ) {
        let val = registers.get_8bit_reg_value(from);
        registers.ld_8bit_reg(to, val);
        self.length = 1;
        self.cycle = 4
    }

    // LD (HL), C
    pub fn ld_reg_to_mem_reg(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        from: Register8Bit,
        to: Register16Bit,
    ) {
        let address = registers.get_16bit_reg_value(to);
        let value = registers.get_8bit_reg_value(from);
        bus_write(memory, address, value);
        self.length = 1;
        self.cycle = 8;
    }

    // LD C, (HL)
    pub fn ld_mem_reg_to_reg(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        to: Register8Bit,
        from: Register16Bit,
    ) {
        let address = registers.get_16bit_reg_value(from);
        let result = bus_read(memory, address).unwrap();
        registers.ld_8bit_reg(to, result);
        self.length = 1;
        self.cycle = 8;
    }

    // LD (C), A
    pub fn ld_a_c(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let value = registers.get_8bit_reg_value(Register8Bit::A);
        let addr = registers.get_8bit_reg_value(Register8Bit::C) as u16;
        bus_write(memory, 0xFF00 + addr, value);
        self.length = 2;
        self.cycle = 8;
    }

    // LD A, (C)
    pub fn ld_c_a(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) {
        let value = 0xFF00 + (registers.get_8bit_reg_value(Register8Bit::C) as u16);
        let data = bus_read(memory, value).unwrap();
        self.ld_reg_8bit(registers, Register8Bit::A, data);
    }

    // LD (a16), A
    pub fn ld_reg_to_mem(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        from: Register8Bit,
        to: u16,
    ) {
        let data = registers.get_8bit_reg_value(from);
        bus_write(memory, to, data);
        self.length = 3;
        self.cycle = 16;
    }

    // LD A, (a16)
    pub fn ld_mem_to_reg(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &mut Memory,
        to: Register8Bit,
        from: u16,
    ) {
        let data = bus_read(memory, from).unwrap();
        registers.ld_8bit_reg(to, data);
        self.length = 3;
        self.cycle = 16;
    }

    pub fn new() -> Self {
        Instruction {
            flag: vec![],
            length: 0,
            cycle: 0,
        }
    }
}
