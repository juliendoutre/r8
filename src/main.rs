use r8;
use std::env;
use std::process;

fn main() {
    r8::Cpu::new();

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No rom path passed as argument, terminating program.");
        process::exit(0);
    }

    println!("{}", args[1]);
}
