use crate::conf::MEM_OFF;
use crate::dram::Dram;

#[derive(Debug)]
#[allow(dead_code)]
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
        self.dram.prange(begin - MEM_OFF, end - MEM_OFF);
    }

    pub fn lb_dram(&self, addr: u64) -> u8 {
        self.dram.load_byte(addr - MEM_OFF as u64)
    }

    pub fn lh_dram(&self, addr: u64) -> u16 {
        self.dram.load_hword(addr - MEM_OFF as u64)
    }

    pub fn lw_dram(&self, addr: u64) -> u32 {
        self.dram.load_word(addr - MEM_OFF as u64)
    }

    pub fn ld_dram(&self, addr: u64) -> u64 {
        self.dram.load_dword(addr - MEM_OFF as u64)
    }

    pub fn sb_dram(&mut self, addr: u64, data: u8) {
        self.dram.store_byte(addr - MEM_OFF as u64, data);
    }

    pub fn sh_dram(&mut self, addr: u64, data: u16) {
        self.dram.store_hword(addr - MEM_OFF as u64, data);
    }

    pub fn sw_dram(&mut self, addr: u64, data: u32) {
        self.dram.store_word(addr - MEM_OFF as u64, data);
    }

    pub fn sd_dram(&mut self, addr: u64, data: u64) {
        self.dram.store_dword(addr - MEM_OFF as u64, data);
    }
}
