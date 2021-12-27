// $ cargo run --example load_file -- -f examples/test.dump

use kotodori::emulator::Emulator;
use kotodori::cmd::Command;

fn main() {

    let cmd = Command::get();
    let mut emu = Emulator::new(cmd);
    emu.exe();
    emu.print_dram(0, 64);
}
