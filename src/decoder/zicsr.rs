use crate::{
    decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    instruction::{
        zicsr::{
            ZicsrOpcodeB, ZicsrOpcodeI, ZicsrOpcodeJ, ZicsrOpcodeR, ZicsrOpcodeS, ZicsrOpcodeU,
        },
        Instruction,
    },
};

pub struct ZicsrDecoder;

impl Decoder for ZicsrDecoder {
    type OpcodeR = ZicsrOpcodeR;
    type OpcodeI = ZicsrOpcodeI;
    type OpcodeS = ZicsrOpcodeS;
    type OpcodeB = ZicsrOpcodeB;
    type OpcodeU = ZicsrOpcodeU;
    type OpcodeJ = ZicsrOpcodeJ;

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
        match opcode {
            0b1110011 => Self::decode_i(
                match funct3 {
                    0b001 => Some(ZicsrOpcodeI::Csrrw),
                    0b010 => Some(ZicsrOpcodeI::Csrrs),
                    0b101 => Some(ZicsrOpcodeI::Csrrwi),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
