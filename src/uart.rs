// https://en.wikibooks.org/wiki/Serial_Programming/8250_UART_Programming#UART_Registers

pub const UART: u64 = 0x10000000;
pub const THR: u64 = UART;
pub const RBR: u64 = UART;
pub const DLL: u64 = UART;
pub const IER: u64 = UART + 1;
pub const DLH: u64 = UART + 1;
pub const IIR: u64 = UART + 2;
pub const FCR: u64 = UART + 2;
pub const LCR: u64 = UART + 3;
pub const MCR: u64 = UART + 4;
pub const LSR: u64 = UART + 5;
pub const MSR: u64 = UART + 6;
pub const SR: u64 = UART + 7;
pub const UART_END: u64 = SR;

// IER bit
const IER_RHRI: u8 = 0b0000_0001; // receive holding register interrupt
const IER_THRI: u8 = 0b0000_0010; // transmit holding register interrupt
const IER_RLSI: u8 = 0b0000_0100; // receive line status interrupt
const IER_MSI: u8 = 0b0000_1000; // modem status interrupt

// LCR bit
const LCR_WLB0: u8 = 0b0000_0001; // word length bit 0
const LCR_WLB1: u8 = 0b0000_0010; // word length bit 1
const LCR_SBi: u8 = 0b0000_0100; // stop bits
const LCR_PE: u8 = 0b0000_1000; // parity enable
const LCR_EP: u8 = 0b0001_0000; // even parity
const LCR_SP: u8 = 0b0010_0000; // set parity
const LCR_SBr: u8 = 0b0100_0000; // set break
const LCR_DLE: u8 = 0b1000_0000; // divisor latch enable

// LSR bit
const LSR_RDR: u8 = 0b0000_0001; // receive data ready
const LSR_OE: u8 = 0b0000_0010; // overrun error
const LSR_PE: u8 = 0b0000_0100; // parity error
const LSR_FE: u8 = 0b0000_1000; // framing error
const LSR_BI: u8 = 0b0001_0000; // break interrupt
const LSR_THE: u8 = 0b0010_0000; // transmit holding empty
const LSR_TE: u8 = 0b0100_0000; // transmit empty
const LSR_0FE: u8 = 0b1000_0000; // 0/FIFO error

#[derive(Debug)]
pub struct Uart {
    pub thr: u8, // Transmit Holding Buffer
    pub rbr: u8, // Receive Buffer
    pub dll: u8, // Divisor Latch Low Byte
    pub ier: u8, // Interrupt Enable Register
    pub dlh: u8, // Divisor Latch High Byte
    pub iir: u8, // Interrupt Identification Register
    pub fcr: u8, // FIFO control Register
    pub lcr: u8, // Line Control Register
    pub mcr: u8, // Modem Control Register
    pub lsr: u8, // Line Status Register
    pub msr: u8, // Modem Status Register
    pub sr: u8,  // Scratch Register
}

impl Uart {
    pub fn new() -> Uart {
        Uart {
            thr: 0,
            rbr: 0,
            dll: 0,
            ier: 0,
            dlh: 0,
            iir: 0,
            fcr: 0,
            lcr: 0,
            mcr: 0,
            lsr: 0,
            msr: 0,
            sr: 0,
        }
    }

    pub fn print(&self) {
        println!("thr: {:08b}", self.thr);
        println!("rbr: {:08b}", self.rbr);
        println!("dll: {:08b}", self.dll);
        println!("ier: {:08b}", self.ier);
        println!("dlh: {:08b}", self.dlh);
        println!("iir: {:08b}", self.iir);
        println!("fcr: {:08b}", self.fcr);
        println!("lcr: {:08b}", self.lcr);
        println!("mcr: {:08b}", self.mcr);
        println!("lsr: {:08b}", self.lsr);
        println!("msr: {:08b}", self.msr);
        println!("sr: {:08b}", self.sr);
    }

    pub fn read(&self, addr: u64) -> u64 {
        0
    }

    pub fn write(&mut self, addr: u64, data: u64) {
        match addr {
            THR => self.w_thr(data),
            IER => self.ier = data as u8,
            FCR => self.fcr = data as u8,
            LCR => self.lcr = data as u8,
            MCR => self.mcr = data as u8,
            SR => self.sr = data as u8,
            _ => panic!("invalid write to uart register"),
        }
    }

    fn w_thr(&mut self, data: u64) {
        if self.lcr & LCR_DLE == 1 {}

        if self.lsr & LSR_THE == 0 {
            panic!("UART.THR is full");
        }
        self.lsr |= !LSR_THE;
        self.thr = data as u8;
        self.lsr |= LSR_THE;
    }
}
