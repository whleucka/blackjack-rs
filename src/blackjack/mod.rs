use rand::Rng;
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug)]
struct Player {
    name: String,
    human: bool,
    bankroll: i64,
    hand: Option<Vec<Card>>,
}

#[derive(Debug)]
struct Dealer {
    decks: Option<Vec<Deck>>,
    hand: Option<Vec<Card>>,
}

#[derive(Debug)]
struct Deck {
    cards: Option<Vec<Card>>,
}

#[derive(Debug, Clone)]
struct Card {
    suit: String,
    face: String,
    value: u8,
}

#[derive(Debug)]
struct Game {
    state: GameState,
    dealer: Dealer,
    players: Option<Vec<Player>>,
}

#[derive(Debug)]
enum GameState {
    None,
    RoundStart,
    NewGame,
    ShuffleDecks,
    PlaceBets,
    DealCards,
    PlayersTurn,
    DealersTurn,
    Pay,
}

pub fn go() {
    let mut game = Game {
        state: GameState::None,
        dealer: Dealer {
            decks: None,
            hand: None,
        },
        players: None,
    };
    game.setup_players();
    game.shuffle_decks();
    println!("{:#?}", game);
    //game.new_game();
}

impl Game {
    pub fn new_game(&mut self) {
        self.state = GameState::NewGame;
        'game_loop: loop {
            self.state = GameState::RoundStart;
        }
    }
    pub fn deal_cards(&mut self) {
        self.state = GameState::DealCards;
    }
    pub fn place_bets(&mut self) {
        self.state = GameState::PlaceBets;
    }
    pub fn players_turn(&mut self) {
        self.state = GameState::PlayersTurn;
        'players: loop {}
    }
    pub fn dealers_turn(&mut self) {
        self.state = GameState::DealersTurn;
        'dealers: loop {}
    }
    pub fn draw_card(&mut self) -> Option<Card> {
        let decks = self.dealer.decks.as_mut().unwrap();
        let deck = decks.pop()?;
        deck.cards?.pop()
    }
    pub fn shuffle_decks(&mut self) {
        self.state = GameState::ShuffleDecks;
        self.setup_decks();
        println!("\n*** Shuffle! ***\n");
        // The dealer's decks
        let decks = self.dealer.decks.as_mut().unwrap();
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
    pub fn pay(&mut self) {
        self.state = GameState::Pay;
    }
    pub fn setup_decks(&mut self) {
        let number_of_decks: u8 = 6;
        // Create new Vec<Deck>
        let mut decks = Vec::new();
        // Create n decks
        for i in 0..number_of_decks {
            let deck = self.create_deck();
            decks.push(deck);
        }
        // Assign the dealer some decks
        self.dealer.decks = Some(decks);
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
        Deck { cards: Some(cards) }
    }
    pub fn setup_players(&mut self) {
        let number_of_players = self.get_number_of_players();
        if number_of_players.is_err() {
            // This might not be a number, try again
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
            self.add_player(&is_human, &format!("Player {}", &i + 1));
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
    pub fn add_player(&mut self, is_human: &bool, name: &str) {
        // Add player to the game
        self.players.as_mut().unwrap().push(Player {
            name: String::from(name),
            human: *is_human,
            bankroll: 100,
            hand: Some(Vec::<Card>::new()),
        });
    }
}
