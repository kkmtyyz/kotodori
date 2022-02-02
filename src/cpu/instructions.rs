fn to_format(opcode: u8, funct3: u8, funct7: u8) -> InstFmt {
    match opcode {
        0b011_0111 => InstFmt::U,
        0b001_0111 => InstFmt::U,
        0b110_1111 => InstFmt::J,
        0b110_0111 => InstFmt::I,
        0b110_0011 => InstFmt::B,
        0b000_0011 => InstFmt::I,
        0b010_0011 => InstFmt::S,
        0b001_0011 => InstFmt::I,
        0b011_0011 => InstFmt::R,
        0b000_1111 => InstFmt::I,
        0b111_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => InstFmt::I,
                _ => InstFmt::R,
            },
            _ => InstFmt::I,
        },
        0b001_1011 => InstFmt::I,
        _ => panic!("convert to instruction format"),
    }
}

fn to_name(opcode: u8, funct3: u8, funct7: u8, funct12: u16) -> InstName {
    match opcode {
        0b011_0111 => InstName::Lui("lui".to_owned()),
        0b001_0111 => InstName::Auipc("auipc".to_owned()),
        0b110_1111 => InstName::Jal("jal".to_owned()),
        0b110_0111 => InstName::Jalr("jalr".to_owned()),
        0b110_0011 => match funct3 {
            0b000 => InstName::Beq("beq".to_owned()),
            0b001 => InstName::Bne("bne".to_owned()),
            0b100 => InstName::Blt("blt".to_owned()),
            0b101 => InstName::Bge("bge".to_owned()),
            0b110 => InstName::Bltu("bltu".to_owned()),
            0b111 => InstName::Bgeu("bgeu".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b000_0011 => match funct3 {
            0b000 => InstName::Lb("lb".to_owned()),
            0b001 => InstName::Lh("lh".to_owned()),
            0b010 => InstName::Lw("lw".to_owned()),
            0b100 => InstName::Lbu("lbu".to_owned()),
            0b101 => InstName::Lhu("lhu".to_owned()),
            0b110 => InstName::Lwu("lwu".to_owned()),
            0b011 => InstName::Ld("ld".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b010_0011 => match funct3 {
            0b000 => InstName::Sb("sb".to_owned()),
            0b001 => InstName::Sh("sh".to_owned()),
            0b010 => InstName::Sw("sw".to_owned()),
            0b011 => InstName::Sd("sd".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b001_0011 => match funct3 {
            0b000 => InstName::Addi("addi".to_owned()),
            0b010 => InstName::Slti("slti".to_owned()),
            0b011 => InstName::Sltiu("sltiu".to_owned()),
            0b100 => InstName::Xori("xori".to_owned()),
            0b110 => InstName::Ori("ori".to_owned()),
            0b111 => InstName::Andi("andi".to_owned()),
            0b001 => InstName::Slli("slli".to_owned()),
            0b101 => match funct7 {
                0b000_0000 => InstName::Srli("srli".to_owned()),
                0b010_0000 => InstName::Srai("srai".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            _ => panic!("convert to instruction name"),
        },
        0b011_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => InstName::Add("add".to_owned()),
                0b010_0000 => InstName::Sub("sub".to_owned()),
                0b000_0001 => InstName::Mul("mul".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b001 => match funct7 {
                0b000_0000 => InstName::Sll("sll".to_owned()),
                0b000_0001 => InstName::Mulh("mulh".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b010 => match funct7 {
                0b000_0000 => InstName::Slt("slt".to_owned()),
                0b000_0001 => InstName::Mulhsu("mulhsu".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b011 => match funct7 {
                0b000_0000 => InstName::Sltu("sltu".to_owned()),
                0b000_0001 => InstName::Mulhu("mulhu".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b100 => match funct7 {
                0b000_0000 => InstName::Xor("xor".to_owned()),
                0b000_0001 => InstName::Div("div".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b101 => match funct7 {
                0b000_0000 => InstName::Srl("srl".to_owned()),
                0b010_0000 => InstName::Sra("sra".to_owned()),
                0b000_0001 => InstName::Divu("divu".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b110 => match funct7 {
                0b000_0000 => InstName::Or("or".to_owned()),
                0b000_0001 => InstName::Rem("rem".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b111 => match funct7 {
                0b000_0000 => InstName::And("and".to_owned()),
                0b000_0001 => InstName::Remu("remu".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            _ => panic!("convert to instruction name"),
        },
        0b000_1111 => match funct3 {
            0b000 => InstName::Fence("fence".to_owned()),
            0b001 => InstName::FenceI("fence.i".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b111_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_1001 => InstName::SfenceVma("sfence.vma".to_owned()),
                _ => match funct12 {
                    0b0000_0000_0000 => InstName::Ecall("ecall".to_owned()),
                    0b0000_0000_0001 => InstName::Ebreak("ebreak".to_owned()),
                    0b0001_0000_0010 => InstName::Sret("sret".to_owned()),
                    0b0011_0000_0010 => InstName::Mret("mret".to_owned()),
                    0b0001_0000_0101 => InstName::Wfi("wfi".to_owned()),
                    _ => panic!("convert to instruction name"),
                },
            },
            0b001 => InstName::Csrrw("csrrw".to_owned()),
            0b010 => InstName::Csrrs("csrrs".to_owned()),
            0b011 => InstName::Csrrc("csrrc".to_owned()),
            0b101 => InstName::Csrrwi("csrrwi".to_owned()),
            0b110 => InstName::Csrrsi("csrrsi".to_owned()),
            0b111 => InstName::Csrrci("csrrci".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b010_1111 => {
            let funct7 = (funct7 >> 2) & 0b1_1111;
            match funct3 {
                0b010 => match funct7 {
                    0b0_0010 => InstName::LrW("lr.w".to_owned()),
                    0b0_0011 => InstName::ScW("sc.w".to_owned()),
                    0b0_0001 => InstName::AmoswapW("amoswap.w".to_owned()),
                    0b0_0000 => InstName::AmoaddW("amoadd.w".to_owned()),
                    0b0_0100 => InstName::AmoxorW("amoxor.w".to_owned()),
                    0b0_1100 => InstName::AmoandW("amoand.w".to_owned()),
                    0b0_1000 => InstName::AmoorW("amoor.w".to_owned()),
                    0b1_0000 => InstName::AmominW("amomin.w".to_owned()),
                    0b1_0100 => InstName::AmomaxW("amomax.w".to_owned()),
                    0b1_1000 => InstName::AmominuW("amominu.w".to_owned()),
                    0b1_1100 => InstName::AmomaxuW("amomaxu.w".to_owned()),
                    _ => panic!("convert to instruction name"),
                },
                0b011 => match funct7 {
                    0b0_0010 => InstName::LrD("lr.d".to_owned()),
                    0b0_0011 => InstName::ScD("sc.d".to_owned()),
                    0b0_0001 => InstName::AmoswapD("amoswap.d".to_owned()),
                    0b0_0000 => InstName::AmoaddD("amoadd.d".to_owned()),
                    0b0_0100 => InstName::AmoxorD("amoxor.d".to_owned()),
                    0b0_1100 => InstName::AmoandD("amoand.d".to_owned()),
                    0b0_1000 => InstName::AmoorD("amoor.d".to_owned()),
                    0b1_0000 => InstName::AmominD("amomin.d".to_owned()),
                    0b1_0100 => InstName::AmomaxD("amomax.d".to_owned()),
                    0b1_1000 => InstName::AmominuD("amominu.d".to_owned()),
                    0b1_1100 => InstName::AmomaxuD("amomaxu.d".to_owned()),
                    _ => panic!("convert to instruction name"),
                },
                _ => panic!("convert to instruction name"),
            }
        }
        0b001_1011 => match funct3 {
            0b000 => InstName::Addiw("addiw".to_owned()),
            0b001 => InstName::Slliw("slliw".to_owned()),
            0b101 => match funct7 {
                0b000_0000 => InstName::Srliw("srliw".to_owned()),
                0b010_0000 => InstName::Sraiw("sraiw".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            _ => panic!("convert to instruction name"),
        },
        0b011_1011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => InstName::Addw("addw".to_owned()),
                0b010_0000 => InstName::Subw("subw".to_owned()),
                0b000_0001 => InstName::Mulw("mulw".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b001 => InstName::Sllw("sllw".to_owned()),
            0b100 => InstName::Divw("divw".to_owned()),
            0b101 => match funct7 {
                0b000_0000 => InstName::Srlw("srlw".to_owned()),
                0b010_0000 => InstName::Sraw("sraw".to_owned()),
                0b000_0001 => InstName::Divuw("divuw".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b110 => InstName::Remw("remw".to_owned()),
            0b111 => InstName::Remuw("remuw".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        _ => panic!("convert to instruction name"),
    }
}

#[derive(Debug)]
pub enum InstFmt {
    R,
    I,
    S,
    B,
    U,
    J,
}

#[derive(Debug)]
pub enum InstName {
    // RV32I
    Lui(String),
    Auipc(String),
    Jal(String),
    Jalr(String),
    Beq(String),
    Bne(String),
    Blt(String),
    Bge(String),
    Bltu(String),
    Bgeu(String),
    Lb(String),
    Lh(String),
    Lw(String),
    Lbu(String),
    Lhu(String),
    Sb(String),
    Sh(String),
    Sw(String),
    Addi(String),
    Slti(String),
    Sltiu(String),
    Xori(String),
    Ori(String),
    Andi(String),
    Slli(String),
    Srli(String),
    Srai(String),
    Add(String),
    Sub(String),
    Sll(String),
    Slt(String),
    Sltu(String),
    Xor(String),
    Srl(String),
    Sra(String),
    Or(String),
    And(String),
    Fence(String),
    FenceI(String),
    Ecall(String),
    Ebreak(String),
    Csrrw(String),
    Csrrs(String),
    Csrrc(String),
    Csrrwi(String),
    Csrrsi(String),
    Csrrci(String),
    Sret(String),
    Mret(String),
    Wfi(String),
    SfenceVma(String),

    // RV32A
    Mul(String),
    Mulh(String),
    Mulhsu(String),
    Mulhu(String),
    Div(String),
    Divu(String),
    Rem(String),
    Remu(String),

    // RV32A
    LrW(String),
    ScW(String),
    AmoswapW(String),
    AmoaddW(String),
    AmoxorW(String),
    AmoandW(String),
    AmoorW(String),
    AmominW(String),
    AmomaxW(String),
    AmominuW(String),
    AmomaxuW(String),

    // RV64I
    Lwu(String),
    Ld(String),
    Sd(String),
    Addiw(String),
    Slliw(String),
    Srliw(String),
    Sraiw(String),
    Addw(String),
    Subw(String),
    Sllw(String),
    Srlw(String),
    Sraw(String),

    // RV64M
    Mulw(String),
    Divw(String),
    Divuw(String),
    Remw(String),
    Remuw(String),

    // RV64A
    LrD(String),
    ScD(String),
    AmoswapD(String),
    AmoaddD(String),
    AmoxorD(String),
    AmoandD(String),
    AmoorD(String),
    AmominD(String),
    AmomaxD(String),
    AmominuD(String),
    AmomaxuD(String),
}

fn to_funct(inst: u32, fmt: &InstFmt) -> (u8, u8, u16) {
    let mut funct3: u8 = 0;
    let mut funct7: u8 = 0;
    let mut funct12: u16 = 0;
    match fmt {
        InstFmt::R | InstFmt::I | InstFmt::S | InstFmt::B => {
            funct3 = (inst >> 12 & 0b111) as u8;
            if let InstFmt::I | InstFmt::R = fmt {
                funct7 = (inst >> 25 & 0b111_1111) as u8;
                funct12 = (inst >> 20 & 0b1111_1111_1111) as u16;
            }
        }
        _ => (),
    }
    (funct3, funct7, funct12)
}

fn to_ri(inst: u32, fmt: &InstFmt) -> (u8, u8, u8, u32) {
    let mut rs1: u8 = 0;
    let mut rs2: u8 = 0;
    let mut rd: u8 = 0;
    let mut imm: u32 = 0;
    match fmt {
        InstFmt::R => {
            rs1 = (inst >> 15 & 0b1_1111) as u8;
            rs2 = (inst >> 20 & 0b1_1111) as u8;
            rd = (inst >> 7 & 0b1_1111) as u8;
        }
        InstFmt::I => {
            rs1 = (inst >> 15 & 0b1_1111) as u8;
            rd = (inst >> 7 & 0b1_1111) as u8;
            imm = inst >> 20 & 0b1111_1111_1111;
        }
        InstFmt::S => {
            rs1 = (inst >> 15 & 0b1_1111) as u8;
            rs2 = (inst >> 20 & 0b1_1111) as u8;
            imm = inst >> 25 & 0b111_1111;
            imm <<= 5;
            imm |= inst >> 7 & 0b1_1111;
        }
        InstFmt::B => {
            rs1 = (inst >> 15 & 0b1_1111) as u8;
            rs2 = (inst >> 20 & 0b1_1111) as u8;
            imm = inst >> 31 & 0b1;
            imm <<= 1;
            imm |= inst >> 7 & 0b1;
            imm <<= 6;
            imm |= inst >> 25 & 0b11_1111;
            imm <<= 4;
            imm |= inst >> 8 & 0b1111;
            imm <<= 1;
        }
        InstFmt::U => {
            rd = (inst >> 7 & 0b1_1111) as u8;
            imm = inst >> 12
        }
        InstFmt::J => {
            rd = (inst >> 7 & 0b1_1111) as u8;
            imm = inst >> 31 & 0b1;
            imm <<= 8;
            imm |= inst >> 12 & 0b1111_1111;
            imm <<= 1;
            imm |= inst >> 20 & 0b1;
            imm <<= 10;
            imm |= inst >> 21 & 0b11_1111_1111;
            imm <<= 1;
        }
    }
    (rs1, rs2, rd, imm)
}

pub struct Instruction {
    pub opcode: u8, // 7bit
    pub name: InstName,
    pub fmt: InstFmt,
    pub rs1: u8,  // 5bit
    pub rs2: u8,  // 5bit
    pub rd: u8,   // 5bit
    pub imm: u32, // 19bit
    pub raw_inst: u32,
}

impl Instruction {
    pub fn decode(inst: u32) -> Instruction {
        let opcode = (inst & 0b0111_1111) as u8;
        // TODO refactoring
        let funct3 = (inst >> 12 & 0b111) as u8;
        let funct7 = (inst >> 25 & 0b111_1111) as u8;
        let fmt = to_format(opcode, funct3, funct7);
        let (funct3, funct7, funct12) = to_funct(inst, &fmt);
        let name = to_name(opcode, funct3, funct7, funct12);
        let (rs1, rs2, rd, imm) = to_ri(inst, &fmt);
        Instruction {
            opcode,
            name,
            fmt,
            rs1,
            rs2,
            rd,
            imm,
            raw_inst: inst,
        }
    }

    pub fn print(&self) {
        println!(
            "opcode: {:b}, name: {:?}, fmt: {:?}, raw_inst: {:08X}",
            self.opcode, self.name, self.fmt, self.raw_inst
        );
        println!(
            "rs1: {:05b}, rs2: {:05b}, rd: {:05b}, imm: {:032b}",
            self.rs1, self.rs2, self.rd, self.imm
        );
    }
}
