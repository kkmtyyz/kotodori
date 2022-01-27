pub mod instructions;
use crate::conf::MEM_OFF;
use chrono::{DateTime, Duration, Local};

use crate::bus::Bus;
use crate::conf;
use instructions::InstName;
use instructions::Instruction;

const MTIME: u64 = 0x200_BFF8;
const MTIMECMP: u64 = 0x200_4000;

#[derive(Debug)]
enum Mode {
    M,
    S,
    U,
}

#[derive(Debug, PartialEq)]
enum PMPPerm {
    R,
    W,
    X,
}

#[derive(Debug)]
pub struct Cpu {
    bus: Bus,
    mem_reserved_w: Vec<u8>,
    time: DateTime<Local>,
    mode: Mode, // privilege mode

    // memory mapped
    mtime: u64,
    mtimecmp: u64,

    // registers
    zero: u64,
    ra: u64,
    sp: u64,
    gp: u64,
    tp: u64,
    t0: u64,
    t1: u64,
    t2: u64,
    fp: u64, // s0
    s1: u64,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
    t3: u64,
    t4: u64,
    t5: u64,
    t6: u64,

    pc: u64,

    // supervisor-level csr
    sstatus: u64,    // 0x100
    sie: u64,        // 0x104
    stvec: u64,      // 0x105
    scounteren: u64, // 0x106
    senvcfg: u64,    // 0x10A
    sscratch: u64,   // 0x140
    sepc: u64,       // 0x141
    scause: u64,     // 0x142
    stval: u64,      // 0x143
    sip: u64,        // 0x144
    satp: u64,       // 0x180
    scontext: u64,   // 0x5A8

    // machine-level csr
    mvendorid: u64,  // 0xF11
    marchid: u64,    // 0xF12
    mimpid: u64,     // 0xF13
    mhartid: u64,    // 0xF14
    mconfigptr: u64, // 0xF15
    mstatus: u64,    // 0x300
    misa: u64,       // 0x301
    medeleg: u64,    // 0x302
    mideleg: u64,    // 0x303
    mie: u64,        // 0x304
    mtvec: u64,      // 0x305
    mcounteren: u64, // 0x306
    mstatush: u64,   // 0x310
    mscratch: u64,   // 0x340
    mepc: u64,       // 0x341
    mcause: u64,     // 0x342
    mtval: u64,      // 0x343
    mip: u64,        // 0x344
    mtinst: u64,     // 0x34A
    mtval2: u64,     // 0x34B
    menvcfg: u64,    // 0x30A
    menvcfgh: u64,   // 0x31A
    mseccfg: u64,    // 0x747
    mseccfgh: u64,   // 0x757
    pmpcfg0: u64,    // 0x3A0
    pmpcfg1: u64,    // 0x3A1
    pmpcfg2: u64,    // 0x3A2
    pmpcfg3: u64,    // 0x3A3
    pmpcfg4: u64,    // 0x3A4
    pmpcfg5: u64,    // 0x3A5
    pmpcfg6: u64,    // 0x3A6
    pmpcfg7: u64,    // 0x3A7
    pmpcfg8: u64,    // 0x3A8
    pmpcfg9: u64,    // 0x3A9
    pmpcfg10: u64,   // 0x3AA
    pmpcfg11: u64,   // 0x3AB
    pmpcfg12: u64,   // 0x3AC
    pmpcfg13: u64,   // 0x3AD
    pmpcfg14: u64,   // 0x3AE
    pmpcfg15: u64,   // 0x3AF
    pmpaddr0: u64,   // 0x3B0
    pmpaddr1: u64,   // 0x3B1
    pmpaddr2: u64,   // 0x3B2
    pmpaddr3: u64,   // 0x3B3
    pmpaddr4: u64,   // 0x3B4
    pmpaddr5: u64,   // 0x3B5
    pmpaddr6: u64,   // 0x3B6
    pmpaddr7: u64,   // 0x3B7
    pmpaddr8: u64,   // 0x3B8
    pmpaddr9: u64,   // 0x3B9
    pmpaddr10: u64,  // 0x3BA
    pmpaddr11: u64,  // 0x3BB
    pmpaddr12: u64,  // 0x3BC
    pmpaddr13: u64,  // 0x3BD
    pmpaddr14: u64,  // 0x3BE
    pmpaddr15: u64,  // 0x3BF
    pmpaddr16: u64,  // 0x3C0
    pmpaddr17: u64,  // 0x3C1
    pmpaddr18: u64,  // 0x3C2
    pmpaddr19: u64,  // 0x3C3
    pmpaddr20: u64,  // 0x3C4
    pmpaddr21: u64,  // 0x3C5
    pmpaddr22: u64,  // 0x3C6
    pmpaddr23: u64,  // 0x3C7
    pmpaddr24: u64,  // 0x3C8
    pmpaddr25: u64,  // 0x3C9
    pmpaddr26: u64,  // 0x3CA
    pmpaddr27: u64,  // 0x3CB
    pmpaddr28: u64,  // 0x3CC
    pmpaddr29: u64,  // 0x3CD
    pmpaddr30: u64,  // 0x3CE
    pmpaddr31: u64,  // 0x3CF
    pmpaddr32: u64,  // 0x3D0
    pmpaddr33: u64,  // 0x3D1
    pmpaddr34: u64,  // 0x3D2
    pmpaddr35: u64,  // 0x3D3
    pmpaddr36: u64,  // 0x3D4
    pmpaddr37: u64,  // 0x3D5
    pmpaddr38: u64,  // 0x3D6
    pmpaddr39: u64,  // 0x3D7
    pmpaddr40: u64,  // 0x3D8
    pmpaddr41: u64,  // 0x3D9
    pmpaddr42: u64,  // 0x3DA
    pmpaddr43: u64,  // 0x3DB
    pmpaddr44: u64,  // 0x3DC
    pmpaddr45: u64,  // 0x3DD
    pmpaddr46: u64,  // 0x3DE
    pmpaddr47: u64,  // 0x3DF
    pmpaddr48: u64,  // 0x3E0
    pmpaddr49: u64,  // 0x3E1
    pmpaddr50: u64,  // 0x3E2
    pmpaddr51: u64,  // 0x3E3
    pmpaddr52: u64,  // 0x3E4
    pmpaddr53: u64,  // 0x3E5
    pmpaddr54: u64,  // 0x3E6
    pmpaddr55: u64,  // 0x3E7
    pmpaddr56: u64,  // 0x3E8
    pmpaddr57: u64,  // 0x3E9
    pmpaddr58: u64,  // 0x3EA
    pmpaddr59: u64,  // 0x3EB
    pmpaddr60: u64,  // 0x3EC
    pmpaddr61: u64,  // 0x3ED
    pmpaddr62: u64,  // 0x3EE
    pmpaddr63: u64,  // 0x3EF
}

