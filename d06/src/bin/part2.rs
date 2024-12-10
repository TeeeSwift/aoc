use d6::guard::PatrolResult;
use d6::{parse_input, Guard};

use std::thread::spawn;

fn main() {
    let v = parse_input();

    let guard: Guard = Guard::new(v);
    let mut results: Vec<PatrolResult> = vec![];
    let mut handles = vec![];

    for (y, row) in guard.grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if guard.grid[y][x] == '^' {
                continue;
            };

            let mut new_guard = guard.clone();
            let handle = spawn(move || {
                new_guard.grid[y][x] = '#';
                new_guard.patrol()
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }

    let count = results
        .iter()
        .filter(|x| match **x {
            PatrolResult::LOOP => true,
            _ => false,
        })
        .count();

    println!("{}", count);
}
