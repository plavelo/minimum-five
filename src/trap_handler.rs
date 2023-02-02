use std::ops::Range;

use crate::{
    cause::{Cause, Exception},
    csr::{
        ControlAndStatusRegister, MCAUSE, MEDELEG, MEPC, MIDELEG, MSTATUS, MTVAL, MTVEC, SCAUSE,
        SEDELEG, SEPC, SIDELEG, SSTATUS, STATUS_MIE, STATUS_MPIE, STATUS_MPP, STATUS_SIE,
        STATUS_SPIE, STATUS_SPP, STATUS_UIE, STATUS_UPIE, STVAL, STVEC, UCAUSE, UEPC, USTATUS,
        UTVAL, UTVEC,
    },
    mode::PrivilegeMode,
};

fn delegated_privilege_mode(csr: &mut ControlAndStatusRegister, cause: &Cause) -> PrivilegeMode {
    let m_addr = if cause.is_interrupt() {
        MIDELEG
    } else {
        MEDELEG
    };
    let s_addr = if cause.is_interrupt() {
        SIDELEG
    } else {
        SEDELEG
    };
    let code = cause.exception_code();
    if ((csr.csrrs(m_addr, 0) >> code) & 1) == 0 {
        PrivilegeMode::Machine
    } else if ((csr.csrrs(s_addr, 0) >> code) & 1) == 0 {
        PrivilegeMode::Supervisor
    } else {
        PrivilegeMode::User
    }
}

fn select_address(
    privilege_mode: &PrivilegeMode,
    m_address: u64,
    s_address: u64,
    u_address: u64,
) -> u64 {
    match privilege_mode {
        PrivilegeMode::Machine => m_address,
        PrivilegeMode::Supervisor => s_address,
        PrivilegeMode::User => u_address,
    }
}

fn select_status_field(
    privilege_mode: &PrivilegeMode,
    m_field: Range<usize>,
    s_field: Range<usize>,
    u_field: Range<usize>,
) -> Range<usize> {
    match privilege_mode {
        PrivilegeMode::Machine => m_field,
        PrivilegeMode::Supervisor => s_field,
        PrivilegeMode::User => u_field,
    }
}

fn read_status_field(
    csr: &mut ControlAndStatusRegister,
    address: u64,
    field: &Range<usize>,
) -> u64 {
    let status = csr.csrrs(address, 0);
    let length = (field.end - field.start) as u64;
    let mask = !((2 ^ length) - 1);
    (status >> field.start) & mask
}

fn update_status_field(
    csr: &mut ControlAndStatusRegister,
    address: u64,
    field: &Range<usize>,
    value: u64,
) {
    let length = (field.end - field.start) as u64;
    let mask = !((2 ^ length) - 1);
    let shifted_value = value << field.start;
    csr.csrrc(address, mask);
    csr.csrrs(address, shifted_value);
}

fn select_tval(cause: &Cause, faulting_instruction: u32) -> u64 {
    if let Cause::Exception(exception) = cause {
        match exception {
            Exception::IllegalInstruction => faulting_instruction as u64,
            _ => 0,
        }
    } else {
        0
    }
}

fn handle_trap(
    cause: &Cause,
    pc_address: u64,
    instruction: u32,
    current_privilege_mode: PrivilegeMode,
    csr: &mut ControlAndStatusRegister,
) -> (PrivilegeMode, u64) {
    let next_privilege_mode = delegated_privilege_mode(csr, cause);
    // set cause register
    let cause_address = select_address(&next_privilege_mode, MCAUSE, SCAUSE, UCAUSE);
    csr.csrrw(cause_address, cause.to_primitive());

    // set exception program counter
    let epc_address = select_address(&next_privilege_mode, MEPC, SEPC, UEPC);
    csr.csrrw(epc_address, pc_address);

    // set trap value register
    let tval_address = select_address(&next_privilege_mode, MTVAL, STVAL, UTVAL);
    let tval = select_tval(cause, instruction);
    csr.csrrw(tval_address, tval);

    // set previous privilege
    let status_address = select_address(&next_privilege_mode, MSTATUS, SSTATUS, USTATUS);
    match next_privilege_mode {
        PrivilegeMode::Machine => update_status_field(
            csr,
            status_address,
            &STATUS_MPP,
            current_privilege_mode as u64,
        ),
        PrivilegeMode::Supervisor => update_status_field(
            csr,
            status_address,
            &STATUS_SPP,
            current_privilege_mode as u64,
        ),
        PrivilegeMode::User => {}
    }

    // set previous interrupt enable
    let ie_field = select_status_field(&next_privilege_mode, STATUS_MIE, STATUS_SIE, STATUS_UIE);
    let ie = read_status_field(csr, status_address, &ie_field);
    let pie_field =
        select_status_field(&next_privilege_mode, STATUS_MPIE, STATUS_SPIE, STATUS_UPIE);
    update_status_field(csr, status_address, &pie_field, ie);

    // disable interrupt enable
    update_status_field(csr, status_address, &ie_field, 0);

    // set pc to trap-vector base-address register
    let tvec_address = select_address(&next_privilege_mode, MTVEC, STVEC, UTVEC);
    let tvec = csr.csrrs(tvec_address, 0);
    (next_privilege_mode, tvec)
}

fn handle_exception_return(
    current_privilege_mode: PrivilegeMode,
    csr: &mut ControlAndStatusRegister,
) -> (PrivilegeMode, u64) {
    let status_address = select_address(&current_privilege_mode, MSTATUS, SSTATUS, USTATUS);

    // restore interrupt enable
    let pie_field = select_status_field(
        &current_privilege_mode,
        STATUS_MPIE,
        STATUS_SPIE,
        STATUS_UPIE,
    );
    let ie_field = select_status_field(&current_privilege_mode, STATUS_MIE, STATUS_SIE, STATUS_UIE);
    let pie = read_status_field(csr, status_address, &pie_field);
    update_status_field(csr, status_address, &ie_field, pie);

    // set 1 to previous interrupt enable
    update_status_field(csr, status_address, &pie_field, 1);

    // read previous privilege
    let pp = match current_privilege_mode {
        PrivilegeMode::Machine => {
            PrivilegeMode::from_primitive(read_status_field(csr, status_address, &STATUS_MPP))
        }
        PrivilegeMode::Supervisor => {
            PrivilegeMode::from_primitive(read_status_field(csr, status_address, &STATUS_SPP))
        }
        PrivilegeMode::User => PrivilegeMode::User,
    };

    // set 0 to previous privilege
    match current_privilege_mode {
        PrivilegeMode::Machine => update_status_field(csr, status_address, &STATUS_MPP, 0),
        PrivilegeMode::Supervisor => update_status_field(csr, status_address, &STATUS_SPP, 0),
        PrivilegeMode::User => {}
    };

    // read exception program counter
    let epc_address = select_address(&current_privilege_mode, MEPC, SEPC, UEPC);
    let epc = csr.csrrs(epc_address, 0);

    (pp, epc)
}

pub fn handle_cause(
    cause: &Cause,
    pc_address: u64,
    instruction: u32,
    current_privilege_mode: PrivilegeMode,
    csr: &mut ControlAndStatusRegister,
) -> (PrivilegeMode, u64) {
    match cause {
        Cause::ExceptionReturn(_) => handle_exception_return(current_privilege_mode, csr),
        _ => handle_trap(cause, pc_address, instruction, current_privilege_mode, csr),
    }
}
