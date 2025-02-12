use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub fn parse_input(filename: &str) -> Result<Vec<Vec<String>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut v: Vec<Vec<String>> = vec![];
    let mut temp: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            v.push(temp.clone());
            temp.clear();
            continue;
        }

        temp.push(line);
    }

    v.push(temp);

    Ok(v)
}

pub fn check_symmetry(string: &str, index: usize) -> Result<bool> {
    let mut result: bool = false;

    let mut left = string.to_string();
    let right = left.split_off(index);
    left = left.chars().rev().collect::<String>();

    let (shorter, longer) = match left.len().cmp(&right.len()) {
        std::cmp::Ordering::Greater => (right, left),
        _ => (left, right),
    };

    if longer.starts_with(&shorter) {
        result = true;
    }

    Ok(result)
}

pub fn symmetry_errors(string: &str, index: usize) -> Result<usize> {
    let mut left = string.to_string();
    let right = left.split_off(index);
    left = left.chars().rev().collect::<String>();

    let (shorter, longer) = match left.len().cmp(&right.len()) {
        std::cmp::Ordering::Greater => (right, left),
        _ => (left, right),
    };

    if longer.starts_with(&shorter) {
        return Ok(0);
    }

    let mut count: usize = 0;

    for i in 0..shorter.len() {
        if shorter.chars().nth(i).unwrap() != longer.chars().nth(i).unwrap() {
            count += 1;
        }
    }

    Ok(count)
}

pub fn rotate_grid(mut grid: Vec<String>) -> Result<Vec<String>> {
    let mut new_grid: Vec<String> = vec![];

    while !grid[0].is_empty() {
        new_grid.push(
            grid.iter_mut()
                .map(|string| string.pop().unwrap())
                .collect::<String>(),
        );
    }

    Ok(new_grid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let a = parse_input("src/sample")?;
        for v in a {
            println!("{v:?}");
        }

        Ok(())
    }

    #[test]
    fn t_rotate_grid() -> Result<()> {
        let grid = vec!["ABC".to_string(), "ABC".to_string()];

        let a = rotate_grid(grid)?;
        assert!(a.contains(&"CC".to_string()));
        assert!(a.contains(&"BB".to_string()));
        assert!(a.contains(&"AA".to_string()));

        Ok(())
    }

    #[test]
    fn t_check_symmetry() -> Result<()> {
        let s = "ABCCBA".to_string();

        let r = check_symmetry(&s, 3)?;
        assert!(r);

        let r = check_symmetry(&s, 4)?;
        assert!(!r);

        Ok(())
    }

    #[test]
    fn t_symmetry_errors() -> Result<()> {
        let s = "#.....".to_string();
        let num = symmetry_errors(&s, 3)?;
        assert_eq!(num, 1);

        let s = "##....".to_string();
        let num = symmetry_errors(&s, 3)?;
        assert_eq!(num, 2);

        Ok(())
    }
}
