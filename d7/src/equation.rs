#[derive(Debug)]
pub struct Equation {
    pub target: u64,
    pub numbers: Vec<u64>,
    pub operations: Vec<Operations>,
    pub combinations: Vec<Vec<Operations>>,
    pub potential_results: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operations {
    Add,
    Multiply,
    Concat,
}

impl Operations {
    pub fn operate(&self, num1: u64, num2: u64) -> u64 {
        match self {
            Operations::Add => num1 + num2,
            Operations::Multiply => num1 * num2,
            Operations::Concat => format!("{}{}", num1, num2).parse::<u64>().unwrap(),
        }
    }
}

impl Equation {
    pub fn generate_operation_combinations(&mut self) {
        for _ in 1..self.numbers.len() {
            self.add_next_operation();
        }
    }

    fn add_next_operation(&mut self) {
        let mut new_v: Vec<Vec<Operations>> = vec![];

        for operation in &self.operations {
            if self.combinations.len() == 0 {
                new_v.push(vec![*operation]);
            } else {
                for combination in self.combinations.iter() {
                    let mut new = combination.clone();
                    new.push(*operation);
                    new_v.push(new);
                }
            }
        }

        self.combinations = new_v;
    }

    pub fn compute_potential_results(&mut self) {
        for combination in self.combinations.iter() {
            let mut current: u64 = self.numbers[0];
            for (index, operation) in combination.iter().enumerate() {
                current = operation.operate(current, self.numbers[index + 1]);
            }
            self.potential_results.push(current);
        }
    }
}

#[test]
fn generate_combinations() {
    let mut e = Equation {
        target: 100,
        numbers: vec![1, 2, 3],
        operations: vec![Operations::Add, Operations::Multiply],
        combinations: vec![],
        potential_results: vec![],
    };

    e.generate_operation_combinations();

    for combination in &e.combinations {
        println!("{:?}", combination);
    }

    e.compute_potential_results();
    println!("{:?}", e.potential_results);
}
