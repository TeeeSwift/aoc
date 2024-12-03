use regex::Regex;

pub fn extract_factor_pairs(string: &str) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = vec![];

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for caps in re.captures_iter(string) {
        let num1 = *&caps[1].parse::<i32>().unwrap(); // First captured group
        let num2 = *&caps[2].parse::<i32>().unwrap(); // Second captured group
        println!("Extracted: {}, {}", num1, num2);
        v.push((num1, num2));
    }

    v
}

pub fn parse(string: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut enabled: bool = true;

    let re = Regex::new(r"(do|don't)\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for caps in re.captures_iter(string) {
        // if it matched "do()" or "don't()"
        if let Some(keyword) = caps.get(1) {
            match keyword.as_str() {
                "do" => enabled = true,
                "don't" => enabled = false,
                _ => {}
            }
        } else {
            // if it matched "mul(###, ###)"
            match (caps.get(2), caps.get(3)) {
                (Some(num1), Some(num2)) => {
                    if enabled {
                        sum += num1.as_str().parse::<i32>().unwrap()
                            * num2.as_str().parse::<i32>().unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    return sum;
}
