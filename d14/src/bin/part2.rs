use d14::parse_input;
use image::{GrayImage, Luma};

pub fn main() {
    let robots = parse_input();

    for i in 0..10000 {
        let mut img = GrayImage::new(101, 103);
        for robot in robots.iter() {
            let position = robot.position_after_n_iterations(i);
            img.put_pixel(position.0 as u32, position.1 as u32, Luma([255]));
        }

        let _ = img.save(format!("./src/images/{}.jpg", i));
    }
}
