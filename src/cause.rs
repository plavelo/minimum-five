pub enum Cause {
    Exception(Exception),
    ExceptionReturn(ExceptionReturn),
}

pub enum Exception {
    IllegalInstruction,
    EnvironmentCallFromUserMode,
    EnvironmentCallFromSupervisorMode,
    EnvironmentCallFromMachineMode,
}

pub enum ExceptionReturn {
    Machine,
}

impl Cause {
    pub fn to_primitive(&self) -> u64 {
        match self {
            Self::Exception(exception) => exception.to_primitive(),
            Self::ExceptionReturn(_) => panic!(),
        }
    }

    pub fn is_interrupt(&self) -> bool {
        false
    }

    pub fn exception_code(&self) -> u64 {
        match self {
            Self::Exception(exception) => exception.to_primitive(),
            Self::ExceptionReturn(_) => panic!(),
        }
    }
}

impl Exception {
    pub fn to_primitive(&self) -> u64 {
        match self {
            Self::IllegalInstruction => 2,
            Self::EnvironmentCallFromUserMode => 8,
            Self::EnvironmentCallFromSupervisorMode => 9,
            Self::EnvironmentCallFromMachineMode => 11,
        }
    }
}
