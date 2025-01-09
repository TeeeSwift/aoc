use d21::keypad::*;
use d21::{extract_number, parse_input};

pub fn main() -> Result<(), std::io::Error> {
    let numpad = KeyPad::new("numpad");
    let arrowpad = KeyPad::new("arrows");
    let mut total = 0;

    let commands = parse_input("src/input").unwrap();

    for command in commands {
        let num: u64 = extract_number(command.as_str()).unwrap();

        let mut sequences = numpad.generate_sequences(&command);

        sequences = sequences
            .into_iter()
            .flat_map(|string| arrowpad.generate_sequences(&string))
            .collect::<Vec<String>>();

        sequences = sequences
            .into_iter()
            .flat_map(|string| arrowpad.generate_sequences(&string))
            .collect::<Vec<String>>();

        if let Some(shortest) = sequences.into_iter().min_by_key(|s| s.len()) {
            let l = shortest.len();
            println!("{} {} {}", command, l, num * l as u64);
            total += num * l as u64;
        }
    }

    println!("{total}");

    Ok(())
}
