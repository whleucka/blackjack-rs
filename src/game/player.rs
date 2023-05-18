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
            bankroll: 100,
            wager: 0,
        }
    }
    pub fn set_pay(&mut self, amount: i64) {
        self.bankroll += amount;
    }
    pub fn set_wager(&mut self, wager: i64) {
        self.wager = wager;
    }
    pub fn clear_wager(&mut self) {
        self.wager = 0;
    }
    pub fn computer_wager(&mut self) {
        let mut rng = rand::thread_rng();
        let pct: f64 = 0.05;
        let max: i64 = 100;
        let bet = self.bankroll as f64 * pct;
        let mut upper = bet.floor() as i64;
        if upper <= 5 {
            upper = 10;
        }
        let mut wager = rng.gen_range(5..upper);
        if wager > self.bankroll {
            wager = self.bankroll;
        }
        self.set_wager(wager.min(max));
    }
    pub fn set_human(&mut self, is_human: bool) {
        self.human = is_human;
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
        let mut _action: String = String::new();
        let total = self.hand.get_total_single();
        // TODO: implement strategy lookup from HashMap
        if total < 17 {
            _action = String::from("h");
        } else {
            _action = String::from("s");
        }
        _action
    }
}
