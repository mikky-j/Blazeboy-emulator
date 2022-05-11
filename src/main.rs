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

fn main() {
    let mut blazeboy = BlazeBoy::new();
    blazeboy.cpu.step(&mut blazeboy.memory);
    println!("{}", blazeboy.cpu.registers.pc);
}
