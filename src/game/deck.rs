use crate::game::card::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Option<Vec<Card>>,
}
impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: Some(Vec::<Card>::new()),
        }
    }
}
