use d25::*;

pub fn main() {
    let (keys, locks) = parse_input("src/input");
    let mut combinations: usize = 0;

    for key in keys {
        combinations += locks.iter().filter(|&lock| fits(&key, lock)).count();
    }

    println!("{combinations}");
}
