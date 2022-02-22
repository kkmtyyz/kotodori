// https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc

pub const PLIC: u64 = 0xC00_0000;
pub const PLIC_END: u64 = PLIC + 0x3FF_FFFC;

#[derive(Debug)]
pub struct Plic {}

impl Plic {
    pub fn new() -> Plic {
        Plic {}
    }

    pub fn read(&self, addr: u64) -> u64 {
        0
    }

    pub fn write(&mut self, addr: u64, data: u64) {}
}
