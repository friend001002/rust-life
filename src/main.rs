use std::io;
use std::io::Write;
use std::{thread, time};

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
    //rotation: f64,  // Rotation for the square.
    width: usize,
    height: usize,
    cell_size: f64,
    epoch: u128,
    grid: &'a mut [&'a mut [bool]]/* = grid_base.as_mut_slice();*/,
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const PADDING: f64 = 0.0;
        const START_X: f64 = 10.0;
        const START_Y: f64 = 10.0;

        let mut square: [f64; 4] = rectangle::square(START_X, START_Y, self.cell_size);
        //let rotation = self.rotation;
        //let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |context, gl| {
            clear(GREEN, gl);

            for row in 0..self.height {
                for col in 0..self.width {
                    //print!("{} ", self.grid[r][c]);

                    square = rectangle::square(START_X + col as f64 * self.cell_size + col as f64 * PADDING, START_Y + row as f64 * self.cell_size + row as f64 * PADDING, self.cell_size);

                    let transform = context
                        .transform
                        .trans(0.0, 0.0);
                        //.trans(x, y);
                        //.rot_rad(rotation)
                        //.trans(-25.0, -25.0);

                    if self.grid[row][col] {
                        rectangle(RED, square, transform, gl);
                    }
                    else {
                        rectangle(BLACK, square, transform, gl);
                    }
                }
                //println!();
            }
        });
    }

    fn count_neighbors(&mut self, row: usize, col:  usize) -> u32 {
        let mut neighbors: u32 = 0;

        if row > 0 {
            // Can look up.
            if self.grid[row - 1][col] {
                neighbors += 1;
            }

            if col > 0 {
                // Can look left.
                if self.grid[row - 1][col - 1] {
                    neighbors += 1;
                }
            }

            if col < self.width - 1 {
                // Can look right.
                if self.grid[row - 1][col + 1] {
                    neighbors += 1;
                }
            }
        }

        if row < self.height - 1 {
            // Can look down.
            if self.grid[row + 1][col] {
                neighbors += 1;
            }

            if col > 0 {
                // Can look left.
                if self.grid[row + 1][col - 1] {
                    neighbors += 1;
                }
            }

            if col < self.width - 1 {
                // Can look right.
                if self.grid[row + 1][col + 1] {
                    neighbors += 1;
                }
            }
        }

        if col > 0 {
            // Can look left.
            if self.grid[row][col - 1] {
                neighbors += 1;
            }
        }

        if col < self.width - 1 {
            // Can look right.
            if self.grid[row][col + 1] {
                neighbors += 1;
            }
        }

        neighbors
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
        self.epoch += 1;

        for row in 0..self.height {
            for col in 0..self.width {
                // We are looking at 8 neighbors closest to row,col
                // If row,col is true and
                //     2 or 3 neighbors are true, then row,col remains true.
                //     more that 3 neighbors are true, then row,col becomes false.
                //     less than 2 neighbors are true, then row,col becomes false.
                // If row,col is false and 3 neighbors are true, then row,col becomes true.

                let neighbors: u32 = self.count_neighbors(row, col);

                if self.grid[row][col] {
                    if neighbors > 3 || neighbors < 2 {
                        self.grid[row][col] = false;
                    }
                }
                else {
                    if neighbors == 3 {
                        self.grid[row][col] = true;
                    }
                }
            }
        }
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

    if width < 3 {
        panic!("Width must be >= 3!");
    }

    if height < 3 {
        panic!("Height must be >= 3!");
    }

    (width, height)
}

fn main() {
    let board_size: (usize, usize) = ask_for_size();

    let (width, height) = board_size;

    let opengl: OpenGL = OpenGL::V3_2;

    let cell_size: f64 = 5.0;
    let win_width: f64 = width as f64 * cell_size + 20.0;
    let win_height: f64 = height as f64 * cell_size + 20.0;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [win_width, win_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut grid_raw: Vec<bool> = vec![false; width * height];
    let mut grid_base: Vec<&mut [bool]> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid: &mut [&mut [bool]] = grid_base.as_mut_slice();

    grid[0][0] = true;
    grid[1][1] = true;
    grid[2][2] = true;
    grid[10][2] = true;
    grid[10][12] = true;
    grid[5][2] = true;
    grid[14][8] = true;
    grid[10][7] = true;
    grid[3][12] = true;

    let mut app = App {
        gl: GlGraphics::new(opengl),
        //rotation: 0.0,
        width: width,
        height: height,
        cell_size,
        epoch: 0,
        grid: grid
    };

    let mut events = Events::new(EventSettings::new());
    let sleep_dur_millis = time::Duration::from_millis(300);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            println!("Epoch: {}", app.epoch);
            app.update(&args);
            thread::sleep(sleep_dur_millis);
        }
    }
}
