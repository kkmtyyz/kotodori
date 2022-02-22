use crate::dram::Dram;
use crate::plic::{self, Plic};
use crate::uart::{self, Uart};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Bus {
    address: u32,
    data: u32,
    control: u32,
    dram: Dram,
    uart: Uart,
    plic: Plic,
}

impl Bus {
    pub fn new(dram: Dram, uart: Uart, plic: Plic) -> Bus {
        Bus {
            address: 0,
            data: 0,
            control: 0,
            dram,
            uart,
            plic,
        }
    }

    pub fn pdram_range(&self, begin: usize, end: usize) {
        self.dram.prange(begin, end);
    }

    pub fn puart(&self) {
        self.uart.print();
    }

    pub fn lb_dram(&self, addr: u64) -> u8 {
        self.dram.load_byte(addr)
    }

    pub fn lh_dram(&self, addr: u64) -> u16 {
        self.dram.load_hword(addr)
    }

    pub fn lw_dram(&self, addr: u64) -> u32 {
        self.dram.load_word(addr)
    }

    pub fn ld_dram(&self, addr: u64) -> u64 {
        self.dram.load_dword(addr)
    }

    pub fn sb_dram(&mut self, addr: u64, data: u8) {
        self.dram.store_byte(addr, data);
    }

    pub fn sh_dram(&mut self, addr: u64, data: u16) {
        self.dram.store_hword(addr, data);
    }

    pub fn sw_dram(&mut self, addr: u64, data: u32) {
        self.dram.store_word(addr, data);
    }

    pub fn sd_dram(&mut self, addr: u64, data: u64) {
        self.dram.store_dword(addr, data);
    }

    pub fn l_mm(&self, addr: u64) -> u64 {
        match addr {
            uart::UART..=uart::UART_END => self.uart.read(addr),
            plic::PLIC..=plic::PLIC_END => self.plic.read(addr),
            _ => panic!("invalid memory mapped address"),
        }
    }

    pub fn s_mm(&mut self, addr: u64, data: u64) {
        match addr {
            uart::UART..=uart::UART_END => self.uart.write(addr, data),
            plic::PLIC..=plic::PLIC_END => self.plic.write(addr, data),
            _ => panic!("invalid memory mapped address: 0x{:016X}", addr),
        }
    }
}
