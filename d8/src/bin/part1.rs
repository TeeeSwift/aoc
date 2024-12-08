use d8::{find_antinode, parse_input};

fn main() {
    let (grid, antennas) = parse_input();

    let mut antinodes: Vec<(i16, i16)> = vec![];

    for (_, nodes) in antennas.iter() {
        let mut local_antinodes: Vec<(i16, i16)> = vec![];
        for node1 in nodes.iter() {
            for node2 in nodes.iter() {
                match find_antinode(node1, node2) {
                    Some(x) => local_antinodes.push(x),
                    None => {}
                };
            }
        }

        local_antinodes.retain(|e| {
            e.0 >= 0 && e.0 < grid.len() as i16 && e.1 >= 0 && e.1 < grid[0].len() as i16
        });

        antinodes.extend(local_antinodes);
    }

    antinodes.sort();
    antinodes.dedup();
    println!("{}", antinodes.len());
}
