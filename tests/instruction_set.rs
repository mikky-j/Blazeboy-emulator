#[cfg(test)]
mod instruction_test {

    use blazeboy::{
        bus_read, get_bit, BitwiseOperator, CpuRegisters, Flag, Instruction, Memory, Register16Bit,
        Register8Bit,
    };
    const REGISTER_ARR: [Register8Bit; 8] = [
        Register8Bit::A,
        Register8Bit::B,
        Register8Bit::C,
        Register8Bit::D,
        Register8Bit::E,
        Register8Bit::F,
        Register8Bit::H,
        Register8Bit::L,
    ];
    const REGISTER_ARR_16BIT: [Register16Bit; 4] = [
        Register16Bit::AF,
        Register16Bit::BC,
        Register16Bit::DE,
        Register16Bit::HL,
    ];

    fn test_flag(instruction: &Instruction, flags: &[Flag]) {
        for flag in flags {
            match flag {
                Flag::Carry(v) => assert_eq!(
                    *v,
                    get_bit(instruction.registers.f, 4) == 1,
                    "Testing the Carry flag"
                ),
                Flag::HalfCarry(v) => assert_eq!(
                    *v,
                    get_bit(instruction.registers.f, 5) == 1,
                    "Tesing the Half Carry flag"
                ),
                Flag::Subtraction(v) => assert_eq!(
                    *v,
                    get_bit(instruction.registers.f, 6) == 1,
                    "Testing the Subtraction Flag"
                ),
                Flag::Zero(v) => assert_eq!(
                    *v,
                    get_bit(instruction.registers.f, 7) == 1,
                    "Testing the Zero Flag"
                ),
                _ => (),
            }
        }
    }

    fn check_instruction_props(instruction: &Instruction, length: u8, cycle: u8) {
        assert_eq!(
            instruction.length, length,
            "The length of the instruction is {} and we were expecting this {}",
            instruction.length, length
        );
        assert_eq!(
            instruction.cycle, cycle,
            "The cycle of the instruction is {} and we were expecting this {}",
            instruction.cycle, cycle
        );
    }

