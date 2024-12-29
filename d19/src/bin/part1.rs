use d19::*;

pub fn main() {
    let (patterns, designs) = parse_input();

    println!("{:?}", patterns);
    println!("{:?}", designs);

    let mut possible: usize = 0;

    for design in designs {
        println!("DOING {design}");
        if design_possible(&patterns, &design) {
            possible += 1;
        }
        println!("DONE {design}");
        println!("{possible}");
    }

    println!("{possible}");
}
