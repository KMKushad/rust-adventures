use rand::Rng;
use std::io;

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    name: String,
    value: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn draw_from(&mut self, deck: &mut Deck, card: Card) {
        for (idx, item) in deck.cards.iter().enumerate() {
            if item.name == card.name {
                self.cards.push(card);
                deck.cards.remove(idx);
                break;
            }
        } 
    }

    fn get_value(&self) -> i32 {
        let mut sum = 0;
        for card in self.cards.iter() {
            sum += card.value;
        }
        
        if sum > 21 {
            for card in self.cards.iter() {
                if card.value == 11 {
                    sum -= 10;
                    if sum <= 21 {
                        break;
                    }
                }
            }
        }

        sum
    }

    fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
}

impl Deck {
    fn select(&self) -> Card {
        let idx = rand::thread_rng().gen_range(1..=self.cards.len());
        self.cards[idx - 1].clone()
    }

    fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
}

fn main() {
    let mut deck = init_deck();

    let mut dealer: Hand = Hand {
        cards: Vec::new(),
    };

    for _ in 0..2 {
        let card = deck.select();
        dealer.draw_from(&mut deck, card);
    }
    
    let mut player: Hand = Hand {
        cards: Vec::new(),
    };

    for _ in 0..2 {
        let card = deck.select();
        player.draw_from(&mut deck, card);
    }

    println!("The dealer has a {} and one unrevealed card.\n", dealer.get_cards()[0].name);

    let mut player_standed = false;

    while player_standed == false {
        let mut player_card_string = String::new();

        for (idx, card) in player.get_cards().iter().enumerate() {
            if idx == 0 {
                player_card_string = format!("{}", card.name)
            }
            else if player.get_cards().len() == 2 {
                player_card_string = format!("You have a {} and a {} for a total of {} points.", 
                    player.get_cards()[0].name, 
                    player.get_cards()[1].name, 
                    player.get_value()
                )
            }
            else if idx != player.get_cards().len() - 1 {
                player_card_string = format!("{}, {}", player_card_string, card.name);
            }
            else {
                player_card_string = format!("You have a {}, and a {} for a total of {} points.", 
                    player_card_string, 
                    card.name, 
                    player.get_value());
            }
        }

        println!("{player_card_string}");

        println!("You can choose to [h]it or [s]tand.");

        let mut action = String::new();

        io::stdin()
            .read_line( &mut action)
            .expect("Failed to read line");

        if action.trim() == "h".to_string() {
            let card = hit(&mut player, &mut deck, true);
            println!("You drew a {}.", card.name);
        }
        else if action.trim() == "s".to_string() {
            player_standed = true;
        }

        if player.get_value() >= 21 {
            player_standed = true;
        }
    }

    println!("\nThe dealer's unrevealed card was a {}.", dealer.get_cards()[1].name);

    while dealer.get_value() < 17 {
        let card = hit(&mut dealer, &mut deck, false);
        println!("The dealer drew a {}.", card.name);
    }

    let result = get_winner(&dealer, &player);
    println!("\n{}", result);

}

fn hit(player: &mut Hand, mut deck: &mut Deck, rigged: bool) -> Card {
    if rigged {
        if player.get_value() == 20 {
            let mut aces = Vec::new();

            for i in deck.get_cards() {
                if i.value == 11 {
                    aces.push(i);
                }
            }

            let idx = rand::thread_rng().gen_range(1..=aces.len());

            let ace = aces[idx-1].clone();

            player.draw_from(&mut deck, ace.clone());

            ace
        }
        else if 21 - player.get_value() < 12 {
            let mut cards = Vec::new();

            for i in deck.get_cards() {
                if i.value == 21 - player.get_value() {
                    cards.push(i);
                }
            }
            
            let idx = rand::thread_rng().gen_range(1..=cards.len());

            let card = cards[idx-1].clone();

            player.draw_from(&mut deck, card.clone());

            card
        }
        else {
            let mut aces = Vec::new();

            for i in deck.get_cards() {
                if i.value == 11 {
                    aces.push(i);
                }
            }

            let idx = rand::thread_rng().gen_range(1..=aces.len());

            let ace = aces[idx-1].clone();

            player.draw_from(&mut deck, ace.clone());
            
            ace
        }
    }
    else {
        let card: Card = deck.select();
        player.draw_from(&mut deck, card.clone());
        card
    }
}

fn get_winner(dealer: &Hand, player: &Hand) -> String {
    if dealer.get_value() == player.get_value() {
        String::from("How did you get here? It's a tie.")
    }
    else if dealer.get_value() == 21 {
        String::from("The dealer got a 21 and wins!")
    }
    else if player.get_value() == 21 {
        String::from("The player got a 21 and wins!")
    }
    else if dealer.get_value() > 21 {
        String::from("Oops! The dealer got over 21 and busted. The other player wins!")
    }
    else if player.get_value() > 21 {
        String::from("Oops! The player got over 21 and busted. The dealer wins!")
    }
    else if dealer.get_value() > player.get_value() {
        String::from("The dealer's card total was greater than the other player's card total. The dealer wins.")
    }
    else if player.get_value() > dealer.get_value() {
        String::from("The player's card total was greater than the dealer's card total. The player wins.")
    }
    else {
        String::from("How did you get here? It's a tie.")
    }

}

fn init_deck() -> Deck {
    let mut deck = Deck {
        cards: Vec::new(), 
    };

    let suits = vec![
        "Clubs",
        "Spades",
        "Diamonds",
        "Hearts",
        ];

    let ranks = vec![
        "Ace",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Jack",
        "Queen",
        "King",
        ];

    for i in suits {
        for j in &ranks {
            let name = format!("{j} of {i}");
            let value = match *j {
                "Ace" => 11,
                "Two" => 2,
                "Three" => 3,
                "Four" => 4,
                "Five" => 5,
                "Six" => 6,
                "Seven" => 7,
                "Eight" => 8,
                "Nine" => 9,
                "Ten" | "Jack" | "Queen" | "King" => 10,
                &_ => 0,
            };
            
            let card: Card = Card {
                name,
                value,
            };

            deck.cards.push(card);
        }
    }

    deck
}