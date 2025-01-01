use d21::keypad::*;
use d21::{extract_number, parse_input};

pub fn main() -> Result<(), std::io::Error> {
    let numpad = KeyPad::new("numpad");
    let mut arrowpad = KeyPad::new("arrows");

    let mut total = 0;

    let commands = parse_input("src/input").unwrap();

    for command in commands {
        let num: u64 = extract_number(command.as_str()).unwrap();

        let sequences = numpad.generate_sequences(&command);

        let shortest_length = sequences
            .into_iter()
            .map(|sequence| arrowpad.shortest_seq(sequence, 25))
            .min()
            .unwrap();

        println!("{} {} {}", command, num, shortest_length);
        total += num * shortest_length as u64;
    }

    println!("{total}");

    Ok(())
}
