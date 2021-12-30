// $ cargo run --example load_file -- -f examples/test.dump

use kotodori::cmd::Command;
use kotodori::emulator::Emulator;

fn main() {
    let cmd = Command::get();
    let mut emu = Emulator::new(cmd);
    emu.exec();
    emu.print_dram(0, 64);
}
