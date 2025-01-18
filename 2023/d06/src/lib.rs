use anyhow::Result;
use std::{fs::File, io::BufRead, io::BufReader};

pub fn parse_input(filename: &str) -> Result<Vec<(usize, usize)>> {
    let mut v: Vec<(usize, usize)> = vec![];

    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;

    for value in line.split_whitespace().skip(1) {
        v.push((value.parse()?, 0));
    }

    let line = lines.next().unwrap()?;

    for (index, value) in line.split_whitespace().enumerate().skip(1) {
        v[index - 1].1 = value.parse()?;
    }

    Ok(v)
}

pub fn parse_input2(filename: &str) -> Result<(usize, usize)> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    let time: usize = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .skip(1)
        .map(|x| x.into())
        .collect::<Vec<String>>()
        .join("")
        .parse()?;

    let benchmark: usize = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .skip(1)
        .map(|x| x.into())
        .collect::<Vec<String>>()
        .join("")
        .parse()?;

    Ok((time, benchmark))
}

pub fn earliest_win(time: usize, benchmark: usize) -> Option<usize> {
    (0..time).find(|&i| i * (time - i) > benchmark)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let mut races = parse_input("src/sample")?;
        let mut test_races: Vec<(usize, usize)> = vec![(7, 9), (15, 40), (30, 200)];
        test_races.sort();
        races.sort();

        assert_eq!(races, test_races);
        Ok(())
    }

    #[test]
    fn t_parse_input2() -> Result<()> {
        let race = parse_input2("src/sample")?;
        println!("{race:?}");
        assert_eq!(race, (71530, 940200));
        Ok(())
    }

    #[test]
    fn t_earliest_win() {
        let (time, benchmark) = (7, 9);
        let e = earliest_win(time, benchmark).unwrap();

        assert_eq!(e, 2);

        let combinations = time - e - e + 1;
        assert_eq!(combinations, 4);

        let (time, benchmark) = (15, 40);
        let e = earliest_win(time, benchmark).unwrap();
        println!("{e}");

        assert_eq!(e, 4);

        let combinations = time - e - e + 1;
        assert_eq!(combinations, 8);

        let (time, benchmark) = (30, 200);
        let e = earliest_win(time, benchmark).unwrap();
        println!("{e}");

        assert_eq!(e, 11);

        let combinations = time - e - e + 1;
        assert_eq!(combinations, 9);
    }
}
