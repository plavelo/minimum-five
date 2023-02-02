pub mod privileged;
pub mod rv32i;
pub mod rv64i;

#[derive(Debug, PartialEq)]
pub enum Instruction<OpcodeR, OpcodeI, OpcodeS, OpcodeB, OpcodeU, OpcodeJ> {
    TypeR {
        opcode: OpcodeR,
        rd: usize,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        funct7: usize,
    },
    TypeI {
        opcode: OpcodeI,
        rd: usize,
        funct3: usize,
        rs1: usize,
        imm: u64,
    },
    TypeS {
        opcode: OpcodeS,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeB {
        opcode: OpcodeB,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        imm: u64,
    },
    TypeU {
        opcode: OpcodeU,
        rd: usize,
        imm: u64,
    },
    TypeJ {
        opcode: OpcodeJ,
        rd: usize,
        imm: u64,
    },
}
