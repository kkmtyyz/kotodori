// $ cargo run --release -- --elf kernel/kernel --drive kernel/fs.img
// $ cargo run --release -- --elf kernel/kernel --debug

use kotodori::cmd::Command;
use kotodori::emulator::Emulator;

fn main() {
    let cmd = Command::get();
    let mut emu = Emulator::new(cmd);
    emu.exec();
}
