use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::bus::Bus;
use crate::cmd::Command;
use crate::conf::MEM_OFF;
use crate::cpu::Cpu;
use crate::dram::Dram;
use crate::plic::Plic;
use crate::uart::Uart;
use crate::virtio::Virtio;

pub struct Emulator {
    cpu: Cpu,
    entry_point: usize,
}

impl Emulator {
    pub fn new(cmd: Command) -> Emulator {
        let mut dram = Dram::new(cmd.mem_size.unwrap());
        let mut entry_point = MEM_OFF;
        if let Some(in_f) = cmd.in_f.clone() {
            Emulator::load_file_to_dram(&mut dram, in_f);
        }
        if let Some(elf) = cmd.elf.clone() {
            entry_point = Emulator::load_elf_to_dram(&mut dram, elf);
        }

        let bus = Bus::new(dram, Uart::new(), Plic::new(), Virtio::new());

        Emulator {
            cpu: Cpu::new(bus, cmd.mem_size.unwrap(), cmd.dbg.clone()),
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
        self.cpu.init(self.entry_point);
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
                let entry_point = get_ltl(&data, 24, 8);
                load_program(dram, data);

                entry_point
            }
            Err(_) => panic!("erorr input file read"),
        }
    }
}

/// Returns the `size` byte from the` idx` byte of the `data`
/// as little endian.
/// If the `size` is 0, 0 is returned.
///
/// # Examples
/// ```ignore
/// let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
/// assert_eq!(get_ltl(&data, 1, 2), 0x5634);
/// assert_eq!(get_ltl(&data, 0, 1), 0x12);
/// assert_eq!(get_ltl(&data, 0, 0), 0x0);
/// assert_eq!(get_ltl(&data, 0, 4), 0x78563412);
/// ```
fn get_ltl(data: &Vec<u8>, idx: usize, size: usize) -> usize {
    let mut addr: usize = 0;
    for i in 0..size {
        addr |= (data[idx + i] as usize) << (8 * i);
    }
    addr
}

fn is_not_elf(data: &Vec<u8>) -> bool {
    if data[0] != 0x7F || data[1] != 'E' as u8 || data[2] != 'L' as u8 || data[3] != 'F' as u8 {
        return true;
    }
    false
}

const PROGRAM_HEADER_OFFSET: usize = 0x20;
const PROGRAM_HEADER_SIZE_OFFSET: usize = 0x36;
const PROGRAM_HEADER_NUM_OFFSET: usize = 0x38;
const SEGMENT_TYPE_LOAD: usize = 0x1;
// const SEGMENT_TYPE_GNU_STACK: usize = 0x6474E551;
const SEGMENT_OFFSET_OFFSET: usize = 0x8;
const SEGMENT_PHYS_ADDR_OFFSET: usize = 0x18;
const SEGMENT_SIZE_OFFSET: usize = 0x20;
fn load_program(dram: &mut Dram, data: Vec<u8>) {
    let ph_off = get_ltl(&data, PROGRAM_HEADER_OFFSET, 8);
    let ph_size = get_ltl(&data, PROGRAM_HEADER_SIZE_OFFSET, 2);
    let ph_num = get_ltl(&data, PROGRAM_HEADER_NUM_OFFSET, 2);
    // println!("ph_off: 0x{:016X}", ph_off);
    // println!("ph_size: 0x{:04X}", ph_size);
    // println!("ph_num: 0x{:04X}", ph_num);

    for i in 0..ph_num {
        let ph_addr = ph_off + (i * ph_size);
        let seg_type = get_ltl(&data, ph_addr, 4);
        if seg_type != SEGMENT_TYPE_LOAD {
            continue;
        }
        // println!("seg_type: 0x{:04X}", seg_type);
        let seg_off = get_ltl(&data, ph_addr + SEGMENT_OFFSET_OFFSET, 8);
        // println!("seg_off: 0x{:016X}", seg_off);
        let seg_phys_addr = get_ltl(&data, ph_addr + SEGMENT_PHYS_ADDR_OFFSET, 8);
        // println!("seg_phys_addr: 0x{:016X}", seg_phys_addr);
        let seg_size = get_ltl(&data, ph_addr + SEGMENT_SIZE_OFFSET, 8);
        // println!("seg_size: 0x{:016X}", seg_size);
        dram.load_seg(&data, seg_off, seg_phys_addr, seg_size);
    }
}
