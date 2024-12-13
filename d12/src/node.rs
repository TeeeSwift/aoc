use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node {
    pub visited: bool,
    pub value: char,
    pub y: usize,
    pub x: usize,
    pub grid: Option<Rc<RefCell<Vec<Vec<Node>>>>>,
}

impl Node {
    pub fn new(c: char, y: usize, x: usize) -> Self {
        Node {
            visited: false,
            value: c,
            y,
            x,
            grid: None,
        }
    }

    pub fn set_grid(&mut self, grid: Rc<RefCell<Vec<Vec<Node>>>>) {
        self.grid = Some(grid);
    }

    pub fn get_neighbors(&self) -> Vec<(usize, usize)> {
        println!("getting neighbors for: {}, {}", self.y, self.x);
        let mut v: Vec<(usize, usize)> = vec![];

        let grid = self.grid.as_ref().unwrap().borrow();

        // returns valid neighbors
        if self.y != 0 {
            // get up
            v.push((self.y - 1, self.x));
        };

        if self.y < grid.len() - 1 && self.is_valid(&grid, (self.y + 1, self.x)) {
            // get get bottom
            v.push((self.y + 1, self.x));
        }

        if self.x != 0 && self.is_valid(&grid, (self.y, self.x - 1)) {
            // get left
            v.push((self.y, self.x - 1));
        }

        if self.x < grid[0].len() - 1 && self.is_valid(&grid, (self.y, self.x + 1)) {
            // get get right
            v.push((self.y, self.x + 1));
        }

        println!("adding: {:?}", v);
        v
    }

    fn is_valid(&self, grid: &Vec<Vec<Node>>, coordinates: (usize, usize)) -> bool {
        let neighbor = &grid[coordinates.0][coordinates.1];

        neighbor.value == self.value && !(neighbor.visited)
    }
}
