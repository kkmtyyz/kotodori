#[derive(Debug)]
pub struct Register {
    // registers
    pub zero: u64,
    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub fp: u64, // s0
    pub s1: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,

    pub pc: u64,

    // supervisor-level csr
    pub sstatus: u64,    // 0x100
    pub sie: u64,        // 0x104
    pub stvec: u64,      // 0x105
    pub scounteren: u64, // 0x106
    pub senvcfg: u64,    // 0x10A
    pub sscratch: u64,   // 0x140
    pub sepc: u64,       // 0x141
    pub scause: u64,     // 0x142
    pub stval: u64,      // 0x143
    pub sip: u64,        // 0x144
    pub satp: u64,       // 0x180
    pub scontext: u64,   // 0x5A8

    // machine-level csr
    pub mvendorid: u64,  // 0xF11
    pub marchid: u64,    // 0xF12
    pub mimpid: u64,     // 0xF13
    pub mhartid: u64,    // 0xF14
    pub mconfigptr: u64, // 0xF15
    pub mstatus: u64,    // 0x300
    pub misa: u64,       // 0x301
    pub medeleg: u64,    // 0x302
    pub mideleg: u64,    // 0x303
    pub mie: u64,        // 0x304
    pub mtvec: u64,      // 0x305
    pub mcounteren: u64, // 0x306
    pub mstatush: u64,   // 0x310
    pub mscratch: u64,   // 0x340
    pub mepc: u64,       // 0x341
    pub mcause: u64,     // 0x342
    pub mtval: u64,      // 0x343
    pub mip: u64,        // 0x344
    pub mtinst: u64,     // 0x34A
    pub mtval2: u64,     // 0x34B
    pub menvcfg: u64,    // 0x30A
    pub menvcfgh: u64,   // 0x31A
    pub mseccfg: u64,    // 0x747
    pub mseccfgh: u64,   // 0x757
    pub pmpcfg0: u64,    // 0x3A0
    pub pmpcfg1: u64,    // 0x3A1
    pub pmpcfg2: u64,    // 0x3A2
    pub pmpcfg3: u64,    // 0x3A3
    pub pmpcfg4: u64,    // 0x3A4
    pub pmpcfg5: u64,    // 0x3A5
    pub pmpcfg6: u64,    // 0x3A6
    pub pmpcfg7: u64,    // 0x3A7
    pub pmpcfg8: u64,    // 0x3A8
    pub pmpcfg9: u64,    // 0x3A9
    pub pmpcfg10: u64,   // 0x3AA
    pub pmpcfg11: u64,   // 0x3AB
    pub pmpcfg12: u64,   // 0x3AC
    pub pmpcfg13: u64,   // 0x3AD
    pub pmpcfg14: u64,   // 0x3AE
    pub pmpcfg15: u64,   // 0x3AF
    pub pmpaddr0: u64,   // 0x3B0
    pub pmpaddr1: u64,   // 0x3B1
    pub pmpaddr2: u64,   // 0x3B2
    pub pmpaddr3: u64,   // 0x3B3
    pub pmpaddr4: u64,   // 0x3B4
    pub pmpaddr5: u64,   // 0x3B5
    pub pmpaddr6: u64,   // 0x3B6
    pub pmpaddr7: u64,   // 0x3B7
    pub pmpaddr8: u64,   // 0x3B8
    pub pmpaddr9: u64,   // 0x3B9
    pub pmpaddr10: u64,  // 0x3BA
    pub pmpaddr11: u64,  // 0x3BB
    pub pmpaddr12: u64,  // 0x3BC
    pub pmpaddr13: u64,  // 0x3BD
    pub pmpaddr14: u64,  // 0x3BE
    pub pmpaddr15: u64,  // 0x3BF
    pub pmpaddr16: u64,  // 0x3C0
    pub pmpaddr17: u64,  // 0x3C1
    pub pmpaddr18: u64,  // 0x3C2
    pub pmpaddr19: u64,  // 0x3C3
    pub pmpaddr20: u64,  // 0x3C4
    pub pmpaddr21: u64,  // 0x3C5
    pub pmpaddr22: u64,  // 0x3C6
    pub pmpaddr23: u64,  // 0x3C7
    pub pmpaddr24: u64,  // 0x3C8
    pub pmpaddr25: u64,  // 0x3C9
    pub pmpaddr26: u64,  // 0x3CA
    pub pmpaddr27: u64,  // 0x3CB
    pub pmpaddr28: u64,  // 0x3CC
    pub pmpaddr29: u64,  // 0x3CD
    pub pmpaddr30: u64,  // 0x3CE
    pub pmpaddr31: u64,  // 0x3CF
    pub pmpaddr32: u64,  // 0x3D0
    pub pmpaddr33: u64,  // 0x3D1
    pub pmpaddr34: u64,  // 0x3D2
    pub pmpaddr35: u64,  // 0x3D3
    pub pmpaddr36: u64,  // 0x3D4
    pub pmpaddr37: u64,  // 0x3D5
    pub pmpaddr38: u64,  // 0x3D6
    pub pmpaddr39: u64,  // 0x3D7
    pub pmpaddr40: u64,  // 0x3D8
    pub pmpaddr41: u64,  // 0x3D9
    pub pmpaddr42: u64,  // 0x3DA
    pub pmpaddr43: u64,  // 0x3DB
    pub pmpaddr44: u64,  // 0x3DC
    pub pmpaddr45: u64,  // 0x3DD
    pub pmpaddr46: u64,  // 0x3DE
    pub pmpaddr47: u64,  // 0x3DF
    pub pmpaddr48: u64,  // 0x3E0
    pub pmpaddr49: u64,  // 0x3E1
    pub pmpaddr50: u64,  // 0x3E2
    pub pmpaddr51: u64,  // 0x3E3
    pub pmpaddr52: u64,  // 0x3E4
    pub pmpaddr53: u64,  // 0x3E5
    pub pmpaddr54: u64,  // 0x3E6
    pub pmpaddr55: u64,  // 0x3E7
    pub pmpaddr56: u64,  // 0x3E8
    pub pmpaddr57: u64,  // 0x3E9
    pub pmpaddr58: u64,  // 0x3EA
    pub pmpaddr59: u64,  // 0x3EB
    pub pmpaddr60: u64,  // 0x3EC
    pub pmpaddr61: u64,  // 0x3ED
    pub pmpaddr62: u64,  // 0x3EE
    pub pmpaddr63: u64,  // 0x3EF
}

