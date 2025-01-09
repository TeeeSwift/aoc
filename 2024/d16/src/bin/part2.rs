use std::collections::{HashMap, HashSet, VecDeque};

use d16::position::Position;
use d16::{_print_grid, get_options, parse_input};

fn main() {
    let (mut grid, start, goal_positions) = parse_input();
    // _print_grid(&grid);

    // cost to get to each position
    let mut cost_map: HashMap<Position, usize> = HashMap::new();
    // cheapest parent of each position
    let mut parent_map: HashMap<Position, Vec<Position>> = HashMap::new();
    // queue of positions to evaluate
    let mut queue: VecDeque<Position> = VecDeque::new();

    queue.push_back(start);

    while let Some(position) = queue.pop_front() {
        // get options for cell
        let options = get_options(&grid, position);
        if options.is_empty() {
            continue;
        }

        for (next_position, step_cost) in options {
            let cost_till_current = cost_map.entry(position).or_insert(0);
            let mut new_next_position_cost = *cost_till_current + step_cost;

            // current minimum cost to get to next_position
            let current_next_position_cost = cost_map.entry(next_position).or_insert(usize::MAX);

            match current_next_position_cost.cmp(&&mut new_next_position_cost) {
                std::cmp::Ordering::Equal => {
                    parent_map
                        .entry(next_position)
                        .and_modify(|e| e.push(position));
                    queue.push_back(next_position);
                }
                std::cmp::Ordering::Greater => {
                    *current_next_position_cost = new_next_position_cost;
                    parent_map
                        .entry(next_position)
                        .and_modify(|e| *e = vec![position])
                        .or_insert(vec![position]);
                    queue.push_back(next_position);
                }
                std::cmp::Ordering::Less => {}
            }
        }
    }

    let mut current_lowest_cost_to_goal: usize = usize::MAX;
    let mut lowest_path: VecDeque<Position> = VecDeque::new();

    for goal_position in goal_positions.into_iter() {
        let cost = cost_map.get(&goal_position).unwrap_or(&usize::MAX);
        if *cost < current_lowest_cost_to_goal {
            current_lowest_cost_to_goal = *cost;
            lowest_path.clear();
            lowest_path.push_back(goal_position);
        }
    }

    println!("{:?}", lowest_path);

    let mut positions: HashSet<Position> = HashSet::new();
    let mut cells: HashSet<(usize, usize)> = HashSet::new();

    while let Some(position) = lowest_path.pop_front() {
        match positions.insert(position) {
            true => {
                grid[position.0][position.1] = 'o';
                cells.insert((position.0, position.1));
                // get parents
                if let Some(parents) = parent_map.get(&position) {
                    let mut costs: Vec<usize> = vec![];
                    for parent in parents {
                        costs.push(*cost_map.get(parent).unwrap());
                        lowest_path.push_back(*parent);
                    }
                    println!(
                        "position: {:?}, parents: {:?}, {:?}",
                        position, parents, costs
                    );
                }
            }
            false => {
                continue;
            }
        };
    }

    // _print_grid(&grid);
    println!("{}", cells.len());
}
