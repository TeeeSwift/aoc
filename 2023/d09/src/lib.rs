use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

type Dataset = Vec<isize>;

pub fn parse_input(filename: &str) -> Result<Vec<Dataset>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Dataset>()
        })
        .collect::<Vec<Dataset>>())
}

pub fn diff_once(dataset: &Dataset) -> Result<Dataset> {
    let mut v: Dataset = vec![];

    for i in 0..dataset.len() - 1 {
        v.push(dataset[i + 1] - dataset[i])
    }

    Ok(v)
}

pub fn diff_many(dataset: &Dataset) -> Result<Vec<Dataset>> {
    let mut v: Vec<Dataset> = vec![dataset.clone()];
    let mut done: bool = false;

    while !done {
        let new = diff_once(v.last().unwrap())?;
        done = new.iter().all(|&x| x == 0);
        v.push(new);
    }

    Ok(v)
}

pub fn find_next(mut datasets: Vec<Dataset>) -> Result<isize> {
    let mut placeholder: isize = 0;

    while let Some(n) = datasets.pop() {
        placeholder += *n.last().unwrap();
    }

    Ok(placeholder)
}

pub fn find_prev(mut datasets: Vec<Dataset>) -> Result<isize> {
    let mut placeholder: isize = 0;

    while let Some(x) = datasets.pop() {
        let prev_num = x[0];
        placeholder = prev_num - placeholder;
    }

    Ok(placeholder)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let a = parse_input("src/sample")?;
        println!("{a:?}");

        Ok(())
    }

    #[test]
    fn t_diff_once() -> Result<()> {
        let a = parse_input("src/sample")?;
        let b = diff_once(&a[0])?;
        let c: Vec<isize> = vec![3, 3, 3, 3, 3];
        assert_eq!(b, c);

        let b = diff_once(&a[1])?;
        let c: Vec<isize> = vec![2, 3, 4, 5, 6];
        assert_eq!(b, c);

        Ok(())
    }

    #[test]
    fn t_diff_many() -> Result<()> {
        let a = parse_input("src/sample")?;
        let b = diff_many(&a[1])?;
        let c: Vec<Vec<isize>> = vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
            vec![0, 0, 0],
        ];

        assert_eq!(b, c);
        Ok(())
    }

    #[test]
    fn t_find_next() -> Result<()> {
        let a = parse_input("src/sample")?;
        let datasets = diff_many(&a[1])?;

        let next = find_next(datasets)?;

        assert_eq!(next, 28);
        Ok(())
    }

    #[test]
    fn t_find_prev() -> Result<()> {
        let a = parse_input("src/sample")?;
        let datasets = diff_many(&a[2])?;

        let prev = find_prev(datasets)?;

        assert_eq!(prev, 5);
        Ok(())
    }
}
