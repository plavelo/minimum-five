use crate::{
    bitops::{extend_sign, MASK_6BIT},
    cause::{Cause, Exception},
    executor::Executor,
    instruction::{
        rv32i::{
            Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
        },
        Instruction,
    },
    memory::Memory,
    mode::PrivilegeMode,
    pc::ProgramCounter,
    x::IntegerRegister,
};

pub struct Rv32iExecutor;

impl Executor for Rv32iExecutor {
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv32iOpcodeR,
            Rv32iOpcodeI,
            Rv32iOpcodeS,
            Rv32iOpcodeB,
            Rv32iOpcodeU,
            Rv32iOpcodeJ,
        >,
        prv: &PrivilegeMode,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        memory: &mut Memory,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv32iOpcodeR::Add => {
                    x.write(rd, x.read(rs1).wrapping_add(x.read(rs2)));
                    Ok(())
                }
            },
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32iOpcodeI::Slli => {
                    x.write(rd, x.read(rs1) << (imm << MASK_6BIT));
                    Ok(())
                }
                Rv32iOpcodeI::Addi => {
                    x.write(rd, x.read(rs1).wrapping_add(extend_sign(imm, 12)));
                    Ok(())
                }
                Rv32iOpcodeI::Fence => Ok(()), // not yet supported
                Rv32iOpcodeI::Ecall => match prv {
                    PrivilegeMode::User => {
                        Err(Cause::Exception(Exception::EnvironmentCallFromUserMode))
                    }
                    PrivilegeMode::Supervisor => Err(Cause::Exception(
                        Exception::EnvironmentCallFromSupervisorMode,
                    )),
                    PrivilegeMode::Machine => {
                        Err(Cause::Exception(Exception::EnvironmentCallFromMachineMode))
                    }
                },
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeS::Sw => {
                    memory.store(
                        x.read(rs1).wrapping_add(extend_sign(imm, 12)) as u64,
                        x.read(rs2) as u32,
                    );
                    Ok(())
                }
            },
            Instruction::TypeB {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeB::Beq => {
                    if x.read(rs1) == x.read(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bne => {
                    if x.read(rs1) != x.read(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bge => {
                    if x.read(rs1) >= x.read(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
            },
            Instruction::TypeU { opcode, rd, imm } => match opcode {
                Rv32iOpcodeU::Lui => {
                    x.write(rd, (extend_sign(imm, 20) << 12) & 0xfffffffffffff000);
                    Ok(())
                }
                Rv32iOpcodeU::Auipc => {
                    x.write(
                        rd,
                        (pc.read() as i64).wrapping_add((extend_sign(imm, 20) as i64) << 12) as u64,
                    );
                    Ok(())
                }
            },
            Instruction::TypeJ { opcode, rd, imm } => match opcode {
                Rv32iOpcodeJ::Jal => {
                    x.write(rd, pc.read().wrapping_add(4));
                    pc.jumpr(imm as i64);
                    Ok(())
                }
            },
        }
    }
}
