use d23::*;

pub fn main() {
    let v = parse_input("src/input");
    let m = generate_map(v);

    let computers = m.keys().cloned().collect::<Vec<String>>();

    let mut longest: Vec<String> = vec![];

    for computer in computers {
        let mut a = find_largest(computer, &m);
        if a.len() > longest.len() {
            a.sort();
            longest = a;
        }
    }

    let b = longest.join(",");
    println!("{b:?}");
}
