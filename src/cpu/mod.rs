pub mod instructions;

use crate::bus::Bus;
use crate::conf;
use instructions::InstName;
use instructions::Instruction;

#[derive(Debug)]
pub struct Cpu {
    bus: Bus,

    // registers
    zero: u32,
    ra: u32,
    sp: u32,
    gp: u32,
    tp: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    fp: u32, // s0
    s1: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    pc: u32,
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
        }
    }

    pub fn print(&self) {
        println!(
            "zero:{:032b}, ra:{:032b}, sp:{:032b}, gp:{:032b}",
            self.zero, self.ra, self.sp, self.gp
        );
        println!(
            "tp:{:032b}, t0:{:032b}, t1:{:032b}, t2:{:032b}",
            self.tp, self.t0, self.t1, self.t2
        );
        println!(
            "fp:{:032b}, s1:{:032b}, a0:{:032b}, a1:{:032b}",
            self.fp, self.s1, self.a0, self.a1
        );
        println!(
            "a2:{:032b}, a3:{:032b}, a4:{:032b}, a5:{:032b}",
            self.a2, self.a3, self.a4, self.a5
        );
        println!(
            "a6:{:032b}, a7:{:032b}, a2:{:032b}, a3:{:032b}",
            self.a6, self.a7, self.s2, self.s3
        );
        println!(
            "s4:{:032b}, s5:{:032b}, s6:{:032b}, s7:{:032b}",
            self.s4, self.s5, self.s6, self.s7
        );
        println!(
            "s8:{:032b}, s9:{:032b}, s10:{:032b}, s11:{:032b}",
            self.s8, self.s9, self.s10, self.s11
        );
        println!(
            "t3:{:032b}, t4:{:032b}, t5:{:032b}, t6:{:032b}",
            self.t3, self.t4, self.t5, self.t6
        );
        println!("pc:{:032b}", self.pc);
    }

    pub fn pdram_range(&self, begin: usize, end: usize) {
        self.bus.pdram_range(begin, end);
    }

    pub fn get_reg(&self, reg: u8) -> u32 {
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

    pub fn set_reg(&mut self, reg: u8, value: u32) {
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
            _ => (),
        }
    }

    fn lui(&mut self, inst: &Instruction) {
        let v = (inst.imm as i32) << 12;
        self.set_reg(inst.rd, v as u32);
    }

    fn auipc(&mut self, inst: &Instruction) {
        let v = ((inst.imm as i32) << 12) + self.pc as i32;
        self.set_reg(inst.rd, v as u32);
    }

    fn jal(&mut self, inst: &Instruction) {
        if inst.rd == 0 {
            self.ra = self.pc + 4; // ra == x1
        } else {
            self.set_reg(inst.rd, self.pc + 4);
        }
        let v = self.pc as i32 + inst.imm as i32;
        self.pc = v as u32;
    }

    fn jalr(&mut self, inst: &Instruction) {
        let t = self.pc + 4;
        let v = (self.get_reg(inst.rs1) as i32 + inst.imm as i32) as u32;
        self.pc = v as u32 & 0xFFFF_FFFE;

        if inst.rd == 0 {
            self.ra = t;
        } else {
            self.set_reg(inst.rd, t);
        }
    }

    fn beq(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) == self.get_reg(inst.rs2) {
            self.pc = (self.pc as i32 + inst.imm as i32) as u32;
        }
    }

    fn bne(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) != self.get_reg(inst.rs2) {
            self.pc = (self.pc as i32 + inst.imm as i32) as u32;
        }
    }

    fn blt(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i32) < (self.get_reg(inst.rs2) as i32) {
            self.pc = (self.pc as i32 + inst.imm as i32) as u32;
        }
    }

    fn bge(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i32) >= (self.get_reg(inst.rs2) as i32) {
            self.pc = (self.pc as i32 + inst.imm as i32) as u32;
        }
    }

    fn bltu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < self.get_reg(inst.rs2) {
            self.pc = (self.pc as i32 + inst.imm as i32) as u32;
        }
    }

    fn bgeu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) >= self.get_reg(inst.rs2) {
            self.pc = (self.pc as i32 + inst.imm as i32) as u32;
        }
    }

    fn lb(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        let v = self.bus.lb_dram(addr as u32) as i32;
        self.set_reg(inst.rd, v as u32);
    }

    fn lh(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        let v = self.bus.lh_dram(addr as u32) as i32;
        self.set_reg(inst.rd, v as u32);
    }

    fn lw(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        let v = self.bus.lw_dram(addr as u32);
        self.set_reg(inst.rd, v);
    }

    fn lbu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        let v = self.bus.lb_dram(addr as u32);
        self.set_reg(inst.rd, v as u32);
    }

    fn lhu(&mut self, inst: &Instruction) {
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        let v = self.bus.lh_dram(addr as u32);
        self.set_reg(inst.rd, v as u32);
    }

    fn sb(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs2) as u8;
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        self.bus.sb_dram(addr as u32, v);
    }

    fn sh(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs2) as u16;
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        self.bus.sh_dram(addr as u32, v);
    }

    fn sw(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs2);
        let addr = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        self.bus.sw_dram(addr as u32, v);
    }

    fn addi(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) as i32 + inst.imm as i32;
        self.set_reg(inst.rd, v as u32);
    }

    fn slti(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i32) < (inst.imm as i32) {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    fn sltiu(&mut self, inst: &Instruction) {
        if self.get_reg(inst.rs1) < inst.imm {
            self.set_reg(inst.rd, 1);
        } else {
            self.set_reg(inst.rd, 0);
        }
    }

    fn xori(&mut self, inst: &Instruction) {
        let v = (inst.imm as i32) ^ (self.get_reg(inst.rs1) as i32);
        self.set_reg(inst.rd, v as u32);
    }

    fn ori(&mut self, inst: &Instruction) {
        let v = (inst.imm as i32) | (self.get_reg(inst.rs1) as i32);
        self.set_reg(inst.rd, v as u32);
    }

    fn andi(&mut self, inst: &Instruction) {
        let v = (inst.imm as i32) & (self.get_reg(inst.rs1) as i32);
        self.set_reg(inst.rd, v as u32);
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
        let v = self.get_reg(inst.rs1) as i32 + self.get_reg(inst.rs2) as i32;
        self.set_reg(inst.rd, v as u32);
    }

    fn sub(&mut self, inst: &Instruction) {
        let v = self.get_reg(inst.rs1) as i32 - self.get_reg(inst.rs2) as i32;
        self.set_reg(inst.rd, v as u32);
    }

    fn sll(&mut self, inst: &Instruction) {
        let shamt = self.get_reg(inst.rs2) & 0b1_1111;
        let v = self.get_reg(inst.rs1) << shamt;
        self.set_reg(inst.rd, v);
    }

    fn slt(&mut self, inst: &Instruction) {
        if (self.get_reg(inst.rs1) as i32) < (self.get_reg(inst.rs2) as i32) {
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
}
