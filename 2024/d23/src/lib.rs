use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_input(filename: &str) -> Vec<Vec<String>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vector: Vec<Vec<String>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        vector.push(
            line.split('-')
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        )
    }

    vector
}

pub fn generate_map(pairs: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut computers: Vec<&String> = pairs.iter().flatten().collect();
    computers.sort();
    computers.dedup();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for computer in computers {
        pairs.iter().for_each(|pair| {
            if pair[0] == *computer {
                map.entry(computer.clone())
                    .and_modify(|e| e.push(pair[1].clone()))
                    .or_insert(vec![pair[1].clone()]);
            }
            if pair[1] == *computer {
                map.entry(computer.clone())
                    .and_modify(|e| e.push(pair[0].clone()))
                    .or_insert(vec![pair[0].clone()]);
            }
        });
    }

    map
}

pub fn find_largest(start: String, map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let computers = map.keys().cloned().collect::<Vec<String>>();
    let mut in_network: Vec<String> = vec![start];

    for computer in &computers {
        if in_network
            .iter()
            .all(|connection| map.get(connection).unwrap().contains(computer))
        {
            in_network.push(computer.to_string());
        }
    }

    in_network
}

pub fn find_sets(map: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>> = vec![];

    for start in map.keys() {
        let first_degree_connections = map.get(start).unwrap();
        // println!("{start} {first_degree_connections:?}");
        for first in first_degree_connections {
            let second_degree_connections = map.get(first).unwrap();
            // println!("{start} {first} {second_degree_connections:?}");
            for second in second_degree_connections {
                let third_degree_connections = map.get(second).unwrap();
                // println!("{start} {first} {second} {third_degree_connections:?}");
                if third_degree_connections.contains(start) {
                    let mut a = vec![start.clone(), first.clone(), second.clone()];
                    a.sort();
                    v.push(a);
                }
            }
        }
    }

    v.sort();
    v.dedup();
    v
}

#[test]
fn t_parse_input() {
    let a = parse_input("src/sample");
    println!("{a:?}");
}

#[test]
fn t_generate_map() {
    let v = parse_input("src/sample");
    let m = generate_map(v);
    println!("{m:?}");
}

#[test]
fn t_filter_sets() {
    let v = parse_input("src/sample");
    let m = generate_map(v);
    let sets = find_sets(m);
    println!("{sets:?}");

    let count = sets
        .into_iter()
        .filter(|set| set.iter().any(|x| x.starts_with('t')))
        .count();

    println!("{count}");
}

#[test]
fn t_find_sets() {
    let v = parse_input("src/sample");
    let m = generate_map(v);
    let sets = find_sets(m);
    println!("{sets:?}");
}

#[test]
fn t_find_longest() {
    let v = parse_input("src/sample");
    let m = generate_map(v);
    let longest = find_largest("co".to_string(), m);
    println!("{longest:?}");
}
