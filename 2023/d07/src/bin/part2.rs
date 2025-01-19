use anyhow::Result;
use d07::*;

pub fn main() -> Result<()> {
    let mut hands = parse_input("src/input")?;

    hands.iter_mut().for_each(|h| {
        h.get_rank();
        h.j_to_joker();
        h.get_rank_with_wildcard();
    });

    hands.sort();

    for hand in &hands {
        println!("{hand:?}");
    }

    let mut total: usize = 0;

    let mut prev: Vec<usize> = vec![];
    let mut i: usize = 1;

    for hand in hands {
        if format!("{:?}", hand.cards) == format!("{:?}", prev) {
            total += i * hand.bet;
            prev = hand.cards.clone();
            continue;
        }

        total += hand.bet * i;
        prev = hand.cards.clone();
        i += 1;
    }

    println!("{total}");

    Ok(())
}
