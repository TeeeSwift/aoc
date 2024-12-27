#[derive(Debug)]
pub struct Computer {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub output: Vec<usize>,
}

impl Computer {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            output: vec![],
        }
    }

    pub fn accept_commands(&mut self, commands: Vec<usize>) {
        let mut i: usize = 0;
        while i < commands.len() {
            let command = commands[i];
            let operand = commands[i + 1];

            let (cursor_location, _): (Option<usize>, Option<usize>) =
                self.execute(command, operand);
            match cursor_location {
                Some(x) => i = x,
                None => i += 2,
            }
        }
    }

    pub fn duplicate(mut commands: Vec<usize>) -> String {
        let mut candidates: Vec<usize> = vec![0];
        commands.reverse();

        for target_num in commands {
            println!("target number: {target_num}");
            // get options
            let mut pairs: Vec<(usize, usize)> = vec![];
            for i in 0..=7 {
                for j in 0..=7 {
                    if i ^ j == target_num {
                        pairs.push((i, j));
                    }
                }
            }
            println!("pairs {:?}", pairs);

            let mut new_candidates: Vec<usize> = vec![];

            for candidate in &candidates {
                for (b, _) in &pairs {
                    let mut original_b = *b ^ 7;
                    original_b ^= 2;
                    new_candidates.push(*candidate * 8 + original_b);
                }
            }
            println!("new candidates: {:?}", new_candidates);

            new_candidates.retain(|x| {
                let mut b = *x % 8;
                b ^= 2;
                let c = x / 2usize.pow(b as u32);
                b ^= 7;
                b ^= c;
                target_num == b % 8
            });

            println!("after pruning: {:?}", new_candidates);
            candidates = new_candidates;
        }

        candidates.sort();
        println!("{:?}", candidates);

        String::new()
    }

    pub fn execute(&mut self, opcode: usize, operand: usize) -> (Option<usize>, Option<usize>) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxz(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            _ => self.cdv(operand),
        }
    }

    pub fn adv(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        let denominator = 2usize.pow(self.get_combo_operand(operand) as u32);
        self.a /= denominator;
        (None, None)
    }

    pub fn bxl(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        self.b ^= operand;

        (None, None)
    }
    pub fn bst(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        self.b = self.get_combo_operand(operand) % 8;
        (None, None)
    }

    pub fn jnz(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        if self.a == 0 {
            return (None, None);
        }

        (Some(operand), None)
    }

    pub fn bxz(&mut self, _: usize) -> (Option<usize>, Option<usize>) {
        self.b ^= self.c;

        (None, None)
    }

    pub fn out(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        let num = self.get_combo_operand(operand) % 8;
        self.output.push(num);
        (None, Some(num))
    }

    pub fn bdv(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        let denominator = 2usize.pow(self.get_combo_operand(operand) as u32);
        self.b = self.a / denominator;
        (None, None)
    }
    pub fn cdv(&mut self, operand: usize) -> (Option<usize>, Option<usize>) {
        let denominator = 2usize.pow(self.get_combo_operand(operand) as u32);
        self.c = self.a / denominator;

        (None, None)
    }

    pub fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0,
        }
    }

    pub fn print_output(&self) -> String {
        self.output
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[test]
fn stuff() {
    // If register C contains 9, the program 2,6 would set register B to 1.
    let mut computer = Computer::new(0, 0, 9);
    computer.accept_commands(vec![2, 6]);
    assert_eq!(1, computer.b);

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    let mut computer = Computer::new(10, 0, 0);
    computer.accept_commands(vec![5, 0, 5, 1, 5, 4]);
    assert_eq!("0,1,2", computer.print_output());

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    let mut computer = Computer::new(2024, 0, 0);
    computer.accept_commands(vec![0, 1, 5, 4, 3, 0]);
    assert_eq!("4,2,5,6,7,7,7,7,3,1,0", computer.print_output());
    assert_eq!(0, computer.a);

    // If register B contains 29, the program 1,7 would set register B to 26.
    let mut computer = Computer::new(0, 29, 0);
    computer.accept_commands(vec![1, 7]);
    assert_eq!(26, computer.b);

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    let mut computer = Computer::new(0, 2024, 43690);
    computer.accept_commands(vec![4, 0]);
    assert_eq!(44354, computer.b);
}

#[test]
fn duplicate() {
    let commands = vec![0, 3, 5, 4, 3, 0];
    let s = Computer::duplicate(commands.clone());
    println!("{s}");
}

#[test]
fn confirm() {
    let commands = vec![2, 4, 1, 2, 7, 5, 1, 7, 4, 4, 0, 3, 5, 5, 3, 0];
    // [378, 346]
    let mut computer = Computer::new(190593313094671, 0, 0);
    computer.accept_commands(commands);
    println!("{}", computer.print_output());
}
