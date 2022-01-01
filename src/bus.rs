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

    pub fn lb_dram(&self, addr: u32) -> u8 {
        self.dram.load_byte(addr)
    }

    pub fn lh_dram(&self, addr: u32) -> u16 {
        self.dram.load_hword(addr)
    }

    pub fn lw_dram(&self, addr: u32) -> u32 {
        self.dram.load_word(addr)
    }

    pub fn sb_dram(&mut self, addr: u32, data: u8) {
        self.dram.store_byte(addr, data);
    }

    pub fn sh_dram(&mut self, addr: u32, data: u16) {
        self.dram.store_hword(addr, data);
    }

    pub fn sw_dram(&mut self, addr: u32, data: u32) {
        self.dram.store_word(addr, data);
    }
}
