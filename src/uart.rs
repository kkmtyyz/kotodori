// http://byterunner.com/16550.html

pub const UART: u64 = 0x10000000;
pub const RHR: u64 = UART;
pub const THR: u64 = UART;
pub const IER: u64 = UART + 1;
pub const FCR: u64 = UART + 2;
pub const ISR: u64 = UART + 2;
pub const LCR: u64 = UART + 3;
pub const MCR: u64 = UART + 4;
pub const LSR: u64 = UART + 5;
pub const MSR: u64 = UART + 6;
pub const SPR: u64 = UART + 7;
pub const UART_END: u64 = SPR;

#[derive(Debug)]
pub struct Uart {
    pub rhr: u8,
    pub thr: u8,
    pub ier: u8,
    pub fcr: u8,
    pub isr: u8,
    pub lcr: u8,
    pub mcr: u8,
    pub lsr: u8,
    pub msr: u8,
    pub spr: u8,
}

impl Uart {
    pub fn new() -> Uart {
        Uart {
            rhr: 0,
            thr: 0,
            ier: 0,
            fcr: 0,
            isr: 0,
            lcr: 0,
            mcr: 0,
            lsr: 0,
            msr: 0,
            spr: 0,
        }
    }

    pub fn wirte(&mut self, addr: u64, data: u64) {}
}
