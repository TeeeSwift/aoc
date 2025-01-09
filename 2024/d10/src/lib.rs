use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

pub mod node;

use node::Node;

pub fn parse_input() -> (Vec<Vec<Node>>, Vec<Node>) {
    let file = File::open("src/input").unwrap();
    // let file = File::open("src/sample").unwrap();
    let reader = BufReader::new(file);

    let mut v: Vec<Vec<Node>> = vec![];
    let mut starting_nodes: Vec<Node> = vec![];

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        v.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let value = c.to_string().parse::<i32>().unwrap();
                    if value == 0 {
                        starting_nodes.push(Node {
                            y: y as i16,
                            x: x as i16,
                            value: value as i16,
                        })
                    };

                    return Node {
                        y: y as i16,
                        x: x as i16,
                        value: value as i16,
                    };
                })
                .collect::<Vec<Node>>(),
        );
    }

    (v, starting_nodes)
}

pub fn find_summits(starting_node: Node, grid_arc: Arc<Vec<Vec<Node>>>, dedup_flag: bool) -> usize {
    let mut summit: Vec<Node> = vec![];

    let mut queue: Vec<Node> = vec![];

    queue.push(starting_node);

    while queue.len() > 0 {
        // - Take most recent node off the stack
        let node = queue.pop().unwrap();

        // - If it's 9, increment and continue
        if node.value == 9 {
            summit.push(node);
            continue;
        }

        // - If it's not, see its value, see its neighbors
        let neighbors = node.get_neighbors(&grid_arc);

        // - Enqueue valid neighbors
        for neighbor in neighbors {
            queue.push(*neighbor);
        }
    }

    let mut s = summit
        .iter()
        .map(|n| (n.y, n.x))
        .collect::<Vec<(i16, i16)>>();
    s.sort();

    if dedup_flag {
        s.dedup();
    }

    return s.len();
}
