use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Error, Result};

pub fn parse_input(filename: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

pub fn find_s(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                return Some((y, x));
            }
        }
    }

    None
}

pub fn get_first(grid: &[Vec<char>], start: (usize, usize)) -> Result<(usize, usize)> {
    if start.0 > 0 && ['|', 'F', '7'].contains(&grid[start.0 - 1][start.1]) {
        return Ok((start.0 - 1, start.1));
    }

    if start.0 < grid.len() - 1 && ['|', 'L', 'J'].contains(&grid[start.0 + 1][start.1]) {
        return Ok((start.0 + 1, start.1));
    }

    if start.1 < grid[0].len() - 1 && ['-', '7', 'J'].contains(&grid[start.0][start.1 + 1]) {
        return Ok((start.0, start.1 + 1));
    }

    if ['-', 'F', 'L'].contains(&grid[start.0][start.1 - 1]) {
        return Ok((start.0, start.1 - 1));
    }

    Err(Error::msg("idk"))
}

pub fn get_loop(grid: &[Vec<char>]) -> Result<Vec<(usize, usize)>> {
    let mut path: Vec<(usize, usize)> = vec![];

    let start = find_s(grid).unwrap();
    let mut prev: (usize, usize) = start;
    let mut next = get_first(grid, start).unwrap();

    path.push(start);

    while next != start {
        path.push(next);
        match grid[next.0][next.1] {
            '|' => match prev == (next.0 - 1, next.1) {
                true => {
                    prev = next;
                    next = (next.0 + 1, next.1)
                }
                false => {
                    prev = next;
                    next = (next.0 - 1, next.1)
                }
            },
            '-' => match prev == (next.0, next.1 - 1) {
                true => {
                    prev = next;
                    next = (next.0, next.1 + 1)
                }
                false => {
                    prev = next;
                    next = (next.0, next.1 - 1)
                }
            },
            'L' => match prev == (next.0 - 1, next.1) {
                true => {
                    prev = next;
                    next = (next.0, next.1 + 1)
                }
                false => {
                    prev = next;
                    next = (next.0 - 1, next.1)
                }
            },
            'F' => match prev == (next.0 + 1, next.1) {
                true => {
                    prev = next;
                    next = (next.0, next.1 + 1)
                }
                false => {
                    prev = next;
                    next = (next.0 + 1, next.1)
                }
            },
            '7' => match prev == (next.0 + 1, next.1) {
                true => {
                    prev = next;
                    next = (next.0, next.1 - 1)
                }
                false => {
                    prev = next;
                    next = (next.0 + 1, next.1)
                }
            },
            'J' => match prev == (next.0, next.1 - 1) {
                true => {
                    prev = next;
                    next = (next.0 - 1, next.1)
                }
                false => {
                    prev = next;
                    next = (next.0, next.1 - 1)
                }
            },
            _ => {}
        }
    }

    Ok(path)
}

pub fn purge(
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    path: Vec<(usize, usize)>,
) -> Result<Vec<Vec<char>>> {
    let mut new_grid: Vec<Vec<char>> = vec![];

    for (y, row) in grid.iter().enumerate() {
        let mut new_row: Vec<char> = vec![];
        for (x, char) in row.iter().enumerate() {
            if path.contains(&(y, x)) {
                new_row.push(*char);
            } else {
                new_row.push('.');
            }
        }
        new_grid.push(new_row);
    }

    let up = start.0 > 0 && ['|', '7', 'F'].contains(&new_grid[start.0 - 1][start.1]);
    let down =
        start.0 < grid.len() - 1 && ['|', 'J', 'L'].contains(&new_grid[start.0 + 1][start.1]);

    let left = start.1 > 0 && ['-', 'L', 'F'].contains(&new_grid[start.0][start.1 - 1]);
    let right =
        start.1 < grid[0].len() - 1 && ['-', 'J', '7'].contains(&new_grid[start.0][start.1 + 1]);

    let replacement: char = match (up, right, down, left) {
        (true, true, _, _) => 'L',
        (false, true, true, false) => 'F',
        (false, false, true, true) => '7',
        (false, true, false, true) => '-',
        (true, false, true, false) => '|',
        (true, false, false, true) => 'J',
        _ => '.',
    };

    new_grid[start.0][start.1] = replacement;

    Ok(new_grid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let a = parse_input("src/sample")?;
        println!("{a:?}");

        let s = find_s(&a);
        println!("{s:?}");

        assert_eq!(s, Some((1, 1)));
        Ok(())
    }

    #[test]
    fn t_get_loop() -> Result<()> {
        let a = parse_input("src/sample")?;
        let l = get_loop(&a)?;
        assert_eq!(l.len() / 2, 4);

        let a = parse_input("src/sample2")?;
        let l = get_loop(&a)?;
        assert_eq!(l.len() / 2, 8);

        Ok(())
    }
    #[test]
    fn t_get_first() -> Result<()> {
        let cases: Vec<Vec<Vec<char>>> = vec![
            vec![vec!['|'], vec!['S']],
            vec![vec!['F'], vec!['S']],
            vec![vec!['7'], vec!['S']],
        ];

        assert!(cases.iter().all(|v| {
            let start = find_s(v).unwrap();
            get_first(v, start).unwrap() == (0, 0)
        }));

        let cases: Vec<Vec<Vec<char>>> = vec![
            vec![vec!['S'], vec!['|']],
            vec![vec!['S'], vec!['L']],
            vec![vec!['S'], vec!['J']],
        ];

        assert!(cases.iter().all(|v| {
            let start = find_s(v).unwrap();
            get_first(v, start).unwrap() == (1, 0)
        }));

        let cases: Vec<Vec<Vec<char>>> = vec![
            vec![vec!['S', '-']],
            vec![vec!['S', 'J']],
            vec![vec!['S', '7']],
        ];

        assert!(cases.iter().all(|v| {
            let start = find_s(v).unwrap();
            get_first(v, start).unwrap() == (0, 1)
        }));

        let cases: Vec<Vec<Vec<char>>> = vec![
            vec![vec!['-', 'S']],
            vec![vec!['F', 'S']],
            vec![vec!['L', 'S']],
        ];

        assert!(cases.iter().all(|v| {
            let start = find_s(v).unwrap();
            get_first(v, start).unwrap() == (0, 0)
        }));

        Ok(())
    }

    #[test]
    fn t_new_grid() -> Result<()> {
        let grid = parse_input("src/sample3")?;
        let start = find_s(&grid).unwrap();

        for row in &grid {
            let a = row.iter().collect::<String>();
            println!("{a}");
        }

        println!("---------------");

        let path = get_loop(&grid)?;
        let p = purge(grid, start, path)?;

        for line in p {
            let a = line.iter().collect::<String>();
            println!("{a}");
        }

        println!("---------------");

        Ok(())
    }
}
