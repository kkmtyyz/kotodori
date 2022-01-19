use crate::conf::MEM_OFF;

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

    #[inline(always)]
    fn set_mem(&mut self, idx: usize, data: u8) {
        if self.memory.len() < idx {
            panic!("access to invalid address");
        }
        self.memory[idx] = data;
    }

    #[inline(always)]
    fn get_mem(&self, idx: usize) -> u8 {
        match self.memory.get(idx) {
            Some(data) => *data,
            None => panic!("access to invalid address: 0x{:016X}", idx),
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.memory);
    }

    pub fn prange(&self, mut begin: usize, mut end: usize) {
        begin -= MEM_OFF;
        end -= MEM_OFF;
        if begin % 16 != 0 {
            begin -= begin % 16;
        }
        if end % 16 != 0 {
            end += 16 - end % 16;
        }

        for i in begin..end {
            if i % 16 == 0 {
                print!("{:016X} | ", i + MEM_OFF);
            }

            print!("{:02X}", self.get_mem(i));

            if (i + 1) % 2 == 0 {
                print!(" ");
            }
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
    }

    pub fn load(&mut self, data: String) {
        let mut data = to_byte_array(data);
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
            self.set_mem(i + MEM_OFF, data[i + 3]);
            self.set_mem(i + 1 + MEM_OFF, data[i + 2]);
            self.set_mem(i + 2 + MEM_OFF, data[i + 1]);
            self.set_mem(i + 3 + MEM_OFF, data[i]);
        }
    }

    pub fn load_seg(
        &mut self,
        data: &Vec<u8>,
        seg_off: usize,
        seg_phys_addr: usize,
        seg_size: usize,
    ) {
        if self.memory.len() < seg_size {
            panic!("segment is big");
        }

        for i in 0..seg_size {
            self.set_mem(seg_phys_addr + i - MEM_OFF, data[seg_off + i]);
        }
    }

    pub fn load_byte(&self, addr: u64) -> u8 {
        self.get_mem(addr as usize)
    }

    pub fn load_hword(&self, begin: u64) -> u16 {
        let mut res: u16 = 0;
        for i in begin..begin + 2 {
            res <<= 8;
            res |= self.get_mem(i as usize) as u16;
        }
        res
    }

    pub fn load_word(&self, addr: u64) -> u32 {
        let mut res: u32 = 0;
        for i in addr..addr + 4 {
            res <<= 8;
            res |= self.get_mem(i as usize) as u32;
        }
        res
    }

    pub fn load_dword(&self, addr: u64) -> u64 {
        let mut res: u64 = 0;
        for i in addr..addr + 8 {
            res <<= 8;
            res |= self.get_mem(i as usize) as u64;
        }
        res
    }

    pub fn store_byte(&mut self, addr: u64, data: u8) {
        self.set_mem(addr as usize, data);
    }

    pub fn store_hword(&mut self, addr: u64, data: u16) {
        self.set_mem(addr as usize, (data & 0xF) as u8);
        let addr = addr + 1;
        self.set_mem(addr as usize, (data >> 8) as u8);
    }

    pub fn store_word(&mut self, mut addr: u64, data: u32) {
        self.set_mem(addr as usize, (data & 0xF) as u8);
        for _ in 0..3 {
            addr = addr + 1;
            self.set_mem(addr as usize, ((data >> 8) & 0xF) as u8);
        }
    }

    pub fn store_dword(&mut self, mut addr: u64, data: u64) {
        self.set_mem(addr as usize, (data & 0xF) as u8);
        for _ in 0..7 {
            addr = addr + 1;
            self.set_mem(addr as usize, ((data >> 8) & 0xF) as u8);
        }
    }
}

fn to_byte_array(data: String) -> Vec<u8> {
    if data.len() % 2 != 0 {
        panic!("data length is odd");
    }

    let mut chars = data.chars();
    let mut res = Vec::new();
    for _ in 0..data.len() / 2 {
        let hi = to_hex(chars.next().unwrap());
        let lo = to_hex(chars.next().unwrap());
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
