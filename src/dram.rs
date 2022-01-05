#[derive(Debug)]
pub struct Dram {
    memory: Vec<u8>,
}

impl Dram {
    pub fn new(mem_size: usize) -> Dram {
        Dram {
            memory: vec![0; mem_size],
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.memory);
    }

    pub fn prange(&self, mut begin: usize, mut end: usize) {
        if begin % 16 != 0 {
            begin -= begin % 16;
        }
        if end % 16 != 0 {
            end += 16 - end % 16;
        }

        for i in begin..end {
            if i % 16 == 0 {
                print!("{:04X} | ", i);
            }

            print!("{:02X}", self.memory[i]);

            if (i + 1) % 2 == 0 {
                print!(" ");
            }
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
    }

    pub fn load(&mut self, data: String) {
        let mut data = Dram::to_byte_array(data);
        if self.memory.len() < data.len() {
            panic!("data is big");
        }

        if data.len() % 4 != 0 {
            for _ in 0..data.len() % 4 {
                data.push(0);
            }
        }

        // to little endian
        for i in (0..data.len()).step_by(4) {
            self.memory[i] = data[i + 3];
            self.memory[i + 1] = data[i + 2];
            self.memory[i + 2] = data[i + 1];
            self.memory[i + 3] = data[i];
        }
    }

    fn to_byte_array(data: String) -> Vec<u8> {
        if data.len() % 2 != 0 {
            panic!("data length is odd");
        }

        let mut chars = data.chars();
        let mut res = Vec::new();
        for _ in 0..data.len() / 2 {
            let hi = Dram::to_hex(chars.next().unwrap());
            let lo = Dram::to_hex(chars.next().unwrap());
            let byte = hi << 4 | lo;
            res.push(byte);
        }
        res
    }

    fn to_hex(c: char) -> u8 {
        match c.to_ascii_lowercase() {
            '0' => 0x0,
            '1' => 0x1,
            '2' => 0x2,
            '3' => 0x3,
            '4' => 0x4,
            '5' => 0x5,
            '6' => 0x6,
            '7' => 0x7,
            '8' => 0x8,
            '9' => 0x9,
            'a' => 0xa,
            'b' => 0xb,
            'c' => 0xc,
            'd' => 0xd,
            'e' => 0xe,
            'f' => 0xf,
            _ => panic!("Not hex: {}", c),
        }
    }

    pub fn load_byte(&self, addr: u64) -> u8 {
        match self.memory.get(addr as usize) {
            Some(data) => *data,
            None => panic!("access to invalid address"),
        }
    }

    pub fn load_hword(&self, begin: u64) -> u16 {
        let mut res: u16 = 0;
        for i in begin..begin + 2 {
            match self.memory.get(i as usize) {
                Some(data) => {
                    res <<= 8;
                    res |= *data as u16;
                }
                None => panic!("access to invalid address"),
            }
        }
        res
    }

    pub fn load_word(&self, addr: u64) -> u32 {
        let mut res: u32 = 0;
        for i in addr..addr + 4 {
            match self.memory.get(i as usize) {
                Some(data) => {
                    res <<= 8;
                    res |= *data as u32;
                }
                None => panic!("access to invalid address"),
            }
        }
        res
    }

    pub fn load_dword(&self, addr: u64) -> u64 {
        let mut res: u64 = 0;
        for i in addr..addr + 8 {
            match self.memory.get(i as usize) {
                Some(data) => {
                    res <<= 8;
                    res |= *data as u64;
                }
                None => panic!("access to invalid address"),
            }
        }
        res
    }

    pub fn store_byte(&mut self, addr: u64, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn store_hword(&mut self, addr: u64, data: u16) {
        self.memory[addr as usize] = (data & 0xF) as u8;
        let addr = addr + 1;
        self.memory[addr as usize] = (data >> 8) as u8;
    }

    pub fn store_word(&mut self, mut addr: u64, data: u32) {
        self.memory[addr as usize] = (data & 0xF) as u8;

        for _ in 0..3 {
            addr = addr + 1;
            self.memory[addr as usize] = ((data >> 8) & 0xF) as u8;
        }
    }

    pub fn store_dword(&mut self, mut addr: u64, data: u64) {
        self.memory[addr as usize] = (data & 0xF) as u8;

        for _ in 0..7 {
            addr = addr + 1;
            self.memory[addr as usize] = ((data >> 8) & 0xF) as u8;
        }
    }
}
