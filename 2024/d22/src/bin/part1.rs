use d22::*;

pub fn main() -> Result<(), std::io::Error> {
    let starting_numbers = parse_input("src/input");

    let sum: u64 = starting_numbers
        .into_iter()
        .map(|x| produce_n_secret_numbers(x, 2000))
        .sum();

    println!("{sum}");

    Ok(())
}
