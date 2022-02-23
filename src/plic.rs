// https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc

pub const PLIC: u64 = 0xC00_0000;
pub const PRIORITY: u64 = PLIC + 0x4; // 0x0004 - 0x0FFC
pub const PRIORITY_END: u64 = PLIC + 0xFFC;
pub const PENDING: u64 = PLIC + 0x1000; // 0x1000 - 0x107C
pub const PENDING_END: u64 = PLIC + 0x107C;
pub const ENABLE: u64 = PLIC + 0x2000; // 0x2000 - 0x1F_1FFC
pub const ENABLE_END: u64 = PLIC + 0x1F_1FFC;
pub const PRIORITY_THR0: u64 = PLIC + 0x20_0000;
pub const CLAIM0: u64 = PLIC + 0x20_0004;
pub const PRIORITY_THR: u64 = PLIC + 0x20_1000; // 0x20_1000 - 0x3FF_F000
pub const CLAIM: u64 = PLIC + 0x20_1004; // 0x20_1004 - 0x3FF_F004
pub const CLAIM_END: u64 = PLIC + 0x3FF_F004;
pub const PLIC_END: u64 = PLIC + 0x3FF_FFFC;

#[derive(Debug)]
pub struct Plic {
    pub priority: Vec<u32>,
    pub pending: Vec<u32>,
    pub enable: Vec<u32>,
    pub priority_thr: Vec<u32>,
    pub claim: Vec<u32>,
}

impl Plic {
    pub fn new() -> Plic {
        Plic {
            priority: vec![0; 1023],      // Interrupt source 1 - 1023 priority
            pending: vec![0; 32],         // Interrupt Pending bit 0 - 1023
            enable: vec![0; 32 * 15872],  // Enable bits for sources. 1024bit. context 0 - 15871
            priority_thr: vec![0; 15872], // Priority threshold for context 0 - 15871
            claim: vec![0; 15872],        // Claim/complete for context 0 - 15871
        }
    }

    pub fn read(&self, addr: u64) -> u64 {
        match addr {
            PRIORITY..=PRIORITY_END => self.read_priority(addr),
            PENDING..=PENDING_END => self.read_pending(addr),
            ENABLE..=ENABLE_END => self.read_enable(addr),
            PRIORITY_THR0 => self.read_priority_thr(addr),
            CLAIM0 => self.read_claim(addr),
            PRIORITY_THR..=CLAIM_END => {
                if addr & 0xF == 0x0 || addr & 0xF == 0x8 {
                    return self.read_priority_thr(addr);
                }
                return self.read_claim(addr);
            }
            _ => panic!("invalid PLIC address: 0x{:016X}", addr),
        }
    }

    fn read_priority(&self, addr: u64) -> u64 {
        0
    }

    fn read_pending(&self, addr: u64) -> u64 {
        0
    }

    fn read_enable(&self, addr: u64) -> u64 {
        0
    }

    fn read_priority_thr(&self, addr: u64) -> u64 {
        0
    }

    fn read_claim(&self, addr: u64) -> u64 {
        0
    }

    pub fn write(&mut self, addr: u64, data: u64) {}
}
