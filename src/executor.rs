pub mod privileged;
pub mod rv32i;
pub mod rv64i;
pub mod zicsr;

use crate::{
    cause::Cause, csr::ControlAndStatusRegister, instruction::Instruction, memory::Memory,
    mode::PrivilegeMode, pc::ProgramCounter, x::IntegerRegister,
};

pub trait Executor {
    type OpcodeR;
    type OpcodeI;
    type OpcodeS;
    type OpcodeB;
    type OpcodeU;
    type OpcodeJ;

    #[allow(clippy::type_complexity)]
    fn execute(
        instruction: Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
        prv: &PrivilegeMode,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        csr: &mut ControlAndStatusRegister,
        memory: &mut Memory,
    ) -> Result<(), Cause>;
}
