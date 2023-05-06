use crate::game::card::Card;

#[derive(Debug, PartialEq)]
pub struct Hand {
    pub state: HandState,
    pub cards: Option<Vec<Card>>,
}
#[derive(Debug, PartialEq)]
pub enum HandState {
    Idle,
    Win,
    Lose,
    Blackjack,
}
impl Hand {
    pub fn new() -> Self {
        Hand {
            state: HandState::Idle,
            cards: Some(Vec::<Card>::new()),
        }
    }
    /**
     * Return hand count
     */
    pub fn count(&mut self) -> usize {
        self.cards.as_ref().unwrap().len()
    }
    pub fn clear(&mut self) {
        self.cards = Some(Vec::<Card>::new());
    }
    pub fn get_total(&mut self) -> (u8, u8) {
        let cards = self.cards.as_ref().unwrap();
        let mut total = (0, 0);
        for card in cards {
            let special = if card.face == "Ace" { 10 } else { 0 };
            total.0 += card.value;
            total.1 += card.value + special;
        }
        total
    }
    pub fn get_total_single(&mut self) -> u8 {
        let total = self.get_total();
        let (sum, special) = total;
        if special < 22 && special > sum {
            return special;
        }
        sum
    }
    /**
     * Print hand total
     */
    pub fn display_total(&mut self) {
        let (sum, special) = self.get_total();
        if special < 22 && special > sum {
            println!("Total: {} or {}", sum, special);
        } else {
            println!("Total: {}", sum)
        }
    }
    /**
     * Print hand
     */
    pub fn display(&mut self) {
        if let Some(cards) = &self.cards {
            for card in cards {
                if cards.len() == 1 {
                    println!("{} of {}", card.face, card.suit);
                } else {
                    println!("{} of {}", card.face, card.suit);
                }
            }
        }
    }
}
