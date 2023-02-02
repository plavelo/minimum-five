use crate::{
    decoder::Decoder,
    instruction::{
        rv32i::{
            Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
        },
        Instruction,
    },
};

use super::{MASK_3BIT, MASK_7BIT};

pub struct Rv32iDecoder;

impl Decoder for Rv32iDecoder {
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

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
    > {
        let opcode = instruction & MASK_7BIT;
        let funct3 = (instruction >> 12) & MASK_3BIT;
        let funct7 = (instruction >> 25) & MASK_7BIT;
        match opcode {
            0b0110111 => Self::decode_u(Some(Rv32iOpcodeU::Lui), instruction),
            0b0010111 => Self::decode_u(Some(Rv32iOpcodeU::Auipc), instruction),
            0b1101111 => Self::decode_j(Some(Rv32iOpcodeJ::Jal), instruction),
            0b1100011 => Self::decode_b(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeB::Beq),
                    0b001 => Some(Rv32iOpcodeB::Bne),
                    0b101 => Some(Rv32iOpcodeB::Bge),
                    _ => None,
                },
                instruction,
            ),
            0b0100011 => Self::decode_s(
                match funct3 {
                    0b010 => Some(Rv32iOpcodeS::Sw),
                    _ => None,
                },
                instruction,
            ),
            0b0010011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Addi),
                    0b001 => Some(Rv32iOpcodeI::Slli),
                    _ => None,
                },
                instruction,
            ),
            0b0110011 => Self::decode_r(
                match funct7 {
                    0b0000000 => match funct3 {
                        0b000 => Some(Rv32iOpcodeR::Add),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            0b0001111 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv32iOpcodeI::Fence),
                    _ => None,
                },
                instruction,
            ),
            0b1110011 => Self::decode_i(
                match funct3 {
                    0b000 => match instruction >> 20 {
                        0b0 => Some(Rv32iOpcodeI::Ecall),
                        _ => None,
                    },
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
