use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    suit: Suit,
    value: &'static str,
}

impl Card {
    fn new(suit: Suit, value: &'static str) -> Self {
        Self { suit, value }
    }

    fn get_value(&self) -> u8 {
        match self.value {
            "A" => 11,
            "K" | "Q" | "J" => 10,
            _ => self.value.parse().unwrap(),
        }
    }

    pub fn display(&self) -> String {
        format!("{}{}", self.value, match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        })
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let mut cards = Vec::new();

        for &suit in &suits {
            for &value in &values {
                cards.push(Card::new(suit, value));
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn get_value(&self) -> u8 {
        let mut value = 0;
        let mut ace_count = 0;

        for &card in &self.cards {
            value += card.get_value();
            if card.value == "A" {
                ace_count += 1;
            }
        }

        while value > 21 && ace_count > 0 {
            value -= 10;
            ace_count -= 1;
        }

        value
    }

    pub fn display(&self, hide_second_card: bool) -> String {
        let mut cards_display = self.cards.iter().map(|card| card.display()).collect::<Vec<_>>();
        if hide_second_card && cards_display.len() > 1 {
            cards_display[1] = "Hidden".to_string();
        }
        let total_value = self.get_value();
        format!("{} (Total: {})", cards_display.join(", "), total_value)
    }
}

pub struct Game {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    player_money: i32,
    current_bet: i32,
    wins: u32,
    losses: u32,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        Self {
            deck,
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            player_money: 100,
            current_bet: 0,
            wins: 0,
            losses: 0,
            game_over: false,
        }
    }

    pub fn start_round(&mut self, bet: i32) {
        self.current_bet = bet;
        self.player_hand = Hand::new();
        self.dealer_hand = Hand::new();
        self.game_over = false;

        self.player_hand.add_card(self.deck.deal_card());
        self.dealer_hand.add_card(self.deck.deal_card());
        self.player_hand.add_card(self.deck.deal_card());
        self.dealer_hand.add_card(self.deck.deal_card());
    }

    pub fn hit(&mut self) {
        self.player_hand.add_card(self.deck.deal_card());
        if self.player_hand.get_value() > 21 {
            self.game_over = true;
            self.losses += 1;
            self.player_money -= self.current_bet;
        }
    }

    pub fn stand(&mut self) {
        while self.dealer_hand.get_value() < 17 {
            self.dealer_hand.add_card(self.deck.deal_card());
        }

        let player_value = self.player_hand.get_value();
        let dealer_value = self.dealer_hand.get_value();

        if dealer_value > 21 || player_value > dealer_value {
            self.wins += 1;
            self.player_money += self.current_bet;
        } else if player_value < dealer_value {
            self.losses += 1;
            self.player_money -= self.current_bet;
        }

        self.game_over = true;
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_player_hand(&self) -> &Hand {
        &self.player_hand
    }

    pub fn get_dealer_hand(&self) -> &Hand {
        &self.dealer_hand
    }

    pub fn get_player_money(&self) -> i32 {
        self.player_money
    }

    pub fn get_wins(&self) -> u32 {
        self.wins
    }

    pub fn get_losses(&self) -> u32 {
        self.losses
    }
}