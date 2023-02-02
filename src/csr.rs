use std::collections::HashMap;
use std::ops::Range;

pub const STATUS_UIE: Range<usize> = 0..0;
pub const STATUS_SIE: Range<usize> = 1..1;
pub const STATUS_MIE: Range<usize> = 3..3;
pub const STATUS_UPIE: Range<usize> = 4..4;
pub const STATUS_SPIE: Range<usize> = 5..5;
pub const STATUS_MPIE: Range<usize> = 7..7;
pub const STATUS_SPP: Range<usize> = 8..8;
pub const STATUS_MPP: Range<usize> = 11..12;

// Machine Trap Setup (MRW)
pub const MSTATUS: u64 = 0x300; // Machine status register.
pub const MEDELEG: u64 = 0x302; // Machine exception delegation register.
pub const MIDELEG: u64 = 0x303; // Machine interrupt delegation register.
pub const MTVEC: u64 = 0x305; // Machine trap-handler base address.

// Machine Trap Handling (MRW)
pub const MEPC: u64 = 0x341; // Machine exception program counter.
pub const MCAUSE: u64 = 0x342; // Machine trap cause.
pub const MTVAL: u64 = 0x343; // Machine bad address or instruction.

// Supervisor Trap Setup (SRW)
pub const SSTATUS: u64 = 0x100; // Supervisor status register.
pub const SEDELEG: u64 = 0x102; // Supervisor exception delegation register.
pub const SIDELEG: u64 = 0x103; // Supervisor interrupt delegation register.
pub const STVEC: u64 = 0x105; // Supervisor trap handler base address.

// Supervisor Trap Handling (SRW)
pub const SEPC: u64 = 0x141; // Supervisor exception program counter.
pub const SCAUSE: u64 = 0x142; // Supervisor trap cause.
pub const STVAL: u64 = 0x143; // Supervisor bad address or instruction.

// User Trap Setup (URW)
pub const USTATUS: u64 = 0x000; // User status register.
pub const UTVEC: u64 = 0x005; // User trap handler base address.

// User Trap Handling (URW)
pub const UEPC: u64 = 0x041; // User exception program counter.
pub const UCAUSE: u64 = 0x042; // User trap cause.
pub const UTVAL: u64 = 0x043; // User bad address or instruction.

pub struct ControlAndStatusRegister {
    csr: HashMap<u64, u64>,
}

impl ControlAndStatusRegister {
    fn contains(&self, address: u64) -> bool {
        self.csr.contains_key(&address)
    }

    fn read(&self, address: u64) -> u64 {
        if self.contains(address) {
            return self.csr[&address];
        }
        panic!("address not found. {:x}", address);
    }

    fn write(&mut self, address: u64, value: u64) {
        if self.contains(address) {
            *self.csr.get_mut(&address).unwrap() = value;
            return;
        }
        panic!("address not found. {:x}", address);
    }

    pub fn csrrw(&mut self, address: u64, value: u64) -> u64 {
        let t = self.read(address);
        self.write(address, value);
        t
    }

    pub fn csrrs(&mut self, address: u64, value: u64) -> u64 {
        let t = self.read(address);
        self.write(address, self.csr[&address] | value);
        t
    }

    pub fn csrrc(&mut self, address: u64, value: u64) -> u64 {
        let t = self.read(address);
        self.write(address, self.csr[&address] & !value);
        t
    }
}

impl Default for ControlAndStatusRegister {
    fn default() -> Self {
        Self {
            csr: [
                MSTATUS, MEDELEG, MIDELEG, MTVEC, MEPC, MCAUSE, MTVAL, SSTATUS, SEDELEG, SIDELEG,
                STVEC, SEPC, SCAUSE, STVAL, USTATUS, UTVEC, UEPC, UCAUSE, UTVAL,
            ]
            .iter()
            .cloned()
            .map(|a| (a, 0))
            .collect::<HashMap<_, _>>(),
        }
    }
}
