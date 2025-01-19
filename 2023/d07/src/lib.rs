use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub mod hand;
use hand::*;

pub fn parse_input(filename: &str) -> Result<Vec<Hand>> {
    let mut v = vec![];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut sp = line.split_whitespace();
        v.push(Hand::new(
            sp.next().unwrap().chars().collect(),
            sp.next().unwrap().parse()?,
        ))
    }

    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_rank() -> Result<()> {
        let mut hands = parse_input("src/sample")?;
        hands.iter_mut().for_each(|h| h.get_rank());

        Ok(())
    }
}
