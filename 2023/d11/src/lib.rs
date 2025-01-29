use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub struct Grid(pub Vec<Vec<char>>);
pub struct Galaxies(pub Vec<(usize, usize)>);
type EmptyLine = Vec<usize>;
type EmptyRows = EmptyLine;
type EmptyColumns = EmptyLine;

pub fn parse_input(filename: &str) -> Result<(Grid, Galaxies)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut galaxies: Galaxies = Galaxies(vec![]);
    let mut grid = Grid(vec![]);

    for (row, line) in reader.lines().enumerate() {
        let mut v: Vec<char> = vec![];

        let line = line?;

        for (col, char) in line.chars().enumerate() {
            v.push(char);
            if char == '#' {
                galaxies.0.push((row, col));
            }
        }
        grid.0.push(v);
    }

    Ok((grid, galaxies))
}

pub fn get_empty_ranges(grid: &Grid) -> Result<(EmptyRows, EmptyColumns)> {
    let mut empty_rows: Vec<usize> = vec![];

    for (index, row) in grid.0.iter().enumerate() {
        if row.iter().all(|&el| el == '.') {
            empty_rows.push(index);
        }
    }

    let mut empty_columns: Vec<usize> = vec![];
    for i in 0..grid.0[0].len() {
        let values = grid.0.iter().map(|r| r[i]).collect::<Vec<char>>();
        if values.iter().all(|&ch| ch == '.') {
            empty_columns.push(i);
        }
    }

    Ok((empty_rows, empty_columns))
}

pub fn count_empty(start: usize, end: usize, empty_lines: &[usize]) -> Result<usize> {
    let mut count: usize = 0;
    let s = start.min(end);
    let e = start.max(end);

    for i in s + 1..e {
        if empty_lines.contains(&i) {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let (grid, galaxies) = parse_input("src/sample")?;

        println!("{:?}", galaxies.0);

        for line in grid.0 {
            println!("{}", line.into_iter().collect::<String>());
        }

        Ok(())
    }

    #[test]
    fn t_get_empty() -> Result<()> {
        let (grid, _) = parse_input("src/sample")?;
        let (er, ec) = get_empty_ranges(&grid)?;

        assert_eq!(er, vec![3, 7]);
        assert_eq!(ec, vec![2, 5, 8]);

        Ok(())
    }

    #[test]
    fn t_count_empty() -> Result<()> {
        let g1 = (0, 0);
        let g2 = (6, 6);

        let er = vec![2, 5];

        let empty = count_empty(g1.0, g2.0, &er)?;

        assert_eq!(empty, 2);

        Ok(())
    }
}
