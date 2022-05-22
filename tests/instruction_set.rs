#[cfg(test)]
mod instruction_test {

    use blazeboy::{
        bus_read, get_bit, BitwiseOperator, CpuRegisters, Flag, Instruction, Memory, Register16Bit,
        Register8Bit,
    };
    use rand::Rng;

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
    const REGISTER_ARR_16BIT: [Register16Bit; 6] = [
        Register16Bit::AF,
        Register16Bit::BC,
        Register16Bit::DE,
        Register16Bit::HL,
        Register16Bit::SP,
        Register16Bit::PC,
    ];

    fn test_flag(registers: &CpuRegisters, flags: &[Flag]) {
        for flag in flags {
            match flag {
                Flag::Carry(v) => {
                    assert_eq!(*v, get_bit(registers.f, 4) == 1, "Testing the Carry flag")
                }
                Flag::HalfCarry(v) => assert_eq!(
                    *v,
                    get_bit(registers.f, 5) == 1,
                    "Tesing the Half Carry flag"
                ),
                Flag::Subtraction(v) => assert_eq!(
                    *v,
                    get_bit(registers.f, 6) == 1,
                    "Testing the Subtraction Flag"
                ),
                Flag::Zero(v) => {
                    assert_eq!(*v, get_bit(registers.f, 7) == 1, "Testing the Zero Flag")
                }
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
        let mut instruction = Instruction::new();
        for i in 0..8 {
            let r = REGISTER_ARR[i];
            for j in 0..8 {
                let d = REGISTER_ARR[j];
                registers.set_random_number_reg(r);
                instruction.ld_reg_reg(&mut registers, d, r);
                assert_eq!(
                    registers.get_8bit_reg_value(r),
                    registers.get_8bit_reg_value(d)
                );
                registers.clear_reg(d);
            }
            registers.clear_reg(r);
        }
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    pub fn test_ld_reg_to_mem_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        for i in 0..8 {
            let r = REGISTER_ARR[i];
            registers.set_random_number_reg(r);
            registers.set_random_number_reg_16bit(Register16Bit::HL);
            let addr = registers.get_16bit_reg_value(Register16Bit::HL);
            instruction.ld_reg_to_mem_reg(&mut registers, &mut memory, r, Register16Bit::HL);
            assert_eq!(
                registers.get_8bit_reg_value(r),
                bus_read(&memory, addr).unwrap()
            );
        }
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_8bit_reg() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        for reg in REGISTER_ARR {
            let random_value = thread_rng.gen::<u8>();
            instruction.ld_reg_8bit(&mut registers, reg, random_value);
            assert_eq!(registers.get_8bit_reg_value(reg), random_value);
        }
        check_instruction_props(&&instruction, 2, 8);
    }

    #[test]
    fn test_ld_mem_reg_to_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        for reg in REGISTER_ARR {
            registers.set_random_number_reg(reg);
            registers.set_random_number_reg_16bit(Register16Bit::HL);
            let addr = registers.get_16bit_reg_value(Register16Bit::HL);
            memory.set_random_number_at_addr(addr);
            instruction.ld_mem_reg_to_reg(&mut registers, &mut memory, reg, Register16Bit::HL);
            assert_eq!(
                registers.get_8bit_reg_value(reg),
                bus_read(&memory, addr).unwrap()
            );
        }
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_8bit_to_mem_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let random_value = thread_rng.gen::<u8>();

        registers.set_random_number_reg_16bit(Register16Bit::HL);
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        memory.set_random_number_at_addr(addr);
        instruction.ld_8bit_into_mem(&mut registers, &mut memory, random_value);
        assert_eq!(random_value, bus_read(&memory, addr).unwrap());
        check_instruction_props(&instruction, 2, 12);
    }

    #[test]
    fn test_ld_reg_16bit_to_reg_8bit() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        for reg in REGISTER_ARR_16BIT {
            registers.set_random_number_reg(Register8Bit::A);
            registers.set_random_number_reg_16bit(reg);
            let addr = registers.get_16bit_reg_value(reg);
            memory.set_random_number_at_addr(addr);
            instruction.ld_mem_reg_to_reg(&mut registers, &mut memory, Register8Bit::A, reg);
            assert_eq!(
                registers.get_8bit_reg_value(Register8Bit::A),
                bus_read(&memory, addr).unwrap()
            );
        }
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_c_into_a() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        registers.set_random_number_reg(Register8Bit::C);
        let addr = 0xFF00 + registers.c as u16;
        memory.set_random_number_at_addr(addr);
        instruction.ld_c_a(&mut registers, &mut memory);
        assert_eq!(
            registers.get_8bit_reg_value(Register8Bit::A),
            bus_read(&memory, addr).unwrap()
        );

        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_ld_a_into_c() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        registers.set_random_number_reg(Register8Bit::A);
        registers.set_random_number_reg(Register8Bit::C);
        let addr = 0xFF00 + registers.c as u16;
        memory.set_random_number_at_addr(addr);
        instruction.ld_a_c(&mut registers, &mut memory);
        assert_eq!(
            registers.get_8bit_reg_value(Register8Bit::A),
            bus_read(&memory, addr).unwrap()
        );

        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_ld_mem_to_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let random_address = thread_rng.gen::<u16>();
        memory.set_random_number_at_addr(random_address);

        instruction.ld_mem_to_reg(&mut registers, &mut memory, Register8Bit::A, random_address);
        assert_eq!(
            registers.get_8bit_reg_value(Register8Bit::A),
            bus_read(&memory, random_address).unwrap()
        );

        check_instruction_props(&instruction, 3, 16);
    }

