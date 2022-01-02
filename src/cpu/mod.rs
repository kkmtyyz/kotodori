pub mod instructions;

use crate::bus::Bus;
use crate::conf;
use instructions::InstName;
use instructions::Instruction;

#[derive(Debug)]
pub struct Cpu {
    bus: Bus,

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
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            bus,
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
        println!( "zero:0x{:016X}, 0b{:064b}", self.zero, self.zero);
        println!( "ra:0x{:016X}, 0b{:064b}", self.ra, self.ra);
        println!( "sp:0x{:016X}, 0b{:064b}", self.sp, self.sp);
        println!( "gp:0x{:016X}, 0b{:064b}", self.gp, self.gp);
        println!( "tp:0x{:016X}, 0b{:064b}", self.tp, self.tp);
        println!( "t0:0x{:016X}, 0b{:064b}", self.t0, self.t0);
        println!( "t1:0x{:016X}, 0b{:064b}", self.t1, self.t1);
        println!( "t2:0x{:016X}, 0b{:064b}", self.t2, self.t2);
        println!( "fp:0x{:016X}, 0b{:064b}", self.fp, self.fp);
        println!( "s1:0x{:016X}, 0b{:064b}", self.s1, self.s1);
        println!( "a0:0x{:016X}, 0b{:064b}", self.a0, self.a0);
        println!( "a1:0x{:016X}, 0b{:064b}", self.a1, self.a1);
        println!( "a6:0x{:016X}, 0b{:064b}", self.a6, self.a6);
        println!( "a7:0x{:016X}, 0b{:064b}", self.a7, self.a7);
        println!( "a2:0x{:016X}, 0b{:064b}", self.a2, self.a2);
        println!( "a5:0x{:016X}, 0b{:064b}", self.a5, self.a5);
        println!( "a6:0x{:016X}, 0b{:064b}", self.a6, self.a6);
        println!( "a7:0x{:016X}, 0b{:064b}", self.a7, self.a7);
        println!( "s2:0x{:016X}, 0b{:064b}", self.s5, self.s5);
        println!( "s3:0x{:016X}, 0b{:064b}", self.s3, self.s3);
        println!( "s4:0x{:016X}, 0b{:064b}", self.s4, self.s4);
        println!( "s5:0x{:016X}, 0b{:064b}", self.s5, self.s5);
        println!( "s6:0x{:016X}, 0b{:064b}", self.s6, self.s6);
        println!( "s7:0x{:016X}, 0b{:064b}", self.s7, self.s7);
        println!( "s8:0x{:016X}, 0b{:064b}", self.s8, self.s8);
        println!( "s9:0x{:016X}, 0b{:064b}", self.s9, self.s9);
        println!( "s10:0x{:016X}, 0b{:064b}", self.s10, self.s10);
        println!( "s11:0x{:016X}, 0b{:064b}", self.s11, self.s11);
        println!( "t3:0x{:016X}, 0b{:064b}", self.t3, self.t3);
        println!( "t4:0x{:016X}, 0b{:064b}", self.t4, self.t4);
        println!( "t5:0x{:016X}, 0b{:064b}", self.t5, self.t5);
        println!( "t6:0x{:016X}, 0b{:064b}", self.t6, self.t6);
        println!( "pc:0x{:016X}, 0b{:064b}", self.pc, self.pc);
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

    pub fn set_reg(&mut self, reg: u8, value: u64) {
        match reg {
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

    pub fn init(&mut self) {
        self.sp = conf::STACK_BOTTOM;
        self.pc = conf::TEXT_START;
    }

    pub fn run(&mut self) {
        for _ in 0..2 {
            // in dev
            //loop { // in prod
            let data = self.fetch();
            let inst = Instruction::decode(data);
            println!("instruction: ");
            inst.print();

            self.exec_instruction(&inst);
            self.pc += 4;
        }
    }

    fn fetch(&self) -> u32 {
        let mut ltl_data = self.bus.lw_dram(self.pc); // little endian data
        let mut data: u32 = 0;
        for _ in 0..4 {
            data <<= 8;
            data |= ltl_data & 0xFF;
            ltl_data >>= 8;
        }
        data
    }

    fn exec_instruction(&mut self, inst: &Instruction) {
        match inst.name {
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
            InstName::Csrrw(_) => self.csrrw(inst),
            InstName::Csrrs(_) => self.csrrs(inst),
            InstName::Csrrc(_) => self.csrrc(inst),
            InstName::Csrrwi(_) => self.csrrwi(inst),
            InstName::Csrrsi(_) => self.csrrsi(inst),
            InstName::Csrrci(_) => self.csrrci(inst),
            _ => (),
        }
    }

    fn lui(&mut self, inst: &Instruction) {
        let v = (inst.imm as i32) << 12;
        self.set_reg(inst.rd, v as u64);
    }

    fn auipc(&mut self, inst: &Instruction) {
        let v = ((inst.imm as i32) << 12) + self.pc as i32;
        self.set_reg(inst.rd, v as u64);
    }

    fn jal(&mut self, inst: &Instruction) {
        if inst.rd == 0 {
            self.ra = self.pc + 4; // ra == x1
        } else {
            self.set_reg(inst.rd, self.pc + 4);
        }
        let v = self.pc as i32 + inst.imm as i32;
        self.pc = v as u64;
    }

    fn jalr(&mut self, inst: &Instruction) {
        let t = self.pc + 4;
        let v = (self.get_reg(inst.rs1) as i32 + inst.imm as i32) as u32;
        self.pc = v as u64 & 0xFFFF_FFFF_FFFF_FFFE;

        if inst.rd == 0 {
            self.ra = t;
        } else {
            self.set_reg(inst.rd, t);
        }
    }

    fn beq(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) == self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    fn bne(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) != self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    fn blt(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) < (self.get_reg(inst.rs2) as i64) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    fn bge(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) >= (self.get_reg(inst.rs2) as i64) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    fn bltu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    fn bgeu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) >= self.get_reg(inst.rs2) {
            self.pc = (self.pc as i64 + inst.imm as i64) as u64;
        }
    }

