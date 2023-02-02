mod bitops;
mod cause;
mod decoder;
mod executor;
mod instruction;
mod memory;
mod mode;
mod pc;
mod x;

use memory::Memory;
use pc::ProgramCounter;

fn main() {
    let mut simulator = Simulator::default();
    let terminator = |simulator: &Simulator| {
        let value = simulator.memory.load(0x80001000);
        if value == 1 {
            Some(1)
        } else {
            None
        }
    };
    let result = simulator.run(terminator);
    println!("{}", if result == 1 { "PASS" } else { "FAIL" });
}

#[derive(Default)]
pub struct Simulator {
    pc: ProgramCounter,
    memory: Memory,
}

impl Simulator {
    fn run(&mut self, terminator: impl Fn(&Simulator) -> Option<u64>) -> u64 {
        while self.pc.read() < self.memory.size() {
            let address = self.pc.read();
            let instruction = self.memory.load(address);

            // TODO: decode and execute the instruction

            if let Some(result) = terminator(self) {
                return result;
            }

            self.pc.increment();
        }
        0
    }
}
