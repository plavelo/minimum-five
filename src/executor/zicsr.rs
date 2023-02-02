use crate::{
    bitops::MASK_12BIT,
    cause::Cause,
    csr::ControlAndStatusRegister,
    executor::Executor,
    instruction::{
        zicsr::{
            ZicsrOpcodeB, ZicsrOpcodeI, ZicsrOpcodeJ, ZicsrOpcodeR, ZicsrOpcodeS, ZicsrOpcodeU,
        },
        Instruction,
    },
    memory::Memory,
    mode::PrivilegeMode,
    pc::ProgramCounter,
    x::IntegerRegister,
};

pub struct ZicsrExecutor;

impl Executor for ZicsrExecutor {
    type OpcodeR = ZicsrOpcodeR;
    type OpcodeI = ZicsrOpcodeI;
    type OpcodeS = ZicsrOpcodeS;
    type OpcodeB = ZicsrOpcodeB;
    type OpcodeU = ZicsrOpcodeU;
    type OpcodeJ = ZicsrOpcodeJ;

    fn execute(
        instruction: Instruction<
            ZicsrOpcodeR,
            ZicsrOpcodeI,
            ZicsrOpcodeS,
            ZicsrOpcodeB,
            ZicsrOpcodeU,
            ZicsrOpcodeJ,
        >,
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        csr: &mut ControlAndStatusRegister,
        _: &mut Memory,
    ) -> Result<(), Cause> {
        if let Instruction::TypeI {
            opcode,
            rd,
            funct3: _,
            rs1,
            imm,
        } = instruction
        {
            match opcode {
                ZicsrOpcodeI::Csrrw => x.write(rd, csr.csrrw(imm & MASK_12BIT, x.read(rs1))),
                ZicsrOpcodeI::Csrrs => x.write(rd, csr.csrrs(imm & MASK_12BIT, x.read(rs1))),
                ZicsrOpcodeI::Csrrwi => x.write(rd, csr.csrrw(imm & MASK_12BIT, rs1 as u64)),
            }
        }
        Ok(())
    }
}