    fn lb(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let v = self.bus.lb_dram(addr as u64) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    fn lh(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let v = self.bus.lh_dram(addr as u64) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    fn lw(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let v = self.bus.lw_dram(addr as u64) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    fn lbu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let v = self.bus.lb_dram(addr as u64);
        self.set_reg(inst.rd, v as u64);
    }

    fn lhu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let v = self.bus.lh_dram(addr as u64);
        self.set_reg(inst.rd, v as u64);
    }

    fn sb(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs2) as u8;
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.bus.sb_dram(addr as u64, v);
    }

    fn sh(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs2) as u16;
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.bus.sh_dram(addr as u64, v);
    }

    fn sw(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs2) as u32;
        let addr = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.bus.sw_dram(addr as u64, v);
    }

    fn addi(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) as i64 + inst.imm as i64;
        self.set_reg(inst.rd, v as u64);
    }

    fn slti(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) < (inst.imm as i64) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    fn sltiu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < inst.imm as u64 {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    fn xori(&mut self, inst: &Instruction) {
        let v = (inst.imm as i64) ^ (self.get_reg(inst.rs1) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    fn ori(&mut self, inst: &Instruction) {
        let v = (inst.imm as i64) | (self.get_reg(inst.rs1) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    fn andi(&mut self, inst: &Instruction) {
        let v = (inst.imm as i64) & (self.get_reg(inst.rs1) as i64);
        self.set_reg(inst.rd, v as u64);
    }

    fn slli(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("slli instruction overflow");
        }
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v);
    }

    fn srli(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("srli instruction overflow");
        }
        let v = self.get_reg(inst.rs1) >> shamt;
        self.set_reg(inst.rd, v);
    }

    fn srai(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("srli instruction overflow");
        }
        let rs1 = self.get_reg(inst.rs1);
        let sign = rs1 & 0x8000_0000;
        let mut v = self.get_reg(inst.rs1) >> shamt;
        v |= sign;
        self.set_reg(inst.rd, v);
    }

    fn add(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) as i64 + self.get_reg(inst.rs2) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    fn sub(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) as i64 - self.get_reg(inst.rs2) as i64;
        self.set_reg(inst.rd, v as u64);
    }

    fn sll(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v);
    }

    fn slt(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i64) < (self.get_reg(inst.rs2) as i64) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    fn sltu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < self.get_reg(inst.rs2) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    fn xor(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) ^ self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    fn srl(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = self.get_reg(inst.rs1) >> shamt;
        self.set_reg(inst.rd, v);
    }

    fn sra(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let rs1 = self.get_reg(inst.rs1);
        let sign = rs1 & 0x8000_0000;
        let mut v = self.get_reg(inst.rs1) >> shamt;
        v |= sign;
        self.set_reg(inst.rd, v);
    }

    fn or(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) | self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    fn and(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) & self.get_reg(inst.rs2);
        self.set_reg(inst.rd, v);
    }

    fn fence(&mut self, inst: &Instruction) {
        let pred = inst.imm >> 4 & 0b1111;
        let succ = inst.imm & 0b1111;
        // Implement when needed.
    }

    fn fence_i(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    fn ecall(&mut self, inst: &Instruction) {
        // Implement when needed.
    }

    fn csrrw(&mut self, inst: &Instruction) {
    }

    fn csrrs(&mut self, inst: &Instruction) {
    }

    fn csrrc(&mut self, inst: &Instruction) {
    }

    fn csrrwi(&mut self, inst: &Instruction) {
    }

    fn csrrsi(&mut self, inst: &Instruction) {
    }

    fn csrrci(&mut self, inst: &Instruction) {
    }
}
