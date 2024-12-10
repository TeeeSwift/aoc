#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub y: i16,
    pub x: i16,
    pub value: i16,
}

#[derive(Debug)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn get_vector(&self) -> (i16, i16) {
        match self {
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

impl Node {
    pub fn get_neighbor<'a>(
        &self,
        direction: &Direction,
        grid: &'a Vec<Vec<Node>>,
    ) -> Option<&'a Node> {
        let (dy, dx) = direction.get_vector();
        let newy = self.y + dy;
        let newx = self.x + dx;

        if newy < 0 || newx < 0 || newy >= grid.len() as i16 || newx >= grid[0].len() as i16 {
            return None;
        }

        let node = &grid[newy as usize][newx as usize];

        return Some(node);
    }

    pub fn get_neighbors<'a>(&self, grid: &'a Vec<Vec<Node>>) -> Vec<&'a Node> {
        let mut neighbors: Vec<&Node> = vec![];

        for direction in vec![
            Direction::UP,
            Direction::RIGHT,
            Direction::DOWN,
            Direction::LEFT,
        ]
        .iter()
        {
            match self.get_neighbor(&direction, &grid) {
                Some(node) => {
                    if node.value == self.value + 1 {
                        neighbors.push(node)
                    }
                }
                None => {}
            };
        }

        neighbors
    }
}
