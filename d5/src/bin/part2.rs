use d5::parser;
use d5::validator::{find_mistake, fix};

// part 1
fn main() -> std::io::Result<()> {
    let (rules, updates) = parser::parse_input()?;
    let mut invalid: Vec<Vec<u16>> = vec![];

    // generate vector of invalid ones
    for update in updates.iter() {
        println!("checking update: {:?}", update);

        if find_mistake(&rules, update).is_some() {
            println!("invalid, adding to fixable updates vector");
            invalid.push(update.clone())
        }
    }

    let mut total: u16 = 0;
    // loop over invalid updates
    for update in invalid.iter_mut() {
        let mut mistake = find_mistake(&rules, update);

        while mistake.is_some() {
            let rule = mistake.unwrap();
            println!("BEFORE: update: {:?}, mistake: {:?}", update, rule);

            fix(rule, update);
            mistake = find_mistake(&rules, update);
        }

        total += update[update.len() / 2];
    }

    println!("total: {}", total);

    Ok(())
}
