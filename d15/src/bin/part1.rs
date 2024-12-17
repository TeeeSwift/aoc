use d15::direction::Direction::{Down, Left, Right, Up};
use d15::{node::Node, parse_input};

pub fn main() {
    let (mut grid, commands, robot_position) = parse_input();

    let mut robot: Node = Node::Robot {
        y: robot_position.0,
        x: robot_position.1,
    };

    let mut i: usize = 0;

    for char in commands.chars().into_iter() {
        // if i == 5 {
        //     break;
        // };
        println!("{}", char);
        i += 1;

        println!("current robot position: {robot_position:?}");
        let result = match char {
            '^' => robot.try_move(&mut grid, &Up),
            '>' => robot.try_move(&mut grid, &Right),
            '<' => robot.try_move(&mut grid, &Left),
            'v' => robot.try_move(&mut grid, &Down),
            _ => false,
        };
        println!("moved: {}", result);
        println!("new robot position: {robot_position:?}");
        println!("{:?}", grid);
    }

    let total: usize = grid
        .0
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|n| match n {
                    Some(Node::Box { .. }) => true,
                    _ => false,
                })
                .map(|n| {
                    if let Some(Node::Box { y, x, .. }) = n {
                        (y * 100) + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();

    println!("{}", total);
}
