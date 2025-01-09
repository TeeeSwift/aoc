use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn parse_input() -> Vec<Robot> {
    let file = File::open("src/sample").unwrap();
    let height = 7;
    let width = 11;

    let file = File::open("src/input").unwrap();
    let height = 103;
    let width = 101;

    let reader = BufReader::new(file);
    let mut robots: Vec<Robot> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let re = Regex::new(r"-?\d+").unwrap();

        let captures: Vec<isize> = re
            .find_iter(&line)
            .filter_map(|m| m.as_str().parse::<isize>().ok())
            .collect();

        robots.push(Robot::new(
            captures[0],
            captures[1],
            captures[2],
            captures[3],
            height,
            width,
        ));
    }

    return robots;
}

#[derive(Debug)]
pub struct Robot {
    pub x: isize,
    pub y: isize,
    pub dx: isize,
    pub dy: isize,
    height: isize,
    width: isize,
}

impl Robot {
    pub fn new(x: isize, y: isize, dx: isize, dy: isize, height: isize, width: isize) -> Self {
        Robot {
            x,
            y,
            dx,
            dy,
            height,
            width,
        }
    }

    pub fn position_after_n_iterations(&self, n: isize) -> (isize, isize) {
        let distance_traveled_x = n * self.dx;
        let distance_traveled_y = n * self.dy;

        let x = self.adjust_movement(self.x, distance_traveled_x, self.width);
        let y = self.adjust_movement(self.y, distance_traveled_y, self.height);

        return (x, y);
    }

    fn adjust_movement(
        &self,
        starting_position: isize,
        distance_moved: isize,
        dimension: isize,
    ) -> isize {
        match starting_position + distance_moved {
            result if result % dimension == 0 => return 0,
            result if result >= 0 && result < dimension => return result,
            result if result >= 0 && result > dimension => return result % dimension,
            result if result < 0 => {
                let a = result.abs() % dimension;
                dimension - a
            }
            _ => return self.x,
        }
    }
}

pub fn quadrant(position: (isize, isize), width: isize, height: isize) -> Option<Quadrant> {
    let (x, y) = position;
    let midx = width / 2;
    let midy = height / 2;

    if position.0 == midx || position.1 == midy {
        return None;
    }

    if x < midx && y < midy {
        return Some(Quadrant::NW);
    }
    if x > midx && y < midy {
        return Some(Quadrant::NE);
    }
    if x < midx && y > midy {
        return Some(Quadrant::SW);
    }
    // x > midx && y > midy {
    else {
        return Some(Quadrant::SE);
    }
}

#[derive(Debug)]
pub enum Quadrant {
    NE,
    NW,
    SE,
    SW,
}
