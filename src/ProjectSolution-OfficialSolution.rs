use rand::{rng,Rng,seq::SliceRandom};

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
    suit:Option<Suit>,
    rank:Rank,
}

#[derive(Debug)]
struct Deck {
    cards:Vec<Card>,
}
impl Deck{
    fn new() -> Self {
        let suits:[Suit;4]=[Suit::Spades, Suit::Hearts,Suit::Diamonds,Suit::Clubs];
        let ranks:[Rank;13]=[Rank::Two,Rank::Three,Rank::Four,Rank::Five,Rank::Six,Rank::Seven,
        Rank::Eight,Rank::Nine,Rank::Ten,Rank::Jack,Rank::Queen,Rank::King,Rank::Ace];

        let mut cards=Vec::with_capacity(52);
        
        for suit in suits.into_iter(){
            for rank in ranks.into_iter(){
                cards.push(Card{suit:Some(suit),rank:rank});
            }
        }
        Self{cards}
    }
    fn shuffle(&mut self){
        self.cards.shuffle(&mut rand::rng());
    }
    fn insert_jokers(&mut self){
        let mut my_rng=rng();
        for _ in 0..2 {
            let random_index= my_rng.random_range(0..self.cards.len()); // 0..52
            self.cards.insert(random_index,Card{suit:None,rank:Rank::Joker});
        }
    }
    fn delete_random_card(&mut self){
        let mut my_rng=rng();
        //if rng().random_ratio(65,100){
        if my_rng.random_bool(0.65){
            let pos_to_delete:usize=my_rng.random_range(0..self.cards.len());
            self.cards.remove(pos_to_delete);
        }
    }
}


fn main(){
    let mut deck=Deck::new();
    println!("{:#?}", deck);
    deck.shuffle();
    println!("{:#?}", deck);
    for _ in 1..=10 {
        deck.delete_random_card();
        println!("{:?}", deck.cards.len());
    }
}