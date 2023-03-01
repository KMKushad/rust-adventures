use rand::Rng;

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
}

fn main() {
    let mut deck = init_deck();

    let mut _queen_of_hearts: Card = Card {
        name: String::from("Queen of Hearts"),
        value: 10,
    };

    let mut _two_of_hearts: Card = Card {
        name: String::from("Two of Hearts"),
        value: 2,
    };

    let mut hand1: Hand = Hand {
        cards: Vec::new(),
    };

    for _ in 0..2 {
        let card = deck.select();
        hand1.draw_from(&mut deck, card);
    }
    
    let mut hand2: Hand = Hand {
        cards: Vec::new(),
    };

    for _ in 0..2 {
        let card = deck.select();
        hand2.draw_from(&mut deck, card);
    }

    //hand2.draw_from(&mut deck, two_of_hearts);
    //hand2.draw_from(&mut deck, queen_of_hearts);

    println!("{:#?}", hand1.get_cards());
    println!("{:#?}", hand1.get_value());

    println!("{:#?}", hand2.get_cards());
    println!("{:#?}", hand2.get_value());

    let winner = get_winner(hand1, hand2);
    println!("{:#?}", winner);
}

fn hit() {
    
}

fn get_winner(player1: Hand, player2: Hand) -> i8 {
    if player1.get_value() == 21 {
        1
    }
    else if player2.get_value() == 21 {
        2
    }
    else if player1.get_value() > 21 {
        2
    }
    else if player2.get_value() > 21 {
        1
    }
    else if player1.get_value() > player2.get_value() {
        1
    }
    else if player2.get_value() > player1.get_value() {
        2
    }
    else {
        0
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