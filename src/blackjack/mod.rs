use std::num::ParseIntError;

#[derive(Debug)]
struct Player {
    human: bool,
    bankroll: i64,
    hand: Option<Cards>,
}

#[derive(Debug)]
struct Dealer {
    decks: Option<Vec<Deck>>,
    hand: Option<Cards>,
}

#[derive(Debug)]
struct Cards {
    cards: Option<Vec<Card>>,
}

#[derive(Debug)]
struct Deck {
    card: Option<Vec<Cards>>,
}

#[derive(Debug)]
struct Card {
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
    New,
    Shuffle,
    Bet,
    Deal,
    Pay,
}

pub fn go() {
    let mut game = Game {
        state: GameState::New,
        dealer: Dealer {
            decks: None,
            hand: None,
        },
        players: None,
    };
    game.setup_players();
    println!("{:#?}", game);
}

impl Game {
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
            self.add_player(&is_human);
        }
    }
    pub fn ask_create_player(&mut self, player_index: &u8) -> bool {
        println!(
            "Player # {}: is this a computer (c) player or a human (h) player?",
            player_index + 1
        );
        let mut mode: String = String::new();
        // Get user input
        std::io::stdin()
            .read_line(&mut mode)
            .expect("unable to read line");
        // The only options are c or h
        if !["c", "h"].contains(&mode.as_str().trim()) {
            self.ask_create_player(player_index);
        }
        // Return if the player is human based on input
        mode.contains("h")
    }
    pub fn add_player(&mut self, is_human: &bool) {
        self.players.as_mut().unwrap().push(Player {
            human: *is_human,
            bankroll: 100,
            hand: None,
        });
    }
}
