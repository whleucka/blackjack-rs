use crate::game::card::Card;
use crate::game::deck::Deck;
use crate::game::hand::{Hand, HandState};
use crate::game::player::Player;

use rand::Rng;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Dealer {
    pub decks: Option<Vec<Deck>>,
    pub hand: Hand,
}
impl Dealer {
    pub fn new() -> Self {
        Dealer {
            decks: Some(Vec::<Deck>::new()),
            hand: Hand::new(),
        }
    }
    /**
     * Remove a player from the game
     */
    pub fn remove_player(&mut self, player: &mut Player) {
        player.active = false;
    }
    pub fn hand_status(&mut self, player: &mut Player) {
        if player.hand.state == HandState::Blackjack || player.hand.state == HandState::Lose {
            return;
        }
        let player_total = player.hand.get_total_single();
        let dealer_total = self.hand.get_total_single();
        if player_total > dealer_total {
            player.hand.state = HandState::Win;
        } else if player_total == dealer_total {
            player.hand.state = HandState::Push;
        } else {
            player.hand.state = HandState::Lose;
        }
    }
    pub fn payout(&mut self, player: &mut Player) {
        println!("\n");
        match player.hand.state {
            HandState::Idle => {}
            HandState::Push => {
                println!("{}, push!", player.name);
                player.set_pay(player.wager);
            }
            HandState::Win => {
                println!("{}, you win ${}", player.name, player.wager * 2);
                player.set_pay(player.wager * 2);
            }
            HandState::Lose => {
                println!("{}, you lose ${}", player.name, player.wager);
                player.set_lose(player.wager);
            }
            HandState::Blackjack => {
                println!("{}, you win ${}", player.name, player.wager * 3);
                player.set_pay(player.wager * 3);
            }
        }
        player.clear_wager();
        player.hand.state = HandState::Idle;
        self.hand.state = HandState::Idle;
    }
    /**
     * As player for wager
     */
    pub fn ask_wager(&mut self, player: &mut Player) {
        loop {
            println!("{}, how much would you like to wager?", player.name);
            println!("Current bankroll: ${}", player.bankroll);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("couldn't read line");
            let number = response.trim().parse::<i64>();
            if number.is_ok() {
                let wager = number.unwrap();
                if wager == 0 {
                    println!("Wager must be greater than 0")
                } else if wager > player.bankroll {
                    println!("You don't have that much to wager")
                } else {
                    player.set_wager(wager);
                    break;
                }
            } else {
                println!("Not a number, please try again");
            }
        }
    }
    /**
     * Ask for the number of players
     */
    pub fn human_or_computer(&mut self, player: &mut Player) {
        loop {
            println!("{}: are you Human (h) or Computer (c)?", player.name);
            let mut mode: String = String::new();
            // Get user input
            std::io::stdin()
                .read_line(&mut mode)
                .expect("unable to read line");
            // The only options are c or h
            if !["h", "c"].contains(&mode.as_str().to_lowercase().trim()) {
                continue;
            }
            // Return if the player is human based on input
            if mode.contains("h") {
                player.set_human(true);
                break;
            } else {
                player.set_human(false);
                break;
            }
        }
    }
    pub fn number_of_players(&self) -> u8 {
        println!("How many players are playing?");
        loop {
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("couldn't read line");
            let number = response.trim().parse::<u8>();
            if number.is_ok() {
                let number = number.unwrap();
                if number == 0 {
                    println!("Number of players must be greater than 0")
                } else if number > 8 {
                    println!("Number of players must be 8 or less")
                } else {
                    return number;
                }
            } else {
                println!("Not a number, please try again");
            }
        }
    }
    pub fn dealer_turn(&mut self) {
        loop {
            let total = self.hand.get_total_single();
            println!("\nDealer hand:");
            self.hand.display();
            self.hand.display_total();
            if total < 17 {
                self.dealer_card();
            } else if total > 21 {
                println!("Dealer bust\n");
                self.hand.state = HandState::Lose;
                break;
            } else if total == 21 && self.hand.count() == 2 {
                println!("Dealer blackjack!\n");
                self.hand.state = HandState::Blackjack;
                break;
            } else {
                println!("Dealer stand\n");
                // stand
                break;
            }
        }
    }
    pub fn player_turn(&mut self, player: &mut Player) {
        println!("{}, it is your turn:", player.name);
        loop {
            println!("\nDealer hand:");
            self.hand.display();
            self.hand.display_total();
            println!("\n{} hand:", player.name);
            player.hand.display();
            player.hand.display_total();
            let total = player.hand.get_total_single();
            if total > 21 {
                println!("{} bust\n", player.name);
                player.hand.state = HandState::Lose;
                break;
            } else if total == 21 && player.hand.count() == 2 {
                println!("{} blackjack!\n", player.name);
                player.hand.state = HandState::Blackjack;
                break;
            }
            println!("\n");
            let action = if player.human {
                player.human_action()
            } else {
                player.computer_action()
            };
            if action.trim().to_lowercase() == "h" {
                println!("{} hit\n", player.name);
                self.deal_card(player);
            } else if action.trim().to_lowercase() == "s" {
                println!("{} stand\n", player.name);
                break;
            }
        }
    }
    /**
     * Deal a card from the deck to the dealer
     */
    pub fn dealer_card(&mut self) {
        // Attempt to draw a card
        if let Some(card) = self.draw_card() {
            self.hand.cards.as_mut().unwrap().push(card);
        } else {
            // Deck is None -- time to shuffle
            self.shuffle_decks();
            self.dealer_card();
        }
    }
    /**
     * Deal a card from the deck to a player
     */
    pub fn deal_card(&mut self, player: &mut Player) {
        // Attempt to draw a card
        if let Some(card) = self.draw_card() {
            player.hand.cards.as_mut().unwrap().push(card);
        } else {
            // Deck is None -- time to shuffle
            self.shuffle_decks();
            self.deal_card(player);
        }
    }
    /**
     * Draw a card from the deck
     */
    pub fn draw_card(&mut self) -> Option<Card> {
        let decks = self.decks.as_mut().unwrap();
        let mut card: Option<Card> = None;
        for i in 0..decks.len() {
            // Draw a card from a non-empty deck
            if decks[i].cards.as_ref().unwrap().len() > 0 {
                card = decks[i].cards.as_mut().unwrap().pop();
                break;
            } else if i == decks.len() - 1 {
                return None;
            }
        }
        card
    }
    pub fn create_decks(&mut self) {
        let number_of_decks: u8 = 6;
        // Create new Vec<Deck>
        let mut decks = Vec::new();
        // Create n decks
        for _i in 0..number_of_decks {
            let deck = self.create_deck();
            decks.push(deck);
        }
        // Assign the dealer some decks
        self.decks = Some(decks);
    }
    pub fn create_deck(&mut self) -> Deck {
        // Pips are not used, they are the symbols on the Card
        // Suit are the symbols on the cards
        let suits = vec!["Hearts", "Diamonds", "Spades", "Clubs"];
        // Face value hash map
        let face_values = HashMap::from([
            ("Ace", 1),
            ("Two", 2),
            ("Three", 3),
            ("Four", 4),
            ("Five", 5),
            ("Six", 6),
            ("Seven", 7),
            ("Eight", 8),
            ("Nine", 9),
            ("Ten", 10),
            ("Jack", 10),
            ("Queen", 10),
            ("King", 10),
        ]);
        // Empty card vector, this will be the card deck
        let mut cards: Vec<Card> = Vec::new();

        for suit in suits {
            // for each face value
            for fv in face_values.iter() {
                let (face, value) = fv;
                let card = Card {
                    suit: String::from(suit),
                    face: String::from(*face),
                    value: *value,
                };
                cards.push(card);
            }
        }
        // Return a deck with some cards
        Deck { cards: Some(cards) }
    }
    pub fn shuffle_decks(&mut self) {
        self.create_decks();
        // The dealer's decks
        let decks = self.decks.as_mut().expect("decks are none");
        // Loop around each dec in Vec<Deck>
        for deck in decks {
            // Unwrap the cards in the deck
            let cards = deck.cards.as_mut().expect("cards are none");
            let mut rng = rand::thread_rng();
            let mut temp: Vec<Card> = Vec::new();
            // Rearrange cards
            while cards.len() > 0 {
                let idx = rng.gen_range(0..=cards.len() - 1);
                let card = cards.get(idx).expect("card index doesn't exist").clone();
                temp.push(card);
                cards.remove(idx);
            }
            // The shuffled deck
            let shuffled_deck = Deck { cards: Some(temp) };
            // Assign the dealer's deck to the shuffled deck
            *deck = shuffled_deck;
        }
    }
}
