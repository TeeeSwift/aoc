pub struct Node<'a> {
    pub value: char,
    pub y: usize,
    pub x: usize,
    pub grid: &'a Vec<Vec<char>>,
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl<'a> Node<'a> {
    pub fn new(value: char, y: usize, x: usize, grid: &'a Vec<Vec<char>>) -> Self {
        Node {
            value,
            y,
            x,
            grid,
            north: true,
            east: true,
            south: true,
            west: true,
        }
    }

    pub fn get_valid_neighbors(&mut self) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = vec![];

        // UP
        if self.y > 0 {
            if self.grid[self.y - 1][self.x] == self.value {
                self.north = false;
                v.push((self.y - 1, self.x));
            }
        };

        // RIGHT
        if self.x < self.grid[0].len() - 1 {
            if self.grid[self.y][self.x + 1] == self.value {
                self.east = false;
                v.push((self.y, self.x + 1));
            }
        };

        // DOWN
        if self.y < self.grid.len() - 1 {
            if self.grid[self.y + 1][self.x] == self.value {
                self.south = false;
                v.push((self.y + 1, self.x));
            }
        };

        // LEFT
        if self.x > 0 {
            if self.grid[self.y][self.x - 1] == self.value {
                self.west = false;
                v.push((self.y, self.x - 1));
            }
        };

        return v;
    }

    pub fn count_edges(&self) -> usize {
        let mut count: usize = 0;
        if self.north {
            count += 1;
        };
        if self.east {
            count += 1;
        };
        if self.south {
            count += 1;
        };
        if self.west {
            count += 1;
        };

        count
    }

    pub fn count_corners(&self) -> usize {
        match self.count_edges() {
            0 => {
                let mut count: usize = 0;
                if self.grid[self.y - 1][self.x - 1] != self.value {
                    count += 1;
                }
                if self.grid[self.y - 1][self.x + 1] != self.value {
                    count += 1;
                }
                if self.grid[self.y + 1][self.x + 1] != self.value {
                    count += 1;
                }
                if self.grid[self.y + 1][self.x - 1] != self.value {
                    count += 1;
                }
                count
            }
            1 => {
                let mut count: usize = 0;
                if self.north && self.grid[self.y + 1][self.x - 1] != self.value {
                    count += 1;
                }
                if self.north && self.grid[self.y + 1][self.x + 1] != self.value {
                    count += 1;
                }

                if self.east && self.grid[self.y - 1][self.x - 1] != self.value {
                    count += 1;
                }
                if self.east && self.grid[self.y + 1][self.x - 1] != self.value {
                    count += 1;
                }

                if self.west && self.grid[self.y - 1][self.x + 1] != self.value {
                    count += 1;
                }
                if self.west && self.grid[self.y + 1][self.x + 1] != self.value {
                    count += 1;
                }

                if self.south && self.grid[self.y - 1][self.x - 1] != self.value {
                    count += 1;
                }
                if self.south && self.grid[self.y - 1][self.x + 1] != self.value {
                    count += 1;
                }

                count
            }
            2 => {
                if self.north && self.east {
                    if self.grid[self.y + 1][self.x - 1] == self.value {
                        return 1;
                    };
                    return 2;
                } else if self.north && self.west {
                    if self.grid[self.y + 1][self.x + 1] == self.value {
                        return 1;
                    };
                    return 2;
                } else if self.east && self.south {
                    if self.grid[self.y - 1][self.x - 1] == self.value {
                        return 1;
                    };
                    return 2;
                } else if self.west && self.south {
                    if self.grid[self.y - 1][self.x + 1] == self.value {
                        return 1;
                    };
                    return 2;
                } else if self.north && self.south {
                    return 0;
                } else {
                    return 0;
                }
            }
            3 => 2,
            4 => 4,
            _ => 0,
        }
    }
}
