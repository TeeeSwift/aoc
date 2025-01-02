use d23::*;

pub fn main() {
    let v = parse_input("src/input");
    let m = generate_map(v);
    let sets = find_sets(m);

    let count = sets
        .into_iter()
        .filter(|set| set.iter().any(|x| x.starts_with('t')))
        .count();

    println!("{count}");
}
