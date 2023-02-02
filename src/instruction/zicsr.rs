#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeR {}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeI {
    Csrrw,
    Csrrs,
    Csrrwi,
}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeS {}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeB {}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeU {}

#[derive(Debug, PartialEq)]
pub enum ZicsrOpcodeJ {}
