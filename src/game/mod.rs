use std::io;

#[derive(Debug)]
enum GameState {
    Idle,
    NewGame,
    RoundStart,
    PlaceBets,
    DealHands,
    PlayersTurm,
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
                GameState::PlayersTurm => self.players_turn(),
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
        self.setup_players();
        self.human_or_computer();
        self.state = GameState::RoundStart;
    }
    pub fn round_start(&mut self) {}
    pub fn place_bets(&mut self) {}
    pub fn deal_hands(&mut self) {}
    pub fn players_turn(&mut self) {}
    pub fn dealer_turn(&mut self) {}
    pub fn payout(&mut self) {}
    pub fn round_end(&mut self) {}

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
                if !["c", "h"].contains(&mode.as_str().to_lowercase().trim()) {
                    continue;
                }
                // Return if the player is human based on input
                if mode.contains("h") {
                    player.is_human(true);
                    break;
                } else {
                    player.is_human(false);
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
    pub fn is_human(&mut self, is_human: bool) {
        self.human = is_human;
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
    pub fn create_decks() {}
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
    pub fn get_total() {}
    pub fn display() {}
}

pub fn main() {
    let mut game = Game::new();
    game.run();
}
