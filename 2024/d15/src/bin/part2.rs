use d15::direction::Direction::{Down, Left, Right, Up};
use d15::{node::Node, parse_input2};

pub fn main() {
    let (mut grid, commands, robot_position) = parse_input2();

    let mut robot: Node = Node::Robot {
        y: robot_position.0,
        x: robot_position.1,
    };

    let mut i: usize = 0;

    for char in commands.chars().into_iter() {
        // if i == 25 {
        //     break;
        // };
        println!("{}: {}", i, char);
        i += 1;

        match char {
            '^' => {
                robot.try_move(&mut grid, &Up);
            }
            '>' => {
                robot.try_move(&mut grid, &Right);
            }
            '<' => {
                robot.try_move(&mut grid, &Left);
            }
            'v' => {
                robot.try_move(&mut grid, &Down);
            }
            _ => {}
        };
        println!("{:?}", grid);
    }

    let total: usize = grid
        .0
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|n| match n {
                    Some(Node::Boxleft { .. }) => true,
                    _ => false,
                })
                .map(|n| {
                    if let Some(Node::Boxleft { y, x, .. }) = n {
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
