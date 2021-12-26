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

    pub fn print(self) {
        print!("{:?}", self.memory);
    }

    pub fn load(&mut self, data: String) {
        if self.memory.len() < data.len() {
            panic!("data is big");
        }
        for (i, c) in data.chars().enumerate() {
            self.memory[i] = Dram::to_hex(c);
        }
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
            _ => panic!("Not hex: {}", c)
        }
    }
}