impl Cpu {
    pub fn new(bus: Bus, mem_size: usize) -> Cpu {
        Cpu {
            bus,
            mem_reserved_w: vec![0; mem_size / 32],
            time: Local::now(),
            mode: Mode::M,

            mtime: 0,
            mtimecmp: 0,

            zero: 0,
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            fp: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            pc: 0,

            // supervisor-level csr
            sstatus: 0,    // 0x100
            sie: 0,        // 0x104
            stvec: 0,      // 0x105
            scounteren: 0, // 0x106
            senvcfg: 0,    // 0x10A
            sscratch: 0,   // 0x140
            sepc: 0,       // 0x141
            scause: 0,     // 0x142
            stval: 0,      // 0x143
            sip: 0,        // 0x144
            satp: 0,       // 0x180
            scontext: 0,   // 0x5A8

            // machine-level csr
            mvendorid: 0,  // 0xF11
            marchid: 0,    // 0xF12
            mimpid: 0,     // 0xF13
            mhartid: 0,    // 0xF14
            mconfigptr: 0, // 0xF15
            mstatus: 0,    // 0x300
            misa: 0,       // 0x301
            medeleg: 0,    // 0x302
            mideleg: 0,    // 0x303
            mie: 0,        // 0x304
            mtvec: 0,      // 0x305
            mcounteren: 0, // 0x306
            mstatush: 0,   // 0x310
            mscratch: 0,   // 0x340
            mepc: 0,       // 0x341
            mcause: 0,     // 0x342
            mtval: 0,      // 0x343
            mip: 0,        // 0x344
            mtinst: 0,     // 0x34A
            mtval2: 0,     // 0x34B
            menvcfg: 0,    // 0x30A
            menvcfgh: 0,   // 0x31A
            mseccfg: 0,    // 0x747
            mseccfgh: 0,   // 0x757
            pmpcfg0: 0,    // 0x3A0
            pmpcfg1: 0,    // 0x3A1
            pmpcfg2: 0,    // 0x3A2
            pmpcfg3: 0,    // 0x3A3
            pmpcfg4: 0,    // 0x3A4
            pmpcfg5: 0,    // 0x3A5
            pmpcfg6: 0,    // 0x3A6
            pmpcfg7: 0,    // 0x3A7
            pmpcfg8: 0,    // 0x3A8
            pmpcfg9: 0,    // 0x3A9
            pmpcfg10: 0,   // 0x3AA
            pmpcfg11: 0,   // 0x3AB
            pmpcfg12: 0,   // 0x3AC
            pmpcfg13: 0,   // 0x3AD
            pmpcfg14: 0,   // 0x3AE
            pmpcfg15: 0,   // 0x3AF
            pmpaddr0: 0,   // 0x3B0
            pmpaddr1: 0,   // 0x3B1
            pmpaddr2: 0,   // 0x3B2
            pmpaddr3: 0,   // 0x3B3
            pmpaddr4: 0,   // 0x3B4
            pmpaddr5: 0,   // 0x3B5
            pmpaddr6: 0,   // 0x3B6
            pmpaddr7: 0,   // 0x3B7
            pmpaddr8: 0,   // 0x3B8
            pmpaddr9: 0,   // 0x3B9
            pmpaddr10: 0,  // 0x3BA
            pmpaddr11: 0,  // 0x3BB
            pmpaddr12: 0,  // 0x3BC
            pmpaddr13: 0,  // 0x3BD
            pmpaddr14: 0,  // 0x3BE
            pmpaddr15: 0,  // 0x3BF
            pmpaddr16: 0,  // 0x3C0
            pmpaddr17: 0,  // 0x3C1
            pmpaddr18: 0,  // 0x3C2
            pmpaddr19: 0,  // 0x3C3
            pmpaddr20: 0,  // 0x3C4
            pmpaddr21: 0,  // 0x3C5
            pmpaddr22: 0,  // 0x3C6
            pmpaddr23: 0,  // 0x3C7
            pmpaddr24: 0,  // 0x3C8
            pmpaddr25: 0,  // 0x3C9
            pmpaddr26: 0,  // 0x3CA
            pmpaddr27: 0,  // 0x3CB
            pmpaddr28: 0,  // 0x3CC
            pmpaddr29: 0,  // 0x3CD
            pmpaddr30: 0,  // 0x3CE
            pmpaddr31: 0,  // 0x3CF
            pmpaddr32: 0,  // 0x3D0
            pmpaddr33: 0,  // 0x3D1
            pmpaddr34: 0,  // 0x3D2
            pmpaddr35: 0,  // 0x3D3
            pmpaddr36: 0,  // 0x3D4
            pmpaddr37: 0,  // 0x3D5
            pmpaddr38: 0,  // 0x3D6
            pmpaddr39: 0,  // 0x3D7
            pmpaddr40: 0,  // 0x3D8
            pmpaddr41: 0,  // 0x3D9
            pmpaddr42: 0,  // 0x3DA
            pmpaddr43: 0,  // 0x3DB
            pmpaddr44: 0,  // 0x3DC
            pmpaddr45: 0,  // 0x3DD
            pmpaddr46: 0,  // 0x3DE
            pmpaddr47: 0,  // 0x3DF
            pmpaddr48: 0,  // 0x3E0
            pmpaddr49: 0,  // 0x3E1
            pmpaddr50: 0,  // 0x3E2
            pmpaddr51: 0,  // 0x3E3
            pmpaddr52: 0,  // 0x3E4
            pmpaddr53: 0,  // 0x3E5
            pmpaddr54: 0,  // 0x3E6
            pmpaddr55: 0,  // 0x3E7
            pmpaddr56: 0,  // 0x3E8
            pmpaddr57: 0,  // 0x3E9
            pmpaddr58: 0,  // 0x3EA
            pmpaddr59: 0,  // 0x3EB
            pmpaddr60: 0,  // 0x3EC
            pmpaddr61: 0,  // 0x3ED
            pmpaddr62: 0,  // 0x3EE
            pmpaddr63: 0,  // 0x3EF
        }
    }

    pub fn print(&self) {
        println!("zero(0x000):\t0x{:016X}, 0b{:064b}", self.zero, self.zero);
        println!("ra(0x001):\t0x{:016X}, 0b{:064b}", self.ra, self.ra);
        println!("sp(0x002):\t0x{:016X}, 0b{:064b}", self.sp, self.sp);
        println!("gp(0x003):\t0x{:016X}, 0b{:064b}", self.gp, self.gp);
        println!("tp(0x004):\t0x{:016X}, 0b{:064b}", self.tp, self.tp);
        println!("t0(0x005):\t0x{:016X}, 0b{:064b}", self.t0, self.t0);
        println!("t1(0x006):\t0x{:016X}, 0b{:064b}", self.t1, self.t1);
        println!("t2(0x007):\t0x{:016X}, 0b{:064b}", self.t2, self.t2);
        println!("fp(0x008):\t0x{:016X}, 0b{:064b}", self.fp, self.fp);
        println!("s1(0x009):\t0x{:016X}, 0b{:064b}", self.s1, self.s1);
        println!("a0(0x00A):\t0x{:016X}, 0b{:064b}", self.a0, self.a0);
        println!("a1(0x00B):\t0x{:016X}, 0b{:064b}", self.a1, self.a1);
        println!("a2(0x00C):\t0x{:016X}, 0b{:064b}", self.a2, self.a2);
        println!("a3(0x00D):\t0x{:016X}, 0b{:064b}", self.a3, self.a3);
        println!("a4(0x00E):\t0x{:016X}, 0b{:064b}", self.a4, self.a4);
        println!("a5(0x00F):\t0x{:016X}, 0b{:064b}", self.a5, self.a5);
        println!("a6(0x010):\t0x{:016X}, 0b{:064b}", self.a6, self.a6);
        println!("a7(0x011):\t0x{:016X}, 0b{:064b}", self.a7, self.a7);
        println!("s2(0x012):\t0x{:016X}, 0b{:064b}", self.s5, self.s5);
        println!("s3(0x013):\t0x{:016X}, 0b{:064b}", self.s3, self.s3);
        println!("s4(0x014):\t0x{:016X}, 0b{:064b}", self.s4, self.s4);
        println!("s5(0x015):\t0x{:016X}, 0b{:064b}", self.s5, self.s5);
        println!("s6(0x016):\t0x{:016X}, 0b{:064b}", self.s6, self.s6);
        println!("s7(0x017):\t0x{:016X}, 0b{:064b}", self.s7, self.s7);
        println!("s8(0x018):\t0x{:016X}, 0b{:064b}", self.s8, self.s8);
        println!("s9(0x019):\t0x{:016X}, 0b{:064b}", self.s9, self.s9);
        println!("s10(0x01A):\t0x{:016X}, 0b{:064b}", self.s10, self.s10);
        println!("s11(0x01B):\t0x{:016X}, 0b{:064b}", self.s11, self.s11);
        println!("t3(0x01C):\t0x{:016X}, 0b{:064b}", self.t3, self.t3);
        println!("t4(0x01D):\t0x{:016X}, 0b{:064b}", self.t4, self.t4);
        println!("t5(0x01E):\t0x{:016X}, 0b{:064b}", self.t5, self.t5);
        println!("t6(0x01F):\t0x{:016X}, 0b{:064b}", self.t6, self.t6);
        println!("pc(0x020):\t0x{:016X}, 0b{:064b}", self.pc, self.pc);
        println!(
            "mstatus(0x300):\t0x{:016X}, 0b{:064b}",
            self.mstatus, self.mstatus
        );
        println!("mie(0x304):\t0x{:016X}, 0b{:064b}", self.mie, self.mie);
        println!("mip(0x344):\t0x{:016X}, 0b{:064b}", self.mip, self.mip);
        println!("mepc(0x341):\t0x{:016X}, 0b{:064b}", self.mepc, self.mepc);
        println!(
            "medeleg(0x302):\t0x{:016X}, 0b{:064b}",
            self.medeleg, self.medeleg
        );
        println!(
            "mideleg(0x303):\t0x{:016X}, 0b{:064b}",
            self.mideleg, self.mideleg
        );
        println!(
            "sstatus(0x100):\t0x{:016X}, 0b{:064b}",
            self.sstatus, self.sstatus
        );
        println!("sie(0x104):\t0x{:016X}, 0b{:064b}", self.sie, self.sie);
        println!("satp(0x180):\t0x{:016X}, 0b{:064b}", self.satp, self.satp);
        println!(
            "pmpaddr0(0x3B0):0x{:016X}, 0b{:064b}",
            self.pmpaddr0, self.pmpaddr0
        );
        println!(
            "pmpcfg0(0x3A0):\t0x{:016X}, 0b{:064b}",
            self.pmpcfg0, self.pmpcfg0
        );
        println!("mtime:\t\t0x{:016X}, 0b{:064b}", self.mtime, self.mtime);
        println!(
            "mtimecmp:\t0x{:016X}, 0b{:064b}",
            self.mtimecmp, self.mtimecmp
        );
        println!(
            "mtvec(0x305):\t0x{:016X}, 0b{:064b}",
            self.mtvec, self.mtvec
        );
    }

