use r8::cpu;
use std::env;
use std::process;

fn main() {
    let mut vm = cpu::Cpu::new();

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("missing argument: path to a rom file");
        process::exit(0);
    }

    let path: &str = &args[1];
    vm.load(path);

    loop {
        vm.emulate();
    }
}
