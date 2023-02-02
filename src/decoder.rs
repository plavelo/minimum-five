pub mod privileged;
pub mod rv32i;
pub mod rv64i;

use crate::instruction::Instruction;

const MASK_3BIT: u32 = 0b111;
const MASK_5BIT: u32 = 0b11111;
const MASK_7BIT: u32 = 0b1111111;

pub trait Decoder {
    type OpcodeR;
    type OpcodeI;
    type OpcodeS;
    type OpcodeB;
    type OpcodeU;
    type OpcodeJ;

    #[allow(clippy::type_complexity)]
    fn decode(
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    >;

    #[allow(clippy::type_complexity)]
    fn decode_r(
        opcode: Option<Self::OpcodeR>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let funct3 = ((instruction >> 12) & MASK_3BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let funct7 = ((instruction >> 25) & MASK_7BIT) as usize;
            Instruction::TypeR {
                opcode: o,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_i(
        opcode: Option<Self::OpcodeI>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let funct3 = ((instruction >> 12) & MASK_3BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let imm = (instruction >> 20) as u64;
            Instruction::TypeI {
                opcode: o,
                rd,
                funct3,
                rs1,
                imm,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_s(
        opcode: Option<Self::OpcodeS>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let funct3 = ((instruction >> 12) & MASK_3BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0xfe000000) as i32 >> 20) as u64
                | ((instruction >> 7) & MASK_5BIT) as u64;
            Instruction::TypeS {
                opcode: o,
                funct3,
                rs1,
                rs2,
                imm,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_b(
        opcode: Option<Self::OpcodeB>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let funct3 = ((instruction >> 12) & MASK_3BIT) as usize;
            let rs1 = ((instruction >> 15) & MASK_5BIT) as usize;
            let rs2 = ((instruction >> 20) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 19) as u64
                | ((instruction & 0x80) << 4) as u64
                | ((instruction >> 20) & 0x7e0) as u64
                | ((instruction >> 7) & 0x1e) as u64;
            Instruction::TypeB {
                opcode: o,
                funct3,
                rs1,
                rs2,
                imm,
            }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_u(
        opcode: Option<Self::OpcodeU>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = ((instruction & 0xfffff000) >> 12) as u64;
            Instruction::TypeU { opcode: o, rd, imm }
        })
    }

    #[allow(clippy::type_complexity)]
    fn decode_j(
        opcode: Option<Self::OpcodeJ>,
        instruction: u32,
    ) -> Option<
        Instruction<
            Self::OpcodeR,
            Self::OpcodeI,
            Self::OpcodeS,
            Self::OpcodeB,
            Self::OpcodeU,
            Self::OpcodeJ,
        >,
    > {
        opcode.map(|o| {
            let rd = ((instruction >> 7) & MASK_5BIT) as usize;
            let imm = ((instruction & 0x80000000) as i32 >> 11) as u64
                | (instruction & 0xff000) as u64
                | ((instruction >> 9) & 0x800) as u64
                | ((instruction >> 20) & 0x7fe) as u64;
            Instruction::TypeJ { opcode: o, rd, imm }
        })
    }
}
