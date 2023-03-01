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
                println!("{} {:?}", idx, item);
                self.cards.push(card);
                deck.cards.remove(idx);
                break;
            }
        } 
    }
}

fn main() {
    let queen_of_hearts: Card = Card {
        name: String::from("Queen of Hearts"),
        value: 10,
    };

    let king_of_hearts: Card = Card {
        name: String::from("King of Hearts"),
        value: 10,
    };

    let jack_of_hearts: Card = Card {
        name: String::from("Jack of Hearts"),
        value: 10,
    };

    println!("{:?}", queen_of_hearts);
    println!("{:?}", king_of_hearts);

    let mut player_hand: Vec<Card> = Vec::new();

    let mut deck = init_deck();

    let mut hand1: Hand = Hand {
        cards: player_hand.clone(),
    };

    hand1.draw_from(&mut deck, queen_of_hearts);
    hand1.draw_from(&mut deck, jack_of_hearts);
    hand1.draw_from(&mut deck, king_of_hearts);

    println!("{:#?}", deck);

    println!("{:#?}", hand1);
}

fn hit() {
    
}

fn get_winner() {
    
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