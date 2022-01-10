use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::bus::Bus;
use crate::cmd::Command;
use crate::cpu::Cpu;
use crate::dram::Dram;

pub struct Emulator {
    cpu: Cpu,
    entry_point: usize,
}

impl Emulator {
    pub fn new(cmd: Command) -> Emulator {
        let mut dram = Dram::new(cmd.mem_size.unwrap());
        let mut entry_point = 0;
        if let Some(in_f) = cmd.in_f.clone() {
            Emulator::load_file_to_dram(&mut dram, in_f);
        }
        if let Some(elf) = cmd.elf.clone() {
            entry_point = Emulator::load_elf_to_dram(&mut dram, elf);
        }

        let bus = Bus::new(dram);

        Emulator {
            cpu: Cpu::new(bus, cmd.mem_size.unwrap()),
            entry_point,
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
            Ok(_) => {
                dram.load(data.trim().to_string());
            }
            Err(_) => panic!("erorr input file read"),
        }
    }

    fn load_elf_to_dram(dram: &mut Dram, elf: String) -> usize {
        if !Path::new(&elf).exists() {
            panic!("file not found: {}", elf);
        }
        let mut f = OpenOptions::new().read(true).open(elf).unwrap();
        let mut data: Vec<u8> = Vec::new();
        match f.read_to_end(&mut data) {
            Ok(_) => {
                if is_not_elf(&data) {
                    panic!("invalid elf format");
                }
                let entry_point = get_entry_point(&data);
                load_program(dram, data);

                entry_point
            }
            Err(_) => panic!("erorr input file read"),
        }
    }
}

fn is_not_elf(data: &Vec<u8>) -> bool {
    if data[0] != 0x7F || data[1] != 'E' as u8 || data[2] != 'L' as u8 || data[3] != 'F' as u8 {
        return true;
    }
    false
}

fn get_entry_point(data: &Vec<u8>) -> usize {
    let mut addr: usize = 0;
    for i in 0..8 {
        addr |= (data[24 + i] as usize) << (8 * i);
    }
    addr
}

fn load_program(dram: &mut Dram, data: Vec<u8>) {
    let p_header_off: usize = get_p_header_off(&data);
    let p_header_num: usize = get_p_header_num(&data);
    let p_header_ent_size: usize = get_p_header_ent_size(&data);
}

fn get_p_header_off(data: &Vec<u8>) -> usize {
    0
}

fn get_p_header_num(data: &Vec<u8>) -> usize {
    0
}

fn get_p_header_ent_size(data: &Vec<u8>) -> usize {
    0
}
