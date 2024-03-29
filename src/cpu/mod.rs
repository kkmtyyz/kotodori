pub mod instructions;
mod int;
pub mod register;
use crate::bus::Bus;
use crate::conf;
use crate::conf::MEM_OFF;
use crate::dbg::Debug;
use crate::util;
use instructions::InstName;
use instructions::Instruction;
use register::Register;
use std::io::{stdout, Write};

const MTIME: u64 = 0x200_BFF8;
const MTIMECMP: u64 = 0x200_4000;

// paging
const BARE: u64 = 0x00;
const SV39: u64 = 0x08;
const SV48: u64 = 0x09;
const SV57: u64 = 0x10;
const SV64: u64 = 0x11;
const SATP_PPN: u64 = 0xFFF_FFFF_FFFF;
const PAGE_OFF: u64 = 0xFFF;
const VPN_SIZE: u64 = 9; // bit

// Sv39
const SV39_VPN: u64 = 0x7F_C000_0000;
const SV39_PTE_PPN: u64 = 0x3F_FFFF_FFFF_FC00;

#[derive(Debug, PartialEq)]
pub enum Mode {
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
    dbg: Debug,
    dbg_step: bool,

    bus: Bus,
    mem_reserved_w: Vec<u8>,
    mode: Mode, // privilege mode

    // memory mapped
    mtime: u64,
    mtimecmp: u64,

    reg: Register,
}

impl Cpu {
    pub fn new(bus: Bus, mem_size: usize, dbg: Debug) -> Cpu {
        let dbg_step = if dbg.bp == 0 { true } else { false };

        Cpu {
            dbg,
            dbg_step: dbg_step,

            bus,
            mem_reserved_w: vec![0; mem_size / 32],
            mode: Mode::M,

            mtime: 0,
            mtimecmp: 0,

            reg: Register::new(),
        }
    }

    pub fn print(&self) {
        println!("mode:\t {:?}", self.mode);
        self.reg.print();
        println!("mtime:\t\t0x{:016X}, 0b{:064b}", self.mtime, self.mtime);
        println!(
            "mtimecmp:\t0x{:016X}, 0b{:064b}",
            self.mtimecmp, self.mtimecmp
        );
    }

    pub fn pdram_range(&self, begin: usize, end: usize) {
        self.bus.pdram_range(begin, end);
    }

    fn l_mm(&self, addr: u64) -> u64 {
        match addr {
            MTIME => self.mtime,
            MTIMECMP => self.mtimecmp,
            _ => self.bus.l_mm(addr),
        }
    }

    fn s_mm(&mut self, addr: u64, data: u64) {
        match addr {
            MTIME => self.mtime = data,
            MTIMECMP => self.mtimecmp = data,
            _ => self.bus.s_mm(addr, data),
        }
    }

    fn trans_addr(&self, addr: u64) -> u64 {
        match self.reg.satp >> 60 {
            BARE => addr,
            SV39 => self.sv39(addr),
            SV48 => self.sv48(addr),
            SV57 => self.sv57(addr),
            SV64 => self.sv64(addr),
            _ => panic!("invalid satp mode"),
        }
    }

    /// Sv39 virtual address
    ///   30..38 -- 9 bits of level-2 index.
    ///   21..29 -- 9 bits of level-1 index.
    ///   12..20 -- 9 bits of level-0 index.
    ///    0..11 -- 12 bits of byte offset within the page.
    fn sv39(&self, va: u64) -> u64 {
        let ppn = self.reg.satp & SATP_PPN;
        let ppn = ppn << 12; // * 4096 (page size)
        let vpn = (va & SV39_VPN) >> 30;
        let vpn = vpn << 3; // * 8 (pte size)
        let mut pte = self.bus.ld_dram(ppn + vpn - MEM_OFF as u64);

        let mut vpn_shift = 21;
        for i in 1..3 {
            if pte & 0x1 == 0 {
                panic!("page fault");
            }
            let ppn = pte & SV39_PTE_PPN;
            let ppn = ppn << 2; // * 4096 (page size)
            let vpn = (va & SV39_VPN >> (VPN_SIZE * i)) >> vpn_shift;
            let vpn = vpn << 3; // * 8 (pte size)
            pte = self.bus.ld_dram(ppn + vpn - MEM_OFF as u64);
            vpn_shift -= VPN_SIZE;
        }

        let ppn = pte & SV39_PTE_PPN;
        let ppn = ppn << 2; // * 4096 (page size)
        let pa = ppn + (va & PAGE_OFF);
        pa
    }

