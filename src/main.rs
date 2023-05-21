use std::io;
use std::io::Write;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    width: usize,
    height: usize,
    epoch: u128,
    grid: &'a mut [&'a mut [bool]]/* = grid_base.as_mut_slice();*/,
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {

        for r in 0..self.height {
            for c in 0..self.width {
                print!("{} ", self.grid[r][c]);
            }
            println!();
        }

        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.epoch += 1;
    }
}

fn ask_for_size() -> (usize, usize) {
    let mut width_s: String = String::new();
    print!("Enter the board's width: ");
    io::stdout().flush().expect("Failed to flush stdio");
    io::stdin()
        .read_line(&mut width_s)
        .expect("Failed to read the number!");

    width_s = width_s.replace('\n', "");

    let mut height_s: String = String::new();
    print!("Enter the board's height: ");
    io::stdout().flush().expect("Failed to flush stdio");
    io::stdin()
        .read_line(&mut height_s)
        .expect("Failed to read the number!");

    height_s = height_s.replace('\n', "");

    println!("{}, {}", width_s, height_s);

    let width: usize = width_s.parse::<usize>().unwrap();
    let height: usize = height_s.parse::<usize>().unwrap();

    (width, height)
}

/*fn create_board(width, height) {

}*/

fn main() {
    println!("Hello, world!");

    let board_size: (usize, usize) = ask_for_size();
    //create_board()

    let (width, height) = board_size;

    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [width as u32, height as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut grid_raw: Vec<bool> = vec![false; width * height];
    let mut grid_base: Vec<&mut [bool]> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        width: width,
        height: height,
        epoch: 0,
        grid: grid_base.as_mut_slice()
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
            println!("Epoch: {}", app.epoch);
        }
    }
}
