use anyhow::Result;
use d13::*;

pub fn main() -> Result<()> {
    let grids: Vec<Vec<String>> = parse_input("src/input")?;
    let mut total: usize = 0;

    'grid_loop: for mut grid in grids {
        for i in 1..grid[0].len() {
            if grid.iter().all(|string| check_symmetry(string, i).unwrap()) {
                total += i;
                continue 'grid_loop;
            }
        }

        grid = rotate_grid(grid)?;

        for i in 1..grid[0].len() {
            if grid.iter().all(|string| check_symmetry(string, i).unwrap()) {
                total += i * 100;
                continue 'grid_loop;
            }
        }
    }

    println!("{total}");

    Ok(())
}
