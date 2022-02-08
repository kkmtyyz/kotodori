#[derive(Debug, Clone)]
pub struct Debug {
    pub enable: bool,
    pub bp: u64,
}

impl Debug {
    pub fn new(enable: bool, addr: u64) -> Debug {
        Debug { enable, bp: addr }
    }

    pub fn is_bp(&mut self, addr: u64) -> bool {
        self.bp == addr
    }
}
