use piston_window::*;
use r8::cpu;
use std::env;
use std::process;

const PIXEL_SIZE: usize = 10;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "r8",
        [
            (cpu::SCREEN_WIDTH * PIXEL_SIZE) as u32,
            (cpu::SCREEN_HEIGHT * PIXEL_SIZE) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut vm = cpu::Cpu::new();

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("missing argument: path to a rom file");
        process::exit(0);
    }

    let path: &str = &args[1];
    vm.load(path);

    let mut refresh_counter = 0;

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            if refresh_counter == 3 {
                window.draw_2d(&e, |c, g, _device| {
                    clear(color::BLACK, g);

                    for i in 0..cpu::SCREEN_WIDTH {
                        for j in 0..cpu::SCREEN_HEIGHT {
                            let mut color = color::BLACK;

                            if vm.screen[i][j] {
                                color = graphics::color::WHITE;
                            }

                            rectangle(
                                color,
                                [
                                    (i * PIXEL_SIZE) as f64,
                                    (j * PIXEL_SIZE) as f64,
                                    PIXEL_SIZE as f64,
                                    PIXEL_SIZE as f64,
                                ],
                                c.transform,
                                g,
                            );
                        }
                    }
                });
            }
        }

        if let Some(ref args) = e.press_args() {
            vm.press_key(args);
        }

        if let Some(ref args) = e.release_args() {
            vm.release_key(args);
        }

        vm.emulate();

        refresh_counter += 1;
        refresh_counter %= 4;
    }
}
