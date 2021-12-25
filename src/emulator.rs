use crate::cpu::Register;
use crate::dram::Dram;

pub struct Emulator {
    reg: Register,
    dram: Dram
}

impl Emulator {
    pub fn new(mem_size: usize) -> Emulator {
        Emulator {
            reg: Register::new(),
            dram: Dram::new(mem_size),
        }
    }

    pub fn print_reg(self) {
        self.reg.print();
    }

    pub fn print_dram(self) {
        self.dram.print();
    }
}
