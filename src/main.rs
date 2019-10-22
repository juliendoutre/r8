use r8;
use std::env;
use std::process;

fn main() {
    let mut vm = r8::Cpu::new();

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a ROM path to load.");
        process::exit(0);
    }

    let path: &str = &args[1];
    vm.load(path);

    loop {
        vm.emulate();
    }
}
