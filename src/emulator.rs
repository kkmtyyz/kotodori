use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::bus::Bus;
use crate::cmd::Command;
use crate::cpu::Cpu;
use crate::dram::Dram;

pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    pub fn new(cmd: Command) -> Emulator {
        let mut dram = Dram::new(cmd.mem_size.unwrap());
        if let Some(in_f) = cmd.in_f.clone() {
            Emulator::load_file_to_dram(&mut dram, in_f);
        }

        let bus = Bus::new(dram);

        Emulator {
            cpu: Cpu::new(bus, cmd.mem_size.unwrap()),
        }
    }

    pub fn print_cpu(&self) {
        self.cpu.print();
    }

    pub fn print_dram(&self, begin: usize, end: usize) {
        self.cpu.pdram_range(begin, end);
    }

    pub fn exec(&mut self) {
        self.cpu.init();
        self.cpu.run();
    }

    fn load_file_to_dram(dram: &mut Dram, in_f: String) {
        if !Path::new(&in_f).exists() {
            panic!("file not found: {}", in_f);
        }
        let mut f = OpenOptions::new().read(true).open(in_f).unwrap();
        let mut data = String::new();
        match f.read_to_string(&mut data) {
            Ok(_) => dram.load(data.trim().to_string()),
            Err(_) => panic!("erorr input file read"),
        }
    }
}