impl Register {
    pub fn new() -> Register {
        Register {
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
        let mut zero_ls: Vec<String> = vec![];
        non_zero_print("zero", "0x000", self.zero, &mut zero_ls);
        non_zero_print("ra", "0x001", self.ra, &mut zero_ls);
        non_zero_print("sp", "0x002", self.sp, &mut zero_ls);
        non_zero_print("gp", "0x003", self.gp, &mut zero_ls);
        non_zero_print("tp", "0x004", self.tp, &mut zero_ls);
        non_zero_print("t0", "0x005", self.t0, &mut zero_ls);
        non_zero_print("t1", "0x006", self.t1, &mut zero_ls);
        non_zero_print("t2", "0x007", self.t2, &mut zero_ls);
        non_zero_print("fp", "0x008", self.fp, &mut zero_ls);
        non_zero_print("s1", "0x009", self.s1, &mut zero_ls);
        non_zero_print("a0", "0x00A", self.a0, &mut zero_ls);
        non_zero_print("a1", "0x00B", self.a1, &mut zero_ls);
        non_zero_print("a2", "0x00C", self.a2, &mut zero_ls);
        non_zero_print("a3", "0x00D", self.a3, &mut zero_ls);
        non_zero_print("a4", "0x00E", self.a4, &mut zero_ls);
        non_zero_print("a5", "0x00F", self.a5, &mut zero_ls);
        non_zero_print("a6", "0x010", self.a6, &mut zero_ls);
        non_zero_print("a7", "0x011", self.a7, &mut zero_ls);
        non_zero_print("s2", "0x012", self.s2, &mut zero_ls);
        non_zero_print("s3", "0x013", self.s3, &mut zero_ls);
        non_zero_print("s4", "0x014", self.s4, &mut zero_ls);
        non_zero_print("s5", "0x015", self.s5, &mut zero_ls);
        non_zero_print("s6", "0x016", self.s6, &mut zero_ls);
        non_zero_print("s7", "0x017", self.s7, &mut zero_ls);
        non_zero_print("s8", "0x018", self.s8, &mut zero_ls);
        non_zero_print("s9", "0x019", self.s9, &mut zero_ls);
        non_zero_print("s10", "0x01A", self.s10, &mut zero_ls);
        non_zero_print("s11", "0x01B", self.s11, &mut zero_ls);
        non_zero_print("t3", "0x01C", self.t3, &mut zero_ls);
        non_zero_print("t4", "0x01D", self.t4, &mut zero_ls);
        non_zero_print("t5", "0x01E", self.t5, &mut zero_ls);
        non_zero_print("t6", "0x01F", self.t6, &mut zero_ls);
        non_zero_print("pc", "0x020", self.pc, &mut zero_ls);

        non_zero_print("mstatus", "0x300", self.mstatus, &mut zero_ls);
        non_zero_print("medeleg", "0x302", self.medeleg, &mut zero_ls);
        non_zero_print("mideleg", "0x303", self.mideleg, &mut zero_ls);
        non_zero_print("mie", "0x304", self.mie, &mut zero_ls);
        non_zero_print("mscratch", "0x340", self.mscratch, &mut zero_ls);
        non_zero_print("mepc", "0x341", self.mepc, &mut zero_ls);
        non_zero_print("mip", "0x344", self.mip, &mut zero_ls);
        non_zero_print("mtvec", "0x3B0", self.mtvec, &mut zero_ls);

        non_zero_print("sstatus", "0x100", self.sstatus, &mut zero_ls);
        non_zero_print("sie", "0x104", self.sie, &mut zero_ls);
        non_zero_print("stvec", "0x105", self.stvec, &mut zero_ls);
        non_zero_print("sscratch", "0x140", self.sscratch, &mut zero_ls);
        non_zero_print("sepc", "0x141", self.sepc, &mut zero_ls);
        non_zero_print("scause", "0x142", self.scause, &mut zero_ls);
        non_zero_print("sip", "0x144", self.sip, &mut zero_ls);
        non_zero_print("satp", "0x180", self.satp, &mut zero_ls);

        non_zero_print("pmpaddr0", "0x3B0", self.pmpaddr0, &mut zero_ls);
        non_zero_print("pmpcfg0", "0x3A0", self.pmpcfg0, &mut zero_ls);

        println!("zero: {:?}", zero_ls);
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
}

/// example:
/// println!("zero(0x000):\t0x{:016X}, 0b{:064b}", <zero register value>, <zero register value>);
fn non_zero_print(reg_name: &str, reg_code: &str, value: u64, zero_ls: &mut Vec<String>) {
    if value == 0 {
        zero_ls.push(reg_name.to_string());
        return;
    }
    let mut reg_name = format!("{}({}):", reg_name, reg_code);
    if reg_name.len() < 16 {
        reg_name += "\t";
    }
    println!("{}0x{:016X}, 0b{:064b}", reg_name, value, value);
}
