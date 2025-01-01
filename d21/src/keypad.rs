use crate::build_commands;
use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyPad {
    commands: HashMap<(char, char), Vec<String>>,
    sequence_cache: HashMap<(String, usize), usize>,
}

impl KeyPad {
    pub fn new(keypad_type: &str) -> Self {
        let grid = match keypad_type {
            "numpad" => vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec!['#', '0', 'A'],
            ],
            "arrows" => vec![vec!['#', '^', 'A'], vec!['<', 'v', '>']],
            _ => vec![],
        };

        let commands = build_commands(&grid);
        Self {
            commands,
            sequence_cache: HashMap::new(),
        }
    }

    pub fn command_sequences(&self, start: char, end: char) -> Option<&Vec<String>> {
        self.commands.get(&(start, end))
    }

    pub fn append_command(&self, current_sequence: String, start: char, end: char) -> Vec<String> {
        let next_sequences = self.command_sequences(start, end).unwrap().clone();

        next_sequences
            .into_iter()
            .map(|sequence| {
                let mut current_sequence = current_sequence.clone();
                current_sequence.push_str(sequence.as_str());
                current_sequence.push('A');
                current_sequence
            })
            .collect::<Vec<String>>()
    }

    pub fn generate_sequences(&self, input: &str) -> Vec<String> {
        let mut command_sequences: Vec<String> = vec![String::new()];
        let input = format!("A{input}");

        for i in 0..input.len() - 1 {
            let mut new_command_sequences: Vec<String> = vec![];

            for sequence in command_sequences {
                let new_sequences = self.append_command(
                    sequence,
                    input.chars().nth(i).unwrap(),
                    input.chars().nth(i + 1).unwrap(),
                );
                for seq in new_sequences {
                    new_command_sequences.push(seq);
                }
            }

            command_sequences = new_command_sequences;
        }
        if let Some(min_length) = command_sequences.iter().map(|s| s.len()).min() {
            // Keep only strings with the minimum length
            command_sequences.retain(|s| s.len() == min_length);
        };

        command_sequences
    }

    pub fn shortest_seq(&mut self, keys: String, depth: usize) -> usize {
        if depth == 0 {
            return keys.len();
        }

        if let Some(&x) = self.sequence_cache.get(&(keys.clone(), depth)) {
            x
        } else {
            let mut total: usize = 0;

            for (i, sub_key) in keys.split('A').enumerate() {
                // Skip the last one by checking if we're on the last iteration
                if i == keys.split('A').count() - 1 {
                    break; // Skip processing the last element
                }

                let sub_key = format!("{sub_key}A");
                let sequences = self.generate_sequences(&sub_key);
                total += sequences
                    .into_iter()
                    .map(|sequence| self.shortest_seq(sequence, depth - 1))
                    .min()
                    .unwrap();
            }

            self.sequence_cache.insert((keys, depth), total);

            total
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parse_input;

    use super::*;

    #[test]
    fn t_build() {
        let numpad = KeyPad::new("numpad");

        for command in &numpad.commands {
            println!("{command:?}");
        }

        assert_eq!(2, numpad.commands.get(&('7', '3')).unwrap().len());
    }

    #[test]
    fn t_get_shortest_numpad() {
        let numpad = KeyPad::new("numpad");

        let a = numpad.command_sequences('1', 'A').unwrap();
        assert_eq!(*a, [">>v".to_string()]);
    }

    #[test]
    fn t_generate_commands_numpad() {
        let numpad = KeyPad::new("numpad");

        let mut strings = numpad.generate_sequences("029A");

        let mut answers = ["<A^A>^^AvvvA".to_string(), "<A^A^^>AvvvA".to_string()];

        println!("{strings:?}");
        strings.sort();
        answers.sort();

        assert_eq!(strings, answers);
    }

    #[test]
    fn t_generate_commands_arrowpad() {
        let arrow_pad = KeyPad::new("arrows");
        let mut strings = arrow_pad.generate_sequences("<A");
        let mut answers = ["v<<A>>^A".to_string()];

        println!("{strings:?}");
        strings.sort();
        answers.sort();

        assert_eq!(strings, answers);
    }

    #[test]
    fn t_calculate() {
        let numpad = KeyPad::new("numpad");
        let arrowpad = KeyPad::new("arrows");

        let commands = parse_input("src/sample").unwrap();

        for command in commands {
            let mut sequences = numpad.generate_sequences(&command);

            sequences = sequences
                .into_iter()
                .flat_map(|string| arrowpad.generate_sequences(&string))
                .collect::<Vec<String>>();

            sequences = sequences
                .into_iter()
                .flat_map(|string| arrowpad.generate_sequences(&string))
                .collect::<Vec<String>>();

            if let Some(shortest) = sequences.into_iter().min_by_key(|s| s.len()) {
                println!("{} {}", command, shortest.len());
            }
        }
    }

    #[test]
    fn t_shortest_seq() {
        let mut arrow_pad = KeyPad::new("arrows");
        let a = arrow_pad.shortest_seq("<A".to_string(), 25);
        println!("{:?}", a);
    }
}
