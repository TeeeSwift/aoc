use anyhow::Result;
use d10::*;

pub fn main() -> Result<()> {
    let grid = parse_input("src/input")?;
    let start = find_s(&grid).unwrap();
    let path = get_loop(&grid)?;
    let grid = purge(grid, start, path)?;

    for line in &grid {
        println!("{}", line.iter().collect::<String>());
    }

    let mut inside_count: usize = 0;

    for row in grid.iter() {
        let mut inside: bool = false;
        let mut last_bend: char = '.';

        for &char in row.iter() {
            if char == '.' {
                if inside {
                    inside_count += 1;
                }
                continue;
            }

            if char == '-' {
                continue;
            }

            if char == '|' {
                inside = !inside;
            }

            if ['J', 'L', 'F', '7'].contains(&char) {
                if last_bend == '.' {
                    last_bend = char;
                    continue;
                }
                match (last_bend, char) {
                    ('L', 'J') => {
                        last_bend = '.';
                        continue;
                    }
                    ('F', '7') => {
                        last_bend = '.';
                        continue;
                    }

                    _ => {
                        last_bend = '.';
                        inside = !inside;
                    }
                }
            }
        }
    }

    println!("{inside_count}");

    Ok(())
}
