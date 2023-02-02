pub enum Cause {
    Interrupt(Interrupt),
    Exception(Exception),
    ExceptionReturn(ExceptionReturn),
}

pub enum Interrupt {
    UserSoftware,
    SupervisorSoftware,
    MachineSoftware,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
}

pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUserMode,
    EnvironmentCallFromSupervisorMode,
    EnvironmentCallFromMachineMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
}

pub enum ExceptionReturn {
    User,
    Supervisor,
    Machine,
}

impl Cause {
    pub fn to_primitive(&self) -> u64 {
        match self {
            Self::Interrupt(interrupt) => interrupt.to_primitive(),
            Self::Exception(exception) => exception.to_primitive(),
            Self::ExceptionReturn(_) => panic!(),
        }
    }

    pub fn is_interrupt(&self) -> bool {
        matches!(self, Self::Interrupt(_))
    }

    pub fn exception_code(&self) -> u64 {
        match self {
            Self::Interrupt(interrupt) => interrupt.to_primitive() & 0b1111,
            Self::Exception(exception) => exception.to_primitive(),
            Self::ExceptionReturn(_) => panic!(),
        }
    }
}

impl Interrupt {
    pub fn to_primitive(&self) -> u64 {
        match self {
            Self::UserSoftware => 1u64 << 63,
            Self::SupervisorSoftware => 1u64 << 63 | 1,
            Self::MachineSoftware => 1u64 << 63 | 3,
            Self::UserTimer => 1u64 << 63 | 4,
            Self::SupervisorTimer => 1u64 << 63 | 5,
            Self::MachineTimer => 1u64 << 63 | 7,
            Self::UserExternal => 1u64 << 63 | 8,
            Self::SupervisorExternal => 1u64 << 63 | 9,
            Self::MachineExternal => 1u64 << 63 | 11,
        }
    }

    pub fn is_interrupt(&self) -> bool {
        self.to_primitive() >> 63 == 1
    }

    pub fn exception_code(&self) -> u64 {
        self.to_primitive() & 0b1111
    }
}

impl Exception {
    pub fn to_primitive(&self) -> u64 {
        match self {
            Self::InstructionAddressMisaligned => 0,
            Self::InstructionAccessFault => 1,
            Self::IllegalInstruction => 2,
            Self::Breakpoint => 3,
            Self::LoadAddressMisaligned => 4,
            Self::LoadAccessFault => 5,
            Self::StoreAddressMisaligned => 6,
            Self::StoreAccessFault => 7,
            Self::EnvironmentCallFromUserMode => 8,
            Self::EnvironmentCallFromSupervisorMode => 9,
            Self::EnvironmentCallFromMachineMode => 11,
            Self::InstructionPageFault => 12,
            Self::LoadPageFault => 13,
            Self::StorePageFault => 15,
        }
    }

    pub fn is_interrupt(&self) -> bool {
        self.to_primitive() >> 63 == 1
    }

    pub fn exception_code(&self) -> u64 {
        self.to_primitive() & 0b1111
    }
}