    #[test]
    fn test_ld_reg_to_mem() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let random_address = thread_rng.gen::<u16>();
        memory.set_random_number_at_addr(random_address);

        instruction.ld_reg_to_mem(&mut registers, &mut memory, Register8Bit::A, random_address);
        assert_eq!(
            registers.get_8bit_reg_value(Register8Bit::A),
            bus_read(&memory, random_address).unwrap()
        );

        check_instruction_props(&instruction, 3, 16);
    }

    #[test]
    fn test_ld_8bit_addr_to_reg() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let random_address = thread_rng.gen::<u8>();
        memory.set_random_number_at_addr(0xFF00 + random_address as u16);

        instruction.ld_8bit_addr_to_reg_8bit(&mut registers, &mut memory, random_address);
        assert_eq!(
            registers.get_8bit_reg_value(Register8Bit::A),
            bus_read(&memory, 0xFF00 + random_address as u16).unwrap()
        );

        check_instruction_props(&instruction, 2, 12);
    }

    #[test]
    fn test_ld_reg_8bit_to_addr_8bit() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        registers.set_random_number_reg(Register8Bit::A);
        let random_address = thread_rng.gen::<u8>();

        instruction.ld_reg_8bit_to_addr_8bit(&mut registers, &mut memory, random_address);
        assert_eq!(
            registers.get_8bit_reg_value(Register8Bit::A),
            bus_read(&memory, 0xFF00 + random_address as u16).unwrap()
        );

        check_instruction_props(&instruction, 2, 12);
    }

    #[test]
    fn test_ld_hld() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        registers.set_random_number_reg_16bit(Register16Bit::HL);
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        let value = bus_read(&memory, addr).unwrap();

        instruction.ld_hld(&mut registers, &mut memory);

        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), value);
        assert_eq!(
            addr.wrapping_sub(1),
            registers.get_16bit_reg_value(Register16Bit::HL)
        );

        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_hli() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        registers.set_random_number_reg_16bit(Register16Bit::HL);
        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        memory.set_random_number_at_addr(addr);
        let value = bus_read(&memory, addr).unwrap();

        instruction.ld_hli(&mut registers, &mut memory);

        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), value);
        assert_eq!(
            registers.get_16bit_reg_value(Register16Bit::HL),
            addr.wrapping_add(1)
        );

        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_mem_hli() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        registers.set_random_number_reg(Register8Bit::A);
        registers.set_random_number_reg_16bit(Register16Bit::HL);

        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        instruction.ld_mem_hli(&mut registers, &mut memory);
        let value = bus_read(&memory, addr).unwrap();

        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), value);
        assert_eq!(
            registers.get_16bit_reg_value(Register16Bit::HL),
            addr.wrapping_add(1)
        );

        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_mem_hld() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();

        registers.set_random_number_reg(Register8Bit::A);
        registers.set_random_number_reg_16bit(Register16Bit::HL);

        let addr = registers.get_16bit_reg_value(Register16Bit::HL);
        instruction.ld_mem_hld(&mut registers, &mut memory);
        let value = bus_read(&memory, addr).unwrap();

        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), value);
        assert_eq!(
            registers.get_16bit_reg_value(Register16Bit::HL),
            addr.wrapping_sub(1)
        );

        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_16bit_to_reg_16bit() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();
        for reg in REGISTER_ARR_16BIT {
            let rand_number = thread_rng.gen::<u16>();
            instruction.ld_reg_16bit(&mut registers, reg, rand_number);
            let value = registers.get_16bit_reg_value(reg);
            assert_eq!(value, rand_number);
        }

        check_instruction_props(&instruction, 3, 12);
    }

    #[test]
    fn test_ld_hl_sp() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.set_random_number_reg_16bit(Register16Bit::HL);
        let value = registers.get_16bit_reg_value(Register16Bit::HL);

        instruction.ld_hl_to_sp(&mut registers);
        assert_eq!(registers.get_16bit_reg_value(Register16Bit::SP), value);

        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_ld_sp_to_mem() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let addr = thread_rng.gen::<u16>();

        registers.set_random_number_reg_16bit(Register16Bit::SP);
        let [hi, lo] = registers
            .get_16bit_reg_value(Register16Bit::SP)
            .to_be_bytes();
        instruction.ld_sp_to_mem(&mut registers, &mut memory, addr);
        let val1 = bus_read(&memory, addr).unwrap();
        let val2 = bus_read(&memory, addr.wrapping_add(1)).unwrap();
        assert_eq!(lo, val1);
        assert_eq!(hi, val2);

        check_instruction_props(&instruction, 3, 20);
    }

    #[test]
    fn test_push() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        for reg in REGISTER_ARR_16BIT {
            match reg {
                Register16Bit::SP => (),
                _ => {
                    registers.set_random_number_reg_16bit(Register16Bit::SP);
                    registers.set_random_number_reg_16bit(reg);
                    instruction.push_16bit_reg(&mut registers, &mut memory, reg);
                    let value1 = bus_read(&memory, registers.sp + 1).unwrap();
                    let value2 = bus_read(&memory, registers.sp).unwrap();
                    let value = (value1 as u16) << 8 | value2 as u16;
                    assert_eq!(registers.get_16bit_reg_value(reg), value);
                }
            }
        }
        check_instruction_props(&instruction, 1, 16);
    }

    #[test]
    fn test_pop() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        for reg in REGISTER_ARR_16BIT {
            match reg {
                Register16Bit::SP => (),
                _ => {
                    registers.set_random_number_reg_16bit(Register16Bit::SP);
                    let value1 = bus_read(&memory, registers.sp + 1).unwrap();
                    let value2 = bus_read(&memory, registers.sp).unwrap();
                    let value = (value1 as u16) << 8 | value2 as u16;
                    instruction.pop_16bit_reg(&mut registers, &mut memory, reg);

                    assert_eq!(registers.get_16bit_reg_value(reg), value);
                }
            }
        }
        check_instruction_props(&instruction, 1, 12);
    }

    #[test]
    fn test_add() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.a = 0x3a;
        registers.ld_8bit_reg(Register8Bit::B, 0xc6);
        instruction.add_reg_to_reg_8bit(&mut registers, Register8Bit::B);
        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Carry(true),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.a, 0);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4)
    }

    #[test]
    fn test_adc() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.a = 0xe1;
        registers.ld_8bit_reg(Register8Bit::B, 0x0f);
        registers.ld_8bit_reg(Register8Bit::F, 0b0010000);

        instruction.adc_reg_to_reg_8bit(&mut registers, Register8Bit::B);
        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(true),
            Flag::Carry(false),
            Flag::Subtraction(false),
        ];
        test_flag(&registers, &correct_flags);
        assert_eq!(registers.a, 0xf1);
        check_instruction_props(&instruction, 1, 4)
    }

    #[test]
    fn test_sub() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.a = 0x3e;
        registers.e = 0x40;

        instruction.sub_reg_to_reg_8bit(&mut registers, Register8Bit::E);

        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(true),
            Flag::Carry(true),
        ];

        test_flag(&registers, &correct_flags);
        assert_eq!(registers.a, 0xfe);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_sbc() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.a = 0x3b;
        registers.e = 0x4f;
        registers.set_flag(Flag::Carry(true));

        instruction.sbc_reg_to_reg_8bit(&mut registers, Register8Bit::E);

        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(true),
            Flag::Subtraction(true),
            Flag::Carry(true),
        ];

        test_flag(&registers, &correct_flags);
        assert_eq!(registers.a, 0xeb);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_and() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0x5a);
        registers.ld_8bit_reg(Register8Bit::L, 0);

        instruction.bitwise_reg_8bit(&mut registers, Register8Bit::L, BitwiseOperator::And);

        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Subtraction(false),
            Flag::Carry(false),
        ];

        test_flag(&registers, &correct_flags);
        assert_eq!(registers.a, 0);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_or() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0x5a);

        instruction.bitwise_reg_8bit(&mut registers, Register8Bit::A, BitwiseOperator::Or);

        let correct_flags = [Flag::Zero(false)];

        test_flag(&registers, &correct_flags);
        assert_eq!(registers.a, 0x5a);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_xor() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0xFF);
        registers.ld_8bit_reg(Register8Bit::L, 0xf);

        instruction.bitwise_reg_8bit(&mut registers, Register8Bit::L, BitwiseOperator::Xor);

        let correct_flags = [Flag::Zero(false)];

        test_flag(&registers, &correct_flags);
        assert_eq!(registers.a, 0xf0);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_cp() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0x3c);
        registers.ld_8bit_reg(Register8Bit::B, 0x40);

        instruction.cp_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Carry(true),
            Flag::Subtraction(true),
        ];

        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_inc() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0xff);

        instruction.inc_8bit(&mut registers, Register8Bit::A);

        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Subtraction(false),
        ];

        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_dec() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 1);

        instruction.dec_8bit(&mut registers, Register8Bit::A);

        let correct_flags = [
            Flag::Zero(true),
            Flag::HalfCarry(false),
            Flag::Subtraction(true),
        ];

        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_add_16bit_reg() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_16bit_reg(Register16Bit::HL, 0x8a23);
        registers.ld_16bit_reg(Register16Bit::BC, 0x0605);

        instruction.add_reg_16bit_to_reg_16_bit(&mut registers, Register16Bit::HL);

        let correct_flags = [
            Flag::HalfCarry(true),
            Flag::Carry(true),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_16bit_reg_value(Register16Bit::HL), 0x1446);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_add_sp_r8() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_16bit_reg(Register16Bit::SP, 0xfff8);

        instruction.add_sp_r8(&mut registers, -2);

        let correct_flags = [
            Flag::HalfCarry(false),
            Flag::Carry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_16bit_reg_value(Register16Bit::SP), 0xfff6);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 16);
    }

    #[test]
    fn test_inc_16bit() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_16bit_reg(Register16Bit::SP, 0xffff);

        instruction.inc_16bit(&mut registers, Register16Bit::SP);

        assert_eq!(registers.get_16bit_reg_value(Register16Bit::SP), 0x0);
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_dec_16bit() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_16bit_reg(Register16Bit::SP, 0x235f);

        instruction.dec_16bit(&mut registers, Register16Bit::SP);

        assert_eq!(registers.get_16bit_reg_value(Register16Bit::SP), 0x235e);
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_rlca() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0x85);

        instruction.rlca(&mut registers);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), 0x0B);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rla() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.set_flag(Flag::Carry(true));
        registers.ld_8bit_reg(Register8Bit::A, 0x95);

        instruction.rla(&mut registers);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), 0x2B);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rrca() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0x3b);

        instruction.rrca(&mut registers);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), 0x9D);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rra() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::A, 0x81);

        instruction.rra(&mut registers);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::A), 0x40);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_rlc() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x85);

        instruction.rlc_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0x0B);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_rl() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x11);

        instruction.rl_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0x22);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_rrc() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x1);

        instruction.rrc_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0x80);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_rr() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x1);

        instruction.rr_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(true),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_sla() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0xff);

        instruction.sla_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0xfe);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_srl() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0xff);

        instruction.srl_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(true),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0x7f);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_sra() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x8a);

        instruction.sra_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0xc5);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_swap() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0xf0);

        instruction.swap_reg_8bit(&mut registers, Register8Bit::B);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(false),
            Flag::HalfCarry(false),
            Flag::Subtraction(false),
        ];
        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0x0f);
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_bit() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0xef);

        instruction.bit_reg_8bit(&mut registers, Register8Bit::B, 4);

        let correct_flags = [
            Flag::Carry(false),
            Flag::Zero(true),
            Flag::HalfCarry(true),
            Flag::Subtraction(false),
        ];
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_set() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x80);

        instruction.set_reg_8bit(&mut registers, Register8Bit::B, 3);

        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0x88);
        check_instruction_props(&instruction, 2, 8);
    }

    #[test]
    fn test_res() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.ld_8bit_reg(Register8Bit::B, 0x80);

        instruction.res_reg_8bit(&mut registers, Register8Bit::B, 7);

        assert_eq!(registers.get_8bit_reg_value(Register8Bit::B), 0);
        check_instruction_props(&instruction, 2, 8);
    }

    //-----------------------Tests for JUMP Instructions------------------------------

    #[test]
    fn test_jp() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let addr = thread_rng.gen::<u16>();
        instruction.jp(&mut registers, addr);
        assert_eq!(registers.pc, addr, "Test for normal jump");
        check_instruction_props(&instruction, 0, 16);

        registers.set_flags(&[Flag::Carry(false), Flag::Zero(true)]);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_eq(&mut registers, Flag::Carry(true), addr);
        assert_eq!(registers.pc, addr, "Test for true conditional jump");
        check_instruction_props(&instruction, 0, 16);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_eq(&mut registers, Flag::Carry(false), addr);
        check_instruction_props(&instruction, 3, 12);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_eq(&mut registers, Flag::Zero(true), addr);
        assert_eq!(registers.pc, addr, "Test for true conditional jump");
        check_instruction_props(&instruction, 0, 16);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_eq(&mut registers, Flag::Zero(false), addr);
        check_instruction_props(&instruction, 3, 12);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_not_eq(&mut registers, Flag::Zero(false), addr);
        assert_eq!(registers.pc, addr, "Test for false conditional jump");
        check_instruction_props(&instruction, 0, 16);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_not_eq(&mut registers, Flag::Carry(false), addr);
        assert_eq!(registers.pc, addr, "Test for false conditional jump");
        check_instruction_props(&instruction, 0, 16);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_not_eq(&mut registers, Flag::Zero(true), addr);
        check_instruction_props(&instruction, 3, 12);

        let addr = thread_rng.gen::<u16>();
        instruction.jp_not_eq(&mut registers, Flag::Carry(true), addr);
        check_instruction_props(&instruction, 3, 12);
    }

    #[test]
    fn test_call() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        // Testing if it can load into memory coz it has to be correct
        // Not testing for the rest because it is probably correct ;)
        instruction.call(&mut registers, &mut memory, addr);
        let hi = bus_read(&memory, registers.sp + 1).unwrap();
        let lo = bus_read(&memory, registers.sp).unwrap();
        let res = (hi as u16) << 8 | (lo as u16);
        assert_eq!(
            rand_pc_value, res,
            "Testing if the value was loaded correctly into memory"
        );
        assert_eq!(registers.pc, addr, "Test if I jumped correctly");
        check_instruction_props(&instruction, 0, 24);

        registers.set_flags(&[Flag::Carry(false), Flag::Zero(true)]);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_eq(&mut registers, &mut memory, Flag::Carry(true), addr);
        assert_eq!(registers.pc, addr, "Test if I jumped correctly");
        check_instruction_props(&instruction, 0, 24);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_eq(&mut registers, &mut memory, Flag::Zero(true), addr);
        assert_eq!(registers.pc, addr, "Test if I jumped correctly");
        check_instruction_props(&instruction, 0, 24);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_eq(&mut registers, &mut memory, Flag::Zero(false), addr);
        assert_ne!(registers.pc, addr, "Test pc is not changed");
        check_instruction_props(&instruction, 3, 12);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_eq(&mut registers, &mut memory, Flag::Carry(false), addr);
        assert_ne!(registers.pc, addr, "Test pc is not changed");
        check_instruction_props(&instruction, 3, 12);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_not_eq(&mut registers, &mut memory, Flag::Carry(false), addr);
        assert_eq!(registers.pc, addr, "Test if I jumped correctly");
        check_instruction_props(&instruction, 0, 24);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_not_eq(&mut registers, &mut memory, Flag::Zero(false), addr);
        assert_eq!(registers.pc, addr, "Test if I jumped correctly");
        check_instruction_props(&instruction, 0, 24);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        // Using the pop function coz I am too lazy to write it again
        instruction.call_not_eq(&mut registers, &mut memory, Flag::Zero(true), addr);
        assert_ne!(registers.pc, addr, "Test pc is not changed correctly");
        check_instruction_props(&instruction, 3, 12);

        let rand_sp_value = thread_rng.gen::<u16>();
        let rand_pc_value = thread_rng.gen::<u16>();
        let addr = thread_rng.gen::<u16>();
        registers.pc = rand_pc_value;
        registers.sp = rand_sp_value;
        instruction.call_not_eq(&mut registers, &mut memory, Flag::Carry(true), addr);
        assert_ne!(registers.pc, addr, "Test pc is not changed correctly");
        check_instruction_props(&instruction, 3, 12);
    }

    #[test]
    fn test_ret() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret(&mut registers, &mut memory);
        assert_eq!(registers.pc, rand_val);
        check_instruction_props(&instruction, 0, 16);

        registers.set_flags(&[Flag::Carry(true), Flag::Zero(false)]);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_eq(&mut registers, &mut memory, Flag::Carry(true));
        assert_eq!(registers.pc, rand_val);
        check_instruction_props(&instruction, 0, 20);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_eq(&mut registers, &mut memory, Flag::Zero(true));
        assert_eq!(registers.pc, rand_val);
        check_instruction_props(&instruction, 0, 20);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_eq(&mut registers, &mut memory, Flag::Zero(false));
        assert_ne!(registers.pc, rand_val);
        check_instruction_props(&instruction, 1, 8);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_eq(&mut registers, &mut memory, Flag::Carry(false));
        assert_ne!(registers.pc, rand_val);
        check_instruction_props(&instruction, 1, 8);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_not_eq(&mut registers, &mut memory, Flag::Carry(false));
        assert_eq!(registers.pc, rand_val);
        check_instruction_props(&instruction, 0, 20);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_not_eq(&mut registers, &mut memory, Flag::Zero(false));
        assert_eq!(registers.pc, rand_val);
        check_instruction_props(&instruction, 0, 20);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_not_eq(&mut registers, &mut memory, Flag::Zero(true));
        assert_ne!(registers.pc, rand_val);
        check_instruction_props(&instruction, 1, 8);

        let rand_sp = thread_rng.gen::<u16>();
        let rand_val = thread_rng.gen::<u16>();
        registers.sp = rand_sp;
        registers.ld_16bit_reg(Register16Bit::BC, rand_val);
        instruction.push_16bit_reg(&mut registers, &mut memory, Register16Bit::BC);
        instruction.ret_not_eq(&mut registers, &mut memory, Flag::Carry(true));
        assert_ne!(registers.pc, rand_val);
        check_instruction_props(&instruction, 1, 8);
    }

    #[test]
    fn test_rst() {
        let mut registers = CpuRegisters::new();
        let mut memory = Memory::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();

        let instructions: [(u8, u16); 8] = [
            (0xc7, 0),
            (0xd7, 0x10),
            (0xe7, 0x20),
            (0x0f7, 0x30),
            (0xcf, 0x8),
            (0xdf, 0x18),
            (0xef, 0x28),
            (0xff, 0x38),
        ];
        for (opcode, ret_addr) in instructions {
            registers.sp = thread_rng.gen::<u16>();
            instruction.rst(&mut registers, &mut memory, opcode);
            assert_eq!(registers.pc, ret_addr, "Testing if the address are correct");
            check_instruction_props(&instruction, 1, 16);
        }
    }

    #[test]
    fn test_cpl() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();
        let rand_number = thread_rng.gen::<u8>();
        registers.a = rand_number;
        instruction.cpl(&mut registers);
        let correct_flags = [Flag::HalfCarry(true), Flag::Subtraction(true)];
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
        assert_eq!(registers.a, rand_number ^ u8::MAX);
    }

    #[test]
    fn test_ccf() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        let mut thread_rng = rand::thread_rng();
        let carry: bool = thread_rng.gen();
        registers.set_flag(Flag::Carry(carry));
        instruction.ccf(&mut registers);
        let correct_flags = [
            Flag::Carry(!carry),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_scf() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        instruction.scf(&mut registers);
        let correct_flags = [
            Flag::Carry(true),
            Flag::Subtraction(false),
            Flag::HalfCarry(false),
        ];
        test_flag(&registers, &correct_flags);
        check_instruction_props(&instruction, 1, 4);
    }

    #[test]
    fn test_daa() {
        let mut registers = CpuRegisters::new();
        let mut instruction = Instruction::new();
        registers.a = 0x45;
        registers.b = 0x38;
        instruction.adc_reg_to_reg_8bit(&mut registers, Register8Bit::B);
        instruction.daa(&mut registers);
        assert_eq!(registers.a, 0x83);
        test_flag(&registers, &[Flag::Carry(false), Flag::HalfCarry(false)]);

        instruction.sub_reg_to_reg_8bit(&mut registers, Register8Bit::B);
        assert_eq!(registers.a, 0x4b);
        test_flag(&registers, &[Flag::Carry(false), Flag::HalfCarry(true)]);
        check_instruction_props(&instruction, 1, 4);
    }
}
