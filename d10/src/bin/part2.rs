use std::{sync::Arc, thread};

use d10::{find_summits, parse_input};

fn main() {
    let (grid, starting_nodes) = parse_input();
    let a = Arc::new(grid);

    let mut handles = vec![];

    for node in starting_nodes.iter().cloned() {
        let b = a.clone();
        handles.push(thread::spawn(move || find_summits(node, b, false)))
    }

    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("{}", total);
}
