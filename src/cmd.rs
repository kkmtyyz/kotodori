use crate::conf;
use crate::dbg::Debug;
use crate::util;
use std::env;

#[derive(Debug)]
pub struct Command {
    pub in_f: Option<String>,
    pub mem_size: Option<usize>,
    pub elf: Option<String>,
    pub drive: Option<String>,
    pub dbg: Debug,
}

impl Command {
    fn init() -> Command {
        Command {
            in_f: None,
            mem_size: Some(conf::MEMORY_SIZE),
            elf: None,
            drive: None,
            dbg: Debug::new(false, 0),
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn get() -> Command {
        let mut args: Vec<String> = env::args().collect();
        args.reverse();
        args.pop(); // remove executable path

        let mut cmd = Command::init();
        loop {
            let arg = args.pop();
            if let None = arg {
                break;
            }
            match &*arg.unwrap() {
                "-f" => cmd.in_f = Command::get_arg_string(&mut args),
                "-m" => cmd.mem_size = Command::get_arg_usize(&mut args),
                "--elf" => cmd.elf = Command::get_arg_string(&mut args),
                "--drive" => cmd.drive = Command::get_arg_string(&mut args),
                "--debug" => cmd.dbg = Command::get_arg_debug(&mut args),
                _ => (),
            }
        }
        cmd
    }

    fn get_arg_string(args: &mut Vec<String>) -> Option<String> {
        match args.pop() {
            Some(v) => Some(String::from(v)),
            None => None,
        }
    }

    fn get_arg_usize(args: &mut Vec<String>) -> Option<usize> {
        match args.pop() {
            Some(v) => Some(String::from(v).parse::<usize>().unwrap()),
            None => None,
        }
    }

    fn get_arg_debug(args: &mut Vec<String>) -> Debug {
        match args.pop() {
            Some(v) => {
                let addr = util::hex_to_usize(&String::from(v));
                Debug::new(true, addr as u64)
            }
            None => Debug::new(true, 0),
        }
    }
}
