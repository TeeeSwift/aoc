use d13::parse_input;

pub fn main() {
    let machines = parse_input();
    let mut total: isize = 0;

    for (_, machine) in machines.iter() {
        println!("{:?}", machine);

        // let (a, b): (usize, usize) = machine.get_button_presses();

        total += machine.reddit();
    }

    println!("{}", total);
}
