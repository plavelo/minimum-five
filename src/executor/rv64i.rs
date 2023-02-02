use crate::{
    bitops::extend_sign,
    cause::Cause,
    csr::ControlAndStatusRegister,
    executor::Executor,
    instruction::{
        rv64i::{
            Rv64iOpcodeB, Rv64iOpcodeI, Rv64iOpcodeJ, Rv64iOpcodeR, Rv64iOpcodeS, Rv64iOpcodeU,
        },
        Instruction,
    },
    memory::Memory,
    mode::PrivilegeMode,
    pc::ProgramCounter,
    x::IntegerRegister,
};

pub struct Rv64iExecutor;

impl Executor for Rv64iExecutor {
    type OpcodeR = Rv64iOpcodeR;
    type OpcodeI = Rv64iOpcodeI;
    type OpcodeS = Rv64iOpcodeS;
    type OpcodeB = Rv64iOpcodeB;
    type OpcodeU = Rv64iOpcodeU;
    type OpcodeJ = Rv64iOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv64iOpcodeR,
            Rv64iOpcodeI,
            Rv64iOpcodeS,
            Rv64iOpcodeB,
            Rv64iOpcodeU,
            Rv64iOpcodeJ,
        >,
        _: &PrivilegeMode,
        _: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut ControlAndStatusRegister,
        _: &mut Memory,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv64iOpcodeI::Addiw => x.write(
                    rd,
                    extend_sign(
                        (x.read(rs1) as i64).wrapping_add(extend_sign(imm, 12) as i64) as u64,
                        32,
                    ),
                ),
            },
            _ => (),
        }
        Ok(())
    }
}
