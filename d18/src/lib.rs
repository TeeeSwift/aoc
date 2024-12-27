use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn parse_input() -> (Vec<Vec<char>>, VecDeque<(usize, usize)>) {
    let filename = "src/sample";
    let filename = "src/input";

    let v: Vec<Vec<char>> = match filename {
        "src/sample" => vec![vec!['.'; 7]; 7],
        _ => vec![vec!['.'; 71]; 71],
    };

    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    let mut corruptions: VecDeque<(usize, usize)> = VecDeque::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split(',');
        corruptions.push_back((
            iter.next().unwrap().trim().parse::<usize>().unwrap(),
            iter.next().unwrap().trim().parse::<usize>().unwrap(),
        ))
    }

    (v, corruptions)
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn progress_grid(
    grid: &mut [Vec<char>],
    corruptions: &mut VecDeque<(usize, usize)>,
    count: usize,
) {
    for _ in 1..=count {
        let corruption = corruptions.pop_front().unwrap();
        grid[corruption.1][corruption.0] = '#';
    }
}

pub fn find_shortest_path(grid: &mut [Vec<char>]) -> Result<Vec<(usize, usize)>, bool> {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
    let mut cost_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut parent_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // get distance to queue square
    while let Some(position) = queue.pop_front() {
        let distance_to_current_square = *cost_map.get(&position).unwrap_or(&0);
        // get options
        let options = get_options(grid, position);

        for option in options {
            let current_lowest_cost = *cost_map.get(&option).unwrap_or(&usize::MAX);
            match current_lowest_cost > distance_to_current_square + 1 {
                true => {
                    cost_map.insert(option, distance_to_current_square + 1);
                    parent_map.insert(option, position);
                    queue.push_back(option);
                }
                false => { /* do nothing*/ }
            }
        }
    }

    let mut path: Vec<(usize, usize)> = vec![];
    let mut position = (grid[0].len() - 1, grid.len() - 1);

    if !parent_map.contains_key(&position) {
        Err(false)
    } else {
        while let Some(parent) = parent_map.get(&position) {
            grid[parent.1][parent.0] = 'o';
            path.insert(0, *parent);
            if *parent == (0, 0) {
                break;
            }
            position = (parent.0, parent.1);
        }

        Ok(path)
    }
}

pub fn get_options(grid: &[Vec<char>], position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut options: Vec<(usize, usize)> = vec![];
    let width = grid[0].len() - 1;
    let height = grid.len() - 1;

    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let (newx, newy) = (position.0 as isize + dx, position.1 as isize + dy);
        if newy > height as isize || newy < 0 || newx > width as isize || newx < 0 {
            continue;
        };
        if grid[newy as usize][newx as usize] == '#' {
            continue;
        }
        options.push((newx as usize, newy as usize));
    }

    options
}

#[test]
fn parse_inp() {
    let (a, b) = parse_input();
    println!("{:?}", b);
    print_grid(&a);
}

#[test]
fn test_progress_grid() {
    let (mut a, mut b) = parse_input();
    println!("{:?}", b);
    print_grid(&a);
    println!("------------");
    progress_grid(&mut a, &mut b, 3);
    print_grid(&a);
}

#[test]
fn test_shortest_path() {
    let (mut a, mut b) = parse_input();
    println!("{:?}", b);
    print_grid(&a);
    println!("------------");
    progress_grid(&mut a, &mut b, 12);
    let v = find_shortest_path(&mut a);
    println!("{:?}", v);
    print_grid(&a);
}
