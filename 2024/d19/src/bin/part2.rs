use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use d19::*;

pub fn main() {
    let (patterns, designs) = parse_input();

    let patterns_arc = Arc::new(patterns);
    let hashmap_arc: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut threads: Vec<thread::JoinHandle<usize>> = vec![];

    for design in designs.into_iter() {
        let patterns_clone = Arc::clone(&patterns_arc);
        let hashmap_clone = Arc::clone(&hashmap_arc);
        threads.push(thread::spawn(move || {
            count_combinations(patterns_clone, &design, hashmap_clone)
        }));
    }

    let mut results = vec![];

    for thread in threads {
        let r = thread.join().unwrap();
        results.push(r);
    }

    let sum: usize = results.iter().sum();
    println!("{sum:?}");
}
