use anyhow::Result;
use d11::{count_empty, get_empty_ranges, parse_input};

pub fn main() -> Result<()> {
    let (grid, mut galaxies) = parse_input("src/input")?;
    let (empty_rows, empty_columns) = get_empty_ranges(&grid)?;
    let mut total: isize = 0;

    while let Some(start) = galaxies.0.pop() {
        // println!("start: {start:?}");

        for i in 0..galaxies.0.len() {
            let end = galaxies.0[i];
            /* println!("end: {end:?}"); */

            let dy = (start.1 as isize - end.1 as isize).abs();
            let dx = (start.0 as isize - end.0 as isize).abs();

            let ey = count_empty(start.0, end.0, &empty_rows)?;
            let ex = count_empty(start.1, end.1, &empty_columns)?;

            // let sum: isize = [dy, dx, ey * 99as isize , ex as isize].iter().sum();

            // let sum: isize = [dy, dx, (ey * 99) as isize, (ex * 99) as isize]
            //     .iter()
            //     .sum();

            let sum: isize = [dy, dx, (ey * 999999) as isize, (ex * 999999) as isize]
                .iter()
                .sum();

            /* println!("{dy} {dx} {ey} {ex}. total: {sum}"); */
            total += sum;
        }
    }

    println!("{total}");

    Ok(())
}
