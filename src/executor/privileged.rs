use crate::{
    cause::{Cause, Exception, ExceptionReturn},
    executor::Executor,
    instruction::{
        privileged::{
            PrivilegedOpcodeB, PrivilegedOpcodeI, PrivilegedOpcodeJ, PrivilegedOpcodeR,
            PrivilegedOpcodeS, PrivilegedOpcodeU,
        },
        Instruction,
    },
    memory::Memory,
    mode::PrivilegeMode,
    pc::ProgramCounter,
    x::IntegerRegister,
};

pub struct PrivilegedExecutor;

impl Executor for PrivilegedExecutor {
    type OpcodeR = PrivilegedOpcodeR;
    type OpcodeI = PrivilegedOpcodeI;
    type OpcodeS = PrivilegedOpcodeS;
    type OpcodeB = PrivilegedOpcodeB;
    type OpcodeU = PrivilegedOpcodeU;
    type OpcodeJ = PrivilegedOpcodeJ;

    fn execute(
        instruction: Instruction<
            PrivilegedOpcodeR,
            PrivilegedOpcodeI,
            PrivilegedOpcodeS,
            PrivilegedOpcodeB,
            PrivilegedOpcodeU,
            PrivilegedOpcodeJ,
        >,
        prv: &PrivilegeMode,
        _: &mut ProgramCounter,
        _: &mut IntegerRegister,
        _: &mut Memory,
    ) -> Result<(), Cause> {
        if let Instruction::TypeR {
            opcode,
            rd: _,
            funct3: _,
            rs1: _,
            rs2: _,
            funct7: _,
        } = instruction
        {
            match opcode {
                PrivilegedOpcodeR::Mret => {
                    if prv == &PrivilegeMode::Machine {
                        Err(Cause::ExceptionReturn(ExceptionReturn::Machine))
                    } else {
                        Err(Cause::Exception(Exception::IllegalInstruction))
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}
