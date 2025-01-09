use d17::computer::Computer;
use d17::parse_input;

pub fn main() {
    let (_, _, _, commands) = parse_input();
    let num = Computer::duplicate(commands);
    println!("{:?}", num);
}
