mod bitops;
mod cause;
mod csr;
mod decoder;
mod executor;
mod instruction;
mod memory;
mod mode;
mod pc;
mod trap_handler;
mod x;

use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

use crate::{
    csr::ControlAndStatusRegister,
    decoder::{
        privileged::PrivilegedDecoder, rv32i::Rv32iDecoder, rv64i::Rv64iDecoder,
        zicsr::ZicsrDecoder, Decoder,
    },
    executor::{
        privileged::PrivilegedExecutor, rv32i::Rv32iExecutor, rv64i::Rv64iExecutor,
        zicsr::ZicsrExecutor, Executor,
    },
    memory::{Memory, MEMORY_BASE_ADDRESS},
    mode::PrivilegeMode,
    pc::ProgramCounter,
    trap_handler::handle_cause,
    x::IntegerRegister,
};

fn main() -> Result<()> {
    let file = File::open("./riscv-tests/isa/rv64ui-p-add.bin")?;
    let mut simulator = Simulator::default();
    simulator.load(file)?;
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
    Ok(())
}

#[derive(Default)]
pub struct Simulator {
    prv: PrivilegeMode,
    pc: ProgramCounter,
    x: IntegerRegister,
    csr: ControlAndStatusRegister,
    memory: Memory,
}

impl Simulator {
    pub fn load(&mut self, file: File) -> Result<()> {
        let buffer = BufReader::new(file);
        for (address, byte) in buffer.bytes().enumerate() {
            self.memory
                .store(address as u64 + MEMORY_BASE_ADDRESS, byte? as u32);
        }
        Ok(())
    }

    fn run(&mut self, terminator: impl Fn(&Simulator) -> Option<u64>) -> u64 {
        while self.pc.read() < self.memory.size() {
            let address = self.pc.read();
            let instruction = self.memory.load(address);

            let result = if let Some(decoded) = PrivilegedDecoder::decode(instruction) {
                PrivilegedExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.memory,
                )
            } else if let Some(decoded) = Rv32iDecoder::decode(instruction) {
                Rv32iExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.memory,
                )
            } else if let Some(decoded) = Rv64iDecoder::decode(instruction) {
                Rv64iExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.memory,
                )
            } else if let Some(decoded) = ZicsrDecoder::decode(instruction) {
                ZicsrExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.memory,
                )
            } else {
                // end the loop when unable to decode the instruction
                break;
            };

            if let Some(result) = terminator(self) {
                return result;
            }

            // handle the trap
            if let Err(cause) = result {
                let (prv, pc) =
                    handle_cause(&cause, self.pc.read(), instruction, self.prv, &mut self.csr);
                self.prv = prv;
                self.pc.jump(pc);
            }
            // increment the pc when the pc has not been updated
            else if self.pc.read() == address {
                self.pc.increment();
            }
        }
        0
    }
}
