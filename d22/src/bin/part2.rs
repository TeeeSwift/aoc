use std::collections::HashMap;

use d22::*;

pub fn main() -> Result<(), std::io::Error> {
    let starting_numbers = parse_input("src/input");

    let mut map: HashMap<Sequence, usize> = HashMap::new();

    for starting_number in starting_numbers {
        add_timeline(starting_number, 2000, &mut map);
    }

    if let Some((key, &value)) = map.iter().max_by_key(|&(_, &v)| v) {
        println!("Key with max value: {:?}, Value: {}", key, value);
    } else {
        println!("The map is empty");
    }

    Ok(())
}
