#[derive(Copy, Clone, PartialEq)]
pub enum PrivilegeMode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

impl Default for PrivilegeMode {
    fn default() -> Self {
        PrivilegeMode::Machine
    }
}

impl PrivilegeMode {
    pub fn from_primitive(mode: u64) -> Self {
        match mode {
            0 => Self::User,
            0b01 => Self::Supervisor,
            0b11 => Self::Machine,
            _ => panic!(),
        }
    }
}
