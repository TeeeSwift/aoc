use d11::{parse_input, step};

fn main() {
    let mut hm1 = parse_input();

    let iterations: usize = 75;
    for _ in 0..iterations {
        hm1 = step(hm1);
    }

    let sum: u64 = hm1.iter().map(|(_, v)| *v).sum();
    println!("{}", sum);
}
