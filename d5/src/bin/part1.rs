use d5::parser;
use d5::validator::check;

// part 1
fn main() -> std::io::Result<()> {
    let (rules, updates) = parser::parse_input()?;
    let mut total: u16 = 0;

    'update_loop: for update in updates.iter() {
        println!("checking update: {:?}", update);
        for rule in rules.iter() {
            if !check(&rule, &update) {
                continue 'update_loop; // Skip to the next update
            }
        }
        total += update[update.len() / 2];
    }

    println!("{}", total);

    Ok(())
}
