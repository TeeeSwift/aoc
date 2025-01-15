use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

type Src = isize;
type Dst = isize;
type Rng = isize;
type Map = (Src, Dst, Rng);

pub fn parse_input(filename: &str) -> Result<(Vec<isize>, Vec<Vec<Map>>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;
    let seeds = line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    lines.next();

    let mut maps: Vec<Vec<(Dst, Src, Rng)>> = vec![];
    let mut _temp: Vec<(Dst, Src, Rng)> = vec![];

    for line in lines {
        let line = line?;

        if line.is_empty() {
            maps.push(_temp.clone());
            _temp.clear();
            continue;
        }

        if line.ends_with(':') {
            continue;
        }

        let mut sp = line.split_whitespace();
        let dst = sp.next().unwrap().parse::<isize>()?;
        let src = sp.next().unwrap().parse::<isize>()?;
        let rng = sp.next().unwrap().parse::<isize>()?;

        _temp.push((dst, src, rng));
    }

    maps.push(_temp.clone());

    Ok((seeds, maps))
}

pub fn convert_seed_ranges(seeds: Vec<(isize, isize)>, phase: &Vec<Map>) -> Vec<(isize, isize)> {
    let mut new: Vec<(isize, isize)> = vec![];
    let mut seeds: VecDeque<(isize, isize)> = VecDeque::from(seeds);

    'outer: while let Some((start, end)) = seeds.pop_front() {
        'inner: for &(dst, src, rng) in phase {
            // if seed range is within map range
            if start >= src && end <= src + rng {
                new.push((start - src + dst, end - src + dst));
                continue 'outer;
            }

            // no overlap
            if start >= src + rng || end <= src {
                continue 'inner;
            }

            // add overlap
            let overlap_start = start.max(src);
            let overlap_end = end.min(src + rng);

            seeds.push_back((overlap_start, overlap_end));

            // seed start before map start
            if start < src {
                seeds.push_back((start, src));
            };

            if end > src + rng {
                seeds.push_back((src + rng, end));
            }
            continue 'outer;
        }

        new.push((start, end));
        // if they don't overlap, just push seed to new vector
    }

    // returns new seeds

    new
}

pub fn multi_convert(num: isize, phase: &Vec<Map>) -> isize {
    let mut value: isize = num;

    for map in phase {
        if let Some(x) = convert(value, map) {
            value = x;
            break;
        }
    }

    value
}

pub fn convert(num: isize, map: &Map) -> Option<isize> {
    let (dst, src, rng) = map;
    let transformation = dst - src;
    if num - src < *rng && num - src >= 0 {
        return Some(num + transformation);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_parse_input() -> Result<()> {
        let (_, _) = parse_input("src/sample")?;
        Ok(())
    }

    #[test]
    fn t_convert() {
        let phase: Vec<Map> = vec![(52, 50, 48), (50, 98, 2)];
        assert_eq!(multi_convert(79, &phase), 81);
        assert_eq!(multi_convert(14, &phase), 14);
        assert_eq!(multi_convert(55, &phase), 57);
        assert_eq!(multi_convert(13, &phase), 13);
    }

    #[test]
    fn t_full() -> Result<()> {
        let (seeds, phases) = parse_input("src/sample")?;
        let mut seed = seeds[1];
        for phase in &phases {
            seed = multi_convert(seed, phase);
            println!("{seed}");
        }

        Ok(())
    }

    #[test]
    fn t_convert_seed_range() {
        let phase: Vec<(isize, isize, isize)> = vec![(60, 50, 10)];
        let seeds: Vec<(isize, isize)> = vec![(80, 86)];

        let s: Vec<(isize, isize)> = convert_seed_ranges(seeds, &phase);
        assert_eq!(s, vec![(80, 86)]);

        let phase: Vec<(isize, isize, isize)> = vec![(60, 50, 10)];
        let seeds: Vec<(isize, isize)> = vec![(50, 56)];

        let s: Vec<(isize, isize)> = convert_seed_ranges(seeds, &phase);
        assert_eq!(s, vec![(60, 66)]);

        let phase: Vec<(isize, isize, isize)> = vec![(60, 50, 5)];
        // 50, 51, 52, 53, 54 -> 60, 61, 62, 63, 64

        let seeds: Vec<(isize, isize)> = vec![(45, 56)];
        // (45,50), (60, 65), (55, 56)

        let mut s: Vec<(isize, isize)> = convert_seed_ranges(seeds, &phase);
        s.sort();
        let mut a = vec![(45, 50), (60, 65), (55, 56)];
        a.sort();

        assert_eq!(s, a);
    }

    #[test]
    fn t_whole_thing() -> Result<()> {
        let (seeds, phases) = parse_input("src/sample")?;
        println!("{seeds:?}");

        let mut tuples: Vec<(isize, isize)> = vec![];

        let mut i = 0;
        while i < seeds.len() {
            tuples.push((seeds[i], seeds[i] + seeds[i + 1]));
            i += 2;
        }
        tuples.sort();

        println!("{tuples:?}");

        for phase in &phases {
            tuples = convert_seed_ranges(tuples, phase);
        }

        tuples.sort();
        println!("{tuples:?}");

        Ok(())
    }
}
