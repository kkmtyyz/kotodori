use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::cpu::Register;
use crate::dram::Dram;
use crate::cmd::Command;

pub struct Emulator {
    reg: Register,
    dram: Dram,
    cmd: Command,
}

impl Emulator {
    pub fn new(cmd: Command) -> Emulator {
        Emulator {
            reg: Register::new(),
            dram: Dram::new(cmd.mem_size.unwrap()),
            cmd
        }
    }

    pub fn print_reg(&self) {
        print!("{:#?}", self.reg);
    }

    pub fn print_dram(&self, begin: usize, end: usize) {
        self.dram.prange(begin, end);
    }

    pub fn exe(&mut self) {
        if let Some(in_f) = self.cmd.in_f.clone() {
            &mut self.load_file_to_dram(in_f);
        }
    }

    fn load_file_to_dram(&mut self, in_f: String) {
        if !Path::new(&in_f).exists() {
            panic!("file not found: {}", in_f);
        }
        let mut f = OpenOptions::new().read(true).open(in_f).unwrap();
        let mut data = String::new();
        match f.read_to_string(&mut data) {
            Ok(_) => self.dram.load(data.trim().to_string()),
            Err(_) => panic!("erorr input file read"),
        }
    }
}
