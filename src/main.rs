use rand::*;

#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}
impl Suit {
    const ALL:[Suit;4]=[Suit::Spades, Suit::Hearts,Suit::Diamonds,Suit::Clubs];
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
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}
impl Rank {
    const ALL:[Rank;13]=[Rank::Two,Rank::Three,Rank::Four,Rank::Five,Rank::Six,Rank::Seven,
    Rank::Eight,Rank::Nine,Rank::Ten,Rank::Jack,Rank::Queen,Rank::King,Rank::Ace];
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
        
        for itemSuit in Suit::ALL.iter(){
            for itemRank in Rank::ALL.iter(){
                self.cards.push(Card{rank:*itemRank, suit:Some(*itemSuit)})
            }
        }
        //self.cards.shuffle(&mut rng());
    }
}

fn main(){

    let mut d1=Deck{cards:vec![]};
    d1.new();
    //println!("{:?}",d1);


}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn chequeo_mazo(){
        let mut d1=Deck{cards:vec![]};
        d1.new();
        assert_eq!(d1.cards.len(),52,"El mazo no tiene 52 cartas");

    }
    

}