    pub fn pdram_range(&self, begin: usize, end: usize) {
        self.bus.pdram_range(begin, end);
    }

    pub fn get_reg(&self, reg: u8) -> u64 {
        match reg {
            0b0_0000 => self.zero,
            0b0_0001 => self.ra,
            0b0_0010 => self.sp,
            0b0_0011 => self.gp,
            0b0_0100 => self.tp,
            0b0_0101 => self.t0,
            0b0_0110 => self.t1,
            0b0_0111 => self.t2,
            0b0_1000 => self.fp,
            0b0_1001 => self.s1,
            0b0_1010 => self.a0,
            0b0_1011 => self.a1,
            0b0_1100 => self.a2,
            0b0_1101 => self.a3,
            0b0_1110 => self.a4,
            0b0_1111 => self.a5,
            0b1_0000 => self.a6,
            0b1_0001 => self.a7,
            0b1_0010 => self.s2,
            0b1_0011 => self.s3,
            0b1_0100 => self.s4,
            0b1_0101 => self.s5,
            0b1_0110 => self.s6,
            0b1_0111 => self.s7,
            0b1_1000 => self.s8,
            0b1_1001 => self.s9,
            0b1_1010 => self.s10,
            0b1_1011 => self.s11,
            0b1_1100 => self.t3,
            0b1_1101 => self.t4,
            0b1_1110 => self.t5,
            0b1_1111 => self.t6,
            _ => panic!("invalid register"),
        }
    }

    pub fn get_csr(&self, reg: u16) -> u64 {
        match reg {
            // supervisor-level csr
            0x100 => self.sstatus,
            0x104 => self.sie,
            0x105 => self.stvec,
            0x106 => self.scounteren,
            0x10A => self.senvcfg,
            0x140 => self.sscratch,
            0x141 => self.sepc,
            0x142 => self.scause,
            0x143 => self.stval,
            0x144 => self.sip,
            0x180 => self.satp,
            0x5A8 => self.scontext,

            // machine-level csr
            0xF11 => self.mvendorid,
            0xF12 => self.marchid,
            0xF13 => self.mimpid,
            0xF14 => self.mhartid,
            0xF15 => self.mconfigptr,
            0x300 => self.mstatus,
            0x301 => self.misa,
            0x302 => self.medeleg,
            0x303 => self.mideleg,
            0x304 => self.mie,
            0x305 => self.mtvec,
            0x306 => self.mcounteren,
            0x310 => self.mstatush,
            0x340 => self.mscratch,
            0x341 => self.mepc,
            0x342 => self.mcause,
            0x343 => self.mtval,
            0x344 => self.mip,
            0x34A => self.mtinst,
            0x34B => self.mtval2,
            0x30A => self.menvcfg,
            0x31A => self.menvcfgh,
            0x747 => self.mseccfg,
            0x757 => self.mseccfgh,
            0x3A0 => self.pmpcfg0,
            0x3A1 => self.pmpcfg1,
            0x3A2 => self.pmpcfg2,
            0x3A3 => self.pmpcfg3,
            0x3A4 => self.pmpcfg4,
            0x3A5 => self.pmpcfg5,
            0x3A6 => self.pmpcfg6,
            0x3A7 => self.pmpcfg7,
            0x3A8 => self.pmpcfg8,
            0x3A9 => self.pmpcfg9,
            0x3AA => self.pmpcfg10,
            0x3AB => self.pmpcfg11,
            0x3AC => self.pmpcfg12,
            0x3AD => self.pmpcfg13,
            0x3AE => self.pmpcfg14,
            0x3AF => self.pmpcfg15,
            0x3B0 => self.pmpaddr0,
            0x3B1 => self.pmpaddr1,
            0x3B2 => self.pmpaddr2,
            0x3B3 => self.pmpaddr3,
            0x3B4 => self.pmpaddr4,
            0x3B5 => self.pmpaddr5,
            0x3B6 => self.pmpaddr6,
            0x3B7 => self.pmpaddr7,
            0x3B8 => self.pmpaddr8,
            0x3B9 => self.pmpaddr9,
            0x3BA => self.pmpaddr10,
            0x3BB => self.pmpaddr11,
            0x3BC => self.pmpaddr12,
            0x3BD => self.pmpaddr13,
            0x3BE => self.pmpaddr14,
            0x3BF => self.pmpaddr15,
            0x3C0 => self.pmpaddr16,
            0x3C1 => self.pmpaddr17,
            0x3C2 => self.pmpaddr18,
            0x3C3 => self.pmpaddr19,
            0x3C4 => self.pmpaddr20,
            0x3C5 => self.pmpaddr21,
            0x3C6 => self.pmpaddr22,
            0x3C7 => self.pmpaddr23,
            0x3C8 => self.pmpaddr24,
            0x3C9 => self.pmpaddr25,
            0x3CA => self.pmpaddr26,
            0x3CB => self.pmpaddr27,
            0x3CC => self.pmpaddr28,
            0x3CD => self.pmpaddr29,
            0x3CE => self.pmpaddr30,
            0x3CF => self.pmpaddr31,
            0x3D0 => self.pmpaddr32,
            0x3D1 => self.pmpaddr33,
            0x3D2 => self.pmpaddr34,
            0x3D3 => self.pmpaddr35,
            0x3D4 => self.pmpaddr36,
            0x3D5 => self.pmpaddr37,
            0x3D6 => self.pmpaddr38,
            0x3D7 => self.pmpaddr39,
            0x3D8 => self.pmpaddr40,
            0x3D9 => self.pmpaddr41,
            0x3DA => self.pmpaddr42,
            0x3DB => self.pmpaddr43,
            0x3DC => self.pmpaddr44,
            0x3DD => self.pmpaddr45,
            0x3DE => self.pmpaddr46,
            0x3DF => self.pmpaddr47,
            0x3E0 => self.pmpaddr48,
            0x3E1 => self.pmpaddr49,
            0x3E2 => self.pmpaddr50,
            0x3E3 => self.pmpaddr51,
            0x3E4 => self.pmpaddr52,
            0x3E5 => self.pmpaddr53,
            0x3E6 => self.pmpaddr54,
            0x3E7 => self.pmpaddr55,
            0x3E8 => self.pmpaddr56,
            0x3E9 => self.pmpaddr57,
            0x3EA => self.pmpaddr58,
            0x3EB => self.pmpaddr59,
            0x3EC => self.pmpaddr60,
            0x3ED => self.pmpaddr61,
            0x3EE => self.pmpaddr62,
            0x3EF => self.pmpaddr63,

            _ => panic!("invalid register"),
        }
    }

