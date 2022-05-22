#[allow(non_camel_case_types)]
pub enum Command {
    NOP,
    HALT,
    LD_16Bit,
    LD_Mem_Reg,
    LD_Mem_Reg_A,
    INC_16Bit,
    INC_8Bit,
    LD_8Bit,
    LD_A_Mem_Reg,
    LD_Reg_Reg,
    RLCA,
    RLA,
    LD_SP_Signed_HL,
    LD_HL_SP,
    ADD_16Bit,
    LD_8Bit_Mem,
    DEC_16Bit,
    DEC_8bit,
    Stop,
    JR_8Bit,
    JR_Eq_Carry,
    JR_Eq_Zero,
    JR_Not_Eq_Carry,
    JR_Not_Eq_Zero,
    LD_Hli,
    LD_Hld,
    LD_8Bit_Reg,
    LD_Reg_Mem,
    LD_Reg_Addr_16Bit,
    LD_Addr_16bit_Reg,
    LD_Reg_Addr_8bit,
    LD_Addr_8bit_Reg,
    ADD_Reg,
    ADC_Reg,
    SUB_Reg,
    SBC_Reg,
    AND_Reg,
    XOR_Reg,
    OR_Reg,
    CP_Reg,
    CP_Mem,
    CP_8Bit,
    RET,
    RET_Eq_Zero,
    RET_Eq_Carry,
    RET_Not_Eq_Zero,
    RET_Not_Eq_Carry,
    POP,
    PUSH,
    INC_Mem_Reg,
    LDH_Reg,
    LDH_Mem,
    LD_A_C,
    LD_C_A,
    JP,
    JP_Mem,
    JP_Eq_Zero,
    JP_Eq_Carry,
    JP_Not_Eq_Zero,
    JP_Not_Eq_Carry,
    CALL,
    CALL_Eq_Zero,
    CALL_Eq_Carry,
    CALL_Not_Eq_Zero,
    CALL_Not_Eq_Carry,
    ADD_Mem,
    ADD_8Bit,
    ADD_SP_Signed,
    SUB_Mem,
    SUB_8Bit,
    SBC_Mem,
    SBC_8Bit,
    ADC_Mem,
    ADC_8Bit,
    AND_8Bit,
    OR_8Bit,
    XOR_8Bit,
    DAA,
    SCF,
    RRCA,
    RRA,
    CPL,
    DEC_Mem_Reg,
    CCF,
    LD_Sp_To_Mem,
    LD_Mem_Hld,
    LD_Mem_Hli,
    Bitwise_Reg,
    Bitwise_Mem,
    Bitwise_8Bit,
    RST,
    CB,
    RLC_Reg,
    RLC_Mem,
    RRC_Reg,
    RRC_Mem,
    RL_Reg,
    RL_Mem,
    RR_Reg,
    RR_Mem,
    SLA_Reg,
    SLA_Mem,
    SRA_Reg,
    SRA_Mem,
    Swap_Reg,
    Swap_Mem,
    SRL_Reg,
    SRL_Mem,
    BIT_Reg,
    BIT_Mem,
    RES_Reg,
    RES_Mem,
    SET_Reg,
    SET_Mem,
    None,
}
impl Command {
    pub fn get_instruction(opcode: u8) -> Self {
        let (row, col) = (opcode >> 4, opcode & 0xf);
        match (row, col) {
            (0x0, 0x0) => Command::NOP,
            (0x0, 0x7) => Command::RLCA,
            (0x1, 0x7) => Command::RLA,
            (0x2, 0x7) => Command::DAA,
            (0x3, 0x7) => Command::SCF,
            (0x0, 0xf) => Command::RRCA,
            (0x1, 0xf) => Command::RRA,
            (0x2, 0xf) => Command::CPL,
            (0x3, 0xf) => Command::CCF,

            (0x0, 0x8) => Command::LD_Sp_To_Mem,

            (0x0..=0x3, 0x1) => Command::LD_16Bit,
            (0x0..=0x1, 0x2) => Command::LD_Mem_Reg,
            (0x0..=0x1, 0xA) => Command::LD_A_Mem_Reg,
            (0x2, 0x2) => Command::LD_Mem_Hli,
            (0x3, 0x2) => Command::LD_Mem_Hld,
            (0x2, 0xA) => Command::LD_Hli,
            (0x3, 0xA) => Command::LD_Hld,
            (0x0..=0x3, 0x3) => Command::INC_16Bit,
            (0x0..=0x2, 0x4 | 0xC) => Command::INC_8Bit,
            (0x3, 0x4) => Command::INC_Mem_Reg,
            (0x3, 0xC) => Command::INC_8Bit,

            (0x0..=0x2, 0x5 | 0xD) => Command::DEC_8bit,
            (0x3, 0x5) => Command::DEC_Mem_Reg,

            (0x3, 0xD) => Command::DEC_8bit,

            (0x0..=0x2, 0x6 | 0xE) => Command::LD_8Bit_Reg,

            (0x3, 0x6) => Command::LD_8Bit_Mem,

            (0x0..=0x3, 0x9) => Command::ADD_16Bit,

            (0x3, 0xE) => Command::LD_8Bit_Reg,
            (0x1, 0x8) => Command::JR_8Bit,
            (0x2, 0x8) => Command::JR_Eq_Zero,
            (0x3, 0x8) => Command::JR_Eq_Carry,
            (0x2, 0x0) => Command::JR_Not_Eq_Zero,
            (0x3, 0x0) => Command::JR_Not_Eq_Carry,
            (0x4..=0x6, _) => {
                if col == 0x6 || col == 0xE {
                    Command::LD_Reg_Mem
                } else {
                    Command::LD_Reg_Reg
                }
            }
            (0x7, 0x0..=0x7) => {
                if col != 0x6 {
                    Command::LD_Mem_Reg
                } else {
                    Command::HALT
                }
            }
            (0x7, 0x8..=0xF) => {
                if col == 0xE {
                    Command::LD_Reg_Mem
                } else {
                    Command::LD_Reg_Reg
                }
            }
            (0x8, 0x0..=0x7) => {
                if col != 0x6 {
                    Command::ADD_Reg
                } else {
                    Command::ADD_Mem
                }
            }
            (0x8, 0x8..=0xF) => {
                if col != 0xE {
                    Command::ADC_Reg
                } else {
                    Command::ADC_Mem
                }
            }
            (0x9, 0x0..=0x7) => {
                if col != 0x6 {
                    Command::SUB_Reg
                } else {
                    Command::SUB_Mem
                }
            }
            (0x9, 0x8..=0xF) => {
                if col != 0xE {
                    Command::SBC_Reg
                } else {
                    Command::SBC_Mem
                }
            }
            (0xA, _) => {
                if col != 0xE && col != 0x6 {
                    Command::Bitwise_Reg
                } else {
                    Command::Bitwise_Mem
                }
            }
            (0xB, 0x0..=0x7) => {
                if col != 0x6 {
                    Command::Bitwise_Reg
                } else {
                    Command::Bitwise_Mem
                }
            }
            (0xB, 0x8..=0xF) => {
                if col != 0xE {
                    Command::CP_Reg
                } else {
                    Command::CP_Mem
                }
            }
            (0xC, 0x9) => Command::RET,
            (0xC, 0x8) => Command::RET_Eq_Zero,
            (0xD, 0x8) => Command::RET_Eq_Carry,
            (0xC, 0x0) => Command::RET_Not_Eq_Zero,
            (0xD, 0x0) => Command::RET_Not_Eq_Carry,
            (0xC, 0x3) => Command::JP,
            (0xE, 0x9) => Command::JP_Mem,
            (0xC, 0xA) => Command::JP_Eq_Zero,
            (0xD, 0xA) => Command::JP_Eq_Carry,
            (0xC, 0x2) => Command::JP_Not_Eq_Zero,
            (0xD, 0x2) => Command::JP_Not_Eq_Carry,
            (0xC, 0xD) => Command::CALL,
            (0xC, 0xC) => Command::CALL_Eq_Zero,
            (0xD, 0xC) => Command::CALL_Eq_Carry,
            (0xC, 0x4) => Command::CALL_Not_Eq_Zero,
            (0xD, 0x4) => Command::CALL_Not_Eq_Carry,
            (0xC, 0xB) => Command::CB,

            (0xC..=0xF, 0x1) => Command::POP,
            (0xC..=0xF, 0x5) => Command::PUSH,
            (0xC, 0x6) => Command::ADD_8Bit,
            (0xC, 0xE) => Command::ADC_8Bit,
            (0xD, 0x6) => Command::SUB_8Bit,
            (0xD, 0xE) => Command::SBC_8Bit,
            (0xE, 0x6 | 0xE) => Command::Bitwise_8Bit,
            (0xF, 0x6) => Command::Bitwise_8Bit,
            (0xF, 0xE) => Command::CP_8Bit,
            (0xC..=0xF, 0x7 | 0xF) => Command::RST,
            (0xE, 0x0) => Command::LD_Reg_Addr_8bit,
            (0xF, 0x0) => Command::LD_Addr_8bit_Reg,
            (0xE, 0xA) => Command::LD_Reg_Addr_16Bit,
            (0xF, 0xA) => Command::LD_Addr_16bit_Reg,
            (0xE, 0x8) => Command::ADD_SP_Signed,
            (0xF, 0x8) => Command::LD_SP_Signed_HL,
            (0xF, 0x9) => Command::LD_HL_SP,
            (0xE, 0x2) => Command::LD_A_C,
            (0xF, 0x2) => Command::LD_C_A,
            _ => Command::None,
        }
    }

