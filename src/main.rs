// $ cargo run --release -- --elf kernel/kernel

use kotodori::cmd::Command;
use kotodori::emulator::Emulator;

fn main() {
    let cmd = Command::get();
    let mut emu = Emulator::new(cmd);
    emu.exec();
}
