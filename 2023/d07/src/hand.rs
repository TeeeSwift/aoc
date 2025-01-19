use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Hand {
    pub rank: Option<usize>,
    pub cards: Vec<usize>,
    pub bet: usize,
}

impl Hand {
    pub fn new(cards: Vec<char>, bet: usize) -> Self {
        let mut v: Vec<usize> = vec![];

        for card in cards {
            v.push(match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => card.to_digit(10).unwrap() as usize,
            })
        }

        Self {
            cards: v,
            bet,
            rank: None,
        }
    }

    pub fn get_rank(&mut self) {
        let mut m: HashMap<usize, usize> = HashMap::new();
        for &c in &self.cards {
            m.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut values = m.values().cloned().collect::<Vec<usize>>();
        values.sort();
        values.reverse();

        self.rank = Some(match (values.first(), values.get(1)) {
            (Some(5), _) => 6,
            (Some(4), _) => 5,
            (Some(3), Some(2)) => 4,
            (Some(3), _) => 3,
            (Some(2), Some(2)) => 2,
            (Some(2), _) => 1,
            _ => 0,
        });
    }

    pub fn j_to_joker(&mut self) {
        while let Some(card) = self.cards.iter_mut().find(|h| **h == 11) {
            *card = 0;
        }
    }

    pub fn get_rank_with_wildcard(&mut self) {
        let mut m: HashMap<usize, usize> = HashMap::new();
        for &c in &self.cards {
            m.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        if let Some(joker_count) = m.remove(&0) {
            let mut values = m.values().cloned().collect::<Vec<usize>>();
            values.sort();
            values.reverse();

            if values.len() < 2 {
                self.rank = Some(6);
                return;
            };

            if values.len() == 2 {
                self.rank = match (values[0], values[1]) {
                    (3, 1) => Some(5),
                    (2, 2) => Some(4),
                    (2, 1) => Some(5),
                    (1, 1) => Some(5),
                    (1, _) => Some(joker_count),
                    _ => Some(0),
                }
            };

            if values.len() == 3 {
                self.rank = match (values[0], values[1]) {
                    (2, 1) => Some(3),
                    (1, 1) => Some(3),
                    _ => Some(0),
                }
            }

            if values.len() == 4 {
                self.rank = Some(1);
            };
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::parse_input;

    #[test]
    fn t_get_rank() -> Result<()> {
        let mut hands = parse_input("src/sample")?;

        let a = hands
            .iter_mut()
            .map(|h| {
                h.get_rank();
                h.rank.unwrap()
            })
            .collect::<Vec<usize>>();

        assert_eq!(a, vec![1, 3, 2, 2, 3]);

        Ok(())
    }

    #[test]
    fn t_sort_hands() -> Result<()> {
        let mut hands = parse_input("src/sample")?;

        hands.iter_mut().for_each(|h| h.get_rank());
        hands.sort();
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

        assert_eq!(total, 6440);

        Ok(())
    }

    #[test]
    fn t_j_to_joker() {
        let mut hand = super::Hand::new(vec!['J', 'J'], 0);
        hand.j_to_joker();

        assert!(!hand.cards.contains(&11));
    }

    #[test]
    fn t_wildcard() -> Result<()> {
        let mut hands = vec![
            super::Hand::new(vec!['K', 'K', 'J'], 0),
            super::Hand::new(vec!['K', 'K', 'J', 'J'], 0),
            super::Hand::new(vec!['K', 'K', 'K', 'J', 'J'], 0),
            super::Hand::new(vec!['K', 'T', 'J'], 0),
            super::Hand::new(vec!['J', 'J', 'J', 'J', 'J'], 0),
            super::Hand::new(vec!['2', '3', '4', '5', '6'], 0),
            super::Hand::new(vec!['2', '3', '4', '5', 'J'], 0),
        ];
        let mut hands = parse_input("src/sample")?;

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

        let mut i: usize = 1;

        for hand in hands {
            total += hand.bet * i;
            i += 1;
        }

        println!("{total}");

        assert_eq!(total, 5905);

        Ok(())
    }
}
