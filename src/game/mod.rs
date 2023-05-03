use colored::Colorize;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: String,
    pub face: String,
    pub value: i32,
    pub special: i32,
}

#[derive(Debug)]
struct Player {
    pub balance: i32,
    hand: Option<Vec<Card>>,
}

#[derive(Debug)]
struct Casino {
    deck: Option<Vec<Card>>,
    hand: Option<Vec<Card>>,
    player: Player,
    pot: i32,
    chart: HashMap<i32, Vec<char>>,
    header: HashMap<i32, i32>,
}

pub fn run() {
    // Game variable init
    let mut casino = Casino {
        deck: None,
        hand: None,
        pot: 0,
        chart: HashMap::from([
            (2, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (3, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (4, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (5, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (6, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (7, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (8, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (9, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (10, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (11, vec!['h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h']),
            (12, vec!['h', 'h', 's', 's', 's', 'h', 'h', 'h', 'h', 'h']),
            (13, vec!['s', 's', 's', 's', 's', 'h', 'h', 'h', 'h', 'h']),
            (14, vec!['s', 's', 's', 's', 's', 'h', 'h', 'h', 'h', 'h']),
            (15, vec!['s', 's', 's', 's', 's', 'h', 'h', 'h', 'h', 'h']),
            (16, vec!['s', 's', 's', 's', 's', 'h', 'h', 'h', 'h', 'h']),
            (17, vec!['s', 's', 's', 's', 's', 's', 's', 's', 's', 's']),
            (18, vec!['s', 's', 's', 's', 's', 's', 's', 's', 's', 's']),
            (19, vec!['s', 's', 's', 's', 's', 's', 's', 's', 's', 's']),
            (20, vec!['s', 's', 's', 's', 's', 's', 's', 's', 's', 's']),
            (21, vec!['s', 's', 's', 's', 's', 's', 's', 's', 's', 's']),
        ]),
        header: HashMap::from([
            (1, 9),
            (2, 0),
            (3, 1),
            (4, 2),
            (5, 3),
            (6, 4),
            (7, 5),
            (8, 6),
            (9, 7),
            (10, 8),
        ]),
        player: Player {
            balance: 1000,
            hand: None,
        },
    };

    // Game starts
    casino.shuffle_deck();
    'game_loop: loop {
        let div = "--------------------------------------------------------------------".blue();
        println!("{}\n", div);

        if casino.player.balance <= 0 {
            println!("You are out of money! Game over.\n");
            break 'game_loop;
        }
        // User interaction
        //let wager = casino._wager();
        //if wager.is_ok() {
        //    let wager = wager.unwrap();
        //    println!("You will wager: ${}\n", wager);
        //    casino.pot = wager;
        //} else {
        //    continue;
        //}
        println!("Your current balance is: ${}", casino.player.balance);
        let mut rng = rand::thread_rng();
        let bet = rng.gen_range(5..(casino.player.balance as f32 * 0.1) as i32);
        let wager = bet;
        println!("You will wager: ${}\n", wager);
        casino.pot = wager;

        let msg = "Dealing hands".green();
        println!("{}...\n", msg);
        casino.deal();
        println!("Dealer cards:\n{}", casino.display_hand(false));
        'player_loop: loop {
            let (sum, special) = casino.hand_total(true);
            println!("Your cards:\n{}", casino.display_hand(true));
            if sum != special && special < 22 {
                println!("Your total: {} or {}\n", sum, special);
            } else {
                println!("Your total: {}\n", sum);
            }
            // Decide if sum or special is used for the hand total
            let sum = if special < 22 && special > sum {
                special
            } else {
                sum
            };
            // Detect bust / win
            if sum > 21 {
                // You bust
                let msg = "You Bust".red();
                println!("{}!\n", msg);
                continue 'game_loop;
            }
            if casino.player.hand.as_ref().unwrap().len() == 2 && sum == 21 {
                // You win
                let msg = "BlackJack, You Win".green();
                println!("{}!\n", msg);
                continue 'game_loop;
            }
            // User interaction
            // let action = casino._action();
            // if action.is_ok() {
            //     let action = action.unwrap();
            //     if action.trim() == "h" {
            //         casino.deal_hand(true);
            //     } else if action.trim() == "s" {
            //         println!("\nStand..\n");
            //         break 'player_loop;
            //     }
            // }
            // Automated from strategy
            let action = casino.strategy();
            if *action == 'h' {
                casino.deal_hand(true);
            } else if *action == 's' {
                println!("\nStand..\n");
                break 'player_loop;
            }
            let time = std::time::Duration::from_millis(1200);
            std::thread::sleep(time);
        }
        'dealer_loop: loop {
            casino.deal_hand(false);
            let (sum, special) = casino.hand_total(false);
            println!("Dealer cards:\n{}", casino.display_hand(false));
            if sum != special && special < 22 {
                println!("Dealer total: {} or {}\n", sum, special);
            } else {
                println!("Dealer total: {}\n", sum);
            }
            // Same here
            let sum = if special < 22 && special > sum {
                special
            } else {
                sum
            };
            if sum > 21 {
                // Dealer bust
                let msg = "Dealer Bust, You Win".green();
                println!("{}!\n", msg);
                casino.player.balance += casino.pot;
                break 'dealer_loop;
            }
            if casino.hand.as_ref().unwrap().len() == 2 && sum == 21 {
                // Dealer BlackJack
                let msg = "BlackJack, You Lose".red();
                println!("{}!\n", msg);
                casino.player.balance -= casino.pot;
                break 'dealer_loop;
            }
            if sum >= 17 {
                let (player_sum, _) = casino.hand_total(true);
                if player_sum == sum {
                    let msg = "Push".yellow();
                    println!("{}!\n", msg);
                    break 'dealer_loop;
                } else if player_sum > sum {
                    let msg = "You Win".green();
                    println!("{}!\n", msg);
                    casino.player.balance += casino.pot;
                    break 'dealer_loop;
                } else {
                    let msg = "You Lose".red();
                    println!("{}!\n", msg);
                    casino.player.balance -= casino.pot;
                    break 'dealer_loop;
                }
            }
            let time = std::time::Duration::from_millis(1200);
            std::thread::sleep(time);
        }
    } /* end outer */
}

impl Casino {
    pub fn strategy(&mut self) -> &char {
        let dealer_card = self.hand.as_mut().unwrap();
        let header_idx = self.header[&dealer_card[0].value];
        let (sum, special) = self.hand_total(true);
        let sum = if special < 22 && special > sum {
            special
        } else {
            sum
        };
        self.chart[&sum].iter().nth(header_idx as usize).unwrap()
    }

    pub fn shuffle_deck(&mut self) {
        // Shuffle the vector of Cards
        let mut cards = new_deck();
        let mut rng = rand::thread_rng();
        let mut temp: Vec<Card> = Vec::new();
        let msg = "Suffling the deck".yellow();
        println!("\n{}...\n", msg);
        while cards.len() > 0 {
            let idx = rng.gen_range(0..=cards.len() - 1);
            let card = cards.get(idx).expect("card index doesn't exist").clone();
            temp.push(card);
            cards.remove(idx);
        }
        self.deck = Some(temp);
    }

    pub fn display_hand(&mut self, is_player: bool) -> String {
        // Display cards for either player or dealer
        let mut output = String::new();
        if is_player {
            let hand = self.player.hand.as_ref().unwrap();
            for card in hand.iter() {
                output.push_str(&format!("{} of {}\n", card.face, card.suit));
            }
        } else {
            let hand = self.hand.as_ref().unwrap();
            for card in hand.iter() {
                output.push_str(&format!("{} of {}\n", card.face, card.suit));
            }
        }
        output
    }

    pub fn hand_total(&mut self, is_player: bool) -> (i32, i32) {
        // We keep track of the two separate hand values
        // sum: hand value where ace = 1
        let mut sum: i32 = 0;
        // special: hand value where ace = 11
        let mut special: i32 = 0;
        if is_player {
            let hand = self.player.hand.as_ref().unwrap();
            for card in hand.iter() {
                sum += card.value;
                special += card.value + card.special;
            }
        } else {
            let hand = self.hand.as_ref().unwrap();
            for card in hand.iter() {
                sum += card.value;
                special += card.value + card.special;
            }
        }
        (sum, special)
    }

    pub fn _action(&mut self) -> Result<String, ()> {
        // Vector of possible answers
        let answers = vec!["h", "s"];
        // Asking the question: Hit or Stand?
        println!("Hit (h) or Stand (s)?");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        if answers.contains(&answer.as_str().trim()) {
            return Ok(answer);
        }
        Err(())
    }

    pub fn _wager(&mut self) -> Result<i32, ()> {
        let msg = "Place your bets".green();
        println!("{}...\n", msg);
        let mut answer = String::new();
        println!("Your current balance is: ${}", self.player.balance);
        std::io::stdin().read_line(&mut answer).unwrap();
        // Don't forget to trim the stdin \n
        let answer = answer.trim().parse().unwrap();
        if answer > 0 {
            if answer > self.player.balance {
                println!("You don't have enough money to make that wager");
            } else {
                return Ok(answer);
            }
        }
        Err(())
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.as_mut().expect("deck is not initialized").pop()
    }

    pub fn deal(&mut self) {
        let dealer_hand: Vec<Card> = Vec::new();
        let player_hand: Vec<Card> = Vec::new();
        self.hand = Some(dealer_hand);
        self.player.hand = Some(player_hand);
        self.deal_hand(true);
        self.deal_hand(false);
        self.deal_hand(true);
    }

    pub fn deal_hand(&mut self, player_hand: bool) {
        // When we draw the card, the deck could be None.
        // This will require a new shuffle
        // It is easier to use an if let here to handle the None case
        if let Some(card) = self.draw_card() {
            // Note: card is availabe here via if let
            if player_hand {
                self.player
                    .hand
                    .as_mut()
                    .expect("hand is not available")
                    .push(card);
            } else {
                self.hand
                    .as_mut()
                    .expect("hand is not available")
                    .push(card);
            }
        } else {
            // None
            self.shuffle_deck();
            self.deal_hand(player_hand);
        }
    }
}

pub fn new_deck() -> Vec<Card> {
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
            // Special card: Ace
            let special = if face == &"Ace" { 10 } else { 0 };
            let card = Card {
                face: face.to_string(),
                suit: suit.to_string(),
                value: *value,
                special,
            };
            cards.push(card);
        }
    }
    cards
}
