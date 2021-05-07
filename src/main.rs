use piston_window::{PistonWindow, PressEvent, ReleaseEvent, RenderEvent, WindowSettings};
use r8::cpu::Cpu;
use r8::display::{PIXEL_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a path to a ROM file.");
        return;
    }

    let mut vm = Cpu::new(4 * 60);

    let path: &str = &args[1];
    vm.load(path);

    let mut window: PistonWindow = WindowSettings::new(
        "r8",
        [
            (SCREEN_WIDTH * PIXEL_SIZE) as u32,
            (SCREEN_HEIGHT * PIXEL_SIZE) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    main_loop(&mut vm, &mut window);
}

fn main_loop(vm: &mut Cpu, window: &mut PistonWindow) {
    let mut refresh_counter = 0;

    while let Some(e) = window.next() {
        vm.emulate();

        if e.render_args().is_some() {
            if refresh_counter == 3 {
                window.draw_2d(&e, |ctx, graph, _| vm.screen.render(ctx, graph));
            }

            refresh_counter = (refresh_counter + 1) % 4;
        } else if let Some(ref args) = e.press_args() {
            vm.inputs.press(args);
        } else if let Some(ref args) = e.release_args() {
            vm.inputs.release(args);
        }
    }
}
