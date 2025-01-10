use d03::{get_part_numbers, parse_input};

pub fn main() -> anyhow::Result<()> {
    let (grid, _) = parse_input("src/input")?;
    let (total, _) = get_part_numbers(grid)?;
    println!("{total}");
    Ok(())
}