    pub fn set_reg(&mut self, reg: u8, value: u64) {
        match reg {
            0b0_0000 => (),
            0b0_0001 => self.ra = value,
            0b0_0010 => self.sp = value,
            0b0_0011 => self.gp = value,
            0b0_0100 => self.tp = value,
            0b0_0101 => self.t0 = value,
            0b0_0110 => self.t1 = value,
            0b0_0111 => self.t2 = value,
            0b0_1000 => self.fp = value,
            0b0_1001 => self.s1 = value,
            0b0_1010 => self.a0 = value,
            0b0_1011 => self.a1 = value,
            0b0_1100 => self.a2 = value,
            0b0_1101 => self.a3 = value,
            0b0_1110 => self.a4 = value,
            0b0_1111 => self.a5 = value,
            0b1_0000 => self.a6 = value,
            0b1_0001 => self.a7 = value,
            0b1_0010 => self.s2 = value,
            0b1_0011 => self.s3 = value,
            0b1_0100 => self.s4 = value,
            0b1_0101 => self.s5 = value,
            0b1_0110 => self.s6 = value,
            0b1_0111 => self.s7 = value,
            0b1_1000 => self.s8 = value,
            0b1_1001 => self.s9 = value,
            0b1_1010 => self.s10 = value,
            0b1_1011 => self.s11 = value,
            0b1_1100 => self.t3 = value,
            0b1_1101 => self.t4 = value,
            0b1_1110 => self.t5 = value,
            0b1_1111 => self.t6 = value,
            _ => panic!("invalid register"),
        }
    }

    pub fn set_csr(&mut self, reg: u16, value: u64) {
        match reg {
            // supervisor-level csr
            0x100 => self.sstatus = value,
            0x104 => self.sie = value,
            0x105 => self.stvec = value,
            0x106 => self.scounteren = value,
            0x10A => self.senvcfg = value,
            0x140 => self.sscratch = value,
            0x141 => self.sepc = value,
            0x142 => self.scause = value,
            0x143 => self.stval = value,
            0x144 => self.sip = value,
            0x180 => self.satp = value,
            0x5A8 => self.scontext = value,

            // machine-level csr
            0xF11 => self.mvendorid = value,
            0xF12 => self.marchid = value,
            0xF13 => self.mimpid = value,
            0xF14 => self.mhartid = value,
            0xF15 => self.mconfigptr = value,
            0x300 => self.mstatus = value,
            0x301 => self.misa = value,
            0x302 => self.medeleg = value,
            0x303 => self.mideleg = value,
            0x304 => self.mie = value,
            0x305 => self.mtvec = value,
            0x306 => self.mcounteren = value,
            0x310 => self.mstatush = value,
            0x340 => self.mscratch = value,
            0x341 => self.mepc = value,
            0x342 => self.mcause = value,
            0x343 => self.mtval = value,
            0x344 => self.mip = value,
            0x34A => self.mtinst = value,
            0x34B => self.mtval2 = value,
            0x30A => self.menvcfg = value,
            0x31A => self.menvcfgh = value,
            0x747 => self.mseccfg = value,
            0x757 => self.mseccfgh = value,
            0x3A0 => self.pmpcfg0 = value,
            0x3A1 => self.pmpcfg1 = value,
            0x3A2 => self.pmpcfg2 = value,
            0x3A3 => self.pmpcfg3 = value,
            0x3A4 => self.pmpcfg4 = value,
            0x3A5 => self.pmpcfg5 = value,
            0x3A6 => self.pmpcfg6 = value,
            0x3A7 => self.pmpcfg7 = value,
            0x3A8 => self.pmpcfg8 = value,
            0x3A9 => self.pmpcfg9 = value,
            0x3AA => self.pmpcfg10 = value,
            0x3AB => self.pmpcfg11 = value,
            0x3AC => self.pmpcfg12 = value,
            0x3AD => self.pmpcfg13 = value,
            0x3AE => self.pmpcfg14 = value,
            0x3AF => self.pmpcfg15 = value,
            0x3B0 => self.pmpaddr0 = value,
            0x3B1 => self.pmpaddr1 = value,
            0x3B2 => self.pmpaddr2 = value,
            0x3B3 => self.pmpaddr3 = value,
            0x3B4 => self.pmpaddr4 = value,
            0x3B5 => self.pmpaddr5 = value,
            0x3B6 => self.pmpaddr6 = value,
            0x3B7 => self.pmpaddr7 = value,
            0x3B8 => self.pmpaddr8 = value,
            0x3B9 => self.pmpaddr9 = value,
            0x3BA => self.pmpaddr10 = value,
            0x3BB => self.pmpaddr11 = value,
            0x3BC => self.pmpaddr12 = value,
            0x3BD => self.pmpaddr13 = value,
            0x3BE => self.pmpaddr14 = value,
            0x3BF => self.pmpaddr15 = value,
            0x3C0 => self.pmpaddr16 = value,
            0x3C1 => self.pmpaddr17 = value,
            0x3C2 => self.pmpaddr18 = value,
            0x3C3 => self.pmpaddr19 = value,
            0x3C4 => self.pmpaddr20 = value,
            0x3C5 => self.pmpaddr21 = value,
            0x3C6 => self.pmpaddr22 = value,
            0x3C7 => self.pmpaddr23 = value,
            0x3C8 => self.pmpaddr24 = value,
            0x3C9 => self.pmpaddr25 = value,
            0x3CA => self.pmpaddr26 = value,
            0x3CB => self.pmpaddr27 = value,
            0x3CC => self.pmpaddr28 = value,
            0x3CD => self.pmpaddr29 = value,
            0x3CE => self.pmpaddr30 = value,
            0x3CF => self.pmpaddr31 = value,
            0x3D0 => self.pmpaddr32 = value,
            0x3D1 => self.pmpaddr33 = value,
            0x3D2 => self.pmpaddr34 = value,
            0x3D3 => self.pmpaddr35 = value,
            0x3D4 => self.pmpaddr36 = value,
            0x3D5 => self.pmpaddr37 = value,
            0x3D6 => self.pmpaddr38 = value,
            0x3D7 => self.pmpaddr39 = value,
            0x3D8 => self.pmpaddr40 = value,
            0x3D9 => self.pmpaddr41 = value,
            0x3DA => self.pmpaddr42 = value,
            0x3DB => self.pmpaddr43 = value,
            0x3DC => self.pmpaddr44 = value,
            0x3DD => self.pmpaddr45 = value,
            0x3DE => self.pmpaddr46 = value,
            0x3DF => self.pmpaddr47 = value,
            0x3E0 => self.pmpaddr48 = value,
            0x3E1 => self.pmpaddr49 = value,
            0x3E2 => self.pmpaddr50 = value,
            0x3E3 => self.pmpaddr51 = value,
            0x3E4 => self.pmpaddr52 = value,
            0x3E5 => self.pmpaddr53 = value,
            0x3E6 => self.pmpaddr54 = value,
            0x3E7 => self.pmpaddr55 = value,
            0x3E8 => self.pmpaddr56 = value,
            0x3E9 => self.pmpaddr57 = value,
            0x3EA => self.pmpaddr58 = value,
            0x3EB => self.pmpaddr59 = value,
            0x3EC => self.pmpaddr60 = value,
            0x3ED => self.pmpaddr61 = value,
            0x3EE => self.pmpaddr62 = value,
            0x3EF => self.pmpaddr63 = value,

            _ => panic!("invalid register"),
        }
    }

    fn l_mm(&self, addr: u64) -> u64 {
        match addr {
            MTIME => self.mtime,
            MTIMECMP => self.mtimecmp,
            _ => panic!("invalid memory mapped address"),
        }
    }

    fn s_mm(&mut self, addr: u64, data: u64) {
        match addr {
            MTIME => self.mtime = data,
            MTIMECMP => self.mtimecmp = data,
            _ => panic!("invalid memory mapped address: 0x{:016X}", addr),
        }
    }

    pub fn init(&mut self, entry_point: usize) {
        self.sp = conf::STACK_BOTTOM;
        self.pc = entry_point as u64;
    }

    pub fn run(&mut self) {
        loop {
            let data = self.fetch();
            let inst = Instruction::decode(data);
            println!("instruction: ");
            inst.print();
            println!("pc: 0x{:016X}", self.pc);

            let pre_pc = self.pc;
            self.exec_instruction(&inst);
            self.timer_int();
            self.int();

            if pre_pc == self.pc {
                self.pc += 4;
            }

            // println!("pc: 0x{:016X}", self.pc);
            let mut b = String::new();
            std::io::stdin().read_line(&mut b).ok();
            if b.trim() == "p".to_string() {
                self.print();
            } else if b.starts_with("m") {
                let mut b = b.split_whitespace();
                b.next(); // remove m
                let begin = hex_to_usize(b.next().unwrap());
                let end = hex_to_usize(b.next().unwrap());
                self.bus.pdram_range(begin, end);
            }
        }
    }

