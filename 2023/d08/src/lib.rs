use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcm_of_list(numbers: &[usize]) -> usize {
    numbers.iter().copied().reduce(lcm).unwrap_or(1)
}

type LR = (String, String);

pub fn parse_input(filename: &str) -> Result<(Vec<char>, HashMap<String, LR>)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let instructions: String = lines.next().unwrap()?;
    let instructions = instructions.chars().collect::<Vec<char>>();
    lines.next();

    for line in lines {
        let line = line?;
        let segments = line
            .split_whitespace()
            .map(|sp| sp.into())
            .collect::<Vec<String>>();
        let key = segments[0].clone();

        let mut left = segments[2].clone();
        left.pop();
        left.remove(0);

        let mut right = segments[3].clone();
        right.pop();

        map.insert(key, (left, right));
    }

    Ok((instructions, map))
}

pub fn step(current: &mut String, instruction: &char, map: &HashMap<String, LR>) {
    let (left, right) = map.get(current).unwrap();
    match instruction {
        'L' => {
            *current = left.clone();
        }
        'R' => {
            *current = right.clone();
        }
        _ => {}
    };
}

pub fn simulate(start: String, instructions: &[char], map: &HashMap<String, LR>) -> Result<usize> {
    let mut count: usize = 0;
    let mut current: String = start.clone();

    while current.chars().nth(2).unwrap() != 'Z' {
        let inst = instructions[count % instructions.len()];
        step(&mut current, &inst, map);

        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let (a, b) = parse_input("src/sample")?;
        println!("{a:?}");
        println!("{b:?}");

        Ok(())
    }

    #[test]
    fn t_step() -> Result<()> {
        let (instructions, map) = parse_input("src/sample")?;
        let count = simulate("AAA".into(), &instructions, &map)?;

        assert_eq!(count, 2);

        let (instructions, map) = parse_input("src/sample2")?;
        let count = simulate("AAA".into(), &instructions, &map)?;
        assert_eq!(count, 6);
        Ok(())
    }

    #[test]
    fn t_parallell() -> Result<()> {
        let (instructions, map) = parse_input("src/sample3")?;
        let currents: Vec<String> = map
            .iter()
            .filter(|(k, _)| k.chars().nth(2).unwrap() == 'A')
            .map(|(k, _)| k.clone())
            .collect();

        println!("{currents:?}");

        let counts = currents
            .into_iter()
            .map(|start| simulate(start, &instructions, &map).unwrap())
            .collect::<Vec<usize>>();

        let sync = lcm_of_list(&counts);

        println!("{sync:?}");
        assert_eq!(sync, 6);

        Ok(())
    }
}