    fn sv48(&self, _va: u64) -> u64 {
        0
    }

    fn sv57(&self, _va: u64) -> u64 {
        0
    }

    fn sv64(&self, _va: u64) -> u64 {
        0
    }

    pub fn init(&mut self, entry_point: usize) {
        self.reg.sp = conf::STACK_BOTTOM;
        self.reg.pc = entry_point as u64;
    }

    pub fn run(&mut self) {
        loop {
            let data = self.fetch();
            let inst = Instruction::decode(data);

            if self.dbg.enable && self.dbg_step {
                println!("instruction: ");
                inst.print();
                println!("pc: 0x{:016X}", self.reg.pc);
            }

            let pre_pc = self.reg.pc;
            self.exec_instruction(&inst);

            self.mtime += 2500;

            int::timer_int(&mut self.reg, &mut self.mode, self.mtime, self.mtimecmp);
            int::int(&mut self.reg, &mut self.mode);

            if pre_pc == self.reg.pc {
                self.reg.pc += 4;
            }

            if self.dbg.enable {
                if self.dbg_step {
                    self.debug();
                } else if self.dbg.is_bp(self.reg.pc) {
                    self.debug();
                    self.dbg_step = true;
                }
            }
        }
    }

    fn debug(&mut self) {
        loop {
            print!(">> ");
            stdout().flush().unwrap();
            let mut b = String::new();
            std::io::stdin().read_line(&mut b).ok();
            if b.trim() == "".to_string() {
                // next instruction
                break;
            } else if b.trim() == "p".to_string() {
                // print registers
                self.print();
            } else if b.starts_with("m") {
                // print memory range
                // example: m 0x80001000 0x80001FFF
                let mut b = b.split_whitespace();
                b.next(); // remove m
                let begin = util::hex_to_usize(b.next().unwrap());
                let end = util::hex_to_usize(b.next().unwrap());
                self.bus.pdram_range(begin, end);
            } else if b.trim() == "uart".to_string() {
                // print UART registers
                self.bus.puart();
            } else if b.starts_with("b") {
                // set break point
                // example: b 0x8000157c
                let mut b = b.split_whitespace();
                b.next(); // remove b
                let bp = util::hex_to_usize(b.next().unwrap());
                self.dbg.bp = bp as u64;
                self.dbg_step = false;
                break;
            }
        }
        println!();
    }

    fn fetch(&self) -> u32 {
        let addr = self.trans_addr(self.reg.pc);
        self.check_pmp(addr, PMPPerm::X);
        self.bus.lw_dram(addr - MEM_OFF as u64)
    }

    fn check_pmp(&self, addr: u64, perm: PMPPerm) {
        if let Mode::S | Mode::U = self.mode {
            // do PMP
            self.blocked_by_pmp(addr, perm);
            return;
        }

        let mprv = (self.reg.mstatus & 0b10_0000_0000_0000_0000) >> 17;
        if let Mode::S | Mode::U = self.mode {
            if mprv == 0 && perm != PMPPerm::X {
                // do PMP
                self.blocked_by_pmp(addr, perm);
                return;
            }
        }

        let mpp = (self.reg.mstatus & 0b1_1000_0000_0000) >> 11;
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
            let pmpcfg = self.reg.get_csr(pmpcfg_base + (i / 8 * 2));
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
                    begin = self.reg.get_csr(pmpaddr_base + i - 1);
                }
                let end = self.reg.get_csr(pmpaddr_base + i);

                if addr < begin && end <= addr {
                    continue;
                }

