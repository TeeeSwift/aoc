use crate::button::Button;
use crate::prize::Prize;

#[derive(Debug)]
pub struct Machine {
    pub a: Button,
    pub b: Button,
    pub prize: Prize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            a: Button {
                x: 0,
                y: 0,
                coins: 0,
            },
            b: Button {
                x: 0,
                y: 0,
                coins: 0,
            },
            prize: Prize { x: 0, y: 0 },
        }
    }

    pub fn set_button(&mut self, char: &str, x: isize, y: isize, coins: isize) {
        match char {
            "A" => self.a = Button { x, y, coins },
            "B" => self.b = Button { x, y, coins },
            _ => {}
        };
    }

    pub fn set_prize(&mut self, x: isize, y: isize) {
        self.prize = Prize { x, y };
    }

    pub fn get_valid_pairs(&self) -> Vec<(isize, isize)> {
        let mut valid_pairs: Vec<(isize, isize)> = vec![];

        for i in 0..101 {
            if self.a.x * i > self.prize.x {
                break;
            };

            if (self.prize.x - (self.a.x * i)) % self.b.x == 0 {
                let b = (self.prize.x - (self.a.x * i)) / self.b.x;
                if b > 100 {
                    continue;
                }
                if self.prize.y != self.a.y * i + self.b.y * b {
                    continue;
                }

                valid_pairs.push((i, b))
            }
        }

        return valid_pairs;
    }

    pub fn calculate_cost(&self, a: isize, b: isize) -> isize {
        let total = a * self.a.coins + b * self.b.coins;
        println!("{} {} {}", a, b, total);
        total
    }

    pub fn reddit(&self) -> isize {
        let prize = (self.prize.x + 10000000000000, self.prize.y + 10000000000000);
        let det = self.a.x * self.b.y - self.a.y * self.b.x;
        let a = (prize.0 * self.b.y - prize.1 * self.b.x) / det;
        let b = (self.a.x * prize.1 - self.a.y * prize.0) / det;
        if (self.a.x * a + self.b.x * b, self.a.y * a + self.b.y * b) == (prize.0, prize.1) {
            (a * 3 + b) as isize
        } else {
            0
        }
    }

    pub fn get_button_presses(&self) -> (isize, isize) {
        //         A = (p_x*b_y - prize_y*b_x) / (a_x*b_y - a_y*b_x)
        // B = (a_x*p_y - a_y*p_x) / (a_x*b_y - a_y*b_x)

        if self.prize.x * self.b.y < self.prize.y * self.b.x {
            return (0, 0);
        };

        if self.a.x * self.prize.y < self.a.y * self.prize.x {
            return (0, 0);
        };

        let a = (self.prize.x * self.b.y - self.prize.y * self.b.x)
            / (self.a.x * self.b.y - self.a.y * self.b.x);

        let b = (self.a.x * self.prize.y - self.a.y * self.prize.x)
            / (self.a.x * self.b.y - self.a.y * self.b.x);

        let x_success = self.a.x * a + self.b.x * b == self.prize.x;
        let y_success = self.a.y * a + self.b.y * b == self.prize.y;

        if x_success && y_success {
            return (a, b);
        }

        (0, 0)
    }
}

#[test]
fn test_get_valid_pairs() {
    let mut machine = Machine::new();
    machine.set_button(&"A", 2, 3, 3);
    machine.set_button(&"B", 1, 4, 1);
    machine.set_prize(12, 24);
    println!("{:?}", machine);

    let v = machine.get_valid_pairs();
    println!("{:?}", v);
}
