use crate::dram::Dram;

#[derive(Debug)]
pub struct Bus {
    address: u32,
    data: u32,
    control: u32,
    dram: Dram,
}

impl Bus {
    pub fn new(dram: Dram) -> Bus {
        Bus {
            address: 0,
            data: 0,
            control: 0,
            dram,
        }
    }

    pub fn pdram_range(&self, begin: usize, end: usize) {
        self.dram.prange(begin, end);
    }

    pub fn load_dram(&self, addr: u32) -> u32 {
        self.dram.load_dword(addr)
    }
}
