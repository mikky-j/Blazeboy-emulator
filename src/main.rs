use std::fs;

use blazeboy::Cpu;
use blazeboy::Memory;

struct BlazeBoy {
    memory: Memory,
    cpu: Cpu,
}

impl BlazeBoy {
    fn new() -> Self {
        let memory = Memory::new();
        let cpu = Cpu::new();

        BlazeBoy { memory, cpu }
    }
}

use serde::Deserialize;
use serde_json::{Result, Value};

struct Test {
    code: String,
    instruction: String,
    arguments: Vec<String>,
    length: String,
    cycle: u8,
    false_cycle: u8,
    flags: Vec<String>,
}

fn main() {
    // let mut blazeboy = BlazeBoy::new();
    // blazeboy.cpu.step(&mut blazeboy.memory);
    // println!("{}", blazeboy.cpu.registers.pc);
    let file = fs::read_to_string("src/cpu/instructions.json").unwrap();
    let file_str = file.as_str();

    let res: Value = serde_json::from_str(file_str).unwrap();
    for v in res.as_array().unwrap() {
        println!("{:?}", v);
    }
}
