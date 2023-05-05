use rand::Rng;
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug)]
pub struct Player {
    index: u8,
    name: String,
    human: bool,
    bankroll: i64,
    wager: i64,
    hand: Option<Vec<Card>>,
    hand_status: HandState,
}

#[derive(Debug)]
pub struct Dealer {
    decks: Option<Vec<Deck>>,
    hand: Option<Vec<Card>>,
}

#[derive(Debug)]
pub struct Deck {
    cards: Option<Vec<Card>>,
}

#[derive(Debug, Clone)]
pub struct Card {
    suit: String,
    face: String,
    value: u8,
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    dealer: Dealer,
    players: Option<Vec<Player>>,
}

#[derive(Debug)]
pub enum HandState {
    Idle,
    Win,
    Lose,
    BlackJack,
    Push,
}

#[derive(Debug)]
pub enum GameState {
    Idle,
    RoundStart,
    NewGame,
    PlaceBets,
    DealCards,
    PlayersTurn,
    DealerTurn,
    Pay,
    RoundEnd,
    GameOver,
}

pub fn go() {
    let hand = Some(Vec::<Card>::new());
    let dealer = Dealer { decks: None, hand };
    let mut game = Game {
        state: GameState::Idle,
        dealer,
        players: None,
    };
    game.new_game();
}

const DELAY: u64 = 500;