    #[test]
    pub fn test_ld_reg_to_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        for i in 0..8 {
            let r = REGISTER_ARR[i];
            for j in 0..8 {
                let d = REGISTER_ARR[j];
                instruction.registers.set_random_number_reg(r);
                instruction.ld_reg_reg(d, r);
                assert_eq!(
                    instruction.registers.get_8bit_reg_value(r),
                    instruction.registers.get_8bit_reg_value(d)
                );
                instruction.registers.clear_reg(d);
            }
            instruction.registers.clear_reg(r);
        }
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    pub fn test_ld_reg_to_mem_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        for i in 0..8 {
            let r = REGISTER_ARR[i];
            instruction.registers.set_random_number_reg(r);
            instruction.ld_reg_to_mem_reg(r, Register16Bit::HL);
            assert_eq!(
                instruction.registers.get_8bit_reg_value(r),
                instruction.memory.data
                    [instruction.registers.get_16bit_reg_value(Register16Bit::HL) as usize]
            );
        }
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_push() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        for reg in REGISTER_ARR_16BIT {
            instruction
                .registers
                .set_random_number_reg_16bit(Register16Bit::SP);
            instruction.registers.set_random_number_reg_16bit(reg);
            instruction.push_16bit_reg(reg);
            let value1 = bus_read(&instruction.memory, instruction.registers.sp + 1).unwrap();
            let value2 = bus_read(&instruction.memory, instruction.registers.sp).unwrap();
            let value = (value1 as u16) << 8 | value2 as u16;
            assert_eq!(instruction.registers.get_16bit_reg_value(reg), value);
        }
        check_instruction_props(&instruction, 1, 16);
    }

    #[test]
    fn test_pop() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        for reg in REGISTER_ARR_16BIT {
            instruction
                .registers
                .set_random_number_reg_16bit(Register16Bit::SP);
            let value1 = bus_read(&instruction.memory, instruction.registers.sp + 1).unwrap();
            let value2 = bus_read(&instruction.memory, instruction.registers.sp).unwrap();
            let value = (value1 as u16) << 8 | value2 as u16;
            instruction.pop_16bit_reg(reg);

            assert_eq!(instruction.registers.get_16bit_reg_value(reg), value);
        }
        check_instruction_props(&instruction, 1, 12);
    }

    #[test]
    fn test_add() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.a = 0x3a;
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0xc6);
        instruction.add_reg_to_reg_8bit(Register8Bit::B);
        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Carry(true),
            Flag::Subtraction(false),
        ];
        assert_eq!(instruction.registers.a, 0);
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4)
    }

    #[test]
    fn test_adc() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.a = 0xe1;
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x0f);
        instruction
            .registers
            .ld_8bit_reg(Register8Bit::F, 0b0010000);

        instruction.adc_reg_to_reg_8bit(Register8Bit::B);
        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(true),
            Flag::Carry(false),
            Flag::Subtraction(false),
        ];
        test_flag(&instruction, &correct_flags);
        assert_eq!(instruction.registers.a, 0xf1);
        check_instruction_props(&instruction, 1, 4)
    }

    #[test]
    fn test_sub() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.a = 0x3e;
        instruction.registers.e = 0x40;

        instruction.sub_reg_to_reg_8bit(Register8Bit::E);

        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(true),
            Flag::Carry(true),
        ];

        test_flag(&instruction, &correct_flags);
        assert_eq!(instruction.registers.a, 0xfe);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_sbc() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.a = 0x3b;
        instruction.registers.e = 0x4f;
        instruction.registers.set_flag(Flag::Carry(true));

        instruction.sbc_reg_to_reg_8bit(Register8Bit::E);

        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(true),
            Flag::Subtraction(true),
            Flag::Carry(true),
        ];

        test_flag(&instruction, &correct_flags);
        assert_eq!(instruction.registers.a, 0xeb);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_and() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x5a);
        instruction.registers.ld_8bit_reg(Register8Bit::L, 0);

        instruction.bitwise_reg_8bit(Register8Bit::L, BitwiseOperator::And);

        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Subtraction(false),
            Flag::Carry(false),
        ];

        test_flag(&instruction, &correct_flags);
        assert_eq!(instruction.registers.a, 0);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_or() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x5a);

        instruction.bitwise_reg_8bit(Register8Bit::A, BitwiseOperator::Or);

        let correct_flags = [Flag::Zero(false)];

        test_flag(&instruction, &correct_flags);
        assert_eq!(instruction.registers.a, 0x5a);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_xor() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0xFF);
        instruction.registers.ld_8bit_reg(Register8Bit::L, 0xf);

        instruction.bitwise_reg_8bit(Register8Bit::L, BitwiseOperator::Xor);

        let correct_flags = [Flag::Zero(false)];

        test_flag(&instruction, &correct_flags);
        assert_eq!(instruction.registers.a, 0xf0);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_cp() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x3c);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x40);

        instruction.cp_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Carry(true),
            Flag::Subtraction(true),
        ];

        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_inc() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0xff);

        instruction.inc_8bit(Register8Bit::A);

        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Subtraction(false),
        ];

        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_dec() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 1);

        instruction.dec_8bit(Register8Bit::A);

        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(false),
            Flag::Subtraction(true),
        ];

        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_add_16bit_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction
            .registers
            .ld_16bit_reg(Register16Bit::HL, 0x8a23);
        instruction
            .registers
            .ld_16bit_reg(Register16Bit::BC, 0x0605);

        instruction.add_reg_16bit_to_reg_16_bit(Register16Bit::HL);

        let correct_flags = [
            Flag::HalfCarry(true),
            Flag::Carry(true),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_16bit_reg_value(Register16Bit::HL),
            0x1446
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_add_sp_r8() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction
            .registers
            .ld_16bit_reg(Register16Bit::SP, 0xfff8);

        instruction.add_sp_r8(-2);

        let correct_flags = [
            Flag::HalfCarry(false),
            Flag::Carry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_16bit_reg_value(Register16Bit::SP),
            0xfff6
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 16);
    }

    #[test]
    fn test_inc_16bit() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction
            .registers
            .ld_16bit_reg(Register16Bit::SP, 0xffff);

        instruction.inc_16bit(Register16Bit::SP);

        assert_eq!(
            instruction.registers.get_16bit_reg_value(Register16Bit::SP),
            0x0
        );
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_dec_16bit() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction
            .registers
            .ld_16bit_reg(Register16Bit::SP, 0x235f);

        instruction.dec_16bit(Register16Bit::SP);

        assert_eq!(
            instruction.registers.get_16bit_reg_value(Register16Bit::SP),
            0x235e
        );
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_rlca() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x85);

        instruction.rlca();

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::A),
            0x0B
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rla() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.set_flag(Flag::Carry(true));
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x95);

        instruction.rla();

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::A),
            0x2B
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rrca() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x3b);

        instruction.rrca();

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::A),
            0x9D
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rra() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::A, 0x81);

        instruction.rra();

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::A),
            0x40
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rlc() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x85);

        instruction.rlc_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0x0B
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_rl() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x11);

        instruction.rl_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0x22
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_rrc() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x1);

        instruction.rrc_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0x80
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_rr() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x1);

        instruction.rr_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(true),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(instruction.registers.get_8bit_reg_value(Register8Bit::B), 0);
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_sla() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0xff);

        instruction.sla_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0xfe
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_srl() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0xff);

        instruction.srl_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0x7f
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_sra() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x8a);

        instruction.sra_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0xc5
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_swap() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0xf0);

        instruction.swap_reg_8bit(Register8Bit::B);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0x0f
        );
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_bit() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0xef);

        instruction.bit_reg_8bit(Register8Bit::B, 4);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Subtraction(false),
        ];
        test_flag(&instruction, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_set() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x80);

        instruction.set_reg_8bit(Register8Bit::B, 3);

        assert_eq!(
            instruction.registers.get_8bit_reg_value(Register8Bit::B),
            0x88
        );
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_res() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new(&mut registers, &mut memory);
        instruction.registers.ld_8bit_reg(Register8Bit::B, 0x80);

        instruction.res_reg_8bit(Register8Bit::B, 7);

        assert_eq!(instruction.registers.get_8bit_reg_value(Register8Bit::B), 0);
        check_instruction_props(&instruction, 2, 8);
    }
}
