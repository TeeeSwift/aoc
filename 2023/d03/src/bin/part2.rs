use d03::{get_part_numbers, parse_input};

pub fn main() -> anyhow::Result<()> {
    let mut total: usize = 0;

    let (grid, stars) = parse_input("src/input")?;
    let (_, map) = get_part_numbers(grid)?;
    for coordinate in stars {
        if let Some(x) = map.get(&coordinate) {
            if x.len() == 2 {
                total += x[0] * x[1];
            }
        }
    }

    println!("{total}");
    Ok(())
}
