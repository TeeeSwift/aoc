use d14::{parse_input, quadrant, Quadrant};

pub fn main() {
    let robots = parse_input();

    let mut nw: usize = 0;
    let mut ne: usize = 0;
    let mut sw: usize = 0;
    let mut se: usize = 0;

    for robot in robots.iter() {
        let position = robot.position_after_n_iterations(100);
        println!("{:?}", position);

        // let quadrant = quadrant(position, 11, 7);
        let quadrant = quadrant(position, 101, 103);

        // println!("{:?}", quadrant);
        if quadrant.is_none() {
            continue;
        };

        match quadrant.unwrap() {
            Quadrant::NW => nw += 1,
            Quadrant::NE => ne += 1,
            Quadrant::SE => se += 1,
            Quadrant::SW => sw += 1,
        }
    }

    println!("{}", nw);
    println!("{}", ne);
    println!("{}", sw);
    println!("{}", se);
    println!("{}", nw * ne * sw * se);
}
