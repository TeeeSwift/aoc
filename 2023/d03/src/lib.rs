use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

type Grid = Vec<Vec<char>>;
type Coordinate = (usize, usize);

pub fn parse_input(filename: &str) -> Result<(Grid, Vec<(usize, usize)>)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Grid = vec![];
    let mut v: Vec<(usize, usize)> = vec![];

    for (row, line) in reader.lines().enumerate() {
        let l = line?;
        for (col, char) in l.chars().enumerate() {
            if char == '*' {
                v.push((row, col));
            }
        }

        grid.push(l.chars().collect());
    }

    Ok((grid, v))
}

pub fn get_part_numbers(grid: Grid) -> Result<(usize, HashMap<Coordinate, Vec<usize>>)> {
    let mut total: usize = 0;
    let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (row, line) in grid.iter().enumerate() {
        let mut part_number_buffer = String::new();
        let mut part_number_start_index: usize = 0;

        for (col, &char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                if part_number_buffer.is_empty() {
                    part_number_start_index = col;
                };

                part_number_buffer.push(char);
            }

            if !char.is_ascii_digit() && !part_number_buffer.is_empty() {
                let neighbors: Vec<(usize, usize)> = neighbors(
                    (row, part_number_start_index),
                    part_number_buffer.len(),
                    &grid,
                );

                let mut touching_symbol = false;

                for (y, x) in neighbors {
                    let a = grid[y][x];
                    if a == '.' || a.is_ascii_digit() {
                        continue;
                    }
                    touching_symbol = true;
                    map.entry((y, x))
                        .and_modify(|v| v.push(part_number_buffer.parse().unwrap()))
                        .or_insert(vec![part_number_buffer.parse().unwrap()]);
                }

                if touching_symbol {
                    total += part_number_buffer.parse::<usize>().unwrap();
                }

                part_number_buffer = String::new();
                part_number_start_index = 0;
            }
        }

        if !part_number_buffer.is_empty() {
            let neighbors: Vec<(usize, usize)> = neighbors(
                (row, part_number_start_index),
                part_number_buffer.len(),
                &grid,
            );

            let mut touching_symbol = false;

            for (y, x) in neighbors {
                let a = grid[y][x];
                if a == '.' || a.is_ascii_digit() {
                    continue;
                }
                touching_symbol = true;
                map.entry((y, x))
                    .and_modify(|v| v.push(part_number_buffer.parse().unwrap()))
                    .or_insert(vec![part_number_buffer.parse().unwrap()]);
            }
            if touching_symbol {
                total += part_number_buffer.parse::<usize>().unwrap();
            }
        }
    }

    // return part numbers
    Ok((total, map))
}

pub fn neighbors(start: (usize, usize), len: usize, grid: &Grid) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = vec![];
    let row = start.0 as isize;
    let start_col = start.1 as isize;

    // get left
    for (dy, dx) in [(-1, -1), (0, -1), (1, -1)] {
        // do math
        let y = row + dy;
        let x = start_col + dx;
        // see if it's in range
        if y < 0 || x < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize {
            continue;
        }
        // if it is, push it to v
        v.push((y as usize, x as usize));
    }

    // get right
    for (dy, dx) in [(-1, 1), (0, 1), (1, 1)] {
        // do math
        let y = row + dy;
        let x = start_col + len as isize - 1 + dx;
        // see if it's in range
        if y < 0 || x < 0 || y >= grid.len() as isize || x >= grid[0].len() as isize {
            continue;
        }
        // if it is, push it to v
        v.push((y as usize, x as usize));
    }

    // get up and down
    for (dy, _) in [(-1, 0), (1, 0)] {
        for col in start_col as usize..start_col as usize + len {
            let y = row + dy;

            if y < 0 || y >= grid.len() as isize {
                continue;
            }
            v.push((y as usize, col));
        }
    }

    v
}

#[test]
fn t_parse_input() -> Result<()> {
    let a = parse_input("src/sample")?;
    println!("{a:?}");
    Ok(())
}

#[test]
fn t_neighbors() -> Result<()> {
    let grid: Grid = vec![
        vec!['a', 'b', 'c', 'd'],
        vec!['e', 'f', 'g', 'h'],
        vec!['i', 'j', 'k', 'l'],
    ];

    let v = neighbors((1, 1), 2, &grid);
    assert_eq!(
        v,
        [
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 3),
            (1, 3),
            (2, 3),
            (0, 1),
            (0, 2),
            (2, 1),
            (2, 2)
        ]
    );

    let v = neighbors((0, 0), 1, &grid);
    assert_eq!(v, [(0, 1), (1, 1), (1, 0)]);

    Ok(())
}

#[test]
fn t_get_part_numbers() -> Result<()> {
    let grid: Grid = vec![
        vec!['1', '2', '.', '.'],
        vec!['.', '#', '5', '.'],
        vec!['.', '3', '.', '.'],
    ];

    let (a, b) = get_part_numbers(grid)?;
    println!("{a:?}");
    println!("{b:?}");

    let (grid, y) = parse_input("src/sample")?;
    println!("{y:?}");

    let (a, b) = get_part_numbers(grid)?;
    println!("{a:?}");
    println!("{b:?}");
    Ok(())
}
