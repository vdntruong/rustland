use rand::{rng, seq::SliceRandom};

// derive, called the 'drive attribute', specifies that we want to use the Debug trait.
// Debug, called the 'Debug trait', allows us to print the struct in a readable format.
// Trait is a collection of methods. Trait is a contract that defines what methods a type must implement.
//
// Its a trait, the statement defines 'attributes' for the struct.
// Gives the rust compiler some extra instructions.
#[derive(Debug)]
struct Deck {
    cards: Vec<String>, // Vec is a vector, it's a dynamic array (~ Slice in Go)
}

// associated functions are functions that are associated with a type, like Static functions in C#
impl Deck {
    // fn new() -> Deck {}
    fn new() -> Self {
        // Array is a fixed-size sequence of elements of the same type
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                // format is a macro, it's like printf in C
                // to call it like a function, we need to use `format!` instead of `format`
                // `!` is a marker for macros, it's like a function call
                // println! is a macro as well
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // in Rust we don't have variables, we have bindings
        // let is used to create bindings
        // bindings are immutable by default
        // let deck: Deck = Deck { cards: cards };
        // let deck: Deck = Deck { cards };
        // return deck;

        // Initial cards by using vec!, it's a macro
        // let deck: Deck = Deck {
        //     cards: vec![],
        // };

        // Initial cards by using Vec::new(), it's a method/function style
        // let deck: Deck = Deck {
        //     cards: Vec::new()
        // };

        // return Deck { cards };
        Deck { cards } // This is Implicit return, popular to use in Rust
        // Implicit return without semicolon and `return` keyword
    }

    fn shuffle(&mut self) {
        // `&mut self` is a reference to the current instance, and we expecting to modify it

        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards) // this is a Implicit return
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    // {:?} it's `Debug` trait
    println!("Heres your deck: {:#?}", deck);

    // probably need to add error handling !!
    let hand = deck.deal(5);
    println!("Heres your hand: {:#?}", hand);
    println!("Heres deck: {:#?}", deck);
}
