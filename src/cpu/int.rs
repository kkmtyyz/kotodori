use super::register::Register;
use super::Mode;
use chrono::{DateTime, Duration, Local};

pub fn timer_int(reg: &mut Register, time: &mut DateTime<Local>, mtimecmp: u64) {
    let mstatus_mie = reg.mstatus & 0b1000;
    let mie_mtie = reg.mie & 0b1000_0000;
    if mstatus_mie != 0b1000 && mie_mtie != 0b1000_0000 {
        return;
    }
    reg.mip |= 0b1000_0000;

    let duration: Duration = Local::now() - *time;
    if let Some(nano) = duration.num_nanoseconds() {
        if nano <= (mtimecmp as i64 * 100) {
            return;
        }
    }

    reg.mepc = reg.pc;
    reg.pc = reg.mtvec;
    reg.mcause = 0x8000_0000_0000_0007; // timer interrupt
    let mstatus_mpie = mstatus_mie << 4;
    reg.mstatus |= mstatus_mpie;
    reg.mstatus &= !0b1000;

    //reg.mstatus |= !0b1000; // mstatus.mie = 0

    reg.mip &= !0b1000_000;
    *time = Local::now();
}

pub fn int(reg: &mut Register, mode: &mut Mode) {
    if (reg.mip & reg.sip) == 0 {
        return;
    }

    let int = reg.mcause >> 63;
    let e_code = reg.mcause & 0x7FFF_FFFF_FFFF_FFFF;

    if int == 1 {
        if reg.mideleg & e_code == e_code {
            s_int(reg, mode);
        } else {
            m_int(reg, mode);
        }
        return;
    }

    if reg.medeleg & e_code == e_code {
        s_exception(reg, mode);
    } else {
        m_exception(reg, mode);
    }
}

fn m_int(reg: &mut Register, mode: &mut Mode) {
    *mode = Mode::M;
    let mstatus_sie = (reg.mstatus & 0b10) >> 1;
    if mstatus_sie == 0 {
        return;
    }
    if (reg.mip & reg.mie) == 0 {
        return;
    }
    reg.mepc = reg.pc;
    reg.pc = reg.mtvec;
}

fn s_int(reg: &mut Register, mode: &mut Mode) {
    *mode = Mode::S;
    let sstatus_sie = (reg.sstatus & 0b10) >> 1;
    if sstatus_sie == 0 {
        return;
    }
    if (reg.sip & reg.sie) == 0 {
        return;
    }
    reg.sepc = reg.pc;
    reg.pc = reg.stvec;
}

fn m_exception(reg: &mut Register, mode: &mut Mode) {
    *mode = Mode::M;
}

fn s_exception(reg: &mut Register, mode: &mut Mode) {
    *mode = Mode::S;
}
