use kotodori::cmd::Command;
use kotodori::emulator::Emulator;

fn main() {
    let cmd = Command::get();
    let mut emu = Emulator::new(cmd);
    //emu.print_cpu();
    //emu.print_dram();
    //emu.exec();
}
