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
}
