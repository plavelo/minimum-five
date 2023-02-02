use crate::{
    decoder::{Decoder, MASK_3BIT, MASK_5BIT, MASK_7BIT},
    instruction::{
        privileged::{
            PrivilegedOpcodeB, PrivilegedOpcodeI, PrivilegedOpcodeJ, PrivilegedOpcodeR,
            PrivilegedOpcodeS, PrivilegedOpcodeU,
        },
        Instruction,
    },
};

pub struct PrivilegedDecoder;

impl Decoder for PrivilegedDecoder {
    type OpcodeR = PrivilegedOpcodeR;
    type OpcodeI = PrivilegedOpcodeI;
    type OpcodeS = PrivilegedOpcodeS;
    type OpcodeB = PrivilegedOpcodeB;
    type OpcodeU = PrivilegedOpcodeU;
    type OpcodeJ = PrivilegedOpcodeJ;

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
        let rs2 = (instruction >> 20) & MASK_5BIT;
        match opcode {
            0b1110011 => Self::decode_r(
                match funct3 {
                    0b000 => match funct7 {
                        0b0011000 => match rs2 {
                            0b00010 => Some(PrivilegedOpcodeR::Mret),
                            _ => None,
                        },
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
