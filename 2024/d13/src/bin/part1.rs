use d13::parse_input;

pub fn main() {
    let machines = parse_input();
    let mut total: isize = 0;

    for (_, machine) in machines.iter() {
        println!("{:?}", machine);

        let valid_pairs: Vec<(isize, isize)> = machine.get_valid_pairs();
        println!("{:?}", valid_pairs);

        if valid_pairs.len() == 0 {
            continue;
        }

        let mut lowest: isize = isize::MAX;
        for (a, b) in valid_pairs {
            lowest = match machine.calculate_cost(a, b) < lowest {
                true => machine.calculate_cost(a, b),
                false => lowest,
            };
        }
        total += lowest;
    }

    println!("{}", total);
}
