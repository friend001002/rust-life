use std::io;
use std::io::Write;

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

    // Base 1d array
    let mut grid_raw: Vec<bool> = vec![false; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array `&mut [&mut [_]]`
    let grid: &mut [&mut [bool]] = grid_base.as_mut_slice();

    let mut epoch: u128 = 0;

    loop {
        epoch += 1;

        for r in 0..height {
            for c in 0..width {
                print!("{} ", grid[r][c]);
            }
            println!();
        }
    }
}
