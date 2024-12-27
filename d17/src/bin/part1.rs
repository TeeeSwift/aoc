use d17::computer::Computer;
use d17::parse_input;

pub fn main() {
    let (a, b, c, commands) = parse_input();

    let mut computer = Computer::new(a, b, c);
    println!("{:?}", computer);
    computer.accept_commands(commands);

    println!("{:?}", computer);
    computer.print_output();
}
