use crate::game::card::Card;
use crate::game::hand::Hand;

#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub human: bool,
    pub bankroll: i64,
    pub wager: i64,
}
impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            hand: Hand::new(),
            human: true,
            bankroll: 1000,
            wager: 0,
        }
    }
    pub fn clear_wager(&mut self) {
        self.wager = 0;
    }
    pub fn set_human(&mut self, is_human: bool) {
        self.human = is_human;
    }
    pub fn give_card(&mut self, card: Card) {
        self.hand.cards.as_mut().unwrap().push(card);
    }
    pub fn human_action(&self) -> String {
        println!("{}: hit (h) or stand (s)?", self.name);
        let mut action: String = String::new();
        // Get user input
        std::io::stdin()
            .read_line(&mut action)
            .expect("unable to read line");
        if !["h", "s"].contains(&action.as_str().to_lowercase().trim()) {
            self.human_action();
        }
        action
    }
    pub fn computer_action(&self) -> String {
        todo!("implement me");
    }
}
