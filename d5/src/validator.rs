pub fn check(rule: &(u16, u16), update: &Vec<u16>) -> bool {
    let (num1, num2) = rule;
    let result1 = update.iter().position(|x| x == num1);
    let result2 = update.iter().position(|x| x == num2);

    let validity = match (result1, result2) {
        (Some(index1), Some(index2)) if index1 < index2 => true,
        (Some(index1), Some(index2)) if index1 > index2 => false,
        _ => true,
    };

    println!(
        "rule: {:?}, update: {:?}, valid: {}",
        rule, update, validity
    );

    validity
}

pub fn find_mistake(rules: &Vec<(u16, u16)>, update: &Vec<u16>) -> Option<(u16, u16)> {
    for rule in rules.iter() {
        if !check(&rule, &update) {
            return Some(*rule);
        }
    }
    None
}

pub fn fix(rule: (u16, u16), update: &mut Vec<u16>) {
    let (num1, num2) = rule;
    let index_of_num1 = update.iter().position(|x| *x == num1).unwrap();
    update.remove(index_of_num1);

    let index_of_num2 = update.iter().position(|x| *x == num2).unwrap();
    update.insert(index_of_num2, num1);

    println!("After: update: {:?}", update);
}
