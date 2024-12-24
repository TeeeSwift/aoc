use std::collections::{HashMap, VecDeque};

use d16::position::Position;
use d16::{_print_grid, get_options, parse_input};

fn main() {
    let (grid, start, goal_positions) = parse_input();
    _print_grid(&grid);

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
                        .and_modify(|e| e.push(position))
                        .or_insert(vec![position]);
                    queue.push_back(next_position);
                }
                std::cmp::Ordering::Greater => {
                    // Handle the case where the current cost is greater than the new cost
                    // (i.e., you want to update)
                    *current_next_position_cost = new_next_position_cost;
                    parent_map
                        .entry(next_position)
                        .and_modify(|e| e.push(position))
                        .or_insert(vec![position]);
                    queue.push_back(next_position);
                }
                std::cmp::Ordering::Less => {}
            }
        }
    }

    let mut lowest_cost_to_goal: usize = usize::MAX;

    for goal_position in goal_positions.into_iter() {
        let cost = cost_map.get(&goal_position).unwrap_or(&usize::MAX);
        println!("{cost}");
        if *cost < lowest_cost_to_goal {
            lowest_cost_to_goal = *cost;
        }
    }
}
