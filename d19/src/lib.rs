use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

pub fn parse_input() -> (Vec<String>, Vec<String>) {
    let file = std::fs::File::open("src/sample").unwrap();
    let file = std::fs::File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    let mut patterns: Vec<String> = vec![];
    let mut designs: Vec<String> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        match index {
            0 => {
                patterns = line
                    .split(",")
                    .map(|x| String::from(x.trim()))
                    .collect::<Vec<String>>();
            }
            1 => {}
            _ => {
                designs.push(line);
            }
        }
    }

    patterns.sort_by_key(|k| k.len());
    patterns.reverse();

    (patterns, designs)
}

pub fn design_possible(patterns: &Vec<String>, design: &str) -> bool {
    for pattern in patterns {
        match design.starts_with(pattern.as_str()) {
            true => {
                if design.len() > pattern.len() {
                    if design_possible(patterns, &design[pattern.len()..design.len()]) {
                        return true;
                    };
                } else {
                    return true;
                }
            }
            false => {}
        }
    }

    false
}

pub fn count_combinations(
    patterns: Arc<Vec<String>>,
    design: &str,
    map: Arc<Mutex<HashMap<String, usize>>>,
) -> usize {
    // see if its cached
    if let Ok(hashmap) = map.lock() {
        if let Some(&cached) = hashmap.get(design) {
            return cached;
        }
    }

    let mut valid: usize = 0;
    for pattern in patterns.iter() {
        if design.starts_with(pattern.as_str()) {
            if design.len() > pattern.len() {
                let remaining_design = &design[pattern.len()..];
                let possibilities =
                    count_combinations(Arc::clone(&patterns), remaining_design, Arc::clone(&map));
                valid += possibilities;
            } else {
                valid += 1;
            }
        }
    }

    // Store the result in the cache
    if let Ok(mut hashmap) = map.lock() {
        hashmap.insert(design.to_string(), valid);
    }

    valid
}

#[test]
fn t_parse_input() {
    let (a, b) = parse_input();
    println!("{:?}", a);
    println!("{:?}", b);
}

#[test]
fn t_design_possible_1_layer() {
    let patterns = vec!["ab".to_string(), "cde".to_string()];
    let design = "abcde";
    let b = design_possible(&patterns, design);
    println!("{b}");

    let patterns = vec!["ab".to_string(), "cde".to_string()];
    let design = "abcdf";
    let b = design_possible(&patterns, design);
    println!("{b}");

    let patterns = vec!["ab".to_string(), "b".to_string(), "cde".to_string()];
    let design = "abbcde";
    let b = design_possible(&patterns, design);
    println!("{b}");
}

#[test]
fn hashmap() {
    let (patterns, designs) = parse_input();
    let design = &designs[0];
    let patterns = Arc::new(patterns);
    let hashmap: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let b = count_combinations(patterns, design, hashmap.clone());
    println!("{b}");
    println!("{hashmap:?}");
}
