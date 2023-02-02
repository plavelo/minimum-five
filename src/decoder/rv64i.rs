use crate::{
    decoder::{Decoder, MASK_3BIT, MASK_7BIT},
    instruction::{
        rv64i::{
            Rv64iOpcodeB, Rv64iOpcodeI, Rv64iOpcodeJ, Rv64iOpcodeR, Rv64iOpcodeS, Rv64iOpcodeU,
        },
        Instruction,
    },
};

pub struct Rv64iDecoder;

impl Decoder for Rv64iDecoder {
    type OpcodeR = Rv64iOpcodeR;
    type OpcodeI = Rv64iOpcodeI;
    type OpcodeS = Rv64iOpcodeS;
    type OpcodeB = Rv64iOpcodeB;
    type OpcodeU = Rv64iOpcodeU;
    type OpcodeJ = Rv64iOpcodeJ;

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
            0b0011011 => Self::decode_i(
                match funct3 {
                    0b000 => Some(Rv64iOpcodeI::Addiw),
                    _ => None,
                },
                instruction,
            ),
            _ => None,
        }
    }
}
