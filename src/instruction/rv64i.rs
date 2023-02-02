#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeR {}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeI {
    Addiw,
}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeS {}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeB {}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeU {}

#[derive(Debug, PartialEq)]
pub enum Rv64iOpcodeJ {}
