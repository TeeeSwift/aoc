use d24::*;
use gate::Gate;
use operation::Operation;

pub fn main() {
    let (mut wires, mut gates) = parse_input("src/input");
    let early_xors: Vec<Gate> = gates
        .iter()
        .filter(|g| {
            !(g.input1.starts_with('x')
                || g.input1.starts_with('y')
                || g.input2.starts_with('x')
                || g.input2.starts_with('y')
                || g.dest.starts_with('z'))
                && g.op == Operation::XOR
        })
        .cloned()
        .collect();

    println!("early_xors");
    for early_xor in early_xors {
        println!("{early_xor:?}");
    }

    println!("non-xor z outputs");
    gates
        .iter()
        .filter(|g| g.dest.starts_with('z') && g.op != Operation::XOR && g.dest != "z45")
        .for_each(|x| println!("{x:?}"));

    let p = gates.iter().position(|gate| gate.dest == "dsd").unwrap();
    let q = gates.iter().position(|gate| gate.dest == "z37").unwrap();
    gates.get_mut(p).unwrap().dest = "z37".to_string();
    gates.get_mut(q).unwrap().dest = "dsd".to_string();

    let p = gates.iter().position(|gate| gate.dest == "djg").unwrap();
    let q = gates.iter().position(|gate| gate.dest == "z12").unwrap();
    gates.get_mut(p).unwrap().dest = "z12".to_string();
    gates.get_mut(q).unwrap().dest = "djg".to_string();

    let p = gates.iter().position(|gate| gate.dest == "sbg").unwrap();
    let q = gates.iter().position(|gate| gate.dest == "z19").unwrap();
    gates.get_mut(p).unwrap().dest = "z19".to_string();
    gates.get_mut(q).unwrap().dest = "sbg".to_string();

    let p = gates.iter().position(|gate| gate.dest == "mcq").unwrap();
    let q = gates.iter().position(|gate| gate.dest == "hjm").unwrap();
    gates.get_mut(p).unwrap().dest = "hjm".to_string();
    gates.get_mut(q).unwrap().dest = "mcq".to_string();

    while let Some(x) = gates.pop_front() {
        match x.execute(&mut wires.map) {
            Ok(_) => {}
            Err(_) => gates.push_back(x),
        }
    }

    println!("i x y z");
    for i in 0..=44 {
        let &x = wires.map.get(&format!("x{i:02}")).unwrap();
        let &y = wires.map.get(&format!("y{i:02}")).unwrap();
        let &z = wires.map.get(&format!("z{i:02}")).unwrap();

        println!("{i} {x} {y} {z}");
    }
}