impl Dealer {
    pub fn display_hand(&mut self) {
        let dealer_hand = self.hand.as_ref().unwrap();
        for card in dealer_hand {
            if dealer_hand.len() == 1 {
                println!("Dealer card: {} of {}", card.face, card.suit);
            } else {
                println!("Dealer cards: {} of {}", card.face, card.suit);
            }
        }
        let dealer_value: (u8, u8) = {
            let mut sum: u8 = 0;
            let mut modified: u8 = 0;
            for card in dealer_hand {
                let modifier: u8 = if card.face == "Ace" { 10 } else { 0 };
                sum += card.value;
                modified += card.value + modifier;
            }
            (sum, modified)
        };
        {
            let (sum, modified) = dealer_value;
            if sum != modified && modified < 22 {
                println!("Dealer value: {} or {}", sum, modified);
            } else {
                println!("Dealer value: {}", sum);
            }
        }
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
    pub fn setup_decks(&mut self) {
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
    pub fn shuffle_decks(&mut self) {
        self.setup_decks();
        // The dealer's decks
        let decks = self.decks.as_mut().unwrap();
        // Loop around each dec in Vec<Deck>
        for deck in decks {
            // Unwrap the cards in the deck
            let cards = deck.cards.as_mut().unwrap();
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

impl Player {
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
    pub fn display_hand(&self) {
        let player_hand = self.hand.as_ref().unwrap();
        for card in player_hand {
            println!("{} cards: {} of {}", self.name, card.face, card.suit);
        }
        let player_value: (u8, u8) = {
            let mut sum: u8 = 0;
            let mut modified: u8 = 0;
            for card in player_hand {
                let modifier: u8 = if card.face == "Ace" { 10 } else { 0 };
                sum += card.value;
                modified += card.value + modifier;
            }
            (sum, modified)
        };
        {
            let (sum, modified) = player_value;
            if sum != modified && modified < 22 {
                println!("{} value: {} or {}", self.name, sum, modified);
            } else {
                println!("{} value: {}", self.name, sum);
            }
        }
    }
}

impl Game {
    /**
     * BlackJack.rs
     */
    pub fn new_game(&mut self) {
        self.state = GameState::NewGame;
        self.setup_players();
        self.dealer.shuffle_decks();
        self.game_loop();
    }
    pub fn game_loop(&mut self) {
        self.state = GameState::RoundStart;
        loop {
            self.check_players();
            match self.state {
                GameState::Idle => {}
                GameState::RoundStart => self.round_start(),
                GameState::NewGame => self.new_game(),
                GameState::PlaceBets => self.place_bets(),
                GameState::DealCards => self.deal_cards(),
                GameState::PlayersTurn => self.players_turn(),
                GameState::DealerTurn => self.dealer_turn(),
                GameState::Pay => self.pay(),
                GameState::RoundEnd => self.round_end(),
                GameState::GameOver => self.game_over(),
            }
            let time = std::time::Duration::from_millis(DELAY);
            std::thread::sleep(time);
        }
    }
    /**
     * STATE FUNCTIONS
     */
    pub fn check_players(&mut self) {
        let players = self.players.as_ref().unwrap();
        if players.len() == 0 {
            self.state = GameState::GameOver;
        }
    }
    pub fn round_start(&mut self) {
        println!("\n\n-+- New Round! -+-\n\n");
        self.state = GameState::PlaceBets;
    }
    pub fn place_bets(&mut self) {
        println!("Place Your Bets!");
        for player in self.players.as_mut().unwrap() {
            if player.human {
                loop {
                    println!("{}, how much would you like to wager?", player.name);
                    let mut amount: String = String::new();
                    // Get user input
                    std::io::stdin()
                        .read_line(&mut amount)
                        .expect("unable to read line");
                    // Parse result
                    match amount.trim().parse::<i64>() {
                        Ok(amount) => {
                            if amount < 5 {
                                println!("Wager amount must be greater than or equal to 5");
                                continue;
                            }
                            if amount > player.bankroll {
                                println!("You don't have enough money to wager {}", amount);
                                continue;
                            }
                            println!("{} will wager ${}", player.name, amount);
                            player.bankroll -= amount;
                            player.wager = amount;
                            println!("\n");
                            break;
                        }
                        Err(_) => {}
                    }
                }
            } else {
                // random bet for non-human
                let mut rng = rand::thread_rng();
                let amount = rng.gen_range(5..(player.bankroll as f64 * 0.1f64) as i64);
                println!("{} will wager ${}", player.name, amount);
                player.bankroll -= amount;
                player.wager = amount;
            }
        }
        self.state = GameState::DealCards;
    }
    pub fn deal_cards(&mut self) {
        println!("\nDealing Cards!");
        // Deal players first card
        let players = self.players.as_ref().unwrap();
        for i in 0..players.len() {
            self.deal_card(&(i as u8));
        }
        // Dealer card
        self.dealer_card();
        // Deal players second card
        let players = self.players.as_ref().unwrap();
        for i in 0..players.len() {
            self.deal_card(&(i as u8));
        }
        self.display_hands();
        self.state = GameState::PlayersTurn;
    }
    pub fn players_turn(&mut self) {
        println!("\nPlayers Turn!");
        for player in self.players.as_mut().unwrap() {
            'player_turn: loop {
                player.display_hand();
                if player.human {
                    // User interaction (hit, stand, etc)
                    let action = player.human_action();
                    if action.trim() == String::from('h') {
                        // TODO: how to make this work?
                        //self.deal_card();
                    } else if action.trim() == String::from('s') {
                        break 'player_turn;
                    }
                }
            }
            println!("\n");
        }
        self.state = GameState::DealerTurn;
    }
    pub fn dealer_turn(&mut self) {
        println!("\nDealer's Turn!");
        self.state = GameState::Pay;
    }
    pub fn pay(&mut self) {
        println!("\nPaying Out!");
        self.state = GameState::RoundEnd;
    }
    pub fn round_end(&mut self) {
        self.state = GameState::RoundStart;
        // Empty hands
        self.dealer.hand.as_mut().unwrap().clear();
        let players = self.players.as_mut().unwrap();
        for i in 0..players.len() {
            if players[i].bankroll <= 0 {
                players.remove(i);
                continue;
            }
            players[i].hand.as_mut().unwrap().clear();
        }
    }
    /**
     * HELPER FUNCTIONS
     */
    pub fn draw_card(&mut self) -> Option<Card> {
        let decks = self.dealer.decks.as_mut().unwrap();
        let mut card: Option<Card> = None;
        for i in 0..decks.len() {
            // Draw a card from a non-empty deck
            if decks[i].cards.as_ref().unwrap().len() > 0 {
                card = decks[i].cards.as_mut().unwrap().pop();
                break;
            }
        }
        card
    }
    pub fn display_hands(&mut self) {
        self.dealer.display_hand();
        println!("\n");
        for player in self.players.as_mut().unwrap() {
            player.display_hand();
            println!("\n");
        }
    }
    pub fn deal_card(&mut self, player_index: &u8) {
        // Attempt to draw a card
        if let Some(card) = self.draw_card() {
            let players = self.players.as_mut().unwrap();
            players[*player_index as usize]
                .hand
                .as_mut()
                .unwrap()
                .push(card)
        } else {
            // Deck is None -- time to shuffle
            self.dealer.shuffle_decks();
            self.deal_card(player_index);
        }
    }
    pub fn dealer_card(&mut self) {
        // Attempt to draw a card
        if let Some(card) = self.draw_card() {
            self.dealer.hand.as_mut().unwrap().push(card)
        } else {
            // Deck is None -- time to shuffle
            self.dealer.shuffle_decks();
            self.dealer_card();
        }
    }
    pub fn setup_players(&mut self) {
        let number_of_players = self.get_number_of_players();
        if number_of_players.is_err() {
            // Err, not a number, try again
            self.setup_players();
        } else {
            // Now we can create the players
            self.create_players(&number_of_players.unwrap());
        }
    }
    pub fn get_number_of_players(&mut self) -> Result<u8, ParseIntError> {
        println!("How many players will there be?");
        let mut number_of_players: String = String::new();
        // Get user input
        std::io::stdin()
            .read_line(&mut number_of_players)
            .expect("unable to read line");
        // Return the parse result
        number_of_players.trim().parse::<u8>()
    }
    pub fn create_players(&mut self, number_of_players: &u8) {
        // Create some ai / human players
        // Start by initializing a new vector of players
        self.players = Some(Vec::new());
        for i in 0..*number_of_players {
            let is_human: bool = self.ask_create_player(&i);
            // Add player to the game
            let hand = Some(Vec::<Card>::new());
            let player = Player {
                index: i,
                name: format!("Player {}", &i + 1),
                human: is_human,
                bankroll: 100,
                wager: 0,
                hand,
                hand_status: HandState::Idle,
            };
            self.add_player(player);
        }
    }
    pub fn ask_create_player(&mut self, player_index: &u8) -> bool {
        println!(
            "Player #{}: is this a computer (c) player or a human (h) player?",
            player_index + 1
        );
        let mut mode: String = String::new();
        // Get user input
        std::io::stdin()
            .read_line(&mut mode)
            .expect("unable to read line");
        // The only options are c or h
        if !["c", "h"].contains(&mode.as_str().to_lowercase().trim()) {
            self.ask_create_player(player_index);
        }
        // Return if the player is human based on input
        mode.contains("h")
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.as_mut().unwrap().push(player);
    }
    pub fn game_over(&mut self) {
        println!("\nGame over. Thanks for playing!\n");
        std::process::exit(0);
    }
}
