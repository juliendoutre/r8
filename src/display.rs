use piston_window::color;
use piston_window::rectangle::Rectangle;
use piston_window::{clear, draw_state, Context, Graphics};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const PIXEL_SIZE: usize = 10;

pub struct Screen {
    pixels: [[Rectangle; SCREEN_HEIGHT]; SCREEN_WIDTH],
    updates: Vec<(usize, usize)>,
    clear: bool,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: [[Rectangle::new(color::BLACK); SCREEN_HEIGHT]; SCREEN_WIDTH],
            updates: Vec::<(usize, usize)>::new(),
            clear: false,
        }
    }

    pub fn clear(&mut self) {
        self.clear = true;
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                self.pixels[i][j].color = color::BLACK;
            }
        }
    }

    pub fn pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[x % SCREEN_WIDTH][y % SCREEN_HEIGHT].color == color::WHITE
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        self.updates.push((x, y));

        if self.pixel(x, y) {
            self.pixels[x % SCREEN_WIDTH][y % SCREEN_HEIGHT].color = color::BLACK;
        } else {
            self.pixels[x % SCREEN_WIDTH][y % SCREEN_HEIGHT].color = color::WHITE;
        }
    }

    pub fn render<T: Graphics>(&mut self, ctx: Context, graph: &mut T) {
        if self.clear {
            clear(color::BLACK, graph);
            self.clear = false;
        } else {
            self.updates.iter().for_each(|(i, j)| {
                self.pixels[*i][*j].draw(
                    [
                        (i * PIXEL_SIZE) as f64,
                        (j * PIXEL_SIZE) as f64,
                        PIXEL_SIZE as f64,
                        PIXEL_SIZE as f64,
                    ],
                    &draw_state::DrawState::default(),
                    ctx.transform,
                    graph,
                );
            });

            self.updates.clear();
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
