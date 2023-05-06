pub mod card;
pub mod dealer;
pub mod deck;
pub mod hand;
pub mod player;

use crate::game::dealer::Dealer;
use crate::game::player::Player;
use std::io;

pub fn main() {
    let mut game = Game::new();
    game.run();
}

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
        let players = self.players.as_mut().unwrap();
        for player in players {
            self.dealer.player_turn(player);
        }
        self.state = GameState::DealerTurn;
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
