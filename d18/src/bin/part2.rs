use d18::*;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn main() {
    let (grid, original_corruptions) = parse_input();

    let len = original_corruptions.len();

    let mut handles: Vec<thread::JoinHandle<_>> = vec![];

    let arc = Arc::new(&original_corruptions);
    let lowest_failure_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));

    for i in 1..len {
        let mut grid = grid.clone();
        let lowest_failure_index = lowest_failure_index.clone();
        progress_grid(&mut grid, arc.clone(), i);
        handles.push(thread::spawn(move || {
            match find_shortest_path(&mut grid) {
                Ok(_) => {}
                Err(_) => {
                    let mut index = lowest_failure_index.lock().unwrap();
                    if *index > i {
                        *index = i;
                    }
                }
            };
        }));
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for each thread to finish
    }

    println!(
        "{:?}",
        original_corruptions[*lowest_failure_index.lock().unwrap()]
    )
}
