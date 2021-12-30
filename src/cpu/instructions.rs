fn to_format(opcode: u8) -> InstFmt {
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
        0b111_0011 => InstFmt::I,
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
        0b010_0011 => match funct3 {
            0b000 => InstName::Sb("sb".to_owned()),
            0b001 => InstName::Sh("sh".to_owned()),
            0b010 => InstName::Sw("sw".to_owned()),
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
                _ => panic!("convert to instruction name"),
            },
            0b001 => InstName::Sll("sll".to_owned()),
            0b010 => InstName::Slt("slt".to_owned()),
            0b011 => InstName::Sltu("sltu".to_owned()),
            0b100 => InstName::Xor("xor".to_owned()),
            0b101 => match funct7 {
                0b000_0000 => InstName::Srl("srl".to_owned()),
                0b010_0000 => InstName::Sra("sra".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b110 => InstName::Or("or".to_owned()),
            0b111 => InstName::Xor("and".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b000_1111 => match funct3 {
            0b000 => InstName::Fence("fence".to_owned()),
            0b001 => InstName::FenceI("fence.i".to_owned()),
            _ => panic!("convert to instruction name"),
        },
        0b111_0011 => match funct3 {
            0b000 => match funct12 {
                0b0000_0000_0000 => InstName::Ecall("ecall".to_owned()),
                0b0000_0000_0001 => InstName::Ebreak("ebreak".to_owned()),
                _ => panic!("convert to instruction name"),
            },
            0b001 => InstName::Csrrw("csrrw".to_owned()),
            0b010 => InstName::Csrrs("csrrs".to_owned()),
            0b011 => InstName::Csrrc("csrrc".to_owned()),
            0b101 => InstName::Csrrwi("csrrwi".to_owned()),
            0b110 => InstName::Csrrsi("csrrsi".to_owned()),
            0b111 => InstName::Csrrci("csrrci".to_owned()),
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
}

pub struct Instruction {
    opcode: u8,
    name: InstName,
    format: InstFmt,
    inst: u32,
}

impl Instruction {
    pub fn new(inst: u32) -> Instruction {
        let opcode = (inst & 0b0111_1111) as u8;
        let format = to_format(opcode);
        let mut funct3: u8 = 0;
        let mut funct7: u8 = 0;
        let mut funct12: u16 = 0;
        match format {
            InstFmt::R | InstFmt::I | InstFmt::S | InstFmt::B => {
                funct3 = (inst >> 12 & 0b111) as u8;
                if let InstFmt::I | InstFmt::R = format {
                    funct7 = (inst >> 25 & 0b111_1111) as u8;
                }
                if let InstFmt::I = format {
                    funct12 = (inst >> 20 & 0b1111_1111_1111) as u16
                }
            }
            _ => (),
        }
        let name = to_name(opcode, funct3, funct7, funct12);
        Instruction {
            opcode,
            name,
            format,
            inst,
        }
    }

    pub fn print(&self) {
        println!(
            "opcode: {:b}, name: {:?}, format: {:?}, inst: {:08X}",
            self.opcode, self.name, self.format, self.inst
        );
    }
}