    fn fetch(&self) -> u32 {
        self.check_pmp(self.pc, PMPPerm::X);

        let mut ltl_data = self.bus.lw_dram(self.pc - MEM_OFF as u64); // little endian data
        let mut data: u32 = 0;
        for _ in 0..4 {
            data <<= 8;
            data |= ltl_data & 0xFF;
            ltl_data >>= 8;
        }
        data
    }

    fn timer_int(&mut self) {
        let mstatus_mie = self.mstatus & 0b1000;
        let mie_mtie = self.mie & 0b1000_0000;
        if mstatus_mie != 0b1000 && mie_mtie != 0b1000_0000 {
            return;
        }
        self.mip |= 0b1000_0000;

        let duration: Duration = Local::now() - self.time;
        if let Some(nano) = duration.num_nanoseconds() {
            if nano <= (self.mtimecmp as i64 * 100) {
                return;
            }
        }

        self.mepc = self.pc;
        self.pc = self.mtvec;
        self.mcause = 0x8000_0000_0000_0007; // timer interrupt
        let mstatus_mpie = mstatus_mie << 4;
        self.mstatus |= mstatus_mpie;
        self.mstatus &= !0b1000;

        //self.mstatus |= !0b1000; // mstatus.mie = 0

        self.mip &= !0b1000_000;
        self.time = Local::now();
    }

    fn int(&mut self) {
        let int = self.mcause >> 63;
        let e_code = self.mcause & 0x7FFF_FFFF_FFFF_FFFF;

        if int == 1 {
            if self.mideleg & e_code == e_code {
                self.s_int();
            } else {
                self.m_int();
            }
            return;
        }

        if self.medeleg & e_code == e_code {
            self.s_exception();
        } else {
            self.m_exception();
        }
    }

    fn m_int(&mut self) {
        self.mode = Mode::M;
        let mstatus_sie = (self.mstatus & 0b10) >> 1;
        if mstatus_sie == 0 {
            return;
        }
        if (self.mip & self.mie) == 0 {
            return;
        }
        self.mepc = self.pc;
        self.pc = self.mtvec;
    }

    fn s_int(&mut self) {
        self.mode = Mode::S;
        let sstatus_sie = (self.sstatus & 0b10) >> 1;
        if sstatus_sie == 0 {
            return;
        }
        if (self.sip & self.sie) == 0 {
            return;
        }
        self.sepc = self.pc;
        self.pc = self.stvec;
    }

    fn m_exception(&mut self) {
        self.mode = Mode::M;
    }

    fn s_exception(&mut self) {
        self.mode = Mode::S;
    }

    fn check_pmp(&self, addr: u64, perm: PMPPerm) {
        if let Mode::S | Mode::U = self.mode {
            // do PMP
            self.blocked_by_pmp(addr, perm);
            return;
        }

        let mprv = (self.mstatus & 0b10_0000_0000_0000_0000) >> 17;
        if let Mode::S | Mode::U = self.mode {
            if mprv == 0 && perm != PMPPerm::X {
                // do PMP
                self.blocked_by_pmp(addr, perm);
                return;
            }
        }

        let mpp = (self.mstatus & 0b1_1000_0000_0000) >> 11;
        if mprv == 1 && (mpp == 0 || mpp == 1) {
            // do PMP
            self.blocked_by_pmp(addr, perm);
            return;
        }
    }

    fn blocked_by_pmp(&self, addr: u64, perm: PMPPerm) {
        let pmpcfg_base = 0x3A0;
        let pmpaddr_base = 0x3B0;

        for i in 0..64 {
            let pmpcfg = self.get_csr(pmpcfg_base + (i / 8 * 2));
            let pmpcfg = pmpcfg >> ((i % 8) * 8);

            let a = (pmpcfg & 0b1_1000) >> 3;

            // OFF
            if a == 0 {
                continue;
            }

            // TOR
            if a == 1 {
                let begin: u64;
                if i == 0 {
                    begin = 0;
                } else {
                    begin = self.get_csr(pmpaddr_base + i - 1);
                }
                let end = self.get_csr(pmpaddr_base + i);

                if addr < begin && end <= addr {
                    continue;
                }

                match perm {
                    PMPPerm::R => {
                        if pmpcfg & 1 == 1 {
                            return;
                        }
                    }
                    PMPPerm::W => {
                        if pmpcfg & 2 == 0x10 {
                            return;
                        }
                    }
                    PMPPerm::X => {
                        if pmpcfg & 4 == 0x100 {
                            return;
                        }
                    }
                }
                panic!("Fetch blocked by PMP");
            }

            // NA4
            if a == 2 {
                panic!("PMP NA4 is not implemented");
            }

            // NAPOT
            if a == 3 {
                panic!("PMP NAPOT is not implemented");
            }
        }
    }

    fn exec_instruction(&mut self, inst: &Instruction) {
        match inst.name {
            // RV32I
            InstName::Lui(_) => self.lui(inst),
            InstName::Auipc(_) => self.auipc(inst),
            InstName::Jal(_) => self.jal(inst),
            InstName::Jalr(_) => self.jalr(inst),
            InstName::Beq(_) => self.beq(inst),
            InstName::Bne(_) => self.bne(inst),
            InstName::Blt(_) => self.blt(inst),
            InstName::Bge(_) => self.bge(inst),
            InstName::Bltu(_) => self.bltu(inst),
            InstName::Bgeu(_) => self.bgeu(inst),
            InstName::Lb(_) => self.lb(inst),
            InstName::Lh(_) => self.lh(inst),
            InstName::Lw(_) => self.lw(inst),
            InstName::Lbu(_) => self.lbu(inst),
            InstName::Lhu(_) => self.lhu(inst),
            InstName::Sb(_) => self.sb(inst),
            InstName::Sh(_) => self.sh(inst),
            InstName::Sw(_) => self.sw(inst),
            InstName::Addi(_) => self.addi(inst),
            InstName::Slti(_) => self.slti(inst),
            InstName::Sltiu(_) => self.sltiu(inst),
            InstName::Xori(_) => self.xori(inst),
            InstName::Ori(_) => self.ori(inst),
            InstName::Andi(_) => self.andi(inst),
            InstName::Slli(_) => self.slli(inst),
            InstName::Srli(_) => self.srli(inst),
            InstName::Srai(_) => self.srai(inst),
            InstName::Add(_) => self.add(inst),
            InstName::Sub(_) => self.sub(inst),
            InstName::Sll(_) => self.sll(inst),
            InstName::Slt(_) => self.slt(inst),
            InstName::Sltu(_) => self.sltu(inst),
            InstName::Xor(_) => self.xor(inst),
            InstName::Srl(_) => self.srl(inst),
            InstName::Sra(_) => self.sra(inst),
            InstName::Or(_) => self.or(inst),
            InstName::And(_) => self.and(inst),
            InstName::Fence(_) => self.fence(inst),
            InstName::FenceI(_) => self.fence_i(inst),
            InstName::Ecall(_) => self.ecall(inst),
            InstName::Ebreak(_) => self.ebreak(inst),
            InstName::Csrrw(_) => self.csrrw(inst),
            InstName::Csrrs(_) => self.csrrs(inst),
            InstName::Csrrc(_) => self.csrrc(inst),
            InstName::Csrrwi(_) => self.csrrwi(inst),
            InstName::Csrrsi(_) => self.csrrsi(inst),
            InstName::Csrrci(_) => self.csrrci(inst),
            InstName::Sret(_) => self.sret(inst),
            InstName::Mret(_) => self.mret(inst),
            InstName::Wfi(_) => self.wfi(inst),
            InstName::SfenceVma(_) => self.sfence_vma(inst),

            // RV32A
            InstName::Mul(_) => self.mul(inst),
            InstName::Mulh(_) => self.mulh(inst),
            InstName::Mulhsu(_) => self.mulhsu(inst),
            InstName::Mulhu(_) => self.mulhu(inst),
            InstName::Div(_) => self.div(inst),
            InstName::Divu(_) => self.divu(inst),
            InstName::Rem(_) => self.rem(inst),
            InstName::Remu(_) => self.remu(inst),

            // RV32A
            InstName::LrW(_) => self.lr_w(inst),
            InstName::ScW(_) => self.sc_w(inst),
            InstName::AmoswapW(_) => self.amoswap_w(inst),
            InstName::AmoaddW(_) => self.amoadd_w(inst),
            InstName::AmoxorW(_) => self.amoxor_w(inst),
            InstName::AmoandW(_) => self.amoand_w(inst),
            InstName::AmoorW(_) => self.amoor_w(inst),
            InstName::AmominW(_) => self.amomin_w(inst),
            InstName::AmomaxW(_) => self.amomax_w(inst),
            InstName::AmominuW(_) => self.amominu_w(inst),
            InstName::AmomaxuW(_) => self.amomaxu_w(inst),

            // RV64I
            InstName::Lwu(_) => self.lwu(inst),
            InstName::Ld(_) => self.ld(inst),
            InstName::Sd(_) => self.sd(inst),
            InstName::Addiw(_) => self.addiw(inst),
            InstName::Slliw(_) => self.slliw(inst),
            InstName::Srliw(_) => self.srliw(inst),
            InstName::Sraiw(_) => self.sraiw(inst),
            InstName::Addw(_) => self.addw(inst),
            InstName::Subw(_) => self.subw(inst),
            InstName::Sllw(_) => self.sllw(inst),
            InstName::Srlw(_) => self.srlw(inst),
            InstName::Sraw(_) => self.sraw(inst),

            // RV64M
            InstName::Mulw(_) => self.mulw(inst),
            InstName::Divw(_) => self.divw(inst),
            InstName::Divuw(_) => self.divuw(inst),
            InstName::Remw(_) => self.remw(inst),
            InstName::Remuw(_) => self.remuw(inst),

            // RV64A
            InstName::LrD(_) => self.lr_d(inst),
            InstName::ScD(_) => self.sc_d(inst),
            InstName::AmoswapD(_) => self.amoswap_d(inst),
            InstName::AmoaddD(_) => self.amoadd_d(inst),
            InstName::AmoxorD(_) => self.amoxor_d(inst),
            InstName::AmoandD(_) => self.amoand_d(inst),
            InstName::AmoorD(_) => self.amoor_d(inst),
            InstName::AmominD(_) => self.amomin_d(inst),
            InstName::AmomaxD(_) => self.amomax_d(inst),
            InstName::AmominuD(_) => self.amominu_d(inst),
            InstName::AmomaxuD(_) => self.amomaxu_d(inst),
        }
    }