                match perm {
                    PMPPerm::R => {
                        if pmpcfg & 1 == 0b001 {
                            return;
                        }
                    }
                    PMPPerm::W => {
                        if pmpcfg & 2 == 0b010 {
                            return;
                        }
                    }
                    PMPPerm::X => {
                        if pmpcfg & 4 == 0b100 {
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
        let v = (inst.imm << 12) as i32 as i64;
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = pc + sext(immediate[31:12] << 12)
    fn auipc(&mut self, inst: &Instruction) {
        let v = self.reg.pc as i64 + (inst.imm << 12) as i32 as i64;
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = pc+4; pc += sext(offset)
    fn jal(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x8_0000);
        self.reg.set_reg(inst.rd, self.reg.pc + 4);
        let v = self.reg.pc as i64 + imm;
        self.reg.pc = v as u64;
    }

    /// t =pc+4; pc=(x[rs1]+sext(offset))&∼1; x[rd]=t
    fn jalr(&mut self, inst: &Instruction) {
        let t = self.reg.pc + 4;

        let imm = sext(inst.imm as u64, 0x800);
        let v = (self.reg.get_reg(inst.rs1) as i64 + imm) as u64;
        self.reg.pc = v & !1;

        self.reg.set_reg(inst.rd, t);
    }

    /// if (rs1 == rs2) pc += sext(offset)
    fn beq(&mut self, inst: &Instruction) {
        if self.reg.get_reg(inst.rs1) == self.reg.get_reg(inst.rs2) {
            let imm = sext(inst.imm as u64, 0x800);
            self.reg.pc = (self.reg.pc as i64 + imm) as u64;
        }
    }

    /// if (rs1 != rs2) pc += sext(offset)
    fn bne(&mut self, inst: &Instruction) {
        if self.reg.get_reg(inst.rs1) != self.reg.get_reg(inst.rs2) {
            let imm = sext(inst.imm as u64, 0x800);
            self.reg.pc = (self.reg.pc as i64 + imm) as u64;
        }
    }

    /// if (rs1 <s rs2) pc += sext(offset)
    fn blt(&mut self, inst: &Instruction) {
        if (self.reg.get_reg(inst.rs1) as i64) < (self.reg.get_reg(inst.rs2) as i64) {
            let imm = sext(inst.imm as u64, 0x800);
            self.reg.pc = (self.reg.pc as i64 + imm) as u64;
        }
    }

    /// if (rs1 >=s rs2) pc += sext(offset)
    fn bge(&mut self, inst: &Instruction) {
        if (self.reg.get_reg(inst.rs1) as i64) >= (self.reg.get_reg(inst.rs2) as i64) {
            let imm = sext(inst.imm as u64, 0x800);
            self.reg.pc = (self.reg.pc as i64 + imm) as u64;
        }
    }

    /// if (rs1 >u rs2) pc += sext(offset)
    fn bltu(&mut self, inst: &Instruction) {
        if self.reg.get_reg(inst.rs1) < self.reg.get_reg(inst.rs2) {
            let imm = sext(inst.imm as u64, 0x800);
            self.reg.pc = (self.reg.pc as i64 + imm) as u64;
        }
    }

    /// if (rs1 >=u rs2) pc += sext(offset)
    fn bgeu(&mut self, inst: &Instruction) {
        if self.reg.get_reg(inst.rs1) >= self.reg.get_reg(inst.rs2) {
            let imm = sext(inst.imm as u64, 0x800);
            self.reg.pc = (self.reg.pc as i64 + imm) as u64;
        }
    }

    /// x[rd] = sext(M[x[rs1] + sext(offset)][7:0])
    fn lb(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: i64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u8 as i64;
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.lb_dram(addr) as i64;
        }
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = sext(M[x[rs1] + sext(offset)][15:0])
    fn lh(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: i64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u16 as i64;
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.lh_dram(addr) as i64;
        }
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = sext(M[x[rs1] + sext(offset)][31:0])
    fn lw(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: i64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u32 as i64;
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.lw_dram(addr as u64) as i64;
        }
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][7:0]
    fn lbu(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u8 as u64;
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.lb_dram(addr as u64) as u64;
        }
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][15:0]
    fn lhu(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u16 as u64;
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.lh_dram(addr as u64) as u64;
        }
        self.reg.set_reg(inst.rd, v);
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][7:0]
    fn sb(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.reg.get_reg(inst.rs2) as u8;
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v as u64);
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            self.bus.sb_dram(addr as u64, v);
        }
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][15:0]
    fn sh(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.reg.get_reg(inst.rs2) as u16;
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v as u64);
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            self.bus.sh_dram(addr as u64, v);
        }
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][31:0]
    fn sw(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1) as i64 + inst.imm as i64;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.reg.get_reg(inst.rs2) as u32;
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v as u64);
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            self.bus.sw_dram(addr as u64, v);
        }
    }

    /// x[rd] = x[rs1] + sext(immediate)
    fn addi(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let v = self.reg.get_reg(inst.rs1) as i64 + imm;
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] <s sext(immediate)
    fn slti(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        if (self.reg.get_reg(inst.rs1) as i64) < imm {
            self.reg.set_reg(inst.rd, 1);
        } else {
            self.reg.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] <u sext(immediate)
    fn sltiu(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        if self.reg.get_reg(inst.rs1) < imm as u64 {
            self.reg.set_reg(inst.rd, 1);
        } else {
            self.reg.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] ^ sext(immediate)
    fn xori(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let v = imm ^ (self.reg.get_reg(inst.rs1) as i64);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] | sext(immediate)
    fn ori(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let v = imm | (self.reg.get_reg(inst.rs1) as i64);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] & sext(immediate)
    fn andi(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let v = imm & (self.reg.get_reg(inst.rs1) as i64);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] << shamt
    fn slli(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        let v = self.reg.get_reg(inst.rs1) << shamt;
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>u shamt
    fn srli(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        let v = self.reg.get_reg(inst.rs1) >> shamt;
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>s shamt
    fn srai(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        let v = (self.reg.get_reg(inst.rs1) as i64) >> shamt;
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] + x[rs2]
    fn add(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) + self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] - x[rs2]
    fn sub(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) - self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] << x[rs2]
    fn sll(&mut self, inst: &Instruction) {
        let shamt = self.reg.get_reg(inst.rs2) & 0b1_1111;
        let v = self.reg.get_reg(inst.rs1) << shamt;
        self.reg.set_reg(inst.rd, v);
    }

    // x[rd] = x[rs1] <s x[rs2]
    fn slt(&mut self, inst: &Instruction) {
        if (self.reg.get_reg(inst.rs1) as i64) < (self.reg.get_reg(inst.rs2) as i64) {
            self.reg.set_reg(inst.rd, 1);
        } else {
            self.reg.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] <u x[rs2]
    fn sltu(&mut self, inst: &Instruction) {
        if self.reg.get_reg(inst.rs1) < self.reg.get_reg(inst.rs2) {
            self.reg.set_reg(inst.rd, 1);
        } else {
            self.reg.set_reg(inst.rd, 0);
        }
    }

    /// x[rd] = x[rs1] ^ x[rs2]
    fn xor(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) ^ self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>u x[rs2]
    fn srl(&mut self, inst: &Instruction) {
        let shamt = self.reg.get_reg(inst.rs2) & 0b1_1111;
        let v = self.reg.get_reg(inst.rs1) >> shamt;
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] >>s x[rs2]
    fn sra(&mut self, inst: &Instruction) {
        let shamt = self.reg.get_reg(inst.rs2) & 0b1_1111;
        let v = (self.reg.get_reg(inst.rs1) as i64) >> shamt;
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] | x[rs2]
    fn or(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) | self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] & x[rs2]
    fn and(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) & self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v);
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
        let t = self.reg.get_csr(csr);
        self.reg.set_csr(csr, self.reg.get_reg(inst.rs1));
        self.reg.set_reg(inst.rd, t);
    }

    /// t = CSRs[csr]; CSRs[csr] = t | x[rs1]; x[rd] = t
    fn csrrs(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.reg.get_csr(csr);
        self.reg.set_csr(csr, t | self.reg.get_reg(inst.rs1));
        self.reg.set_reg(inst.rd, t);
    }

    /// t = CSRs[csr]; CSRs[csr] = t &∼x[rs1]; x[rd] = t
    fn csrrc(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.reg.get_csr(csr);
        self.reg.set_csr(csr, t & !self.reg.get_reg(inst.rs1));
        self.reg.set_reg(inst.rd, t);
    }

    /// x[rd] = CSRs[csr]; CSRs[csr] = zimm
    fn csrrwi(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        self.reg.set_reg(inst.rd, self.reg.get_csr(csr));
        let zimm = inst.rs1;
        self.reg.set_csr(csr, zimm as u64);
    }

    /// t = CSRs[csr]; CSRs[csr] = t | zimm; x[rd] = t
    fn csrrsi(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.reg.get_csr(csr);
        let zimm = inst.rs1;
        self.reg.set_csr(csr, t | zimm as u64);
        self.reg.set_reg(inst.rd, t);
    }

    /// t = CSRs[csr]; CSRs[csr] = t &∼zimm; x[rd] = t
    fn csrrci(&mut self, inst: &Instruction) {
        let csr = inst.imm as u16;
        let t = self.reg.get_csr(csr);
        let zimm = inst.rs1;
        self.reg.set_csr(csr, t & !(zimm as u64));
        self.reg.set_reg(inst.rd, t);
    }

    /// ExceptionReturn(User)
    fn sret(&mut self, _inst: &Instruction) {
        let pre_spp = (self.reg.sstatus & 0b1_1000_0000_0000) >> 11;
        let spie = self.reg.sstatus & 0b100_0000;
        let sie = spie >> 3;
        self.reg.sstatus |= sie;
        let spie: u64 = 0b100_0000;
        self.reg.sstatus |= spie;
        let spp: u64 = 0b1_1000_0000_0000;
        self.reg.sstatus &= !spp; // mpp = 0; U-MODE
        let sprv: u64 = 0b10_0000_0000_0000_0000;
        self.reg.sstatus &= !sprv; // mprv = 0;
        self.reg.pc = self.reg.sepc;

        match pre_spp {
            0 => self.mode = Mode::U,
            1 => self.mode = Mode::S,
            3 => self.mode = Mode::M,
            _ => (),
        }
    }

    /// ExceptionReturn(Machine)
    fn mret(&mut self, _inst: &Instruction) {
        let pre_mpp = (self.reg.mstatus & 0b1_1000_0000_0000) >> 11;
        let mpie = self.reg.mstatus & 0b100_0000;
        let mie = mpie >> 3;
        self.reg.mstatus |= mie;
        let mpie: u64 = 0b100_0000;
        self.reg.mstatus |= mpie;
        let mpp: u64 = 0b1_1000_0000_0000;
        self.reg.mstatus &= !mpp; // mpp = 0; U-MODE
        let mprv: u64 = 0b10_0000_0000_0000_0000;
        self.reg.mstatus &= !mprv; // mprv = 0;
        self.reg.pc = self.reg.mepc;

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
        let v = self.reg.get_reg(inst.rs1) * self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = (x[rs1] s×s x[rs2]) >>s XLEN
    fn mulh(&mut self, inst: &Instruction) {
        let v = (self.reg.get_reg(inst.rs1) as i64) * (self.reg.get_reg(inst.rs2) as i64);
        self.reg.set_reg(inst.rd, (v >> 32) as u64);
    }

    /// x[rd] = (x[rs1] s × x[rs2]) >>s XLEN
    fn mulhsu(&mut self, inst: &Instruction) {
        let v = ((self.reg.get_reg(inst.rs1) as i64) as u64) * self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, (v >> 32) as u64);
    }

    /// x[rd] = (x[rs1] u × x[rs2]) >>u XLEN
    fn mulhu(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) * self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, (v >> 32) as u64);
    }

    /// x[rd] = x[rs1] /s x[rs2]
    fn div(&mut self, inst: &Instruction) {
        let v = (self.reg.get_reg(inst.rs1) as i64) / (self.reg.get_reg(inst.rs2) as i64);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] /u x[rs2]
    fn divu(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) / self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = x[rs1] %s x[rs2]
    fn rem(&mut self, inst: &Instruction) {
        let v = (self.reg.get_reg(inst.rs1) as i64) % (self.reg.get_reg(inst.rs2) as i64);
        self.reg.set_reg(inst.rd, v as u64);
    }

    /// x[rd] = x[rs1] %u x[rs2]
    fn remu(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) % self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = LoadReserved32(M[x[rs1]])
    fn lr_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::R);

        let data = self.bus.lw_dram(addr) as i64;
        self.reg.set_reg(inst.rd, data as u64);
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
        let addr = self.reg.get_reg(inst.rs1);
        let addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        self.invalidate_mem_reservation(addr, false);
        let data = self.reg.get_reg(inst.rs2) as u32;

        if self.check_mem_reservation(addr, false) {
            panic!("memory is not reserved");
        }

        self.bus.sw_dram(addr, data);
        self.reg.set_reg(inst.rd, 0);
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
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        if mm {
            self.bus
                .s_mm(addr, self.reg.get_reg(inst.rs2) as u32 as u64);
        } else {
            self.bus.sw_dram(addr, self.reg.get_reg(inst.rs2) as u32);
        }
        self.reg.set_reg(inst.rs2, data as i32 as i64 as u64);
        self.reg.set_reg(inst.rd, data as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] + x[rs2])
    fn amoadd_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let data = data as i32 + self.reg.get_reg(inst.rs2) as i32;
        self.bus.sw_dram(addr, data as u32);
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] ^ x[rs2])
    fn amoxor_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let data = data ^ self.reg.get_reg(inst.rs2) as u32;
        self.bus.sw_dram(addr, data);
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] & x[rs2])
    fn amoand_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let data = data & self.reg.get_reg(inst.rs2) as u32;
        self.bus.sw_dram(addr, data);
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] | x[rs2])
    fn amoor_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let data = data | self.reg.get_reg(inst.rs2) as u32;
        self.bus.sw_dram(addr, data);
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] MIN x[rs2])
    fn amomin_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64 as i32;
        if (data as i32) < rs2_v {
            self.bus.sw_dram(addr, data);
        } else {
            self.bus.sw_dram(addr, self.reg.get_reg(inst.rs2) as u32);
        }
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] MAX x[rs2])
    fn amomax_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64 as i32;
        if (data as i32) < rs2_v {
            self.bus.sw_dram(addr, self.reg.get_reg(inst.rs2) as u32);
        } else {
            self.bus.sw_dram(addr, data);
        }
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] MINU x[rs2])
    fn amominu_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64 as i32;
        if (data as i32) < rs2_v {
            self.bus.sw_dram(addr, data);
        } else {
            self.bus.sw_dram(addr, self.reg.get_reg(inst.rs2) as u32);
        }
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO32(M[x[rs1]] MAXU x[rs2])
    fn amomaxu_w(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u32;
        if mm {
            data = self.l_mm(addr as u64) as u32;
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.lw_dram(addr as u64) as u32;
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64 as i32;
        if (data as i32) < rs2_v {
            self.bus.sw_dram(addr, self.reg.get_reg(inst.rs2) as u32);
        } else {
            self.bus.sw_dram(addr, data);
        }
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][31:0]
    fn lwu(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let addr = self.reg.get_reg(inst.rs1) as i64 + imm;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64) as u32 as u64;
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.lw_dram(addr as u64) as u64;
        }
        self.reg.set_reg(inst.rd, v);
    }

    /// x[rd] = M[x[rs1] + sext(offset)][63:0]
    fn ld(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let addr = self.reg.get_reg(inst.rs1) as i64 + imm;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::R);

        let v: u64;
        if (addr as usize) < MEM_OFF {
            v = self.l_mm(addr as u64);
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            v = self.bus.ld_dram(addr as u64);
        }
        self.reg.set_reg(inst.rd, v);
    }

    /// M[x[rs1] + sext(offset)] = x[rs2][63:0]
    fn sd(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let addr = self.reg.get_reg(inst.rs1) as i64 + imm;
        let addr = self.trans_addr(addr as u64);
        self.check_pmp(addr as u64, PMPPerm::W);

        let v = self.reg.get_reg(inst.rs2);
        if (addr as usize) < MEM_OFF {
            self.s_mm(addr as u64, v);
        } else {
            let addr = (addr as i64 - MEM_OFF as i64) as u64;
            self.bus.sd_dram(addr as u64, v);
        }
    }

    /// x[rd] = sext((x[rs1] + sext(immediate))[31:0])
    fn addiw(&mut self, inst: &Instruction) {
        let imm = sext(inst.imm as u64, 0x800);
        let v = self.reg.get_reg(inst.rs1) as i64 + imm;
        self.reg.set_reg(inst.rd, v as i32 as u64);
    }

    /// x[rd] = sext((x[rs1] << shamt)[31:0])
    fn slliw(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("reserved encoding of slliw");
        }
        let v = self.reg.get_reg(inst.rs1) << shamt;
        self.reg.set_reg(inst.rd, v as i32 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>u shamt)
    fn srliw(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("reserved encoding of srliw");
        }
        let v = (self.reg.get_reg(inst.rs1) as u32) >> shamt;
        self.reg.set_reg(inst.rd, v as i32 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>s shamt)
    fn sraiw(&mut self, inst: &Instruction) {
        let shamt = (inst.imm & 0b11_1111) as u8;
        if shamt & 0b10_0000 == 1 {
            panic!("reserved encoding of sraiw");
        }
        let rs1 = self.reg.get_reg(inst.rs1) as i32;
        let v = rs1 >> shamt;
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] + x[rs2])[31:0])
    fn addw(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) + self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v as i32 as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] - x[rs2])[31:0])
    fn subw(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) - self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v as i32 as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] << x[rs2][4:0])[31:0])
    fn sllw(&mut self, inst: &Instruction) {
        let shamt = self.reg.get_reg(inst.rs2) & 0b1_1111;
        let v = self.reg.get_reg(inst.rs1) << shamt;
        self.reg.set_reg(inst.rd, v as u32 as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>u x[rs2][4:0])
    fn srlw(&mut self, inst: &Instruction) {
        let shamt = self.reg.get_reg(inst.rs2) & 0b1_1111;
        let v = (self.reg.get_reg(inst.rs1) as u32) >> shamt;
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] >>s x[rs2][4:0])
    fn sraw(&mut self, inst: &Instruction) {
        let shamt = self.reg.get_reg(inst.rs2) & 0b1_1111;
        let v = (self.reg.get_reg(inst.rs1) as u32 as i32) >> shamt;
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext((x[rs1] × x[rs2])[31:0])
    fn mulw(&mut self, inst: &Instruction) {
        let v = self.reg.get_reg(inst.rs1) * self.reg.get_reg(inst.rs2);
        self.reg.set_reg(inst.rd, v as u32 as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] /s x[rs2][31:0]
    fn divw(&mut self, inst: &Instruction) {
        let v =
            (self.reg.get_reg(inst.rs1) as u32 as i32) / (self.reg.get_reg(inst.rs2) as u32 as i32);
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] /u x[rs2][31:0])
    fn divuw(&mut self, inst: &Instruction) {
        let v = (self.reg.get_reg(inst.rs1) as u32) / (self.reg.get_reg(inst.rs2) as u32);
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] %s x[rs2][31:0])
    fn remw(&mut self, inst: &Instruction) {
        let v =
            (self.reg.get_reg(inst.rs1) as u32 as i32) % (self.reg.get_reg(inst.rs2) as u32 as i32);
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = sext(x[rs1][31:0] %u x[rs2][31:0])
    fn remuw(&mut self, inst: &Instruction) {
        let v = (self.reg.get_reg(inst.rs1) as u32) % (self.reg.get_reg(inst.rs2) as u32);
        self.reg.set_reg(inst.rd, v as i64 as u64);
    }

    /// x[rd] = LoadReserved64(M[x[rs1]])
    fn lr_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::R);

        let data = self.bus.ld_dram(addr);
        self.reg.set_reg(inst.rd, data);
        self.reserve_mem(addr, true);
    }

    /// x[rd] = StoreConditional64(M[x[rs1]], x[rs2])
    fn sc_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);
        self.invalidate_mem_reservation(addr, true);

        let data = self.reg.get_reg(inst.rs2);

        if self.check_mem_reservation(addr, true) {
            panic!("memory is not reserved");
        }

        self.bus.sd_dram(addr, data);
        self.reg.set_reg(inst.rd, 0);
    }

    /// x[rd] = AMO64(M[x[rs1]] SWAP x[rs2])
    fn amoswap_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr);
        }

        if mm {
            self.bus.s_mm(addr, self.reg.get_reg(inst.rs2));
        } else {
            self.bus.sd_dram(addr, self.reg.get_reg(inst.rs2));
        }
        self.reg.set_reg(inst.rs2, data);
        self.reg.set_reg(inst.rd, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] + x[rs2])
    fn amoadd_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr);
        }

        let data = data as i64 + self.reg.get_reg(inst.rs2) as i64;
        self.bus.sd_dram(addr, data as u64);
        self.reg.set_reg(inst.rd, data as u64);
    }

    /// x[rd] = AMO64(M[x[rs1]] ^ x[rs2])
    fn amoxor_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let data = data ^ self.reg.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
        self.reg.set_reg(inst.rd, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] & x[rs2])
    fn amoand_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let data = data & self.reg.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
        self.reg.set_reg(inst.rd, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] | x[rs2])
    fn amoor_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let data = data | self.reg.get_reg(inst.rs2);
        self.bus.sd_dram(addr, data);
        self.reg.set_reg(inst.rd, data as i32 as i64 as u64);
    }

    /// x[rd] = AMO64(M[x[rs1]] MIN x[rs2])
    fn amomin_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64 as i32;
        if (data as i32) < rs2_v {
            self.bus.sd_dram(addr, data);
        } else {
            self.bus.sd_dram(addr, self.reg.get_reg(inst.rs2));
        }
        self.reg.set_reg(inst.rd, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] MAX x[rs2])
    fn amomax_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64;
        if (data as i64) < rs2_v {
            self.bus.sd_dram(addr, self.reg.get_reg(inst.rs2));
        } else {
            self.bus.sd_dram(addr, data);
        }
        self.reg.set_reg(inst.rd, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] MINU x[rs2])
    fn amominu_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64;
        if (data as i64) < rs2_v {
            self.bus.sd_dram(addr, data);
        } else {
            self.bus.sd_dram(addr, self.reg.get_reg(inst.rs2));
        }
        self.reg.set_reg(inst.rd, data);
    }

    /// x[rd] = AMO64(M[x[rs1]] MAXU x[rs2])
    fn amomaxu_d(&mut self, inst: &Instruction) {
        let addr = self.reg.get_reg(inst.rs1);
        let mut addr = self.trans_addr(addr);
        self.check_pmp(addr, PMPPerm::W);

        let mm = if (addr as usize) < MEM_OFF {
            true
        } else {
            false
        };
        let data: u64;
        if mm {
            data = self.l_mm(addr as u64);
        } else {
            addr = (addr as i64 - MEM_OFF as i64) as u64;
            data = self.bus.ld_dram(addr as u64);
        }

        let rs2_v = self.reg.get_reg(inst.rs2) as i64;
        if (data as i64) < rs2_v {
            self.bus.sd_dram(addr, self.reg.get_reg(inst.rs2));
        } else {
            self.bus.sd_dram(addr, data);
        }
        self.reg.set_reg(inst.rd, data);
    }
}

