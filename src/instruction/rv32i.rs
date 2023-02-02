#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeR {
    Add,
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeI {
    Slli,
    Addi,
    Fence,
    Ecall,
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeS {
    Sw,
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeB {
    Beq,
    Bne,
    Bge,
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeU {
    Lui,
    Auipc,
}

#[derive(Debug, PartialEq)]
pub enum Rv32iOpcodeJ {
    Jal,
}
