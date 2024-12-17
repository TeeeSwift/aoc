use crate::direction::Direction;
use crate::grid::Grid;

#[derive(Clone, Debug)]
pub enum Node {
    Robot { y: usize, x: usize },
    Box { y: usize, x: usize },
    Boxleft { y: usize, x: usize },
    Boxright { y: usize, x: usize },
    Wall { y: usize, x: usize },
}

impl Node {
    pub fn check_move(&mut self, grid: &mut Grid, direction: &Direction) -> bool {
        let (y, x) = self.get_yx();

        if let Node::Boxright { .. } = self {
            let left = grid.0[y][x - 1].clone();
            return left.unwrap().try_move(grid, direction);
        };

        println!("current position: {} {}", y, x);

        let (destination_vector, check_vectors) = self.get_vectors(&direction);
        let mut cells_empty: Vec<bool> = vec![];

        for (dy, dx) in check_vectors.iter() {
            let destination_coordinates: Option<(usize, usize)> =
                self.get_coordinates_from_vector((*dy, *dx), &grid);
            if destination_coordinates.is_none() {
                cells_empty.push(false);
                break;
            }

            let (neighbor_y, neighbor_x) = destination_coordinates.unwrap();
            let opt = grid.0[neighbor_y][neighbor_x].clone();

            println!(
                "checking neighbor at {} {}",
                destination_vector.0, destination_vector.1
            );
            println!("neighbors: {} {}", neighbor_y, neighbor_x);

            if opt.is_none() {
                cells_empty.push(true);
                continue;
            }

            if let Some(mut node) = opt {
                match node {
                    Node::Wall { .. } => {
                        println!("theres a wall");
                        cells_empty.push(false);
                        break;
                    }
                    Node::Box { .. } => {
                        println!("there's a box");
                        let secondary_check = node.check_move(grid, &direction);
                        cells_empty.push(secondary_check);
                    }
                    Node::Boxleft { y, x } => {
                        println!("there's a box left");
                        let secondary_check = node.check_move(grid, &direction);
                        cells_empty.push(secondary_check);
                    }
                    Node::Boxright { y, x } => {
                        println!("there's a box right");
                        println!("{} {}", y, x);
                        let left = grid.0[y][x - 1].clone();
                        println!("{:?}", left);
                        let secondary_check = left.unwrap().check_move(grid, &direction);
                        cells_empty.push(secondary_check);
                    }
                    _ => {}
                }
            }
        }

        return cells_empty.into_iter().all(|x| x);
    }

    pub fn try_move(&mut self, grid: &mut Grid, direction: &Direction) -> bool {
        let (y, x) = self.get_yx();

        if let Node::Boxright { .. } = self {
            let left = grid.0[y][x - 1].clone();
            return left.unwrap().try_move(grid, direction);
        };

        println!("current position: {} {}", y, x);

        if !self.check_move(grid, direction) {
            return false;
        }

        let (destination_vector, check_vectors) = self.get_vectors(&direction);
        let mut cells_empty: Vec<bool> = vec![];

        for (dy, dx) in check_vectors.iter() {
            let destination_coordinates: Option<(usize, usize)> =
                self.get_coordinates_from_vector((*dy, *dx), &grid);
            if destination_coordinates.is_none() {
                cells_empty.push(false);
                break;
            }

            let (neighbor_y, neighbor_x) = destination_coordinates.unwrap();
            let opt = grid.0[neighbor_y][neighbor_x].clone();

            println!(
                "checking neighbor at {} {}",
                destination_vector.0, destination_vector.1
            );
            println!("neighbors: {} {}", neighbor_y, neighbor_x);

            if opt.is_none() {
                cells_empty.push(true);
                continue;
            }

            if let Some(mut node) = opt {
                match node {
                    Node::Wall { .. } => {
                        println!("theres a wall");
                        cells_empty.push(false);
                        break;
                    }
                    Node::Box { .. } => {
                        println!("there's a box");
                        let secondary_check = node.try_move(grid, &direction);
                        cells_empty.push(secondary_check);
                    }
                    Node::Boxleft { y, x } => {
                        println!("there's a box left");
                        let secondary_check = node.try_move(grid, &direction);
                        cells_empty.push(secondary_check);
                    }
                    Node::Boxright { y, x } => {
                        println!("there's a box right");
                        println!("{} {}", y, x);
                        let left = grid.0[y][x - 1].clone();
                        println!("{:?}", left);
                        let secondary_check = left.unwrap().try_move(grid, &direction);
                        cells_empty.push(secondary_check);
                    }
                    _ => {}
                }
            }
        }
        if cells_empty.into_iter().all(|x| x) {
            println!("destination ok");
            // get destination coordinates

            let (dest_y, dest_x) = self
                .get_coordinates_from_vector(destination_vector, &grid)
                .unwrap();

            if let Node::Boxleft { .. } = self {
                let mut right = grid.0[y][x + 1].clone();
                right.as_mut().unwrap().set_yx(dest_y, dest_x + 1);

                match direction {
                    Direction::Up => {
                        grid.0[y][x + 1] = None;
                        grid.0[y - 1][x + 1] = right;
                    }
                    Direction::Down => {
                        grid.0[y][x + 1] = None;
                        grid.0[y + 1][x + 1] = right;
                    }
                    Direction::Left => {
                        grid.0[y][x] = right;
                        self.set_yx(dest_y, dest_x);
                        grid.0[dest_y][dest_x] = Some(self.clone());
                        return true;
                    }
                    Direction::Right => {
                        grid.0[y][x + 1] = None;
                        grid.0[y][x + 2] = right;
                    }
                }
            };

            grid.0[y][x] = None;
            self.set_yx(dest_y, dest_x);
            grid.0[dest_y][dest_x] = Some(self.clone());

            // if you're a boxleft, move yourself and boxright
            // if you're a boxright, move yourself and boxleft
            println!("new position: {} {}", dest_y, dest_x);
            return true;
        }

        false
    }