/// Sign-extended when imm is negative.
/// # Examples
/// ```ignore
/// let imm = 0xAD; // 0b1010_1101
/// let sign_bit = 0x80; // 0b1000_0000
/// assert_eq!(0xFFFF_FFFF_FFFF_FFAD, cpu::sext(imm, sign_bit));
/// ```
fn sext(imm: u64, sign_bit: u64) -> i64 {
    if sign_bit & imm == 0 {
        return imm as i64;
    }

    let mut sign = 1;
    let mut shift_cnt = sign_bit;
    loop {
        shift_cnt >>= 1;
        if shift_cnt == 0 {
            break;
        }
        sign <<= 1;
        sign += 1;
    }

    (imm | !sign) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dram::Dram;
    use crate::uart::Uart;
    use crate::plic::Plic;
    use crate::virtio::Virtio;

    #[test]
    fn lui_test() {
        let dram = Dram::new(0);
        let uart = Uart::new();
        let plic = Plic::new();
        let virtio = Virtio::new();
        let bus = Bus::new(dram, uart, plic, virtio);
        let dbg = Debug::new(false, 0);
        let mut cpu = Cpu::new(bus, 0, dbg);
        // lui	a0,0x1
        let inst = Instruction::decode(0x0000_1537);
        cpu.lui(&inst);
        assert_eq!(cpu.reg.a0, 0x1000);
    }
}