    /// x[rd] = sext(immediate[31:12] << 12)
    fn lui(&mut self, inst: &Instruction) {
        let v = (inst.imm << 12) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = pc + sext(immediate[31:12] << 12)
    fn auipc(&mut self, inst: &Instruction) {
        let v = self.pc as i64 + (inst.imm << 12) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = pc+4; pc += sext(offset)
    fn jal(&mut self, inst: &Instruction) {
        let imm = b20_to_sign64(inst.imm);
        self.set_reg(inst.rd, self.pc + 4);
        let v = self.pc as i64 + imm;
        self.pc = v as u64;
    }

    /// t =pc+4; pc=(x[rs1]+sext(offset))&∼1; x[rd]=t
    fn jalr(&mut self, inst: &Instruction) {
        let t = self.pc + 4;

        let imm = b12_to_sign64(inst.imm);
        let v = (self.get_reg(inst.rs1) as i64 + imm) as u64;
        self.pc = v & !1;

        self.set_reg(inst.rd, t);
    }

    /// if (rs1 == rs2) pc += sext(offset)
    fn beq(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) == self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    /// if (rs1 != rs2) pc += sext(offset)
    fn bne(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) != self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    /// if (rs1 <s rs2) pc += sext(offset)
    fn blt(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) < (self.get_reg(inst.rs2) as i64) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    /// if (rs1 >=s rs2) pc += sext(offset)
    fn bge(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) >= (self.get_reg(inst.rs2) as i64) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    /// if (rs1 >u rs2) pc += sext(offset)
    fn bltu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    /// if (rs1 >=u rs2) pc += sext(offset)
    fn bgeu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) >= self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    /// x[rd] = sext(M[x[rs1] + sext(offset)][7:0])
    fn lb(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: i64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u8 as i64;
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.lb_dram(addr) as i64;
        }
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = sext(M[x[rs1] + sext(offset)][15:0])
    fn lh(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: i64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u16 as i64;
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.lh_dram(addr) as i64;
        }
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = sext(M[x[rs1] + sext(offset)][31:0])
    fn lw(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: i64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u32 as i64;
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.lw_dram(addr as u64) as i64;
        }
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][7:0]
    fn lbu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u8 as u64;
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.lb_dram(addr as u64) as u64;
        }
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][15:0]
    fn lhu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u16 as u64;
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.lh_dram(addr as u64) as u64;
        }
        self.set_reg(inst.rd, v);
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][7:0]
    fn sb(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.get_reg(inst.rs2) as u8;
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v as u64);
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            self.bus.sb_dram(addr as u64, v);
        }
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][15:0]
    fn sh(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.get_reg(inst.rs2) as u16;
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v as u64);
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            self.bus.sh_dram(addr as u64, v);
        }
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][31:0]
    fn sw(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.get_reg(inst.rs2) as u32;
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v as u64);
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            self.bus.sw_dram(addr as u64, v);
        }
    }

    /// x[rd] = x[rs1] + sext(immediate)
    fn addi(&mut self, inst: &Instruction) {
        let imm = b12_to_sign64(inst.imm);
        let v = self.get_reg(inst.rs1) as i64 + imm;
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] <s sext(immediate)
    fn slti(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) < (inst.imm as i64) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] <u sext(immediate)
    fn sltiu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < inst.imm as u64 {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] ^ sext(immediate)
    fn xori(&mut self, inst: &Instruction) {
        let v = (inst.imm as i64) ^ (self.get_reg(inst.rs1) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] | sext(immediate)
    fn ori(&mut self, inst: &Instruction) {
        let v = (inst.imm as i64) | (self.get_reg(inst.rs1) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] & sext(immediate)
    fn andi(&mut self, inst: &Instruction) {
        let v = (inst.imm as i64) & (self.get_reg(inst.rs1) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] << shamt
    fn slli(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>u shamt
    fn srli(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        let v = self.get_reg(inst.rs1) >> shamt;
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>s shamt
    fn srai(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        let v = (self.get_reg(inst.rs1) as i64) >> shamt;
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] + x[rs2]
    fn add(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) + self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] - x[rs2]
    fn sub(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) - self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] << x[rs2]
    fn sll(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v);
    }

    // x[rd] = x[rs1] <s x[rs2]
    fn slt(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) < (self.get_reg(inst.rs2) as i64) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] <u x[rs2]
    fn sltu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < self.get_reg(inst.rs2) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] ^ x[rs2]
    fn xor(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) ^ self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>u x[rs2]
    fn srl(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = self.get_reg(inst.rs1) >> shamt;
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>s x[rs2]
    fn sra(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = (self.get_reg(inst.rs1) as i64) >> shamt;
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] | x[rs2]
    fn or(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) | self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] & x[rs2]
    fn and(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) & self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    /// Fence(pred, succ)
    #[allow(unused_variables)]
    fn fence(&mut self, inst: &Instruction) {
        let pred = inst.imm >> 4 & 0b1111;
        let succ = inst.imm & 0b1111;
        // Implement when needed.
    }

    /// Fence(Store, Fetch)
    #[allow(unused_variables)]
    fn fence_i(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    /// RaiseException(EnvironmentCall)
    #[allow(unused_variables)]
    fn ecall(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    /// RaiseException(Breakpoint)
    #[allow(unused_variables)]
    fn ebreak(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    /// t = CSRs[csr]; CSRs[csr] = x[rs1]; x[rd] = t
    fn csrrw(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.get_csr(csr);
        self.set_csr(csr, self.get_reg(inst.rs1));
        self.set_reg(inst.rd, t);
    }

    /// t = CSRs[csr]; CSRs[csr] = t | x[rs1]; x[rd] = t
    fn csrrs(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.get_csr(csr);
        self.set_csr(csr, t | self.get_reg(inst.rs1));
        self.set_reg(inst.rd, t);
    }

    /// t = CSRs[csr]; CSRs[csr] = t &∼x[rs1]; x[rd] = t
    fn csrrc(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.get_csr(csr);
        self.set_csr(csr, t & !self.get_reg(inst.rs1));
        self.set_reg(inst.rd, t);
    }

    /// x[rd] = CSRs[csr]; CSRs[csr] = zimm
    fn csrrwi(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        self.set_reg(inst.rd, self.get_csr(csr));
        let zimm = inst.rs1;
        self.set_csr(csr, zimm as u64);
    }

    /// t = CSRs[csr]; CSRs[csr] = t | zimm; x[rd] = t
    fn csrrsi(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.get_csr(csr);
        let zimm = inst.rs1;
        self.set_csr(csr, t | zimm as u64);
        self.set_reg(inst.rd, t);
    }

    /// t = CSRs[csr]; CSRs[csr] = t &∼zimm; x[rd] = t
    fn csrrci(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.get_csr(csr);
        let zimm = inst.rs1;
        self.set_csr(csr, t & !(zimm as u64));
        self.set_reg(inst.rd, t);
    }

    /// ExceptionReturn(User)
    fn sret(&mut self, inst: &Instruction) {
        let pre_spp = (self.sstatus & 0b1_1000_0000_0000) >> 11;
        let spie = self.sstatus & 0b100_0000;
        let sie = spie >> 3;
        self.sstatus |= sie;
        let spie: u64 = 0b100_0000;
        self.sstatus |= spie;
        let spp: u64 = 0b1_1000_0000_0000;
        self.sstatus &= !spp; // mpp = 0; U-MODE
        let sprv: u64 = 0b10_0000_0000_0000_0000;
        self.sstatus &= !sprv; // mprv = 0;
        self.pc = self.sepc;

        match pre_spp {
            0 => self.mode = Mode::U,
            1 => self.mode = Mode::S,
            3 => self.mode = Mode::M,
            _ => (),
        }
    }

    /// ExceptionReturn(Machine)
    fn mret(&mut self, inst: &Instruction) {
        let pre_mpp = (self.mstatus & 0b1_1000_0000_0000) >> 11;
        let mpie = self.mstatus & 0b100_0000;
        let mie = mpie >> 3;
        self.mstatus |= mie;
        let mpie: u64 = 0b100_0000;
        self.mstatus |= mpie;
        let mpp: u64 = 0b1_1000_0000_0000;
        self.mstatus &= !mpp; // mpp = 0; U-MODE
        let mprv: u64 = 0b10_0000_0000_0000_0000;
        self.mstatus &= !mprv; // mprv = 0;
        self.pc = self.mepc;

        match pre_mpp {
            0 => self.mode = Mode::U,
            1 => self.mode = Mode::S,
            3 => self.mode = Mode::M,
            _ => (),
        }
    }

    /// while (noInterruptsPending) idle
    #[allow(unused_variables)]
    fn wfi(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    /// Fence(Store, AddressTranslation)
    #[allow(unused_variables)]
    fn sfence_vma(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    /// x[rd] = x[rs1] × x[rs2]
    fn mul(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) * self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = (x[rs1] s×s x[rs2]) >>s XLEN
    fn mulh(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as i64) * (self.get_reg(inst.rs2) as i64);
        self.set_reg(inst.rd, (v >> 32) as u64);
    }

    /// x[rd] = (x[rs1] s × x[rs2]) >>s XLEN
    fn mulhsu(&mut self, inst: &Instruction) {
        let v = ((self.get_reg(inst.rs1) as i64) as u64) * self.get_reg(inst.rs2);
        self.set_reg(inst.rd, (v >> 32) as u64);
    }

    /// x[rd] = (x[rs1] u × x[rs2]) >>u XLEN
    fn mulhu(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) * self.get_reg(inst.rs2);
        self.set_reg(inst.rd, (v >> 32) as u64);
    }

    /// x[rd] = x[rs1] /s x[rs2]
    fn div(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as i64) / (self.get_reg(inst.rs2) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] /u x[rs2]
    fn divu(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) / self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] %s x[rs2]
    fn rem(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as i64) % (self.get_reg(inst.rs2) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] %u x[rs2]
    fn remu(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) % self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = LoadReserved32(M[x[rs1]])
    fn lr_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::R);

        let data = self.bus.lw_dram(addr) as i64;
        self.set_reg(inst.rd, data as u64);
        self.reserve_mem(addr, false);
    }

    fn reserve_mem(&mut self, addr: u64, d: bool) {
        if addr % 4 != 0 {
            panic!("invalid alignment");
        }

        let idx = addr / 32;
        match self.mem_reserved_w.get(idx as usize) {
            Some(rsv) => {
                let mut bit = 0x80;
                if d {
                    bit = 0xC0;
                }
                let rsv = (*rsv as usize) | (bit >> ((addr - idx * 32) / 4));
                self.mem_reserved_w[idx as usize] = rsv as u8;
            }
            None => panic!("invalid memory reserved word index"),
        }
    }

    fn sc_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        self.invalidate_mem_reservation(addr, false);
        let data = self.get_reg(inst.rs2) as u32;

        if self.check_mem_reservation(addr, false) {
            panic!("memory is not reserved");
        }

        self.bus.sw_dram(addr, data);
        self.set_reg(inst.rd, 0);
    }

    fn check_mem_reservation(&self, addr: u64, d: bool) -> bool {
        if addr % 4 != 0 {
            panic!("invalid alinement");
        }

        let idx = addr / 32;
        let rsv = self.mem_reserved_w.get(idx as usize).unwrap();
        let mut bit = 0x80;
        if d {
            bit = 0xC0;
        }
        let rsv = (*rsv as usize) & (bit >> ((addr - idx * 32) / 4));
        if rsv == 0 {
            return false;
        }
        true
    }

    fn invalidate_mem_reservation(&mut self, addr: u64, d: bool) {
        if addr & 4 != 0 {
            panic!("invalid alinement");
        }

        let idx = addr / 32;
        let rsv = self.mem_reserved_w.get(idx as usize).unwrap();
        let mut bit = 0x80;
        if d {
            bit = 0xC0;
        }
        let rsv = (*rsv as usize) & !(bit >> ((addr - idx * 32) / 4));
        self.mem_reserved_w[idx as usize] = rsv as u8;
    }

    /// x[rd] = AMO32(M[x[rs1]] SWAP x[rs2])
    fn amoswap_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.lw_dram(addr) as i64;
        self.set_reg(inst.rd, data as u64);
        self.bus.sw_dram(addr, self.get_reg(inst.rs2) as i32 as u32);
        self.set_reg(inst.rs2, data as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] + x[rs2])
    fn amoadd_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.lw_dram(addr) as i32;
        self.set_reg(inst.rd, data as i64 as u64);
        data += self.get_reg(inst.rs2) as i32;
        self.bus.sw_dram(addr, data as u32);
    }

    /// x[rd] = AMO32(M[x[rs1]] ^ x[rs2])
    fn amoxor_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.lw_dram(addr);
        self.set_reg(inst.rd, data as i64 as u64);
        data ^= self.get_reg(inst.rs2) as u32;
        self.bus.sw_dram(addr, data);
    }

    /// x[rd] = AMO32(M[x[rs1]] & x[rs2])
    fn amoand_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.lw_dram(addr);
        self.set_reg(inst.rd, data as i64 as u64);
        data &= self.get_reg(inst.rs2) as u32;
        self.bus.sw_dram(addr, data);
    }

    /// x[rd] = AMO32(M[x[rs1]] | x[rs2])
    fn amoor_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.lw_dram(addr);
        self.set_reg(inst.rd, data as i64 as u64);
        data |= self.get_reg(inst.rs2) as u32;
        self.bus.sw_dram(addr, data);
    }

    /// x[rd] = AMO32(M[x[rs1]] MIN x[rs2])
    fn amomin_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.lw_dram(addr) as i32;
        self.set_reg(inst.rd, data as i64 as u64);

        let rs2_v = self.get_reg(inst.rs2) as i32;
        if data < rs2_v {
            self.bus.sw_dram(addr, data as u32);
        } else {
            self.bus.sw_dram(addr, self.get_reg(inst.rs2) as u32);
        }
    }

    /// x[rd] = AMO32(M[x[rs1]] MAX x[rs2])
    fn amomax_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.lw_dram(addr) as i32;
        self.set_reg(inst.rd, data as i64 as u64);

        let rs2_v = self.get_reg(inst.rs2) as i32;
        if data < rs2_v {
            self.bus.sw_dram(addr, self.get_reg(inst.rs2) as u32);
        } else {
            self.bus.sw_dram(addr, data as u32);
        }
    }

    /// x[rd] = AMO32(M[x[rs1]] MINU x[rs2])
    fn amominu_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.lw_dram(addr);
        self.set_reg(inst.rd, data as i64 as u64);
        let rs2_v = self.get_reg(inst.rs2) as u32;
        if data < rs2_v {
            self.bus.sw_dram(addr, data as u32);
        } else {
            self.bus.sw_dram(addr, self.get_reg(inst.rs2) as u32);
        }
    }

    /// x[rd] = AMO32(M[x[rs1]] MAXU x[rs2])
    fn amomaxu_w(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.lw_dram(addr);
        self.set_reg(inst.rd, data as i64 as u64);
        let rs2_v = self.get_reg(inst.rs2) as u32;
        if data < rs2_v {
            self.bus.sw_dram(addr, self.get_reg(inst.rs2) as u32);
        } else {
            self.bus.sw_dram(addr, data);
        }
    }

    /// x[rd] = M[x[rs1] + sext(offset)][31:0]
    fn lwu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u32 as u64;
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.lw_dram(addr as u64) as u64;
        }
        self.set_reg(inst.rd, v);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][63:0]
    fn ld(&mut self, inst: &Instruction) {
        let imm = b12_to_sign64(inst.imm);
        let addr = self.get_reg(inst.rs1) as i64 + imm;
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64);
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            v = self.bus.ld_dram(addr as u64);
        }
        self.set_reg(inst.rd, v);
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][63:0]
    fn sd(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.get_reg(inst.rs2);
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v);
        } else {
            let addr = (addr - MEM_OFF as i64) as u64;
            self.bus.sd_dram(addr as u64, v);
        }
    }

    /// x[rd] = sext((x[rs1] + sext(immediate))[31:0])
    fn addiw(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.set_reg(inst.rd, v as i32 as u64);
    }

    /// x[rd] = sext((x[rs1] << shamt)[31:0])
    fn slliw(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("reserved encoding of slliw");
        }
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v as i32 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>u shamt)
    fn srliw(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("reserved encoding of srliw");
        }
        let v = (self.get_reg(inst.rs1) as u32) >> shamt;
        self.set_reg(inst.rd, v as i32 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>s shamt)
    fn sraiw(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("reserved encoding of sraiw");
        }
        let rs1 = self.get_reg(inst.rs1) as i32;
        let v = rs1 >> shamt;
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] + x[rs2])[31:0])
    fn addw(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) + self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v as i32 as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] - x[rs2])[31:0])
    fn subw(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) - self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v as i32 as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] << x[rs2][4:0])[31:0])
    fn sllw(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v as u32 as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>u x[rs2][4:0])
    fn srlw(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = (self.get_reg(inst.rs1) as u32) >> shamt;
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>s x[rs2][4:0])
    fn sraw(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = (self.get_reg(inst.rs1) as u32 as i32) >> shamt;
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] × x[rs2])[31:0])
    fn mulw(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) * self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v as u32 as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] /s x[rs2][31:0]
    fn divw(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as u32 as i32) / (self.get_reg(inst.rs2) as u32 as i32);
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] /u x[rs2][31:0])
    fn divuw(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as u32) / (self.get_reg(inst.rs2) as u32);
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] %s x[rs2][31:0])
    fn remw(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as u32 as i32) % (self.get_reg(inst.rs2) as u32 as i32);
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] %u x[rs2][31:0])
    fn remuw(&mut self, inst: &Instruction) {
        let v = (self.get_reg(inst.rs1) as u32) % (self.get_reg(inst.rs2) as u32);
        self.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = LoadReserved64(M[x[rs1]])
    fn lr_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::R);

        let data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        self.reserve_mem(addr, true);
    }

    /// x[rd] = StoreConditional64(M[x[rs1]], x[rs2])
    fn sc_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);
        self.invalidate_mem_reservation(addr, true);

        let data = self.get_reg(inst.rs2);

        if self.check_mem_reservation(addr, true) {
            panic!("memory is not reserved");
        }

        self.bus.sd_dram(addr, data);
        self.set_reg(inst.rd, 0);
    }

    /// x[rd] = AMO64(M[x[rs1]] SWAP x[rs2])
    fn amoswap_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        self.bus.sd_dram(addr, self.get_reg(inst.rs2));
        self.set_reg(inst.rs2, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] + x[rs2])
    fn amoadd_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        data += self.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] ^ x[rs2])
    fn amoxor_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        data ^= self.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] & x[rs2])
    fn amoand_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        data &= self.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] | x[rs2])
    fn amoor_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let mut data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        data |= self.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] MIN x[rs2])
    fn amomin_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        if (data as i64) < (self.get_reg(inst.rs2) as i64) {
            self.bus.sd_dram(addr, data);
        } else {
            self.bus.sd_dram(addr, self.get_reg(inst.rs2));
        }
    }

    /// x[rd] = AMO64(M[x[rs1]] MAX x[rs2])
    fn amomax_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        if (data as i64) < (self.get_reg(inst.rs2) as i64) {
            self.bus.sd_dram(addr, self.get_reg(inst.rs2));
        } else {
            self.bus.sd_dram(addr, data);
        }
    }

    /// x[rd] = AMO64(M[x[rs1]] MINU x[rs2])
    fn amominu_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        if data < self.get_reg(inst.rs2) {
            self.bus.sd_dram(addr, data);
        } else {
            self.bus.sd_dram(addr, self.get_reg(inst.rs2));
        }
    }

    /// x[rd] = AMO64(M[x[rs1]] MAXU x[rs2])
    fn amomaxu_d(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1);
        self.check_pmp(addr, PMPPerm::W);

        let data = self.bus.ld_dram(addr);
        self.set_reg(inst.rd, data);
        if data < self.get_reg(inst.rs2) {
            self.bus.sd_dram(addr, self.get_reg(inst.rs2));
        } else {
            self.bus.sd_dram(addr, data);
        }
    }
}

