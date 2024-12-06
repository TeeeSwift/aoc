use d6::{parse_input, Guard};

fn main() {
    let v = parse_input();
    let mut guard: Guard = Guard::new(v);

    guard.patrol();

    for line in guard.grid.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    let path_len: usize = guard
        .grid
        .iter()
        .map(|row| row.iter().filter(|c| **c == 'X').count())
        .sum();

    println!("{}", path_len + 1);
}
