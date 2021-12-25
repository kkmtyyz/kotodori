use std::env;

use kotodori::emulator::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mem_size = 100_000;
    let emulator = Emulator::new(mem_size);
    // emulator.print_reg();
    emulator.print_dram();
}