    pub fn get_instruction_cb(opcode: u8) -> Self {
        let (row, col) = (opcode >> 4, opcode & 0xF);
        match (row, col) {
            (0x0, 0x0..=0x5 | 0x7) => Command::RLC_Reg,
            (0x0, 0x6) => Command::RLC_Mem,
            (0x0, 0x8..=0xD | 0xF) => Command::RRC_Reg,
            (0x0, 0xE) => Command::RRC_Mem,
            (0x1, 0x0..=0x5 | 0x7) => Command::RL_Reg,
            (0x1, 0x6) => Command::RL_Mem,
            (0x1, 0x8..=0xD | 0xF) => Command::RR_Reg,
            (0x1, 0xE) => Command::RR_Mem,
            (0x2, 0x0..=0x5 | 0x7) => Command::SLA_Reg,
            (0x2, 0x6) => Command::SLA_Mem,
            (0x2, 0x8..=0xD | 0xF) => Command::SRA_Reg,
            (0x2, 0xE) => Command::SRA_Mem,
            (0x3, 0x0..=0x5 | 0x7) => Command::Swap_Reg,
            (0x3, 0x6) => Command::Swap_Mem,
            (0x3, 0x8..=0xD | 0xF) => Command::SLA_Reg,
            (0x3, 0xE) => Command::SLA_Mem,
            (0x4..=0x7, 0x0..=0x5 | 0x7..=0xD | 0xF) => Command::BIT_Reg,
            (0x4..=0x7, 0x6 | 0xE) => Command::BIT_Mem,
            (0x8..=0xB, 0x0..=0x5 | 0x7..=0xD | 0xF) => Command::RES_Reg,
            (0x8..=0xB, 0x6 | 0xE) => Command::RES_Mem,
            (0xC..=0xF, 0x0..=0x5 | 0x7..=0xD | 0xF) => Command::SET_Reg,
            (0xC..=0xF, 0x6 | 0xE) => Command::SET_Mem,
            _ => Command::None,
        }
    }
}
