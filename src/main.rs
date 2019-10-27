// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::*;
// use piston::input::*;
// use piston::window::WindowSettings;
use r8::cpu;
use std::env;
use std::process;

fn main() {
    // let opengl = OpenGL::V3_2;

    // let mut window: Window = WindowSettings::new("", [64, 32])
    //     .graphics_api(opengl)
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();

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
