use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub winners: Vec<usize>,
    pub numbers: Vec<usize>,
}

pub fn parse_input(filename: &str) -> Result<Vec<Card>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut v: Vec<Card> = vec![];

    for (mut id, line) in reader.lines().enumerate() {
        id += 1;

        let mut winners: Vec<usize> = vec![];
        let mut numbers: Vec<usize> = vec![];

        let line = line?;
        let mut iter = line.split_whitespace();
        iter.next();
        iter.next();

        let mut add_to_winners: bool = true;

        for x in iter {
            if x == "|" {
                add_to_winners = !add_to_winners;
                continue;
            }

            if add_to_winners {
                winners.push(x.parse().unwrap());
                continue;
            }

            numbers.push(x.parse().unwrap());
        }

        v.push(Card {
            id,
            winners,
            numbers,
        });
    }

    Ok(v)
}

pub fn calculate(card: Card) -> Result<usize> {
    let Card {
        winners,
        numbers,
        id: _,
    } = card;

    let mut count: usize = 0;
    for number in numbers {
        if winners.contains(&number) {
            count += 1;
        }
    }

    if count == 0 {
        return Ok(0);
    }

    Ok(2usize.pow(count as u32 - 1))
}

pub fn matches(card: Card) -> Result<(usize, usize)> {
    let Card {
        winners,
        numbers,
        id,
    } = card;

    let mut count: usize = 0;
    for number in numbers {
        if winners.contains(&number) {
            count += 1;
        }
    }

    Ok((id, count))
}

#[test]
fn t_parse_input() -> Result<()> {
    let a = parse_input("src/sample")?;
    println!("{a:?}");

    Ok(())
}

#[test]
fn t_calculate() -> Result<()> {
    let a = parse_input("src/sample")?;
    let mut total: usize = 0;
    for card in a {
        let b = calculate(card)?;
        total += b;
    }

    println!("{total}");

    Ok(())
}
