use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_input(filename: &str) -> Vec<u64> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn produce_secret_number(n: u64) -> u64 {
    let mut result: u64 = n;

    let mut a = result * 64;
    result ^= a;
    result %= 16777216;

    a = result / 32;
    result ^= a;
    result %= 16777216;

    a = result * 2048;
    result ^= a;
    result %= 16777216;

    result
}

pub fn produce_n_secret_numbers(start: u64, n: u64) -> u64 {
    let mut result: u64 = start;

    for _ in 0..n {
        let mut a = result * 64;
        result ^= a;
        result %= 16777216;

        a = result / 32;
        result ^= a;
        result %= 16777216;

        a = result * 2048;
        result ^= a;
        result %= 16777216;
    }

    result
}

pub fn add_timeline(start_num: u64, n: usize, map: &mut HashMap<Sequence, usize>) {
    let mut secret_num: u64 = start_num;
    let mut deque: VecDeque<i64> = VecDeque::new();
    let mut last_price: u64 = 0;
    let mut local_map: HashSet<Sequence> = HashSet::new();

    for _ in 1..=n {
        let mut a = secret_num * 64;
        secret_num ^= a;
        secret_num %= 16777216;

        a = secret_num / 32;
        secret_num ^= a;
        secret_num %= 16777216;

        a = secret_num * 2048;
        secret_num ^= a;
        secret_num %= 16777216;

        let price = secret_num % 10;
        match deque.len() {
            0 => {
                deque.push_back(price as i64 - (start_num % 10) as i64);
            }
            1..=2 => {
                let diff: i64 = price as i64 - last_price as i64;
                deque.push_back(diff);
            }
            3 => {
                let diff: i64 = price as i64 - last_price as i64;
                deque.push_back(diff);
                let array: [i64; 4] = deque
                    .clone()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                if local_map.insert(array) {
                    map.entry(array)
                        .and_modify(|x| *x += price as usize)
                        .or_insert(price as usize);
                }
            }
            4 => {
                let diff: i64 = price as i64 - last_price as i64;
                deque.push_back(diff);
                deque.pop_front();
                let array: [i64; 4] = deque
                    .clone()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                if local_map.insert(array) {
                    map.entry(array)
                        .and_modify(|x| *x += price as usize)
                        .or_insert(price as usize);
                }
            }
            _ => {}
        };

        last_price = price;
    }
}

pub type Sequence = [i64; 4];

#[test]
fn t_produce_secret_number() {
    assert_eq!(produce_secret_number(123), 15887950);
}

#[test]
fn t_produce_n_secret_numbers() {
    assert_eq!(15887950, produce_n_secret_numbers(123, 1));
    assert_eq!(8685429, produce_n_secret_numbers(1, 2000));
    assert_eq!(4700978, produce_n_secret_numbers(10, 2000));
    assert_eq!(15273692, produce_n_secret_numbers(100, 2000));
    assert_eq!(8667524, produce_n_secret_numbers(2024, 2000));
}

#[test]
fn t_add_timeline() {
    let mut map: HashMap<Sequence, usize> = HashMap::new();
    add_timeline(1, 2000, &mut map);
    if let Some((key, &value)) = map.iter().max_by_key(|&(_, &v)| v) {
        println!("Key with max value: {:?}, Value: {}", key, value);
    }
    add_timeline(2, 2000, &mut map);
    if let Some((key, &value)) = map.iter().max_by_key(|&(_, &v)| v) {
        println!("Key with max value: {:?}, Value: {}", key, value);
    }
    add_timeline(3, 2000, &mut map);
    if let Some((key, &value)) = map.iter().max_by_key(|&(_, &v)| v) {
        println!("Key with max value: {:?}, Value: {}", key, value);
    }
    add_timeline(2024, 2000, &mut map);

    if let Some((key, &value)) = map.iter().max_by_key(|&(_, &v)| v) {
        println!("Key with max value: {:?}, Value: {}", key, value);
    } else {
        println!("The map is empty");
    }
}