    pub fn get_yx(&self) -> (usize, usize) {
        match self {
            Node::Robot { y, x }
            | Node::Box { y, x }
            | Node::Boxright { y, x }
            | Node::Boxleft { y, x }
            | Node::Wall { y, x } => (*y, *x),
        }
    }

    pub fn set_yx(&mut self, newy: usize, newx: usize) {
        match self {
            Node::Robot {
                ref mut y,
                ref mut x,
            }
            | Node::Box {
                ref mut y,
                ref mut x,
            }
            | Node::Boxright {
                ref mut y,
                ref mut x,
            }
            | Node::Boxleft {
                ref mut y,
                ref mut x,
            }
            | Node::Wall {
                ref mut y,
                ref mut x,
            } => {
                *y = newy;
                *x = newx
            }
        }
    }

    pub fn get_coordinates_from_vector(
        &self,
        destination_vector: (isize, isize),
        grid: &Grid,
    ) -> Option<(usize, usize)> {
        match &self {
            Node::Robot { y, x }
            | Node::Box { y, x }
            | Node::Boxright { y, x }
            | Node::Boxleft { y, x } => {
                let (dy, dx) = destination_vector;
                let neighbor_y = *y as isize + dy;
                let neighbor_x = *x as isize + dx;

                if neighbor_y < 0
                    || neighbor_x < 0
                    || neighbor_y as usize >= grid.0.len()
                    || neighbor_x as usize >= grid.0[0].len()
                {
                    return None;
                }

                let neighbor_y = neighbor_y as usize;
                let neighbor_x = neighbor_x as usize;

                return Some((neighbor_y, neighbor_x));
            }
            _ => None,
        }
    }

    pub fn get_vectors(&self, direction: &Direction) -> ((isize, isize), Vec<(isize, isize)>) {
        match self {
            Node::Box { .. } | Node::Robot { .. } => match direction {
                Direction::Up => ((-1, 0), vec![(-1, 0)]),
                Direction::Down => ((1, 0), vec![(1, 0)]),
                Direction::Left => ((0, -1), vec![(0, -1)]),
                Direction::Right => ((0, 1), vec![(0, 1)]),
            },
            Node::Boxleft { .. } => match direction {
                Direction::Up => ((-1, 0), vec![(-1, 0), (-1, 1)]),
                Direction::Down => ((1, 0), vec![(1, 0), (1, 1)]),
                Direction::Right => ((0, 1), vec![(0, 2)]),
                Direction::Left => ((0, -1), vec![(0, -1)]),
            },
            Node::Boxright { .. } => match direction {
                Direction::Up => ((-1, 0), vec![(-1, 0), (-1, 1)]),
                Direction::Down => ((1, 0), vec![(1, 0), (1, 1)]),
                Direction::Right => ((0, 1), vec![(0, 1)]),
                Direction::Left => ((0, -1), vec![(0, -1)]),
            },
            Node::Wall { .. } => {
                return ((0, 0), vec![]);
            }
        }
    }
}
//
// #[test]
// fn test_try_move() {
//     let v: Vec<Vec<char>> = vec![vec!['@', '.']];
//     let mut grid = Grid::new(v);
//     println!("{:?}", grid);
//
//     let robot = grid.0[0][0].clone();
//     let a = robot.unwrap().try_move(&mut grid, (0, 1));
//     println!("{}", a);
//     println!("{:?}", grid);
// }
//
// #[test]
// fn test_wall() {
//     let v: Vec<Vec<char>> = vec![vec!['@', '#']];
//     let mut grid = Grid::new(v);
//     println!("{:?}", grid);
//
//     let robot = grid.0[0][0].clone();
//     let a = robot.unwrap().try_move(&mut grid, (0, 1));
//     println!("{}", a);
//     println!("{:?}", grid);
// }
//
// #[test]
// fn test_box_wall() {
//     let v: Vec<Vec<char>> = vec![vec!['@', 'O', '#']];
//     let mut grid = Grid::new(v);
//     println!("{:?}", grid);
//
//     let robot = grid.0[0][0].clone();
//     let a = robot.unwrap().try_move(&mut grid, (0, 1));
//     println!("{}", a);
//     println!("{:?}", grid);
// }
//
// #[test]
// fn test_nested_try_move() {
//     let v: Vec<Vec<char>> = vec![vec!['@', 'O', '.']];
//     let mut grid = Grid::new(v);
//     println!("{:?}", grid);
//
//     let robot = grid.0[0][0].clone();
//     let a = robot.unwrap().try_move(&mut grid, (0, 1));
//     println!("{:?}", grid);
// }
//
// #[test]
// fn test_box_gap() {
//     let v: Vec<Vec<char>> = vec![vec!['@', 'O', '.', 'O', '.', '#']];
//     let mut grid = Grid::new(v);
//     println!("{:?}", grid);
//
//     let robot = grid.0[0][0].clone();
//     let a = robot.unwrap().try_move(&mut grid, (0, 1));
//     println!("{:?}", grid);
// }
//
// #[test]
// fn test_double_box() {
//     let v: Vec<Vec<char>> = vec![vec!['@', 'O', 'O', '.', '#']];
//     let mut grid = Grid::new(v);
//     println!("{:?}", grid);
//
//     let robot = grid.0[0][0].clone();
//     let a = robot.unwrap().try_move(&mut grid, (0, 1));
//     println!("{:?}", grid);
// }
