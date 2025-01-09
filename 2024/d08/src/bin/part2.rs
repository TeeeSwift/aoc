use d8::{find_antinodes, parse_input};

fn main() {
    let (grid, antennas) = parse_input();

    let mut antinodes: Vec<(i16, i16)> = vec![];

    for (_, nodes) in antennas.iter() {
        let mut local_antinodes: Vec<(i16, i16)> = vec![];
        for node1 in nodes.iter() {
            for node2 in nodes.iter() {
                local_antinodes.extend(find_antinodes(node1, node2, grid.clone()));
            }
        }

        antinodes.extend(local_antinodes);
    }

    antinodes.sort();
    antinodes.dedup();
    println!("{}", antinodes.len());
}
