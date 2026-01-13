#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}
#[derive(Debug)]
struct Card {
    rank:Rank,
    suit:Option<Suit>,
}

#[derive(Debug)]
struct Deck {
    cards:Vec<Card>,
}

impl Deck{
    fn new(&mut self){
        
        for itemSuit in Suit.iter(){
            for itemRank in Rank.iter(){
                self.cards.push(Card{rank:itemRank, suit:Some(itemSuit)})
            }
        } 
    }
}

fn main(){



}