use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub type Record = (String, Vec<usize>);

pub fn parse_input(filename: &str) -> Result<Vec<Record>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut v: Vec<Record> = vec![];

    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let string = iter.next().unwrap().to_string();

        let pattern = iter.next().unwrap().to_string();
        let groups: Vec<usize> = pattern
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        v.push((string, groups));
    }

    Ok(v)
}

pub fn count(
    string: &str,
    nums: &[usize],
    map: &mut HashMap<(String, Vec<usize>), usize>,
) -> Result<usize> {
    if string.is_empty() {
        if nums.is_empty() {
            return Ok(1);
        }
        return Ok(0);
    }

    if nums.is_empty() {
        if string.contains('#') {
            return Ok(0);
        }
        return Ok(1);
    }

    if map.get(&(string.into(), nums.to_vec())).is_some() {
        return Ok(*map.get(&(string.into(), nums.to_vec())).unwrap());
    };

    let mut result = 0;

    if ".?".contains(string.chars().next().unwrap()) {
        // pretend its a .
        result += count(&string[1..], nums, map)?
    };

    if "#?".contains(string.chars().next().unwrap()) {
        // pretend its a #
        // is it long enough
        // is there no . in the way
        // is it the end of the line or is the next thing a #
        if string.len() >= nums[0]
            && !string[..nums[0]].contains('.')
            && (string.len() == nums[0] || string.chars().nth(nums[0]).unwrap() != '#')
        {
            result += count(
                match string.len() == nums[0] {
                    false => &string[nums[0] + 1..],
                    true => "",
                },
                &nums[1..],
                map,
            )?;
        }
    };

    map.insert((string.into(), nums.to_vec()), result);

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_parse_input() {
        let results = parse_input("src/sample").unwrap();
        for result in results {
            println!("{result:?}");
        }
    }

    #[test]
    fn t_count() -> Result<()> {
        let mut map: HashMap<(String, Vec<usize>), usize> = HashMap::new();

        let a = count("????", &[1, 1], &mut map)?;
        println!("{a:?}");

        assert!(false);
        Ok(())
    }
}