/// Sign-extended when imm is negative. (imm is 12bit)
fn b12_to_sign64(imm: u32) -> i64 {
    if 0x800 & imm == 0x800 {
        return (imm as u64 | 0xFFFF_FFFF_FFFF_F000) as i64;
    }
    imm as i64
}

/// Sign-extended when imm is negative. (imm is 20bit)
fn b20_to_sign64(imm: u32) -> i64 {
    if 0x800 & imm == 0x800 {
        return (imm as u64 | 0xFFFF_FFFF_FFFF_F000) as i64;
    }

    if 0x10_0000 & imm == 0x10_0000 {
        return (imm as u64 | 0xFFFF_FFFF_FFE0_0000) as i64;
    }
    imm as i64
}

/// Argument:
///   hex: 0x0123ABCD
fn hex_to_usize(hex: &str) -> usize {
    let mut chars = hex.chars();
    chars.next(); // remove 0
    chars.next(); // remove x

    let chars = chars.rev();
    let mut res: usize = 0;
    for (i, c) in chars.enumerate() {
        let v: usize;
        match c.to_ascii_lowercase() {
            '0' => v = 0x0,
            '1' => v = 0x1,
            '2' => v = 0x2,
            '3' => v = 0x3,
            '4' => v = 0x4,
            '5' => v = 0x5,
            '6' => v = 0x6,
            '7' => v = 0x7,
            '8' => v = 0x8,
            '9' => v = 0x9,
            'a' => v = 0xa,
            'b' => v = 0xb,
            'c' => v = 0xc,
            'd' => v = 0xd,
            'e' => v = 0xe,
            'f' => v = 0xf,
            _ => panic!("Not hex: {}", c),
        }
        res += v * (16_usize.pow(i as u32));
    }
    res
}
