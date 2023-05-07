pub mod card;
pub mod dealer;
pub mod deck;
pub mod hand;
pub mod player;

use crate::game::dealer::Dealer;
use crate::game::player::Player;
use std::{thread, time};

const DELAY: u64 = 100;

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
    round_number: i64,
    players: Option<Vec<Player>>,
    dealer: Dealer,
    state: GameState,
    running: bool,
}
impl Game {
    pub fn new() -> Self {
        Game {
            round_number: 0,
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
        self.state = GameState::RoundStart;
    }
    pub fn round_start(&mut self) {
        println!(
            "\n------------------ Round {}! ------------------\n",
            self.round_number + 1
        );
        self.state = GameState::PlaceBets;
    }
    pub fn place_bets(&mut self) {
        println!("Place your bets\n");
        // This is the rust way of looping over an option of Vec<T>
        // The rust iterators are very pewerful
        self.players
            // Returns a mut iterator of Vec<Player>
            .iter_mut()
            // Flatten will unwrap all the Players in the Vec, droping None
            .flatten()
            // Filtering on all active players
            .filter(|player| player.active)
            // For each playaer, do xxx
            .for_each(|player| {
                if player.human {
                    self.dealer.ask_wager(player);
                } else {
                    player.computer_wager();
                }
                println!("{} will wager ${}", player.name, player.wager);
            });
        self.state = GameState::DealHands;
    }
    pub fn deal_hands(&mut self) {
        println!("\nDealing hands...\n");
        // Deal the first card
        self.players
            .iter_mut()
            .flatten()
            .filter(|player| player.active)
            .for_each(|player| self.dealer.deal_card(player));
        // A card for the dealer
        self.dealer.dealer_card();
        self.players
            .iter_mut()
            .flatten()
            .filter(|player| player.active)
            .for_each(|player| self.dealer.deal_card(player));
        self.state = GameState::PlayersTurn;
    }
    pub fn players_turn(&mut self) {
        println!("Players turn...\n");
        self.players
            .iter_mut()
            .flatten()
            .filter(|player| player.active)
            .for_each(|player| self.dealer.player_turn(player));
        self.state = GameState::DealerTurn;
    }
    pub fn dealer_turn(&mut self) {
        println!("Dealer's turn...\n");
        self.dealer.dealer_turn();
        self.players
            .iter_mut()
            .flatten()
            .filter(|player| player.active)
            .for_each(|player| self.dealer.hand_status(player));
        self.state = GameState::Payout;
    }
    pub fn payout(&mut self) {
        self.players
            .iter_mut()
            .flatten()
            .filter(|player| player.active)
            .for_each(|player| {
                self.dealer.payout(player);
                println!("{} bankroll ${}", player.name, player.bankroll);
            });
        self.state = GameState::RoundEnd;
    }
    pub fn round_end(&mut self) {
        self.players
            .iter_mut()
            .flatten()
            .filter(|player| player.active)
            .for_each(|player| {
                if player.bankroll <= 5 {
                    println!("{} has been eliminated", player.name);
                    self.dealer.remove_player(player);
                }
                // Clear player hand
                player.hand.clear();
            });
        // Clear dealers hand
        self.dealer.hand.clear();
        let players = self
            .players
            .as_ref()
            .unwrap()
            .iter()
            .filter(|x| x.active)
            .count();
        if players == 0 {
            println!("There are no players remaining. Game over.\n");
            std::process::exit(0);
        }
        self.round_number += 1;
        let sleep = time::Duration::from_millis(DELAY);
        thread::sleep(sleep);
        self.state = GameState::RoundStart;
    }

    /**
     * Setup players by adding them to the game
     */
    pub fn setup_players(&mut self) {
        let number = self.dealer.number_of_players();
        for i in 0..number {
            self.add_player(Player::new(String::from(format!("Player {}", i + 1))));
        }
        let players = self.players.as_mut().unwrap();
        for player in players {
            self.dealer.human_or_computer(player);
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
}
