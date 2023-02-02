mod rv32i;

use crate::{
    cause::Cause, instruction::Instruction, memory::Memory, mode::PrivilegeMode,
    pc::ProgramCounter, x::IntegerRegister,
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
        memory: &mut Memory,
    ) -> Result<(), Cause>;
}
