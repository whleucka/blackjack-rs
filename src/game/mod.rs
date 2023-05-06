use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum GameState {
    Idle,
    NewGame,
    RoundStart,
    PlaceBets,
    DealHands,
    PlayersTurn,
    DealerTurn,
    Payout,
    RoundEnd,
}

#[derive(Debug)]
struct Game {
    players: Option<Vec<Player>>,
    dealer: Dealer,
    state: GameState,
    running: bool,
}
impl Game {
    pub fn new() -> Self {
        Game {
            players: Some(Vec::<Player>::new()),
            dealer: Dealer::new(),
            state: GameState::Idle,
            running: true,
        }
    }
    /**
     * Run the game
     */
    pub fn run(&mut self) {
        // Start a new game
        self.state = GameState::NewGame;
        self.game_loop();
    }
    /**
     * The main game loop
     */
    pub fn game_loop(&mut self) {
        while self.running {
            match self.state {
                GameState::Idle => {}
                GameState::NewGame => self.new_game(),
                GameState::RoundStart => self.round_start(),
                GameState::PlaceBets => self.place_bets(),
                GameState::DealHands => self.deal_hands(),
                GameState::PlayersTurn => self.players_turn(),
                GameState::DealerTurn => self.dealer_turn(),
                GameState::Payout => self.payout(),
                GameState::RoundEnd => self.round_end(),
            }
        }
    }

    /**
     * State methods
     */
    pub fn new_game(&mut self) {
        println!("Welcome to blackjack.js\n");
        self.dealer.create_decks();
        self.dealer.shuffle_decks();
        self.setup_players();
        self.human_or_computer();
        self.state = GameState::RoundStart;
    }
    pub fn round_start(&mut self) {
        println!("\n** Round start! **\n");
        self.state = GameState::PlaceBets;
    }
    pub fn place_bets(&mut self) {
        println!("Place your bets\n");
        self.state = GameState::DealHands;
    }
    pub fn deal_hands(&mut self) {
        println!("Dealing hands...\n");
        // Deal the first card
        let players = self.players.as_mut().unwrap();
        for player in players {
            self.dealer.deal_card(player);
        }
        // A card for the dealer
        self.dealer.dealer_card();
        let players = self.players.as_mut().unwrap();
        for player in players {
            self.dealer.deal_card(player);
        }
        self.state = GameState::PlayersTurn;
    }
    pub fn players_turn(&mut self) {
        let players = self.players.as_ref().unwrap();
        for i in 0..players.len() {
            self.player_turn(&i);
        }
        self.state = GameState::DealerTurn;
    }
    pub fn player_turn(&mut self, index: &usize) {
        let players = self.players.as_mut().unwrap();
        let player = &mut players[*index];
        println!("{}, it is your turn:", player.name);
        loop {
            println!("\nDealer hand:");
            self.dealer.hand.display();
            self.dealer.hand.display_total();
            println!("\n{} hand:", player.name);
            player.hand.display();
            player.hand.display_total();
            println!("\n");
            let action = if player.human {
                player.human_action()
            } else {
                player.computer_action()
            };
            if action.trim().to_lowercase() == "h" {
                self.dealer.deal_card(player);
            } else if action.trim().to_lowercase() == "s" {
                break;
            }
        }
    }
    pub fn dealer_turn(&mut self) {
        panic!("wip");
        self.state = GameState::Payout;
    }
    pub fn payout(&mut self) {
        panic!("wip");
        self.state = GameState::RoundEnd;
    }
    pub fn round_end(&mut self) {
        panic!("wip");
        self.state = GameState::RoundStart;
    }

    /**
     * Ask for the number of players
     */
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
    pub fn human_or_computer(&mut self) {
        for player in self.players.as_mut().unwrap() {
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
    }
    /**
     * Setup players by adding them to the game
     */
    pub fn setup_players(&mut self) {
        let number = self.number_of_players();
        for i in 0..number {
            self.add_player(Player::new(String::from(format!("Player {}", i + 1))));
        }
    }
    /**
     * Add a player to the game
     */
    pub fn add_player(&mut self, player: Player) {
        self.players
            .as_mut()
            .expect("players should not be empty")
            .push(player)
    }
    /**
     * Remove a player from the game
     */
    pub fn remove_player(&mut self, player: Player) {
        // Position returns an index, and we can compare the struct
        // using PartialEq trait
        let index = self
            .players
            .as_mut()
            .unwrap()
            .iter()
            .position(|x| *x == player)
            .unwrap();
        // If the order is not important, use swap replace O(1) vs remove O(n)
        self.players.as_mut().unwrap().swap_remove(index);
    }
}

#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    hand: Hand,
    human: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            hand: Hand::new(),
            human: true,
        }
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

#[derive(Debug)]
struct Dealer {
    decks: Option<Vec<Deck>>,
    hand: Hand,
}
impl Dealer {
    pub fn new() -> Self {
        Dealer {
            decks: Some(Vec::<Deck>::new()),
            hand: Hand::new(),
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
    pub fn shuffle_decks(&mut self) {}
}

#[derive(Debug, PartialEq)]
struct Card {
    suit: String,
    face: String,
    value: u8,
}
impl Card {}

#[derive(Debug)]
struct Deck {
    cards: Option<Vec<Card>>,
}
impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: Some(Vec::<Card>::new()),
        }
    }
    pub fn shuffle_decks() {}
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Option<Vec<Card>>,
}
impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Some(Vec::<Card>::new()),
        }
    }
    /**
     * Return hand count
     */
    pub fn count(&mut self) -> usize {
        self.cards.as_ref().unwrap().len()
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

pub fn main() {
    let mut game = Game::new();
    game.run();
}
