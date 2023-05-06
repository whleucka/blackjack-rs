use crate::game::card::Card;
use crate::game::hand::Hand;

use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub human: bool,
    pub bankroll: i64,
    pub wager: i64,
    pub active: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        Player {
            active: true,
            name,
            hand: Hand::new(),
            human: true,
            bankroll: 1000,
            wager: 0,
        }
    }
    pub fn set_pay(&mut self, amount: i64) {}
    pub fn set_wager(&mut self, wager: i64) {
        self.wager = wager;
        self.bankroll -= wager;
    }
    pub fn clear_wager(&mut self) {
        self.wager = 0;
    }
    pub fn computer_wager(&mut self) {
        let mut rng = rand::thread_rng();
        let mut upper = (self.bankroll as f64 * 0.1f64).floor() as i64;
        if upper <= 5 {
            upper = 10;
        }
        let wager = rng.gen_range(5..upper);
        self.set_wager(wager);
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
    pub fn computer_action(&mut self) -> String {
        println!("{}: hit (h) or stand (s)?", self.name);
        let mut action: String = String::new();
        let total = self.hand.get_total_single();
        // TODO: implement strategy lookup from HashMap
        if total < 17 {
            action = String::from("h");
        } else {
            action = String::from("s");
        }
        action
    }
}
