use std::env;
use crate::conf;

#[derive(Debug)]
pub struct Command {
    pub in_f: Option<String>,
    pub mem_size: Option<usize>,
}

impl Command {
    fn init() -> Command {
        Command {
            in_f: None,
            mem_size: Some(conf::MEMORY_SIZE),
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
}
