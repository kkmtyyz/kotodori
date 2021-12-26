use kotodori::emulator::Emulator;
use kotodori::cmd::Command;

fn main() {

    let cmd = Command::get();
    let mut emu = Emulator::new(cmd);
    // emu.print_reg();
    // emu.print_dram();
    emu.exe();
}
