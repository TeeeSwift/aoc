use d24::*;

pub fn main() {
    let (mut wires, mut gates) = parse_input("src/input");

    while let Some(x) = gates.pop_front() {
        match x.execute(&mut wires.map) {
            Ok(_) => {}
            Err(_) => gates.push_back(x),
        }
    }

    let mut zs = wires
        .map
        .keys()
        .filter(|x| x.starts_with("z"))
        .collect::<Vec<&String>>();

    zs.sort();
    zs.reverse();

    let mut binary_string = String::new();

    for key in zs {
        let &a = wires.map.get(key).unwrap();
        binary_string = format!("{binary_string}{a}");
    }

    let decimal = u64::from_str_radix(&binary_string, 2).unwrap();
    println!("{decimal}");
}
