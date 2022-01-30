use super::register::Register;
use super::Mode;
use chrono::{DateTime, Duration, Local};

const MSTATUS_MIE: u64 = 0b1000;
const SSTATUS_SIE: u64 = 0b0010;

const MIP_SSIP: u64 = 0b0000_0000_0010; // Supervisor software interrupt
const MIP_MSIP: u64 = 0b0000_0000_1000; // Machine software interrupt
const MIP_STIP: u64 = 0b0000_0010_0000; // Supervisor timer interrupt
const MIP_MTIP: u64 = 0b0000_1000_0000; // Machine timer interrupt
const MIP_SEIP: u64 = 0b0010_0000_0000; // Supervisor external interrupt
const MIP_MEIP: u64 = 0b1000_0000_0000; // Machine external interrupt

pub fn timer_int(
    reg: &mut Register,
    current_mode: &mut Mode,
    time: &mut DateTime<Local>,
    mtimecmp: u64,
) {
    if (*current_mode == Mode::M) && (reg.mstatus & MSTATUS_MIE == 0) {
        return;
    }

    let duration: Duration = Local::now() - *time;
    if let Some(nano) = duration.num_nanoseconds() {
        if nano <= (mtimecmp as i64 * 100) {
            return;
        }
    }

    reg.mip |= MIP_MTIP;
    reg.mcause = 0x8000_0000_0000_0007; // timer interrupt
    int(reg, current_mode);

    *time = Local::now();
}

pub fn int(reg: &mut Register, current_mode: &mut Mode) {
    if (*current_mode == Mode::M) && (reg.mstatus & MSTATUS_MIE == 0) {
        return;
    }

    *current_mode = Mode::M;

    if (reg.mip | reg.sip) == 0 {
        return;
    }

    let (int_code, int_mode) = get_int_code(reg);
    if is_disabled_int(reg, int_code, &int_mode) {
        return;
    }

    match int_mode {
        Mode::M => {
            if reg.mideleg & int_code != 0 {
                m_int(reg, current_mode, int_code);
            } else {
                s_int(reg, current_mode, int_code);
            }
        }
        Mode::S => s_int(reg, current_mode, int_code),
        _ => (),
    }

    // free the interrupt pending bit
    match int_mode {
        Mode::M => reg.mip &= !int_code,
        Mode::S => reg.sip &= !int_code,
        _ => (),
    }
}

fn get_int_code(reg: &Register) -> (u64, Mode) {
    if reg.mip != 0 {
        if reg.mip & MIP_SSIP != 0 {
            return (MIP_SSIP, Mode::M);
        }
        if reg.mip & MIP_MSIP != 0 {
            return (MIP_MSIP, Mode::M);
        }
        if reg.mip & MIP_STIP != 0 {
            return (MIP_STIP, Mode::M);
        }
        if reg.mip & MIP_MTIP != 0 {
            return (MIP_MTIP, Mode::M);
        }
        if reg.mip & MIP_SEIP != 0 {
            return (MIP_SEIP, Mode::M);
        }
        if reg.mip & MIP_MEIP != 0 {
            return (MIP_MEIP, Mode::M);
        }
    }

    if reg.sip & MIP_SSIP != 0 {
        return (MIP_SSIP, Mode::S);
    }
    if reg.sip & MIP_MSIP != 0 {
        return (MIP_MSIP, Mode::S);
    }
    if reg.sip & MIP_STIP != 0 {
        return (MIP_STIP, Mode::S);
    }
    if reg.sip & MIP_MTIP != 0 {
        return (MIP_MTIP, Mode::S);
    }
    if reg.sip & MIP_SEIP != 0 {
        return (MIP_SEIP, Mode::S);
    }
    if reg.sip & MIP_MEIP != 0 {
        return (MIP_MEIP, Mode::S);
    }

    panic!(
        "ivalid interrupt code. reg.mip: {}, reg.sip: {}",
        reg.mip, reg.sip
    );
}

fn is_disabled_int(reg: &Register, int_code: u64, int_mode: &Mode) -> bool {
    match *int_mode {
        Mode::M => {
            if (int_code & reg.mie) == 0 {
                return true;
            }
        }
        Mode::S => {
            if (int_code & reg.sie) == 0 {
                return true;
            }
        }
        _ => (),
    }
    false
}

fn m_int(reg: &mut Register, current_mode: &mut Mode, int_code: u64) {
    *current_mode = Mode::M;

    let mstatus_mie = (reg.mstatus & MSTATUS_MIE) >> 3;
    let mstatus_mpie = mstatus_mie << 7;
    reg.mstatus |= mstatus_mpie;
    reg.mstatus &= !MSTATUS_MIE;

    reg.mepc = reg.pc;
    reg.pc = reg.mtvec;
}

fn s_int(reg: &mut Register, current_mode: &mut Mode, int_code: u64) {
    *current_mode = Mode::S;

    let sstatus_sie = (reg.sstatus & SSTATUS_SIE) >> 1;
    let sstatus_spie = sstatus_sie << 5;
    reg.sstatus |= sstatus_spie;
    reg.sstatus &= !SSTATUS_SIE;

    reg.sepc = reg.pc;
    reg.pc = reg.stvec;
}

pub fn m_trap(reg: &mut Register, current_mode: &mut Mode) {
    let e_code = reg.mcause & 0x7FFF_FFFF_FFF_FFF;
    //trap(reg, current_mode, e_code);
    if reg.medeleg & e_code != 0 {
        s_exception(reg, current_mode, e_code);
    } else {
        m_exception(reg, current_mode, e_code);
    }
}

pub fn s_trap(reg: &mut Register, current_mode: &mut Mode) {
    let e_code = reg.scause & 0x7FFF_FFFF_FFF_FFF;
}

fn m_exception(reg: &mut Register, mode: &mut Mode, e_code: u64) {
    *mode = Mode::M;
}

fn s_exception(reg: &mut Register, mode: &mut Mode, e_code: u64) {
    *mode = Mode::S;
}
